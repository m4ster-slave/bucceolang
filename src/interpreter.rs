use crate::expr_types::*;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::stmt_types::StmtVisitor;
use crate::stmt_types::*;
use crate::token::TokenType;

/// A struct responsible for interpreting a list of statements.
///
/// It processes each statement and can potentially return a `RuntimeError`
/// if an issue occurs during execution.
#[derive(Debug)]
pub struct Interpreter;

impl ExprVisitor<Object> for Interpreter {
    // simply pull runtime expression back out of the token
    fn visit_literal_expr(
        &self,
        expr: &crate::expr_types::LiteralExpr,
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
                        expr.literal.line(),
                        "Expected a number literal, but found none.".to_string(),
                    )),
                    TokenType::True => Ok(Object::Boolean(true)),
                    TokenType::False => Ok(Object::Boolean(false)),
                    TokenType::Nil => Ok(Object::Nil),
                    _ => Err(RuntimeError::TypeError(
                        expr.literal.line(),
                        "Unhandled literal type".to_string(),
                    )),
                },
            },
            _ => Err(RuntimeError::TypeError(
                expr.literal.line(),
                "Not a valid literal to be parsed".to_string(),
            )),
        }
    }

    // evaluate the subexpression in the grouping
    fn visit_grouping_expr(
        &self,
        expr: &crate::expr_types::GroupingExpr,
    ) -> Result<Object, RuntimeError> {
        expr.expr.accept(self)
    }

    fn visit_unary_expr(
        &self,
        expr: &crate::expr_types::UnaryExpr,
    ) -> Result<Object, RuntimeError> {
        let right = expr.operator.accept(self)?;

        match expr.prefix.token_type() {
            TokenType::Bang => Ok(Object::Boolean(!is_truthy(&right))),
            TokenType::Minus => {
                if let Object::Number(value) = right {
                    Ok(Object::Number(-value))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.prefix.line(),
                        "Operand must be a number".to_string(),
                    ))
                }
            }
            _ => panic!("Not a valid unary expression"),
        }
    }

    fn visit_binary_expr(
        &self,
        expr: &crate::expr_types::BinaryExpr,
    ) -> Result<Object, RuntimeError> {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;

        match expr.operator.token_type() {
            TokenType::Minus => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val - right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
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
                    expr.operator.line(),
                    "Operands must be two numbers or two strings".to_string(),
                )),
            },
            TokenType::Slash => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    if *right_val == 0.0 {
                        return Err(RuntimeError::DivisionByZero(expr.operator.line()));
                    }
                    Ok(Object::Number(left_val / right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
                        "Operand must be numbersr".to_string(),
                    ))
                }
            }
            TokenType::Asterisk => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Number(left_val * right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
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
                        expr.operator.line(),
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val >= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::Less => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val < right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
                        "Operands must be numbers".to_string(),
                    ))
                }
            }
            TokenType::LessEqual => {
                if let (Object::Number(left_val), Object::Number(right_val)) = (&left, &right) {
                    Ok(Object::Boolean(left_val <= right_val))
                } else {
                    Err(RuntimeError::TypeError(
                        expr.operator.line(),
                        "Operands must be numbers".to_string(),
                    ))
                }
            }

            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            TokenType::BangEqual => Ok(Object::Boolean(!(left == right))),

            _ => panic!("Unknown operator"),
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expr_stmt(&self, stmt: &Expr) -> Result<(), RuntimeError> {
        stmt.accept(self)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &Expr) -> Result<(), RuntimeError> {
        let obj: Object = stmt.accept(self)?;
        println!("{obj}");
        Ok(())
    }
}

/// Determines the truthiness of a runtime `Object`.
///
/// In this language, `nil` and `false` are considered "falsey".
/// All other object types (numbers, strings, `true` boolean) are considered "truthy".
///
/// # Arguments
///
/// * `obj` - A reference to the `Object` to check for truthiness.
///
/// # Returns
///
/// `true` if the object is truthy, `false` if it is falsey.
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Boolean(value) => *value,
        // Any other Object type is considered truthy.
        _ => true,
    }
}

impl Interpreter {
    /// Interprets a list of statements.
    ///
    /// Each statement in the provided vector is evaluated in order.
    /// If any statement evaluation results in a `RuntimeError`, the
    /// interpretation process is halted and the error is returned.
    ///
    /// # Arguments
    ///
    /// * `stmts`: A `Vec<Stmt>` containing the statements to be interpreted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all statements are interpreted successfully.
    /// Returns `Err(RuntimeError)` if an error occurs during the evaluation
    /// of any statement.
    pub fn interprete(&self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            match stmt.evaluate(self) {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::expr_types::*;
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::stmt_types::*;
    use crate::token::{Token, TokenType};

    fn create_number_token(value: f64, line: u64) -> Token {
        Token::new(
            TokenType::Number,
            &value.to_string(),
            Some(Object::Number(value)),
            line,
        )
    }

    fn create_string_token(value: String, line: u64) -> Token {
        Token::new(
            TokenType::String,
            &value.clone(),
            Some(Object::String(value)),
            line,
        )
    }

    fn create_print_token(value: String, line: u64) -> Token {
        Token::new(TokenType::Print, "print", None, line)
    }

    fn create_literal_expr(token: Token) -> Expr {
        Expr::Literal(LiteralExpr { literal: token })
    }

    fn create_print_stmt(expr: Expr) -> Stmt {
        Stmt::Print(expr)
    }

    #[test]
    fn test_create_print_stmt() {
        let interpreter = Interpreter;
        let print_stmt = create_print_stmt(create_literal_expr(create_number_token(50.0, 1)));

        let result = print_stmt.evaluate(&interpreter);
        assert!(
            result.is_ok(),
            "Expected function to succeed, but got an error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_literal_number() {
        let interpreter = Interpreter;
        let number_token = create_number_token(42.0, 1);
        let expr = create_literal_expr(number_token);

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(42.0));
    }

    #[test]
    fn test_literal_string() {
        let interpreter = Interpreter;
        let string_token = create_string_token("hello".to_string(), 1);
        let expr = create_literal_expr(string_token);

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("hello".to_string()));
    }

    #[test]
    fn test_literal_boolean() {
        let interpreter = Interpreter;

        // Test true
        let true_token = Token::new(TokenType::True, "true", Some(Object::Boolean(true)), 1);
        let expr = create_literal_expr(true_token);
        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test false
        let false_token = Token::new(TokenType::False, "false", Some(Object::Boolean(false)), 1);
        let expr = create_literal_expr(false_token);
        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(false));
    }

    #[test]
    fn test_literal_nil() {
        let interpreter = Interpreter;
        let nil_token = Token::new(TokenType::Nil, "nil", Some(Object::Nil), 1);
        let expr = create_literal_expr(nil_token);

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Nil);
    }

    #[test]
    fn test_grouping_expr() {
        let interpreter = Interpreter;
        let number_token = create_number_token(42.0, 1);
        let number_expr = create_literal_expr(number_token);

        let left_paren = Token::new(TokenType::LeftParen, "(", None, 1);
        let right_paren = Token::new(TokenType::RightParen, ")", None, 1);
        let expr = Expr::Grouping(GroupingExpr {
            paren_open: left_paren,
            expr: Box::new(number_expr),
            paren_close: right_paren,
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(42.0));
    }

    #[test]
    fn test_unary_minus() {
        let interpreter = Interpreter;
        let number_token = create_number_token(42.0, 1);
        let number_expr = create_literal_expr(number_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1);
        let expr = Expr::Unary(UnaryExpr {
            prefix: minus_token,
            operator: Box::new(number_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(-42.0));
    }

    #[test]
    fn test_unary_bang() {
        let interpreter = Interpreter;

        // Test !true -> false
        let true_token = Token::new(TokenType::True, "true", Some(Object::Boolean(true)), 1);
        let true_expr = create_literal_expr(true_token);
        let bang_token = Token::new(TokenType::Bang, "!", None, 1);
        let expr = Expr::Unary(UnaryExpr {
            prefix: bang_token.clone(),
            operator: Box::new(true_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(false));

        // Test !false -> true
        let false_token = Token::new(TokenType::False, "false", Some(Object::Boolean(false)), 1);
        let false_expr = create_literal_expr(false_token);
        let expr = Expr::Unary(UnaryExpr {
            prefix: bang_token,
            operator: Box::new(false_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_binary_arithmetic() {
        let interpreter = Interpreter;

        // Test 5 + 3 = 8
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let plus_token = Token::new(TokenType::Plus, "+", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: plus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(8.0));
    }

    #[test]
    fn test_binary_subtraction() {
        let interpreter = Interpreter;

        // Test 5 - 3 = 2
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let minus_token = Token::new(TokenType::Minus, "-", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: minus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(2.0));
    }

    #[test]
    fn test_binary_multiplication() {
        let interpreter = Interpreter;

        // Test 5 * 3 = 15
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let asterisk_token = Token::new(TokenType::Asterisk, "*", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: asterisk_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(15.0));
    }

    #[test]
    fn test_binary_division() {
        let interpreter = Interpreter;

        // Test 6 / 3 = 2
        let left_token = create_number_token(6.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let slash_token = Token::new(TokenType::Slash, "/", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: slash_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(2.0));
    }

    #[test]
    fn test_division_by_zero() {
        let interpreter = Interpreter;

        // Test 5 / 0 = error
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(0.0, 1);
        let right_expr = create_literal_expr(right_token);
        let slash_token = Token::new(TokenType::Slash, "/", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: slash_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_concatenation() {
        let interpreter = Interpreter;

        // Test "hello" + " world" = "hello world"
        let left_token = create_string_token("hello".to_string(), 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_string_token(" world".to_string(), 1);
        let right_expr = create_literal_expr(right_token);
        let plus_token = Token::new(TokenType::Plus, "+", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: plus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("hello world".to_string()));
    }

    #[test]
    fn test_binary_comparison() {
        let interpreter = Interpreter;

        // Test 5 > 3 = true
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token.clone());
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token.clone());

        let greater_token = Token::new(TokenType::Greater, ">", None, 1);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: greater_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test 3 < 5 = true
        let left_expr = create_literal_expr(right_token);
        let right_expr = create_literal_expr(left_token);
        let less_token = Token::new(TokenType::Less, "<", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: less_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_equality() {
        let interpreter = Interpreter;

        // Test 5 == 5 = true
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token.clone());
        let right_token = create_number_token(5.0, 1);
        let right_expr = create_literal_expr(right_token);

        let eq_token = Token::new(TokenType::EqualEqual, "==", None, 1);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: eq_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test 5 != 3 = true
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let left_expr = create_literal_expr(left_token);
        let neq_token = Token::new(TokenType::BangEqual, "!=", None, 1);

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: neq_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_complex_expression() {
        let interpreter = Interpreter;

        // Test (5 + 3) * 2 = 16
        let five_token = create_number_token(5.0, 1);
        let five_expr = create_literal_expr(five_token);
        let three_token = create_number_token(3.0, 1);
        let three_expr = create_literal_expr(three_token);
        let two_token = create_number_token(2.0, 1);
        let two_expr = create_literal_expr(two_token);

        let plus_token = Token::new(TokenType::Plus, "+", None, 1);
        let addition = Expr::Binary(BinaryExpr {
            left: Box::new(five_expr),
            operator: plus_token,
            right: Box::new(three_expr),
        });

        let left_paren = Token::new(TokenType::LeftParen, "(", None, 1);
        let right_paren = Token::new(TokenType::RightParen, "(", None, 1);
        let grouping = Expr::Grouping(GroupingExpr {
            paren_open: left_paren,
            expr: Box::new(addition),
            paren_close: right_paren,
        });

        let asterisk_token = Token::new(TokenType::Asterisk, "*", None, 1);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(grouping),
            operator: asterisk_token,
            right: Box::new(two_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(16.0));
    }

    #[test]
    fn test_type_errors() {
        let interpreter = Interpreter;

        // Test "string" - 5 (type error)
        let string_token = create_string_token("string".to_string(), 1);
        let string_expr = create_literal_expr(string_token);
        let number_token = create_number_token(5.0, 1);
        let number_expr = create_literal_expr(number_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(string_expr),
            operator: minus_token,
            right: Box::new(number_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_err());

        // Test -"string" (type error)
        let string_token = create_string_token("string".to_string(), 1);
        let string_expr = create_literal_expr(string_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1);
        let expr = Expr::Unary(UnaryExpr {
            prefix: minus_token,
            operator: Box::new(string_expr),
        });

        let result = expr.accept(&interpreter);
        assert!(result.is_err());
    }
}
