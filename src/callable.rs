use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;
use std::fmt::Display;

pub trait Callable: Display {
    fn arity(&self) -> usize;
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use crate::native_functions::{ClockFn, RandomFn};
    use crate::Interpreter;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_clock_fn_arity_and_call() {
        let mut interp = Interpreter::new();
        let callable: Rc<RefCell<dyn Callable>> = Rc::new(RefCell::new(ClockFn));

        assert_eq!(callable.borrow().arity(), 0);

        let result = callable.borrow_mut().call(&mut interp, vec![]);
        assert!(result.is_ok());
        if let Ok(Object::Number(_)) = result {
        } else {
            panic!("ClockFn did not return a number");
        }
    }

    #[test]
    fn test_random_fn_arity_and_call() {
        let mut interp = Interpreter::new();
        let callable: Rc<RefCell<dyn Callable>> = Rc::new(RefCell::new(RandomFn));

        assert_eq!(callable.borrow().arity(), 1);

        let result = callable.borrow_mut().call(&mut interp, vec![Object::Number(10.0)]);
        assert!(result.is_ok());
        if let Ok(Object::Number(n)) = result {
            assert!(
                (0.0..=10.0).contains(&n),
                "RandomFn returned value out of range: {}",
                n
            );
        } else {
            panic!("RandomFn did not return a number");
        }
    }
}
