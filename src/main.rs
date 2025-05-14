mod environment;
mod expr_types;
mod interpreter;
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

/** # My Interpreter

This crate implements a simple tree-walking interpreter for a small, dynamically-typed language.

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
*   Functions (`fun`)
*   Classes (`class`)

## Modules

The crate is organized into the following modules:

*   `scanner`: Handles the lexical analysis (scanning) of the input.
*   `token`: Defines the `Token` struct and `TokenType` enum used by the scanner and parser.
*   `runtime_error`: Defines the `RuntimeError` enum for reporting errors during interpretation.
*   `parser`: Implements the recursive descent parser to build the AST.
*   `ast`: Defines the Abstract Syntax Tree structure (`Expr`, `Stmt`, visitor patterns).
*   `object`: Defines the `Object` enum representing runtime values.
*   `interpreter`: Implements the tree-walking interpreter to execute the AST.
*   `environment`: Manages the runtime environment (variable scope).

## Getting Started

To use this interpreter, you would typically:

1.  Instantiate a `Scanner` with your source code string.
2.  Call the scanner's method to produce a list of `Token`s.
3.  Handle any scanning errors.
4.  Instantiate a `Parser` with the list of tokens.
5.  Call the parser's method to produce an AST (e.g., a list of statements).
6.  Handle any parsing errors.
7.  Instantiate an `Interpreter`.
8.  Call the interpreter's method to execute the AST.
9.  Handle any runtime errors.

## Error Handling

The interpreter defines specific error types for scanning (`ScannerError`),
parsing (`ParseError`), and runtime execution (`RuntimeError`) to provide
detailed information about where and why an error occurred.

## Future Improvements

Possible areas for future development include:

*   Adding more language features (arrays, dictionaries, etc.)
*   Implementing a just-in-time (JIT) compiler for performance.
*   Adding a standard library of built-in functions.
*   Optimizing the interpreter's performance.

This documentation provides an overview of the interpreter's structure and
functionality. Refer to the documentation for individual modules and types
for more detailed information.
*/

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File mode - read and run the specified file
        let file_path = &args[1];

        if !file_path.ends_with(".bl") {
            eprintln!("Error: File must have .bl extension");
            return ExitCode::from(64);
        }

        let source = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                return ExitCode::from(66);
            }
        };

        run(&source)
    } else {
        run_repl()
    }
}

fn run(source: &str) -> ExitCode {
    println!("\nRunning: {}", source);
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(65);
        }
    };
    println!("Tokens: {:?}", tokens);
    let mut stmts = match parse(tokens) {
        Ok(e) => e,
        Err(errors) => {
            // TODO: think about the parser errors...
            for error in errors {}
            return ExitCode::from(65);
        }
    };
    let mut interpreter = Interpreter::new();
    match interpreter.interprete(&mut stmts) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Runtime Error: {}", err);
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
