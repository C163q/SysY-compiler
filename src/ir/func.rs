use koopa::ir::{
    BasicBlock, Function, FunctionData, Program, Type, Value,
    builder::{BasicBlockBuilder, LocalInstBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{ConstValue, Instruction, IntoIr, Variable, VariableManager},
    parse::ast::{self, BType},
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
            let ret_type = self.ret_type.into();
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

impl IntoIr for ast::BlockItem {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::BlockItem::Stmt(stmt) => stmt.into_ir(dfg, manager),
            ast::BlockItem::Decl(decl) => decl.into_ir(dfg, manager),
        }
    }
}

impl IntoIr for ast::Decl {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            ast::Decl::Const(decl) => decl.into_ir(dfg, manager),
            ast::Decl::Var(decl) => decl.into_ir(dfg, manager),
        }
    }
}

impl IntoIr for ast::VarDecl {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        let ty = self.ty;
        let defs = self.def;
        let mut vec = vec![];
        for def in defs {
            let ident = def.ident;
            let init_val = def.init_val;
            let ty: Type = ty.into();
            let value = dfg.new_value().alloc(ty.clone());
            vec.push(Instruction::new(value, true));
            manager
                .define_var(ident, value, ty)
                .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
            if let Some(init_val) = init_val {
                let insts = init_val.expr.into_ir(dfg, manager);
                let src = *insts
                    .last()
                    .copied()
                    .expect("Initialization expression should produce at least one value")
                    .inst();
                vec.extend(insts);
                let store = dfg.new_value().store(src, value);
                vec.push(Instruction::new(store, true));
            }
        }
        vec
    }
}

impl IntoIr for ast::ConstDecl {
    fn into_ir(self, _dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        let ty = self.ty;
        let defs = self.def;
        for def in defs {
            let ident = def.ident;
            match ty {
                BType::Int => match def.init_val.const_eval_i32(manager) {
                    Some(value) => {
                        manager
                            .define_const(ident, ConstValue::Int(value))
                            .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
                    }
                    None => {
                        panic!(
                            "Initialization value for constant '{}' is not a constant expression",
                            ident
                        );
                    }
                },
            }
        }

        vec![]
    }
}

impl IntoIr for ast::Stmt {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction> {
        match self {
            // return val;
            ast::Stmt::Return(expr) => {
                let mut vec = vec![];
                let expr_values = expr.into_ir(dfg, manager);
                let some_last = expr_values.last().copied();
                vec.extend(expr_values);
                let ret = dfg.new_value().ret(some_last.map(|v| *v.inst()));
                vec.push(Instruction::new(ret, true));
                vec
            }
            // lval = expr;
            ast::Stmt::Assign(lval, expr) => {
                let mut vec = vec![];
                let var = manager
                    .get(&lval.ident)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", lval.ident));
                match var {
                    Variable::Const(_) => {
                        panic!("Cannot assign to constant variable: {}", lval.ident)
                    }
                    Variable::Var(var) => {
                        let var = var.clone();
                        let insts = expr.into_ir(dfg, manager);
                        let src = *insts
                            .last()
                            .copied()
                            .expect("Assignment expression should produce at least one value")
                            .inst();
                        vec.extend(insts);
                        let dest = *var.value();
                        let store = dfg.new_value().store(src, dest);
                        vec.push(Instruction::new(store, true));
                    }
                }
                vec
            }
        }
    }
}

fn build_blocks(block: ast::Block, dfg: &mut DataFlowGraph) -> Vec<BlockFlow> {
    // Currently, we only support a single basic block for each function, so we can directly build
    // the entry block and VariableManager.
    let mut entry = {
        let block = dfg.new_bb().basic_block(Some("%entry".to_string()));
        BlockFlow::new(block, vec![])
    };

    let mut manager = VariableManager::new();

    block.items.into_iter().for_each(|item| {
        let values = item.into_ir(dfg, &mut manager);
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
