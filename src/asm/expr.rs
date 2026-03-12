use std::collections::HashSet;
use std::iter;

use koopa::ir::entities::ValueData;
use koopa::ir::values::{Alloc, Integer, Store};
use koopa::ir::{BinaryOp as KBinaryOp, TypeKind, ValueKind};
use koopa::ir::{
    Value,
    values::{Binary, Load, Return},
};

use crate::asm::inst::InstContext;
use crate::asm::meta::{RV32Imm, RV32Usize};
use crate::asm::{
    inst,
    meta::{FunctionContext, Register, RiscvAsm, ToAsm},
};

fn handle_special_cases(data: &ValueData) -> Option<Register> {
    if let ValueKind::Integer(num) = data.kind()
        && num.value() == 0
    {
        return Some(Register::Zero);
    }
    None
}

pub fn get_value_from_mem(
    value: Value,
    context: &mut FunctionContext,
    asms: &mut Vec<RiscvAsm>,
) -> Option<Register> {
    let offset = context.memory_mapper.get_offset(
        &value,
        context.func_data.dfg().value(value).ty().size() as u32,
    )?;
    let rd = obtain_caller_directly_usable_register(context);
    asms.extend(inst::add_lw_instruction(
        rd,
        Register::Sp,
        RV32Imm::new(offset as i32),
        Some(InstContext::new(context, value)),
        Some(rd),
    ));
    Some(rd)
}

/// 目前暂时先将所有的变量存储在寄存器当中，之后再考虑将部分变量存储在内存当中。
pub fn get_value(
    value: Value,
    context: &mut FunctionContext,
    asms: &mut Vec<RiscvAsm>,
) -> HashSet<Register> {
    // println!("Getting value: {:?}", value);
    match context.register_mapper.get_by_value(&value) {
        // 由于生命周期的原因，这里暂时只能使用拷贝，之后再考虑优化。
        Some(reg_set) => reg_set.clone(),
        None => match get_value_from_mem(value, context, asms) {
            Some(reg) => iter::once(reg).collect(),
            None => {
                let value_data = context.func_data.dfg().value(value);
                if let Some(r) = handle_special_cases(value_data) {
                    return iter::once(r).collect();
                }
                asms.extend(value_data.to_asm(Some(context), Some(value)));
                let maybe_value = context.register_mapper.get_by_value(&value);
                match maybe_value {
                    Some(reg_set) => reg_set.clone(),
                    None => get_value_from_mem(value, context, asms)
                        .map(iter::once)
                        .unwrap_or_else(|| panic!("No register assigned for value {:?}", value))
                        .collect(),
                }
            }
        },
    }
}

pub fn obtain_caller_directly_usable_register(context: &FunctionContext) -> Register {
    let available_registers = context
        .register_mapper
        .get_available_registers_filtered(|r| r.caller_directly_usable());
    *available_registers
        .iter()
        .next()
        .expect("No available register")
}

impl ToAsm for Integer {
    fn to_asm(
        &self,
        context: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        // When self.value() == 0, you should use the zero register instead of loading 0 into a
        // register with this function.
        let context = context.expect("FunctionContext not found for Integer");
        let id = id.expect("Value not found for Integer");
        let rd = obtain_caller_directly_usable_register(context);
        vec![inst::li_instruction(
            rd,
            self.value(),
            Some(InstContext::new(context, id)),
        )]
    }
}

impl ToAsm for Return {
    fn to_asm(&self, context: Option<&mut FunctionContext>, id: Option<Value>) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for Return");
        let id = id.expect("Value not found for Return");
        let mut asms = vec![];
        match self.value() {
            None => {
                asms.extend(context.memory_mapper.resume_stack());
                context.register_mapper.clear();
                asms.push(inst::ret_instruction());
            }
            Some(value) => {
                let reg_set = get_value(value, context, &mut asms);
                if !reg_set.contains(&Register::A0) {
                    asms.push(inst::mv_instruction(
                        Register::A0,
                        *reg_set
                            .iter()
                            .next()
                            .expect("No register assigned for return value"),
                        Some(InstContext::new(context, id)),
                    ));
                }
                asms.extend(context.memory_mapper.resume_stack());
                context.register_mapper.clear();
                asms.push(inst::ret_instruction());
            }
        }
        asms
    }
}

impl ToAsm for Binary {
    fn to_asm(
        &self,
        func_data: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        let context = func_data.expect("FunctionContext not found for Binary");
        let id = id.expect("Value not found for Binary");
        let mut asms = vec![];

        let lhs_reg = *get_value(self.lhs(), context, &mut asms)
            .iter()
            .next()
            .expect("No register assigned for lhs");
        let rhs_reg = *get_value(self.rhs(), context, &mut asms)
            .iter()
            .next()
            .expect("No register assigned for rhs");

        fn binary_op_helper<F>(
            lhs_reg: Register,
            rhs_reg: Register,
            context: &mut FunctionContext,
            id: Value,
            asms: &mut Vec<RiscvAsm>,
            func: F,
        ) -> Register
        where
            F: Fn(Register, Register, Register, Option<InstContext>) -> RiscvAsm,
        {
            // TODO: Temporarily disable this call because we can't make use of the stack yet.
            // let rd = obtain_caller_directly_usable_register(context);
            // asms.push(func(rd, lhs_reg, rhs_reg, context, id));
            if lhs_reg.caller_directly_usable() {
                asms.push(func(
                    lhs_reg,
                    lhs_reg,
                    rhs_reg,
                    Some(InstContext::new(context, id)),
                ));
                lhs_reg
            } else {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(func(
                    rd,
                    lhs_reg,
                    rhs_reg,
                    Some(InstContext::new(context, id)),
                ));
                rd
            }
        }

        // rd is the register that holds the result of the binary operation.
        // This helps us to push the result to the stack if necessary in the future.
        let rd = match self.op() {
            KBinaryOp::Add => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::add_instruction,
            ),
            KBinaryOp::Sub => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::sub_instruction,
            ),
            KBinaryOp::Mul => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::mul_instruction,
            ),
            KBinaryOp::Div => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::div_instruction,
            ),
            KBinaryOp::Mod => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::rem_instruction,
            ),
            KBinaryOp::And => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::and_instruction,
            ),
            KBinaryOp::Or => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::or_instruction,
            ),
            KBinaryOp::Xor => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::xor_instruction,
            ),
            KBinaryOp::Eq => {
                let rd = obtain_caller_directly_usable_register(context);

                if rhs_reg == Register::Zero {
                    asms.push(inst::seqz_instruction(
                        rd,
                        lhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                } else if lhs_reg == Register::Zero {
                    asms.push(inst::seqz_instruction(
                        rd,
                        rhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                } else {
                    // don't apply binary_op_helper
                    asms.push(inst::xor_instruction(lhs_reg, lhs_reg, rhs_reg, None));
                    asms.push(inst::seqz_instruction(
                        rd,
                        lhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                }

                rd
            }
            KBinaryOp::NotEq => {
                let rd = obtain_caller_directly_usable_register(context);

                if rhs_reg == Register::Zero {
                    asms.push(inst::snez_instruction(
                        rd,
                        lhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                } else if lhs_reg == Register::Zero {
                    asms.push(inst::snez_instruction(
                        rd,
                        rhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                } else {
                    // don't apply binary_op_helper
                    asms.push(inst::xor_instruction(lhs_reg, lhs_reg, rhs_reg, None));
                    asms.push(inst::snez_instruction(
                        rd,
                        lhs_reg,
                        Some(InstContext::new(context, id)),
                    ));
                }

                rd
            }
            KBinaryOp::Gt => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::sgt_instruction,
            ),
            KBinaryOp::Lt => binary_op_helper(
                lhs_reg,
                rhs_reg,
                context,
                id,
                &mut asms,
                inst::slt_instruction,
            ),
            KBinaryOp::Ge => {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(inst::slt_instruction(rd, lhs_reg, rhs_reg, None));
                asms.push(inst::seqz_instruction(
                    rd,
                    rd,
                    Some(InstContext::new(context, id)),
                ));
                rd
            }
            KBinaryOp::Le => {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(inst::sgt_instruction(rd, lhs_reg, rhs_reg, None));
                asms.push(inst::seqz_instruction(
                    rd,
                    rd,
                    Some(InstContext::new(context, id)),
                ));
                rd
            }
            _ => unimplemented!(),
        };

        // For now, we always try to push the result to the stack and clear the bindings between
        // the result register and the value. This is because we haven't implemented register
        // allocation yet.
        let tmp = obtain_caller_directly_usable_register(context);
        let size = context.func_data.dfg().value(id).ty().size() as u32;
        context.memory_mapper.claim(id, size);
        let offset = context.memory_mapper.get_offset(&id, size).expect(
            "Error occurs when trying to allocate stack memory for binary operation result",
        );
        asms.extend(inst::add_sw_instruction(
            rd,
            Register::Sp,
            RV32Imm::new(offset as i32),
            Some(InstContext::new(context, id)),
            Some(tmp),
        ));

        // Erase the binding between the result register and the value, so that the register can be
        // reused
        context.register_mapper.clear();

        asms
    }
}

impl ToAsm for Alloc {
    fn to_asm(
        &self,
        context: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for Alloc");
        let id = id.expect("Value not found for Alloc");

        // Alloc has a type of pointer.
        let ty_ptr = context.func_data.dfg().value(id).ty();

        // But we need to store the underlying type in the stack, so we need to get the underlying
        // type of the pointer.
        let size = match ty_ptr.kind() {
            TypeKind::Pointer(ty) => {
                ty.size()
            }
            _ => panic!("The type of Alloc value is expected to be pointer, but found {:?}", ty_ptr),
        };

        context.memory_mapper.claim(id, size as RV32Usize);
        vec![]
    }
}

impl ToAsm for Load {
    fn to_asm(
        &self,
        context: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for Load");
        let id = id.expect("Value not found for Load");
        let size = context.func_data.dfg().value(self.src()).ty().size() as u32;
        let mut asms = vec![];

        let rd = obtain_caller_directly_usable_register(context);
        let offset = context
            .memory_mapper
            .get_offset(&self.src(), size)
            .expect("Error occurs when trying to get stack memory offset for loading value");

        asms.extend(inst::add_lw_instruction(
            rd,
            Register::Sp,
            RV32Imm::new(offset as i32),
            Some(InstContext::new(context, id)),
            Some(rd),
        ));

        let ld_size = context.func_data.dfg().value(id).ty().size() as u32;
        context.memory_mapper.claim(id, ld_size);
        let offset = context
            .memory_mapper
            .get_offset(&id, ld_size)
            .expect("Error occurs when trying to allocate stack memory for load result");

        // load IR return value into rd, and then store it to the stack.
        //
        // To get value directly from the stack and not be pushed to the stack, get_value() should
        // be used instead of Load instruction.
        let tmp = obtain_caller_directly_usable_register(context);
        asms.extend(inst::add_sw_instruction(
            rd,
            Register::Sp,
            RV32Imm::new(offset as i32),
            Some(InstContext::new(context, id)),
            Some(tmp),
        ));

        context.register_mapper.clear();

        asms
    }
}

impl ToAsm for Store {
    fn to_asm(
        &self,
        context: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for Store");
        let id = id.expect("Value not found for Store");
        let size = context.func_data.dfg().value(self.dest()).ty().size() as u32;
        let mut asms = vec![];

        let rd = *get_value(self.value(), context, &mut asms)
            .iter()
            .next()
            .expect("No register assigned for store value");
        let tmp = obtain_caller_directly_usable_register(context);
        let offset = context
            .memory_mapper
            .get_offset(&self.dest(), size)
            .expect("Error occurs when trying to get stack memory offset for storing value");

        asms.extend(inst::add_sw_instruction(
            rd,
            Register::Sp,
            RV32Imm::new(offset as i32),
            Some(InstContext::new(context, id)),
            Some(tmp),
        ));

        // Only clear the binding between the stored value and the register.
        context.register_mapper.remove(self.value(), rd);

        asms
    }
}
