use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::class::ClassObject;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TimeFn;
#[derive(Debug, Clone)]
pub struct SleepFn;
#[derive(Debug, Clone)]
pub struct NowFn;
#[derive(Debug, Clone)]
pub struct StrftimeFn;

impl Callable for TimeFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("TimeFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 0 }
}
impl Display for TimeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn time>")
    }
}

impl Callable for SleepFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("SleepFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for SleepFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn sleep>")
    }
}

impl Callable for NowFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("NowFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 0 }
}
impl Display for NowFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn now>")
    }
}

impl Callable for StrftimeFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("StrftimeFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 2 }
}
impl Display for StrftimeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn strftime>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert("time".to_string(), Rc::new(RefCell::new(Box::new(TimeFn) as Box<dyn Callable>)));
    static_methods.insert("sleep".to_string(), Rc::new(RefCell::new(Box::new(SleepFn) as Box<dyn Callable>)));
    static_methods.insert("now".to_string(), Rc::new(RefCell::new(Box::new(NowFn) as Box<dyn Callable>)));
    static_methods.insert("strftime".to_string(), Rc::new(RefCell::new(Box::new(StrftimeFn) as Box<dyn Callable>)));
    ClassObject {
        name: "Time".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}