use std::collections::HashMap;

use koopa::ir::{Type, Value, dfg::DataFlowGraph};

/// 转换为IR。
pub trait IntoIr {
    /// 转换为IR。
    ///
    /// dfg用于产生新的Value，此处Value可以代指指令的结果，也可以代指常量或变量的值。
    fn into_ir(self, dfg: &mut DataFlowGraph, manager: &mut VariableManager) -> Vec<Instruction>;

    /// 任何允许编译期求出i32值的表达式都应当返回[`Some`]。
    fn const_eval_i32(&self, _manager: &VariableManager) -> Option<i32> {
        None
    }
}

/// 指令。
///
/// inst代表指令求值结果，也可以代表常量或变量的值。
/// insert表示是否需要将该指令插入到当前基本块中，若某个指令在编译时不会产生具体指令（例如常量），
/// 则不需要插入到基本块中。
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

#[derive(Debug, Clone)]
pub enum ConstValue {
    Int(i32),
}

#[derive(Debug, Clone)]
pub struct VarValue {
    value: Value,
    ty: Type,
}

impl VarValue {
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

/// 一个变量可以是一个常量，也可以是一个变量。
///
/// 此处的常量指的是在编译期就能求出值的常量。
#[derive(Debug, Clone)]
pub enum Variable {
    Const(ConstValue),
    Var(VarValue),
}

/// 管理变量名与其值的映射关系。
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

    pub fn define_var(&mut self, name: String, value: Value, ty: Type) -> Result<(), String> {
        match self.map.get(&name) {
            Some(_) => Err(format!("Variable '{}' already defined", name)),
            None => {
                self.map.insert(name, Variable::Var(VarValue { value, ty }));
                Ok(())
            }
        }
    }
}
