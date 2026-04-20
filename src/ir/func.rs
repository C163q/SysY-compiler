use std::collections::HashSet;

use koopa::ir::{
    Function, FunctionData, Program, Type, ValueKind,
    builder::{BasicBlockBuilder, LocalInstBuilder},
};

use crate::{
    ir::meta::{
        BlockFlow, ConstValue, Instruction, IntoIr, ScopeGuard, Variable, VariableManager,
        last_inst_vec,
    },
    parse::ast,
};

/// Add a new function in program without parsing its body.
pub fn register_func(
    func_decl: &ast::FuncDecl,
    program: &mut Program,
    manager: &mut VariableManager,
    defined_func: &mut HashSet<String>,
    is_def: bool,
) -> Function {
    if is_def && !defined_func.insert(func_decl.ident.clone()) {
        panic!(
            "Function '{}' has been defined multiple times",
            func_decl.ident
        );
    }
    if let Some(v) = manager.get(&func_decl.ident) {
        match v {
            Variable::Var(_) => panic!(
                "Function '{}' has the same name as a variable",
                func_decl.ident
            ),
            Variable::Const(v) => match v {
                ConstValue::Function(_) => (),
                _ => panic!(
                    "Function '{}' has the same name as a constant",
                    func_decl.ident
                ),
            },
        }
    }

    let data = {
        let ret_type = func_decl.ret_type.into();
        let func_name = format!("@{}", func_decl.ident);
        match func_decl.fparams.clone() {
            None => FunctionData::with_param_names(func_name, vec![], ret_type),
            Some(fparams) => FunctionData::with_param_names(
                func_name,
                fparams
                    .params
                    .into_iter()
                    .map(|param| {
                        let mut ty = param.ty.into();
                        if let Some(arr) = param.arr {
                            for size in arr.into_iter().rev() {
                                ty = Type::get_array(ty, size.eval_usize(manager));
                            }
                            ty = Type::get_pointer(ty);
                        }
                        (Some(format!("@{}", param.ident)), ty)
                    })
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
        .define_const(func_decl.ident.clone(), ConstValue::Function(value))
        .expect("Error defining function");
    value
}

impl ast::FuncDef {
    /// parsing function body into IR.
    ///
    /// NOTE: function MUST be registered first.
    pub fn generate_ir(self, data: &mut FunctionData, manager: &mut VariableManager) {
        ast_to_func(self, data, manager);
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

        // We allocate a new memory for each parameter and store the parameter value into the
        // memory. So that we can treat the parameter as a normal variable.
        let alloc = data.dfg_mut().new_value().alloc(ty.clone());
        let store = data.dfg_mut().new_value().store(param, alloc);
        last_inst_vec(&mut flows)
            .extend([Instruction::new(alloc, true), Instruction::new(store, true)]);

        guard
            .inner()
            .define_var(name, alloc, ty)
            .unwrap_or_else(|e| panic!("Error defining parameter variable: {}", e));
    }

    let dfg = data.dfg_mut();

    if !matches!(
        scope.items.last(),
        Some(ast::BlockItem::Stmt(ast::Stmt::Return(_)))
    ) {
        scope
            .items
            .push(ast::BlockItem::Stmt(ast::Stmt::Return(None)));
    }

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
