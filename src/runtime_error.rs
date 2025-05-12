#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError(String),
    DivisionByZero,
    UndefinedVariable(String),
    Other(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(msg) => write!(f, "Type Error: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::Other(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}
