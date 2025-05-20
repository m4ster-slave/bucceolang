use crate::function::Function;
use crate::native_functions::{ClockFn, RandomFn, ReadFn};
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;

use std::cell::RefCell;
use std::rc::Rc;

/// Represents any callable entity in the language runtime.
///
/// This can be a user-defined function or a built-in/native function.
/// Each variant encapsulates a specific kind of callable behavior.
#[derive(Debug, Clone)]
pub enum CallableObject {
    /// A user-defined function
    Function(Rc<RefCell<Function>>),
    /// A natve function that returns the current time
    ClockFn(ClockFn),
    /// A native function that reads input
    ReadFn(ReadFn),
    /// A native function that returns a random value
    RandomFn(RandomFn),
}

impl CallableObject {
    /// Returns the number of arguments (arity) expected by the callable object.
    ///
    /// # Returns
    /// * `usize` - The number of parameters the callable expects.
    pub fn arity(&self) -> usize {
        match self {
            CallableObject::Function(f) => f.borrow().arity(),
            CallableObject::ClockFn(nf) => nf.arity(),
            CallableObject::ReadFn(nf) => nf.arity(),
            CallableObject::RandomFn(nf) => nf.arity(),
        }
    }

    /// Invokes the callable object with the provided arguments within the context of the interpreter.
    ///
    /// # Arguments
    /// * `interp` - A mutable reference to the interpreter, which maintains runtime state.
    /// * `args` - A vector of `Object`s representing the arguments passed to the function.
    ///
    /// # Returns
    /// * `Result<Object, RuntimeError>` - The result of the function call or a runtime error.
    pub fn call(
        &self,
        interp: &mut Interpreter,
        args: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        match self {
            CallableObject::Function(f) => {
                let mut func_clone = f.borrow().clone();
                func_clone.call(interp, args)
            }
            CallableObject::ClockFn(nf) => nf.call(interp, args),
            CallableObject::ReadFn(nf) => nf.call(interp, args),
            CallableObject::RandomFn(nf) => nf.call(interp, args),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_clock_fn_arity_and_call() {
        let mut interp = Interpreter::new();
        let callable = CallableObject::ClockFn(ClockFn);

        assert_eq!(callable.arity(), 0);

        let result = callable.call(&mut interp, vec![]);
        assert!(result.is_ok());
        if let Ok(Object::Number(_)) = result {
        } else {
            panic!("ClockFn did not return a number");
        }
    }

    #[test]
    fn test_random_fn_arity_and_call() {
        let mut interp = Interpreter::new();
        let callable = CallableObject::RandomFn(RandomFn);

        assert_eq!(callable.arity(), 1);

        let result = callable.call(&mut interp, vec![Object::Number(10.0)]);
        assert!(result.is_ok());
        if let Ok(Object::Number(n)) = result {
            assert!(
                (0.0..=10.0).contains(&n),
                "RandomFn returned value out of range: {}",
                n
            );
        } else {
            panic!("RandomFn did not return a number");
        }
    }
}
