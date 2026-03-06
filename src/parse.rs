use std::fmt::{self, Display};

use lalrpop_util::{lalrpop_mod, lexer::Token};

pub mod ast;
pub mod types;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy, "/parse/sysy.rs");

#[derive(Debug, Clone)]
pub struct Ast {
    pub root: ast::CompUnit,
}

impl Ast {
    pub fn new(root: ast::CompUnit) -> Self {
        Self { root }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

pub fn parse<'input>(
    s: &'input str,
) -> Result<Ast, lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
    // 调用 lalrpop 生成的 parser 解析输入文件
    let unit = sysy::CompUnitParser::new().parse(s)?;
    Ok(Ast::new(unit))
}
