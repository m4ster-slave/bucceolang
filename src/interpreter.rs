use crate::callable::CallableObject;
use crate::class::ClassObject;
use crate::environment::Environment;
use crate::expr_types::*;
use crate::function::Function;
use crate::native_functions::*;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::stmt_types::StmtVisitor;
use crate::stmt_types::*;
use crate::token::TokenType;
use crate::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

/// A struct responsible for interpreting a list of statements.
///
/// It processes each statement and can potentially return a `RuntimeError`
/// if an issue occurs during execution.
pub struct Interpreter {
    /// holds any additional environments by the user
    pub environment: Rc<RefCell<Environment>>,
    /// holds the outermost global environment
    pub globals: Rc<RefCell<Environment>>,
    ///
    pub locals: HashMap<Expr, usize>,
    /// output destination for print statements
    pub output: Rc<RefCell<dyn Write>>,
}

impl ExprVisitor<Object> for Interpreter {
    // simply pull runtime expression back out of the token
    fn visit_literal_expr(
        &mut self,
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
        &mut self,
        expr: &mut crate::expr_types::GroupingExpr,
    ) -> Result<Object, RuntimeError> {
        expr.expr.accept(self)
    }

    fn visit_unary_expr(
        &mut self,
        expr: &mut crate::expr_types::UnaryExpr,
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
        &mut self,
        expr: &mut crate::expr_types::BinaryExpr,
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
                // String + anything = concatenation
                (Object::String(left_val), right_val) => {
                    Ok(Object::String(format!("{}{}", left_val, right_val)))
                }
                // Anything + String = concatenation
                (left_val, Object::String(right_val)) => {
                    Ok(Object::String(format!("{}{}", left_val, right_val)))
                }
                _ => Err(RuntimeError::TypeError(
                    expr.operator.line(),
                    format!(
                        "Cannot add {} and {}",
                        match left {
                            Object::Nil => "nil",
                            Object::Boolean(_) => "boolean",
                            Object::Number(_) => "number",
                            Object::String(_) => "string",
                            Object::Callable(_) => "callable",
                            Object::Class(_) => "class",
                            Object::ClassInstance(_) => "class instance",
                        },
                        match right {
                            Object::Nil => "nil",
                            Object::Boolean(_) => "boolean",
                            Object::Number(_) => "number",
                            Object::String(_) => "string",
                            Object::Callable(_) => "callable",
                            Object::Class(_) => "class",
                            Object::ClassInstance(_) => "class instance",
                        }
                    ),
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

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Object, RuntimeError> {
        self.look_up_variable(&expr.name, Expr::Variable(expr.clone()))
    }

    fn visit_assign_expr(&mut self, expr: &mut AssignExpr) -> Result<Object, RuntimeError> {
        let val = expr.value.accept(self)?;

        if let Some(distance) = self.locals.get(&Expr::Assign(expr.clone())) {
            self.environment
                .borrow_mut()
                .assign_at(distance, expr.name.clone(), val.clone())?;
        } else {
            self.globals.borrow_mut().assign(&expr.name, &val)?;
        }

        Ok(val)
    }

    fn visit_logical_expr(&mut self, expr: &mut LogicalExpr) -> Result<Object, RuntimeError> {
        let left = expr.left.accept(self)?;

        if *expr.operator.token_type() == TokenType::Or {
            if is_truthy(&left) {
                return Ok(left);
            }
        } else if !is_truthy(&left) {
            return Ok(left);
        }

        expr.right.accept(self)
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> Result<Object, RuntimeError> {
        let callee = expr.callee.accept(self)?;

        let mut arguments: Vec<Object> = Vec::new();
        for arg in &mut expr.arguments {
            arguments.push(arg.accept(self)?);
        }

        match callee {
            Object::Callable(func) => {
                if arguments.len() != func.arity() {
                    Err(RuntimeError::Other(
                        expr.paren.line(),
                        format!(
                            "Expected {} arguments but got {}.",
                            func.arity(),
                            arguments.len()
                        ),
                    ))
                } else {
                    func.call(self, arguments)
                }
            }
            Object::Class(class) => {
                if arguments.len() != class.arity() {
                    Err(RuntimeError::Other(
                        expr.paren.line(),
                        format!(
                            "Expected {} arguments but got {}.",
                            class.arity(),
                            arguments.len()
                        ),
                    ))
                } else {
                    class.call(self, arguments)
                }
            }
            _ => Err(RuntimeError::TypeError(
                expr.paren.line(),
                "Can only call functions and classes.".to_string(),
            )),
        }
    }

    fn visit_property_access_expr(&mut self, expr: &mut PropertyAccessExpr) -> Result<Object, RuntimeError> {
        let object = expr.object.accept(self)?;
        
        if let Object::ClassInstance(instance) = object {
            instance.get(expr.name.clone())
        } else {
            Err(RuntimeError::TypeError(
                expr.name.line(),
                "Only instances have properties.".to_string(),
            ))
        }
    }

    fn visit_property_assignment_expr(
        &mut self,
        expr: &mut PropertyAssignmentExpr,
    ) -> Result<Object, RuntimeError> {
        let object = expr.object.accept(self)?;

        if let Object::ClassInstance(mut instance) = object.clone() {
            let value = expr.value.accept(self)?;
            instance.set(expr.name.clone(), value.clone());
            Ok(value)
        } else {
            Err(RuntimeError::TypeError(
                expr.name.line(),
                "Only instances have fields.".to_string(),
            ))
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt: &mut Expr) -> Result<(), RuntimeError> {
        stmt.accept(self)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &mut Expr) -> Result<(), RuntimeError> {
        let value = stmt.accept(self)?;
        writeln!(self.output.borrow_mut(), "{}", value)
            .map_err(|e| RuntimeError::Other(0, format!("Print failed: {}", e)))?;
        self.output.borrow_mut().flush().ok();
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &mut VarStmt) -> Result<(), RuntimeError> {
        let val = match &mut stmt.initializer {
            Some(init) => init.accept(self)?,
            None => Object::Nil,
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme().to_string(), val)
    }

    fn visit_block_stmt(&mut self, stmt: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
        let previous = Rc::clone(&self.environment);

        // Create a new environment that encloses the current one
        self.environment = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(
            &previous,
        ))));

        for s in stmt {
            s.evaluate(self)?;
        }

        // Restore previous environment
        self.environment = previous;

        Ok(())
    }

    ///If you compare this code to how the interpreter handles other syntax we’ve implemented, the part that makes control flow special is that Java if statement. Most other syntax trees always evaluate their subtrees. Here, we may not evaluate the then or else statement. If either of those has a side effect, the choice not to evaluate it becomes user visible.
    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Result<(), RuntimeError> {
        if is_truthy(&stmt.condition.accept(self)?) {
            stmt.then_branch.evaluate(self)?
        } else if let Some(else_stmt) = &mut stmt.else_branch {
            else_stmt.evaluate(self)?
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Result<(), RuntimeError> {
        while is_truthy(&stmt.condition.accept(self)?) {
            match stmt.body.evaluate(self) {
                Ok(_) => {}
                Err(RuntimeError::Continue) => continue,
                Err(RuntimeError::Break) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Register a new function in the environment for executing it later
    fn visit_function_stmt(&mut self, stmt: &mut FunctionStmt) -> Result<(), RuntimeError> {
        // .clone increases the RC
        let function = Function::new(stmt.clone(), self.environment.clone());

        self.environment.borrow_mut().define(
            stmt.name.lexeme().to_string(),
            Object::Callable(CallableObject::Function(Rc::new(RefCell::new(function)))),
        )?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt) -> Result<(), RuntimeError> {
        let value = if let Some(expr) = &mut stmt.value {
            Some(expr.accept(self)?)
        } else {
            None
        };

        Err(RuntimeError::Return(value.unwrap()))
    }

    fn visit_break_stmt(&mut self) -> Result<(), RuntimeError> {
        Err(RuntimeError::Break)
    }

    fn visit_continue_stmt(&mut self) -> Result<(), RuntimeError> {
        Err(RuntimeError::Continue)
    }

    fn visit_class_stmt(&mut self, stmt: &mut ClassStmt) -> Result<(), RuntimeError> {
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme().into(), Object::Nil)?;

        let mut methods: HashMap<String, Function> = HashMap::new();
        for method in &mut stmt.methods {
            methods.insert(
                method.name.lexeme().into(),
                Function {
                    declaration: method.clone(),
                    closure: self.environment.clone(),
                },
            );
        }

        let class = ClassObject::new(stmt.name.lexeme(), methods);

        self.environment
            .borrow_mut()
            .assign(&stmt.name, &Object::Class(class))?;
        Ok(())
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
    pub fn interprete(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            match stmt.evaluate(self) {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }

        Ok(())
    }

    /// Creates a new interpreter with the given output destination.
    pub fn new_with_output(output: Rc<RefCell<dyn Write>>) -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals
            .borrow_mut()
            .define(
                "clock".into(),
                Object::Callable(CallableObject::ClockFn(ClockFn)),
            )
            .expect("Failed to define native function 'clock'");

        globals
            .borrow_mut()
            .define(
                "read".into(),
                Object::Callable(CallableObject::ReadFn(ReadFn)),
            )
            .expect("Failed to define native function 'read'");

        globals
            .borrow_mut()
            .define(
                "random".into(),
                Object::Callable(CallableObject::RandomFn(RandomFn)),
            )
            .expect("Failed to define native function 'random'");

        globals
            .borrow_mut()
            .define("sin".into(), Object::Callable(CallableObject::SinFn(SinFn)))
            .expect("Failed to define native function 'sin'");

        globals
            .borrow_mut()
            .define(
                "sqrt".into(),
                Object::Callable(CallableObject::SqrtFn(SqrtFn)),
            )
            .expect("Failed to define native function 'sqrt'");

        Interpreter {
            environment: globals.to_owned(),
            globals,
            locals: HashMap::new(),
            output,
        }
    }

    pub fn resolve(&mut self, expr: Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }

    fn look_up_variable(&mut self, name: &Token, expr: Expr) -> Result<Object, RuntimeError> {
        if let Some(distance) = self.locals.get(&expr) {
            self.environment
                .borrow()
                .get_at(distance, name.lexeme().to_owned())
        } else {
            self.globals.borrow().get(name)
        }
    }

    /// Default constructor, writes to stdout
    pub fn new() -> Self {
        Self::new_with_output(Rc::new(RefCell::new(std::io::stdout())))
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

#[cfg(test)]
mod tests {
    use crate::expr_types::*;
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::stmt_types::*;
    use crate::token::{Token, TokenType};

    fn create_number_token(value: f64, line: usize) -> Token {
        Token::new(
            TokenType::Number,
            &value.to_string(),
            Some(Object::Number(value)),
            line,
            0,
        )
    }

    fn create_string_token(value: String, line: usize) -> Token {
        Token::new(
            TokenType::String,
            &value.clone(),
            Some(Object::String(value)),
            line,
            0,
        )
    }

    fn create_literal_expr(token: Token) -> Expr {
        Expr::Literal(LiteralExpr { literal: token })
    }

    fn create_print_stmt(expr: Expr) -> Stmt {
        Stmt::Print(expr)
    }

    #[test]
    fn test_create_print_stmt() {
        let mut interpreter = Interpreter::new();
        let mut print_stmt = create_print_stmt(create_literal_expr(create_number_token(50.0, 1)));

        let result = print_stmt.evaluate(&mut interpreter);
        assert!(
            result.is_ok(),
            "Expected function to succeed, but got an error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_literal_number() {
        let mut interpreter = Interpreter::new();
        let number_token = create_number_token(42.0, 1);
        let mut expr = create_literal_expr(number_token);

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(42.0));
    }

    #[test]
    fn test_literal_string() {
        let mut interpreter = Interpreter::new();
        let string_token = create_string_token("hello".to_string(), 1);
        let mut expr = create_literal_expr(string_token);

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("hello".to_string()));
    }

    #[test]
    fn test_literal_boolean() {
        let mut interpreter = Interpreter::new();

        // Test true
        let true_token = Token::new(TokenType::True, "true", Some(Object::Boolean(true)), 1, 0);
        let mut expr = create_literal_expr(true_token);
        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test false
        let false_token = Token::new(
            TokenType::False,
            "false",
            Some(Object::Boolean(false)),
            1,
            0,
        );
        let mut expr = create_literal_expr(false_token);
        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(false));
    }

    #[test]
    fn test_literal_nil() {
        let mut interpreter = Interpreter::new();
        let nil_token = Token::new(TokenType::Nil, "nil", Some(Object::Nil), 1, 0);
        let mut expr = create_literal_expr(nil_token);

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Nil);
    }

    #[test]
    fn test_grouping_expr() {
        let mut interpreter = Interpreter::new();
        let number_token = create_number_token(42.0, 1);
        let number_expr = create_literal_expr(number_token);

        let left_paren = Token::new(TokenType::LeftParen, "(", None, 1, 0);
        let right_paren = Token::new(TokenType::RightParen, ")", None, 1, 0);
        let mut expr = Expr::Grouping(GroupingExpr {
            _paren_open: left_paren,
            expr: Box::new(number_expr),
            _paren_close: right_paren,
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(42.0));
    }

    #[test]
    fn test_unary_minus() {
        let mut interpreter = Interpreter::new();
        let number_token = create_number_token(42.0, 1);
        let number_expr = create_literal_expr(number_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1, 0);
        let mut expr = Expr::Unary(UnaryExpr {
            prefix: minus_token,
            operator: Box::new(number_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(-42.0));
    }

    #[test]
    fn test_unary_bang() {
        let mut interpreter = Interpreter::new();

        // Test !true -> false
        let true_token = Token::new(TokenType::True, "true", Some(Object::Boolean(true)), 1, 0);
        let true_expr = create_literal_expr(true_token);
        let bang_token = Token::new(TokenType::Bang, "!", None, 1, 0);
        let mut expr = Expr::Unary(UnaryExpr {
            prefix: bang_token.clone(),
            operator: Box::new(true_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(false));

        // Test !false -> true
        let false_token = Token::new(
            TokenType::False,
            "false",
            Some(Object::Boolean(false)),
            1,
            0,
        );
        let false_expr = create_literal_expr(false_token);
        let mut expr = Expr::Unary(UnaryExpr {
            prefix: bang_token,
            operator: Box::new(false_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_binary_arithmetic() {
        let mut interpreter = Interpreter::new();

        // Test 5 + 3 = 8
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let plus_token = Token::new(TokenType::Plus, "+", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: plus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(8.0));
    }

    #[test]
    fn test_binary_subtraction() {
        let mut interpreter = Interpreter::new();

        // Test 5 - 3 = 2
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let minus_token = Token::new(TokenType::Minus, "-", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: minus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(2.0));
    }

    #[test]
    fn test_binary_multiplication() {
        let mut interpreter = Interpreter::new();

        // Test 5 * 3 = 15
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let asterisk_token = Token::new(TokenType::Asterisk, "*", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: asterisk_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(15.0));
    }

    #[test]
    fn test_binary_division() {
        let mut interpreter = Interpreter::new();

        // Test 6 / 3 = 2
        let left_token = create_number_token(6.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let slash_token = Token::new(TokenType::Slash, "/", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: slash_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(2.0));
    }

    #[test]
    fn test_division_by_zero() {
        let mut interpreter = Interpreter::new();

        // Test 5 / 0 = error
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_number_token(0.0, 1);
        let right_expr = create_literal_expr(right_token);
        let slash_token = Token::new(TokenType::Slash, "/", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: slash_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_concatenation() {
        let mut interpreter = Interpreter::new();

        // Test "hello" + " world" = "hello world"
        let left_token = create_string_token("hello".to_string(), 1);
        let left_expr = create_literal_expr(left_token);
        let right_token = create_string_token(" world".to_string(), 1);
        let right_expr = create_literal_expr(right_token);
        let plus_token = Token::new(TokenType::Plus, "+", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: plus_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("hello world".to_string()));
    }

    #[test]
    fn test_binary_comparison() {
        let mut interpreter = Interpreter::new();

        // Test 5 > 3 = true
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token.clone());
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token.clone());

        let greater_token = Token::new(TokenType::Greater, ">", None, 1, 0);
        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: greater_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test 3 < 5 = true
        let left_expr = create_literal_expr(right_token);
        let right_expr = create_literal_expr(left_token);
        let less_token = Token::new(TokenType::Less, "<", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: less_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_equality() {
        let mut interpreter = Interpreter::new();

        // Test 5 == 5 = true
        let left_token = create_number_token(5.0, 1);
        let left_expr = create_literal_expr(left_token.clone());
        let right_token = create_number_token(5.0, 1);
        let right_expr = create_literal_expr(right_token);

        let eq_token = Token::new(TokenType::EqualEqual, "==", None, 1, 0);
        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: eq_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));

        // Test 5 != 3 = true
        let right_token = create_number_token(3.0, 1);
        let right_expr = create_literal_expr(right_token);
        let left_expr = create_literal_expr(left_token);
        let neq_token = Token::new(TokenType::BangEqual, "!=", None, 1, 0);

        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(left_expr),
            operator: neq_token,
            right: Box::new(right_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
    }

    #[test]
    fn test_complex_expression() {
        let mut interpreter = Interpreter::new();

        // Test (5 + 3) * 2 = 16
        let five_token = create_number_token(5.0, 1);
        let five_expr = create_literal_expr(five_token);
        let three_token = create_number_token(3.0, 1);
        let three_expr = create_literal_expr(three_token);
        let two_token = create_number_token(2.0, 1);
        let two_expr = create_literal_expr(two_token);

        let plus_token = Token::new(TokenType::Plus, "+", None, 1, 0);
        let addition = Expr::Binary(BinaryExpr {
            left: Box::new(five_expr),
            operator: plus_token,
            right: Box::new(three_expr),
        });

        let left_paren = Token::new(TokenType::LeftParen, "(", None, 1, 0);
        let right_paren = Token::new(TokenType::RightParen, "(", None, 1, 0);
        let grouping = Expr::Grouping(GroupingExpr {
            _paren_open: left_paren,
            expr: Box::new(addition),
            _paren_close: right_paren,
        });

        let asterisk_token = Token::new(TokenType::Asterisk, "*", None, 1, 0);
        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(grouping),
            operator: asterisk_token,
            right: Box::new(two_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(16.0));
    }

    #[test]
    fn test_type_errors() {
        let mut interpreter = Interpreter::new();

        // Test "string" - 5 (type error)
        let string_token = create_string_token("string".to_string(), 1);
        let string_expr = create_literal_expr(string_token);
        let number_token = create_number_token(5.0, 1);
        let number_expr = create_literal_expr(number_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1, 0);
        let mut expr = Expr::Binary(BinaryExpr {
            left: Box::new(string_expr),
            operator: minus_token,
            right: Box::new(number_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_err());

        // Test -"string" (type error)
        let string_token = create_string_token("string".to_string(), 1);
        let string_expr = create_literal_expr(string_token);

        let minus_token = Token::new(TokenType::Minus, "-", None, 1, 0);
        let mut expr = Expr::Unary(UnaryExpr {
            prefix: minus_token,
            operator: Box::new(string_expr),
        });

        let result = expr.accept(&mut interpreter);
        assert!(result.is_err());
    }
}
