use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{self, Debug, Display},
    mem,
    num::NonZero,
    ops::{Deref, DerefMut},
};

use koopa::ir::{BasicBlock, FunctionData, Value};

use crate::asm::inst;

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
pub const INST_LOAD_WORD: &str = "lw";
pub const INST_STORE_WORD: &str = "sw";
pub const INST_ADDITION: &str = "add";
pub const INST_ADDITION_IMMEDIATE: &str = "addi";
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
pub const INST_JUMP: &str = "j";
pub const INST_BRANCH_IF_EQUAL_TO_ZERO: &str = "beqz";
pub const INST_BRANCH_IF_NOT_EQUAL_TO_ZERO: &str = "bnez";

pub type RV32Usize = u32;
pub type RV32Isize = i32;

pub const STACK_ALIGNMENT: RV32Usize = 16;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RV32Imm(i32);

impl RV32Imm {
    pub fn new(value: i32) -> Self {
        RV32Imm(value)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn set_value(&mut self, value: i32) {
        self.0 = value;
    }
}

impl Display for RV32Imm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for RV32Imm {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RV32Imm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RV32Imm12(i16);

impl RV32Imm12 {
    pub fn new(value: i16) -> Self {
        if !(-2048..=2047).contains(&value) {
            panic!("Immediate value out of range for RV32I: {}", value);
        }
        RV32Imm12(value)
    }

    pub fn value(&self) -> i16 {
        self.0
    }

    pub fn set_value(&mut self, value: i16) {
        if !(-2048..=2047).contains(&value) {
            panic!("Immediate value out of range for RV32I: {}", value);
        }
        self.0 = value;
    }
}

impl Display for RV32Imm12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for RV32Imm12 {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        // We ensure that the value is in the range of 0-31
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

    pub fn clear(&mut self) {
        self.map.clear();
        self.usage.clear();
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

#[derive(Debug, Clone, Copy)]
pub struct StackSizeAllocator {
    size: RV32Usize,
    aligned_size: RV32Usize,
}

impl Default for StackSizeAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackSizeAllocator {
    pub fn new() -> Self {
        StackSizeAllocator {
            size: 0,
            aligned_size: 0,
        }
    }

    pub fn allocate(&mut self, size: RV32Usize) -> RV32Usize {
        let old_size = self.stack_size();
        self.size += size;
        self.aligned_size = if self.size.is_multiple_of(STACK_ALIGNMENT) {
            self.size
        } else {
            self.size + (STACK_ALIGNMENT - self.size % STACK_ALIGNMENT)
        };
        println!(
            "Allocating stack size: {}, aligned size: {}, total size: {}",
            size,
            self.stack_size(),
            self.size()
        );
        self.stack_size() - old_size
    }

    pub fn stack_size(&self) -> RV32Usize {
        self.aligned_size
    }

    pub fn size(&self) -> RV32Usize {
        self.size
    }

    pub fn is_aligned(&self) -> bool {
        self.size() == self.stack_size()
    }
}

/// Stack:
///
/// ```text, ignore
/// +---------------------+ High address
/// |    Last function    |
/// +---------------------+
/// |   Saved registers   |
/// +---------------------+
/// |   Local variables   |
/// +---------------------+
/// | Function arguments  |
/// +---------------------+             ^    padding
/// |                     |             |       ^
/// |                     |             |       |
/// |        STACK        |        aligned_size |
/// |                     |             |      size
/// |                     |             |       |
/// +---------------------+ <- sp       v       v
/// |                     | Low address
/// ```
///
/// 我们暂时将map中的值视为相对于sp的偏移量。
#[derive(Debug, Clone)]
pub struct MemoryMapper {
    // To count the stack size, but not actually allocate it until the prologue.
    stack_size: StackSizeAllocator,
    // Map from Value to its offset in the stack. See the comment above for details.
    map: HashMap<Value, RV32Usize>,
    // Size of the stack that has been claimed by values. This is used to determine the offset of a
    // memory when claiming.
    claimed: RV32Usize,
    // Actual size of the stack that has been allocated. This should be done by modifying the stack
    // pointer in the prologue.
    allocated: RV32Usize,
}

impl Default for MemoryMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryMapper {
    pub fn new() -> Self {
        MemoryMapper {
            stack_size: StackSizeAllocator::new(),
            map: HashMap::new(),
            claimed: 0,
            allocated: 0,
        }
    }

    pub fn allocate(&mut self, size: RV32Usize) {
        let grow = self.stack_size.allocate(size);
        if grow > 0 {
            for entry in self.map.iter_mut() {
                *entry.1 += grow;
            }
        }
    }

    pub fn claim(&mut self, value: Value, size: RV32Usize) {
        if !self.map.contains_key(&value) {
            if self.claimed + size > self.size() {
                panic!(
                    "Claiming value {:?} with size {} exceeds allocated stack size {}",
                    value,
                    size,
                    self.size()
                );
            }
            self.map.insert(value, self.claimed);
            self.claimed += size;
        } else {
            panic!("Value already claimed in memory mapper: {:?}", value);
        }
    }

    pub fn alloc_size(&self) -> RV32Usize {
        self.stack_size.stack_size()
    }

    pub fn size(&self) -> RV32Usize {
        self.stack_size.size()
    }

    pub fn get_offset(&self, value: &Value, size: RV32Usize) -> Option<RV32Usize> {
        self.map.get(value).copied().inspect(|&offset| {
            if offset + size > self.size() {
                panic!(
                    "Offset {} and the size for value {:?} exceeds claimed stack size {}",
                    offset,
                    value,
                    self.size()
                );
            }
        })
    }

    pub fn extend_stack(&mut self) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        if self.alloc_size() > self.allocated {
            let size = self.alloc_size() - self.allocated;
            if size > i32::MAX as RV32Usize {
                panic!("Stack size exceeds i32::MAX: {}", size);
            }
            let size = size as i32;
            if size > 2048 {
                asms.push(inst::li_instruction(Register::T1, -size, None));
                asms.push(inst::add_instruction(
                    Register::Sp,
                    Register::Sp,
                    Register::T1,
                    None,
                ));
            } else {
                asms.push(inst::addi_instruction(
                    Register::Sp,
                    Register::Sp,
                    -(size as i16),
                    None,
                ));
            }
            self.allocated += size as RV32Usize;
        }
        asms
    }

    pub fn resume_stack(&mut self) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        if self.allocated > 0 {
            let size = self.allocated as i32;
            if size > 2047 {
                asms.push(inst::li_instruction(Register::T1, size, None));
                asms.push(inst::add_instruction(
                    Register::Sp,
                    Register::Sp,
                    Register::T1,
                    None,
                ));
            } else {
                asms.push(inst::addi_instruction(
                    Register::Sp,
                    Register::Sp,
                    size as i16,
                    None,
                ));
            }
            self.allocated = 0;
        }
        asms
    }
}

#[derive(Debug, Clone)]
pub struct BlockLabels {
    entry_id: BasicBlock,
    mapper: HashMap<BasicBlock, usize>,
}

impl BlockLabels {
    pub fn new(entry_id: BasicBlock) -> Self {
        BlockLabels {
            entry_id,
            mapper: HashMap::new(),
        }
    }

    pub fn insert(&mut self, bb: BasicBlock) {
        if bb != self.entry_id && !self.mapper.contains_key(&bb) {
            let label_id = self.mapper.len() + 1;
            self.mapper.insert(bb, label_id);
        }
    }

    pub fn entry_id(&self) -> BasicBlock {
        self.entry_id
    }
}

pub struct FunctionContext<'a> {
    pub func_data: &'a FunctionData,
    pub register_mapper: RegisterMapper,
    /// i32为偏移量
    pub memory_mapper: MemoryMapper,
    pub block_labels: BlockLabels,
    func_id: NonZero<usize>,
}

impl Debug for FunctionContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionContext")
            .field("func_data", &"&FunctionData")
            .field("register_map", &self.register_mapper)
            .field("memory_map", &self.memory_mapper)
            .finish()
    }
}

impl FunctionContext<'_> {
    pub fn new<'a>(func_data: &'a FunctionData, id: NonZero<usize>) -> FunctionContext<'a> {
        let entry_id = func_data
            .layout()
            .entry_bb()
            .expect("FATAL: cannot generate asm for a function declaration.");
        FunctionContext {
            func_data,
            register_mapper: RegisterMapper::new(),
            memory_mapper: MemoryMapper::new(),
            block_labels: BlockLabels::new(entry_id),
            func_id: id,
        }
    }

    pub fn get_label(&self, bb: BasicBlock) -> String {
        if bb == self.block_labels.entry_id() {
            self.func_data.name()[1..].to_string()
        } else {
            let id = self
                .block_labels
                .mapper
                .get(&bb)
                .expect("FATAL: basic block not found in block labels");
            format!("LBB{}_{}", self.get_func_id(), id)
        }
    }

    pub fn get_func_id(&self) -> NonZero<usize> {
        self.func_id
    }
}

/// 产生汇编代码
///
/// 对于某些IR数据，可能需要首先注册才行，否则无法直接使用它们的名字（例如函数）。
pub trait ToAsm {
    /// 产生汇编代码，不负责注册。
    fn to_asm(&self, context: &mut FunctionContext<'_>, id: Value) -> Vec<RiscvAsm>;
}

#[derive(Debug, Clone)]
pub enum RiscvInstruction {
    Ret,
    Li {
        dest: Register,
        imm: RV32Imm,
    },
    Mv {
        dest: Register,
        src: Register,
    },
    Lw {
        dest: Register,
        base: Register,
        offset: RV32Imm12,
    },
    Sw {
        src: Register,
        base: Register,
        offset: RV32Imm12,
    },
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Addi {
        dest: Register,
        src1: Register,
        src2: RV32Imm12,
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
    J {
        label: String,
    },
    Beqz {
        src: Register,
        label: String,
    },
    Bnez {
        src: Register,
        label: String,
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
            RiscvInstruction::Lw { dest, base, offset } => {
                write!(
                    f,
                    "{}{} {}, {}({})",
                    INDENT,
                    INST_LOAD_WORD,
                    dest.name(),
                    offset,
                    base.name()
                )
            }
            RiscvInstruction::Sw { src, base, offset } => {
                write!(
                    f,
                    "{}{} {}, {}({})",
                    INDENT,
                    INST_STORE_WORD,
                    src.name(),
                    offset,
                    base.name()
                )
            }
            RiscvInstruction::Add { dest, src1, src2 } => {
                binary_inst_format!(INST_ADDITION, dest, src1, src2, f)
            }
            RiscvInstruction::Addi { dest, src1, src2 } => {
                write!(
                    f,
                    "{}{} {}, {}, {}",
                    INDENT,
                    INST_ADDITION_IMMEDIATE,
                    dest.name(),
                    src1.name(),
                    src2
                )
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
            RiscvInstruction::J { label } => {
                write!(f, "{}{} {}", INDENT, INST_JUMP, label)
            }
            RiscvInstruction::Beqz { src, label } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_BRANCH_IF_EQUAL_TO_ZERO,
                    src.name(),
                    label
                )
            }
            RiscvInstruction::Bnez { src, label } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_BRANCH_IF_NOT_EQUAL_TO_ZERO,
                    src.name(),
                    label
                )
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
