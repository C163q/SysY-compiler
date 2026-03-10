use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{self, Debug, Display},
    mem,
};

use koopa::ir::{FunctionData, Value};

pub const INDENT: &str = "  ";

pub const TEXT_SECTION: &str = ".text";

pub const GLOBAL_SYMBOL: &str = ".globl";

pub const REGISTER_COUNT: usize = 32;
pub const REGISTER_ABI_NAMES: [&str; REGISTER_COUNT] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub const REGISTER_ID_NAMES: [&str; REGISTER_COUNT] = [
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "x31",
];

pub const INST_LOAD_IMMEDIATE: &str = "li";
pub const INST_RETURN: &str = "ret";
pub const INST_MOVE: &str = "mv";
pub const INST_ADDITION: &str = "add";
pub const INST_SUBTRACTION: &str = "sub";
pub const INST_MULTIPLICATION: &str = "mul";
pub const INST_DIVISION: &str = "div";
pub const INST_MODULO: &str = "rem";
pub const INST_AND: &str = "and";
pub const INST_OR: &str = "or";
pub const INST_XOR: &str = "xor";
pub const INST_SET_IF_EQUAL_TO_ZERO: &str = "seqz";
pub const INST_SET_IF_NOT_EQUAL_TO_ZERO: &str = "snez";
pub const INST_SET_IF_LESS_THAN: &str = "slt";
pub const INST_SET_IF_GREATER_THAN: &str = "sgt";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Register {
    /// x0，恒为0
    Zero = 0,
    /// x1，返回地址
    Ra,
    /// x2，栈指针
    Sp,
    /// x3，全局指针
    Gp,
    /// x4，线程指针
    Tp,
    /// x5，临时/备用链接寄存器
    T0,
    /// x6-7，临时寄存器
    T1,
    T2,
    /// x8，保存寄存器/帧指针
    S0,
    /// x9，保存寄存器
    S1,
    /// x10-11，函数参数/返回值
    A0,
    A1,
    /// x12-17，函数参数
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    /// x18-27，保存寄存器
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    /// x28-31，临时寄存器
    T3,
    T4,
    T5,
    T6,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        if value >= REGISTER_COUNT as u8 {
            panic!("Invaild register")
        }
        // SAFETY:
        // We ensure that value is in the range of 0-31
        unsafe { mem::transmute(value) }
    }
}

impl Register {
    pub fn is_caller_saved(&self) -> bool {
        matches!(
            self,
            Register::Ra
                | Register::T0
                | Register::T1
                | Register::T2
                | Register::A0
                | Register::A1
                | Register::A2
                | Register::A3
                | Register::A4
                | Register::A5
                | Register::A6
                | Register::A7
                | Register::T3
                | Register::T4
                | Register::T5
                | Register::T6
        )
    }

    pub fn caller_directly_usable(&self) -> bool {
        matches!(
            self,
            Register::T0
                | Register::T1
                | Register::T2
                | Register::A0
                | Register::A1
                | Register::A2
                | Register::A3
                | Register::A4
                | Register::A5
                | Register::A6
                | Register::A7
                | Register::T3
                | Register::T4
                | Register::T5
                | Register::T6
        )
    }
}

impl Register {
    pub fn name(&self) -> &'static str {
        REGISTER_ABI_NAMES[*self as u8 as usize]
    }

    pub fn name_id(&self) -> &'static str {
        REGISTER_ID_NAMES[*self as u8 as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterValue {
    InstRet(Value),
    Const,
}

#[derive(Debug)]
pub struct RegisterMapper {
    map: HashMap<Value, HashSet<Register>>,
    usage: HashMap<Register, RegisterValue>,
}

impl Default for RegisterMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl RegisterMapper {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            usage: HashMap::new(),
        }
    }

    pub fn get_available_registers(&self) -> BTreeSet<Register> {
        self.get_available_registers_filtered(|_| true)
    }

    pub fn get_available_registers_filtered<F>(&self, filter: F) -> BTreeSet<Register>
    where
        F: Fn(&Register) -> bool,
    {
        let used_registers: HashSet<Register> = self.usage.keys().cloned().collect();
        (0..REGISTER_COUNT)
            .map(|i| Register::from(i as u8))
            .filter(|reg| !used_registers.contains(reg) && filter(reg))
            .collect()
    }

    /// This function will not obtain registers that are currently mapped to the given values.
    pub fn get_registers_filtered_by_value<F>(
        &self,
        value: &[Value],
        filter: F,
    ) -> BTreeSet<Register>
    where
        F: Fn(&Register) -> bool,
    {
        let registers: HashSet<Register> = value
            .iter()
            .filter_map(|v| self.map.get(v))
            .flatten()
            .cloned()
            .collect();
        (0..REGISTER_COUNT)
            .map(|i| Register::from(i as u8))
            .filter(|reg| !registers.contains(reg) && filter(reg))
            .collect()
    }

    pub fn decl_register(&mut self, register: Register) {
        self.usage.insert(register, RegisterValue::Const);
    }

    pub fn insert(&mut self, value: RegisterValue, register: Register) {
        if let RegisterValue::InstRet(val) = value {
            self.map.entry(val).or_default().insert(register);
        }
        self.usage.insert(register, value);
    }

    pub fn remove(&mut self, value: Value, register: Register) {
        self.map.entry(value).or_default().remove(&register);
        self.usage.remove(&register);
    }

    pub fn remove_by_register(&mut self, register: Register) {
        if let Some(val) = self.usage.remove(&register)
            && let RegisterValue::InstRet(value) = val
        {
            self.map.entry(value).or_default().remove(&register);
        }
    }

    pub fn get_by_value(&self, value: &Value) -> Option<&HashSet<Register>> {
        self.map.get(value).filter(|set| !set.is_empty())
    }

    pub fn get_by_register(&self, register: &Register) -> Option<RegisterValue> {
        self.usage.get(register).copied()
    }

    pub fn contains_value(&self, value: &Value) -> bool {
        self.map.contains_key(value)
    }
}

pub struct FunctionContext<'a> {
    pub func_data: &'a FunctionData,
    pub register_mapper: RegisterMapper,
    /// i32为偏移量
    pub memory_map: HashMap<Value, i32>,
}

impl Debug for FunctionContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionContext")
            .field("func_data", &"&FunctionData")
            .field("register_map", &self.register_mapper)
            .field("memory_map", &self.memory_map)
            .finish()
    }
}

impl FunctionContext<'_> {
    pub fn new<'a>(func_data: &'a FunctionData) -> FunctionContext<'a> {
        FunctionContext {
            func_data,
            register_mapper: RegisterMapper::new(),
            memory_map: HashMap::new(),
        }
    }
}

/// 产生汇编代码
///
/// 对于某些IR数据，可能需要首先注册才行，否则无法直接使用它们的名字（例如函数）。
pub trait ToAsm {
    /// 产生汇编代码，不负责注册。
    fn to_asm(
        &self,
        func_data: Option<&mut FunctionContext<'_>>,
        id: Option<Value>,
    ) -> Vec<RiscvAsm>;

    /// 注册，对于某些全局数据，这是必须的步骤。
    fn register(&self) -> Option<RiscvAsm> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum RiscvInstruction {
    Ret,
    Li {
        dest: Register,
        imm: i32,
    },
    Mv {
        dest: Register,
        src: Register,
    },
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Sub {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Mul {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Div {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Mod {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    And {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Or {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Xor {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Seqz {
        dest: Register,
        src: Register,
    },
    Snez {
        dest: Register,
        src: Register,
    },
    Slt {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Sgt {
        dest: Register,
        src1: Register,
        src2: Register,
    },
}

macro_rules! binary_inst_format {
    ($asm:ident, $dest:expr, $src1:expr, $src2:expr, $f:expr) => {
        write!(
            $f,
            "{}{} {}, {}, {}",
            INDENT,
            $asm,
            $dest.name(),
            $src1.name(),
            $src2.name()
        )
    };
}

impl Display for RiscvInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscvInstruction::Ret => write!(f, "{}{}", INDENT, INST_RETURN),
            RiscvInstruction::Li { dest, imm } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_LOAD_IMMEDIATE,
                    dest.name(),
                    imm
                )
            }
            RiscvInstruction::Mv { dest, src } => {
                write!(f, "{}{} {}, {}", INDENT, INST_MOVE, dest.name(), src.name())
            }
            RiscvInstruction::Add { dest, src1, src2 } => {
                binary_inst_format!(INST_ADDITION, dest, src1, src2, f)
            }
            RiscvInstruction::Sub { dest, src1, src2 } => {
                binary_inst_format!(INST_SUBTRACTION, dest, src1, src2, f)
            }
            RiscvInstruction::Mul { dest, src1, src2 } => {
                binary_inst_format!(INST_MULTIPLICATION, dest, src1, src2, f)
            }
            RiscvInstruction::Div { dest, src1, src2 } => {
                binary_inst_format!(INST_DIVISION, dest, src1, src2, f)
            }
            RiscvInstruction::Mod { dest, src1, src2 } => {
                binary_inst_format!(INST_MODULO, dest, src1, src2, f)
            }
            RiscvInstruction::And { dest, src1, src2 } => {
                binary_inst_format!(INST_AND, dest, src1, src2, f)
            }
            RiscvInstruction::Or { dest, src1, src2 } => {
                binary_inst_format!(INST_OR, dest, src1, src2, f)
            }
            RiscvInstruction::Xor { dest, src1, src2 } => {
                binary_inst_format!(INST_XOR, dest, src1, src2, f)
            }
            RiscvInstruction::Seqz { dest, src } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_SET_IF_EQUAL_TO_ZERO,
                    dest.name(),
                    src.name()
                )
            }
            RiscvInstruction::Snez { dest, src } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_SET_IF_NOT_EQUAL_TO_ZERO,
                    dest.name(),
                    src.name()
                )
            }
            RiscvInstruction::Slt { dest, src1, src2 } => {
                binary_inst_format!(INST_SET_IF_LESS_THAN, dest, src1, src2, f)
            }
            RiscvInstruction::Sgt { dest, src1, src2 } => {
                binary_inst_format!(INST_SET_IF_GREATER_THAN, dest, src1, src2, f)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RiscvAsm {
    Section(String),
    Global(String),
    Label(String),
    Instruction(RiscvInstruction),
    None, // for formatting
}

impl Display for RiscvAsm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscvAsm::Section(name) => write!(f, "{}{}", INDENT, name),
            RiscvAsm::Global(name) => write!(f, "{}.globl {}", INDENT, name),
            RiscvAsm::Label(name) => write!(f, "{}:", name),
            RiscvAsm::Instruction(inst) => write!(f, "{}", inst),
            RiscvAsm::None => Ok(()),
        }
    }
}
