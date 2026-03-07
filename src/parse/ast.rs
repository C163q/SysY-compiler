use std::fmt::{self, Display};

use koopa::ir::Type;

use crate::parse::types;

#[derive(Debug, Clone)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

/// 文法标识符
impl CompUnit {
    pub fn new(func_def: FuncDef) -> Self {
        Self { func_def }
    }
}

impl Display for CompUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func_def)
    }
}

/// 函数的定义
///
/// ```c, ignore
/// // func_type  ident
/// //    ↓         ↓
///      int      main() {
///         // block
///      }
/// ```
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

impl FuncDef {
    pub fn new(func_type: FuncType, ident: String, block: Block) -> Self {
        Self {
            func_type,
            ident,
            block,
        }
    }
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}() {}", self.func_type, self.ident, self.block)
    }
}

/// 一个块由多条语句组成。
///
/// ```c, ignore
/// {   // block
///     int a = 0;  // Stmt
///     return a;   // Stmt
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Block {
    pub stmt: Vec<Stmt>,
}

impl Block {
    pub fn new(stmt: Vec<Stmt>) -> Self {
        Self { stmt }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for stmt in &self.stmt {
            write!(f, " {} ", stmt)?;
        }
        write!(f, "}}")
    }
}

/// 语句
///
/// ```c, ignore
/// return 0;   // Stmt
/// ```
/// Return(i32) <-  return 0;
#[derive(Debug, Clone)]
pub enum Stmt {
    Return(i32),
}

impl Stmt {
    pub fn new_return(val: i32) -> Self {
        Self::Return(val)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Return(val) => write!(f, "return {}", val),
        }
    }
}

/// 函数的返回类型
#[derive(Debug, Clone)]
pub struct FuncType {
    pub val: String,
}

impl FuncType {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

impl From<FuncType> for Type {
    fn from(func_type: FuncType) -> Self {
        types::get_type(&func_type.val)
    }
}

impl Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
