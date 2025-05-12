use crate::Token;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: u64,
    pub location: String,
    pub message: String,
}

pub fn error(token: &Token, message: String) -> ParseError {
    let (line, where_msg) = match token {
        Token::EOF => (0, "at end".to_string()),
        _ => (
            get_token_line(token),
            format!("at '{}'", token_to_string(token)),
        ),
    };

    let error = ParseError {
        line,
        location: where_msg,
        message,
    };

    report(error.line, &error.location, &error.message);

    error
}

// TODO
fn get_token_line(token: &Token) -> u64 {
    0
}

fn token_to_string(token: &Token) -> String {
    match token {
        Token::String(s) => format!("\"{}\"", s),
        Token::Number(n) => n.clone(),
        Token::Var(name) => name.clone(),
        _ => format!("{:?}", token),
    }
}

pub fn report(line: u64, where_msg: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, where_msg, message)
}
