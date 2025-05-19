/// Represents possible errors that can occur during the scanning (tokenization) process.
#[derive(Debug, Clone)]
pub enum ScannerError {
    /// Indicates an invalid variable name was encountered.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
    InvalidVariableName(usize, String),

    /// Indicates a general syntax error was encountered.
    /// The first element is the line number where the error occurred.
    /// The second element is a descriptive error message.
    InvalidSyntax(usize, String),
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
