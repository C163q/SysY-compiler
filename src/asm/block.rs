use koopa::ir::{ValueKind, entities::ValueData, layout::BasicBlockNode};

use crate::asm::{
    inst,
    meta::{FunctionContext, Register, ToAsm},
};

impl ToAsm for ValueData {
    fn to_asm(&self, context: Option<&mut FunctionContext>) -> Vec<String> {
        let context = context.expect("FunctionData not found for ValueData");
        let mut asms = vec![];
        match self.kind() {
            ValueKind::Integer(_) => {}
            ValueKind::Return(expr) => {
                if let Some(val) = expr.value() {
                    let value_data = context.func_data.dfg().value(val);
                    match value_data.kind() {
                        ValueKind::Integer(i) => {
                            asms.push(inst::li_instruction(Register::A0, i.value()));
                        }
                        _ => unimplemented!(),
                    }
                    asms.push(inst::ret_instruction());
                }
            }
            _ => unimplemented!(),
        }
        asms
    }
}

impl ToAsm for BasicBlockNode {
    fn to_asm(&self, context: Option<&mut FunctionContext>) -> Vec<String> {
        let context = context.expect("FunctionData not found for BasicBlockNode");
        let mut asms = vec![];
        let instructions = self.insts();
        for &inst in instructions.keys() {
            let inst_data = context.func_data.dfg().value(inst);
            asms.extend(inst_data.to_asm(Some(context)));
        }
        asms
    }
}
