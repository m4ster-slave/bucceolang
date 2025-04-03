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

pub enum Expr {
    Literal(Box<LiteralExpr>),
    Grouping(Box<GroupingExpr>),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Operator(Box<OperatorExpr>),
}

pub(crate) struct LiteralExpr {
    literal: Token,
}

pub(crate) struct GroupingExpr {
    paren_open: Token,
    expr: Box<Expr>,
    paren_close: Token,
}

pub(crate) struct UnaryExpr {
    prefix: Token,
    operator: Box<Expr>,
}

pub(crate) struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

// technically we dont need this because only the binary expr uses this and it has a token but well
// keep it here for completeness sake
pub(crate) struct OperatorExpr {
    operator: Token,
}

pub fn parse(input: &str) -> i32 {
    return 10;
}
