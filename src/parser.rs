use crate::Token;

//expression     → literal
//               | unary
//               | binary
//               | grouping ;
//
//literal        → NUMBER | STRING | "true" | "false" | "nil" ;
//grouping       → "(" expression ")" ;
//unary          → ( "-" | "!" ) expression ;
//binary         → expression operator expression ;
//operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//               | "+"  | "-"  | "*" | "/" ;
//
//
//expression     → equality ;
//equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//term           → factor ( ( "-" | "+" ) factor )* ;
//factor         → unary ( ( "/" | "*" ) unary )* ;
//unary          → ( "!" | "-" ) unary
//               | primary ;
//primary        → NUMBER | STRING | "true" | "false" | "nil"
//               | "(" expression ")" ;
//

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

// visitor pattern to evaluate expressions
pub trait ExprVisitor<T> {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> T;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub literal: Token,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub paren_open: Token,
    pub expr: Box<Expr>,
    pub paren_close: Token,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub prefix: Token,
    pub operator: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

// Parser implementation
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, String> {
        let expr = self.expression()?;
        Ok(Box::new(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[Token::BangEqual, Token::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Token::Minus, Token::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Token::Slash, Token::Asterisk]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Token::Bang, Token::Minus]) {
            let prefix = self.previous().clone();
            let operator = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                prefix,
                operator: Box::new(operator),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(Token::False)
            || self.match_token(Token::True)
            || self.match_token(Token::Nil)
        {
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }

        // Check for Number token
        if let Some(Token::Number(_)) = self.peek_type() {
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }

        // Check for String token
        if let Some(Token::String(_)) = self.peek_type() {
            self.advance();
            return Ok(Expr::Literal(LiteralExpr {
                literal: self.previous().clone(),
            }));
        }

        if self.match_token(Token::LeftParen) {
            let paren_open = self.previous().clone();
            let expr = self.expression()?;

            if !self.check(&Token::RightParen) {
                return Err("Expected ')' after expression".to_string());
            }

            self.advance();
            let paren_close = self.previous().clone();

            return Ok(Expr::Grouping(GroupingExpr {
                paren_open,
                expr: Box::new(expr),
                paren_close,
            }));
        }

        Err(format!("Expected expression but found {:?}", self.peek()))
    }

    fn match_tokens(&mut self, types: &[Token]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn match_token(&mut self, token_type: Token) -> bool {
        if self.check(&token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        // For specific token types like Number and String, we need to check only the variant, not the value
        match (self.peek(), token_type) {
            (Token::Number(_), Token::Number(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Var(_), Token::Var(_)) => true,
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_type(&self) -> Option<&Token> {
        if self.is_at_end() {
            None
        } else {
            Some(self.peek())
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

pub fn parse(token_input: Vec<Token>) -> Result<Box<Expr>, String> {
    let mut parser = Parser::new(token_input);
    parser.parse()
}
