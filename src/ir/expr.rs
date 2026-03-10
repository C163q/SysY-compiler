use koopa::ir::{
    BinaryOp,
    builder::{LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{Instruction, IntoIr},
    parse::ast,
};

impl IntoIr for ast::Expr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        self.expr.into_ir(dfg)
    }
}

fn binary_op_helper(
    op: BinaryOp,
    lhs_val: Vec<Instruction>,
    rhs_val: Vec<Instruction>,
    dfg: &mut DataFlowGraph,
) -> Vec<Instruction> {
    let mut vec = vec![];
    let comp = dfg.new_value().binary(
        op,
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

macro_rules! impl_into_ir_for_binary_expr {
    ($expr_ty:tt, $next_level:tt, $op_ty:tt, $op1:tt, $( $op:tt ),*) => {
        use ast::$expr_ty;
        use ast::$op_ty;
        impl IntoIr for $expr_ty {
            fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
                match self {
                    $expr_ty::$next_level(expr) => expr.into_ir(dfg),
                    $expr_ty::Binary(lhs, op, rhs) => match op {
                        $op_ty::$op1 => {
                            binary_op_helper(BinaryOp::$op1, lhs.into_ir(dfg), rhs.into_ir(dfg), dfg)
                        }
                        $(
                            $op_ty::$op => {
                                binary_op_helper(BinaryOp::$op, lhs.into_ir(dfg), rhs.into_ir(dfg), dfg)
                            }
                        )*
                    }
                }
            }
        }
    };
    ($expr_ty:tt, $next_level:tt, $op:tt) => {
        use ast::$expr_ty;
        impl IntoIr for $expr_ty {
            fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
                match self {
                    $expr_ty::$next_level(expr) => expr.into_ir(dfg),
                    $expr_ty::Binary(lhs, rhs) => {
                        binary_op_helper(BinaryOp::$op, lhs.into_ir(dfg), rhs.into_ir(dfg), dfg)
                    }
                }
            }
        }
    };
}

impl_into_ir_for_binary_expr!(EqExpr, Rel, EqOp, Eq, NotEq);
impl_into_ir_for_binary_expr!(RelExpr, Add, RelOp, Lt, Gt, Le, Ge);
impl_into_ir_for_binary_expr!(AddExpr, Mul, AddOp, Add, Sub);
impl_into_ir_for_binary_expr!(MulExpr, Unary, MulOp, Mul, Div, Mod);

impl IntoIr for ast::LAndExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        match self {
            ast::LAndExpr::Eq(expr) => expr.into_ir(dfg),
            ast::LAndExpr::Binary(lhs, rhs) => {
                let mut vec = vec![];
                let lhs_val = lhs.into_ir(dfg);
                let rhs_val = rhs.into_ir(dfg);
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

                // lhs && rhs == (lhs != 0) && (rhs != 0)
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
}

impl IntoIr for ast::LOrExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        match self {
            ast::LOrExpr::And(expr) => expr.into_ir(dfg),
            ast::LOrExpr::Binary(lhs, rhs) => {
                let mut vec = vec![];
                let lhs_val = lhs.into_ir(dfg);
                let rhs_val = rhs.into_ir(dfg);
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
}


impl IntoIr for ast::UnaryExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        match self {
            ast::UnaryExpr::Primary(expr) => expr.into_ir(dfg),
            ast::UnaryExpr::UnaryOp(op, expr) => match op {
                ast::UnaryOp::Pos => expr.into_ir(dfg),
                ast::UnaryOp::Neg => {
                    let mut vec = vec![];
                    let val = expr.into_ir(dfg);
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
                    let val = expr.into_ir(dfg);
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
}

impl IntoIr for ast::PrimaryExpr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        match self {
            ast::PrimaryExpr::Expr(boxed_expr) => boxed_expr.into_ir(dfg),
            ast::PrimaryExpr::Num(num) => num.into_ir(dfg),
        }
    }
}

impl IntoIr for ast::Number {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        vec![Instruction::new(
            dfg.new_value().integer(self.get_val()),
            false,
        )]
    }
}
