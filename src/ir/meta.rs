use std::collections::HashMap;

use koopa::ir::{Value, dfg::DataFlowGraph};

pub trait IntoIr {
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction>;
    fn const_eval_i32(&self, _manager: &VariableManager) -> Option<i32> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub inst: Value,
    pub insert: bool,
}

impl Instruction {
    pub fn new(inst: Value, insert: bool) -> Self {
        Self { inst, insert }
    }

    pub fn inst(&self) -> &Value {
        &self.inst
    }

    pub fn insert(&self) -> bool {
        self.insert
    }
}

pub enum ConstValue {
    Int(i32),
}

pub enum Variable {
    Const(ConstValue),
    // Var
}

pub struct VariableManager {
    map: HashMap<String, Variable>,
}

impl Default for VariableManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableManager {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.map.get(name)
    }

    pub fn define_const(&mut self, name: String, value: ConstValue) -> Result<(), String> {
        match self.map.get(&name) {
            Some(_) => Err(format!("Constant '{}' already defined", name)),
            None => {
                self.map.insert(name, Variable::Const(value));
                Ok(())
            }
        }
    }
}

