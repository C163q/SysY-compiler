use std::fmt::{self, Display};

use koopa::ir::Type;

#[derive(Debug, Clone)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

/// 文法标识符
impl CompUnit {
    pub fn new(func_def: FuncDef) -> Self {
        Self { func_def }
    }
}

impl Display for CompUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func_def)
    }
}

/// 函数的定义
///
/// ```c, ignore
/// // func_type  ident
/// //    ↓         ↓
///      int      main() {
///         // block
///      }
/// ```
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub ret_type: BType,
    pub ident: String,
    pub block: Block,
}

impl FuncDef {
    pub fn new(ret_type: BType, ident: String, block: Block) -> Self {
        Self {
            ret_type,
            ident,
            block,
        }
    }
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}() {}", self.ret_type, self.ident, self.block)
    }
}

/// 一个块由多个项组成。
///
/// ```c, ignore
/// {   // block
///     int a = 0;  // Decl
///     return a;   // Stmt
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

impl Block {
    pub fn new(items: Vec<BlockItem>) -> Self {
        Self { items }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for stmt in &self.items {
            write!(f, " {} ", stmt)?;
        }
        write!(f, "}}")
    }
}

/// 项
#[derive(Debug, Clone)]
pub enum BlockItem {
    Stmt(Stmt),
    Decl(Decl),
}

impl BlockItem {
    pub fn new_stmt(stmt: Stmt) -> Self {
        Self::Stmt(stmt)
    }

    pub fn new_decl(decl: Decl) -> Self {
        Self::Decl(decl)
    }
}

impl Display for BlockItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockItem::Stmt(stmt) => write!(f, "{}", stmt),
            BlockItem::Decl(decl) => write!(f, "{}", decl),
        }
    }
}

/// 语句
///
/// ```c, ignore
/// return 0;   // Stmt
/// ```
/// Return(expr) <-  return 0;
#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
}

impl Stmt {
    pub fn new_return(val: Expr) -> Self {
        Self::Return(val)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Return(val) => write!(f, "return {}", val),
        }
    }
}

/// 常变量定义
#[derive(Debug, Clone)]
pub enum Decl {
    Const(ConstDecl),
}

impl Decl {
    pub fn new_const(const_decl: ConstDecl) -> Self {
        Self::Const(const_decl)
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Const(const_decl) => write!(f, "{}", const_decl),
        }
    }
}

/// 常量声明
#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub ty: BType,
    pub def: Vec<ConstDef>,
}

impl ConstDecl {
    pub fn new(ty: BType, def: Vec<ConstDef>) -> Self {
        Self { ty, def }
    }
}

impl Display for ConstDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "const {} ", self.ty)?;
        for (i, def) in self.def.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", def)?;
        }
        Ok(())
    }
}

/// 类型
#[derive(Debug, Clone, Copy)]
pub enum BType {
    Int,
}

impl BType {
    pub fn new_int() -> Self {
        Self::Int
    }
}

impl Display for BType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BType::Int => write!(f, "int"),
        }
    }
}

impl From<BType> for Type {
    fn from(btype: BType) -> Self {
        match btype {
            BType::Int => Type::get_i32(),
        }
    }
}

/// 常量定义
#[derive(Debug, Clone)]
pub struct ConstDef {
    pub ident: String,
    pub init_val: ConstInitVal,
}

impl ConstDef {
    pub fn new(ident: String, init_val: ConstInitVal) -> Self {
        Self { ident, init_val }
    }
}

impl Display for ConstDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.ident, self.init_val)
    }
}

/// 常量初值
#[derive(Debug, Clone)]
pub struct ConstInitVal {
    pub expr: ConstExpr,
}

impl ConstInitVal {
    pub fn new(expr: ConstExpr) -> Self {
        Self { expr }
    }
}

impl Display for ConstInitVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// 常量表达式
#[derive(Debug, Clone)]
pub struct ConstExpr {
    pub expr: Expr,
}

impl ConstExpr {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

impl Display for ConstExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// 表达式
#[derive(Debug, Clone)]
pub struct Expr {
    pub expr: Box<LOrExpr>,
}

impl Expr {
    pub fn new(expr: LOrExpr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// 或
#[derive(Debug, Clone)]
pub enum LOrExpr {
    And(LAndExpr),
    Binary(Box<LOrExpr>, Box<LAndExpr>),
}

impl LOrExpr {
    pub fn new_and(and: LAndExpr) -> Self {
        Self::And(and)
    }

    pub fn new_binary(left: LOrExpr, right: LAndExpr) -> Self {
        Self::Binary(Box::new(left), Box::new(right))
    }
}

impl Display for LOrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LOrExpr::And(and) => write!(f, "{}", and),
            LOrExpr::Binary(left, right) => write!(f, "{} || {}", left, right),
        }
    }
}

/// 与
#[derive(Debug, Clone)]
pub enum LAndExpr {
    Eq(EqExpr),
    Binary(Box<LAndExpr>, Box<EqExpr>),
}

impl LAndExpr {
    pub fn new_eq(eq: EqExpr) -> Self {
        Self::Eq(eq)
    }

    pub fn new_binary(left: LAndExpr, right: EqExpr) -> Self {
        Self::Binary(Box::new(left), Box::new(right))
    }
}

impl Display for LAndExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LAndExpr::Eq(eq) => write!(f, "{}", eq),
            LAndExpr::Binary(left, right) => write!(f, "{} && {}", left, right),
        }
    }
}

/// 等于比较运算符
#[derive(Debug, Clone, Copy)]
pub enum EqOp {
    Eq,    // ==
    NotEq, // !=
}

impl Display for EqOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EqOp::Eq => write!(f, "=="),
            EqOp::NotEq => write!(f, "!="),
        }
    }
}

/// 等于比较表达式
#[derive(Debug, Clone)]
pub enum EqExpr {
    Rel(RelExpr),
    Binary(Box<EqExpr>, EqOp, Box<RelExpr>),
}

impl EqExpr {
    pub fn new_rel(rel: RelExpr) -> Self {
        Self::Rel(rel)
    }

    pub fn new_binary(left: EqExpr, op: EqOp, right: RelExpr) -> Self {
        Self::Binary(Box::new(left), op, Box::new(right))
    }
}

impl Display for EqExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EqExpr::Rel(rel) => write!(f, "{}", rel),
            EqExpr::Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
        }
    }
}

/// 比较运算符
#[derive(Debug, Clone, Copy)]
pub enum RelOp {
    Lt, // <
    Gt, // >
    Le, // <=
    Ge, // >=
}

impl Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::Lt => write!(f, "<"),
            RelOp::Gt => write!(f, ">"),
            RelOp::Le => write!(f, "<="),
            RelOp::Ge => write!(f, ">="),
        }
    }
}

/// 比较表达式
#[derive(Debug, Clone)]
pub enum RelExpr {
    Add(AddExpr),
    Binary(Box<RelExpr>, RelOp, Box<AddExpr>),
}

impl RelExpr {
    pub fn new_add(add: AddExpr) -> Self {
        Self::Add(add)
    }

    pub fn new_binary(left: RelExpr, op: RelOp, right: AddExpr) -> Self {
        Self::Binary(Box::new(left), op, Box::new(right))
    }
}

impl Display for RelExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelExpr::Add(add) => write!(f, "{}", add),
            RelExpr::Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
        }
    }
}

/// 加减法运算符
#[derive(Debug, Clone, Copy)]
pub enum AddOp {
    Add, // +
    Sub, // -
}

impl Display for AddOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddOp::Add => write!(f, "+"),
            AddOp::Sub => write!(f, "-"),
        }
    }
}

/// 加减法表达式
#[derive(Debug, Clone)]
pub enum AddExpr {
    Mul(MulExpr),
    Binary(Box<AddExpr>, AddOp, Box<MulExpr>),
}

impl AddExpr {
    pub fn new_mul(mul: MulExpr) -> Self {
        Self::Mul(mul)
    }

    pub fn new_binary(left: AddExpr, op: AddOp, right: MulExpr) -> Self {
        Self::Binary(Box::new(left), op, Box::new(right))
    }
}

impl Display for AddExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddExpr::Mul(mul) => write!(f, "{}", mul),
            AddExpr::Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
        }
    }
}

/// 乘除法运算符
#[derive(Debug, Clone, Copy)]
pub enum MulOp {
    Mul, // *
    Div, // /
    Mod, // %
}

impl Display for MulOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MulOp::Mul => write!(f, "*"),
            MulOp::Div => write!(f, "/"),
            MulOp::Mod => write!(f, "%"),
        }
    }
}

/// 乘除法表达式
#[derive(Debug, Clone)]
pub enum MulExpr {
    Unary(UnaryExpr),
    Binary(Box<MulExpr>, MulOp, Box<UnaryExpr>),
}

impl MulExpr {
    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::Unary(unary)
    }

    pub fn new_binary(left: MulExpr, op: MulOp, right: UnaryExpr) -> Self {
        Self::Binary(Box::new(left), op, Box::new(right))
    }
}

impl Display for MulExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MulExpr::Unary(unary) => write!(f, "{}", unary),
            MulExpr::Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
        }
    }
}

/// 一元运算符
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Pos, // +
    Neg, // -
    Not, // !
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Pos => write!(f, "+"),
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

/// 一元表达式
#[derive(Debug, Clone)]
pub enum UnaryExpr {
    Primary(PrimaryExpr),
    UnaryOp(UnaryOp, Box<UnaryExpr>),
}

impl UnaryExpr {
    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Primary(primary)
    }

    pub fn new_unary_op(op: UnaryOp, expr: UnaryExpr) -> Self {
        Self::UnaryOp(op, Box::new(expr))
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::Primary(primary) => write!(f, "{}", primary),
            UnaryExpr::UnaryOp(op, expr) => write!(f, "{} {}", op, expr),
        }
    }
}

/// 基础表达式
#[derive(Debug, Clone)]
pub enum PrimaryExpr {
    Num(Number),
    Expr(Box<Expr>),
    LVal(LVal),
}

impl PrimaryExpr {
    pub fn new_num(num: Number) -> Self {
        Self::Num(num)
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Expr(Box::new(expr))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::LVal(lval)
    }
}

impl Display for PrimaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimaryExpr::Num(num) => write!(f, "{}", num),
            PrimaryExpr::Expr(expr) => write!(f, "({})", expr),
            PrimaryExpr::LVal(lval) => write!(f, "{}", lval),
        }
    }
}

/// 左值
#[derive(Debug, Clone)]
pub struct LVal {
    pub ident: String,
}

impl LVal {
    pub fn new(ident: String) -> Self {
        Self { ident }
    }
}

impl Display for LVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

/// 数字
/// TODO: 变量
#[derive(Debug, Clone)]
pub struct Number {
    pub val: i32,
}

impl Number {
    pub fn new(val: i32) -> Self {
        Self { val }
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
