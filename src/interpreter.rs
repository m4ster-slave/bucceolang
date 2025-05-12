use crate::object::Object;
use crate::parser::ExprVisitor;
use crate::runtime_error::RuntimeError;
use crate::token::TokenType;

#[derive(Debug)]
pub struct Interpreter;

impl ExprVisitor<Object> for Interpreter {
    // simply pull runtime expression back out of the token
    fn visit_literal_expr(
        &self,
        expr: &crate::parser::LiteralExpr,
    ) -> Result<Object, RuntimeError> {
        match &expr.literal.token_type() {
            TokenType::String
            | TokenType::Number
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => match &expr.literal.literal() {
                Some(obj) => Ok(obj.clone()),
                None => match expr.literal.token_type() {
                    TokenType::String => Ok(Object::String("".to_string())),
                    TokenType::Number => Err(RuntimeError::TypeError(
                        "Expected a number literal, but found none.".to_string(),
                    )),
                    TokenType::True => Ok(Object::Boolean(true)),
                    TokenType::False => Ok(Object::Boolean(false)),
                    TokenType::Nil => Ok(Object::Nil),
                    _ => Err(RuntimeError::TypeError(
                        "Unhandled literal type".to_string(),
                    )),
                },
            },
            _ => Err(RuntimeError::TypeError(
                "Not a valid literal to be parsed".to_string(),
            )),
        }
    }

    // evaluate the subexpression in the grouping
    fn visit_grouping_expr(
        &self,
        expr: &crate::parser::GroupingExpr,
    ) -> Result<Object, RuntimeError> {
        expr.expr.accept(self)
    }

    fn visit_unary_expr(&self, expr: &crate::parser::UnaryExpr) -> Result<Object, RuntimeError> {
        let right = expr.operator.accept(self)?;

        match expr.prefix.token_type() {
            TokenType::Bang => Ok(Object::Boolean(!is_truthy(&right))),
            TokenType::Minus => {
                if let Object::Number(value) = right {
                    Ok(Object::Number(-value))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operand must be a number".to_string(),
                    ))
                }
            }
            _ => panic!("Not a valid unary expression"),
        }
    }

    fn visit_binary_expr(&self, expr: &crate::parser::BinaryExpr) -> Result<Object, RuntimeError> {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;

        match expr.operator.token_type() {
            TokenType::Minus => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val - right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands musst be numbers".to_string(),
                    ))
                }
            }
            TokenType::Plus => match (&left, &right) {
                (Object::Number(left_val), Object::Number(right_val)) => {
                    Ok(Object::Number(left_val + right_val))
                }
                (Object::String(left_val), Object::String(right_val)) => {
                    Ok(Object::String(format!("{}{}", left_val, right_val)))
                }
                _ => Err(RuntimeError::TypeError(
                    "Operands must be two numbers or two strings".to_string(),
                )),
            },
            TokenType::Slash => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    if *right_val == 0.0 {
                        return Err(RuntimeError::DivisionByZero);
                    }
                    Ok(Object::Number(left_val / right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operand must be numbersr".to_string(),
                    ))
                }
            }
            TokenType::Asterisk => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val * right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }

            // Comparison operators
            TokenType::Greater => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val > right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val >= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::Less => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val < right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::LessEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val <= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }

            TokenType::EqualEqual => Ok(Object::Boolean(is_equal(&left, &right))),
            TokenType::BangEqual => Ok(Object::Boolean(!is_equal(&left, &right))),

            _ => panic!("Unknown operator"),
        }
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Boolean(value) => *value,
        _ => true,
    }
}

fn is_equal(a: &Object, b: &Object) -> bool {
    match (a, b) {
        (Object::Number(a_val), Object::Number(b_val)) => a_val == b_val,
        (Object::String(a_val), Object::String(b_val)) => a_val == b_val,
        (Object::Boolean(a_val), Object::Boolean(b_val)) => a_val == b_val,
        (Object::Nil, Object::Nil) => true,
        _ => false,
    }
}
