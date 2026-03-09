use koopa::ir::{Value, ValueKind, entities::ValueData, layout::BasicBlockNode};

use crate::asm::meta::{FunctionContext, Register, RegisterValue, RiscvAsm, ToAsm};

impl ToAsm for ValueData {
    fn to_asm(&self, context: Option<&mut FunctionContext>, id: Option<Value>) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for ValueData");
        let id = id.expect("Value not found for ValueData");
        let mut asms = vec![];
        match self.kind() {
            ValueKind::Integer(num) => {
                if num.value() != 0 {
                    asms.extend(num.to_asm(Some(context), Some(id)));
                } else {
                    context.register_mapper.insert(RegisterValue::InstRet(id), Register::Zero);
                }
            }
            ValueKind::Return(expr) => {
                asms.extend(expr.to_asm(Some(context), Some(id)));
            }
            ValueKind::Binary(bin) => {
                asms.extend(bin.to_asm(Some(context), Some(id)));
            }
            _ => unimplemented!(),
        }
        asms
    }
}

impl ToAsm for BasicBlockNode {
    fn to_asm(&self, context: Option<&mut FunctionContext>, _: Option<Value>) -> Vec<RiscvAsm> {
        let context = context.expect("FunctionContext not found for BasicBlockNode");
        let mut asms = vec![];
        let instructions = self.insts();
        for &inst in instructions.keys() {
            let inst_data = context.func_data.dfg().value(inst);
            asms.extend(inst_data.to_asm(Some(context), Some(inst)));
        }
        asms
    }
}
