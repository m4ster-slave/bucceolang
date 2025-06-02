use crate::object::Object;

/// Represents errors that can occur during the runtime execution of the code.
#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError { line: usize, message: String },
    DivisionByZero { line: usize, message: String },
    UndefinedVariable { line: usize, message: String },
    ArgumentError { line: usize, message: String },
    Resolver { line: usize, message: String },
    Other { line: usize, message: String },
    Return(Option<Object>),
    Break,
    Continue,
}

// Error constructor functions for each error type
impl RuntimeError {
    pub fn type_error(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::TypeError {
            line,
            message: message.into(),
        }
    }
    pub fn division_by_zero(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::DivisionByZero {
            line,
            message: message.into(),
        }
    }
    pub fn undefined_variable(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::UndefinedVariable {
            line,
            message: message.into(),
        }
    }
    pub fn argument_error(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::ArgumentError {
            line,
            message: message.into(),
        }
    }
    pub fn resolver_error(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::Resolver {
            line,
            message: message.into(),
        }
    }
    pub fn other(line: usize, message: impl Into<String>) -> Self {
        RuntimeError::Other {
            line,
            message: message.into(),
        }
    }
}

/// Helper function to construct a runtime error from a kind string (legacy support).
pub fn runtime_error(kind: &str, line: usize, message: impl Into<String>) -> RuntimeError {
    let msg = message.into();
    match kind {
        "TypeError" => RuntimeError::type_error(line, msg),
        "DivisionByZero" => RuntimeError::division_by_zero(line, msg),
        "UndefinedVariable" => RuntimeError::undefined_variable(line, msg),
        "ArgumentError" => RuntimeError::argument_error(line, msg),
        "Resolver" => RuntimeError::resolver_error(line, msg),
        _ => RuntimeError::other(line, msg),
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Type Error: {}\x1b[0m",
                line, message
            ),
            RuntimeError::DivisionByZero { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Division by zero: {}\x1b[0m",
                line, message
            ),
            RuntimeError::UndefinedVariable { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Undefined variable: {}\x1b[0m",
                line, message
            ),
            RuntimeError::ArgumentError { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Argument Error: {}\x1b[0m",
                line, message
            ),
            RuntimeError::Resolver { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Resolver Error: {}\x1b[0m",
                line, message
            ),
            RuntimeError::Other { line, message } => write!(
                f,
                "\x1b[31;49;1m[line: {}] Runtime Error: {}\x1b[0m",
                line, message
            ),
            RuntimeError::Return(_) => write!(f, "<return>"),
            RuntimeError::Break => write!(f, "<break>"),
            RuntimeError::Continue => write!(f, "<continue>"),
        }
    }
}
