use crate::callable::Callable;
use crate::class::ClassObject;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TimeFn;
#[derive(Debug, Clone)]
pub struct SleepFn;

impl Callable for TimeFn {
    fn call(
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
    fn arity(&self) -> usize {
        0
    }
}
impl Display for TimeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn time>")
    }
}

impl Callable for SleepFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(
                0,
                format!("Expected 1 argument but got {}", _arguments.len()),
            ));
        }
        let secs = match &_arguments[0] {
            Object::Number(n) => *n,
            _ => {
                return Err(RuntimeError::argument_error(
                    0,
                    "sleep(secs): argument must be a number",
                ))
            }
        };
        if secs < 0.0 {
            return Err(RuntimeError::argument_error(
                0,
                "sleep(secs): argument must be non-negative",
            ));
        }
        std::thread::sleep(std::time::Duration::from_secs_f64(secs));
        Ok(Object::Nil)
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for SleepFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sleep>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "time".to_string(),
        Rc::new(RefCell::new(Box::new(TimeFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "sleep".to_string(),
        Rc::new(RefCell::new(Box::new(SleepFn) as Box<dyn Callable>)),
    );

    ClassObject {
        name: "Time".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}