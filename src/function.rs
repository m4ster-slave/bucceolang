use std::rc::Rc;

use crate::{
    callable::Callable,
    environment::Environment,
    object::Object,
    runtime_error::RuntimeError,
    stmt_types::{FunctionStmt, StmtVisitor},
};

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: FunctionStmt,
}

impl Callable for Function {
    fn call(
        &mut self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut environment = Environment::new_enclosed(interpreter.globals.clone());

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

        interpreter.visit_block_stmt(&mut self.declaration.body)?;

        // restore previous environment
        interpreter.environment = previous;

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme())
    }
}
