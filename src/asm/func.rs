use koopa::ir::{FunctionData, Program, Value};

use crate::asm::{
    inst,
    meta::{FunctionContext, RiscvAsm, ToAsm},
};

impl ToAsm for FunctionData {
    fn to_asm(&self, _: Option<&mut FunctionContext<'_>>, _: Option<Value>) -> Vec<RiscvAsm> {
        let mut context =
            FunctionContext::new(self);
        let name = &self.name()[1..]; // ignore the leading '@'
        let mut insts = vec![inst::label(name)];
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
