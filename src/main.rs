mod callable;
mod environment;
mod expr_types;
mod function;
mod interpreter;
mod native_functions;
mod object;
mod parser;
mod parser_error;
mod runtime_error;
mod scanner;
mod scanner_error;
mod stmt_types;
mod token;

use interpreter::Interpreter;
use parser::parse;
use scanner::tokenize;
use token::Token;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

/** # Bucceolang

This project implements a simple tree-walking interpreter for a small, dynamically-typed language.

The interpreter processes source code in several stages:

1.  **Scanning (Lexing):** The input source code is broken down into a sequence of tokens by the `scanner` module.
2.  **Parsing:** The `parser` module takes the token stream and constructs an abstract syntax tree (AST) representing the structure of the program.
3.  **Interpretation (Evaluation):** The `interpreter` module traverses the AST and executes the program's instructions, managing the runtime environment and producing results.

## Language Features (Conceptual)

The language supported by this interpreter includes:

*   Basic arithmetic operations (`+`, `-`, `*`, `/`)
*   Comparison operators (`==`, `!=`, `>`, `>=`, `<`, `<=`)
*   Logical operators (`&&`, `||`, `!`)
*   Literal values: numbers, strings, booleans (`true`, `false`), and `nil`.
*   Variable declarations (`var`)
*   Printing to the console (`print`)
*   Control flow: `if` statements, `while` loops, `for` loops
*   Functions (`fn`)
*   Classes (`class`)
*/

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File mode - read and run the specified file
        let file_path = &args[1];

        if !file_path.ends_with(".bl") {
            eprintln!("\x1b[31;49;1mError: File must have .bl extension\x1b[0m");
            return ExitCode::from(64);
        }

        let source = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "\x1b[31;49;1mError reading file '{}': {}\x1b[0m",
                    file_path, e
                );
                return ExitCode::from(66);
            }
        };

        run(&source)
    } else {
        run_repl()
    }
}

fn run(source: &str) -> ExitCode {
    // println!("\nRunning: {}", source);
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(65);
        }
    };
    let mut stmts = match parse(tokens) {
        Ok(e) => e,
        Err(errors) => {
            eprintln!(
                "\x1b[31;1;4mParser exited with {} error(s).\x1b[0m",
                errors.len()
            );
            return ExitCode::from(65);
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interprete(&mut stmts) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\x1b[31;49;1mRuntime Error: {}\x1b[0m", err);
            ExitCode::from(70)
        }
    }
}

fn run_repl() -> ExitCode {
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                }

                let tokens = match tokenize(&input) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                let mut stmts = match parse(tokens) {
                    Ok(e) => e,
                    Err(errors) => {
                        // Handle parsing errors but continue REPL
                        for error in errors {
                            eprintln!("Parse error: {:?}", error);
                        }
                        continue;
                    }
                };

                match interpreter.interprete(&mut stmts) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Runtime Error: {}", err);
                        // Continue REPL even after runtime errors
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                return ExitCode::from(74);
            }
        }
    }

    ExitCode::SUCCESS
}
