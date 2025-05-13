use crate::ast_types::*;
use crate::parser_error::{self, ParseError};
use crate::token::TokenType;
use crate::Token;

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, String> {
        match self.expression() {
            Ok(expr) if !self.had_error => Ok(Box::new(expr)),
            Ok(_) => Err("Parsing completed with errors.".into()),
            Err(e) => Err(format!("Parsing error: {}", e.message)),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[TokenType::Slash, TokenType::Asterisk]) {
            let operator = self.previous().clone();
            let right = self.unary().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let prefix = self.previous().clone();
            let operator = self.unary().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            return Ok(Expr::Unary(UnaryExpr {
                prefix,
                operator: Box::new(operator),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }
        if let TokenType::Number = &self.peek().token_type() {
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }
        if let TokenType::String = &self.peek().token_type() {
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }
        if self.match_token(TokenType::LeftParen) {
            let paren_open = self.previous().clone();
            let expr = self.expression().map_err(|e| {
                self.had_error = true;
                self.synchronize();
                e
            })?;
            if !self.check(&TokenType::RightParen) {
                let error =
                    parser_error::error(self.peek(), "Expected ')' after expression".into());
                self.had_error = true;
                self.synchronize();
                return Err(error);
            }
            self.advance();
            let paren_close = self.previous().clone();
            return Ok(Expr::Grouping(GroupingExpr {
                paren_open,
                expr: Box::new(expr),
                paren_close,
            }));
        }
        let error = parser_error::error(self.peek(), "Expected expression".into());
        self.had_error = true;
        self.synchronize();
        Err(error)
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(&token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        // For specific token types like Number and String, we need to check only the variant, not the value
        std::mem::discriminant(self.peek().token_type()) == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type(), TokenType::EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if matches!(self.previous().token_type(), TokenType::Semicolon) {
                return;
            }
            match &self.peek().token_type() {
                TokenType::Class
                | TokenType::Fn
                | TokenType::VarKeyword
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}

pub fn parse(token_input: Vec<Token>) -> Result<Box<Expr>, String> {
    let mut parser = Parser::new(token_input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use crate::token::{Token, TokenType};

    fn create_token(
        token_type: TokenType,
        lexeme: &str,
        line: u64,
        literal: Option<Object>,
    ) -> Token {
        Token::new(token_type, lexeme, literal, line)
    }

    fn create_eof(line: u64) -> Token {
        create_token(TokenType::EOF, "", line, None)
    }

    #[test]
    fn test_parse_number_literal() {
        let tokens = vec![
            create_token(TokenType::Number, "123", 1, Some(Object::Number(123.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Literal(lit) => match lit.literal.literal() {
                Some(Object::Number(val)) => assert_eq!(*val, 123.0),
                _ => panic!("Expected number literal"),
            },
            _ => panic!("Expected literal expression"),
        }
    }

    #[test]
    fn test_parse_string_literal() {
        let tokens = vec![
            create_token(
                TokenType::String,
                "\"hello\"",
                1,
                Some(Object::String("hello".to_string())),
            ),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Literal(lit) => match lit.literal.literal() {
                Some(Object::String(val)) => assert_eq!(val, "hello"),
                _ => panic!("Expected string literal"),
            },
            _ => panic!("Expected literal expression"),
        }
    }

    #[test]
    fn test_parse_boolean_literals() {
        // Test true
        let tokens = vec![
            create_token(TokenType::True, "true", 1, Some(Object::Boolean(true))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Literal(lit) => match lit.literal.literal() {
                Some(Object::Boolean(val)) => assert!(*val),
                _ => panic!("Expected boolean literal"),
            },
            _ => panic!("Expected literal expression"),
        }

        // Test false
        let tokens = vec![
            create_token(TokenType::False, "false", 1, Some(Object::Boolean(false))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Literal(lit) => match lit.literal.literal() {
                Some(Object::Boolean(val)) => assert!(!(*val)),
                _ => panic!("Expected boolean literal"),
            },
            _ => panic!("Expected literal expression"),
        }
    }

    #[test]
    fn test_parse_nil_literal() {
        let tokens = vec![
            create_token(TokenType::Nil, "nil", 1, Some(Object::Nil)),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Literal(lit) => match lit.literal.literal() {
                Some(Object::Nil) => {}
                _ => panic!("Expected nil literal"),
            },
            _ => panic!("Expected literal expression"),
        }
    }

    #[test]
    fn test_parse_grouping() {
        let tokens = vec![
            create_token(TokenType::LeftParen, "(", 1, None),
            create_token(TokenType::Number, "123", 1, Some(Object::Number(123.0))),
            create_token(TokenType::RightParen, ")", 1, None),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Grouping(group) => match group.expr.as_ref() {
                Expr::Literal(lit) => match lit.literal.literal() {
                    Some(Object::Number(val)) => assert_eq!(*val, 123.0),
                    _ => panic!("Expected number literal inside grouping"),
                },
                _ => panic!("Expected literal expression inside grouping"),
            },
            _ => panic!("Expected grouping expression"),
        }
    }

    #[test]
    fn test_parse_unary_minus() {
        let tokens = vec![
            create_token(TokenType::Minus, "-", 1, None),
            create_token(TokenType::Number, "123", 1, Some(Object::Number(123.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Unary(unary) => {
                assert!(matches!(unary.prefix.token_type(), TokenType::Minus));
                match unary.operator.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 123.0),
                        _ => panic!("Expected number literal in unary expression"),
                    },
                    _ => panic!("Expected literal expression in unary operator"),
                }
            }
            _ => panic!("Expected unary expression"),
        }
    }

    #[test]
    fn test_parse_unary_bang() {
        let tokens = vec![
            create_token(TokenType::Bang, "!", 1, None),
            create_token(TokenType::True, "true", 1, Some(Object::Boolean(true))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Unary(unary) => {
                assert!(matches!(unary.prefix.token_type(), TokenType::Bang));
                match unary.operator.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Boolean(val)) => assert!(*val),
                        _ => panic!("Expected boolean literal in unary expression"),
                    },
                    _ => panic!("Expected literal expression in unary operator"),
                }
            }
            _ => panic!("Expected unary expression"),
        }
    }

    #[test]
    fn test_parse_binary_addition() {
        let tokens = vec![
            create_token(TokenType::Number, "1", 1, Some(Object::Number(1.0))),
            create_token(TokenType::Plus, "+", 1, None),
            create_token(TokenType::Number, "2", 1, Some(Object::Number(2.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Plus));

                match binary.left.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 1.0),
                        _ => panic!("Expected number literal for left operand"),
                    },
                    _ => panic!("Expected literal expression for left operand"),
                }

                match binary.right.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 2.0),
                        _ => panic!("Expected number literal for right operand"),
                    },
                    _ => panic!("Expected literal expression for right operand"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_binary_subtraction() {
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::Minus, "-", 1, None),
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Minus));

                match binary.left.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 5.0),
                        _ => panic!("Expected number literal for left operand"),
                    },
                    _ => panic!("Expected literal expression for left operand"),
                }

                match binary.right.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 3.0),
                        _ => panic!("Expected number literal for right operand"),
                    },
                    _ => panic!("Expected literal expression for right operand"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_binary_multiplication() {
        let tokens = vec![
            create_token(TokenType::Number, "2", 1, Some(Object::Number(2.0))),
            create_token(TokenType::Asterisk, "*", 1, None),
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Asterisk));

                match binary.left.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 2.0),
                        _ => panic!("Expected number literal for left operand"),
                    },
                    _ => panic!("Expected literal expression for left operand"),
                }

                match binary.right.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 3.0),
                        _ => panic!("Expected number literal for right operand"),
                    },
                    _ => panic!("Expected literal expression for right operand"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_binary_division() {
        let tokens = vec![
            create_token(TokenType::Number, "6", 1, Some(Object::Number(6.0))),
            create_token(TokenType::Slash, "/", 1, None),
            create_token(TokenType::Number, "2", 1, Some(Object::Number(2.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Slash));

                match binary.left.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 6.0),
                        _ => panic!("Expected number literal for left operand"),
                    },
                    _ => panic!("Expected literal expression for left operand"),
                }

                match binary.right.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 2.0),
                        _ => panic!("Expected number literal for right operand"),
                    },
                    _ => panic!("Expected literal expression for right operand"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_comparison_operators() {
        // Test >
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::Greater, ">", 1, None),
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Greater));
            }
            _ => panic!("Expected binary expression with > operator"),
        }

        // Test >=
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::GreaterEqual, ">=", 1, None),
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(
                    binary.operator.token_type(),
                    TokenType::GreaterEqual
                ));
            }
            _ => panic!("Expected binary expression with >= operator"),
        }

        // Test <
        let tokens = vec![
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_token(TokenType::Less, "<", 1, None),
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Less));
            }
            _ => panic!("Expected binary expression with < operator"),
        }

        // Test <=
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::LessEqual, "<=", 1, None),
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::LessEqual));
            }
            _ => panic!("Expected binary expression with <= operator"),
        }
    }

    #[test]
    fn test_parse_equality_operators() {
        // Test ==
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::EqualEqual, "==", 1, None),
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(
                    binary.operator.token_type(),
                    TokenType::EqualEqual
                ));
            }
            _ => panic!("Expected binary expression with == operator"),
        }

        // Test !=
        let tokens = vec![
            create_token(TokenType::Number, "5", 1, Some(Object::Number(5.0))),
            create_token(TokenType::BangEqual, "!=", 1, None),
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::BangEqual));
            }
            _ => panic!("Expected binary expression with != operator"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // Test 1 + 2 * 3
        let tokens = vec![
            create_token(TokenType::Number, "1", 1, Some(Object::Number(1.0))),
            create_token(TokenType::Plus, "+", 1, None),
            create_token(TokenType::Number, "2", 1, Some(Object::Number(2.0))),
            create_token(TokenType::Asterisk, "*", 1, None),
            create_token(TokenType::Number, "3", 1, Some(Object::Number(3.0))),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should parse as (1 + (2 * 3)) due to operator precedence
        match expr.as_ref() {
            Expr::Binary(binary) => {
                assert!(matches!(binary.operator.token_type(), TokenType::Plus));

                match binary.left.as_ref() {
                    Expr::Literal(lit) => match lit.literal.literal() {
                        Some(Object::Number(val)) => assert_eq!(*val, 1.0),
                        _ => panic!("Expected number literal for left operand"),
                    },
                    _ => panic!("Expected literal expression for left operand"),
                }

                match binary.right.as_ref() {
                    Expr::Binary(inner_binary) => {
                        assert!(matches!(
                            inner_binary.operator.token_type(),
                            TokenType::Asterisk
                        ));
                    }
                    _ => panic!("Expected binary expression for right operand"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_error_missing_paren() {
        let tokens = vec![
            create_token(TokenType::LeftParen, "(", 1, None),
            create_token(TokenType::Number, "123", 1, Some(Object::Number(123.0))),
            // Missing right paren
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_expression() {
        let tokens = vec![
            // Missing expression
            create_token(TokenType::Plus, "+", 1, None),
            create_eof(1),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
    }
}
