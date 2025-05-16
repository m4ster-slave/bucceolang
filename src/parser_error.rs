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
        _ => (token.line(), format!("at '{}'", token_to_string(token))),
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

/// Converts a `Token` into a human-readable string representation for error reporting.
///
/// This helper function provides a more user-friendly string representation of
/// different token types, especially for literals like strings and numbers,
/// and for variable names.
///
/// # Arguments
///
/// * `token` - The `Token` to convert to a string.
///
/// # Returns
///
/// A `String` representation of the token.
fn token_to_string(token: &Token) -> String {
    match token.token_type() {
        // Handle string literals, including the quotes.
        TokenType::String => {
            if let Some(obj) = token.literal() {
                // Assuming Object implements Display
                format!("\"{}\"", obj)
            } else {
                // Fallback for an empty string literal (though this might not happen
                // if the scanner correctly handles empty strings)
                "\"\"".to_string()
            }
        }
        // Handle number literals.
        TokenType::Number => {
            if let Some(obj) = token.literal() {
                // Assuming Object implements Display
                obj.to_string()
            } else {
                // Fallback for a number with no literal value (shouldn't happen
                // if scanner is correct)
                "0".to_string()
            }
        }
        // Handle variable names.
        TokenType::Var => {
            // Variable tokens should have a lexeme that is the variable name,
            // but the literal might be None until resolved. Using the lexeme
            // is generally more reliable here for error reporting.
            token.lexeme().to_string()
            // The original logic with obj.to_string() on the literal might be
            // intended for a different purpose or might be a potential issue
            // if the literal is None for a Var token. Let's stick to the lexeme
            // for error reporting as it's the source text.
            // if let Some(obj) = token.literal() {
            //     obj.to_string()
            // } else {
            //     "unnamed_var".to_string()
            // }
        }
        // For all other token types, use the Debug representation.
        _ => {
            format!("{:?}", token.token_type()) // Use TokenType's Debug for conciseness
        }
    }
}
