use std::num::NonZero;

use koopa::ir::{FunctionData, Program, TypeKind, Value, ValueKind, values::FuncArgRef};

use crate::asm::{
    block, expr, inst,
    meta::{self, FunctionContext, RV32Usize, RiscvAsm},
};

/// Create empty stack frame before calling this function.
pub fn build_call_stack_and_registers(
    context: &mut FunctionContext,
    callee_data: &FunctionData,
    args: &[Value],
) -> Vec<RiscvAsm> {
    let mut asms = vec![];
    if callee_data.params().len() != args.len() {
        panic!(
            "Argument count mismatch: expected {}, but got {}",
            callee_data.params().len(),
            args.len()
        );
    }
    for (i, param) in callee_data.params().iter().enumerate() {
        let param_data = callee_data.dfg().value(*param);
        if i >= 8 {
            context
                .memory_mapper
                .function_reserve(param_data.ty().size() as RV32Usize);
        }
    }
    asms.extend(context.memory_mapper.function_extend());

    for (i, (param, arg)) in callee_data.params().iter().zip(args).enumerate() {
        let param_data = callee_data.dfg().value(*param);
        let arg_data = context.func_data.dfg().value(*arg);
        if param_data.ty() != arg_data.ty() {
            panic!(
                "Argument type mismatch: expected {:?}, but got {:?}",
                param_data.ty(),
                arg_data.ty()
            );
        }

        let reg = *expr::get_value(*arg, context, &mut asms)
            .iter()
            .next()
            .expect("Failed to get argument value register");
        if i < 8 {
            let dest = meta::Register::from(meta::Register::A0 as u8 + i as u8);
            context.register_mapper.remove_by_register(dest);
            let mv = inst::mv_instruction(dest, reg, None);
            asms.push(mv);
        } else {
            context
                .memory_mapper
                .function_claim(*arg, arg_data.ty().clone());
            let offset = context
                .memory_mapper
                .get_offset(arg)
                .expect("Failed to get argument stack offset, it may not exist");

            let tmp = expr::obtain_caller_directly_usable_register(context);
            let stores = inst::add_sw_instruction(
                reg,
                meta::Register::Sp,
                meta::RV32Imm::new(offset.offset() as i32),
                None,
                Some(tmp),
            );
            asms.extend(stores);
        }
        context.register_mapper.remove(*arg, reg);
    }

    asms
}

pub fn register_func(func: &FunctionData) -> Option<RiscvAsm> {
    let name = &func.name()[1..]; // ignore the leading '@'
    Some(RiscvAsm::Global(name.to_string()))
}

pub fn function_assembly(
    program: &Program,
    func: &FunctionData,
    id: NonZero<usize>,
) -> Vec<RiscvAsm> {
    let mut context = FunctionContext::new(program, func, id);
    let name = context.get_label(
        func.layout()
            .entry_bb()
            .expect("Function declaration cannot generate assembly"),
    );
    let mut insts = vec![inst::label(&name)];
    insts.extend(function_prologue(&mut context));

    // insert block labels first so that jump instructions can find the labels
    for (&bb, _) in func.layout().bbs() {
        context.block_labels.insert(bb);
    }

    // generate instructions after all block labels are inserted
    for (&bb, node) in func.layout().bbs() {
        insts.extend(block::create_block(node, &mut context, bb));
    }
    insts
}

pub fn function_prologue(context: &mut FunctionContext) -> Vec<RiscvAsm> {
    // For return address
    context.memory_mapper.stack_reserve(meta::PTR_SIZE);
    context.memory_mapper.set_meta_size(meta::PTR_SIZE);

    struct ArgRefInfo<'a> {
        arg_ref: &'a FuncArgRef,
        size: RV32Usize,
    }

    impl ArgRefInfo<'_> {
        fn new<'a>(arg_ref: &'a FuncArgRef, size: RV32Usize) -> ArgRefInfo<'a> {
            ArgRefInfo { arg_ref, size }
        }
    }

    // FuncArgRef does not always come in order, so we need to save them.
    let mut arg_ref_vec = vec![];
    for data in context.func_data.dfg().values().values() {
        let ty = data.ty();
        if !ty.is_unit() {
            match data.kind() {
                ValueKind::Integer(_) | ValueKind::Return(_) | ValueKind::ZeroInit(_) => continue,
                ValueKind::Alloc(_) => {
                    // alloc return the type of the pointer
                    match ty.kind() {
                        TypeKind::Pointer(ty) => {
                            context.memory_mapper.stack_reserve(ty.size() as RV32Usize);
                        }
                        _ => panic!("Alloc value should return pointer type, but got {:?}", ty),
                    }
                    // and reserve pointer size to track the allocated address
                    context.memory_mapper.stack_reserve(ty.size() as RV32Usize);
                }
                ValueKind::Load(_)
                | ValueKind::Store(_)
                | ValueKind::Binary(_)
                | ValueKind::Call(_)
                | ValueKind::GetElemPtr(_) => {
                    context.memory_mapper.stack_reserve(ty.size() as RV32Usize)
                }
                ValueKind::FuncArgRef(func_arg) => {
                    arg_ref_vec.push(ArgRefInfo::new(func_arg, ty.size() as RV32Usize));
                }
                _ => unimplemented!(
                    "Value kind {:?} not implemented in function prologue",
                    data.kind()
                ),
            }
        }
    }

    // Make sure FuncArgRef are registered in order of their index, so that they can be registered
    // to the correct location (register or stack).
    arg_ref_vec.sort_unstable_by_key(|info| info.arg_ref.index());
    for info in arg_ref_vec {
        context
            .memory_mapper
            .register_func_arg(info.arg_ref, info.size);
    }

    // Save return address to stack
    let mut asms = context.memory_mapper.stack_extend();
    asms.extend(inst::add_sw_instruction(
        meta::Register::Ra,
        meta::Register::Sp,
        meta::RV32Imm::new(context.memory_mapper.meta_offset() as i32),
        None,
        Some(meta::Register::T1),
    ));
    asms
}

pub fn generate_funcs(program: &Program) -> Vec<RiscvAsm> {
    let mut vec = vec![];
    for (id, &func) in program.func_layout().iter().enumerate() {
        let func_data = program.func(func);
        if func_data.layout().entry_bb().is_none() {
            continue; // skip function declaration
        }
        vec.extend(register_func(func_data).into_iter());
        vec.extend(function_assembly(
            program,
            func_data,
            NonZero::new(id + 1).unwrap(),
        ));
        vec.push(RiscvAsm::None);
    }
    vec
}
