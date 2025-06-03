#[cfg(not(target_arch = "wasm32"))]
mod io;
#[cfg(not(target_arch = "wasm32"))]
mod math;
#[cfg(not(target_arch = "wasm32"))]
mod native_tests;
#[cfg(not(target_arch = "wasm32"))]
mod network;
#[cfg(not(target_arch = "wasm32"))]
mod string;
#[cfg(not(target_arch = "wasm32"))]
mod system;
#[cfg(not(target_arch = "wasm32"))]
mod time;

use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::object::Object;

#[cfg(not(target_arch = "wasm32"))]
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
        .define(
            "Network".to_string(),
            Object::Class(network::create_class()),
        )
        .expect("Failed to define Network class");
}

#[cfg(target_arch = "wasm32")]
pub fn add_native_functions(_globals: &Rc<RefCell<Environment>>) {
    // No native functions available in WASM
}
