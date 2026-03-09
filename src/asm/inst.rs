use koopa::ir::Value;

use crate::asm::meta::{FunctionContext, Register, RegisterValue, RiscvAsm, RiscvInstruction};

pub fn label(label: &str) -> RiscvAsm {
    RiscvAsm::Label(label.to_string())
}

pub fn ret_instruction() -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::Ret)
}

pub fn li_instruction(
    dest: Register,
    imm: i32,
    context: &mut FunctionContext,
    id: Value,
) -> RiscvAsm {
    context.register_mapper.remove_by_register(dest);
    context
        .register_mapper
        .insert(RegisterValue::InstRet(id), dest);
    RiscvAsm::Instruction(RiscvInstruction::Li { dest, imm })
}

pub fn mv_instruction(
    dest: Register,
    src: Register,
    context: &mut FunctionContext,
    id: Value,
) -> RiscvAsm {
    context.register_mapper.remove_by_register(dest);
    context
        .register_mapper
        .insert(RegisterValue::InstRet(id), dest);
    RiscvAsm::Instruction(RiscvInstruction::Mv { dest, src })
}

pub fn sub_instruction(
    dest: Register,
    src1: Register,
    src2: Register,
    context: &mut FunctionContext,
    id: Value,
) -> RiscvAsm {
    context.register_mapper.remove_by_register(dest);
    context
        .register_mapper
        .insert(RegisterValue::InstRet(id), dest);
    RiscvAsm::Instruction(RiscvInstruction::Sub { dest, src1, src2 })
}

pub fn xor_instruction(
    dest: Register,
    src1: Register,
    src2: Register,
    context: &mut FunctionContext,
    id: Value,
) -> RiscvAsm {
    context.register_mapper.remove_by_register(dest);
    context
        .register_mapper
        .insert(RegisterValue::InstRet(id), dest);
    RiscvAsm::Instruction(RiscvInstruction::Xor { dest, src1, src2 })
}

pub fn seqz_instruction(
    dest: Register,
    src: Register,
    context: &mut FunctionContext,
    id: Value,
) -> RiscvAsm {
    context.register_mapper.remove_by_register(dest);
    context
        .register_mapper
        .insert(RegisterValue::InstRet(id), dest);
    RiscvAsm::Instruction(RiscvInstruction::Seqz { dest, src })
}
