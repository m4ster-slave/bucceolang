#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "Nil"),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Number(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "{}", string),
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
            _ => false,
        }
    }
}
