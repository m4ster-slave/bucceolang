use crate::object::Object;
use crate::{runtime_error::RuntimeError, token::Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Represents a variable environment (scope) for the interpreter.
///
/// Stores variable bindings and supports lexical scoping via an optional enclosing environment.
#[derive(Debug, Clone)]
pub struct Environment {
    /// Optional reference to the enclosing (parent) environment
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    /// Mapping of variable names to their values in the current scope
    pub values: HashMap<String, Object>,
}

impl Environment {
    /// Creates a new environment with the given parent environment.
    ///
    /// # Arguments
    ///
    /// * `enclosing` - Reference-counted reference cell containing the parent environment
    ///
    /// # Returns
    ///
    /// A new environment with the specified parent environment
    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// Creates a new global environment with no parent.
    ///
    /// # Returns
    ///
    /// A new environment with no parent environment
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    /// Defines a new variable in the current environment.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to define
    /// * `value` - The value to associate with the variable
    ///
    /// # Returns
    ///
    /// `Ok(())` if the variable was defined successfully,
    /// or an error if a variable with that name already exists
    pub fn define(&mut self, name: String, value: Object) -> Result<(), RuntimeError> {
        match self.values.entry(name.clone()) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(value);
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(_) => Err(RuntimeError::other(
                0,
                format!("name \"{}\" already defined", name),
            )),
        }
    }

    /// Retrieves the value of a variable from the environment or its ancestors.
    ///
    /// If the variable is not found in the current environment,
    /// the search continues in the parent environments.
    ///
    /// Since making it a static error makes recursive declarations too difficult,
    /// we'll defer the error to runtime. It's *OK* to refer to a variable before
    /// it's defined as long as you *don't* evaluate the reference. This enables
    /// mutually recursive functions like even/odd to work.
    ///
    /// # Arguments
    ///
    /// * `name` - The token containing the name of the variable to look up
    ///
    /// # Returns
    ///
    /// `Ok(Object)` containing the value of the variable if found,
    /// or a RuntimeError if the variable is not defined in any accessible scope
    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        match self.values.get(name.lexeme()) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(parent) => parent.borrow().get(name),
                None => Err(RuntimeError::undefined_variable(
                    name.line(),
                    format!("undefined variable '{}'", name.lexeme()),
                )),
            },
        }
    }
    pub fn get_at(&self, distance: &usize, name: String) -> Result<Object, RuntimeError> {
        if *distance == 0 {
            match self.values.get(&name) {
                None => Err(RuntimeError::undefined_variable(
                    0,
                    format!(
                        "Variable 1 \"{}\" cannot be resolved at the specified location",
                        name
                    ),
                )),
                Some(obj) => Ok(obj.clone()),
            }
        } else {
            match self.ancestors(distance) {
                Some(env) => match env.borrow().values.get(&name) {
                    None => Err(RuntimeError::undefined_variable(
                        0,
                        format!(
                            "Variable 2 \"{}\" cannot be resolved at the specified location",
                            name
                        ),
                    )),
                    Some(obj) => Ok(obj.clone()),
                },
                None => Err(RuntimeError::undefined_variable(
                    0,
                    format!(
                        "Variable 3 \"{}\" cannot be resolved at the specified location",
                        name
                    ),
                )),
            }
        }
    }

    pub fn assign_at(
        &mut self,
        distance: &usize,
        name: Token,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        if *distance == 0 {
            if !self.values.contains_key(name.lexeme()) {
                return Err(RuntimeError::undefined_variable(
                    name.line(),
                    format!("Undefined variable '{}'", name.lexeme()),
                ));
            }
            let old_value = self.values.insert(name.lexeme().to_owned(), value);
            Ok(old_value.unwrap_or(Object::Nil))
        } else {
            // Handle ancestor environments
            match self.ancestors(distance) {
                Some(env) => match env
                    .borrow_mut()
                    .values
                    .insert(name.lexeme().to_owned(), value)
                {
                    Some(old_value) => Ok(old_value),
                    None => Err(RuntimeError::undefined_variable(
                        name.line(),
                        format!(
                            "Variable \"{}\" cannot be resolved at the specified location",
                            name.lexeme()
                        ),
                    )),
                },
                None => Err(RuntimeError::undefined_variable(
                    name.line(),
                    format!(
                        "Variable \"{}\" cannot be resolved at the specified location",
                        name.lexeme()
                    ),
                )),
            }
        }
    }

    fn ancestors(&self, distance: &usize) -> Option<Rc<RefCell<Environment>>> {
        if *distance == 0 {
            return None;
        }

        let mut current = self.enclosing.clone();

        for _ in 1..*distance {
            current = match current {
                Some(ref env) => env.borrow().enclosing.clone(),
                None => return None,
            }
        }
        current
    }

    /// Assigns a new value to an existing variable in the environment chain.
    ///
    /// The assignment succeeds only if the variable already exists
    /// in the current environment or any parent environment.
    ///
    /// # Arguments
    ///
    /// * `name` - The token containing the name of the variable to assign to
    /// * `value` - The new value to assign to the variable
    ///
    /// # Returns
    ///
    /// `Ok(())` if the assignment was successful,
    /// or a RuntimeError if the variable is not defined in any accessible scope
    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_owned(), value.clone());
            Ok(())
        } else if let Some(ref parent) = self.enclosing {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::other(
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
