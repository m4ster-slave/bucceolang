use crate::runtime_error::RuntimeError;
use crate::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

// Visitor pattern
pub trait ExprVisitor<T> {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, RuntimeError>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, RuntimeError>;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
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
