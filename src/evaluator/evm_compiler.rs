use crate::parser::ast::{Statement, Expr, BinaryOp, Literal, UnaryOp};

/// EVM Solidity Compiler for Agni Bhasha
/// This takes an Agni Bhasha AST and outputs an EVM-Compatible Solidity (.sol) source code.
pub struct AgniEvmCompiler {
    solidity_code: String,
    indent_level: usize,
}

impl AgniEvmCompiler {
    pub fn new() -> Self {
        Self {
            solidity_code: "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract AgniContract {\n".to_string(),
            indent_level: 1,
        }
    }

    pub fn compile(&mut self, program: &[Statement]) -> String {
        for stmt in program {
            self.compile_statement(stmt);
        }
        self.solidity_code.push_str("}\n");
        self.solidity_code.clone()
    }

    fn push_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.solidity_code.push_str("    ");
        }
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::SutraDefinition { name, params, body } => {
                self.push_indent();
                self.solidity_code.push_str(&format!("function {}(", self.sanitize(name)));
                
                // Solidity requires types. Since Agni is dynamic, we default to uint256 for EVM
                let params_sol: Vec<String> = params.iter().map(|p| format!("uint256 {}", self.sanitize(p))).collect();
                self.solidity_code.push_str(&params_sol.join(", "));
                
                self.solidity_code.push_str(") public returns (uint256) {\n");
                self.indent_level += 1;
                
                for b in body {
                    self.compile_statement(b);
                }
                
                self.indent_level -= 1;
                self.push_indent();
                self.solidity_code.push_str("}\n\n");
            }
            Statement::Assignment { name, value } => {
                self.push_indent();
                self.solidity_code.push_str(&format!("uint256 {} = ", self.sanitize(name)));
                self.compile_expr(value);
                self.solidity_code.push_str(";\n");
            }
            Statement::Return(expr) => {
                self.push_indent();
                self.solidity_code.push_str("return ");
                self.compile_expr(expr);
                self.solidity_code.push_str(";\n");
            }
            Statement::IfElse { condition, then_branch, else_branch } => {
                self.push_indent();
                self.solidity_code.push_str("if (");
                self.compile_expr(condition);
                self.solidity_code.push_str(") {\n");
                
                self.indent_level += 1;
                for b in then_branch {
                    self.compile_statement(b);
                }
                self.indent_level -= 1;
                self.push_indent();
                self.solidity_code.push_str("}");
                
                if let Some(else_b) = else_branch {
                    self.solidity_code.push_str(" else {\n");
                    self.indent_level += 1;
                    for b in else_b {
                        self.compile_statement(b);
                    }
                    self.indent_level -= 1;
                    self.push_indent();
                    self.solidity_code.push_str("}\n");
                } else {
                    self.solidity_code.push_str("\n");
                }
            }
            Statement::WhileLoop { condition, body } => {
                self.push_indent();
                self.solidity_code.push_str("while (");
                self.compile_expr(condition);
                self.solidity_code.push_str(") {\n");
                
                self.indent_level += 1;
                for b in body {
                    self.compile_statement(b);
                }
                self.indent_level -= 1;
                self.push_indent();
                self.solidity_code.push_str("}\n");
            }
            Statement::Expression(Expr::Dhatu(crate::parser::ast::DhatuExpr { root, params, .. })) if root == "vac" || root == "वच्" => {
                // EVM Solidity doesn't have a direct print, we emit an event
                // This is a simplified mapping for "recognition" of EVM by Agni Bhasha
                self.push_indent();
                self.solidity_code.push_str("// emit Vachana(");
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { self.solidity_code.push_str(", "); }
                    self.compile_expr(p);
                }
                self.solidity_code.push_str(");\n");
            }
            Statement::Expression(expr) => {
                self.push_indent();
                self.compile_expr(expr);
                self.solidity_code.push_str(";\n");
            }
            _ => {
                self.push_indent();
                self.solidity_code.push_str("// Unsupported AST Node for EVM Cross-compilation\n");
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(Literal::Integer(i)) => {
                self.solidity_code.push_str(&i.to_string());
            }
            Expr::Literal(Literal::Str(s)) => {
                self.solidity_code.push_str(&format!("\"{}\"", s));
            }
            Expr::Literal(Literal::Tattva(t)) => {
                match t {
                    crate::evaluator::TattvaState::Sat => self.solidity_code.push_str("1"),
                    crate::evaluator::TattvaState::Asat => self.solidity_code.push_str("0"),
                    _ => self.solidity_code.push_str("0"),
                }
            }
            Expr::Identifier(name) => {
                self.solidity_code.push_str(&self.sanitize(name));
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(left);
                match operator {
                    BinaryOp::Add => self.solidity_code.push_str(" + "),
                    BinaryOp::Subtract => self.solidity_code.push_str(" - "),
                    BinaryOp::Multiply => self.solidity_code.push_str(" * "),
                    BinaryOp::Divide => self.solidity_code.push_str(" / "),
                    BinaryOp::Modulo => self.solidity_code.push_str(" % "),
                    BinaryOp::Equal => self.solidity_code.push_str(" == "),
                    BinaryOp::NotEqual => self.solidity_code.push_str(" != "),
                    BinaryOp::Less => self.solidity_code.push_str(" < "),
                    BinaryOp::LessEqual => self.solidity_code.push_str(" <= "),
                    BinaryOp::Greater => self.solidity_code.push_str(" > "),
                    BinaryOp::GreaterEqual => self.solidity_code.push_str(" >= "),
                    BinaryOp::And => self.solidity_code.push_str(" && "),
                    BinaryOp::Or => self.solidity_code.push_str(" || "),
                    _ => self.solidity_code.push_str(" /* unknown op */ "),
                }
                self.compile_expr(right);
            }
            Expr::Unary { operator, operand } => {
                match operator {
                    UnaryOp::Negate => self.solidity_code.push_str("-"),
                    UnaryOp::Not => self.solidity_code.push_str("!"),
                }
                self.compile_expr(operand);
            }
            Expr::Call { callee, args } => {
                self.compile_expr(callee);
                self.solidity_code.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.solidity_code.push_str(", "); }
                    self.compile_expr(arg);
                }
                self.solidity_code.push_str(")");
            }
            _ => {
                self.solidity_code.push_str("0 /* unsupported expr */");
            }
        }
    }

    fn sanitize(&self, name: &str) -> String {
        // Agni uses Sanskrit unicode natively, Solidity allows unicode identifiers in 0.8+
        // But for safety across older EVM tools, we might need a transliterator.
        // For now, we trust Solidity 0.8's native unicode support.
        name.to_string()
    }
}
