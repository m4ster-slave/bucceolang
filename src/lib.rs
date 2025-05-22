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

use std::cell::RefCell;
use std::rc::Rc;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: &str) -> String {
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return String::from("error");
        }
    };

    let mut stmts = match parse(tokens) {
        Ok(s) => s,
        Err(_errors) => return String::from("Parsing error"),
    };

    let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let output_for_interp = output.clone();

    let mut interpreter = Interpreter::new_with_output(output_for_interp);
    match interpreter.interprete(&mut stmts) {
        Ok(_) => (),
        Err(e) => return format!("{}", e),
    };

    let bytes: std::cell::Ref<'_, Vec<u8>> = output.borrow();
    match str::from_utf8(&bytes) {
        Ok(v) => v.to_owned(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}
