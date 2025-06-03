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
        unimplemented!("LenFn native logic not implemented yet")
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
        unimplemented!("SplitFn native logic not implemented yet")
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
        unimplemented!("JoinFn native logic not implemented yet")
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
        unimplemented!("ReplaceFn native logic not implemented yet")
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
        unimplemented!("LowerFn native logic not implemented yet")
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
        unimplemented!("UpperFn native logic not implemented yet")
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
        unimplemented!("StripFn native logic not implemented yet")
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
        unimplemented!("StartsWithFn native logic not implemented yet")
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
        unimplemented!("EndsWithFn native logic not implemented yet")
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
        unimplemented!("FindFn native logic not implemented yet")
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
        unimplemented!("ContainsFn native logic not implemented yet")
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

