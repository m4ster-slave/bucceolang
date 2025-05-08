use crate::*;

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
        todo!();
    }
}

pub fn parse(token_input: Vec<Token>) -> Result<Box<Expr>, String> {
    let mut parser = Parser::new(token_input);
    parser.parse()
}
