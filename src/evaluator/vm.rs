use crate::evaluator::{Value, TattvaState};
use crate::evaluator::compiler::Opcode;
use std::collections::HashMap;

pub struct KasturiVM {
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
}

impl KasturiVM {
    pub fn new() -> Self {
        KasturiVM {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, instructions: &[Opcode]) -> Result<Value, String> {
        let mut pc = 0;
        
        // --- THE IMMORTAL NETWORK PATCH: Gas Limits ---
        // Maximum instructions a contract can execute before Halting (Time-Freeze Virus Protection)
        const MAX_GAS: usize = 10_000;
        let mut gas_used = 0;

        while pc < instructions.len() {
            if gas_used >= MAX_GAS {
                return Err("CRITICAL: Out of Gas! Infinite loop or heavy computation detected.".to_string());
            }
            gas_used += 1;
            
            let op = &instructions[pc];
            pc += 1;

            match op {
                Opcode::LoadLiteral(lit) => {
                    let val = match lit.clone() {
                        crate::parser::ast::Literal::Integer(i) => Value::Integer(i),
                        crate::parser::ast::Literal::Float(f) => Value::Float(f),
                        crate::parser::ast::Literal::Str(s) => Value::Str(s),
                        crate::parser::ast::Literal::Tattva(t) => Value::Tattva(t),
                        crate::parser::ast::Literal::Shunya => Value::Shunya,
                    };
                    self.stack.push(val);
                }
                Opcode::LoadVar(name) => {
                    if let Some(val) = self.globals.get(name) {
                        self.stack.push(val.clone());
                    } else {
                        return Err(format!("Undefined variable: {}", name));
                    }
                }
                Opcode::StoreVar(name) => {
                    if let Some(val) = self.stack.pop() {
                        self.globals.insert(name.clone(), val);
                    } else {
                        return Err("Stack underflow on StoreVar".to_string());
                    }
                }
                Opcode::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x + y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                        _ => return Err("Invalid types for Add".to_string()),
                    }
                }
                Opcode::Sub => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        _ => return Err("Invalid types for Sub".to_string()),
                    }
                }
                Opcode::Mul => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        _ => return Err("Invalid types for Mul".to_string()),
                    }
                }
                Opcode::Div => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => {
                            if y == 0 { return Err("Division by zero".to_string()); }
                            self.stack.push(Value::Integer(x / y))
                        },
                        (Value::Float(x), Value::Float(y)) => {
                            if y == 0.0 { return Err("Division by zero".to_string()); }
                            self.stack.push(Value::Float(x / y))
                        },
                        _ => return Err("Invalid types for Div".to_string()),
                    }
                }
                Opcode::Jump(offset) => {
                    // --- THE APOCALYPSE PATCH: Bytecode Jump Overflow Protection ---
                    // Prevent Rust Thread Panic (DoS) caused by jumping beyond maximum usize
                    pc = pc.saturating_add(*offset);
                    if pc > instructions.len() {
                        return Err("CRITICAL: Opcode::Jump exceeded instruction bounds. Contract execution halted.".to_string());
                    }
                }
                Opcode::JumpIfFalse(offset) => {
                    let val = self.stack.pop().ok_or("Stack underflow")?;
                    
                    // --- THE IMMORTAL NETWORK PATCH: Strict Logic Evaluation ---
                    // Prevent "The Oracle Illusion" where non-Tattva types bypass logic gates
                    let is_false = match val {
                        Value::Tattva(TattvaState::Sadasat) => {
                            return Err("CRITICAL SECURITY: Ambiguous 'Sadasat' (True/False) state encountered in boolean control flow. Execution halted to prevent logic bypass.".to_string());
                        },
                        Value::Tattva(t) => t == TattvaState::Asat || t == TattvaState::Avyaktam,
                        Value::Integer(i) => i == 0,
                        Value::Float(f) => f == 0.0,
                        Value::Str(s) => s.is_empty(),
                        Value::Shunya => true, // Shunya (Null) is always Falsy
                        _ => false, // Anything else is truthy
                    };
                    
                    if is_false {
                        // --- THE APOCALYPSE PATCH: Bytecode Jump Overflow Protection ---
                        pc = pc.saturating_add(*offset);
                        if pc > instructions.len() {
                            return Err("CRITICAL: Opcode::JumpIfFalse exceeded instruction bounds. Contract execution halted.".to_string());
                        }
                    }
                }
                Opcode::Return => {
                    if let Some(val) = self.stack.pop() {
                        return Ok(val);
                    } else {
                        return Ok(Value::Shunya);
                    }
                }
                Opcode::Halt => {
                    break;
                }
                _ => return Err(format!("Unimplemented opcode: {:?}", op)),
            }
        }

        if let Some(val) = self.stack.pop() {
            Ok(val)
        } else {
            Ok(Value::Shunya)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Literal;

    #[test]
    fn test_vm_load_literal() {
        let mut vm = KasturiVM::new();
        let ops = vec![Opcode::LoadLiteral(Literal::Integer(42)), Opcode::Halt];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(42));
    }

    #[test]
    fn test_vm_addition() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(10)),
            Opcode::LoadLiteral(Literal::Integer(20)),
            Opcode::Add,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(30));
    }

    #[test]
    fn test_vm_subtraction() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(50)),
            Opcode::LoadLiteral(Literal::Integer(15)),
            Opcode::Sub,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(35));
    }

    #[test]
    fn test_vm_multiplication() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(6)),
            Opcode::LoadLiteral(Literal::Integer(7)),
            Opcode::Mul,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(42));
    }

    #[test]
    fn test_vm_division() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(100)),
            Opcode::LoadLiteral(Literal::Integer(4)),
            Opcode::Div,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(25));
    }

    #[test]
    fn test_vm_division_by_zero() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(10)),
            Opcode::LoadLiteral(Literal::Integer(0)),
            Opcode::Div,
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_vm_store_and_load_var() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(99)),
            Opcode::StoreVar("x".to_string()),
            Opcode::LoadVar("x".to_string()),
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(99));
        assert_eq!(vm.globals.get("x").unwrap(), &Value::Integer(99));
    }

    #[test]
    fn test_vm_load_undefined_var() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadVar("y".to_string()),
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_jump() {
        let mut vm = KasturiVM::new();
        // Load 10, Jump over Load 20
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(10)),
            Opcode::Jump(1),
            Opcode::LoadLiteral(Literal::Integer(20)),
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(10)); // Top of stack should still be 10, 20 was skipped
    }

    #[test]
    fn test_vm_jump_if_false_taken() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Tattva(TattvaState::Asat)),
            Opcode::JumpIfFalse(2),
            Opcode::LoadLiteral(Literal::Integer(100)), // Skipped
            Opcode::Halt,
            Opcode::LoadLiteral(Literal::Integer(200)), // Executed
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(200));
    }

    #[test]
    fn test_vm_jump_if_false_not_taken() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Tattva(TattvaState::Sat)),
            Opcode::JumpIfFalse(2),
            Opcode::LoadLiteral(Literal::Integer(100)), // Executed
            Opcode::Halt,
            Opcode::LoadLiteral(Literal::Integer(200)), // Unreachable
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(100));
    }

    #[test]
    fn test_vm_float_addition() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Float(1.5)),
            Opcode::LoadLiteral(Literal::Float(2.5)),
            Opcode::Add,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        if let Value::Float(f) = res {
            assert!((f - 4.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected float");
        }
    }

    #[test]
    fn test_vm_type_mismatch() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(10)),
            Opcode::LoadLiteral(Literal::Float(2.5)),
            Opcode::Add,
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_return() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(77)),
            Opcode::Return,
            Opcode::LoadLiteral(Literal::Integer(88)),
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Integer(77));
    }

    #[test]
    fn test_vm_empty_stack_return() {
        let mut vm = KasturiVM::new();
        let ops = vec![Opcode::Return];
        let res = vm.run(&ops).unwrap();
        assert_eq!(res, Value::Shunya);
    }

    #[test]
    fn test_vm_stack_underflow_add() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Integer(10)),
            Opcode::Add,
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Stack underflow");
    }

    #[test]
    fn test_vm_stack_underflow_sub() {
        let mut vm = KasturiVM::new();
        let ops = vec![Opcode::Sub, Opcode::Halt];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_stack_underflow_mul() {
        let mut vm = KasturiVM::new();
        let ops = vec![Opcode::Mul, Opcode::Halt];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_stack_underflow_div() {
        let mut vm = KasturiVM::new();
        let ops = vec![Opcode::Div, Opcode::Halt];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_unimplemented_opcode() {
        let mut vm = KasturiVM::new();
        // InvokeDhatu is not implemented in run() yet
        let ops = vec![Opcode::InvokeDhatu("test".to_string(), 0), Opcode::Halt];
        let res = vm.run(&ops);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Unimplemented opcode"));
    }

    #[test]
    fn test_vm_store_var_underflow() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::StoreVar("x".to_string()),
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Stack underflow on StoreVar");
    }

    #[test]
    fn test_vm_jump_if_false_underflow() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::JumpIfFalse(1),
            Opcode::Halt
        ];
        let res = vm.run(&ops);
        assert!(res.is_err());
    }

    #[test]
    fn test_vm_float_subtraction() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Float(5.5)),
            Opcode::LoadLiteral(Literal::Float(2.5)),
            Opcode::Sub,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        if let Value::Float(f) = res {
            assert!((f - 3.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected float");
        }
    }

    #[test]
    fn test_vm_float_multiplication() {
        let mut vm = KasturiVM::new();
        let ops = vec![
            Opcode::LoadLiteral(Literal::Float(2.0)),
            Opcode::LoadLiteral(Literal::Float(3.5)),
            Opcode::Mul,
            Opcode::Halt
        ];
        let res = vm.run(&ops).unwrap();
        if let Value::Float(f) = res {
            assert!((f - 7.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected float");
        }
    }
}
