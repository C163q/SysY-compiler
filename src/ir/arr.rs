use std::fmt::Debug;

use koopa::ir::{
    Program, Type, TypeKind, Value,
    builder::{LocalInstBuilder, ValueBuilder},
    dfg::DataFlowGraph,
};

use crate::{
    ir::meta::{
        BlockFlow, Instruction, IntoIr, VariableManager, last_inst_vec, last_inst_vec_value,
    },
    parse::ast,
};

fn get_dim_unchecked(init: &ast::InitVal, dims: &[usize], elem_ty: Type) -> usize {
    match init {
        ast::InitVal::Expr(_) => 0,
        ast::InitVal::Array(arr) => {
            arr.iter()
                .map(|e| get_dim_unchecked(e, &dims[1..], elem_ty.clone()))
                .max()
                .unwrap_or(0)
                + 1
        }
        ast::InitVal::ZeroInit(ty) => {
            let mut dim = 0;
            let mut arr_ty = ty;
            while let TypeKind::Array(ty, _) = arr_ty.kind() {
                dim += 1;
                arr_ty = ty;
            }

            dim
        }
    }
}

pub(super) fn normalize_array(
    mut arr: Vec<ast::InitVal>,
    dims: &[usize],
    elem_ty: Type,
) -> Vec<ast::InitVal> {
    fn push_array<T: Clone + Debug>(size: usize, mut vals: Vec<T>, default: T) -> Vec<T> {
        if size < vals.len() {
            panic!("Initializer has more elements than array size");
        }
        let remain = size - vals.len();
        for _ in 0..remain {
            vals.push(default.clone());
        }
        vals
    }

    fn normalize(arr: &mut Vec<ast::InitVal>, dims: &[usize], elem_ty: Type) -> Vec<ast::InitVal> {
        if dims.is_empty() {
            panic!("Cannot initialize array with zero dimensions");
        }
        let mut ret = vec![];
        if dims.len() == 1 {
            let mut count = 0;
            while let Some(elem) = arr.pop() {
                match elem {
                    ast::InitVal::Expr(_) => {
                        if count >= dims[0] {
                            arr.push(elem);
                            break;
                        }
                        ret.push(elem);
                        count += 1;
                    }
                    ast::InitVal::ZeroInit(ref ty) => {
                        if count >= dims[0] || ty.clone() != elem_ty {
                            arr.push(elem);
                            break;
                        }
                        ret.push(elem);
                        count += 1;
                    }
                    ast::InitVal::Array(_) => {
                        arr.push(elem);
                        break;
                    }
                }
            }
            push_array(dims[0], ret, ast::InitVal::ZeroInit(elem_ty))
        } else {
            let dim = dims.len();
            let mut count = 0;
            while let Some(elem) = arr.pop() {
                if count >= dims[0] {
                    arr.push(elem);
                    break;
                }

                let elem_dim = get_dim_unchecked(&elem, dims, elem_ty.clone());
                if elem_dim >= dim {
                    break;
                }

                match elem {
                    ast::InitVal::Array(mut sub_arr) => {
                        if elem_dim == dim - 1 {
                            sub_arr.reverse();
                            ret.push(ast::InitVal::Array(normalize(
                                &mut sub_arr,
                                &dims[1..],
                                elem_ty.clone(),
                            )));
                            count += 1;
                        } else {
                            ret.push(ast::InitVal::Array(normalize(
                                arr,
                                &dims[1..],
                                elem_ty.clone(),
                            )));
                            count += 1;
                        }
                    }
                    ast::InitVal::Expr(_) => {
                        arr.push(elem);
                        let vec = normalize(arr, &dims[1..], elem_ty.clone());
                        arr.push(ast::InitVal::Array(vec));
                    }
                    ast::InitVal::ZeroInit(_) => {
                        unreachable!();
                    }
                }
            }

            let mut sub_arr_ty = elem_ty;
            for size in dims[1..].iter().rev() {
                sub_arr_ty = Type::get_array(sub_arr_ty, *size);
            }

            push_array(dims[0], ret, ast::InitVal::ZeroInit(sub_arr_ty))
        }
    }

    if dims.is_empty() {
        panic!("Cannot initialize array with zero dimensions");
    }
    arr.reverse();
    let ret = normalize(&mut arr, dims, elem_ty);
    assert!(
        arr.is_empty(),
        "Initializer has more elements than array size"
    );
    ret
}

pub(super) fn normal_global_arr_to_aggregate(
    arr: &[ast::InitVal],
    level: usize,
    result: &mut Vec<Value>,
    program: &mut Program,
    manager: &mut VariableManager,
) {
    for init in arr {
        match init {
            ast::InitVal::Expr(expr) => {
                if level != 1 {
                    panic!("Initializer has too few elements for array dimension");
                }
                let val = expr.const_eval_i32(manager).expect(
                    "Initialization expression for global array must be a constant expression",
                );
                let val = program.new_value().integer(val);
                result.push(val);
            }
            ast::InitVal::Array(sub_arr) => {
                if level == 1 {
                    panic!("Initializer has too many elements for array dimension");
                }
                let mut sub_result = vec![];
                normal_global_arr_to_aggregate(
                    sub_arr,
                    level - 1,
                    &mut sub_result,
                    program,
                    manager,
                );
                let agg = program.new_value().aggregate(sub_result);
                result.push(agg);
            }
            ast::InitVal::ZeroInit(ty) => {
                result.push(program.new_value().zero_init(ty.clone()));
            }
        }
    }
}

pub(super) fn normal_arr_to_aggregate(
    arr: Vec<ast::InitVal>,
    elem_ty: Type,
    arr_val: Value,
    idxs: &mut [usize],
    dims: &[usize],
    dfg: &mut DataFlowGraph,
    manager: &mut VariableManager,
    flows: &mut Vec<BlockFlow>,
) {
    assert!(!dims.is_empty());

    let idxs_len = idxs.len();
    assert!(
        idxs[idxs_len - dims.len()..]
            .iter()
            .zip(dims.iter())
            .all(|(&idx, &dim)| idx < dim)
    );
    for init in arr {
        match init {
            ast::InitVal::Expr(expr) => {
                if dims.len() > 1 {
                    panic!("Initializer has too few elements for array dimension");
                }
                let (src, push) = if expr.is_const {
                    let src = expr.const_eval_i32(manager).expect(
                        "Initialization expression for array constant must be a constant expression",
                    );
                    let src = dfg.new_value().integer(src);
                    (src, true)
                } else {
                    expr.into_ir(dfg, manager, flows);
                    let src = last_inst_vec_value(flows);
                    (src, false)
                };
                let mut value = arr_val;
                for idx in idxs.iter() {
                    let idx_val = dfg.new_value().integer(*idx as i32);
                    value = dfg.new_value().get_elem_ptr(value, idx_val);
                    last_inst_vec(flows).extend([
                        Instruction::new(idx_val, false),
                        Instruction::new(value, true),
                    ]);
                }
                let store = dfg.new_value().store(src, value);
                if push {
                    last_inst_vec(flows).push(Instruction::new(src, false));
                }
                last_inst_vec(flows).push(Instruction::new(store, true));
                idxs[idxs_len - dims.len()] += 1;
            }
            ast::InitVal::Array(sub_arr) => {
                if dims.len() == 1 {
                    panic!("Initializer has too many elements for array dimension");
                }
                normal_arr_to_aggregate(
                    sub_arr,
                    elem_ty.clone(),
                    arr_val,
                    idxs,
                    &dims[1..],
                    dfg,
                    manager,
                    flows,
                );
                idxs[idxs_len - dims.len()] += 1;
                idxs.iter_mut()
                    .skip(idxs_len - dims.len() + 1)
                    .for_each(|idx| *idx = 0);
            }
            ast::InitVal::ZeroInit(ty) => {
                fn store_zero_init(
                    elem_ty: Type,
                    arr_val: Value,
                    ty: Type,
                    idxs: &mut [usize],
                    dims: &[usize],
                    dfg: &mut DataFlowGraph,
                    flows: &mut Vec<BlockFlow>,
                ) {
                    assert!(
                        idxs[idxs.len() - dims.len()..]
                            .iter()
                            .zip(dims.iter())
                            .all(|(&idx, &dim)| idx < dim)
                    );
                    match ty.kind() {
                        TypeKind::Array(sub_ty, len) => {
                            assert!(dims.len() > 1);
                            for _ in 0..*len {
                                store_zero_init(
                                    elem_ty.clone(),
                                    arr_val,
                                    sub_ty.clone(),
                                    idxs,
                                    &dims[1..],
                                    dfg,
                                    flows,
                                );
                            }
                            idxs[idxs.len() - dims.len()] += 1;
                            let len = idxs.len();
                            idxs.iter_mut()
                                .skip(len - dims.len() + 1)
                                .for_each(|idx| *idx = 0);
                        }
                        _ => {
                            assert!(dims.len() == 1);
                            assert_eq!(ty, elem_ty);
                            let mut value = arr_val;
                            for idx in idxs.iter() {
                                let idx_val = dfg.new_value().integer(*idx as i32);
                                value = dfg.new_value().get_elem_ptr(value, idx_val);
                                last_inst_vec(flows).extend([
                                    Instruction::new(idx_val, false),
                                    Instruction::new(value, true),
                                ]);
                            }
                            let zero = dfg.new_value().zero_init(elem_ty);
                            let store = dfg.new_value().store(zero, value);
                            last_inst_vec(flows).extend([
                                Instruction::new(zero, false),
                                Instruction::new(store, true),
                            ]);
                            idxs[idxs.len() - dims.len()] += 1;
                        }
                    }
                }
                assert!(get_array_elem_ty(&ty) == elem_ty);
                store_zero_init(elem_ty.clone(), arr_val, ty, idxs, dims, dfg, flows);
            }
        }
    }
}

pub fn get_array_ty(mut elem_ty: Type, dims: &[usize]) -> Type {
    for size in dims.iter().rev() {
        elem_ty = Type::get_array(elem_ty, *size);
    }
    elem_ty
}

pub fn get_array_elem_ty(arr_ty: &Type) -> Type {
    match arr_ty.kind() {
        TypeKind::Array(elem_ty, _) => get_array_elem_ty(elem_ty),
        _ => arr_ty.clone(),
    }
}

pub fn eval_array_dim(sizes: &[ast::InitExpr], manager: &VariableManager) -> Vec<usize> {
    let sizes: Vec<usize> = sizes.iter().map(|s| s.eval_usize(manager)).collect();
    if sizes.is_empty() {
        panic!("Cannot define array with zero dimensions");
    }
    if sizes.contains(&0) {
        panic!("Array dimension size cannot be zero");
    }
    sizes
}
