use koopa::ir::{FunctionData, Program, TypeKind, Value, ValueKind};

use crate::asm::{
    inst,
    meta::{FunctionContext, RV32Usize, RiscvAsm, ToAsm},
};

impl ToAsm for FunctionData {
    fn to_asm(&self, _: Option<&mut FunctionContext<'_>>, _: Option<Value>) -> Vec<RiscvAsm> {
        let mut context = FunctionContext::new(self);
        let name = &self.name()[1..]; // ignore the leading '@'
        let mut insts = vec![inst::label(name)];
        insts.extend(function_prologue(&mut context));
        for (&_bb, node) in self.layout().bbs() {
            insts.extend(node.to_asm(Some(&mut context), None));
        }
        insts
    }

    fn register(&self) -> Option<RiscvAsm> {
        let name = &self.name()[1..]; // ignore the leading '@'
        Some(RiscvAsm::Global(name.to_string()))
    }
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
                _ => unimplemented!("Value kind {:?} not implemented in function prologue", data.kind()),
            }
        }
    }

    context.memory_mapper.extend_stack()
}

pub fn register_global_func(program: &Program) -> Vec<RiscvAsm> {
    let mut vec = vec![];
    for &func in program.func_layout() {
        let func_data = program.func(func);
        vec.extend(func_data.register().into_iter());
    }
    vec
}

pub fn generate_funcs(program: &Program) -> Vec<RiscvAsm> {
    let mut vec = vec![];
    for &func in program.func_layout() {
        let func_data = program.func(func);
        vec.extend(func_data.to_asm(None, None));
        vec.push(RiscvAsm::None);
    }
    vec
}
