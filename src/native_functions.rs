use crate::{interpreter::Interpreter, object::Object, runtime_error::RuntimeError};
use std::time::{SystemTime, UNIX_EPOCH};

use std::cell::RefCell;
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

thread_local! {
    static RNG_STATE: RefCell<u64> = RefCell::new(seed_from_time());
}

fn seed_from_time() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    (nanos & 0x7fffffff) as u64
}

#[derive(Clone, Debug)]
pub struct RandomFn;
impl RandomFn {
    fn lcg_next() -> u64 {
        RNG_STATE.with(|state| {
            let mut s = state.borrow_mut();
            // constants for LCG: same as glibc
            *s = s.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fffffff;
            *s
        })
    }

    pub fn call(
        &self,
        _interpreter: &mut Interpreter,
        mut arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let range_obj = arguments.pop().ok_or_else(|| {
            RuntimeError::Other(
                0,
                format!("not enough arguments in function {}", self.to_string()),
            )
        })?;

        if let Object::Number(max) = range_obj {
            if max <= 0.0 {
                return Err(RuntimeError::Other(0, "range must be positive".into()));
            }
            let rand_value = Self::lcg_next();
            let result = (rand_value as f64 % max).floor();
            Ok(Object::Number(result))
        } else {
            Err(RuntimeError::Other(0, "argument must be a number".into()))
        }
    }

    pub fn arity(&self) -> usize {
        1
    }

    pub fn to_string(&self) -> String {
        String::from("<native fn random>")
    }
}
