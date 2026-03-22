pub mod block;
pub mod expr;
pub mod func;
pub mod global;
pub mod meta;
pub mod stdlib;

use std::io;

use koopa::{
    back::{NameManager, Visitor},
    ir::Program,
};

use crate::{
    ir::meta::ConstValue,
    parse::{self, ast},
};

pub struct Ast {
    program: Program,
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
        }
    }

    pub fn get_ir(&self) -> io::Result<String> {
        let mut buf = Vec::new();
        let mut visitor = koopa::back::koopa::Visitor;
        visitor.visit(&mut buf, &mut NameManager::new(), &self.program)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

impl From<Program> for Ast {
    fn from(program: Program) -> Self {
        Self { program }
    }
}

impl From<parse::Ast> for Ast {
    fn from(ast: parse::Ast) -> Self {
        Self::from(parse_to_ir(ast))
    }
}

/// Koopa IR 中，最大的单位是 Program，它代表一个 Koopa IR 程序。
/// Program 由若干全局变量 (Value) 和函数 (Function) 构成。
/// Function 又由若干基本块 (BasicBlock) 构成，基本块中是一系列指令，指令也是 Value。
fn parse_to_ir(ast: parse::Ast) -> Program {
    let mut program = Program::new();
    let mut manager = meta::VariableManager::new();
    let components = ast::Components::new(ast.root);
    manager.new_scope();

    for decl in stdlib::get_function_decls() {
        let name = decl.name()[1..].to_string();
        let func = program.new_func(decl);
        manager
            .define_const(name, ConstValue::Function(func))
            .expect("Error defining function");
    }

    // 必须先注册然后再加载函数体的内容。
    // koopa 库的文档可能说得不太清楚，`Program`会有一个用于存储全局名称的HashMap，
    // 如果没有注册函数，则向`FunctionData`写入数据时，无法向全局写入名称。
    // 本质就是`Weak`无法指向正确的`Rc`导致`upgrade`后调用`unwrap`，然后`panic`了。
    for item in components.list {
        item.generate_ir(&mut program, &mut manager);
    }

    manager.exit_scope();

    program
}
