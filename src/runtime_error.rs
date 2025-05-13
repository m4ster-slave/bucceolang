/// Represents errors that can occur during the runtime execution of the code.
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// Indicates a type mismatch error.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
    TypeError(u64, String),

    /// Indicates an attempt to divide by zero.
    /// The element is the line number where the division by zero occurred.
    DivisionByZero(u64),

    /// Indicates that a variable was accessed before it was defined.
    /// The first element is the line number where the undefined variable was used.
    /// The second element is the name of the undefined variable.
    UndefinedVariable(u64, String),

    /// Represents any other runtime error not covered by specific variants.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
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
