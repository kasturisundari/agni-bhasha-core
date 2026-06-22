// Kasturi WASM Engine (Agni Bhasha -> WebAssembly Compiler)
// Translates the Agni AST directly into WebAssembly instructions for near-native execution speed.

use crate::parser::ast::{Statement, Expression};

pub struct WasmCompiler {
    wasm_instructions: Vec<String>,
    memory_offset: u32,
}

impl WasmCompiler {
    pub fn new() -> Self {
        WasmCompiler {
            wasm_instructions: Vec::new(),
            memory_offset: 0,
        }
    }

    /// Entry point to compile an entire Agni Bhasha Smart Contract
    pub fn compile_contract(&mut self, statements: &[Statement]) -> String {
        self.wasm_instructions.push("(module".to_string());
        
        // Setup Memory (1 page = 64KB)
        self.wasm_instructions.push("  (memory $0 1)".to_string());
        self.wasm_instructions.push("  (export \"memory\" (memory $0))".to_string());

        // Process all statements
        for stmt in statements {
            self.compile_statement(stmt);
        }

        self.wasm_instructions.push(")".to_string());
        
        self.wasm_instructions.join("\n")
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::FunctionDecl(name, _params, body) => {
                // Translate Agni Sutra to WASM Func
                self.wasm_instructions.push(format!("  (func $export_{} (export \"{}\")", name, name));
                
                for body_stmt in body {
                    self.compile_statement(body_stmt);
                }
                
                self.wasm_instructions.push("  )".to_string());
            }
            Statement::Let(name, expr) => {
                // Compile the expression and store it
                self.compile_expression(expr);
                self.wasm_instructions.push(format!("    ;; Store {}", name));
            }
            _ => {
                self.wasm_instructions.push("    ;; Unimplemented Agni Bhasha statement to WASM".to_string());
            }
        }
    }

    fn compile_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Integer(val) => {
                self.wasm_instructions.push(format!("    i32.const {}", val));
            }
            Expression::Float(val) => {
                self.wasm_instructions.push(format!("    f64.const {}", val));
            }
            Expression::Binary(left, op, right) => {
                self.compile_expression(left);
                self.compile_expression(right);
                
                match op.as_str() {
                    "+" => self.wasm_instructions.push("    i32.add".to_string()),
                    "-" => self.wasm_instructions.push("    i32.sub".to_string()),
                    "*" => self.wasm_instructions.push("    i32.mul".to_string()),
                    "/" => self.wasm_instructions.push("    i32.div_s".to_string()),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
