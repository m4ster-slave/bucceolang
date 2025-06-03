pub mod io;
pub mod math;
pub mod network;
pub mod string;
pub mod system;
pub mod time;

use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::native_functions::*;
use crate::object::Object;

pub fn add_native_functions(globals: &Rc<RefCell<Environment>>) {
    globals
        .borrow_mut()
        .define("IO".to_string(), Object::Class(io::create_class()))
        .expect("Failed to define IO class");

    globals
        .borrow_mut()
        .define(
            "clock".into(),
            Object::Callable(Rc::new(RefCell::new(Box::new(ClockFn)))),
        )
        .expect("Failed to define native function 'clock'");

    globals
        .borrow_mut()
        .define(
            "random".into(),
            Object::Callable(Rc::new(RefCell::new(Box::new(RandomFn)))),
        )
        .expect("Failed to define native function 'random'");

    globals
        .borrow_mut()
        .define(
            "sin".into(),
            Object::Callable(Rc::new(RefCell::new(Box::new(SinFn)))),
        )
        .expect("Failed to define native function 'sin'");

    globals
        .borrow_mut()
        .define(
            "sqrt".into(),
            Object::Callable(Rc::new(RefCell::new(Box::new(SqrtFn)))),
        )
        .expect("Failed to define native function 'sqrt'");
}

