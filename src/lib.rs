mod callable;
mod class;
mod environment;
mod expr_types;
mod function;
mod interpreter;
mod native;
mod object;
mod parser;
mod parser_error;
mod resolver;
mod runtime_error;
mod scanner;
mod scanner_error;
mod stmt_types;
mod token;

use interpreter::Interpreter;
use parser::parse;
use resolver::Resolver;
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
        Err(e) => return format!("{}", e),
    };

    let mut stmts = match parse(tokens) {
        Ok(s) => s,
        Err(errors) => {
            return errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        }
    };

    let output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let output_for_interp = output.clone();

    let mut interpreter = Interpreter::new_with_output_without_natives(output_for_interp);

    let mut resolver = Resolver::new(&mut interpreter);
    match resolver.resolve(&mut stmts) {
        Ok(_) => (),
        Err(e) => return format!("{}", e),
    };

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
