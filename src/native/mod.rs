mod io;
mod math;
mod string;
mod time;
mod system;
mod network;

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
        .define("Math".to_string(), Object::Class(math::create_class()))
        .expect("Failed to define Math class");
    globals
        .borrow_mut()
        .define("String".to_string(), Object::Class(string::create_class()))
        .expect("Failed to define String class");
    globals
        .borrow_mut()
        .define("Time".to_string(), Object::Class(time::create_class()))
        .expect("Failed to define Time class");
    globals
        .borrow_mut()
        .define("System".to_string(), Object::Class(system::create_class()))
        .expect("Failed to define System class");
    globals
        .borrow_mut()
        .define("Network".to_string(), Object::Class(network::create_class()))
        .expect("Failed to define Network class");

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

