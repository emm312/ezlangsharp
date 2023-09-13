use std::{ops::Range, fmt::Display};

use lalrpop_util::{lexer::Token, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Float(f64),
    Int(i64),
    StringLit(String),
    Ident(String),
    ArrayLit(Vec<Expr>),
    ArrayIndex(String, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Return(Box<Expr>),
    Break,
    Continue,
    Error
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
    Import(String),
    FuncDef(String, Vec<String>, Vec<Stmt>),
    Error
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Block(Vec<Stmt>),
    Return(Expr),
    Let(String, Expr),
    Assign(String, Expr),
    Error
}

pub struct Error<L, T, E> {
    pub err: ParseError<L, T, E>
}

impl<L, T, E> Error<L, T, E> {
    pub fn new(err: ParseError<L, T, E>) -> Self {
        Self {
            err
        }
    }
}
