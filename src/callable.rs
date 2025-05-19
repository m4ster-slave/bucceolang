use crate::function::Function;
use crate::native_functions::{ClockFn, ReadFn};
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Interpreter;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CallableObject {
    Function(Rc<RefCell<Function>>),
    ClockFn(ClockFn),
    ReadFn(ReadFn),
}

impl CallableObject {
    pub fn arity(&self) -> usize {
        match self {
            CallableObject::Function(f) => f.borrow().arity(),
            CallableObject::ClockFn(nf) => nf.arity(),
            CallableObject::ReadFn(nf) => nf.arity(),
        }
    }

    pub fn call(
        &self,
        interp: &mut Interpreter,
        args: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        match self {
            CallableObject::Function(f) => {
                let mut func_clone = f.borrow().clone();
                func_clone.call(interp, args)
            }
            CallableObject::ClockFn(nf) => nf.call(interp, args),
            CallableObject::ReadFn(nf) => nf.call(interp, args),
        }
    }
}
