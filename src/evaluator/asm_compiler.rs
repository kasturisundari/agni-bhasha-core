use crate::parser::ast::{Statement, Expr, BinaryOp, Literal};

/// Bare-Metal x86_64 NASM Compiler for Agni Bhasha (Sutra)
pub struct AgniAsmCompiler {
    data_section: String,
    bss_section: String,
    text_section: String,
    string_counter: usize,
}

impl AgniAsmCompiler {
    pub fn new() -> Self {
        Self {
            data_section: "section .data\n".to_string(),
            bss_section: "section .bss\n".to_string(),
            text_section: "section .text\n    global _start\n\n_start:\n".to_string(),
            string_counter: 0,
        }
    }

    pub fn compile(&mut self, program: &[Statement]) -> String {
        for stmt in program {
            self.compile_statement(stmt);
        }

        // Exit syscall
        self.text_section.push_str("    ; Exit Process\n");
        self.text_section.push_str("    mov rax, 60\n");
        self.text_section.push_str("    xor rdi, rdi\n");
        self.text_section.push_str("    syscall\n");

        format!("{}\n{}\n{}", self.data_section, self.bss_section, self.text_section)
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assignment { name, value: expr } => {
                // Reserve 8 bytes for a 64-bit variable
                self.bss_section.push_str(&format!("    var_{} resq 1\n", self.sanitize(name)));
                self.compile_expr(expr);
                // Result of expr is in rax, store it in the variable
                self.text_section.push_str(&format!("    mov [var_{}], rax\n", self.sanitize(name)));
            }
            Statement::Expression(Expr::Dhatu(crate::parser::ast::DhatuExpr { root, params, .. })) if root == "vac" || root == "वच्" => {
                for expr in params {
                    if let Expr::Literal(Literal::Str(s)) = expr {
                        let label = format!("str_{}", self.string_counter);
                        self.string_counter += 1;
                        self.data_section.push_str(&format!("    {} db \"{}\", 10, 0\n", label, s));
                        let len = s.len() + 1; // +1 for newline
                        
                        self.text_section.push_str(&format!("    ; Print '{}'\n", s));
                        self.text_section.push_str("    mov rax, 1\n"); // sys_write
                        self.text_section.push_str("    mov rdi, 1\n"); // stdout
                        self.text_section.push_str(&format!("    mov rsi, {}\n", label));
                        self.text_section.push_str(&format!("    mov rdx, {}\n", len));
                        self.text_section.push_str("    syscall\n");
                    }
                }
            }
            Statement::Expression(expr) => {
                self.compile_expr(expr);
            }
            _ => {
                // Return, Function, If, While handled here in advanced versions
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(Literal::Integer(i)) => {
                self.text_section.push_str(&format!("    mov rax, {}\n", i));
            }
            Expr::Identifier(name) => {
                self.text_section.push_str(&format!("    mov rax, [var_{}]\n", self.sanitize(name)));
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(right);
                self.text_section.push_str("    push rax\n");
                self.compile_expr(left);
                self.text_section.push_str("    pop rbx\n");
                
                match operator {
                    BinaryOp::Add => self.text_section.push_str("    add rax, rbx\n"),
                    BinaryOp::Subtract => self.text_section.push_str("    sub rax, rbx\n"),
                    BinaryOp::Multiply => self.text_section.push_str("    imul rax, rbx\n"),
                    BinaryOp::Divide => {
                        self.text_section.push_str("    cqo\n"); // sign extend rax into rdx
                        self.text_section.push_str("    idiv rbx\n");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn sanitize(&self, name: &str) -> String {
        // Simple sanitization for ASM labels, replacing unsupported chars
        name.replace(" ", "_")
    }
}
