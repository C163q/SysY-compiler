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
        match self {
            ast::Expr::Unary(expr) => expr.into_ir(dfg),
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
