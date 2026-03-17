use koopa::ir::{
    BasicBlock, Function, FunctionData, Program, Type, ValueKind,
    builder::{BasicBlockBuilder, LocalInstBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{
        ConstValue, Instruction, IntoIr, ScopeGuard, Variable, VariableManager, last_inst_vec,
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
    pub fn register_func(&self, program: &mut Program, manager: &mut VariableManager) -> Function {
        let data = {
            let ret_type = self.ret_type.into();
            let func_name = format!("@{}", self.ident);
            match self.fparams.clone() {
                None => FunctionData::with_param_names(func_name, vec![], ret_type),
                Some(fparams) => FunctionData::with_param_names(
                    func_name,
                    fparams
                        .params
                        .into_iter()
                        .map(|param| (Some(format!("@{}", param.ident)), param.ty.into()))
                        .inspect(|param: &(_, Type)| {
                            if param.1.is_unit() {
                                panic!(
                                    "Parameter '{}' cannot have void type",
                                    param.0.as_ref().unwrap()
                                )
                            }
                        })
                        .collect(),
                    ret_type,
                ),
            }
        };
        let value = program.new_func(data);
        manager
            .define_const(self.ident.clone(), ConstValue::Function(value))
            .expect("Error defining function");
        value
    }

    /// parsing function body into IR.
    ///
    /// NOTE: function MUST be registered first.
    pub fn generate_ir(self, data: &mut FunctionData, manager: &mut VariableManager) {
        ast_to_func(self, data, manager);
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
                let src = *last_inst_vec(flows)
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
                BType::Void => panic!("Void type cannot be used for constant '{}'", ident),
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
                let ret_val = match expr {
                    Some(expr) => {
                        expr.into_ir(dfg, manager, flows);
                        let vec = last_inst_vec(flows);
                        let last = vec
                            .last()
                            .copied()
                            .expect("Return expression should produce at least one value");
                        Some(*last.inst())
                    }
                    None => None,
                };
                let ret = dfg.new_value().ret(ret_val);
                last_inst_vec(flows).push(Instruction::new(ret, true));
                // Return means a basic block should end, we must push a new basic block for the
                // following statements. Otherwise, we may generate instructions after return
                // instructions, which is invalid.
                flows.push(BlockFlow::new(dfg.new_bb().basic_block(None), vec![]));
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
            ast::Stmt::If(if_block) => if_block.into_ir(dfg, manager, flows),
            ast::Stmt::Else(_) => unreachable!("Else should be handled in if_else_bind"),
            ast::Stmt::IfElse(if_block, else_block) => {
                (*if_block, *else_block).into_ir(dfg, manager, flows)
            }
            ast::Stmt::While(while_block) => while_block.into_ir(dfg, manager, flows),
            ast::Stmt::ControlFlow(control_flow) => control_flow.into_ir(dfg, manager, flows),
        }
    }
}

impl IntoIr for ast::IfBranch {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        // END declaration
        let end_block = dfg.new_bb().basic_block(None);

        // IF
        self.cond.into_ir(dfg, manager, flows);
        let cond_val = *last_inst_vec(flows)
            .last()
            .copied()
            .expect("Condition expression should produce at least one value")
            .inst();

        let mut if_flow = vec![];

        // THEN
        let then_block = dfg.new_bb().basic_block(None);
        let then_flow = BlockFlow::new(then_block, vec![]);
        {
            let mut guard = ScopeGuard::new(manager);
            if_flow.push(then_flow);

            self.stmt.into_ir(dfg, guard.inner(), &mut if_flow);
            last_inst_vec(&mut if_flow)
                .push(Instruction::new(dfg.new_value().jump(end_block), true));
        }

        // END
        let end_flow = BlockFlow::new(end_block, vec![]);
        last_inst_vec(flows).push(Instruction::new(
            dfg.new_value().branch(cond_val, then_block, end_block),
            true,
        ));
        if_flow.push(end_flow);

        flows.extend(if_flow);
    }
}

impl IntoIr for (ast::IfBranch, ast::ElseBranch) {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let (if_branch, else_branch) = self;

        // END declaration
        let end_block = dfg.new_bb().basic_block(None);

        // IF
        if_branch.cond.into_ir(dfg, manager, flows);
        let cond_val = *last_inst_vec(flows)
            .last()
            .copied()
            .expect("Condition expression should produce at least one value")
            .inst();

        let mut if_flow = vec![];

        let then_block = dfg.new_bb().basic_block(None);
        let then_flow = BlockFlow::new(then_block, vec![]);
        let else_block = dfg.new_bb().basic_block(None);
        let else_flow = BlockFlow::new(else_block, vec![]);

        {
            // THEN
            let mut guard = ScopeGuard::new(manager);
            if_flow.push(then_flow);

            if_branch.stmt.into_ir(dfg, guard.inner(), &mut if_flow);
            last_inst_vec(&mut if_flow)
                .push(Instruction::new(dfg.new_value().jump(end_block), true));

            // ELSE
            if_flow.push(else_flow);

            else_branch.stmt.into_ir(dfg, guard.inner(), &mut if_flow);
            last_inst_vec(&mut if_flow)
                .push(Instruction::new(dfg.new_value().jump(end_block), true));
        }

        // END
        let end_flow = BlockFlow::new(end_block, vec![]);
        last_inst_vec(flows).push(Instruction::new(
            dfg.new_value().branch(cond_val, then_block, else_block),
            true,
        ));
        if_flow.push(end_flow);

        flows.extend(if_flow);
    }
}

impl IntoIr for ast::WhileBranch {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let mut while_flow = vec![];

        // Basic block declaration
        let entry = dfg.new_bb().basic_block(None);
        let body = dfg.new_bb().basic_block(None);
        let end = dfg.new_bb().basic_block(None);

        // WHILE Condition
        last_inst_vec(flows).push(Instruction::new(dfg.new_value().jump(entry), true));
        let entry_flow = BlockFlow::new(entry, vec![]);
        while_flow.push(entry_flow);

        self.cond.into_ir(dfg, manager, &mut while_flow);
        let cond_val = *last_inst_vec(&mut while_flow)
            .last()
            .copied()
            .expect("Condition expression should produce at least one value")
            .inst();
        last_inst_vec(&mut while_flow).push(Instruction::new(
            dfg.new_value().branch(cond_val, body, end),
            true,
        ));

        // BODY
        manager.new_loop(entry, end);
        manager.new_scope();
        while_flow.push(BlockFlow::new(body, vec![]));
        self.stmt.into_ir(dfg, manager, &mut while_flow);
        last_inst_vec(&mut while_flow).push(Instruction::new(dfg.new_value().jump(entry), true));
        manager.exit_scope();
        manager.exit_loop();

        // END
        while_flow.push(BlockFlow::new(end, vec![]));
        flows.extend(while_flow);
    }
}

impl IntoIr for ast::ControlFlow {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        match self {
            ast::ControlFlow::Break => {
                let target = manager
                    .last_loop()
                    .expect("Break statement must be inside a loop")
                    .end();
                last_inst_vec(flows).push(Instruction::new(dfg.new_value().jump(target), true));
                flows.push(BlockFlow::new(dfg.new_bb().basic_block(None), vec![]));
            }
            ast::ControlFlow::Continue => {
                let target = manager
                    .last_loop()
                    .expect("Continue statement must be inside a loop")
                    .begin();
                last_inst_vec(flows).push(Instruction::new(dfg.new_value().jump(target), true));
                flows.push(BlockFlow::new(dfg.new_bb().basic_block(None), vec![]));
            }
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

fn bind_if_else(
    else_branch: Box<ast::ElseBranch>,
    stmt: ast::Stmt,
) -> Result<ast::Stmt, (ast::Stmt, Box<ast::ElseBranch>)> {
    // We get Ok if the else branch can be successfully bound to the if statement, and we get Err
    // if not.
    match stmt {
        ast::Stmt::Return(_)
        | ast::Stmt::Assign(_, _)
        | ast::Stmt::Expr(_)
        // Else statement cannot be bound to if statement in a different block.
        | ast::Stmt::Block(_)
        | ast::Stmt::ControlFlow(_) => Err((stmt, else_branch)),
        // see the comment of the `Else` branch in the `bind_if_else_stmt` function for the
        // motivation of the recursive search for the if statement.
        ast::Stmt::If(branch) => match bind_if_else(else_branch, branch.stmt) {
            // Else statement has been bound to the if statement. Just do nothing.
            Ok(sub_stmt) => Ok(ast::Stmt::new_if(ast::IfBranch::new(branch.cond, sub_stmt))),
            // Else statement is still not bound to the if statement, so the current if statement
            // is the most nested if statement we can bind with the else statement. So we can
            // return Ok.
            Err((stmt, else_branch)) => Ok(ast::Stmt::new_if_else(
                ast::IfBranch::new(branch.cond, stmt),
                *else_branch,
            )),
        },
        // Search for the if statement in the while branch.
        ast::Stmt::While(branch) => match bind_if_else(else_branch, branch.stmt) {
            Ok(sub_stmt) => Ok(ast::Stmt::new_while(ast::WhileBranch::new(branch.cond, sub_stmt))),
            Err((stmt, else_branch)) => Err((
                ast::Stmt::new_while(ast::WhileBranch::new(branch.cond, stmt)),
                else_branch
            )),
        },
        // We guarantee that else statements do not exist in `stmt` because they have been bound in
        // the previous call.
        ast::Stmt::Else(_) => panic!("Nested else statements are not allowed"),
        // This is necessary because `stmt` has been bound and `IfElse` is vaild.
        ast::Stmt::IfElse(if_branch, binded_else) => {
            match bind_if_else(else_branch, binded_else.stmt) {
                Ok(sub_stmt) => Ok(ast::Stmt::new_if_else(
                    *if_branch,
                    ast::ElseBranch::new(sub_stmt),
                )),
                Err((stmt, else_branch)) => Err((
                    ast::Stmt::new_if_else(*if_branch, ast::ElseBranch::new(stmt)),
                    else_branch,
                )),
            }
        }
    }
}

fn bind_if_else_stmt(stmt: ast::Stmt, vec: &mut Vec<ast::BlockItem>) -> Result<ast::Stmt, String> {
    match stmt {
        // For simple statements, we can directly return them without binding.
        ast::Stmt::Return(_)
        | ast::Stmt::Assign(_, _)
        | ast::Stmt::Expr(_)
        | ast::Stmt::ControlFlow(_) => Ok(stmt),
        // For block statements, we need to bind the if-else statements inside the block.
        // A else statement can only be bound to an if statement in the same block, so we can
        // directly return the new block after binding.
        ast::Stmt::Block(block) => {
            let new_block = bind_if_else_block(block)?;
            Ok(ast::Stmt::Block(new_block))
        }
        // For if statements, we need to bind the if-else statements inside the if branch.
        // We can't decide whether the if statement can be bound with an else statement until we
        // see the next statement, so we can directly return the new if statement after binding the
        // if statement.
        ast::Stmt::If(mut if_branch) => {
            if_branch.stmt = bind_if_else_stmt(if_branch.stmt, vec)?;
            Ok(ast::Stmt::If(if_branch))
        }
        ast::Stmt::While(mut while_branch) => {
            while_branch.stmt = bind_if_else_stmt(while_branch.stmt, vec)?;
            Ok(ast::Stmt::While(while_branch))
        }
        // For else statements, we need to bind the else statement with the previous if statement.
        // So we need to pop the last statement from the block and search for the nested if
        // statement to bind with the else statement. If we can't find any if statement, it means
        // the else statement is invalid.
        //
        // For example, in the following code:
        //
        // ```c
        // if (cond1)
        //   if (cond2)
        //     if (cond3) {
        //       if (cond4)
        //         a = 1;
        //     }
        // else a = 2;
        // ```
        //
        // Else should be bound to `if (cond3)`. But for our parser, the else statement will be
        // treated as an independent statement with the same indentation as `if (cond1)`. So the
        // recursive search for the if statement is necessary in the `bind_if_else` function.
        ast::Stmt::Else(else_branch) => {
            let last = vec
                .pop()
                .ok_or_else(|| "Else cannot be the first statement".to_string())?;
            let last = match last {
                ast::BlockItem::Decl(_) => {
                    return Err(
                        "Else must follow an If statement, but found a declaration".to_string()
                    );
                }
                ast::BlockItem::Stmt(stmt) => stmt,
            };
            let stmt = bind_if_else(else_branch, last).map_err(|(stmt, _)| {
                format!("Else must follow an If statement, but found: {:?}", stmt)
            })?;
            // Rebind the statement.
            //
            // Currently, we can believe that the else statement has been successfully bound to the
            // if statement. But for the statement in the else branch, it may still contain else
            // statements that need to be bound. The call to `bind_if_else_stmt` helps us do this.
            bind_if_else_stmt(stmt, vec)
        }
        // Because of the rebinding of the else statement, the if statement may be transformed into
        // an if-else statement. But we believe that the statement in if branch has been properly
        // bound so that we just need to bind statements in the else branch.
        ast::Stmt::IfElse(if_branch, mut else_branch) => {
            else_branch.stmt = bind_if_else_stmt(else_branch.stmt, vec)?;
            Ok(ast::Stmt::IfElse(if_branch, else_branch))
        }
    }
}

fn bind_if_else_block(block: ast::Block) -> Result<ast::Block, String> {
    let mut items = vec![];
    for item in block.items {
        match item {
            ast::BlockItem::Decl(_) => items.push(item),
            ast::BlockItem::Stmt(stmt) => {
                let stmt = bind_if_else_stmt(stmt, &mut items)?;
                items.push(ast::BlockItem::Stmt(stmt));
            }
        }
    }

    Ok(ast::Block::new(items))
}

fn func_scope(
    mut scope: ast::Block,
    data: &mut FunctionData,
    manager: &mut VariableManager,
) -> Vec<BlockFlow> {
    // Currently, we only support a single basic block for each function, so we can directly build
    // the entry block and VariableManager.
    let entry = {
        let block = data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".to_string()));
        BlockFlow::new(block, vec![])
    };

    let mut flows = vec![entry];

    let mut guard = ScopeGuard::new(manager);

    scope = bind_if_else_block(scope)
        .unwrap_or_else(|e| panic!("Error in binding if-else statements: {}", e));

    let params: Vec<_> = data.params().to_vec();
    for param in params {
        let value = data.dfg().value(param);
        let ty = value.ty().clone();
        let name = value
            .name()
            .as_ref()
            .map(|s| &s[1..])
            .unwrap_or_else(|| panic!("Parameter value should have a name starting with '@'"))
            .to_string();

        let alloc = data.dfg_mut().new_value().alloc(ty.clone());
        let store = data.dfg_mut().new_value().store(param, alloc);
        last_inst_vec(&mut flows).extend(vec![
            Instruction::new(alloc, true),
            Instruction::new(store, true),
        ]);

        guard
            .inner()
            .define_var(name, alloc, ty)
            .unwrap_or_else(|e| panic!("Error defining parameter variable: {}", e));
    }

    let dfg = data.dfg_mut();

    for item in scope.items {
        item.into_ir(dfg, guard.inner(), &mut flows);
    }

    // We always insert a new basic block after return. So we have to filter them out.
    let mut flows = flows
        .into_iter()
        .filter(|flow| !flow.insts.is_empty())
        .collect::<Vec<_>>();

    for insts in flows.iter_mut() {
        let mut reach_end = false;
        insts.insts = insts
            .insts
            .drain(..)
            .filter(|inst| {
                // For some kind of instructions (e.g., constant definitions), we may not want to
                // insert them into the basic block.
                inst.insert
            })
            .take_while(|inst| {
                // Note that the basic block should end with Return, Jump or Branch Instruction.
                // And no instruction should be generated after these instructions. So we can
                // simply filter out.
                if reach_end {
                    return false;
                }
                match dfg.value(inst.inst).kind() {
                    ValueKind::Return(_) | ValueKind::Jump(_) | ValueKind::Branch { .. } => {
                        reach_end = true;
                    }
                    _ => {}
                }
                true
            })
            .collect::<Vec<_>>();
    }

    if flows.last().is_none() {
        // void f() {}
        flows.push(BlockFlow::new(
            dfg.new_bb().basic_block(Some("%entry".to_string())),
            vec![Instruction::new(dfg.new_value().ret(None), true)],
        ));
    }

    flows
}

fn ast_to_func(func: ast::FuncDef, data: &mut FunctionData, manager: &mut VariableManager) {
    let seq = func_scope(func.block, data, manager);

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
