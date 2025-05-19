use crate::{interpreter::Interpreter, object::Object, runtime_error::RuntimeError};
use std::io;
use std::io::prelude::*;

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

#[derive(Clone, Debug)]
pub struct ReadFn;
impl ReadFn {
    pub fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut s = String::new();
        match io::stdin().lock().read_line(&mut s) {
            Ok(_) => Ok(Object::String(s.trim().into())),
            Err(e) => Err(RuntimeError::Other(
                0,
                format!("error when reading from stdin: {e}"),
            )),
        }
    }

    pub fn arity(&self) -> usize {
        0
    }

    pub fn to_string(&self) -> String {
        String::from("<native fn read>")
    }
}
