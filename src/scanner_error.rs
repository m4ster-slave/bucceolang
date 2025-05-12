#[derive(Debug, Clone)]
pub enum ScannerError {
    InvalidVariableName(String),
    InvalidSyntax(String),
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScannerError::InvalidVariableName(msg) => write!(f, "Invalid variable name: {}", msg),
            ScannerError::InvalidSyntax(msg) => write!(f, "Invalid Syntax: {}", msg),
        }
    }
}
