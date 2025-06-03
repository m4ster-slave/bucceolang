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
struct InputFn;
#[derive(Debug, Clone)]
struct ReadFileFn;
#[derive(Debug, Clone)]
struct WriteFileFn;
#[derive(Debug, Clone)]
struct AppendFileFn;
#[derive(Debug, Clone)]
struct ExistsFn;
#[derive(Debug, Clone)]
struct IsFileFn;
#[derive(Debug, Clone)]
struct IsDirFn;
#[derive(Debug, Clone)]
struct ListDirFn;
#[derive(Debug, Clone)]
struct RemoveFileFn;
#[derive(Debug, Clone)]
struct MkdirFn;

impl Callable for ReadFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("ReadFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ReadFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn read_file>")
    }
}

impl Callable for WriteFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("WriteFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for WriteFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn write_file>")
    }
}

impl Callable for AppendFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("AppendFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for AppendFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn append_file>")
    }
}

impl Callable for ExistsFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("ExistsFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ExistsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn exists>")
    }
}

impl Callable for IsFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("IsFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for IsFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn is_file>")
    }
}

impl Callable for IsDirFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("IsDirFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for IsDirFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn is_dir>")
    }
}

impl Callable for ListDirFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("ListDirFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for ListDirFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn list_dir>")
    }
}

impl Callable for RemoveFileFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("RemoveFileFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for RemoveFileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn remove_file>")
    }
}

impl Callable for MkdirFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        unimplemented!("MkdirFn native logic not implemented yet")
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for MkdirFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn mkdir>")
    }
}

impl Callable for InputFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        use std::io::{self, Write};
        if arguments.len() != 1 {
            return Err(RuntimeError::argument_error(
                0,
                format!("Expected 1 argument but got {}", arguments.len()),
            ));
        }
        let prompt = match &arguments[0] {
            Object::String(s) => s,
            _ => {
                return Err(RuntimeError::argument_error(
                    0,
                    "input(prompt): prompt must be a string",
                ))
            }
        };
        print!("{}", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(Object::String(input.trim_end().to_string())),
            Err(e) => Err(RuntimeError::other(0, format!("IO error: {e}"))),
        }
    }
    fn arity(&self) -> usize {
        1
    }
}

impl Display for InputFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn input>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new(); // Instance methods
    let mut static_methods = HashMap::new(); // Static methods

    static_methods.insert(
        "input".to_string(),
        Rc::new(RefCell::new(Box::new(InputFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "read_file".to_string(),
        Rc::new(RefCell::new(Box::new(ReadFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "write_file".to_string(),
        Rc::new(RefCell::new(Box::new(WriteFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "append_file".to_string(),
        Rc::new(RefCell::new(Box::new(AppendFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "exists".to_string(),
        Rc::new(RefCell::new(Box::new(ExistsFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "is_file".to_string(),
        Rc::new(RefCell::new(Box::new(IsFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "is_dir".to_string(),
        Rc::new(RefCell::new(Box::new(IsDirFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "list_dir".to_string(),
        Rc::new(RefCell::new(Box::new(ListDirFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "remove_file".to_string(),
        Rc::new(RefCell::new(Box::new(RemoveFileFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "mkdir".to_string(),
        Rc::new(RefCell::new(Box::new(MkdirFn) as Box<dyn Callable>)),
    );

    ClassObject {
        name: "IO".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}
