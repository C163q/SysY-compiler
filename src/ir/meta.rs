use koopa::ir::{Value, dfg::DataFlowGraph};

pub trait IntoIr {
    fn into_ir(self, dfg: &mut DataFlowGraph) -> Vec<Instruction>;
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
