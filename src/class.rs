use crate::callable::Callable;

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
    pub superclass: Option<Box<Object>>,
    pub methods: HashMap<String, Function>,
}

impl ClassObject {
    pub fn new(
        name: &str,
        superclass: Option<Box<Object>>,
        methods: HashMap<String, Function>,
    ) -> Self {
        Self {
            name: name.into(),
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = &self.superclass {
            match **superclass {
                Object::Class(ref class) => class.find_method(name),
                _ => None,
            }
        } else {
            None
        }
    }
}

// callable
impl Callable for ClassObject {
    fn arity(&self) -> usize {
        if let Some(init) = self.find_method("init") {
            init.arity()
        } else {
            0
        }
    }

    fn call(&self, interp: &mut Interpreter, args: Vec<Object>) -> Result<Object, RuntimeError> {
        let instance = ClassInstance::new(self.clone());

        if let Some(init) = self.find_method("init") {
            init.bind(instance.clone())?.call(interp, args)?;
        }

        Ok(Object::ClassInstance(instance))
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
    fields: Rc<RefCell<HashMap<String, Object>>>,
}

impl ClassInstance {
    pub fn new(class: ClassObject) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        let fields = self.fields.borrow();
        if fields.contains_key(name.lexeme()) {
            Ok(fields.get(name.lexeme()).unwrap().clone())
        } else {
            // first try to find the method in the class before erroring
            match self.class.find_method(name.lexeme()) {
                Some(method) => match method.clone().bind(self.clone()) {
                    Ok(bound_method) => Ok(Object::Callable(Rc::new(RefCell::new(bound_method)))),
                    Err(e) => Err(e),
                },
                None => Err(RuntimeError::undefined_variable(
                    name.line(),
                    format!("Undefined property '{}'.", name.lexeme()),
                )),
            }
        }
    }

    pub fn set(&mut self, name: Token, value: Object) {
        let mut fields = self.fields.borrow_mut();
        fields.insert(name.lexeme().into(), value);
    }
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
