use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;

pub trait Callable: std::fmt::Debug {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
