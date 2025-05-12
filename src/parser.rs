use crate::ast_types::*;
use crate::parser_error::{self, ParseError};
use crate::runtime_error::RuntimeError;
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
            Err(e) => Err(format!("Parse error: {}", e.message)),
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
