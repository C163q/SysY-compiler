use koopa::ir::Value;

use crate::asm::meta::{FunctionContext, Register, RegisterValue, RiscvAsm, RiscvInstruction};

fn register_dest(dest: Register, context: &mut FunctionContext, id: Option<Value>) {
    if let Some(id) = id {
        context.register_mapper.remove_by_register(dest);
        context
            .register_mapper
            .insert(RegisterValue::InstRet(id), dest);
    }
}

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
    id: Option<Value>,
) -> RiscvAsm {
    register_dest(dest, context, id);
    RiscvAsm::Instruction(RiscvInstruction::Li { dest, imm })
}

pub fn mv_instruction(
    dest: Register,
    src: Register,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> RiscvAsm {
    register_dest(dest, context, id);
    RiscvAsm::Instruction(RiscvInstruction::Mv { dest, src })
}

macro_rules! binary_instruction {
    ($dest:expr, $src1:expr, $src2:expr, $context:expr, $id:expr, $variant:tt) => {
        use crate::asm::meta::{RiscvAsm, RiscvInstruction};
        register_dest($dest, $context, $id);
        return RiscvAsm::Instruction(RiscvInstruction::$variant {
            dest: $dest,
            src1: $src1,
            src2: $src2,
        });
    };
}

macro_rules! define_binary_instruction {
    ($name:ident, $variant:tt) => {
        pub fn $name(
            dest: Register,
            src1: Register,
            src2: Register,
            context: &mut FunctionContext,
            id: Option<Value>,
        ) -> RiscvAsm {
            binary_instruction!(dest, src1, src2, context, id, $variant);
        }
    };
}

define_binary_instruction!(add_instruction, Add);
define_binary_instruction!(sub_instruction, Sub);
define_binary_instruction!(mul_instruction, Mul);
define_binary_instruction!(div_instruction, Div);
define_binary_instruction!(rem_instruction, Mod);
define_binary_instruction!(and_instruction, And);
define_binary_instruction!(or_instruction, Or);
define_binary_instruction!(xor_instruction, Xor);
define_binary_instruction!(slt_instruction, Slt);
define_binary_instruction!(sgt_instruction, Sgt);

pub fn seqz_instruction(
    dest: Register,
    src: Register,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> RiscvAsm {
    register_dest(dest, context, id);
    RiscvAsm::Instruction(RiscvInstruction::Seqz { dest, src })
}

pub fn snez_instruction(
    dest: Register,
    src: Register,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> RiscvAsm {
    register_dest(dest, context, id);
    RiscvAsm::Instruction(RiscvInstruction::Snez { dest, src })
}
