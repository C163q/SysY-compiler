use koopa::ir::{
    BinaryOp, Type, TypeKind,
    builder::{LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{
        BlockFlow, ConstValue, Instruction, IntoIr, Variable, VariableManager, last_inst_vec,
        last_inst_vec_value,
    },
    parse::ast::{self, EqExpr},
};

impl IntoIr for ast::Expr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        self.expr.into_ir(dfg, manager, flows)
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        self.expr.const_eval_i32(manager)
    }
}

/// 通过左侧表达式和右侧表达式的IR生成二元表达式的IR。
fn binary_op_helper<L: IntoIr, R: IntoIr>(
    op: BinaryOp,
    lhs: L,
    rhs: R,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    // 左侧表达式和右侧表达式最后一个指令的结果分别为两者的值。
    lhs.into_ir(dfg, manager, flows);
    let lhs_val = *last_inst_vec(flows)
        .last()
        .copied()
        .expect("BinaryExpr expect a value")
        .inst();
    rhs.into_ir(dfg, manager, flows);
    let rhs_val = *last_inst_vec(flows)
        .last()
        .copied()
        .expect("BinaryExpr expect a value")
        .inst();

    let comp = dfg.new_value().binary(op, lhs_val, rhs_val);
    let vec = last_inst_vec(flows);
    vec.push(Instruction::new(comp, true));
}

/// 针对不同二元运算符求取左右表达式的IR并生成二元表达式的IR。
macro_rules! impl_into_ir_for_binary_expr {
    ($expr_ty:tt, $next_level:tt, $op_ty:tt; $( $op:tt ),*) => {
        fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager, flows: &mut Vec<BlockFlow>) {
            match self {
                ast::$expr_ty::$next_level(expr) => expr.into_ir(dfg, manager, flows),
                ast::$expr_ty::Binary(lhs, op, rhs) => match op {
                    $(
                        ast::$op_ty::$op => {
                            binary_op_helper(BinaryOp::$op, *lhs, *rhs, dfg, manager, flows)
                        }
                    )*
                }
            }
        }
    };
    ($expr_ty:tt, $next_level:tt, $op:tt) => {
        fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
            match self {
                ast::$expr_ty::$next_level(expr) => expr.into_ir(dfg, manager),
                ast::$expr_ty::Binary(lhs, rhs) => {
                    binary_op_helper(BinaryOp::$op, lhs.into_ir(dfg, manager), rhs.into_ir(dfg, manager), dfg)
                }
            }
        }
    };
}

/// 若二元表达式的左右表达式都能在编译期求出i32值，则求出该二元表达式的i32值。
macro_rules! impl_const_eval_i32_for_binary_expr {
    ($expr_ty:tt, $next_level:tt, $op_ty:tt; $( $op:tt, $sym:tt ),*) => {
        fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
            match self {
                ast::$expr_ty::$next_level(expr) => expr.const_eval_i32(manager),
                ast::$expr_ty::Binary(lhs, op, rhs) => match op {
                    $(
                        ast::$op_ty::$op => {
                            let lhs_val = lhs.const_eval_i32(manager)?;
                            let rhs_val = rhs.const_eval_i32(manager)?;
                            Some((lhs_val $sym rhs_val) as i32)
                        }
                    )*
                }
            }
        }
    };
    ($expr_ty:tt, $next_level:tt, $op:tt) => {
        fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
            match self {
                ast::$expr_ty::$next_level(expr) => expr.const_eval_i32(manager),
                ast::$expr_ty::Binary(lhs, rhs) => {
                    let lhs_val = lhs.const_eval_i32(manager)?;
                    let rhs_val = rhs.const_eval_i32(manager)?;
                    Some((lhs_val $op rhs_val) as i32)
                }
            }
        }
    };
}

impl IntoIr for ast::EqExpr {
    impl_into_ir_for_binary_expr!(EqExpr, Rel, EqOp; Eq, NotEq);
    impl_const_eval_i32_for_binary_expr!(EqExpr, Rel, EqOp; Eq, ==, NotEq, !=);
}
impl IntoIr for ast::RelExpr {
    impl_into_ir_for_binary_expr!(RelExpr, Add, RelOp; Lt, Gt, Le, Ge);
    impl_const_eval_i32_for_binary_expr!(RelExpr, Add, RelOp; Lt, <, Gt, >, Le, <=, Ge, >=);
}
impl IntoIr for ast::AddExpr {
    impl_into_ir_for_binary_expr!(AddExpr, Mul, AddOp; Add, Sub);
    impl_const_eval_i32_for_binary_expr!(AddExpr, Mul, AddOp; Add, +, Sub, -);
}
impl IntoIr for ast::MulExpr {
    impl_into_ir_for_binary_expr!(MulExpr, Unary, MulOp; Mul, Div, Mod);
    impl_const_eval_i32_for_binary_expr!(MulExpr, Unary, MulOp; Mul, *, Div, /, Mod, %);
}

impl IntoIr for ast::LAndExpr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::LAndExpr::Eq(expr) => expr.into_ir(dfg, manager, flows),
            ast::LAndExpr::Binary(lhs, rhs) => {
                // int result = 0;
                // if (lhs != 0) {
                //   result = rhs != 0;
                // }

                let tmp_name = manager.unique_tmpname("land");

                let tmp_val = dfg.new_value().alloc(Type::get_i32());
                dfg.set_value_name(tmp_val, Some(tmp_name.clone()));
                manager
                    .define_var(tmp_name.clone(), tmp_val, Type::get_i32())
                    .expect("%tmp variable should not be defined");
                let zero = dfg.new_value().integer(0);
                let store_zero = dfg.new_value().store(zero, tmp_val);
                last_inst_vec(flows).extend([
                    Instruction::new(tmp_val, true),
                    Instruction::new(zero, false),
                    Instruction::new(store_zero, true),
                ]);

                ast::IfBranch::new(
                    ast::Expr::new_eq(ast::EqExpr::new_binary(
                        EqExpr::new_expr(ast::Expr::new_land(*lhs)),
                        ast::EqOp::NotEq,
                        ast::RelExpr::new_num(ast::Number::new(0)),
                    )),
                    ast::Stmt::Assign(
                        ast::LVal::new_ident(tmp_name.clone()),
                        ast::Expr::new_eq(ast::EqExpr::new_binary(
                            *rhs,
                            ast::EqOp::NotEq,
                            ast::RelExpr::new_num(ast::Number::new(0)),
                        )),
                    ),
                )
                .into_ir(dfg, manager, flows);

                let load = dfg.new_value().load(tmp_val);
                last_inst_vec(flows).push(Instruction::new(load, true));
            }
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::LAndExpr::Eq(expr) => expr.const_eval_i32(manager),
            ast::LAndExpr::Binary(lhs, rhs) => {
                let lhs_val = lhs.const_eval_i32(manager)?;
                let rhs_val = rhs.const_eval_i32(manager)?;
                Some(((lhs_val != 0) && (rhs_val != 0)) as i32)
            }
        }
    }
}

impl IntoIr for ast::LOrExpr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::LOrExpr::And(expr) => expr.into_ir(dfg, manager, flows),
            ast::LOrExpr::Binary(lhs, rhs) => {
                // int result = 1;
                // if (lhs == 0) {
                //   result = rhs != 0;
                // }

                let tmp_name = manager.unique_tmpname("lor");

                let tmp_val = dfg.new_value().alloc(Type::get_i32());
                dfg.set_value_name(tmp_val, Some(tmp_name.clone()));
                manager
                    .define_var(tmp_name.clone(), tmp_val, Type::get_i32())
                    .expect("tmp variable should not be defined");

                let one = dfg.new_value().integer(1);
                let store_one = dfg.new_value().store(one, tmp_val);
                last_inst_vec(flows).extend([
                    Instruction::new(tmp_val, true),
                    Instruction::new(one, false),
                    Instruction::new(store_one, true),
                ]);

                ast::IfBranch::new(
                    ast::Expr::new_eq(ast::EqExpr::new_binary(
                        EqExpr::new_expr(ast::Expr::new_lor(*lhs)),
                        ast::EqOp::Eq,
                        ast::RelExpr::new_num(ast::Number::new(0)),
                    )),
                    ast::Stmt::Assign(
                        ast::LVal::new_ident(tmp_name.clone()),
                        ast::Expr::new_eq(ast::EqExpr::new_binary(
                            ast::EqExpr::new_expr(ast::Expr::new_land(*rhs)),
                            ast::EqOp::NotEq,
                            ast::RelExpr::new_num(ast::Number::new(0)),
                        )),
                    ),
                )
                .into_ir(dfg, manager, flows);

                let load = dfg.new_value().load(tmp_val);
                last_inst_vec(flows).push(Instruction::new(load, true));
            }
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::LOrExpr::And(expr) => expr.const_eval_i32(manager),
            ast::LOrExpr::Binary(lhs, rhs) => {
                let lhs_val = lhs.const_eval_i32(manager)?;
                let rhs_val = rhs.const_eval_i32(manager)?;
                Some(((lhs_val != 0) || (rhs_val != 0)) as i32)
            }
        }
    }
}

impl IntoIr for ast::UnaryExpr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::UnaryExpr::Primary(expr) => expr.into_ir(dfg, manager, flows),
            ast::UnaryExpr::UnaryOp(op, expr) => match op {
                ast::UnaryOp::Pos => expr.into_ir(dfg, manager, flows),
                ast::UnaryOp::Neg => {
                    expr.into_ir(dfg, manager, flows);
                    let vec = last_inst_vec(flows);
                    let zero = dfg.new_value().integer(0);
                    let comp = dfg.new_value().binary(
                        BinaryOp::Sub,
                        zero,
                        *vec.last()
                            .copied()
                            .expect("UnaryExpr expect a value")
                            .inst(),
                    );
                    vec.push(Instruction::new(comp, true));
                }
                ast::UnaryOp::Not => {
                    expr.into_ir(dfg, manager, flows);
                    let vec = last_inst_vec(flows);
                    let zero = dfg.new_value().integer(0);
                    let comp = dfg.new_value().binary(
                        BinaryOp::Eq,
                        *vec.last()
                            .copied()
                            .expect("UnaryExpr expect a value")
                            .inst(),
                        zero,
                    );
                    vec.push(Instruction::new(comp, true));
                }
            },
            ast::UnaryExpr::Call(func_call) => {
                func_call.into_ir(dfg, manager, flows);
            }
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::UnaryExpr::Primary(expr) => expr.const_eval_i32(manager),
            ast::UnaryExpr::UnaryOp(op, expr) => {
                let val = expr.const_eval_i32(manager)?;
                match op {
                    ast::UnaryOp::Pos => Some(val),
                    ast::UnaryOp::Neg => Some(-val),
                    // !val in rust is not the same as in C, so we need to convert it to 0 or 1.
                    ast::UnaryOp::Not => Some((val == 0) as i32),
                }
            }
            ast::UnaryExpr::Call(_) => None,
        }
    }
}

impl IntoIr for ast::PrimaryExpr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::PrimaryExpr::Expr(boxed_expr) => boxed_expr.into_ir(dfg, manager, flows),
            ast::PrimaryExpr::Num(num) => num.into_ir(dfg, manager, flows),
            ast::PrimaryExpr::LVal(lval) => lval.into_ir(dfg, manager, flows),
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::PrimaryExpr::Expr(boxed_expr) => boxed_expr.const_eval_i32(manager),
            ast::PrimaryExpr::Num(num) => num.const_eval_i32(manager),
            ast::PrimaryExpr::LVal(lval) => lval.const_eval_i32(manager),
        }
    }
}

impl IntoIr for ast::FuncCall {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let func = match manager
            .get(&self.ident)
            .expect("Function should be defined")
        {
            Variable::Var(_) => panic!("'{}' is not a function", self.ident),
            Variable::Const(val) => match val {
                ConstValue::Int(_) | ConstValue::Array(_, _) => {
                    panic!("'{}' is not a function", self.ident)
                }
                ConstValue::Function(func) => *func,
            },
        };
        let args = self
            .args
            .unwrap_or(ast::FuncRParams::new(Vec::new()))
            .params
            .into_iter()
            .map(|arg| {
                arg.into_ir(dfg, manager, flows);
                *last_inst_vec(flows)
                    .last()
                    .copied()
                    .expect("FuncCall expect a value")
                    .inst()
            })
            .collect::<Vec<_>>();

        let call = dfg.new_value().call(func, args);
        last_inst_vec(flows).push(Instruction::new(call, true));
    }
}

fn ident_lval_ir(
    ident: String,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut [BlockFlow],
) {
    match manager.get(&ident) {
        Some(var) => match var {
            // Constant
            Variable::Const(val) => match val {
                // i32 can be constantly evaluated
                ConstValue::Int(val) => last_inst_vec(flows)
                    .push(Instruction::new(dfg.new_value().integer(*val), false)),
                ConstValue::Function(_) => {
                    panic!("Function '{}' cannot be used as a value", ident)
                }
                ConstValue::Array(value, _) => {
                    let zero = dfg.new_value().integer(0);
                    let load = dfg.new_value().get_elem_ptr(*value, zero);
                    last_inst_vec(flows)
                        .extend([Instruction::new(zero, false), Instruction::new(load, true)])
                }
            },
            // Variable
            Variable::Var(var) => {
                // If var is arr, `var.ty()` return Types like `[i32, size]`.
                // Note that the real type of var.value() is a pointer to the var.ty(). That is why
                // we need to use `load` or `get_elem_ptr`.
                let ty = var.ty().kind();
                match ty {
                    TypeKind::Array(_, _) => {
                        // For array variable, we need to get a pointer to the first element of the
                        // array, which is equivalent to the address of the array.
                        let zero = dfg.new_value().integer(0);
                        // *[i32, size] -> *i32
                        let load = dfg.new_value().get_elem_ptr(*var.value(), zero);
                        last_inst_vec(flows)
                            .extend([Instruction::new(zero, false), Instruction::new(load, true)])
                    }
                    TypeKind::Int32 | TypeKind::Pointer(_) => {
                        let load = dfg.new_value().load(*var.value());
                        last_inst_vec(flows).push(Instruction::new(load, true))
                    }
                    _ => panic!("Variable '{}' has unsupported type", ident),
                }
            }
        },
        None => panic!("Variable '{}' not defined", ident),
    }
}

fn array_lval_ir(
    ident: String,
    mut index: Vec<ast::Expr>,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    assert!(
        !index.is_empty(),
        "Array access must have at least one index"
    );
    match manager.get(&ident) {
        Some(var) => match var {
            Variable::Const(val) => match val {
                ConstValue::Int(_) | ConstValue::Function(_) => {
                    panic!("'{}' is not an array", ident)
                }
                ConstValue::Array(value, ty) => {
                    // @arr: *[[i32, 2], 3]
                    // arr[1] -> *[i32, 2]
                    let mut ptr = *value;
                    let mut ty = ty.clone();
                    for idx in index {
                        ty = match ty.kind() {
                            TypeKind::Array(elem_ty, _) => elem_ty.clone(),
                            TypeKind::Pointer(elem_ty) => elem_ty.clone(),
                            _ => panic!("Array or pointer type expected, but got {:?}", ty),
                        };
                        idx.into_ir(dfg, manager, flows);
                        let idx_val = last_inst_vec_value(flows);
                        ptr = dfg.new_value().get_elem_ptr(ptr, idx_val);
                        last_inst_vec(flows).push(Instruction::new(ptr, true));
                    }

                    match ty.kind() {
                        TypeKind::Int32 | TypeKind::Pointer(_) => {
                            let load = dfg.new_value().load(ptr);
                            last_inst_vec(flows).push(Instruction::new(load, true))
                        }
                        TypeKind::Array(_, _) => {
                            // If the result is still an array, we return a pointer to the first
                            // element of the array.
                            let zero = dfg.new_value().integer(0);
                            let load = dfg.new_value().get_elem_ptr(ptr, zero);
                            last_inst_vec(flows).extend([
                                Instruction::new(zero, false),
                                Instruction::new(load, true),
                            ])
                        }
                        _ => panic!("Array element has unsupported type: {:?}", ty),
                    }
                }
            },
            Variable::Var(var) => {
                let mut ptr = *var.value();
                let mut ty = var.ty().clone();
                if !ptr.is_global()
                    && let TypeKind::Pointer(p) = dfg.value(ptr).ty().kind()
                    && let TypeKind::Pointer(_) = p.kind()
                {
                    // Function parameters that are arrays are treated as pointers.
                    let first_idx = index.remove(0); // This should never panic.
                    first_idx.into_ir(dfg, manager, flows);
                    let idx_val = last_inst_vec_value(flows);
                    // Assume that array has type *i32 (decay of array type)
                    // The alloc IR at the beginning od the function produces a pointer of type **i32.
                    // So we need to load the pointer to get the actual array pointer of type *i32.
                    let load = dfg.new_value().load(ptr);
                    // Since we get the array pointer of type *i32, we cannot use get_elem_ptr
                    // because it requires a pointer to an array type. So we use get_ptr to make the
                    // offset calculation manually. The result is still a pointer of type *i32.
                    ptr = dfg.new_value().get_ptr(load, idx_val);
                    last_inst_vec(flows)
                        .extend([Instruction::new(load, true), Instruction::new(ptr, true)]);
                    ty = match ty.kind() {
                        TypeKind::Pointer(elem_ty) => elem_ty.clone(),
                        _ => panic!("Pointer type expected, but got {:?}", ty),
                    };
                }
                for idx in index {
                    ty = match ty.kind() {
                        TypeKind::Array(elem_ty, _) => elem_ty.clone(),
                        TypeKind::Pointer(elem_ty) => elem_ty.clone(),
                        _ => panic!("Array or pointer type expected, but got {:?}", ty),
                    };
                    idx.into_ir(dfg, manager, flows);
                    let idx_val = last_inst_vec_value(flows);
                    ptr = dfg.new_value().get_elem_ptr(ptr, idx_val);
                    last_inst_vec(flows).push(Instruction::new(ptr, true));
                }

                match ty.kind() {
                    TypeKind::Int32 | TypeKind::Pointer(_) => {
                        let load = dfg.new_value().load(ptr);
                        last_inst_vec(flows).push(Instruction::new(load, true))
                    }
                    TypeKind::Array(_, _) => {
                        // If the result is still an array, we return a pointer to the first
                        // element of the array.
                        let zero = dfg.new_value().integer(0);
                        ptr = dfg.new_value().get_elem_ptr(ptr, zero);
                        last_inst_vec(flows)
                            .extend([Instruction::new(zero, false), Instruction::new(ptr, true)]);
                    }
                    _ => panic!("Array element has unsupported type: {:?}", ty),
                }
            }
        },
        None => panic!("Variable '{}' not defined", ident),
    }
}

impl IntoIr for ast::LVal {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::LVal::Ident(ident) => ident_lval_ir(ident, dfg, manager, flows),
            ast::LVal::Array { ident, index } => array_lval_ir(ident, index, dfg, manager, flows),
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::LVal::Ident(ident) => match manager.get(ident) {
                Some(var) => match var {
                    Variable::Const(val) => match val {
                        ConstValue::Int(val) => Some(*val),
                        ConstValue::Function(_) | ConstValue::Array(_, _) => None,
                    },
                    // 变量不允许在编译期求值。
                    Variable::Var(_) => None,
                },
                None => None,
            },
            // 不支持编译期求值数组元素
            ast::LVal::Array { .. } => None,
        }
    }
}

impl IntoIr for ast::InitVal {
    fn into_ir(self, _: &mut DataFlowGraph, _: &mut VariableManager, _: &mut Vec<BlockFlow>) {
        panic!("ConstInitVal should not be handled here!");
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match self {
            ast::InitVal::Expr(val) => val.const_eval_i32(manager),
            ast::InitVal::Array(_) => None,
            ast::InitVal::ZeroInit(ty) => Some(0).filter(|_| matches!(ty.kind(), TypeKind::Int32)),
        }
    }
}

impl IntoIr for ast::InitExpr {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        self.expr.into_ir(dfg, manager, flows)
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        // FIXME: some non-constant expression may also be evaluated to a constant value
        self.expr.const_eval_i32(manager)
    }
}

impl ast::InitExpr {
    pub fn eval_usize(&self, manager: &VariableManager) -> usize {
        self.const_eval_i32(manager)
            .expect("Not a constant expression")
            .try_into()
            .expect("Array size must be non-negative")
    }
}

impl IntoIr for ast::Number {
    fn into_ir(self, dfg: &mut DataFlowGraph, _: &mut VariableManager, flows: &mut Vec<BlockFlow>) {
        last_inst_vec(flows).push(Instruction::new(
            dfg.new_value().integer(self.get_val()),
            false,
        ))
    }

    fn const_eval_i32(&self, _: &VariableManager) -> Option<i32> {
        Some(self.get_val())
    }
}
