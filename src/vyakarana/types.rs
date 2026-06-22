use crate::parser::ast::{Expr, Literal, Statement, BinaryOp, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SutraType {
    Integer,
    Float,
    String,
    Boolean,
    Tattva,
    List(Box<SutraType>),
    Map(Box<SutraType>, Box<SutraType>),
    Shunya, // Null/Void
    Any,    // For gradual typing or uninferable types
}

impl std::fmt::Display for SutraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SutraType::Integer => write!(f, "Integer"),
            SutraType::Float => write!(f, "Float"),
            SutraType::String => write!(f, "String"),
            SutraType::Boolean => write!(f, "Boolean"),
            SutraType::Tattva => write!(f, "Tattva"),
            SutraType::List(inner) => write!(f, "List<{}>", inner),
            SutraType::Map(k, v) => write!(f, "Map<{}, {}>", k, v),
            SutraType::Shunya => write!(f, "Shunya"),
            SutraType::Any => write!(f, "Any"),
        }
    }
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, SutraType>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define_var(&mut self, name: &str, ty: SutraType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    pub fn get_var(&self, name: &str) -> SutraType {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return ty.clone();
            }
        }
        SutraType::Any
    }

    pub fn check_program(&mut self, program: &crate::parser::ast::Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Assignment { name, value } => {
                let val_ty = self.check_expr(value)?;
                self.define_var(name, val_ty);
            }
            Statement::Expression(expr) => {
                self.check_expr(expr)?;
            }
            Statement::SutraDefinition { name, params, body } => {
                // In a fully typed language, params would have types. For now we assume Any.
                self.enter_scope();
                for param in params {
                    self.define_var(param, SutraType::Any);
                }
                for s in body {
                    self.check_statement(s)?;
                }
                self.exit_scope();
                // Define the sutra itself in the outer scope
                self.define_var(name, SutraType::Any);
            }
            Statement::IfElse { condition, then_branch, else_branch } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != SutraType::Boolean && cond_ty != SutraType::Tattva && cond_ty != SutraType::Any {
                    return Err(format!("Type error: Condition must be Boolean or Tattva, found {}", cond_ty));
                }
                self.enter_scope();
                for s in then_branch {
                    self.check_statement(s)?;
                }
                self.exit_scope();

                if let Some(else_b) = else_branch {
                    self.enter_scope();
                    for s in else_b {
                        self.check_statement(s)?;
                    }
                    self.exit_scope();
                }
            }
            Statement::Return(expr) => {
                self.check_expr(expr)?;
            }
            Statement::Import(path) => {
                // In a static checker, we would read the imported file, parse it, and check its types
                // But for now, we just skip it or assume it adds Any types.
                // We'll let the engine do the dynamic check later.
            }
            Statement::Adhikara { context, body } | Statement::Prakarana { context, body } => {
                self.check_expr(context)?;
                self.enter_scope();
                for s in body {
                    self.check_statement(s)?;
                }
                self.exit_scope();
            }
            Statement::ForEach { item, collection, body } => {
                let coll_ty = self.check_expr(collection)?;
                self.enter_scope();
                let item_ty = match coll_ty {
                    SutraType::List(inner) => *inner,
                    SutraType::String => SutraType::String,
                    SutraType::Any => SutraType::Any,
                    _ => return Err(format!("Type error: Cannot iterate over {}", coll_ty)),
                };
                self.define_var(item, item_ty);
                self.define_var("◈", SutraType::Any); // Current element loop variable
                for s in body {
                    self.check_statement(s)?;
                }
                self.exit_scope();
            }
            Statement::WhileLoop { condition, body } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != SutraType::Boolean && cond_ty != SutraType::Tattva && cond_ty != SutraType::Any {
                    return Err(format!("Type error: While condition must be Boolean or Tattva, found {}", cond_ty));
                }
                self.enter_scope();
                for s in body {
                    self.check_statement(s)?;
                }
                self.exit_scope();
            }
            _ => {
                // Not all statements are fully checked yet
            }
        }
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<SutraType, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Integer(_) => Ok(SutraType::Integer),
                Literal::Float(_) => Ok(SutraType::Float),
                Literal::Str(_) => Ok(SutraType::String),
                Literal::Tattva(_) => Ok(SutraType::Tattva),
                Literal::Shunya => Ok(SutraType::Shunya),
            },
            Expr::Identifier(name) => Ok(self.get_var(name)),
            Expr::Binary { left, operator, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;
                
                if left_ty == SutraType::Any || right_ty == SutraType::Any {
                    return Ok(SutraType::Any);
                }

                match operator {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => {
                        if (left_ty == SutraType::Integer || left_ty == SutraType::Float) &&
                           (right_ty == SutraType::Integer || right_ty == SutraType::Float) {
                            if left_ty == SutraType::Float || right_ty == SutraType::Float {
                                Ok(SutraType::Float)
                            } else {
                                Ok(SutraType::Integer)
                            }
                        } else {
                            Err(format!("Type error: Cannot perform arithmetic on {} and {}", left_ty, right_ty))
                        }
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        Ok(SutraType::Boolean)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        Ok(SutraType::Boolean)
                    }
                    BinaryOp::Join => {
                        Ok(SutraType::String) // string concatenation
                    }
                }
            }
            Expr::Unary { operator, operand } => {
                let op_ty = self.check_expr(operand)?;
                if op_ty == SutraType::Any {
                    return Ok(SutraType::Any);
                }
                match operator {
                    UnaryOp::Negate => {
                        if op_ty == SutraType::Integer || op_ty == SutraType::Float {
                            Ok(op_ty)
                        } else {
                            Err(format!("Type error: Cannot negate {}", op_ty))
                        }
                    }
                    UnaryOp::Not => {
                        Ok(SutraType::Boolean)
                    }
                }
            }
            Expr::Dict(pairs) => {
                let mut key_ty = SutraType::Shunya;
                let mut val_ty = SutraType::Shunya;
                
                for (k, v) in pairs {
                    let k_t = self.check_expr(k)?;
                    let v_t = self.check_expr(v)?;
                    
                    if key_ty == SutraType::Shunya {
                        key_ty = k_t;
                        val_ty = v_t;
                    } else if key_ty != SutraType::Any && key_ty != k_t {
                        key_ty = SutraType::Any;
                    }
                }
                Ok(SutraType::Map(Box::new(key_ty), Box::new(val_ty)))
            }
            Expr::Call { callee, args } => {
                // Return Any for now, unless we can resolve the function type signature
                for arg in args {
                    self.check_expr(arg)?;
                }
                Ok(SutraType::Any)
            }
            Expr::Dhatu(_) => Ok(SutraType::Any), // Native root calls
            _ => Ok(SutraType::Any),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Literal, Expr, BinaryOp, UnaryOp};

    #[test]
    fn test_typechecker_new_scope() {
        let mut tc = TypeChecker::new();
        tc.define_var("x", SutraType::Integer);
        assert_eq!(tc.get_var("x"), SutraType::Integer);

        tc.enter_scope();
        tc.define_var("x", SutraType::String);
        assert_eq!(tc.get_var("x"), SutraType::String);

        tc.exit_scope();
        assert_eq!(tc.get_var("x"), SutraType::Integer);
    }

    #[test]
    fn test_typechecker_undefined_var() {
        let tc = TypeChecker::new();
        assert_eq!(tc.get_var("unknown"), SutraType::Any);
    }

    #[test]
    fn test_check_literal_integer() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Literal(Literal::Integer(42));
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Integer);
    }

    #[test]
    fn test_check_literal_float() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Literal(Literal::Float(3.14));
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Float);
    }

    #[test]
    fn test_check_literal_string() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Literal(Literal::Str("hello".to_string()));
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::String);
    }

    #[test]
    fn test_check_binary_add_integers() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Integer(20))),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Integer);
    }

    #[test]
    fn test_check_binary_add_floats() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Float(10.0))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Float(20.0))),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Float);
    }

    #[test]
    fn test_check_binary_add_mixed() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Float(20.0))),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Float);
    }

    #[test]
    fn test_check_binary_type_mismatch() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Str("hello".to_string()))),
        };
        assert!(tc.check_expr(&expr).is_err());
    }

    #[test]
    fn test_check_comparison_returns_boolean() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Greater,
            right: Box::new(Expr::Literal(Literal::Integer(5))),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Boolean);
    }

    #[test]
    fn test_check_logical_returns_boolean() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: BinaryOp::And,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Boolean);
    }

    #[test]
    fn test_check_unary_negate_integer() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Unary {
            operator: UnaryOp::Negate,
            operand: Box::new(Expr::Literal(Literal::Integer(42))),
        };
        assert_eq!(tc.check_expr(&expr).unwrap(), SutraType::Integer);
    }

    #[test]
    fn test_check_unary_negate_string_fails() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Unary {
            operator: UnaryOp::Negate,
            operand: Box::new(Expr::Literal(Literal::Str("error".to_string()))),
        };
        assert!(tc.check_expr(&expr).is_err());
    }

    #[test]
    fn test_check_dict_inference() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Dict(vec![
            (Expr::Literal(Literal::Str("key".to_string())), Expr::Literal(Literal::Integer(1))),
        ]);
        let ty = tc.check_expr(&expr).unwrap();
        assert_eq!(ty, SutraType::Map(Box::new(SutraType::String), Box::new(SutraType::Integer)));
    }

    #[test]
    fn test_check_dict_any_inference() {
        let mut tc = TypeChecker::new();
        let expr = Expr::Dict(vec![
            (Expr::Literal(Literal::Str("key".to_string())), Expr::Literal(Literal::Integer(1))),
            (Expr::Literal(Literal::Integer(42)), Expr::Literal(Literal::Integer(2))),
        ]);
        let ty = tc.check_expr(&expr).unwrap();
        assert_eq!(ty, SutraType::Map(Box::new(SutraType::Any), Box::new(SutraType::Integer)));
    }
}
