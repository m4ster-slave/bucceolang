mod ast_types;
mod interpreter;
mod object;
mod parser;
mod parser_error;
mod runtime_error;
mod scanner;
mod scanner_error;
mod token;

use interpreter::Interpreter;
use parser::parse;
use scanner::tokenize;
use std::process::ExitCode;
use token::Token;

fn main() -> ExitCode {
    let source = "1 + 33 / 0";
    println!("\nRunning: {}", source);

    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(65);
        }
    };
    println!("Tokens: {:?}", tokens);

    let expr = match parse(tokens) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(65);
        }
    };

    match expr.accept(&Interpreter) {
        Ok(res) => {
            println!("Result: {}", res);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Runtime Error: {}", err);
            ExitCode::from(70)
        }
    }
}
