use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle};

use crate::ast::{Stmt, TopLvl};

use super::Function;

pub struct FunctionChecker {
    funcs: Vec<Function>,
}

impl FunctionChecker {
    pub fn new(ast: &Vec<TopLvl>) -> FunctionChecker {
        let mut funcs = Vec::new();
        for line in ast {
            match line {
                TopLvl::FuncDef(name, args, body, source_info) => {
                    let func = Function {
                        body: body.clone(),
                        name: name.clone(),
                        args: args.clone(),
                        source_info: source_info.clone(),
                    };
                    funcs.push(func);
                }
                _ => (),
            }
        }
        FunctionChecker { funcs }
    }

    pub fn main_func_test(&self) -> Option<Diagnostic<usize>> {
        let mut main_func = false;
        for func in &self.funcs {
            if func.name == "main" {
                main_func = true;
            }
        }
        if !main_func {
            return Some(
                Diagnostic::error()
                    .with_message("No main function found")
                    .with_notes(vec!["Try defining a function called main".to_string()]),
            );
        }
        None
    }

    /// makes sure you cant do f(x, y) when the function is only f(x)
    pub fn check_expr_args(&self) -> Option<Vec<Diagnostic<usize>>> {
        let mut errors = Vec::new();
        for func in &self.funcs {
            for stmt in &func.body {
                if let Stmt::Expr(expr, ..) = stmt {
                    match expr {
                        crate::ast::Expr::Call(name, args, info) => {
                            let callee_option = self.find_func(name.clone());
                            if let None = callee_option {
                                errors.push(
                                    Diagnostic::error()
                                        .with_message("Function not found")
                                        .with_labels(vec![Label::primary(0, info.span.clone())
                                            .with_message(format!("Function {} not found", name))]),
                                );
                                continue;
                            }
                            let callee = callee_option.unwrap();
                            let arg_count = args.len();
                            if arg_count != callee.args.len() {
                                errors.push(
                                    Diagnostic::error()
                                        .with_message("Function called with too many args")
                                        .with_labels(vec![
                                            Label::primary(0, info.span.clone()).with_message(
                                                format!(
                                                    "Expected {} arguments, got {}",
                                                    callee.args.len(),
                                                    arg_count
                                                ),
                                            ),
                                            Label::secondary(0, callee.source_info.span.clone())
                                                .with_message(format!(
                                                    "Note: Function defined here"
                                                )),
                                        ]),
                                );
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        if errors.len() > 0 {
            return Some(errors);
        }
        None
    }

    pub fn run_checks(&self) -> Option<Vec<Diagnostic<usize>>> {
        let mut errors = Vec::new();
        if let Some(error) = self.main_func_test() {
            errors.push(error);
        }
        if let Some(error) = self.check_expr_args() {
            errors.extend(error);
        }
        if errors.len() > 0 {
            return Some(errors);
        }
        None
    }

    fn find_func(&self, name: String) -> Option<&Function> {
        self.funcs.iter().filter(|e| e.name == name).nth(0)
    }
}
