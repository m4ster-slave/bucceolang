use crate::{
    callable::CallableObject,
    class::{ClassInstance, ClassObject},
};
/// Represents the different types of values that can be produced and manipulated by the interpreter.
///
/// These are the runtime values of the language, such as the absence of a value (`Nil`),
/// boolean true/false, floating-point numbers, and strings.
#[derive(Debug, Clone)]
pub enum Object {
    /// Represents the absence of a value, similar to `null` in other languages.
    Nil,
    /// Represents a boolean value, either `true` or `false`.
    Boolean(bool),
    /// Represents a floating-point number.
    Number(f64),
    /// Represents a text string.
    String(String),
    /// Closure or function
    Callable(CallableObject), // hav to use a Rc because the trait size is not known so we cant
    // make a deep copy of it
    Class(ClassObject),
    ClassInstance(ClassInstance),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "Nil"),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Number(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "{}", string),
            Object::Callable(callable) => write!(f, "{}", callable),
            Object::Class(class) => write!(f, "{}", class),
            Object::ClassInstance(instance) => write!(f, "{}", instance),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            // callable are just never eq
            (Object::Callable(_), Object::Callable(_)) => false,
            (Object::Class(_), Object::Class(_)) => false,
            _ => false,
        }
    }
}
