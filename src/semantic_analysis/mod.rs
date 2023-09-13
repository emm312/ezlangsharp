use codespan_reporting::diagnostic::Diagnostic;

use crate::ast::{SourceInfo, Stmt, TopLvl};

pub mod basic_checks;
pub mod typechecking;

// TODO:
//      - Check for main function
//      - Check for unused variables
//      - Check for repeated function defs

pub struct Function {
    pub body: Vec<Stmt>,
    pub name: String,
    pub args: Vec<String>,
    pub source_info: SourceInfo,
}

pub fn run_all_checks(ast: &Vec<TopLvl>) -> Vec<Diagnostic<usize>> {
    let mut diagnostics = Vec::new();
    let func_checker = basic_checks::FunctionChecker::new(ast);
    if let Some(diag) = func_checker.main_func_test() {
        diagnostics.push(diag);
    }
    if let Some(diags) = func_checker.check_expr_args() {
        diagnostics.extend(diags);
    }
    diagnostics
}
