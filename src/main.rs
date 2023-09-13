use std::process::exit;

use clap::Parser;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use ezlangsharp::grammar;
use lalrpop_util::{lexer::Token, ParseError};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg()]
    input_path: String,
    #[arg(short, long, default_value_t = false)]
    dump_ast: bool,
}

fn main() {
    let args = Args::parse();
    let mut files = SimpleFiles::new();
    let content = std::fs::read_to_string(&args.input_path).unwrap();
    files.add(&args.input_path, &content);
    let mut errors = Vec::new();
    let ast = grammar::EzLangSharpParser::new()
        .parse(&mut errors, &content)
        .unwrap_or_else(|err| {
            println!("{}", err);
            exit(-1);
        });
    if args.dump_ast {
        println!("{:#?}", ast);
    }
    let mut should_exit = false;
    if !errors.is_empty() {
        print_errors(&mut files, &errors);
        should_exit = true;
    }

    let diagnostics = ezlangsharp::semantic_analysis::run_all_checks(&ast);
    if !diagnostics.is_empty() {
        for diag in diagnostics {
            term::emit(
                &mut StandardStream::stderr(ColorChoice::Always).lock(),
                &codespan_reporting::term::Config::default(),
                &files,
                &diag,
            )
            .unwrap();
        }
        should_exit = true;
    }

    if should_exit {
        exit(-1);
    }
}

pub fn print_errors(
    files: &mut SimpleFiles<&String, &String>,
    errors: &Vec<ParseError<usize, Token<'_>, &'static str>>,
) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for err in errors {
        let mut diagnostic: Diagnostic<usize> = Diagnostic::error();
        match err {
            ParseError::InvalidToken { location } => {
                diagnostic = diagnostic.with_message("Invalid token");
                diagnostic =
                    diagnostic.with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                        0,
                        *location..*location,
                    )
                    .with_message("Invalid token")]);
            }
            ParseError::ExtraToken { token } => {
                diagnostic = diagnostic.with_message("Extra token");
                diagnostic =
                    diagnostic.with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                        0,
                        token.0..token.2,
                    )
                    .with_message("Extra token")]);
            }
            ParseError::UnrecognizedEof { location, expected } => {
                diagnostic = diagnostic.with_message("Unexpected end of file");
                diagnostic =
                    diagnostic.with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                        0,
                        *location..*location,
                    )
                    .with_message(format!("Expected one of: {}", expected.join(", ")))]);
            }
            ParseError::UnrecognizedToken { token, expected } => {
                diagnostic = diagnostic.with_message("Unrecognized token");
                diagnostic =
                    diagnostic.with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                        0,
                        token.0..token.2,
                    )
                    .with_message(format!("Expected one of: {}", expected.join(", ")))]);
            }
            _ => unreachable!(),
        }
        term::emit(&mut writer.lock(), &config, files, &diagnostic).unwrap();
    }
}
