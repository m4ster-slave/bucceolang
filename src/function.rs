use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    class::ClassInstance,
    environment::Environment,
    object::Object,
    runtime_error::RuntimeError,
    stmt_types::{FunctionStmt, StmtVisitor},
};

/// Represents a user-defined function in the language.
#[derive(Debug, Clone)]
pub struct Function {
    /// The statement that declares this function.
    pub declaration: FunctionStmt,
    /// The environment in which the function was defined (the closure).
    pub closure: Rc<RefCell<Environment>>,
}

impl Function {
    /// Creates a new `Function` instance.
    ///
    /// # Arguments
    ///
    /// * `declaration` - The `FunctionStmt` that declares this function.
    /// * `closure` - The environment where the function was defined.
    pub fn new(declaration: FunctionStmt, closure: Rc<RefCell<Environment>>) -> Function {
        Function {
            declaration,
            closure,
        }
    }

    pub fn bind(&self, instance: ClassInstance) -> Result<Function, RuntimeError> {
        let mut environment = Environment::new_enclosed(self.closure.clone());
        environment.define("this".to_string(), Object::ClassInstance(instance))?;
        Ok(Function {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
        })
    }
}

impl Function {
    /// Calls the function with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `interpreter` - The interpreter executing the code.
    /// * `arguments` - A vector of `Object` representing the arguments passed to the function.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the return value of the function as an `Object`, or a `RuntimeError` if an error occurs during execution
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

    /// Returns the number of parameters the function expects
    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    /// Returns a string representation of the function
    pub fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme())
    }
}
