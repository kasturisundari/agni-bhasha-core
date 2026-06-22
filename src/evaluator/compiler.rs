use crate::parser::ast::{Expr, Statement, Program, BinaryOp, Literal, UnaryOp};
use std::collections::HashMap;

/// Kasturi Virtual Machine Opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    /// Load a literal value onto the stack
    LoadLiteral(Literal),
    /// Load a variable from environment
    LoadVar(String),
    /// Store top of stack into variable
    StoreVar(String),
    /// Add top two stack elements
    Add,
    /// Subtract top from second top
    Sub,
    /// Multiply top two stack elements
    Mul,
    /// Divide second top by top
    Div,
    /// Jump relative if top of stack is false
    JumpIfFalse(usize),
    /// Unconditional jump relative
    Jump(usize),
    /// Unconditional jump backward relative
    JumpBack(usize),
    /// Call function with N arguments
    Call(String, usize),
    /// Return from function
    Return,
    /// Native Dhatu Call
    InvokeDhatu(String, usize), // Dhatu name, num params
    /// End of program
    Halt,
}

pub struct BytecodeCompiler {
    pub instructions: Vec<Opcode>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.compile_statement(stmt)?;
        }
        self.instructions.push(Opcode::Halt);
        Ok(())
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr)?;
            }
            Statement::Assignment { name, value } => {
                self.compile_expr(value)?;
                self.instructions.push(Opcode::StoreVar(name.clone()));
            }
            Statement::IfElse { condition, then_branch, else_branch } => {
                self.compile_expr(condition)?;
                
                let jump_if_false_idx = self.instructions.len();
                self.instructions.push(Opcode::JumpIfFalse(0)); // Placeholder

                for s in then_branch {
                    self.compile_statement(s)?;
                }

                if let Some(else_b) = else_branch {
                    let jump_idx = self.instructions.len();
                    self.instructions.push(Opcode::Jump(0)); // Placeholder

                    // Patch JumpIfFalse
                    let offset = self.instructions.len() - jump_if_false_idx - 1;
                    self.instructions[jump_if_false_idx] = Opcode::JumpIfFalse(offset);

                    for s in else_b {
                        self.compile_statement(s)?;
                    }

                    // Patch Jump
                    let offset2 = self.instructions.len() - jump_idx - 1;
                    self.instructions[jump_idx] = Opcode::Jump(offset2);
                } else {
                    // Patch JumpIfFalse
                    let offset = self.instructions.len() - jump_if_false_idx - 1;
                    self.instructions[jump_if_false_idx] = Opcode::JumpIfFalse(offset);
                }
            }
            Statement::Return(expr) => {
                self.compile_expr(expr)?;
                self.instructions.push(Opcode::Return);
            }
            Statement::WhileLoop { condition, body } => {
                let loop_start = self.instructions.len();
                
                self.compile_expr(condition)?;
                
                let jump_if_false_idx = self.instructions.len();
                self.instructions.push(Opcode::JumpIfFalse(0)); // Placeholder
                
                for s in body {
                    self.compile_statement(s)?;
                }
                
                // Jump back to loop start
                let jump_back_offset = self.instructions.len() - loop_start + 1;
                self.instructions.push(Opcode::JumpBack(jump_back_offset));
                
                // Patch exit jump
                let exit_offset = self.instructions.len() - jump_if_false_idx - 1;
                self.instructions[jump_if_false_idx] = Opcode::JumpIfFalse(exit_offset);
            }
            // Other statements are deferred for the fully complete VM
            _ => {
                // Not fully implemented in Bytecode compiler yet
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Literal(lit) => {
                self.instructions.push(Opcode::LoadLiteral(lit.clone()));
            }
            Expr::Identifier(name) => {
                self.instructions.push(Opcode::LoadVar(name.clone()));
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                match operator {
                    BinaryOp::Add => self.instructions.push(Opcode::Add),
                    BinaryOp::Subtract => self.instructions.push(Opcode::Sub),
                    BinaryOp::Multiply => self.instructions.push(Opcode::Mul),
                    BinaryOp::Divide => self.instructions.push(Opcode::Div),
                    _ => return Err(format!("Unsupported binary operator in compiler: {:?}", operator)),
                }
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                if let Expr::Identifier(name) = &**callee {
                    self.instructions.push(Opcode::Call(name.clone(), args.len()));
                } else {
                    return Err("Only direct function calls are supported in bytecode yet".to_string());
                }
            }
            _ => {
                // Not implemented
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_literal_int() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Literal(Literal::Integer(42));
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions.len(), 1);
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Integer(42)));
    }

    #[test]
    fn test_compile_literal_float() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Literal(Literal::Float(3.14));
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Float(3.14)));
    }

    #[test]
    fn test_compile_literal_bool() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Literal(Literal::Boolean(true));
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Boolean(true)));
    }

    #[test]
    fn test_compile_literal_str() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Literal(Literal::Str("om".to_string()));
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Str("om".to_string())));
    }

    #[test]
    fn test_compile_identifier() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Identifier("x".to_string());
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[0], Opcode::LoadVar("x".to_string()));
    }

    #[test]
    fn test_compile_binary_add() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(1))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions.len(), 3);
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Integer(1)));
        assert_eq!(compiler.instructions[1], Opcode::LoadLiteral(Literal::Integer(2)));
        assert_eq!(compiler.instructions[2], Opcode::Add);
    }

    #[test]
    fn test_compile_binary_sub() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Subtract,
            right: Box::new(Expr::Literal(Literal::Integer(5))),
        };
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[2], Opcode::Sub);
    }

    #[test]
    fn test_compile_binary_mul() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(3))),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Literal(Literal::Integer(4))),
        };
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[2], Opcode::Mul);
    }

    #[test]
    fn test_compile_binary_div() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Divide,
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        compiler.compile_expr(&expr).unwrap();
        assert_eq!(compiler.instructions[2], Opcode::Div);
    }

    #[test]
    fn test_compile_assignment() {
        let mut compiler = BytecodeCompiler::new();
        let stmt = Statement::Assignment {
            name: "var1".to_string(),
            value: Box::new(Expr::Literal(Literal::Integer(100))),
        };
        compiler.compile_statement(&stmt).unwrap();
        assert_eq!(compiler.instructions.len(), 2);
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Integer(100)));
        assert_eq!(compiler.instructions[1], Opcode::StoreVar("var1".to_string()));
    }

    #[test]
    fn test_compile_if_else() {
        let mut compiler = BytecodeCompiler::new();
        let stmt = Statement::IfElse {
            condition: Expr::Literal(Literal::Boolean(true)),
            then_branch: vec![Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Literal(Literal::Integer(1))),
            }],
            else_branch: Some(vec![Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Literal(Literal::Integer(2))),
            }]),
        };
        compiler.compile_statement(&stmt).unwrap();
        // Check structure
        // 0: LoadLiteral(true)
        // 1: JumpIfFalse(offset to else)
        // 2: LoadLiteral(1)
        // 3: StoreVar(x)
        // 4: Jump(offset to end)
        // 5: LoadLiteral(2)
        // 6: StoreVar(x)
        assert_eq!(compiler.instructions.len(), 7);
        assert_eq!(compiler.instructions[1], Opcode::JumpIfFalse(3));
        assert_eq!(compiler.instructions[4], Opcode::Jump(2));
    }

    #[test]
    fn test_compile_if_no_else() {
        let mut compiler = BytecodeCompiler::new();
        let stmt = Statement::IfElse {
            condition: Expr::Literal(Literal::Boolean(true)),
            then_branch: vec![Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Literal(Literal::Integer(1))),
            }],
            else_branch: None,
        };
        compiler.compile_statement(&stmt).unwrap();
        // 0: LoadLiteral(true)
        // 1: JumpIfFalse(offset to end)
        // 2: LoadLiteral(1)
        // 3: StoreVar(x)
        assert_eq!(compiler.instructions.len(), 4);
        assert_eq!(compiler.instructions[1], Opcode::JumpIfFalse(2));
    }

    #[test]
    fn test_compile_return() {
        let mut compiler = BytecodeCompiler::new();
        let stmt = Statement::Return(Expr::Literal(Literal::Integer(42)));
        compiler.compile_statement(&stmt).unwrap();
        assert_eq!(compiler.instructions.len(), 2);
        assert_eq!(compiler.instructions[0], Opcode::LoadLiteral(Literal::Integer(42)));
        assert_eq!(compiler.instructions[1], Opcode::Return);
    }

    #[test]
    fn test_compile_program() {
        let mut compiler = BytecodeCompiler::new();
        let program = Program {
            statements: vec![
                Statement::Assignment {
                    name: "y".to_string(),
                    value: Box::new(Expr::Literal(Literal::Integer(10))),
                }
            ],
        };
        compiler.compile(&program).unwrap();
        // Load, Store, Halt
        assert_eq!(compiler.instructions.len(), 3);
        assert_eq!(compiler.instructions.last().unwrap(), &Opcode::Halt);
    }

    #[test]
    fn test_unsupported_binary_op() {
        let mut compiler = BytecodeCompiler::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(1))),
            operator: BinaryOp::And, // not supported in basic compiler yet
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        assert!(compiler.compile_expr(&expr).is_err());
    }
}
