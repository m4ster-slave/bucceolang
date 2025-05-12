mod interpreter;
mod parser;
mod parser_error;
mod runtime_error;
mod scanner;

use interpreter::Interpreter;
use parser::parse;
use scanner::{tokenize, Token};

fn main() {
    let source = "1 + (1 * 10)";
    run(source);
}

fn run(source: &str) {
    println!("\nRunning: {}", source);

    // Tokenize
    let tokens = tokenize(source);
    println!("Tokens: {:?}", tokens);

    // Parse
    match parse(tokens) {
        Ok(expr) => {
            // Interpret
            let interpreter = Interpreter;
            match expr.accept(&interpreter) {
                Ok(result) => println!("Result: {}", result),
                Err(error) => eprintln!("Runtime Error: {}", error),
            }
        }
        Err(error) => {
            println!("Parsing Error: {}", error);
        }
    }
}
