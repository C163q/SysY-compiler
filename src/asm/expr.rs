use std::collections::HashSet;
use std::iter;

use koopa::ir::entities::ValueData;
use koopa::ir::values::{Alloc, Branch, Call, Integer, Jump, Store};
use koopa::ir::{BinaryOp as KBinaryOp, TypeKind, ValueKind};
use koopa::ir::{
    Value,
    values::{Binary, Load, Return},
};

use crate::asm::func::build_call_stack_and_registers;
use crate::asm::inst::InstContext;
use crate::asm::meta::{RV32Imm, RV32Usize};
use crate::asm::var;
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
    let (vec, rd) = var::load(value, context, Some(value)).ok()?;
    asms.extend(vec);
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
                asms.extend(value_data.to_asm(context, value));
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

pub fn get_param_registers_filter(count: u8) -> impl Fn(&Register) -> bool {
    move |r: &Register| {
        ((*r as u8) < (Register::A0 as u8)) || ((*r as u8) >= (Register::A0 as u8 + count.min(8)))
    }
}

pub fn obtain_caller_directly_usable_register(context: &FunctionContext) -> Register {
    let param_count = context.func_data.params().len().min(8) as u8;
    let available_registers = context
        .register_mapper
        .get_available_registers_filtered(|r| {
            r.caller_directly_usable() && get_param_registers_filter(param_count)(r)
        });
    *available_registers
        .iter()
        .next()
        .expect("No available register")
}

impl ToAsm for Integer {
    fn to_asm(&self, context: &mut FunctionContext, id: Value) -> Vec<RiscvAsm> {
        // When self.value() == 0, you should use the zero register instead of loading 0 into a
        // register with this function.
        let rd = obtain_caller_directly_usable_register(context);
        vec![inst::li_instruction(
            rd,
            self.value(),
            Some(InstContext::new(context, id)),
        )]
    }
}

impl ToAsm for Return {
    fn to_asm(&self, context: &mut FunctionContext, id: Value) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        match self.value() {
            None => {
                asms.extend(inst::add_lw_instruction(
                    Register::Ra,
                    Register::Sp,
                    RV32Imm::new(context.memory_mapper.meta_offset() as i32),
                    None,
                    Some(Register::T1),
                ));
                asms.extend(context.memory_mapper.stack_resume());
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
                asms.extend(inst::add_lw_instruction(
                    Register::Ra,
                    Register::Sp,
                    RV32Imm::new(context.memory_mapper.meta_offset() as i32),
                    None,
                    Some(Register::T1),
                ));
                asms.extend(context.memory_mapper.stack_resume());
                context.register_mapper.clear();
                asms.push(inst::ret_instruction());
            }
        }
        asms
    }
}

impl ToAsm for Binary {
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm> {
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
        asms.extend(
            var::store(rd, id, context, Some(id), true)
                .expect("Error occurs when trying to store binary operation result to stack"),
        );

        // Erase the binding between the result register and the value, so that the register can be
        // reused
        context.register_mapper.clear();

        asms
    }
}

impl ToAsm for Alloc {
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm> {
        // Alloc has a type of pointer.
        let ty_ptr = context.func_data.dfg().value(id).ty();

        // But we need to store the underlying type in the stack, so we need to get the underlying
        // type of the pointer.
        let size = match ty_ptr.kind() {
            TypeKind::Pointer(ty) => ty.size(),
            _ => panic!(
                "The type of Alloc value is expected to be pointer, but found {:?}",
                ty_ptr
            ),
        };

        context.memory_mapper.stack_claim(id, size as RV32Usize);
        vec![]
    }
}

impl ToAsm for Load {
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        let (vec, rd) = var::load(self.src(), context, None)
            .expect("Error occurs when trying to load value for load instruction");
        asms.extend(vec);

        // load IR return value into rd, and then store it to the stack.
        //
        // To get value directly from the stack and not be pushed to the stack, get_value() should
        // be used instead of Load instruction.
        asms.extend(
            var::store(rd, id, context, Some(id), true)
                .expect("Error occurs when trying to store load result to stack"),
        );

        context.register_mapper.clear();

        asms
    }
}

impl ToAsm for Store {
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm> {
        let mut asms = vec![];

        let rd = *get_value(self.value(), context, &mut asms)
            .iter()
            .next()
            .expect("No register assigned for store value");

        asms.extend(
            var::store(rd, self.dest(), context, Some(id), false)
                .expect("Error occurs when trying to store value to destination"),
        );

        // Only clear the binding between the stored value and the register.
        context.register_mapper.remove(self.value(), rd);

        asms
    }
}

impl ToAsm for Jump {
    fn to_asm(&self, context: &mut FunctionContext<'_>, _: Value) -> Vec<RiscvAsm> {
        let _args = self.args(); // For now, the IR doesn't support passing arguments.
        let target = self.target();
        vec![inst::j_instruction(&context.get_label(target))]
    }
}

impl ToAsm for Branch {
    fn to_asm(&self, context: &mut FunctionContext<'_>, _: Value) -> Vec<RiscvAsm> {
        let _true_args = self.true_args(); // For now, the IR doesn't support passing arguments.
        let _false_args = self.false_args();
        let mut asms = vec![];

        let cond_reg = *get_value(self.cond(), context, &mut asms)
            .iter()
            .next()
            .expect("No register assigned for branch condition");

        let true_label = context.get_label(self.true_bb());
        let false_label = context.get_label(self.false_bb());

        asms.push(inst::bnez_instruction(cond_reg, &true_label));
        asms.push(inst::j_instruction(&false_label));

        context.register_mapper.remove(self.cond(), cond_reg);

        asms
    }
}

impl ToAsm for Call {
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        context.memory_mapper.new_function_call();
        let data = context.program.func(self.callee());
        let args = self.args();
        let ret_ty = context.func_data.dfg().value(id).ty();
        asms.extend(build_call_stack_and_registers(context, data, args));
        context.register_mapper.clear();

        // ignore leading '@' in function name
        asms.push(inst::call_instruction(&data.name()[1..]));

        asms.extend(context.memory_mapper.function_resume());
        context.memory_mapper.end_function_call();

        if !ret_ty.is_unit() {
            asms.extend(
                var::store(Register::A0, id, context, Some(id), true)
                    .expect("Error occurs when trying to store return value to stack"),
            );
        }

        asms
    }
}
