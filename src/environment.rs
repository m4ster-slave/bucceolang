use std::collections::HashMap;

use crate::object::Object;
use crate::{runtime_error::RuntimeError, token::Token};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError> {
        if !self.values.contains_key(&name) {
            self.values.insert(name, value);

            Ok(())
        } else {
            Err(RuntimeError::Other(
                0,
                format!("name \"{name}\" already defined"),
            ))
        }
    }

    /// Since making it a static error makes recursive declarations too difficult, we’ll defer the error to runtime. It’s _OK_ to refer to a variable before it’s defined as long as you *don’t* evaluate the reference. That lets the program for even and odd numbers work, but you’d get a runtime error in:
    pub fn get(&self, name: &Token) -> Result<&Object, RuntimeError> {
        match self.values.get(name.lexeme()) {
            Some(val) => Ok(val),
            None => Err(RuntimeError::UndefinedVariable(
                name.line(),
                "trying to access undefined variable".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Environment;
    use crate::object::Object;
    use std::collections::HashMap;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment {
            values: HashMap::new(),
        };

        env.define("KEY".to_string(), Object::String("VALUE".to_string()));

        assert!(env.values.contains_key("KEY"));
        assert_eq!(
            env.values.get("KEY").unwrap(),
            &Object::String("VALUE".to_string())
        );
    }
}
