use std::{
    collections::{HashMap, HashSet},
    vec,
};

use koopa::ir::{FunctionData, Program};

use crate::asm::{
    inst,
    meta::{self, FunctionContext, ToAsm},
};

impl ToAsm for FunctionData {
    fn to_asm(&self, _: Option<&mut FunctionContext<'_>>) -> Vec<String> {
        let mut context =
            FunctionContext::new(self, HashMap::new(), HashSet::new(), HashMap::new());
        let name = &self.name()[1..]; // ignore the leading '@'
        let mut insts = vec![inst::label(name)];
        for (&_bb, node) in self.layout().bbs() {
            insts.extend(node.to_asm(Some(&mut context)));
        }
        insts
    }

    fn register(&self) -> Option<String> {
        let name = &self.name()[1..]; // ignore the leading '@'
        Some(format!("{}{} {}", meta::INDENT, meta::GLOBAL_SYMBOL, name))
    }
}

pub fn register_global_func(program: &Program) -> Vec<String> {
    let mut vec = vec![];
    for &func in program.func_layout() {
        let func_data = program.func(func);
        vec.extend(func_data.register().into_iter());
    }
    vec
}

pub fn generate_funcs(program: &Program) -> Vec<String> {
    let mut vec = vec![];
    for &func in program.func_layout() {
        let func_data = program.func(func);
        vec.extend(func_data.to_asm(None));
        vec.push(String::new());
    }
    vec
}
