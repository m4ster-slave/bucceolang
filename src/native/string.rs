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
pub struct LenFn;
#[derive(Debug, Clone)]
pub struct SplitFn;
#[derive(Debug, Clone)]
pub struct JoinFn;
#[derive(Debug, Clone)]
pub struct ReplaceFn;
#[derive(Debug, Clone)]
pub struct LowerFn;
#[derive(Debug, Clone)]
pub struct UpperFn;
#[derive(Debug, Clone)]
pub struct StripFn;
#[derive(Debug, Clone)]
pub struct StartsWithFn;
#[derive(Debug, Clone)]
pub struct EndsWithFn;
#[derive(Debug, Clone)]
pub struct FindFn;
#[derive(Debug, Clone)]
pub struct ContainsFn;

impl Callable for LenFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "len(s): argument must be a string")),
        };
        Ok(Object::Number(s.chars().count() as f64))
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for LenFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn len>")
    }
}

impl Callable for SplitFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "split(s, sep): first argument must be a string")),
        };
        let sep = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "split(s, sep): second argument must be a string")),
        };
        Ok(Object::String(s.split(sep).collect::<Vec<_>>().join(",")))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for SplitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn split>")
    }
}

impl Callable for JoinFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let sep = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "join(sep, items): first argument must be a string")),
        };
        let items = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "join(sep, items): second argument must be a comma-separated string")),
        };
        let joined = items.split(',').collect::<Vec<_>>().join(sep);
        Ok(Object::String(joined))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for JoinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn join>")
    }
}

impl Callable for ReplaceFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 3 {
            return Err(RuntimeError::argument_error(0, format!("Expected 3 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "replace(s, from, to): first argument must be a string")),
        };
        let from = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "replace(s, from, to): second argument must be a string")),
        };
        let to = match &_arguments[2] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "replace(s, from, to): third argument must be a string")),
        };
        Ok(Object::String(s.replace(from, to)))
    }
    fn arity(&self) -> usize {
        3
    }
}
impl Display for ReplaceFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn replace>")
    }
}

impl Callable for LowerFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "lower(s): argument must be a string")),
        };
        Ok(Object::String(s.to_lowercase()))
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for LowerFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn lower>")
    }
}

impl Callable for UpperFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "upper(s): argument must be a string")),
        };
        Ok(Object::String(s.to_uppercase()))
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for UpperFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn upper>")
    }
}

impl Callable for StripFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 1 {
            return Err(RuntimeError::argument_error(0, format!("Expected 1 argument but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "strip(s): argument must be a string")),
        };
        Ok(Object::String(s.trim().to_string()))
    }
    fn arity(&self) -> usize {
        1
    }
}
impl Display for StripFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn strip>")
    }
}

impl Callable for StartsWithFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "startswith(s, prefix): first argument must be a string")),
        };
        let prefix = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "startswith(s, prefix): second argument must be a string")),
        };
        Ok(Object::Boolean(s.starts_with(prefix)))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for StartsWithFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn startswith>")
    }
}

impl Callable for EndsWithFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "endswith(s, suffix): first argument must be a string")),
        };
        let suffix = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "endswith(s, suffix): second argument must be a string")),
        };
        Ok(Object::Boolean(s.ends_with(suffix)))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for EndsWithFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn endswith>")
    }
}

impl Callable for FindFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "find(s, sub): first argument must be a string")),
        };
        let sub = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "find(s, sub): second argument must be a string")),
        };
        match s.find(sub) {
            Some(idx) => Ok(Object::Number(idx as f64)),
            None => Ok(Object::Nil),
        }
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for FindFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn find>")
    }
}

impl Callable for ContainsFn {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        if _arguments.len() != 2 {
            return Err(RuntimeError::argument_error(0, format!("Expected 2 arguments but got {}", _arguments.len())));
        }
        let s = match &_arguments[0] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "contains(s, sub): first argument must be a string")),
        };
        let sub = match &_arguments[1] {
            Object::String(s) => s,
            _ => return Err(RuntimeError::argument_error(0, "contains(s, sub): second argument must be a string")),
        };
        Ok(Object::Boolean(s.contains(sub)))
    }
    fn arity(&self) -> usize {
        2
    }
}
impl Display for ContainsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn contains>")
    }
}

pub fn create_class() -> ClassObject {
    let methods = HashMap::new();
    let mut static_methods = HashMap::new();
    static_methods.insert(
        "len".to_string(),
        Rc::new(RefCell::new(Box::new(LenFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "split".to_string(),
        Rc::new(RefCell::new(Box::new(SplitFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "join".to_string(),
        Rc::new(RefCell::new(Box::new(JoinFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "replace".to_string(),
        Rc::new(RefCell::new(Box::new(ReplaceFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "lower".to_string(),
        Rc::new(RefCell::new(Box::new(LowerFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "upper".to_string(),
        Rc::new(RefCell::new(Box::new(UpperFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "strip".to_string(),
        Rc::new(RefCell::new(Box::new(StripFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "startswith".to_string(),
        Rc::new(RefCell::new(Box::new(StartsWithFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "endswith".to_string(),
        Rc::new(RefCell::new(Box::new(EndsWithFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "find".to_string(),
        Rc::new(RefCell::new(Box::new(FindFn) as Box<dyn Callable>)),
    );
    static_methods.insert(
        "contains".to_string(),
        Rc::new(RefCell::new(Box::new(ContainsFn) as Box<dyn Callable>)),
    );
    ClassObject {
        name: "String".to_string(),
        superclass: None,
        methods,
        static_methods,
    }
}

