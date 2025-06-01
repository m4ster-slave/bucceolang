use crate::callable::Callable;
use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

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
    pub is_initializer: bool,
}

impl Function {
    /// Creates a new `Function` instance.
    ///
    /// # Arguments
    ///
    /// * `declaration` - The `FunctionStmt` that declares this function.
    /// * `closure` - The environment where the function was defined.
    pub fn new(
        declaration: FunctionStmt,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Function {
        Function {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: ClassInstance) -> Result<Function, RuntimeError> {
        let mut environment = Environment::new_enclosed(self.closure.clone());
        environment.define("this".to_string(), Object::ClassInstance(instance))?;
        Ok(Function {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
        })
    }
}

impl Callable for Function {
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
    fn call(
        &self,
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

        let mut body = self.declaration.body.clone();
        let return_val = match interpreter.visit_block_stmt(&mut body) {
            Ok(()) => Object::Nil,
            Err(RuntimeError::Return(value)) => value.unwrap_or(Object::Nil),
            Err(e) => return Err(e),
        };

        // restore previous environment
        interpreter.environment = previous;

        if self.is_initializer {
            return self.closure.borrow().get_at(&0, "this".to_string());
        }

        Ok(return_val)
    }

    /// Returns the number of parameters the function expects
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme())
    }
}
