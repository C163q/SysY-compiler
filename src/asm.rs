use koopa::ir::Program;

use crate::asm::meta::RiscvAsm;

pub mod block;
pub mod expr;
pub mod func;
pub mod inst;
pub mod meta;

pub fn generate_instruction(program: &Program) -> Vec<RiscvAsm> {
    let mut instructions = vec![RiscvAsm::Section(meta::TEXT_SECTION.to_string())];
    instructions.extend(func::register_global_func(program));
    instructions.extend(func::generate_funcs(program));

    instructions
}

pub fn generate_asm(program: &Program) -> Vec<RiscvAsm> {
    generate_instruction(program)
}
