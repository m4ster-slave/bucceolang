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
pub struct HttpGetFn;
#[derive(Debug, Clone)]
pub struct HttpPostFn;
#[derive(Debug, Clone)]
pub struct DownloadFileFn;
#[derive(Debug, Clone)]
pub struct PingFn;

impl Callable for HttpGetFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("HttpGetFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for HttpGetFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn http_get>")
    }
}

impl Callable for HttpPostFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("HttpPostFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for HttpPostFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn http_post>")
    }
}

impl Callable for DownloadFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("DownloadFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for DownloadFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn download_file>")
    }
}

impl Callable for PingFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("PingFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for PingFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn ping>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "http_get".to_string(),
        Rc::new(RefCell::new(Box::new(HttpGetFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "http_post".to_string(),
        Rc::new(RefCell::new(Box::new(HttpPostFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "download_file".to_string(),
        Rc::new(RefCell::new(Box::new(DownloadFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "ping".to_string(),
        Rc::new(RefCell::new(Box::new(PingFn) as Box<dyn Callable>)),
    );
    ClassObject {
        name: "Network".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}

