use koopa::ir::{
    BasicBlock, Function, FunctionData, Program, Type,
    builder::{BasicBlockBuilder, LocalInstBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{
        ConstValue, Instruction, IntoIr, ScopeGuard, Variable, VariableManager, last_flow,
        last_inst_vec,
    },
    parse::ast::{self, BType},
};

pub struct BlockFlow {
    pub block: BasicBlock,
    pub insts: Vec<Instruction>,
}

impl BlockFlow {
    pub fn new(block: BasicBlock, insts: Vec<Instruction>) -> Self {
        Self { block, insts }
    }

    pub fn block(&self) -> &BasicBlock {
        &self.block
    }

    pub fn values(&self) -> &[Instruction] {
        &self.insts
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

impl IntoIr for ast::BlockItem {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::BlockItem::Stmt(stmt) => stmt.into_ir(dfg, manager, flows),
            ast::BlockItem::Decl(decl) => decl.into_ir(dfg, manager, flows),
        }
    }
}

impl IntoIr for ast::Decl {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::Decl::Const(decl) => decl.into_ir(dfg, manager, flows),
            ast::Decl::Var(decl) => decl.into_ir(dfg, manager, flows),
        }
    }
}

impl IntoIr for ast::VarDecl {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let ty = self.ty;
        let defs = self.def;
        for def in defs {
            let vec = last_inst_vec(flows);
            let ident = def.ident;
            let init_val = def.init_val;
            let ty: Type = ty.into();
            let value = dfg.new_value().alloc(ty.clone());
            vec.push(Instruction::new(value, true));
            manager
                .define_var(ident, value, ty)
                .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
            if let Some(init_val) = init_val {
                init_val.expr.into_ir(dfg, manager, flows);
                let src = *last_flow(flows)
                    .insts
                    .last()
                    .copied()
                    .expect("Initialization expression should produce at least one value")
                    .inst();
                let vec = last_inst_vec(flows);
                let store = dfg.new_value().store(src, value);
                vec.push(Instruction::new(store, true));
            }
        }
    }
}

impl IntoIr for ast::ConstDecl {
    fn into_ir(
        self,
        _dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        _flows: &mut Vec<BlockFlow>,
    ) {
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
    }
}

impl IntoIr for ast::Stmt {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            // return val;
            ast::Stmt::Return(expr) => {
                expr.into_ir(dfg, manager, flows);
                let vec = last_inst_vec(flows);
                let some_last = vec.last().copied();
                let ret = dfg.new_value().ret(some_last.map(|v| *v.inst()));
                vec.push(Instruction::new(ret, true));
            }
            // lval = expr;
            ast::Stmt::Assign(lval, expr) => {
                let var = manager
                    .get(&lval.ident)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", lval.ident));
                match var {
                    Variable::Const(_) => {
                        panic!("Cannot assign to constant variable: {}", lval.ident)
                    }
                    Variable::Var(var) => {
                        let var = var.clone();
                        expr.into_ir(dfg, manager, flows);
                        let vec = last_inst_vec(flows);
                        let src = *vec
                            .last()
                            .copied()
                            .expect("Assignment expression should produce at least one value")
                            .inst();
                        let dest = *var.value();
                        let store = dfg.new_value().store(src, dest);
                        vec.push(Instruction::new(store, true));
                    }
                }
            }
            // [expr];
            ast::Stmt::Expr(maybe_expr) => {
                if let Some(expr) = maybe_expr {
                    expr.into_ir(dfg, manager, flows);
                }
            }
            // { BlockItems,* }
            ast::Stmt::Block(block) => block.into_ir(dfg, manager, flows),
            ast::Stmt::If(_if_block) => unimplemented!(),
            ast::Stmt::Else(_) => unreachable!("Else should be handled in if_else_bind"),
            ast::Stmt::IfElse(_if_block, _else_block) => unimplemented!(),
        }
    }
}

impl IntoIr for ast::Block {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let mut guard = ScopeGuard::new(manager);
        for item in self.items {
            item.into_ir(dfg, guard.inner(), flows);
        }
    }
}

fn if_else_bind(block: ast::Block) -> Result<ast::Block, String> {
    let mut items = vec![];
    for item in block.items {
        match item {
            ast::BlockItem::Decl(_) => items.push(item),
            ast::BlockItem::Stmt(stmt) => match stmt {
                ast::Stmt::Return(_)
                | ast::Stmt::Assign(_, _)
                | ast::Stmt::Expr(_)
                | ast::Stmt::If(_) => items.push(ast::BlockItem::Stmt(stmt)),
                ast::Stmt::Else(else_block) => {
                    let last = items
                        .pop()
                        .ok_or_else(|| "Else cannot be the first statement".to_string())?;
                    if let ast::BlockItem::Stmt(ast::Stmt::If(if_block)) = last {
                        items.push(ast::BlockItem::Stmt(ast::Stmt::IfElse(
                            if_block, else_block,
                        )));
                    } else {
                        return Err("Else must follow an If statement".to_string());
                    }
                }
                ast::Stmt::IfElse(_, _) => {
                    unreachable!("IfElse should not appear in the original AST")
                }
                ast::Stmt::Block(block) => {
                    let new_block = if_else_bind(block)?;
                    items.push(ast::BlockItem::Stmt(ast::Stmt::Block(new_block)));
                }
            },
        }
    }
    Ok(ast::Block::new(items))
}

fn func_scope(mut scope: ast::Block, dfg: &mut DataFlowGraph) -> Vec<BlockFlow> {
    // Currently, we only support a single basic block for each function, so we can directly build
    // the entry block and VariableManager.
    let entry = {
        let block = dfg.new_bb().basic_block(Some("%entry".to_string()));
        BlockFlow::new(block, vec![])
    };

    let mut flows = vec![entry];

    let mut manager = VariableManager::new();
    let mut guard = ScopeGuard::new(&mut manager);

    scope = if_else_bind(scope)
        .unwrap_or_else(|e| panic!("Error in binding if-else statements: {}", e));
    for item in scope.items {
        item.into_ir(dfg, guard.inner(), &mut flows);
    }

    for insts in flows.iter_mut() {
        insts.insts = insts
            .insts
            .drain(..)
            .filter(|inst| inst.insert)
            .collect::<Vec<_>>();
    }

    flows
}

fn ast_to_func(func: ast::FuncDef, data: &mut FunctionData) {
    let flow = data.dfg_mut();
    let seq = func_scope(func.block, flow);

    data.layout_mut()
        .bbs_mut()
        .extend(seq.iter().map(|b| &b.block).copied());
    for bf in seq {
        data.layout_mut()
            .bb_mut(bf.block)
            .insts_mut()
            .extend(bf.insts.into_iter().map(|i| i.inst));
    }
}
