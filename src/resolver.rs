use std::collections::HashMap;

use crate::{
    expr_types::VariableExpr, expr_types::*, runtime_error::RuntimeError, stmt_types::*,
    token::Token, Interpreter,
};

/// The `Resolver` is responsible for performing static analysis on the AST to resolve variable scopes and ensure correct variable usage before interpretation.
///
/// It tracks variable declarations and definitions in nested scopes using a stack of hash maps, and communicates scope distances to the interpreter for efficient variable lookup at runtime.
///
/// The resolver also detects errors such as reading a variable in its own initializer and helps enforce language scoping rules.
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    /// we use this vector as stack here with push() and pop()
    ///
    /// The value associated with a key in the scope map represents
    /// whether or not we have finished resolving that variable’s initializer.
    scopes: Vec<HashMap<String, bool>>,
    loop_depth: usize,
}

impl Resolver<'_> {
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            loop_depth: 0,
        }
    }
}

impl StmtVisitor<()> for Resolver<'_> {
    fn visit_block_stmt(&mut self, stmt: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
        self.begin_scope()?;
        self.resolve_stmts(stmt)?;
        self.end_scope()?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &mut VarStmt) -> Result<(), RuntimeError> {
        self.declare(&stmt.name, false)?;

        if let Some(initializer) = &mut stmt.initializer {
            self.resolve_expr(initializer)?;
        }

        self.declare(&stmt.name, true)?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &mut FunctionStmt) -> Result<(), RuntimeError> {
        self.declare(&stmt.name, true)?;
        self.resolve_function(stmt)?;
        Ok(())
    }

    fn visit_expr_stmt(&mut self, stmt: &mut Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(stmt)
    }

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut stmt.condition)?;
        self.resolve_stmt(&mut stmt.then_branch)?;
        if let Some(else_branch) = &mut stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &mut Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(stmt)
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt) -> Result<(), RuntimeError> {
        if let Some(ret) = &mut stmt.value {
            self.resolve_expr(ret)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut stmt.condition)?;
        self.loop_depth += 1;
        self.resolve_stmt(&mut stmt.body)?;
        self.loop_depth -= 1;
        Ok(())
    }

    fn visit_continue_stmt(&mut self) -> Result<(), RuntimeError> {
        if self.loop_depth == 0 {
            Err(RuntimeError::Other(
                0,
                "Cannot use 'continue' outside of a loop.".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn visit_break_stmt(&mut self) -> Result<(), RuntimeError> {
        if self.loop_depth == 0 {
            Err(RuntimeError::Other(
                0,
                "Cannot use 'break' outside of a loop.".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl ExprVisitor<()> for Resolver<'_> {
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<(), RuntimeError> {
        if self
            .scopes
            .last_mut()
            .and_then(|scope| scope.get(expr.name.lexeme()))
            .is_some_and(|defined| !*defined)
        {
            return Err(RuntimeError::Resolver(
                expr.name.line(),
                "Can't read local variable in its own initializer.".to_owned(),
            ));
        }

        self.resolve_local(Expr::Variable(expr.clone()), &expr.name)?;
        Ok(())
    }

    fn visit_assign_expr(&mut self, expr: &mut AssignExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.value)?;
        self.resolve_local(Expr::Assign(expr.clone()), &expr.name)?;
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &mut BinaryExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.left)?;
        self.resolve_expr(&mut expr.right)
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.callee)?;

        for arg in &mut expr.arguments {
            self.resolve_expr(arg)?;
        }

        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: &mut GroupingExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.expr)
    }

    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &mut LogicalExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.left)?;
        self.resolve_expr(&mut expr.right)
    }

    fn visit_unary_expr(&mut self, expr: &mut UnaryExpr) -> Result<(), RuntimeError> {
        self.resolve_expr(&mut expr.operator)
    }
}

impl Resolver<'_> {
    pub fn resolve(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
        self.resolve_stmts(stmts)
    }
    fn resolve_stmts(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            stmt.evaluate(self)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RuntimeError> {
        stmt.evaluate(self)?;
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> Result<(), RuntimeError> {
        expr.accept(self)
    }

    /// Begins a new variable scope by pushing a new HashMap onto the scope stack.
    fn begin_scope(&mut self) -> Result<(), RuntimeError> {
        self.scopes.push(HashMap::new());
        Ok(())
    }

    /// Ends the current variable scope by popping the last HashMap from the scope stack.
    fn end_scope(&mut self) -> Result<(), RuntimeError> {
        self.scopes.pop();
        Ok(())
    }

    /// Declares a variable in the current scope.
    ///
    /// # Arguments
    /// * `name` - The token representing the variable name.
    /// * `defined` - Whether the variable has been fully defined (initialized).
    fn declare(&mut self, name: &Token, defined: bool) -> Result<(), RuntimeError> {
        match self.scopes.last_mut() {
            // last_mut() only ever returns none if the Vec is empty
            None => (),
            Some(scope) => {
                scope.insert(name.lexeme().to_owned(), defined);
            }
        };
        Ok(())
    }

    /// Resolves a variable reference to its scope distance and informs the interpreter.
    ///
    /// # Arguments
    /// * `expr` - The expression referencing the variable.
    /// * `name` - The token representing the variable name.
    fn resolve_local(&mut self, expr: Expr, name: &Token) -> Result<(), RuntimeError> {
        if let Some(distance) = self
            .scopes
            .iter()
            .rev()
            .position(|scope| scope.contains_key(name.lexeme()))
        {
            // distance == 0 for the innermost scope, 1 for the next, etc.
            self.interpreter.resolve(expr, distance);
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: &mut FunctionStmt) -> Result<(), RuntimeError> {
        self.begin_scope()?;
        for param in function.params.iter() {
            self.declare(param, false)?;
            self.declare(param, true)?;
        }
        self.begin_scope()?;
        self.resolve_stmts(&mut function.body)?;
        self.end_scope()?;
        self.end_scope()
    }
}
