mod parser;
mod scanner;

use parser::{parse, Expr, ExprVisitor};
use scanner::scanner::{tokenize, Token};

// A simple implementation of the visitor pattern to print expressions
struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&self, expr: &parser::LiteralExpr) -> String {
        match &expr.literal {
            Token::String(s) => format!("\"{}\"", s),
            Token::Number(n) => n.clone(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::Nil => "nil".to_string(),
            _ => format!("{:?}", expr.literal),
        }
    }

    fn visit_grouping_expr(&self, expr: &parser::GroupingExpr) -> String {
        format!("(group {})", expr.expr.accept(self))
    }

    fn visit_unary_expr(&self, expr: &parser::UnaryExpr) -> String {
        let operator = match &expr.prefix {
            Token::Minus => "-",
            Token::Bang => "!",
            _ => panic!("Invalid unary operator"),
        };
        format!("({} {})", operator, expr.operator.accept(self))
    }

    fn visit_binary_expr(&self, expr: &parser::BinaryExpr) -> String {
        let operator = match &expr.operator {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::BangEqual => "!=",
            Token::EqualEqual => "==",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            _ => panic!("Invalid binary operator"),
        };
        format!(
            "({} {} {})",
            operator,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
}

fn main() {
    // Example usage
    let source = "1 + 2 * (3 - 4)";
    println!("Source: {}", source);

    // Tokenize
    let tokens = tokenize(source);
    println!("Tokens: {:?}", tokens);

    // Parse
    match parse(tokens) {
        Ok(expr) => {
            let printer = AstPrinter;
            println!("AST: {}", expr.accept(&printer));
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
