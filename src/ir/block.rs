use koopa::ir::{
    Type, TypeKind, Value,
    builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::{
        arr::{eval_array_dim, get_array_ty, normal_arr_to_aggregate, normalize_array},
        meta::{
            BlockFlow, ConstValue, Instruction, IntoIr, ScopeGuard, Variable, VariableManager,
            last_inst_vec, last_inst_vec_value,
        },
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

/// ```c
/// int x = 10;
/// ```
///
/// Generated IR:
///
/// ```IR
/// @x = alloc i32
/// store 10, @x
/// ```
fn define_simple_var(
    ident: String,
    ty: Type,
    init_val: Option<ast::InitVal>,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    let vec = last_inst_vec(flows);

    // IR: @x = alloc type
    let value = dfg.new_value().alloc(ty.clone());
    vec.push(Instruction::new(value, true));

    // Variable manager: ident -> value
    manager
        .define_var(ident, value, ty)
        .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
    if let Some(init_val) = init_val {
        match init_val {
            ast::InitVal::Array(_) => {
                panic!("Cannot initialize scalar variable with array initializer")
            }
            // IR: IR<%expr>
            ast::InitVal::Expr(expr) => expr.into_ir(dfg, manager, flows),
            ast::InitVal::ZeroInit(ty) => {
                last_inst_vec(flows).push(Instruction::new(dfg.new_value().zero_init(ty), false))
            }
        }
        let src = last_inst_vec_value(flows);

        // IR: store RET<%expr>, @x
        let vec = last_inst_vec(flows);
        let store = dfg.new_value().store(src, value);
        vec.push(Instruction::new(store, true));
    }
}

fn init_arr(
    value: Value,
    init_val: ast::InitVal,
    elem_ty: Type,
    sizes: &[usize],
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    match init_val {
        ast::InitVal::Expr(_) => {
            panic!("Cannot initialize array variable with scalar initializer")
        }
        ast::InitVal::Array(arr) => {
            let arr = normalize_array(arr, sizes, elem_ty.clone());
            normal_arr_to_aggregate(
                arr,
                elem_ty,
                value,
                &mut vec![0; sizes.len()],
                sizes,
                dfg,
                manager,
                flows,
            );
        }
        ast::InitVal::ZeroInit(ty) => {
            let zero = dfg.new_value().zero_init(ty.clone());
            let store = dfg.new_value().store(zero, value);
            last_inst_vec(flows).extend(vec![
                Instruction::new(zero, false),
                Instruction::new(store, true),
            ]);
        }
    }
}

/// ```c
/// int arr[2] = {1};
/// ```
///
/// Generated IR:
///
/// ```IR
/// @arr = alloc [i32, 2]
/// %0 = getelemptr @arr, 0
/// store 1, %0
/// %1 = getelemptr @arr, 1
/// store zero_init, %1
/// ```
fn define_array_var(
    ident: String,
    sizes: Vec<ast::InitExpr>,
    ty: Type,
    init_val: Option<ast::InitVal>,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    assert!(
        matches!(ty.kind(), TypeKind::Int32),
        "Unsupported type for array variable '{}'",
        ident
    );
    let sizes = eval_array_dim(&sizes, manager);
    let arr_ty = get_array_ty(ty.clone(), &sizes);

    // IR: @arr = alloc [type, size]
    let value = dfg.new_value().alloc(arr_ty.clone());

    // Variable manager: ident -> value
    manager
        .define_var(ident, value, arr_ty)
        .unwrap_or_else(|e| panic!("Error defining variable: {}", e));
    last_inst_vec(flows).push(Instruction::new(value, true));
    if let Some(init_val) = init_val {
        init_arr(value, init_val, ty, &sizes, dfg, manager, flows);
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
                ast::Def::Array { ident, sizes } => {
                    define_array_var(ident, sizes, ty, init_val, dfg, manager, flows)
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

/// ```c
/// const int x = 10;
/// ```
///
/// No IR generated.
pub(super) fn define_simple_const(
    ident: String,
    ty: BType,
    init_val: ast::InitVal,
    manager: &mut VariableManager,
) {
    match ty {
        //
        // Consteval the expression.
        BType::Int => match init_val.const_eval_i32(manager) {
            Some(value) => {
                // Variable manager: ident -> value
                manager
                    .define_const(ident, ConstValue::Int(value))
                    .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
            }
            None => {
                // Consteval failed.
                panic!(
                    "Initialization value for constant '{}' is not a constant expression",
                    ident
                );
            }
        },
        BType::Void => panic!("Void type cannot be used for constant '{}'", ident),
    }
}

/// ```c
/// const int arr[2] = {1};
/// ```
///
/// Generated IR:
///
/// ```IR
/// @arr = alloc [i32, 2]
/// %0 = getelemptr @arr, 0
/// store 1, %0
/// %1 = getelemptr @arr, 1
/// store zero_init, %1
/// ```
pub(super) fn define_array_const(
    ident: String,
    sizes: Vec<ast::InitExpr>,
    ty: Type,
    init_val: ast::InitVal,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    // Only support one-dimensional array for now.
    assert!(
        matches!(ty.kind(), TypeKind::Int32),
        "Unsupported type for array constant '{}'",
        ident
    );
    let sizes = eval_array_dim(&sizes, manager);
    let arr_ty = get_array_ty(ty.clone(), &sizes);

    // IR: @arr = alloc [type, size]
    let value = dfg.new_value().alloc(arr_ty.clone());

    // Variable manager: ident -> value
    manager
        .define_const(ident, ConstValue::Array(value))
        .unwrap_or_else(|e| panic!("Error defining constant: {}", e));
    last_inst_vec(flows).push(Instruction::new(value, true));

    init_arr(value, init_val, ty, &sizes, dfg, manager, flows);
}

impl ast::ConstDecl {
    pub fn define_const(
        self,
        dfg: &mut DataFlowGraph,
        manager: &mut VariableManager,
        flows: &mut Vec<BlockFlow>,
    ) {
        let ty = self.ty;
        let defs = self.def;
        for def in defs {
            let definition = def.definition;
            match definition {
                ast::Def::Ident { ident } => define_simple_const(ident, ty, def.init_val, manager),
                ast::Def::Array { ident, sizes } => {
                    define_array_const(ident, sizes, ty.into(), def.init_val, dfg, manager, flows)
                }
            }
        }
    }
}

/// ```c
/// x = x + 1;
/// ```
///
/// Generated IR:
///
/// ```IR
/// %0 = add %0, 1
/// ```
fn assign_ir(
    lval: ast::LVal,
    expr: ast::Expr,
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    // IR: IR<%expr>
    expr.into_ir(dfg, manager, flows);
    let src = last_inst_vec_value(flows);
    let vec = last_inst_vec(flows);
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
                    // IR: store RET<%expr>, @x
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
                    if index.is_empty() {
                        panic!("Array variable '{}' must be indexed", ident);
                    }
                    let arr = *var.value();
                    let mut value = arr;
                    for idx in index {
                        // IR: IR<%idx>
                        idx.into_ir(dfg, manager, flows);
                        let idx_val = last_inst_vec_value(flows);
                        value = dfg.new_value().get_elem_ptr(value, idx_val);
                        last_inst_vec(flows).push(Instruction::new(value, true));
                    }

                    // IR: store RET<%expr>, %0
                    let store = dfg.new_value().store(src, value);
                    last_inst_vec(flows).push(Instruction::new(store, true));
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
        let cond_val = last_inst_vec_value(flows);

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
        let cond_val = last_inst_vec_value(&mut while_flow);
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
