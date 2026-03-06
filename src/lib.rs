pub mod parse;
pub mod ir;

use std::{fs, path::Path};

use crate::parse::Ast;

pub fn read_and_parse(path: &Path) -> anyhow::Result<Ast> {
    let input = fs::read_to_string(path)?;
    let ast = parse::parse(&input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(ast)
}

pub fn parse_to_ir(ast: Ast) -> ir::Ast {
    ast.into()
}
