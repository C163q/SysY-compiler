use koopa::ir::{
    BinaryOp,
    builder::{LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{ConstValue, Instruction, IntoIr, Variable, VariableManager},
    parse::ast,
};

impl IntoIr for ast::Expr {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        self.expr.into_ir(dfg, manager)
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        self.expr.const_eval_i32(manager)
    }
}

/// 通过左侧表达式和右侧表达式的IR生成二元表达式的IR。
fn binary_op_helper(
    op: BinaryOp,
    lhs_val: Vec<Instruction>,
    rhs_val: Vec<Instruction>,
    dfg: &mut DataFlowGraph,
) -> Vec<Instruction> {
    let mut vec = vec![];
    let comp = dfg.new_value().binary(
        op,
        // 左侧表达式和右侧表达式最后一个指令的结果分别为两者的值。
        *lhs_val
            .last()
            .copied()
            .expect("AddExpr expect a value")
            .inst(),
        *rhs_val
            .last()
            .copied()
            .expect("AddExpr expect a value")
            .inst(),
    );
    vec.extend(lhs_val);
    vec.extend(rhs_val);
    vec.push(Instruction::new(comp, true));
    vec
}

/// 针对不同二元运算符求取左右表达式的IR并生成二元表达式的IR。
macro_rules! impl_into_ir_for_binary_expr {
    ($expr_ty:tt, $next_level:tt, $op_ty:tt; $( $op:tt ),*) => {
        fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
            match self {
                ast::$expr_ty::$next_level(expr) => expr.into_ir(dfg, manager),
                ast::$expr_ty::Binary(lhs, op, rhs) => match op {
                    $(
                        ast::$op_ty::$op => {
                            binary_op_helper(BinaryOp::$op, lhs.into_ir(dfg, manager), rhs.into_ir(dfg, manager), dfg)
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
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::LAndExpr::Eq(expr) => expr.into_ir(dfg, manager),
            ast::LAndExpr::Binary(lhs, rhs) => {
                let mut vec = vec![];
                let lhs_val = lhs.into_ir(dfg, manager);
                let rhs_val = rhs.into_ir(dfg, manager);
                let lhs = *lhs_val
                    .last()
                    .copied()
                    .expect("LAndExpr expect a value")
                    .inst();
                let rhs = *rhs_val
                    .last()
                    .copied()
                    .expect("LAndExpr expect a value")
                    .inst();

                // lhs && rhs == (lhs != 0) & (rhs != 0)
                let zero = dfg.new_value().integer(0);
                let lhs_comp = dfg.new_value().binary(BinaryOp::NotEq, lhs, zero);
                let rhs_comp = dfg.new_value().binary(BinaryOp::NotEq, rhs, zero);
                let comp = dfg.new_value().binary(BinaryOp::And, lhs_comp, rhs_comp);
                vec.extend(lhs_val);
                vec.extend(rhs_val);
                vec.extend(vec![
                    // Zero is not an actual IR, but we need it to compare with lhs and rhs.
                    Instruction::new(zero, false),
                    Instruction::new(lhs_comp, true),
                    Instruction::new(rhs_comp, true),
                    Instruction::new(comp, true),
                ]);
                vec
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
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::LOrExpr::And(expr) => expr.into_ir(dfg, manager),
            ast::LOrExpr::Binary(lhs, rhs) => {
                let mut vec = vec![];
                let lhs_val = lhs.into_ir(dfg, manager);
                let rhs_val = rhs.into_ir(dfg, manager);
                let lhs = *lhs_val
                    .last()
                    .copied()
                    .expect("LAndExpr expect a value")
                    .inst();
                let rhs = *rhs_val
                    .last()
                    .copied()
                    .expect("LAndExpr expect a value")
                    .inst();

                // lhs || rhs == (lhs | rhs) != 0
                let zero = dfg.new_value().integer(0);
                let or = dfg.new_value().binary(BinaryOp::Or, lhs, rhs);
                let comp = dfg.new_value().binary(BinaryOp::NotEq, or, zero);
                vec.extend(lhs_val);
                vec.extend(rhs_val);
                vec.extend(vec![
                    // Zero is not an actual IR, but we need it to compare with lhs and rhs.
                    Instruction::new(zero, false),
                    Instruction::new(or, true),
                    Instruction::new(comp, true),
                ]);
                vec
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
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::UnaryExpr::Primary(expr) => expr.into_ir(dfg, manager),
            ast::UnaryExpr::UnaryOp(op, expr) => match op {
                ast::UnaryOp::Pos => expr.into_ir(dfg, manager),
                ast::UnaryOp::Neg => {
                    let mut vec = vec![];
                    let val = expr.into_ir(dfg, manager);
                    let zero = dfg.new_value().integer(0);
                    let comp = dfg.new_value().binary(
                        BinaryOp::Sub,
                        zero,
                        *val.last()
                            .copied()
                            .expect("UnaryExpr expect a value")
                            .inst(),
                    );
                    vec.extend(val);
                    vec.push(Instruction::new(comp, true));
                    vec
                }
                ast::UnaryOp::Not => {
                    let mut vec = vec![];
                    let val = expr.into_ir(dfg, manager);
                    let zero = dfg.new_value().integer(0);
                    let comp = dfg.new_value().binary(
                        BinaryOp::Eq,
                        *val.last()
                            .copied()
                            .expect("UnaryExpr expect a value")
                            .inst(),
                        zero,
                    );
                    vec.extend(val);
                    vec.push(Instruction::new(comp, true));
                    vec
                }
            },
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
        }
    }
}

impl IntoIr for ast::PrimaryExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::PrimaryExpr::Expr(boxed_expr) => boxed_expr.into_ir(dfg, manager),
            ast::PrimaryExpr::Num(num) => num.into_ir(dfg, manager),
            ast::PrimaryExpr::LVal(lval) => lval.into_ir(dfg, manager),
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

impl IntoIr for ast::LVal {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match manager.get(&self.ident) {
            Some(var) => match var {
                // 若为常量，直接取得其常量值且不产生对应IR。
                Variable::Const(val) => match val {
                    ConstValue::Int(val) => {
                        vec![Instruction::new(dfg.new_value().integer(*val), false)]
                    }
                },
                // 若为变量，产生load指令来取得其值。
                Variable::Var(var) => {
                    let load = dfg.new_value().load(*var.value());
                    vec![Instruction::new(load, true)]
                }
            },
            None => panic!("Variable '{}' not defined", self.ident),
        }
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        match manager.get(&self.ident) {
            Some(var) => match var {
                Variable::Const(val) => match val {
                    ConstValue::Int(val) => Some(*val),
                },
                // 变量不允许在编译期求值。
                Variable::Var(_) => None,
            },
            None => None,
        }
    }
}

impl IntoIr for ast::ConstInitVal {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        self.expr.into_ir(dfg, manager)
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        self.expr.const_eval_i32(manager)
    }
}

impl IntoIr for ast::ConstExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        self.expr.into_ir(dfg, manager)
    }

    fn const_eval_i32(&self, manager: &VariableManager) -> Option<i32> {
        self.expr.const_eval_i32(manager)
    }
}

impl IntoIr for ast::Number {
    fn into_ir(self, dfg: &mut DataFlowGraph, _: &mut VariableManager) -> Vec<Instruction> {
        vec![Instruction::new(
            dfg.new_value().integer(self.get_val()),
            false,
        )]
    }

    fn const_eval_i32(&self, _: &VariableManager) -> Option<i32> {
        Some(self.get_val())
    }
}
