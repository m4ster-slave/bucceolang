use std::fmt;

/// Represents the different types of tokens that can be produced by the scanner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Asterisk,   // *

    // One or two character tokens
    BangEqual,    // !=
    Bang,         // !
    EqualEqual,   // ==
    Equal,        // =
    LessEqual,    // <=
    Less,         // <
    GreaterEqual, // >=
    Greater,      // >
    Slash,        // /

    // Logical and Bitwise Operators
    And,        // &&
    Or,         // ||
    BitwiseAnd, // &
    BitwiseOr,  // |

    // Literals
    String, // "..."
    Number, // 123, 123.45

    // Keywords
    Else,
    False,
    For,
    Fn,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    VarKeyword, // Represents the 'var' keyword specifically when used for declaration
    While,
    Class,

    // Identifier
    Var, // Represents a variable or identifier name

    // End of file
    Eof,
}

/// Represents a single token produced by the scanner.
#[derive(Debug, Clone)]
pub struct Token {
    /// The type of the token.
    token_type: TokenType,
    /// The raw string slice from the input that produced this token.
    lexeme: String,
    /// The literal value of the token, if it's a literal (string, number, boolean, nil).
    /// This is `None` for other token types.
    literal: Option<crate::object::Object>,
    /// The line number in the input where this token was found.
    line: usize,
}

impl Token {
    /// Creates a new `Token`.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token.
    /// * `lexeme` - The raw string slice from the input.
    /// * `literal` - The literal value, if applicable (e.g., parsed number or string).
    /// * `line` - The line number where the token was found.
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<crate::object::Object>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    /// Returns a reference to the token's type.
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    /// Returns the line number where the token was found.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Returns a string slice representing the token's lexeme.
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    /// Returns a reference to the token's literal value, if present.
    pub fn literal(&self) -> &Option<crate::object::Object> {
        &self.literal
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}

#[cfg(test)]
mod test {
    use crate::object::Object;

    use super::*;
    #[test]
    fn test_token() {
        let t = Token::new(TokenType::Nil, "nil", Some(Object::Nil), 0);
        assert_eq!(t.line(), 0);
        assert_eq!(t.lexeme(), "nil");
        assert_eq!(*t.token_type(), TokenType::Nil);
        assert_eq!(*t.literal(), Some(Object::Nil));
    }
}
