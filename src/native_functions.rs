use crate::{interpreter::Interpreter, object::Object, runtime_error::RuntimeError};

#[derive(Clone, Debug)]
pub struct ClockFn;

impl ClockFn {
    pub fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Object::Number(now))
    }

    pub fn arity(&self) -> usize {
        0
    }

    pub fn to_string(&self) -> String {
        String::from("<native fn clock>")
    }
}
