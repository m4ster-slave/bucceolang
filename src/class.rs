use crate::callable::CallableObject;
use crate::function::Function;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;
use crate::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ClassObject {
    pub name: String,
    pub methods: HashMap<String, Function>,
}

impl ClassObject {
    pub fn new(name: &str, methods: HashMap<String, Function>) -> Self {
        Self {
            name: name.into(),
            methods,
        }
    }

    fn find_method(&self, name: &str) -> Option<Object> {
        self.methods.get(name).map(|method| {
            Object::Callable(CallableObject::Function(Rc::new(RefCell::new(
                method.clone(),
            ))))
        })
    }
}

// callable
impl ClassObject {
    pub fn arity(&self) -> usize {
        0
    }

    pub fn call(
        &self,
        interp: &mut Interpreter,
        args: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::ClassInstance(ClassInstance::new(self.clone())))
    }
}

impl Display for ClassObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct ClassInstance {
    class: ClassObject,
    fields: HashMap<String, Object>,
}

impl ClassInstance {
    pub fn new(class: ClassObject) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        if self.fields.contains_key(name.lexeme()) {
            Ok(self.fields.get(name.lexeme()).unwrap().clone())
        } else {
            // first try to find the method in the class before erroring
            match self.class.find_method(name.lexeme()) {
                Some(method) => Ok(method),
                None => Err(RuntimeError::UndefinedVariable(
                    name.line(),
                    format!("Undefined property '{}'.", name.lexeme()),
                )),
            }
        }
    }

    pub fn set(&mut self, name: Token, value: Object) {
        self.fields.insert(name.lexeme().into(), value);
    }
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
