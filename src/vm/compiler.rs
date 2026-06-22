use super::opcode::OpCode;
use crate::parser::ast::{Expr, Statement, Program, Literal, BinaryOp};
use crate::evaluator::Value;

/// Compiles an AST into Vedic Bytecode
pub struct BytecodeCompiler {
    pub instructions: Vec<OpCode>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.compile_statement(stmt);
        }
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr);
                self.instructions.push(OpCode::Pop); // Discard result if not assigned
            }
            Statement::Assignment { name, value } => {
                self.compile_expr(value);
                self.instructions.push(OpCode::StoreLocal(name.clone()));
            }
            Statement::Return(expr) => {
                self.compile_expr(expr);
                self.instructions.push(OpCode::Return);
            }
            // For MVP, skip complex AST nodes or map them conceptually
            _ => {}
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(lit) => {
                let val = match lit {
                    Literal::Integer(n) => Value::Integer(*n),
                    Literal::Float(n) => Value::Float(*n),
                    Literal::Str(s) => Value::Str(s.clone()),
                    Literal::Shunya => Value::Shunya,
                    _ => Value::Shunya,
                };
                self.instructions.push(OpCode::Push(val));
            }
            Expr::Identifier(name) => {
                self.instructions.push(OpCode::LoadLocal(name.clone()));
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match operator {
                    BinaryOp::Add => self.instructions.push(OpCode::Add),
                    BinaryOp::Subtract => self.instructions.push(OpCode::Sub),
                    BinaryOp::Multiply => self.instructions.push(OpCode::Mul),
                    BinaryOp::Divide => self.instructions.push(OpCode::Div),
                    BinaryOp::Equal => self.instructions.push(OpCode::Equal),
                    _ => {}
                }
            }
            Expr::Dhatu(d) => {
                for arg in &d.params {
                    self.compile_expr(arg);
                }
                self.instructions.push(OpCode::InvokeDhatu {
                    root: d.root.clone(),
                    suffix: d.suffix.clone().unwrap_or_else(|| "ti".into()),
                    arg_count: d.params.len(),
                });
            }
            _ => {
                self.instructions.push(OpCode::Push(Value::Shunya));
            }
        }
    }

    // ═══════════════════════════════════════════
    // Sabha DAO Contract Compilation Extensions
    // ═══════════════════════════════════════════

    /// Compile a contract class (अधिकार) declaration into bytecode.
    /// Handles state variables, functions, and cross-contract references.
    pub fn compile_contract(&mut self, contract_name: &str, program: &Program) {
        // Emit contract initialization event
        self.instructions.push(OpCode::EmitEvent {
            event_name: format!("ContractDeployed:{}", contract_name),
            arg_count: 0,
        });

        for stmt in &program.statements {
            self.compile_contract_statement(contract_name, stmt);
        }
    }

    fn compile_contract_statement(&mut self, contract_id: &str, stmt: &Statement) {
        match stmt {
            Statement::Assignment { name, value } => {
                // Check Strilinga immutability constraint
                if name.ends_with('ā') || name.ends_with('ī') {
                    self.instructions.push(OpCode::LinganushasanamGuard {
                        variable_name: name.clone(),
                    });
                }

                self.compile_expr(value);
                self.instructions.push(OpCode::ContractStore {
                    contract_id: contract_id.to_string(),
                    key: name.clone(),
                });
            }
            Statement::Expression(expr) => {
                // Handle cross-contract invocations (√invoke+ति)
                if let Expr::Dhatu(d) = expr {
                    if d.root == "invoke" {
                        self.compile_cross_call(d);
                        return;
                    }
                }
                self.compile_expr(expr);
                self.instructions.push(OpCode::Pop);
            }
            Statement::Return(expr) => {
                self.compile_expr(expr);
                self.instructions.push(OpCode::Return);
            }
            _ => {}
        }
    }

    /// Compile a cross-contract call (√invoke+ति·TargetContract·function(args))
    fn compile_cross_call(&mut self, d: &crate::parser::ast::DhatuCall) {
        // First param is the target contract, second is the function
        let target = d.params.get(0)
            .map(|p| match p { Expr::Identifier(s) => s.clone(), _ => "Unknown".into() })
            .unwrap_or_else(|| "Unknown".into());
        let function = d.params.get(1)
            .map(|p| match p { Expr::Identifier(s) => s.clone(), _ => "Unknown".into() })
            .unwrap_or_else(|| "Unknown".into());

        // Compile remaining args
        for arg in d.params.iter().skip(2) {
            self.compile_expr(arg);
        }

        self.instructions.push(OpCode::CrossCall {
            target_contract: target,
            function_name: function,
            arg_count: d.params.len().saturating_sub(2),
        });
    }

    /// Emit a governance event (proposal, vote, reward, etc.)
    pub fn emit_event(&mut self, event_name: &str, args: &[&Expr]) {
        for arg in args {
            self.compile_expr(arg);
        }
        self.instructions.push(OpCode::EmitEvent {
            event_name: event_name.to_string(),
            arg_count: args.len(),
        });
    }

    /// Compile authorization check (√parīkṣ+तुम्)
    pub fn compile_auth_check(&mut self, role: &str) {
        self.instructions.push(OpCode::RequireAuth(role.to_string()));
    }
}
