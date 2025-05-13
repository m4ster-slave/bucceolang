#[derive(Debug, Clone)]
pub enum ScannerError {
    InvalidVariableName(u64, String),
    InvalidSyntax(u64, String),
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScannerError::InvalidVariableName(line, msg) => {
                write!(f, "[line: {}] Error: Invalid variable name: {}", line, msg)
            }
            ScannerError::InvalidSyntax(line, msg) => {
                write!(f, "[line: {}] Invalid Syntax: {}", line, msg)
            }
        }
    }
}
