use crate::ast::{SourceInfo, BinOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Array(Box<Type>),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpr {
    Float(f64, SourceInfo),
    Int(i64, SourceInfo),
    StringLit(String, SourceInfo),
    Ident(String, SourceInfo),
    ArrayLit(Vec<TypedExpr>, SourceInfo),
    ArrayIndex(String, Box<TypedExpr>, SourceInfo, Type),
    BinOp(Box<TypedExpr>, BinOp, Box<TypedExpr>, SourceInfo, Type),
    Call(String, Vec<TypedExpr>, SourceInfo, Type),
    Return(Box<TypedExpr>, SourceInfo),
    Break(SourceInfo),
    Continue(SourceInfo),
    BoolLit(bool, SourceInfo),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedTopLvl {
    Import(String, SourceInfo),
    FuncDef(String, Vec<(String, Type)>, Vec<TypedStmt>, SourceInfo),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedStmt {
    Expr(TypedExpr, SourceInfo),
    If(TypedExpr, Vec<TypedStmt>, Option<Vec<TypedStmt>>, SourceInfo),
    While(TypedExpr, Vec<TypedStmt>, SourceInfo),
    Block(Vec<TypedStmt>, SourceInfo),
    Return(TypedExpr, SourceInfo),
    Let(String, TypedExpr, SourceInfo),
    Assign(String, TypedExpr, SourceInfo),
    Error,
}
