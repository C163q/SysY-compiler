use koopa::ir::Program;

pub mod block;
pub mod func;
pub mod inst;
pub mod meta;

pub fn generate_instruction(program: &Program) -> Vec<String> {
    let mut instructions = vec![format!("{}{}", meta::INDENT, meta::TEXT_SECTION)];
    instructions.extend(func::register_global_func(program));
    instructions.extend(func::generate_funcs(program));

    instructions
}

pub fn generate_asm(program: &Program) -> Vec<String> {
    generate_instruction(program)
}
