use crate::{token::TokenType, Token};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: u64,
    pub location: String,
    pub message: String,
}

pub fn error(token: &Token, message: String) -> ParseError {
    let (line, where_msg) = match token.token_type() {
        TokenType::EOF => (0, "at end".to_string()),
        _ => (token.line(), format!("at '{}'", token_to_string(token))),
    };

    let error = ParseError {
        line,
        location: where_msg,
        message,
    };

    eprintln!(
        "[line {}] Error {}: {}",
        error.line, error.location, error.message
    );

    error
}

fn token_to_string(token: &Token) -> String {
    match token.token_type() {
        TokenType::String => {
            if let Some(obj) = token.literal() {
                format!("\"{}\"", obj)
            } else {
                "\"\"".to_string()
            }
        }
        TokenType::Number => {
            if let Some(obj) = token.literal() {
                obj.to_string()
            } else {
                "0".to_string()
            }
        }
        TokenType::Var => {
            if let Some(obj) = token.literal() {
                obj.to_string()
            } else {
                "unnamed_var".to_string()
            }
        }
        _ => {
            format!("{:?}", token)
        }
    }
}
