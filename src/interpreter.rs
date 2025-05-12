use crate::parser::ExprVisitor;
use crate::runtime_error::RuntimeError;
use crate::Token;

pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "Nil"),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Number(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "{}", string),
        }
    }
}

#[derive(Debug)]
pub struct Interpreter;

impl ExprVisitor<Object> for Interpreter {
    // simply pull runtime expression back out of the token
    fn visit_literal_expr(
        &self,
        expr: &crate::parser::LiteralExpr,
    ) -> Result<Object, RuntimeError> {
        match &expr.literal {
            Token::String(s) => Ok(Object::String(s.to_string())),
            Token::Number(n) => Ok(Object::Number(
                n.parse::<f64>()
                    .expect("Should have been able to parse the value"),
            )),
            Token::True => Ok(Object::Boolean(true)),
            Token::False => Ok(Object::Boolean(false)),
            Token::Nil => Ok(Object::Nil),
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

        match expr.prefix {
            Token::Bang => Ok(Object::Boolean(!is_truthy(&right))),
            Token::Minus => {
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

        match expr.operator {
            Token::Minus => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val - right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands musst be numbers".to_string(),
                    ))
                }
            }
            Token::Plus => match (&left, &right) {
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
            Token::Slash => {
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
            Token::Asterisk => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val * right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }

            // Comparison operators
            Token::Greater => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val > right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            Token::GreaterEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val >= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            Token::Less => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val < right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            Token::LessEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val <= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        "Operands must be numbers".to_string(),
                    ))
                }
            }

            Token::EqualEqual => Ok(Object::Boolean(is_equal(&left, &right))),
            Token::BangEqual => Ok(Object::Boolean(!is_equal(&left, &right))),

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
