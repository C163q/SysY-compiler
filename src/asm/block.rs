use koopa::ir::{BasicBlock, Value, ValueKind, entities::ValueData, layout::BasicBlockNode};

use crate::asm::meta::{FunctionContext, Register, RegisterValue, RiscvAsm, ToAsm};

impl ToAsm for ValueData {
    fn to_asm(&self, context: &mut FunctionContext, id: Value) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        match self.kind() {
            ValueKind::Integer(num) => {
                if num.value() != 0 {
                    asms.extend(num.to_asm(context, id));
                } else {
                    context
                        .register_mapper
                        .insert(RegisterValue::InstRet(id), Register::Zero);
                }
            }
            ValueKind::Return(expr) => {
                asms.extend(expr.to_asm(context, id));
            }
            ValueKind::Binary(bin) => {
                asms.extend(bin.to_asm(context, id));
            }
            ValueKind::Alloc(alloc) => {
                asms.extend(alloc.to_asm(context, id));
            }
            ValueKind::Load(load) => {
                asms.extend(load.to_asm(context, id));
            }
            ValueKind::Store(store) => {
                asms.extend(store.to_asm(context, id));
            }
            ValueKind::Jump(jump) => {
                asms.extend(jump.to_asm(context, id));
            }
            ValueKind::Branch(branch) => {
                asms.extend(branch.to_asm(context, id));
            }
            _ => unimplemented!(),
        }
        asms
    }
}

pub fn create_block(
    node: &BasicBlockNode,
    context: &mut FunctionContext,
    id: BasicBlock,
) -> Vec<RiscvAsm> {
    let mut asms = vec![];
    if id != context.block_labels.entry_id() {
        asms.push(RiscvAsm::Label(context.get_label(id)));
    }
    let insts = node.insts();
    for &insts in insts.keys() {
        let inst_data = context.func_data.dfg().value(insts);
        asms.extend(inst_data.to_asm(context, insts));
    }
    asms
}
