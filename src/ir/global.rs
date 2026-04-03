use koopa::ir::{
    Program, Type,
    builder::{GlobalInstBuilder, ValueBuilder},
};

use crate::{
    ir::{
        arr::{eval_array_dim, get_array_ty, normal_global_arr_to_aggregate, normalize_array},
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
            ast::Def::Array { ident, sizes } => {
                let sizes = eval_array_dim(&sizes, manager);
                let ty: Type = ty.into();
                let arr_ty = get_array_ty(ty.clone(), &sizes);

                let init = match def.init_val {
                    ast::InitVal::Expr(_) => {
                        panic!("Cannot initialize array constant with scalar initializer")
                    }
                    ast::InitVal::Array(arr) => {
                        let arr = normalize_array(arr, &sizes, ty);
                        let level = sizes.len();
                        let mut elems = vec![];
                        normal_global_arr_to_aggregate(&arr, level, &mut elems, program, manager);
                        program.new_value().aggregate(elems)
                    }
                    ast::InitVal::ZeroInit(_) => program.new_value().zero_init(arr_ty.clone()),
                };

                let value = program.new_value().global_alloc(init);
                program.set_value_name(value, Some(format!("@{}", ident)));
                manager
                    .define_const(ident, ConstValue::Array(value, arr_ty))
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
                    Some(init) => {
                        let init = match init {
                            ast::InitVal::Array(_) => panic!("Cannot initialize scalar variable with array initializer"),
                            ast::InitVal::Expr(expr) => expr.const_eval_i32(manager)
                                .expect("Initialization value for global variable must be a constant expression"),
                            ast::InitVal::ZeroInit(_) => 0,
                        };
                        program.new_value().integer(init)
                    }
                };
                let global = program.new_value().global_alloc(init);
                program.set_value_name(global, Some(format!("@{}", ident)));
                manager
                    .define_var(ident, global, ty.into())
                    .unwrap_or_else(|e| panic!("Error defining global variable: {}", e));
            }
            ast::Def::Array { ident, sizes } => {
                let sizes = eval_array_dim(&sizes, manager);
                let ty: Type = ty.into();
                let arr_ty = get_array_ty(ty.clone(), &sizes);

                let init = match def.init_val {
                    Some(init) => match init {
                        ast::InitVal::Expr(_) => {
                            panic!("Cannot initialize array constant with scalar initializer")
                        }
                        ast::InitVal::Array(arr) => {
                            let arr = normalize_array(arr, &sizes, ty);
                            let level = sizes.len();
                            let mut elems = vec![];
                            normal_global_arr_to_aggregate(
                                &arr, level, &mut elems, program, manager,
                            );
                            program.new_value().aggregate(elems)
                        }
                        ast::InitVal::ZeroInit(_) => program.new_value().zero_init(arr_ty.clone()),
                    },
                    None => program.new_value().zero_init(arr_ty.clone()),
                };
                let value = program.new_value().global_alloc(init);
                program.set_value_name(value, Some(format!("@{}", ident)));
                manager
                    .define_var(ident, value, arr_ty)
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
