use crate::asm::meta::{self, Register};

pub fn label(label: &str) -> String {
    format!("{}:", label)
}

pub fn ret_instruction() -> String {
    format!("{}ret", meta::INDENT)
}

pub fn li_instruction(dest: Register, imm: i32) -> String {
    format!("{}li {}, {}", meta::INDENT, dest.name(), imm)
}
