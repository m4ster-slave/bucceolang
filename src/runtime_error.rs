#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError(u64, String),
    DivisionByZero(u64),
    UndefinedVariable(u64, String),
    Other(u64, String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(line, msg) => write!(f, "[line: {}] Type Error: {}", line, msg),
            RuntimeError::DivisionByZero(line) => write!(f, "[line: {}] Division by zero", line),
            RuntimeError::UndefinedVariable(line, name) => {
                write!(f, "[line: {}] Undefined variable: {}", line, name)
            }
            RuntimeError::Other(line, msg) => write!(f, "[line: {}] Runtime Error: {}", line, msg),
        }
    }
}
