use crate::{token::TokenType, Token};

/// Represents an error that occurred during the parsing phase.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The line number in the source code where the error occurred.
    pub line: u64,
    /// A description of the location within the line where the error was detected.
    pub location: String,
    /// A detailed message describing the parsing error.
    pub message: String,
}

/// Creates a new `ParseError` and reports it to stderr.
///
/// This function constructs a `ParseError` based on the provided `Token` and
/// error message. It also prints a formatted error message to the standard
/// error stream, including the line number, location, and error message.
///
/// # Arguments
///
/// * `token` - The `Token` that caused the parsing error. This is used to
///   determine the line number and location of the error.
/// * `message` - A string describing the specific parsing error.
///
/// # Returns
///
/// A `ParseError` struct containing details about the error.
pub fn error(token: &Token, message: String) -> ParseError {
    let (line, where_msg) = match token.token_type() {
        // Special handling for EOF token to indicate the error is at the end of the input.
        TokenType::Eof => (token.line(), "at end".to_string()),
        // For other tokens, the location is the token itself.
        _ => (token.line(), format!("at '{}'", token)),
    };

    let error = ParseError {
        line,
        location: where_msg,
        message,
    };

    // Report the error to the standard error stream.
    eprintln!(
        "[line {}] Error {}: {}",
        error.line, error.location, error.message
    );

    error
}
