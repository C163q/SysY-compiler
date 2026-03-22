use koopa::ir::{
    Program, Type,
    builder::{GlobalInstBuilder, ValueBuilder},
};

use crate::{
    ir::{
        block,
        meta::{ConstValue, IntoIr, VariableManager},
    },
    parse::ast::{self, BType},
};

fn global_decl_const_ir(
    decl: ast::ConstDecl,
    program: &mut Program,
    manager: &mut VariableManager,
) {
    let ty = decl.ty;
    let defs = decl.def;
    assert!(
        matches!(ty, BType::Int),
        "Cannot define global constant with unsupported type"
    );

    for def in defs {
        let definition = def.definition;
        match definition {
            ast::Def::Ident { ident } => {
                block::define_simple_const(ident, ty, def.init_val, manager)
            }
            ast::Def::Array { ident, size } => {
                let size: usize = size.eval_usize(manager);

                let init = match def.init_val {
                    ast::ConstInitVal::Expr(_) => {
                        panic!("Cannot initialize array constant with scalar initializer")
                    }
                    ast::ConstInitVal::Array(arr) => {
                        let arr_size = arr.len();
                        let mut elems: Vec<_> = arr.into_iter()
                            .map(|e|
                                program.new_value().integer(
                                    e.const_eval_i32(manager)
                                    .expect("Initialization expression for array constant must be a constant expression")
                                )
                            ).collect();
                        for _ in arr_size..size {
                            elems.push(program.new_value().integer(0));
                        }
                        program.new_value().aggregate(elems)
                    }
                };

                let value = program.new_value().global_alloc(init);
                program.set_value_name(value, Some(format!("@{}", ident)));
                manager
                    .define_const(ident, ConstValue::Array(value))
                    .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
            }
        };
    }
}

fn global_decl_var_ir(decl: ast::VarDecl, program: &mut Program, manager: &mut VariableManager) {
    let defs = decl.def;
    let ty = decl.ty;
    assert!(
        matches!(ty, BType::Int),
        "Cannot define global variable with unsupported type"
    );

    for def in defs {
        let definition = def.definition;
        match definition {
            ast::Def::Ident { ident } => {
                let init = match def.init_val {
                    None => program.new_value().zero_init(ty.into()),
                    Some(init) => match ty {
                        BType::Int => {
                            let init = match init {
                                ast::InitVal::Array(_) => panic!("Cannot initialize scalar variable with array initializer"),
                                ast::InitVal::Expr(expr) => expr.const_eval_i32(manager)
                                    .expect("Initialization value for global variable must be a constant expression"),
                            };
                            program.new_value().integer(init)
                        }
                        BType::Void => {
                            panic!("Void type cannot be used for global variable '{}'", ident)
                        }
                    },
                };
                let global = program.new_value().global_alloc(init);
                program.set_value_name(global, Some(format!("@{}", ident)));
                manager
                    .define_var(ident, global, ty.into())
                    .unwrap_or_else(|e| panic!("Error defining global variable: {}", e));
            }
            ast::Def::Array { ident, size } => {
                let size: usize = size.eval_usize(manager);
                let init = match def.init_val {
                    None => {
                        let arr_ty = Type::get_array(ty.into(), size);
                        program.new_value().zero_init(arr_ty)
                    }
                    Some(init) => match init {
                        ast::InitVal::Expr(_) => {
                            panic!("Cannot initialize array constant with scalar initializer")
                        }
                        ast::InitVal::Array(arr) => {
                            let arr_size = arr.len();
                            let mut elems: Vec<_> = arr.into_iter()
                            .map(|e|
                                program.new_value().integer(
                                    e.const_eval_i32(manager)
                                    .expect("Initialization expression for array constant must be a constant expression")
                                )
                            ).collect();
                            for _ in arr_size..size {
                                elems.push(program.new_value().integer(0));
                            }
                            program.new_value().aggregate(elems)
                        }
                    },
                };

                let value = program.new_value().global_alloc(init);
                program.set_value_name(value, Some(format!("@{}", ident)));
                manager
                    .define_const(ident, ConstValue::Array(value))
                    .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
            }
        }
    }
}

impl ast::GlobalItem {
    pub fn generate_ir(self, program: &mut Program, manager: &mut VariableManager) {
        match self {
            ast::GlobalItem::FuncDef(func_def) => {
                let func = func_def.register_func(program, manager);
                func_def.generate_ir(program.func_mut(func), manager);
            }
            ast::GlobalItem::Decl(decl) => match decl {
                ast::Decl::Const(decl) => global_decl_const_ir(decl, program, manager),
                ast::Decl::Var(decl) => global_decl_var_ir(decl, program, manager),
            },
        }
    }
}
