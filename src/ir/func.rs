use koopa::ir::{
    BasicBlock, Function, FunctionData, Program, Value,
    builder::{BasicBlockBuilder, LocalInstBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{Instruction, IntoIr},
    parse::ast,
};

pub struct BlockFlow {
    block: BasicBlock,
    values: Vec<Value>,
}

impl BlockFlow {
    pub fn new(block: BasicBlock, values: Vec<Value>) -> Self {
        Self { block, values }
    }

    pub fn block(&self) -> &BasicBlock {
        &self.block
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }
}

impl ast::FuncDef {
    /// Add a new function in program without parsing its body.
    pub fn register_func(&self, program: &mut Program) -> Function {
        let data = {
            let ret_type = self.func_type.clone().into();
            let func_name = format!("@{}", self.ident);
            FunctionData::with_param_names(func_name, vec![], ret_type)
        };
        program.new_func(data)
    }

    /// parsing function body into IR.
    ///
    /// NOTE: function MUST be registered first.
    pub fn load_data(self, data: &mut FunctionData) {
        ast_to_func(self, data);
    }
}

impl ast::Block {
    pub fn into_func_ir(self, dfg: &mut DataFlowGraph) -> Vec<BlockFlow> {
        build_blocks(self, dfg)
    }
}

impl IntoIr for ast::Stmt {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction> {
        #[allow(clippy::match_single_binding)]
        match self {
            ast::Stmt::Return(expr) => {
                let mut vec = vec![];
                let expr_values = expr.into_ir(dfg);
                let some_last = expr_values.last().copied();
                vec.extend(expr_values);
                let ret = dfg.new_value().ret(some_last.map(|v| *v.inst()));
                vec.push(Instruction::new(ret, true));
                vec
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

fn build_blocks(block: ast::Block, dfg: &mut DataFlowGraph) -> Vec<BlockFlow> {
    let mut entry = {
        let block = dfg.new_bb().basic_block(Some("%entry".to_string()));
        BlockFlow::new(block, vec![])
    };
    block.stmt.into_iter().for_each(|stmt| {
        let values = stmt.into_ir(dfg);
        entry.values.extend(
            values
                .into_iter()
                .filter_map(|v| v.insert().then_some(*v.inst())),
        );
    });

    vec![entry]
}

fn ast_to_func(func: ast::FuncDef, data: &mut FunctionData) {
    let flow = data.dfg_mut();
    let seq = func.block.into_func_ir(flow);

    data.layout_mut()
        .bbs_mut()
        .extend(seq.iter().map(|b| &b.block).copied());
    for bf in seq {
        data.layout_mut()
            .bb_mut(bf.block)
            .insts_mut()
            .extend(bf.values);
    }
}
