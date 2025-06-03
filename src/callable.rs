use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;
use std::fmt::Display;

pub trait Callable: Display {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;
}
