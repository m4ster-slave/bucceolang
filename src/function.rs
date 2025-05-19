use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    environment::Environment,
    object::Object,
    runtime_error::RuntimeError,
    stmt_types::{FunctionStmt, StmtVisitor},
};

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: FunctionStmt,
    pub closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: FunctionStmt, closure: Rc<RefCell<Environment>>) -> Function {
        Function {
            declaration,
            closure,
        }
    }
}

impl Function {
    pub fn call(
        &mut self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut environment = Environment::new_enclosed(self.closure.clone());

        for i in 0..self.declaration.params.len() {
            environment.define(
                self.declaration.params.get(i).unwrap().lexeme().to_string(),
                arguments.get(i).unwrap().clone(),
            )?;
        }

        // save the current environment
        let previous = interpreter.environment.clone();

        // replace interpreter's environment with new one
        interpreter.environment = Rc::new(environment.into());

        let return_val = match interpreter.visit_block_stmt(&mut self.declaration.body) {
            Ok(()) => Object::Nil,
            Err(RuntimeError::Return(value)) => value,
            Err(e) => return Err(e),
        };

        // restore previous environment
        interpreter.environment = previous;

        Ok(return_val)
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    pub fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme())
    }
}
