use std::sync::atomic::Ordering;

use koopa::ir::{
    BasicBlock, Value, ValueKind, entities::ValueData, layout::BasicBlockNode, values::FuncArgRef,
};

use crate::asm::{
    expr,
    inst::{self, InstContext},
    meta::{self, FunctionContext, Register, RegisterValue, RiscvAsm, ToAsm},
};

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
            ValueKind::FuncArgRef(arg_ref) => {
                asms.extend(arg_ref.to_asm(context, id));
            }
            ValueKind::Call(call) => {
                asms.extend(call.to_asm(context, id));
            }
            ValueKind::GetElemPtr(get_elem_ptr) => {
                asms.extend(get_elem_ptr.to_asm(context, id));
            }
            ValueKind::GetPtr(get_ptr) => {
                asms.extend(get_ptr.to_asm(context, id));
            }
            ValueKind::ZeroInit(_) => {
                context
                    .register_mapper
                    .insert(RegisterValue::InstRet(id), Register::Zero);
            }
            _ => unimplemented!("Value kind {:?} is not implemented yet", self.kind()),
        }
        asms
    }
}

impl ToAsm for FuncArgRef {
    fn to_asm(&self, context: &mut FunctionContext, id: Value) -> Vec<RiscvAsm> {
        let size = context.func_data.dfg().value(id).ty().size();
        let location = context
            .memory_mapper
            .get_arg(self, size as meta::RV32Usize)
            .expect("Failed to get function argument location, it may not exist");
        match location {
            meta::ArgLocation::Register(reg) => {
                context.register_mapper.remove_by_register(reg);
                context
                    .register_mapper
                    .insert(RegisterValue::InstRet(id), reg);
                vec![]
            }
            meta::ArgLocation::Stack(offset) => {
                let reg = expr::obtain_caller_directly_usable_register(context);
                inst::add_lw_instruction(
                    reg,
                    Register::Sp,
                    meta::RV32Imm::new(offset as i32),
                    Some(InstContext::new(context, id)),
                    Some(reg),
                )
            }
        }
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
        if meta::ASM_SHOW_IR.load(Ordering::Acquire) {
            asms.push(RiscvAsm::Comment(meta::value_type_to_string(
                inst_data.kind(),
            )));
        }
        asms.extend(inst_data.to_asm(context, insts));
    }
    asms
}
