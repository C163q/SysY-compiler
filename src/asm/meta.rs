use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use koopa::ir::{FunctionData, Value};

pub const INDENT: &str = "  ";

pub const TEXT_SECTION: &str = ".text";

pub const GLOBAL_SYMBOL: &str = ".globl";

pub const REGISTER_COUNT: usize = 32;
pub const REGISTER_NAMES: [&str; REGISTER_COUNT] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub const INST_LOAD_IMMEDIATE: &str = "li";
pub const INST_RETURN: &str = "ret";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Register {
    pub fn name(&self) -> &'static str {
        REGISTER_NAMES[*self as u8 as usize]
    }
}

pub struct FunctionContext<'a> {
    pub func_data: &'a FunctionData,
    pub register_map: HashMap<Value, Register>,
    pub register_usage: HashSet<Register>,
    /// i32为偏移量
    pub memory_map: HashMap<Value, i32>,
}

impl Debug for FunctionContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionContext")
            .field("func_data", &"&FunctionData")
            .field("register_map", &self.register_map)
            .field("register_usage", &self.register_usage)
            .field("memory_map", &self.memory_map)
            .finish()
    }
}

impl FunctionContext<'_> {
    pub fn new<'a>(
        func_data: &'a FunctionData,
        register_map: HashMap<Value, Register>,
        register_usage: HashSet<Register>,
        memory_map: HashMap<Value, i32>,
    ) -> FunctionContext<'a> {
        FunctionContext {
            func_data,
            register_map,
            register_usage,
            memory_map,
        }
    }
}

/// 产生汇编代码
///
/// 对于某些IR数据，可能需要首先注册才行，否则无法直接使用它们的名字（例如函数）。
pub trait ToAsm {
    /// 产生汇编代码，不负责注册。
    fn to_asm(&self, func_data: Option<&mut FunctionContext<'_>>) -> Vec<String>;

    /// 注册，对于某些全局数据，这是必须的步骤。
    fn register(&self) -> Option<String> {
        None
    }
}
