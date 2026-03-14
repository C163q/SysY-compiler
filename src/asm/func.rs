use std::num::NonZero;

use koopa::ir::{FunctionData, Program, TypeKind, ValueKind};

use crate::asm::{
    block, inst,
    meta::{FunctionContext, RV32Usize, RiscvAsm},
};

pub fn register_func(func: &FunctionData) -> Option<RiscvAsm> {
    let name = &func.name()[1..]; // ignore the leading '@'
    Some(RiscvAsm::Global(name.to_string()))
}

pub fn function_assembly(func: &FunctionData, id: NonZero<usize>) -> Vec<RiscvAsm> {
    let mut context = FunctionContext::new(func, id);
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
    for data in context.func_data.dfg().values().values() {
        let ty = data.ty();
        if !ty.is_unit() {
            match data.kind() {
                ValueKind::Integer(_) | ValueKind::Return(_) => continue,
                ValueKind::Alloc(_) => {
                    // alloc return the type of the pointer
                    match ty.kind() {
                        TypeKind::Pointer(ty) => {
                            context.memory_mapper.allocate(ty.size() as RV32Usize);
                        }
                        _ => panic!("Alloc value should return pointer type, but got {:?}", ty),
                    }
                }
                ValueKind::Load(_) | ValueKind::Store(_) | ValueKind::Binary(_) => {
                    context.memory_mapper.allocate(ty.size() as RV32Usize)
                }
                _ => unimplemented!(
                    "Value kind {:?} not implemented in function prologue",
                    data.kind()
                ),
            }
        }
    }

    context.memory_mapper.extend_stack()
}

pub fn register_global_func(program: &Program) -> Vec<RiscvAsm> {
    let mut vec = vec![];
    for &func in program.func_layout() {
        let func_data = program.func(func);
        vec.extend(register_func(func_data).into_iter());
    }
    vec
}

pub fn generate_funcs(program: &Program) -> Vec<RiscvAsm> {
    let mut vec = vec![];
    for (id, &func) in program.func_layout().iter().enumerate() {
        let func_data = program.func(func);
        vec.extend(function_assembly(func_data, NonZero::new(id + 1).unwrap()));
        vec.push(RiscvAsm::None);
    }
    vec
}
