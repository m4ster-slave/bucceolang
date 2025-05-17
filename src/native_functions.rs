use crate::{
    callable::Callable, interpreter::Interpreter, object::Object, runtime_error::RuntimeError,
};

#[derive(Debug)]
pub struct ClockFn;

impl Callable for ClockFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Object::Number(now))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        String::from("<native fn clock>")
    }
}
