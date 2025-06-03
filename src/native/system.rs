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
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let code = match &_arguments[0] {
            Object::Number(n) => *n as i32,
            _ => return Err(RuntimeError::argument_error(0, "exit(code): argument must be a number")),
        };
        std::process::exit(code);
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ExitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exit>")
    }
}

impl Callable for EnvFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let key = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "env(key): argument must be a string")),
        };
        match std::env::var(key) {
            Ok(val) => Ok(Object::String(val)),
            Err(_) => Ok(Object::Nil),
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for EnvFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn env>")
    }
}

impl Callable for ArgsFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let args: Vec<String> = std::env::args().collect();
        Ok(Object::String(args.join(",")))
    }
    fn arity(&self) -> usize {
        0
    }
}
impl Display for ArgsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn args>")
    }
}

impl Callable for ExecFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let cmd = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "exec(cmd): argument must be a string")),
        };
        match std::process::Command::new("sh").arg("-c").arg(cmd).output() {
            Ok(output) => {
                let out = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(Object::String(out))
            },
            Err(e) => Err(RuntimeError::other(0, format!("Exec error: {e}"))),
        }
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ExecFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exec>")
    }
}

impl Callable for PlatformFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::String(std::env::consts::OS.to_string()))
    }
    fn arity(&self) -> usize {
        0
    }
}
impl Display for PlatformFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn platform>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "exit".to_string(),
        Rc::new(RefCell::new(Box::new(ExitFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "env".to_string(),
        Rc::new(RefCell::new(Box::new(EnvFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "args".to_string(),
        Rc::new(RefCell::new(Box::new(ArgsFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "exec".to_string(),
        Rc::new(RefCell::new(Box::new(ExecFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "platform".to_string(),
        Rc::new(RefCell::new(Box::new(PlatformFn) as Box<dyn Callable>)),
    );
    ClassObject {
        name: "System".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}

