use std::fmt::{self, Display};

use koopa::ir::Type;

#[derive(Debug, Clone)]
pub struct CompUnit {
    pub comp_unit: Option<Box<CompUnit>>,
    pub func_def: FuncDef,
}

/// 文法标识符
impl CompUnit {
    pub fn new(comp_unit: Option<CompUnit>, func_def: FuncDef) -> Self {
        Self {
            comp_unit: comp_unit.map(Box::new),
            func_def,
        }
    }
}

impl Display for CompUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func_def)
    }
}

#[derive(Debug, Clone)]
pub struct Components {
    pub list: Vec<FuncDef>,
}

impl Components {
    pub fn new(unit: CompUnit) -> Self {
        let mut defs = vec![unit.func_def];
        let mut maybe_unit = unit.comp_unit;
        while let Some(unit) = maybe_unit {
            defs.push(unit.func_def);
            maybe_unit = unit.comp_unit;
        }
        defs.reverse();
        Components { list: defs }
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
    pub fparams: Option<FuncFParams>,
    pub block: Block,
}

impl FuncDef {
    pub fn new(ret_type: BType, ident: String, fparams: Option<FuncFParams>, block: Block) -> Self {
        Self {
            ret_type,
            ident,
            fparams,
            block,
        }
    }
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}() {}", self.ret_type, self.ident, self.block)
    }
}

#[derive(Debug, Clone)]
pub struct FuncFParams {
    pub params: Vec<FuncFParam>,
}

impl FuncFParams {
    pub fn new(params: Vec<FuncFParam>) -> Self {
        Self { params }
    }
}
impl Display for FuncFParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FuncFParam {
    pub ty: BType,
    pub ident: String,
}

impl FuncFParam {
    pub fn new(ty: BType, ident: String) -> Self {
        Self { ty, ident }
    }
}

impl Display for FuncFParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.ident)
    }
}

#[derive(Debug, Clone)]
pub struct FuncRParams {
    pub params: Vec<Expr>,
}

impl FuncRParams {
    pub fn new(params: Vec<Expr>) -> Self {
        Self { params }
    }
}

impl Display for FuncRParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        Ok(())
    }
}

/// 一个块由多个项组成。
///
/// ```c, ignore
/// {   // block
///     int a = 0;  // BlockItem: Decl
///     return a;   // BlockItem: Stmt
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
///
/// Return(expr) <-  return 0;
#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Option<Expr>),
    Assign(LVal, Expr),
    Expr(Option<Expr>),
    If(Box<IfBranch>),
    Else(Box<ElseBranch>),
    Block(Block),
    IfElse(Box<IfBranch>, Box<ElseBranch>),
    While(Box<WhileBranch>),
    ControlFlow(ControlFlow),
}

impl Stmt {
    pub fn new_return(val: Option<Expr>) -> Self {
        Self::Return(val)
    }

    pub fn new_assign(lval: LVal, expr: Expr) -> Self {
        Self::Assign(lval, expr)
    }

    pub fn new_expr(expr: Option<Expr>) -> Self {
        Self::Expr(expr)
    }

    pub fn new_block(block: Block) -> Self {
        Self::Block(block)
    }

    pub fn new_if(if_branch: IfBranch) -> Self {
        Self::If(Box::new(if_branch))
    }

    pub fn new_else(else_branch: ElseBranch) -> Self {
        Self::Else(Box::new(else_branch))
    }

    pub fn new_if_else(if_branch: IfBranch, else_branch: ElseBranch) -> Self {
        Self::IfElse(Box::new(if_branch), Box::new(else_branch))
    }

    pub fn new_while(while_branch: WhileBranch) -> Self {
        Self::While(Box::new(while_branch))
    }

    pub fn new_control_flow(control_flow: ControlFlow) -> Self {
        Self::ControlFlow(control_flow)
    }

    pub fn new_break() -> Self {
        Self::ControlFlow(ControlFlow::new_break())
    }

    pub fn new_continue() -> Self {
        Self::ControlFlow(ControlFlow::new_continue())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Return(val) => match val {
                Some(val) => write!(f, "return {};", val),
                None => write!(f, "return;"),
            },
            Stmt::Assign(lval, expr) => write!(f, "{} = {};", lval, expr),
            Stmt::Expr(expr) => match expr {
                Some(e) => write!(f, "{};", e),
                None => write!(f, ";"),
            },
            Stmt::Block(block) => write!(f, "{}", block),
            Stmt::If(if_branch) => write!(f, "{}", if_branch),
            Stmt::Else(else_branch) => write!(f, "{}", else_branch),
            Stmt::IfElse(if_branch, else_branch) => {
                write!(f, "{}", if_branch)?;
                write!(f, " {}", else_branch)?;
                Ok(())
            }
            Stmt::While(while_branch) => write!(f, "{}", while_branch),
            Stmt::ControlFlow(control_flow) => write!(f, "{}", control_flow),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfBranch {
    pub cond: Expr,
    pub stmt: Stmt,
}

impl IfBranch {
    pub fn new(cond: Expr, stmt: Stmt) -> Self {
        Self { cond, stmt }
    }
}

impl Display for IfBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if ({}) {}", self.cond, self.stmt)
    }
}

#[derive(Debug, Clone)]
pub struct ElseBranch {
    pub stmt: Stmt,
}

impl ElseBranch {
    pub fn new(stmt: Stmt) -> Self {
        Self { stmt }
    }
}

impl Display for ElseBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "else {}", self.stmt)
    }
}

#[derive(Debug, Clone)]
pub struct WhileBranch {
    pub cond: Expr,
    pub stmt: Stmt,
}

impl WhileBranch {
    pub fn new(cond: Expr, stmt: Stmt) -> Self {
        Self { cond, stmt }
    }
}

impl Display for WhileBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {}", self.cond, self.stmt)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlFlow {
    Break,
    Continue,
}

impl ControlFlow {
    pub fn new_break() -> Self {
        Self::Break
    }

    pub fn new_continue() -> Self {
        Self::Continue
    }
}

impl Display for ControlFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlFlow::Break => write!(f, "break;"),
            ControlFlow::Continue => write!(f, "continue;"),
        }
    }
}

/// 常变量定义
#[derive(Debug, Clone)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

impl Decl {
    pub fn new_const(const_decl: ConstDecl) -> Self {
        Self::Const(const_decl)
    }

    pub fn new_var(var_decl: VarDecl) -> Self {
        Self::Var(var_decl)
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Const(const_decl) => write!(f, "{}", const_decl),
            Decl::Var(var_decl) => write!(f, "{}", var_decl),
        }
    }
}

/// 常量声明
///
/// ```c
/// //    ty  def
/// //     ↓   ↓
/// //    --- --------
/// const int x, y = 0;
/// ```
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
    Int,  // int
    Void, // void
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
            BType::Void => write!(f, "void"),
        }
    }
}

impl From<BType> for Type {
    fn from(btype: BType) -> Self {
        match btype {
            BType::Int => Type::get_i32(),
            BType::Void => Type::get_unit(),
        }
    }
}

/// 常量定义
///
/// ```c
/// //    indent init_val
/// //        ↓   ↓
/// //        -   -
/// const int x = 0, y = 1;
/// ```
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
///
/// ```c
/// //              expr
/// //                ↓
/// //            ---------
/// const int a = 1 + 2 * 3;
/// ```
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

/// 变量声明
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ty: BType,
    pub def: Vec<VarDef>,
}

impl VarDecl {
    pub fn new(ty: BType, def: Vec<VarDef>) -> Self {
        Self { ty, def }
    }
}

impl Display for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.ty)?;
        for (i, def) in self.def.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", def)?;
        }
        write!(f, ";")?;
        Ok(())
    }
}

/// 变量定义
#[derive(Debug, Clone)]
pub struct VarDef {
    pub ident: String,
    pub init_val: Option<InitVal>,
}

impl VarDef {
    pub fn new(ident: String, init_val: Option<InitVal>) -> Self {
        Self { ident, init_val }
    }
}

impl Display for VarDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.init_val {
            Some(init_val) => write!(f, "{} = {}", self.ident, init_val),
            None => write!(f, "{}", self.ident),
        }
    }
}

/// 初值
#[derive(Debug, Clone)]
pub struct InitVal {
    pub expr: Expr,
}

impl InitVal {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

impl Display for InitVal {
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::new(LOrExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::new(LOrExpr::new_unary(unary))
    }

    pub fn new_mul(mul: MulExpr) -> Self {
        Self::new(LOrExpr::new_mul(mul))
    }

    pub fn new_add(add: AddExpr) -> Self {
        Self::new(LOrExpr::new_add(add))
    }

    pub fn new_rel(rel: RelExpr) -> Self {
        Self::new(LOrExpr::new_rel(rel))
    }

    pub fn new_eq(eq: EqExpr) -> Self {
        Self::new(LOrExpr::new_eq(eq))
    }

    pub fn new_land(land: LAndExpr) -> Self {
        Self::new(LOrExpr::new_land(land))
    }

    pub fn new_lor(lor: LOrExpr) -> Self {
        Self::new(lor)
    }

    pub fn new_num(num: Number) -> Self {
        Self::new(LOrExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::new(LOrExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::new(LOrExpr::new_expr(expr))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// 或
///
/// ```c
/// //    LOrExpr
/// //       ↓
/// //     ------
/// return 1 || 2;
/// ```
#[derive(Debug, Clone)]
pub enum LOrExpr {
    And(LAndExpr),
    Binary(Box<LOrExpr>, Box<LAndExpr>),
}

impl LOrExpr {
    pub fn new_land(land: LAndExpr) -> Self {
        Self::And(land)
    }

    pub fn new_binary(left: LOrExpr, right: LAndExpr) -> Self {
        Self::Binary(Box::new(left), Box::new(right))
    }

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::And(LAndExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::And(LAndExpr::new_unary(unary))
    }

    pub fn new_mul(mul: MulExpr) -> Self {
        Self::And(LAndExpr::new_mul(mul))
    }

    pub fn new_add(add: AddExpr) -> Self {
        Self::And(LAndExpr::new_add(add))
    }

    pub fn new_rel(rel: RelExpr) -> Self {
        Self::And(LAndExpr::new_rel(rel))
    }

    pub fn new_eq(eq: EqExpr) -> Self {
        Self::And(LAndExpr::new_eq(eq))
    }

    pub fn new_num(num: Number) -> Self {
        Self::And(LAndExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::And(LAndExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::And(LAndExpr::new_expr(expr))
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
///
/// ```c
/// //    LAndExpr
/// //       ↓
/// //     ------
/// return 1 && 2;
/// ```
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Eq(EqExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::Eq(EqExpr::new_unary(unary))
    }

    pub fn new_mul(mul: MulExpr) -> Self {
        Self::Eq(EqExpr::new_mul(mul))
    }

    pub fn new_add(add: AddExpr) -> Self {
        Self::Eq(EqExpr::new_add(add))
    }

    pub fn new_rel(rel: RelExpr) -> Self {
        Self::Eq(EqExpr::new_rel(rel))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Eq(EqExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Eq(EqExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Eq(EqExpr::new_expr(expr))
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
///
/// ```c
/// //     EqExpr
/// //       ↓
/// //     ------
/// return 1 == 2;
/// return 1 != 2;
/// ```
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Rel(RelExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::Rel(RelExpr::new_unary(unary))
    }

    pub fn new_mul(mul: MulExpr) -> Self {
        Self::Rel(RelExpr::new_mul(mul))
    }

    pub fn new_add(add: AddExpr) -> Self {
        Self::Rel(RelExpr::new_add(add))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Rel(RelExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Rel(RelExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Rel(RelExpr::new_expr(expr))
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
///
/// ```c
/// //    RelExpr
/// //       ↓
/// //     ------
/// return 1 >  2;
/// return 1 <  2;
/// return 1 >= 2;
/// return 1 <= 2;
/// ```
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Add(AddExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::Add(AddExpr::new_unary(unary))
    }

    pub fn new_mul(mul: MulExpr) -> Self {
        Self::Add(AddExpr::new_mul(mul))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Add(AddExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Add(AddExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Add(AddExpr::new_expr(expr))
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
///
/// ```c
/// //    AddExpr
/// //       ↓
/// //     -----
/// return 1 + 2;
/// return 1 - 2;
/// ```
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Mul(MulExpr::new_primary(primary))
    }

    pub fn new_unary(unary: UnaryExpr) -> Self {
        Self::Mul(MulExpr::new_unary(unary))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Mul(MulExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Mul(MulExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Mul(MulExpr::new_expr(expr))
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
///
/// ```c
/// //    MulExpr
/// //       ↓
/// //     -----
/// return 1 * 2;
/// return 1 / 2;
/// return 1 % 2;
/// ```
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

    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Unary(UnaryExpr::new_primary(primary))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Unary(UnaryExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Unary(UnaryExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Unary(UnaryExpr::new_expr(expr))
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
///
/// ```c
/// //  UnaryExpr
/// //      ↓
/// //     ---
/// return + 1;
/// return - 1;
/// return ! 1;
/// ```
#[derive(Debug, Clone)]
pub enum UnaryExpr {
    Primary(PrimaryExpr),
    UnaryOp(UnaryOp, Box<UnaryExpr>),
    Call(FuncCall),
}

impl UnaryExpr {
    pub fn new_primary(primary: PrimaryExpr) -> Self {
        Self::Primary(primary)
    }

    pub fn new_unary_op(op: UnaryOp, expr: UnaryExpr) -> Self {
        Self::UnaryOp(op, Box::new(expr))
    }

    pub fn new_num(num: Number) -> Self {
        Self::Primary(PrimaryExpr::new_num(num))
    }

    pub fn new_lval(lval: LVal) -> Self {
        Self::Primary(PrimaryExpr::new_lval(lval))
    }

    pub fn new_expr(expr: Expr) -> Self {
        Self::Primary(PrimaryExpr::new_expr(expr))
    }

    pub fn new_call(func_call: FuncCall) -> Self {
        Self::Call(func_call)
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::Primary(primary) => write!(f, "{}", primary),
            UnaryExpr::UnaryOp(op, expr) => write!(f, "{} {}", op, expr),
            UnaryExpr::Call(func_call) => write!(f, "{}", func_call),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncCall {
    pub ident: String,
    pub args: Option<FuncRParams>,
}

impl FuncCall {
    pub fn new(ident: String, args: Option<FuncRParams>) -> Self {
        Self { ident, args }
    }
}

impl Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.args {
            Some(args) => write!(f, "{}({})", self.ident, args),
            None => write!(f, "{}()", self.ident),
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
