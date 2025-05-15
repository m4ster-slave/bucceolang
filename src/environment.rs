use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;
use crate::{runtime_error::RuntimeError, token::Token};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError> {
        match self.values.entry(name.clone()) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(value);
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(_) => Err(RuntimeError::Other(
                0,
                format!("name \"{}\" already defined", name.clone()),
            )),
        }
    }
    /// Since making it a static error makes recursive declarations too difficult, we’ll defer the error to runtime. It’s _OK_ to refer to a variable before it’s defined as long as you *don’t* evaluate the reference. That lets the program for even and odd numbers work, but you’d get a runtime error in:
    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        match self.values.get(name.lexeme()) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(parent) => parent.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable(
                    name.line(),
                    format!("undefined variable '{}'", name.lexeme()),
                )),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_owned(), value);

            Ok(())
        } else if let Some(ref parent) = self.enclosing {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::Other(
                0,
                format!("undefined variable \"{name}\""),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::Environment;
    use crate::object::Object;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::new();

        let _ = env.define("KEY".to_string(), Object::String("VALUE".to_string()));

        assert!(env.values.contains_key("KEY"));
        assert_eq!(
            env.values.get("KEY").unwrap(),
            &Object::String("VALUE".to_string())
        );
    }
}
