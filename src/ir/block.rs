use koopa::ir::{
    Type,
    builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{
        BlockFlow, ConstValue, Instruction, IntoIr, ScopeGuard, Variable, VariableManager,
        last_inst_vec,
    },
    parse::ast::{self, BType},
};

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

fn define_simple_var(
    ident: String,
    ty: Type,
    init_val: Option<ast::InitVal>,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    let vec = last_inst_vec(flows);
    let value = dfg.new_value().alloc(ty.clone());
    vec.push(Instruction::new(value, true));
    manager
        .define_var(ident, value, ty)
        .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
    if let Some(init_val) = init_val {
        match init_val {
            ast::InitVal::Array(_) => {
                panic!("Cannot initialize scalar variable with array initializer")
            }
            ast::InitVal::Expr(expr) => expr.into_ir(dfg, manager, flows),
        }
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

fn define_array_var(
    ident: String,
    size: ast::ConstExpr,
    ty: Type,
    init_val: Option<ast::InitVal>,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    // Only support one-dimensional array for now.
    let size = size
        .const_eval_i32(manager)
        .expect("Array size must be a constant expression");
    let arr_ty = Type::get_array(
        ty.clone(),
        size.try_into().expect("Array size must be non-negative"),
    );
    let vec = last_inst_vec(flows);
    let value = dfg.new_value().alloc(arr_ty.clone());
    manager
        .define_var(ident, value, arr_ty)
        .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
    vec.push(Instruction::new(value, true));
    if let Some(init_val) = init_val {
        match init_val {
            ast::InitVal::Expr(_) => {
                panic!("Cannot initialize array variable with scalar initializer")
            }
            ast::InitVal::Array(arr) => {
                let arr_len = arr.len() as i32;
                for (i, expr) in arr.into_iter().enumerate() {
                    expr.into_ir(dfg, manager, flows);
                    let src = *last_inst_vec(flows)
                        .last()
                        .copied()
                        .expect("Initialization expression should produce at least one value")
                        .inst();
                    let index = dfg.new_value().integer(i as i32);
                    let ptr = dfg.new_value().get_elem_ptr(value, index);
                    let store = dfg.new_value().store(src, ptr);
                    let vec = last_inst_vec(flows);
                    vec.extend(vec![
                        Instruction::new(index, false),
                        Instruction::new(ptr, true),
                        Instruction::new(store, true),
                    ]);
                }

                for i in arr_len..size {
                    let index = dfg.new_value().integer(i);
                    let ptr = dfg.new_value().get_elem_ptr(value, index);
                    let zero = dfg.new_value().zero_init(ty.clone());
                    let store = dfg.new_value().store(zero, ptr);
                    let vec = last_inst_vec(flows);
                    vec.extend(vec![
                        Instruction::new(index, false),
                        Instruction::new(ptr, true),
                        Instruction::new(zero, false),
                        Instruction::new(store, true),
                    ]);
                }
            }
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
            let init_val = def.init_val;
            let definition = def.definition;
            let ty: Type = ty.into();
            match definition {
                ast::Def::Ident { ident } => {
                    define_simple_var(ident, ty, init_val, dfg, manager, flows)
                }
                ast::Def::Array { ident, size } => {
                    define_array_var(ident, size, ty, init_val, dfg, manager, flows)
                }
            }
        }
    }
}

impl IntoIr for ast::ConstDecl {
    fn into_ir(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        self.define_const(dfg, manager, flows);
    }
}

pub(super) fn define_simple_const(
    ident: String,
    ty: BType,
    init_val: ast::ConstInitVal,
    manager: &mut VariableManager,
) {
    match ty {
        BType::Int => match init_val.const_eval_i32(manager) {
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

pub(super) fn define_array_const(
    ident: String,
    size: ast::ConstExpr,
    ty: BType,
    init_val: ast::ConstInitVal,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut [BlockFlow],
) {
    // Only support one-dimensional array for now.
    assert!(
        matches!(ty, BType::Int),
        "Unsupported type for array constant '{}'",
        ident
    );
    let size = size
        .const_eval_i32(manager)
        .expect("Array size must be a constant expression");
    let arr_ty = Type::get_array(
        ty.into(),
        size.try_into().expect("Array size must be non-negative"),
    );
    let vec = last_inst_vec(flows);
    let value = dfg.new_value().alloc(arr_ty.clone());
    manager
        .define_const(ident, ConstValue::Array(value))
        .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
    vec.push(Instruction::new(value, true));

    match init_val {
        ast::ConstInitVal::Expr(_) => {
            panic!("Cannot initialize array constant with scalar initializer")
        }
        ast::ConstInitVal::Array(arr) => match ty {
            BType::Int => {
                let arr_len = arr.len() as i32;
                for (i, expr) in arr.into_iter().enumerate() {
                    let src = expr.const_eval_i32(manager).expect(
                            "Initialization expression for array constant must be a constant expression",
                        );
                    let index = dfg.new_value().integer(i as i32);
                    let ptr = dfg.new_value().get_elem_ptr(value, index);
                    let num = dfg.new_value().integer(src);
                    let store = dfg.new_value().store(num, ptr);
                    vec.extend(vec![
                        Instruction::new(index, false),
                        Instruction::new(ptr, true),
                        Instruction::new(num, false),
                        Instruction::new(store, true),
                    ]);
                }

                for i in arr_len..size {
                    let index = dfg.new_value().integer(i);
                    let ptr = dfg.new_value().get_elem_ptr(value, index);
                    let zero = dfg.new_value().zero_init(ty.into());
                    let store = dfg.new_value().store(zero, ptr);
                    vec.extend(vec![
                        Instruction::new(index, false),
                        Instruction::new(ptr, true),
                        Instruction::new(zero, false),
                        Instruction::new(store, true),
                    ]);
                }
            }
            BType::Void => unreachable!(),
        },
    }
}

impl ast::ConstDecl {
    pub fn define_const(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut [BlockFlow],
    ) {
        let ty = self.ty;
        let defs = self.def;
        for def in defs {
            let definition = def.definition;
            match definition {
                ast::Def::Ident { ident } => define_simple_const(ident, ty, def.init_val, manager),
                ast::Def::Array { ident, size } => {
                    define_array_const(ident, size, ty, def.init_val, dfg, manager, flows)
                }
            }
        }
    }
}

fn assign_ir(
    lval: ast::LVal,
    expr: ast::Expr,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    expr.into_ir(dfg, manager, flows);
    let vec = last_inst_vec(flows);
    let src = *vec
        .last()
        .copied()
        .expect("Assignment expression should produce at least one value")
        .inst();
    match lval {
        ast::LVal::Ident(ident) => {
            let var = manager
                .get(&ident)
                .unwrap_or_else(|| panic!("Undefined variable: {}", ident));
            match var {
                Variable::Const(_) => {
                    panic!("Cannot assign to constant variable: {}", ident)
                }
                Variable::Var(var) => {
                    let var = var.clone();
                    let dest = *var.value();
                    let store = dfg.new_value().store(src, dest);
                    vec.push(Instruction::new(store, true));
                }
            }
        }
        ast::LVal::Array { ident, index } => {
            let var = manager
                .get(&ident)
                .unwrap_or_else(|| panic!("Undefined variable: {}", ident));
            match var {
                Variable::Const(_) => panic!("Cannot assign to constant variable: {}", ident),
                Variable::Var(var) => {
                    let arr = *var.value();
                    index.into_ir(dfg, manager, flows);
                    let vec = last_inst_vec(flows);
                    let index = *vec
                        .last()
                        .copied()
                        .expect("Array index expression should produce at least one value")
                        .inst();
                    let ptr = dfg.new_value().get_elem_ptr(arr, index);
                    let store = dfg.new_value().store(src, ptr);
                    vec.extend(vec![
                        Instruction::new(ptr, true),
                        Instruction::new(store, true),
                    ]);
                }
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
                assign_ir(lval, expr, dfg, manager, flows);
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
