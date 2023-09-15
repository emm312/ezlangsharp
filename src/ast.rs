use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Float(f64, SourceInfo),
    Int(i64, SourceInfo),
    StringLit(String, SourceInfo),
    Ident(String, SourceInfo),
    ArrayLit(Vec<Expr>, SourceInfo),
    ArrayIndex(String, Box<Expr>, SourceInfo),
    BinOp(Box<Expr>, BinOp, Box<Expr>, SourceInfo),
    Call(String, Vec<Expr>, SourceInfo),
    Return(Box<Expr>, SourceInfo),
    Break(SourceInfo),
    Continue(SourceInfo),
    BoolLit(bool, SourceInfo),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLvl {
    Import(String, SourceInfo),
    FuncDef(String, Vec<String>, Vec<Stmt>, SourceInfo),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr, SourceInfo),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>, SourceInfo),
    While(Expr, Vec<Stmt>, SourceInfo),
    Block(Vec<Stmt>, SourceInfo),
    Return(Expr, SourceInfo),
    Let(String, Expr, SourceInfo),
    Assign(String, Expr, SourceInfo),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    pub span: Range<usize>,
}

impl SourceInfo {
    pub fn new(range: Range<usize>) -> SourceInfo {
        SourceInfo { span: range }
    }
}
