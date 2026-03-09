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
        let available_registers = context
            .register_mapper
            .get_available_registers_filtered(|r| r.is_caller_saved());
        vec![inst::li_instruction(
            *available_registers
                .iter()
                .next()
                .expect("No available register"),
            self.value(),
            context,
            id,
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
                        id,
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
        )
        where
            F: Fn(Register, Register, Register, &mut FunctionContext, Value) -> RiscvAsm,
        {
            let available_registers = context
                .register_mapper
                .get_available_registers_filtered(|r| r.is_caller_saved());
            let rd = *available_registers
                .iter()
                .next()
                .expect("No available register");

            asms.push(func(rd, lhs_reg, rhs_reg, context, id));
        }
        match self.op() {
            KBinaryOp::Add => {
                binary_op_helper(lhs_reg, rhs_reg, context, id, &mut asms, inst::add_instruction);
            }
            KBinaryOp::Sub => {
                binary_op_helper(lhs_reg, rhs_reg, context, id, &mut asms, inst::sub_instruction);
            }
            KBinaryOp::Mul => {
                binary_op_helper(lhs_reg, rhs_reg, context, id, &mut asms, inst::mul_instruction);
            }
            KBinaryOp::Div => {
                binary_op_helper(lhs_reg, rhs_reg, context, id, &mut asms, inst::div_instruction);
            }
            KBinaryOp::Mod => {
                binary_op_helper(lhs_reg, rhs_reg, context, id, &mut asms, inst::rem_instruction);
            }
            KBinaryOp::Eq => {
                // don't apply binary_op_helper
                asms.push(inst::xor_instruction(
                    lhs_reg, lhs_reg, rhs_reg, context, id,
                ));
                asms.push(inst::seqz_instruction(lhs_reg, lhs_reg, context, id));
            }
            _ => unimplemented!(),
        }

        asms
    }
}
