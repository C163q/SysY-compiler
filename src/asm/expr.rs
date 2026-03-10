use std::collections::HashSet;
use std::iter;

use koopa::ir::entities::ValueData;
use koopa::ir::values::Integer;
use koopa::ir::{BinaryOp as KBinaryOp, ValueKind};
use koopa::ir::{
    Value,
    values::{Binary, Return},
};

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
        None => {
            let value_data = context.func_data.dfg().value(value);
            if let Some(r) = handle_special_cases(value_data) {
                return iter::once(r).collect();
            }
            asms.extend(value_data.to_asm(Some(context), Some(value)));
            context
                .register_mapper
                .get_by_value(&value)
                .expect("No register assigned for value")
                .clone()
        }
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
        func_data: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm> {
        // When self.value() == 0, you should use the zero register instead of loading 0 into a
        // register with this function.
        let context = func_data.expect("FunctionContext not found for Integer");
        let id = id.expect("Value not found for Integer");
        let rd = obtain_caller_directly_usable_register(context);
        vec![inst::li_instruction(rd, self.value(), context, Some(id))]
    }
}

impl ToAsm for Return {
    fn to_asm(&self, context: Option<&mut FunctionContext>, id: Option<Value>) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for Return");
        let id = id.expect("Value not found for Return");
        let mut asms = vec![];
        match self.value() {
            None => {
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
                        context,
                        Some(id),
                    ));
                }
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
        ) where
            F: Fn(Register, Register, Register, &mut FunctionContext, Option<Value>) -> RiscvAsm,
        {
            // TODO: Temporarily disable this call because we can't make use of the stack yet.
            // let rd = obtain_caller_directly_usable_register(context);
            // asms.push(func(rd, lhs_reg, rhs_reg, context, id));
            if lhs_reg.caller_directly_usable() {
                asms.push(func(lhs_reg, lhs_reg, rhs_reg, context, Some(id)));
            } else {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(func(rd, lhs_reg, rhs_reg, context, Some(id)));
            }
        }

        match self.op() {
            KBinaryOp::Add => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::add_instruction,
                );
            }
            KBinaryOp::Sub => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::sub_instruction,
                );
            }
            KBinaryOp::Mul => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::mul_instruction,
                );
            }
            KBinaryOp::Div => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::div_instruction,
                );
            }
            KBinaryOp::Mod => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::rem_instruction,
                );
            }
            KBinaryOp::And => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::and_instruction,
                );
            }
            KBinaryOp::Or => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::or_instruction,
                );
            }
            KBinaryOp::Xor => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::xor_instruction,
                );
            }
            KBinaryOp::Eq => {
                let rd = obtain_caller_directly_usable_register(context);

                if rhs_reg == Register::Zero {
                    asms.push(inst::seqz_instruction(rd, lhs_reg, context, Some(id)));
                } else if lhs_reg == Register::Zero {
                    asms.push(inst::seqz_instruction(rd, rhs_reg, context, Some(id)));
                } else {
                    // don't apply binary_op_helper
                    asms.push(inst::xor_instruction(
                        lhs_reg, lhs_reg, rhs_reg, context, None,
                    ));
                    asms.push(inst::seqz_instruction(rd, lhs_reg, context, Some(id)));
                }
            }
            KBinaryOp::NotEq => {
                let rd = obtain_caller_directly_usable_register(context);

                if rhs_reg == Register::Zero {
                    asms.push(inst::snez_instruction(rd, lhs_reg, context, Some(id)));
                } else if lhs_reg == Register::Zero {
                    asms.push(inst::snez_instruction(rd, rhs_reg, context, Some(id)));
                } else {
                    // don't apply binary_op_helper
                    asms.push(inst::xor_instruction(
                        lhs_reg, lhs_reg, rhs_reg, context, None,
                    ));
                    asms.push(inst::snez_instruction(rd, lhs_reg, context, Some(id)));
                }
            }
            KBinaryOp::Gt => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::sgt_instruction,
                );
            }
            KBinaryOp::Lt => {
                binary_op_helper(
                    lhs_reg,
                    rhs_reg,
                    context,
                    id,
                    &mut asms,
                    inst::slt_instruction,
                );
            }
            KBinaryOp::Ge => {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(inst::slt_instruction(rd, lhs_reg, rhs_reg, context, None));
                asms.push(inst::seqz_instruction(rd, rd, context, Some(id)));
            }
            KBinaryOp::Le => {
                let rd = obtain_caller_directly_usable_register(context);
                asms.push(inst::sgt_instruction(rd, lhs_reg, rhs_reg, context, None));
                asms.push(inst::seqz_instruction(rd, rd, context, Some(id)));
            }
            _ => unimplemented!(),
        }

        asms
    }
}
