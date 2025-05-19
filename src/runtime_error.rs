use crate::object::Object;

/// Represents errors that can occur during the runtime execution of the code.
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// Indicates a type mismatch error.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
    TypeError(usize, String),

    /// Indicates an attempt to divide by zero.
    /// The element is the line number where the division by zero occurred.
    DivisionByZero(usize),

    /// Indicates that a variable was accessed before it was defined.
    /// The first element is the line number where the undefined variable was used.
    /// The second element is the name of the undefined variable.
    UndefinedVariable(usize, String),

    /// Represents any other runtime error not covered by specific variants.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
    Other(usize, String),
    /// special error type thats used to propagate the the return value of functions thru the
    /// callstack, it gets caught by the 'Call()' function
    Return(Object),
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
            _ => write!(f, "should be a error"),
        }
    }
}
