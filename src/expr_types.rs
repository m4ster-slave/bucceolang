use crate::runtime_error::RuntimeError;
use crate::Token;

/// Represents the different types of expressions in the abstract syntax tree (AST).
///
/// The `Expr` enum defines the nodes of the expression tree generated by the parser.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Represents a literal value (e.g., number, string, boolean, nil).
    Literal(LiteralExpr),
    /// Represents a parenthesized expression used for grouping.
    Grouping(GroupingExpr),
    /// Represents a unary operation (e.g., negation, logical NOT).
    Unary(UnaryExpr),
    /// Represents a binary operation (e.g., addition, subtraction, comparison).
    Binary(BinaryExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
    /// Represents a logical operation (and, or).
    Logical(LogicalExpr),
    Call(CallExpr),
    PropertyAccess(PropertyAccessExpr),
    PropertyAssignment(PropertyAssignmentExpr),
    This(ThisExpr),
    Super(SuperExpr),
}

/// Defines the visitor trait for traversing the `Expr` abstract syntax tree.
///
/// Types that implement this trait can visit each type of expression node in the AST.
/// This is a key component of the Visitor pattern, used for operations like
/// interpretation or static analysis.
///
/// The type parameter `T` represents the return type of the visitor methods.
pub trait ExprVisitor<T> {
    /// Visits a `LiteralExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `LiteralExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<T, RuntimeError>;

    /// Visits a `GroupingExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `GroupingExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_grouping_expr(&mut self, expr: &mut GroupingExpr) -> Result<T, RuntimeError>;

    /// Visits a `UnaryExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `UnaryExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_unary_expr(&mut self, expr: &mut UnaryExpr) -> Result<T, RuntimeError>;

    /// Visits a `BinaryExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `BinaryExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_binary_expr(&mut self, expr: &mut BinaryExpr) -> Result<T, RuntimeError>;

    /// Visits a `VariableExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `VariableExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<T, RuntimeError>;

    /// Visits a `VariableExpr` node.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `VariableExpr` node to visit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the visitor's result or a `RuntimeError`.
    fn visit_assign_expr(&mut self, expr: &mut AssignExpr) -> Result<T, RuntimeError>;

    fn visit_logical_expr(&mut self, expr: &mut LogicalExpr) -> Result<T, RuntimeError>;
    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> Result<T, RuntimeError>;
    fn visit_property_access_expr(
        &mut self,
        expr: &mut PropertyAccessExpr,
    ) -> Result<T, RuntimeError>;
    fn visit_property_assignment_expr(
        &mut self,
        expr: &mut PropertyAssignmentExpr,
    ) -> Result<T, RuntimeError>;
    fn visit_this_expr(&mut self, expr: &mut ThisExpr) -> Result<T, RuntimeError>;
    fn visit_super_expr(&mut self, expr: &mut SuperExpr) -> Result<T, RuntimeError>;
}

impl Expr {
    /// Accepts a visitor and dispatches the call to the appropriate `visit` method
    /// based on the specific type of the `Expr`.
    ///
    /// This method is the entry point for applying the Visitor pattern to an `Expr` node.
    ///
    /// # Arguments
    ///
    /// * `visitor` - A reference to an object implementing the `ExprVisitor` trait.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The return type of the visitor methods.
    ///
    /// # Returns
    ///
    /// A `Result` containing the result of the visitor's operation or a `RuntimeError`.
    pub fn accept<T>(&mut self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
            Expr::Call(expr) => visitor.visit_call_expr(expr),
            Expr::PropertyAccess(expr) => visitor.visit_property_access_expr(expr),
            Expr::PropertyAssignment(expr) => visitor.visit_property_assignment_expr(expr),
            Expr::This(expr) => visitor.visit_this_expr(expr),
            Expr::Super(expr) => visitor.visit_super_expr(expr),
        }
    }
}

/// Represents a literal value expression in the AST.
#[derive(Debug, Clone)]
pub struct LiteralExpr {
    /// The token representing the literal value.
    pub literal: Token,
}

/// Represents a grouping expression (parenthesized expression) in the AST.
#[derive(Debug, Clone)]
pub struct GroupingExpr {
    /// The left parenthesis token.
    pub _paren_open: Token,
    /// The expression contained within the parentheses.
    pub expr: Box<Expr>,
    /// The right parenthesis token.
    pub _paren_close: Token,
}

/// Represents a unary expression (e.g., `-5`, `!is_true`) in the AST.
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    /// The operator token (e.g., `-`, `!`).
    pub prefix: Token,
    /// The operand of the unary operation.
    pub operator: Box<Expr>, // Renamed from `operator` to `operand` for clarity
}

/// Represents a binary expression (e.g., `a + b`, `x > y`) in the AST.
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    /// The left-hand side operand of the binary operation.
    pub left: Box<Expr>,
    /// The operator token (e.g., `+`, `-`, `*`, `/`, `==`, `!=`).
    pub operator: Token,
    /// The right-hand side operand of the binary operation.
    pub right: Box<Expr>,
}

/// Wrapper around the variable name token
#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct PropertyAccessExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct PropertyAssignmentExpr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ThisExpr {
    pub keyword: Token,
}

#[derive(Debug, Clone)]
pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
}

use std::hash::{Hash, Hasher};

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Literal(a), Literal(b)) => a.literal.literal() == b.literal.literal(),
            (Grouping(a), Grouping(b)) => a.expr == b.expr,
            (Unary(a), Unary(b)) => {
                a.prefix.lexeme() == b.prefix.lexeme() && a.operator == b.operator
            }
            (Binary(a), Binary(b)) => {
                a.left == b.left && a.right == b.right && a.operator.lexeme() == b.operator.lexeme()
            }
            (Variable(a), Variable(b)) => {
                a.name.literal() == b.name.literal()
                    && a.name.token_number() == b.name.token_number()
            }
            (Assign(a), Assign(b)) => a.name.literal() == b.name.literal() && a.value == b.value,
            (Logical(a), Logical(b)) => {
                a.left == b.left && a.right == b.right && a.operator.lexeme() == b.operator.lexeme()
            }
            (Call(a), Call(b)) => {
                a.callee == b.callee
                    && a.arguments
                        .iter()
                        .zip(b.arguments.clone())
                        .all(|(a_e, b_e)| *a_e == b_e)
            }
            (PropertyAssignment(a), PropertyAssignment(b)) => a.name.lexeme() == b.name.lexeme(),
            (PropertyAccess(a), PropertyAccess(b)) => a.name.lexeme() == b.name.lexeme(),
            (This(a), This(b)) => {
                a.keyword.token_number() == b.keyword.token_number()
                    && a.keyword.line() == b.keyword.line()
                    && a.keyword.lexeme() == b.keyword.lexeme()
            }
            (Super(a), Super(b)) => {
                a.keyword.token_number() == b.keyword.token_number()
                    && a.keyword.line() == b.keyword.line()
                    && a.keyword.lexeme() == b.keyword.lexeme()
                    && a.method.line() == b.method.line()
                    && a.method.lexeme() == b.method.lexeme()
                    && a.method.token_number() == b.method.token_number()
            }
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash the discriminant first to distinguish between variants
        std::mem::discriminant(self).hash(state);

        // then hash the actual data based on what PartialEq compares
        match self {
            Expr::Literal(expr) => {
                expr.literal.lexeme().hash(state);
            }
            Expr::Grouping(expr) => {
                expr.expr.hash(state);
            }
            Expr::Unary(expr) => {
                expr.prefix.lexeme().hash(state);
                expr.operator.hash(state);
            }
            Expr::Binary(expr) => {
                expr.left.hash(state);
                expr.right.hash(state);
                expr.operator.lexeme().hash(state);
            }
            Expr::Variable(expr) => {
                expr.name.lexeme().hash(state);
                expr.name.line().hash(state);
                expr.name.token_number().hash(state);
            }
            Expr::Assign(expr) => {
                expr.name.lexeme().hash(state);
            }
            Expr::Logical(expr) => {
                expr.left.hash(state);
                expr.right.hash(state);
                expr.operator.lexeme().hash(state);
            }
            Expr::Call(expr) => {
                expr.callee.hash(state);
                for arg in &expr.arguments {
                    arg.hash(state);
                }
            }
            Expr::PropertyAccess(expr) => {
                expr.name.token_number().hash(state);
                expr.name.line().hash(state);
                expr.name.lexeme().hash(state);
                expr.object.hash(state);
            }
            Expr::PropertyAssignment(expr) => {
                expr.name.token_number().hash(state);
                expr.name.line().hash(state);
                expr.name.lexeme().hash(state);
                expr.object.hash(state);
                expr.value.hash(state);
            }
            Expr::This(expr) => {
                expr.keyword.token_number().hash(state);
                expr.keyword.line().hash(state);
                expr.keyword.lexeme().hash(state);
            }
            Expr::Super(expr) => {
                expr.keyword.token_number().hash(state);
                expr.keyword.line().hash(state);
                expr.keyword.lexeme().hash(state);
                expr.method.token_number().hash(state);
                expr.method.line().hash(state);
                expr.method.lexeme().hash(state);
            }
        }
    }
}
