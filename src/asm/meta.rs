use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{self, Debug, Display},
    mem,
    num::NonZero,
};

use koopa::ir::{BasicBlock, FunctionData, Program, Value, values::FuncArgRef};

use crate::asm::inst;

pub const INDENT: &str = "  ";

pub const TEXT_SECTION: &str = ".text";
pub const DATA_SECTION: &str = ".data";

pub const GLOBAL_SYMBOL: &str = ".globl";
pub const ZERO_INIT: &str = ".zero";
pub const INIT_WORD: &str = ".word";

pub const LOWER_12_BIT: &str = "%lo";
pub const UPPER_20_BIT: &str = "%hi";

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
pub const INST_LOAD_UPPER_IMMEDIATE: &str = "lui";
pub const INST_RETURN: &str = "ret";
pub const INST_CALL: &str = "call";
pub const INST_MOVE: &str = "mv";
pub const INST_LOAD_ADDRESS: &str = "la";
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
pub const PTR_SIZE: RV32Usize = 4;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RV32Imm {
    Num(i32),
    Label(String),
}

impl RV32Imm {
    pub fn new(value: i32) -> Self {
        RV32Imm::Num(value)
    }

    pub fn new_label(label: String) -> Self {
        RV32Imm::Label(label)
    }

    pub fn value(&self) -> i32 {
        match self {
            RV32Imm::Num(val) => *val,
            RV32Imm::Label(_) => panic!("Immediate value is a label, not a number"),
        }
    }

    pub fn set_value(&mut self, value: i32) {
        *self = RV32Imm::Num(value);
    }

    pub fn set_label(&mut self, label: String) {
        *self = RV32Imm::Label(label);
    }
}

impl Display for RV32Imm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RV32Imm::Num(val) => write!(f, "{}", val),
            RV32Imm::Label(label) => write!(f, "{}", label),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RV32Imm12 {
    Num(i16),
    LabelLow(String),
}

impl RV32Imm12 {
    pub fn new(value: i16) -> Self {
        if !(-2048..=2047).contains(&value) {
            panic!("Immediate value out of range for RV32I: {}", value);
        }
        RV32Imm12::Num(value)
    }

    pub fn new_label(label: String) -> Self {
        RV32Imm12::LabelLow(label)
    }

    pub fn num(&self) -> i16 {
        match self {
            RV32Imm12::Num(val) => *val,
            RV32Imm12::LabelLow(_) => panic!("Immediate value is a label, not a number"),
        }
    }

    pub fn set_num(&mut self, value: i16) {
        if !(-2048..=2047).contains(&value) {
            panic!("Immediate value out of range for RV32I: {}", value);
        }
        *self = RV32Imm12::Num(value);
    }

    pub fn set_label(&mut self, label: String) {
        *self = RV32Imm12::LabelLow(label);
    }
}

impl Display for RV32Imm12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RV32Imm12::Num(val) => write!(f, "{}", val),
            RV32Imm12::LabelLow(label) => write!(f, "{}({})", LOWER_12_BIT, label),
        }
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
pub struct StackSizeCalculator {
    // To count the stack size, but not actually allocate it until the prologue.
    size: RV32Usize,
    aligned_size: RV32Usize,
}

impl Default for StackSizeCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackSizeCalculator {
    pub fn new() -> Self {
        Self {
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

    pub fn clear(&mut self) {
        self.size = 0;
        self.aligned_size = 0;
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
/// Values in the map are the offset from the stack pointer, and the offset is determined when
/// claiming the stack for the value. So the offsets will not change after claiming, and we can
/// safely get the offset of a value in the stack after claiming.
#[derive(Debug, Clone)]
pub struct StackSizeAllocator {
    calculator: StackSizeCalculator,
    // Map from Value to its offset in the stack.
    map: HashMap<Value, RV32Usize>,
    // Size of the stack that has been claimed by values. This is used to determine the offset of a
    // memory when claiming.
    claimed: RV32Usize,
    // Actual size of the stack that has been allocated. This should be done by modifying the stack
    // pointer in the prologue.
    allocated: RV32Usize,
    // For meta data, e.g. return address
    meta_size: RV32Usize,
}

impl Default for StackSizeAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackSizeAllocator {
    pub fn new() -> Self {
        StackSizeAllocator {
            calculator: StackSizeCalculator::new(),
            map: HashMap::new(),
            claimed: 0,
            allocated: 0,
            meta_size: 0,
        }
    }

    pub fn set_meta_size(&mut self, size: RV32Usize) {
        self.meta_size = size;
    }

    pub fn reserve(&mut self, size: RV32Usize) -> RV32Usize {
        if self.allocated > 0 {
            panic!("Cannot reserve stack size after stack has been allocated");
        }
        self.calculator.allocate(size)
    }

    pub fn reserved_size(&self) -> RV32Usize {
        self.calculator.size()
    }

    pub fn calculated_size(&self) -> RV32Usize {
        self.calculator.stack_size()
    }

    pub fn size(&self) -> RV32Usize {
        self.allocated
    }

    pub fn is_aligned(&self) -> bool {
        self.calculator.is_aligned()
    }

    /// Never extend the stack after mapping values, otherwise the offsets will be wrong. This
    /// function should only be called in the prologue or before the function call.
    pub fn extend_stack(&mut self) -> Vec<RiscvAsm> {
        let mut asms = vec![];
        // Never re-extend
        if self.allocated > 0 {
            panic!("Cannot extend stack after stack has been allocated");
        }
        if self.meta_size > self.calculator.size() {
            panic!(
                "Meta size {} exceeds reserved stack size {}",
                self.meta_size,
                self.calculator.size()
            );
        }
        // Aligned
        if self.calculated_size() > self.allocated {
            let size = self.calculated_size() - self.allocated;
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

    /// A helper function to pop the stack in the epilogue. This should only be called in the
    /// epilogue or after the function call.
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
        }
        asms
    }

    /// [`StackSizeAllocator::claim`] helps to register the offset of a value in the stack. The
    /// offset is determined when claiming, and will not change.
    pub fn claim(&mut self, value: Value, size: RV32Usize) {
        if !self.map.contains_key(&value) {
            if self.claimed + size + self.meta_size() > self.size() {
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

    pub fn get_offset(&self, value: &Value, size: RV32Usize) -> Option<RV32Usize> {
        self.map.get(value).copied().inspect(|&offset| {
            if offset + size + self.meta_size > self.size() {
                panic!(
                    "Offset {} and the size for value {:?} exceeds claimed stack size {} (with meta size {})",
                    offset,
                    value,
                    self.size(),
                    self.meta_size(),
                );
            }
        })
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.calculator.clear();
        self.allocated = 0;
        self.claimed = 0;
    }

    pub fn meta_size(&self) -> RV32Usize {
        self.meta_size
    }
}

/// [`FuncArgManager`] will NOT manage the stack for function arguments. It only manages the
/// offsets for the function arguments in the stack, and the caller is responsible for managing the
/// stack for function arguments.
///
/// So it is necessary to register the function arguments in the [`FuncArgManager`] before getting
/// the offset of the function arguments. And the function arguments should be registered in order.
/// Panic will be raised if the function arguments are not registered in order, e.g. parameter 0 is
/// registered after parameter 1.
#[derive(Debug, Clone)]
pub struct FuncArgManager {
    function_args: HashMap<usize, RV32Usize>,
    size: RV32Usize,
    arg_count: usize,
}

impl Default for FuncArgManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FuncArgManager {
    pub fn new() -> Self {
        FuncArgManager {
            function_args: HashMap::new(),
            size: 0,
            arg_count: 0,
        }
    }

    /// Register the function argument with its index and size, so that we can calculate the offset
    /// of the function argument in the stack. The function arguments MUST be registered in order,
    /// e.g. parameter 0 should be registered before parameter 1. Panic will be raised if not.
    ///
    /// Typically, you may call [`MemoryMapper::register_func_arg`].
    pub fn insert(&mut self, arg_ref: &FuncArgRef, size: RV32Usize) {
        let idx = arg_ref.index();
        if self.function_args.contains_key(&idx) {
            panic!(
                "Function argument {:?} already exists in function argument manager",
                arg_ref
            );
        }
        if self.arg_count != idx {
            panic!(
                "Function argument index {} is not inserted in order, expected {}",
                idx, self.arg_count
            );
        }
        if idx < 8 {
            self.arg_count += 1;
            return;
        }
        self.function_args.insert(idx, self.size);
        self.size += size;
        self.arg_count += 1;
    }

    /// Get the offset of the function argument in the stack. The function arguments MUST be
    /// registered.
    pub fn get_offset(&self, arg_ref: &FuncArgRef, size: RV32Usize) -> Option<RV32Usize> {
        let idx = arg_ref.index();
        if idx < 8 {
            None
        } else {
            self.function_args.get(&idx).copied().inspect(|&offset| {
                if offset + size > self.size {
                    panic!(
                        "Offset {} and the size for function argument {:?} exceeds total function argument size {}",
                        offset,
                        arg_ref,
                        self.size
                    );
                }
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArgLocation {
    Register(Register),
    Stack(RV32Usize),
}

#[derive(Debug, Clone)]
pub struct MemoryMapper {
    /// `stack_allocator` only helps us to manage the stack for the current function call. We can
    /// use it to manage the stack for local variables and saved registers in the current function.
    ///
    /// To build a stack for function calls, we need to use the `caller_stack` field. It contains
    /// the params for the called function, and registers to be saved and restored for the caller
    /// function.
    stack_allocator: StackSizeAllocator,

    /// The [`StackSizeAllocator`] itself should be a stack because when we try to pass the result
    /// of a function (say f()) as the parameter of another function (say g()), we need to reserve
    /// the stack for g() in f() before calling g().
    ///
    /// ```text
    /// +---------------------+ High address
    /// |   Caller function   |
    /// +---------------------+
    /// |  Arguments for g()  |
    /// +---------------------+
    /// |     Stack of g()    |
    /// +---------------------+
    /// |  Arguments for f()  |
    /// +---------------------+
    /// |     Stack of f()    |
    /// +---------------------+
    /// |                     | Low address
    /// ```
    ///
    /// The [`StackSizeAllocator`] helps us to manage the stack for every function call. And [`Vec`]
    /// helps us to manage the nested function calls.
    ///
    /// Note that `caller_stack` is not responsible for manage the stack for the function being
    /// called. Funtions should manage it with their own `stack_allocator`.
    caller_stack: Vec<StackSizeAllocator>,

    /// For every single function, [`MemoryMapper`] is not responsible for the arguments in the
    /// stack. So it is important to register the function arguments in the [`FuncArgManager`]
    /// before getting the offset of the function arguments.
    function_args: FuncArgManager,
}

impl Default for MemoryMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryMapper {
    pub fn new() -> Self {
        MemoryMapper {
            stack_allocator: StackSizeAllocator::new(),
            caller_stack: Vec::new(),
            function_args: FuncArgManager::new(),
        }
    }

    pub fn stack_reserve(&mut self, size: RV32Usize) {
        self.stack_allocator.reserve(size);
    }

    pub fn function_reserve(&mut self, size: RV32Usize) {
        self.caller_stack
            .last_mut()
            .expect("Caller stack should not be empty when reserving function stack")
            .reserve(size);
    }

    pub fn stack_claim(&mut self, value: Value, size: RV32Usize) {
        self.stack_allocator.claim(value, size);
    }

    pub fn function_claim(&mut self, value: Value, size: RV32Usize) {
        self.caller_stack
            .last_mut()
            .expect("Caller stack should not be empty when claiming function stack")
            .claim(value, size);
    }

    pub fn stack_alloc_size(&self) -> RV32Usize {
        self.stack_allocator.size()
    }

    pub fn stack_calculated_size(&self) -> RV32Usize {
        self.stack_allocator.calculated_size()
    }

    pub fn function_alloc_size(&self) -> RV32Usize {
        self.caller_stack
            .iter()
            .fold(0, |last, stack| last + stack.size())
    }

    pub fn function_calculated_size(&self) -> RV32Usize {
        self.caller_stack
            .iter()
            .fold(0, |last, stack| last + stack.calculated_size())
    }

    pub fn alloc_size(&self) -> RV32Usize {
        self.stack_alloc_size() + self.function_alloc_size()
    }

    pub fn calculated_size(&self) -> RV32Usize {
        self.stack_calculated_size() + self.function_calculated_size()
    }

    pub fn get_offset(&self, value: &Value, size: RV32Usize) -> Option<RV32Usize> {
        let mut offset = 0;
        self.caller_stack
            .iter()
            .rev()
            .find_map(|stack| {
                let res = stack.get_offset(value, size).map(|off| off + offset);
                offset += stack.size();
                res
            })
            .or(self
                .stack_allocator
                .get_offset(value, size)
                .map(|off| offset + off))
    }

    pub fn meta_offset(&self) -> RV32Usize {
        let offset = self
            .caller_stack
            .iter()
            .rev()
            .fold(0, |last, stack| last + stack.size());
        offset + self.stack_allocator.reserved_size() - self.stack_allocator.meta_size()
    }

    pub fn stack_extend(&mut self) -> Vec<RiscvAsm> {
        assert!(
            self.caller_stack.is_empty(),
            "Caller stack should be empty when extending stack"
        );
        self.stack_allocator.extend_stack()
    }

    pub fn function_extend(&mut self) -> Vec<RiscvAsm> {
        self.caller_stack
            .last_mut()
            .expect("Caller stack should not be empty when extending function stack")
            .extend_stack()
    }

    pub fn stack_resume(&mut self) -> Vec<RiscvAsm> {
        assert!(
            self.caller_stack.is_empty(),
            "Caller stack should be empty when resuming stack"
        );
        self.stack_allocator.resume_stack()
    }

    pub fn function_resume(&mut self) -> Vec<RiscvAsm> {
        self.caller_stack
            .last_mut()
            .expect("Caller stack should not be empty when resuming function stack")
            .resume_stack()
    }

    pub fn clear(&mut self) {
        self.caller_stack.clear();
        self.stack_allocator.clear();
    }

    pub fn function_clear(&mut self) {
        if let Some(stack) = self.caller_stack.last_mut() {
            stack.clear();
        }
    }

    pub fn register_func_arg(&mut self, arg_ref: &FuncArgRef, size: RV32Usize) {
        self.function_args.insert(arg_ref, size);
    }

    fn get_arg_register(&self, arg_ref: &FuncArgRef) -> Option<Register> {
        let idx = arg_ref.index();
        if idx < 8 {
            Some(Register::from((Register::A0 as u8) + idx as u8))
        } else {
            None
        }
    }

    pub fn get_arg(&mut self, arg_ref: &FuncArgRef, size: RV32Usize) -> Option<ArgLocation> {
        let idx = arg_ref.index();
        if idx < 8 {
            self.get_arg_register(arg_ref).map(ArgLocation::Register)
        } else {
            self.function_args
                .get_offset(arg_ref, size)
                .map(|offset| offset + self.alloc_size())
                .map(ArgLocation::Stack)
        }
    }

    pub fn new_function_call(&mut self) {
        self.caller_stack.push(StackSizeAllocator::new());
    }

    pub fn end_function_call(&mut self) {
        if self.caller_stack.pop().is_none() {
            panic!("Caller stack should not be empty when ending function call");
        }
    }

    pub fn set_meta_size(&mut self, size: RV32Usize) {
        self.stack_allocator.set_meta_size(size);
    }
}

pub struct CallGuard<'a> {
    memory_mapper: &'a mut MemoryMapper,
}

impl CallGuard<'_> {
    pub fn new(memory_mapper: &mut MemoryMapper) -> CallGuard<'_> {
        memory_mapper.new_function_call();
        CallGuard { memory_mapper }
    }

    pub fn inner(&mut self) -> &mut MemoryMapper {
        self.memory_mapper
    }
}

impl Drop for CallGuard<'_> {
    fn drop(&mut self) {
        self.memory_mapper.end_function_call();
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
    pub program: &'a Program,
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
    pub fn new<'a>(
        program: &'a Program,
        func_data: &'a FunctionData,
        id: NonZero<usize>,
    ) -> FunctionContext<'a> {
        let entry_id = func_data
            .layout()
            .entry_bb()
            .expect("FATAL: cannot generate asm for a function declaration.");
        FunctionContext {
            program,
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
    Call {
        func: String,
    },
    Li {
        dest: Register,
        imm: RV32Imm,
    },
    Lui {
        dest: Register,
        imm: RV32Imm,
    },
    Mv {
        dest: Register,
        src: Register,
    },
    La {
        dest: Register,
        label: String,
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
            RiscvInstruction::Call { func } => write!(f, "{}{} {}", INDENT, INST_CALL, func),
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
            RiscvInstruction::Lui { dest, imm } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_LOAD_UPPER_IMMEDIATE,
                    dest.name(),
                    imm
                )
            }
            RiscvInstruction::Mv { dest, src } => {
                write!(f, "{}{} {}, {}", INDENT, INST_MOVE, dest.name(), src.name())
            }
            RiscvInstruction::La { dest, label } => {
                write!(
                    f,
                    "{}{} {}, {}",
                    INDENT,
                    INST_LOAD_ADDRESS,
                    dest.name(),
                    label
                )
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
pub enum RiscvInit {
    Zero(RV32Usize),
    Word(RV32Imm),
}

impl Display for RiscvInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscvInit::Zero(size) => write!(f, "{}{} {}", INDENT, ZERO_INIT, size),
            RiscvInit::Word(value) => write!(f, "{}{} {}", INDENT, INIT_WORD, value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RiscvAsm {
    Section(String),
    Global(String),
    Label(String),
    Init(RiscvInit),
    Instruction(RiscvInstruction),
    None, // for formatting
}

impl Display for RiscvAsm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscvAsm::Section(name) => write!(f, "{}{}", INDENT, name),
            RiscvAsm::Global(name) => write!(f, "{}.globl {}", INDENT, name),
            RiscvAsm::Label(name) => write!(f, "{}:", name),
            RiscvAsm::Init(init) => write!(f, "{}", init),
            RiscvAsm::Instruction(inst) => write!(f, "{}", inst),
            RiscvAsm::None => Ok(()),
        }
    }
}
