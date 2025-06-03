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
pub struct ExitFn;
#[derive(Debug, Clone)]
pub struct EnvFn;
#[derive(Debug, Clone)]
pub struct ArgsFn;
#[derive(Debug, Clone)]
pub struct ExecFn;
#[derive(Debug, Clone)]
pub struct PlatformFn;

impl Callable for ExitFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("ExitFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for ExitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exit>")
    }
}

impl Callable for EnvFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("EnvFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for EnvFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn env>")
    }
}

impl Callable for ArgsFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("ArgsFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 0 }
}
impl Display for ArgsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn args>")
    }
}

impl Callable for ExecFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("ExecFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 1 }
}
impl Display for ExecFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exec>")
    }
}

impl Callable for PlatformFn {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        unimplemented!("PlatformFn native logic not implemented yet")
    }
    fn arity(&self) -> usize { 0 }
}
impl Display for PlatformFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn platform>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert("exit".to_string(), Rc::new(RefCell::new(Box::new(ExitFn) as Box<dyn Callable>)));
    static_methods.insert("env".to_string(), Rc::new(RefCell::new(Box::new(EnvFn) as Box<dyn Callable>)));
    static_methods.insert("args".to_string(), Rc::new(RefCell::new(Box::new(ArgsFn) as Box<dyn Callable>)));
    static_methods.insert("exec".to_string(), Rc::new(RefCell::new(Box::new(ExecFn) as Box<dyn Callable>)));
    static_methods.insert("platform".to_string(), Rc::new(RefCell::new(Box::new(PlatformFn) as Box<dyn Callable>)));
    ClassObject {
        name: "System".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}