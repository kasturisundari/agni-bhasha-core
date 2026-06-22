use super::opcode::OpCode;
use crate::evaluator::Value;
use std::collections::HashMap;

/// Kasturi Stack-Based Virtual Machine
pub struct KasturiVM {
    pub stack: Vec<Value>,
    pub locals: HashMap<String, Value>,
    pub pc: usize, // Program Counter
    /// Contract storage (persisted across calls)
    pub contract_storage: HashMap<String, HashMap<String, Value>>,
    /// Event log
    pub events: Vec<(String, Vec<Value>)>,
}

impl KasturiVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: HashMap::new(),
            pc: 0,
            contract_storage: HashMap::new(),
            events: Vec::new(),
        }
    }

    /// Execute a sequence of Vedic Bytecode instructions
    pub fn execute(&mut self, instructions: &[OpCode]) -> Result<Value, String> {
        self.pc = 0;

        while self.pc < instructions.len() {
            let op = instructions[self.pc].clone();
            self.pc += 1;

            match op {
                OpCode::Push(val) => {
                    self.stack.push(val);
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::StoreLocal(name) => {
                    if let Some(val) = self.stack.pop() {
                        self.locals.insert(name, val);
                    } else {
                        return Err("Stack Underflow on StoreLocal".into());
                    }
                }
                OpCode::LoadLocal(name) => {
                    if let Some(val) = self.locals.get(&name) {
                        self.stack.push(val.clone());
                    } else {
                        self.stack.push(Value::Shunya);
                    }
                }

                // ═══════ Arithmetic ═══════
                OpCode::Add => {
                    let (a, b) = self.pop_two()?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x + y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                        (Value::Integer(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 + y)),
                        (Value::Float(x), Value::Integer(y)) => self.stack.push(Value::Float(x + y as f64)),
                        (Value::Str(x), Value::Str(y)) => self.stack.push(Value::Str(format!("{}{}", x, y))),
                        _ => return Err("Type Error in Add".into()),
                    }
                }
                OpCode::Sub => {
                    let (a, b) = self.pop_two()?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        (Value::Integer(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 - y)),
                        (Value::Float(x), Value::Integer(y)) => self.stack.push(Value::Float(x - y as f64)),
                        _ => return Err("Type Error in Sub".into()),
                    }
                }
                OpCode::Mul => {
                    let (a, b) = self.pop_two()?;
                    match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        (Value::Integer(x), Value::Float(y)) => self.stack.push(Value::Float(x as f64 * y)),
                        (Value::Float(x), Value::Integer(y)) => self.stack.push(Value::Float(x * y as f64)),
                        _ => return Err("Type Error in Mul".into()),
                    }
                }
                OpCode::Div => {
                    let (a, b) = self.pop_two()?;
                    match (a, b) {
                        (Value::Integer(_), Value::Integer(0)) => return Err("Division by zero".into()),
                        (Value::Integer(x), Value::Integer(y)) => self.stack.push(Value::Integer(x / y)),
                        (Value::Float(x), Value::Float(y)) => {
                            if y == 0.0 { return Err("Division by zero".into()); }
                            self.stack.push(Value::Float(x / y));
                        }
                        (Value::Integer(x), Value::Float(y)) => {
                            if y == 0.0 { return Err("Division by zero".into()); }
                            self.stack.push(Value::Float(x as f64 / y));
                        }
                        (Value::Float(x), Value::Integer(y)) => {
                            if y == 0 { return Err("Division by zero".into()); }
                            self.stack.push(Value::Float(x / y as f64));
                        }
                        _ => return Err("Type Error in Div".into()),
                    }
                }
                OpCode::Equal => {
                    let (a, b) = self.pop_two()?;
                    let result = match (&a, &b) {
                        (Value::Integer(x), Value::Integer(y)) => x == y,
                        (Value::Float(x), Value::Float(y)) => (x - y).abs() < f64::EPSILON,
                        (Value::Str(x), Value::Str(y)) => x == y,
                        (Value::Shunya, Value::Shunya) => true,
                        _ => false,
                    };
                    self.stack.push(Value::Tattva(if result {
                        crate::evaluator::TattvaState::Sat
                    } else {
                        crate::evaluator::TattvaState::Asat
                    }));
                }

                // ═══════ Control Flow ═══════
                OpCode::Jump(target) => {
                    if target >= instructions.len() {
                        return Err(format!("Jump target {} out of bounds", target));
                    }
                    self.pc = target;
                }
                OpCode::JumpBack(offset) => {
                    if offset > self.pc {
                        return Err(format!("JumpBack offset {} underflows pc {}", offset, self.pc));
                    }
                    self.pc -= offset;
                }
                OpCode::JumpIfTrue(target) => {
                    let val = self.stack.pop().unwrap_or(Value::Shunya);
                    if self.is_truthy(&val) {
                        if target >= instructions.len() {
                            return Err(format!("JumpIfTrue target {} out of bounds", target));
                        }
                        self.pc = target;
                    }
                }
                OpCode::JumpIfFalse(target) => {
                    let val = self.stack.pop().unwrap_or(Value::Shunya);
                    if !self.is_truthy(&val) {
                        if target >= instructions.len() {
                            return Err(format!("JumpIfFalse target {} out of bounds", target));
                        }
                        self.pc = target;
                    }
                }

                // ═══════ Dhatu Invocation (REAL EXECUTION) ═══════
                OpCode::InvokeDhatu { root, suffix, arg_count } => {
                    let mut args = Vec::new();
                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap_or(Value::Shunya));
                    }
                    args.reverse();

                    let result = self.execute_dhatu(&root, &suffix, &args);
                    self.stack.push(result);
                }

                OpCode::Call(name, arg_count) => {
                    let mut args = Vec::new();
                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap_or(Value::Shunya));
                    }
                    args.reverse();
                    // For now, push a placeholder — real function calls need a call stack
                    self.stack.push(Value::Str(format!("<call {}({})>", name, arg_count)));
                }

                // ═══════ Chatushkoti Logic ═══════
                OpCode::NyayaCheck => {
                    let val = self.stack.pop().unwrap_or(Value::Shunya);
                    let decision = crate::evaluator::chatushkoti::ChatushkotiEngine::evaluate_cognition(&val, 0.0);
                    match decision {
                        crate::evaluator::chatushkoti::CognitiveDecision::Sat(v) =>
                            self.stack.push(v),
                        crate::evaluator::chatushkoti::CognitiveDecision::Asat(_) =>
                            self.stack.push(Value::Shunya),
                        crate::evaluator::chatushkoti::CognitiveDecision::Sadasat(v) =>
                            self.stack.push(v),
                        crate::evaluator::chatushkoti::CognitiveDecision::Avyaktam =>
                            self.stack.push(Value::Shunya),
                    }
                }

                // ═══════ Contract Operations ═══════
                OpCode::ContractStore { contract_id, key } => {
                    if let Some(val) = self.stack.pop() {
                        let storage = self.contract_storage.entry(contract_id).or_insert_with(HashMap::new);
                        storage.insert(key, val);
                    }
                }
                OpCode::ContractLoad { contract_id, key } => {
                    let val = self.contract_storage
                        .get(&contract_id)
                        .and_then(|s| s.get(&key))
                        .cloned()
                        .unwrap_or(Value::Shunya);
                    self.stack.push(val);
                }
                OpCode::EmitEvent { event_name, arg_count } => {
                    let mut args = Vec::new();
                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap_or(Value::Shunya));
                    }
                    args.reverse();
                    self.events.push((event_name, args));
                }
                OpCode::CrossCall { target_contract, function_name, arg_count } => {
                    let mut args = Vec::new();
                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap_or(Value::Shunya));
                    }
                    args.reverse();
                    // Cross-contract calls are logged as events in the VM
                    self.events.push((
                        format!("CrossCall:{}:{}", target_contract, function_name),
                        args,
                    ));
                    self.stack.push(Value::Tattva(crate::evaluator::TattvaState::Sat));
                }
                OpCode::RequireAuth(role) => {
                    // In a real node, this checks msg.sender against authorized roles
                    // For the VM, we push Sat (authorized) if role is non-empty
                    if role.is_empty() {
                        return Err("Authorization required: no role specified".into());
                    }
                    // The auth check passes in VM simulation mode
                }
                OpCode::LinganushasanamGuard { variable_name } => {
                    // Check if variable has Strilinga suffix (ā/ī) and is being mutated
                    if variable_name.ends_with('ā') || variable_name.ends_with('ī') {
                        // In a real scenario, we check if this is an external mutation attempt
                        // For now, this guard is a no-op marker (blocks are enforced at compile time)
                    }
                }

                OpCode::Return => {
                    break;
                }
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::Shunya))
    }

    /// Pop two values from the stack (a = first pushed, b = second pushed)
    fn pop_two(&mut self) -> Result<(Value, Value), String> {
        let b = self.stack.pop().ok_or("Stack Underflow")?;
        let a = self.stack.pop().ok_or("Stack Underflow")?;
        Ok((a, b))
    }

    /// Check if a value is truthy
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Shunya => false,
            Value::Integer(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Tattva(t) => matches!(t,
                crate::evaluator::TattvaState::Sat | crate::evaluator::TattvaState::Sadasat
            ),
            Value::List(l) => !l.is_empty(),
            _ => true,
        }
    }

    /// Execute a Dhatu root with real logic (maps to builtin operations)
    fn execute_dhatu(&self, root: &str, suffix: &str, params: &[Value]) -> Value {
        match root {
            // √vac — print
            "वच्" | "vac" => {
                for p in params {
                    print!("{}", p);
                }
                if suffix != "णम्" { println!(); }
                Value::Shunya
            }
            // √sṛj — create/return first param
            "सृज्" | "srj" => {
                params.first().cloned().unwrap_or(Value::Shunya)
            }
            // √yuj — join/concatenate
            "युज्" | "yuj" => {
                if params.len() >= 2 {
                    match (&params[0], &params[1]) {
                        (Value::Str(a), Value::Str(b)) => Value::Str(format!("{}{}", a, b)),
                        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                        (Value::List(a), Value::List(b)) => {
                            let mut merged = a.clone();
                            merged.extend(b.iter().cloned());
                            Value::List(merged)
                        }
                        _ => Value::Shunya,
                    }
                } else {
                    Value::Shunya
                }
            }
            // √gaṇ — compute/math
            "गण्" | "gan" => {
                if params.len() >= 2 {
                    match (&params[0], &params[1]) {
                        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                        _ => Value::Shunya,
                    }
                } else {
                    params.first().cloned().unwrap_or(Value::Shunya)
                }
            }
            // √mā — measure/length
            "मा" | "ma" => {
                match params.first() {
                    Some(Value::Str(s)) => Value::Integer(s.len() as i64),
                    Some(Value::List(l)) => Value::Integer(l.len() as i64),
                    _ => Value::Integer(0),
                }
            }
            // √bhid — split
            "भिद्" | "bhid" => {
                if let Some(Value::Str(s)) = params.first() {
                    if let Some(Value::Str(sep)) = params.get(1) {
                        Value::List(s.split(sep.as_str()).map(|p| Value::Str(p.to_string())).collect())
                    } else {
                        Value::List(s.chars().map(|c| Value::Str(c.to_string())).collect())
                    }
                } else {
                    Value::Shunya
                }
            }
            // √kram — sort
            "क्रम्" | "kram" => {
                if let Some(Value::List(items)) = params.first() {
                    let mut sorted = items.clone();
                    sorted.sort_by(|a, b| match (a, b) {
                        (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                        (Value::Str(x), Value::Str(y)) => x.cmp(y),
                        _ => std::cmp::Ordering::Equal,
                    });
                    Value::List(sorted)
                } else {
                    Value::Shunya
                }
            }
            // Unknown root — return Shunya
            _ => Value::Shunya,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::opcode::OpCode;

    #[test]
    fn test_push_and_pop() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(42)),
        ]).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_add_integers() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(10)),
            OpCode::Push(Value::Integer(20)),
            OpCode::Add,
        ]).unwrap();
        assert_eq!(result, Value::Integer(30));
    }

    #[test]
    fn test_add_floats() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Float(1.5)),
            OpCode::Push(Value::Float(2.5)),
            OpCode::Add,
        ]).unwrap();
        assert_eq!(result, Value::Float(4.0));
    }

    #[test]
    fn test_sub_integers() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(50)),
            OpCode::Push(Value::Integer(20)),
            OpCode::Sub,
        ]).unwrap();
        assert_eq!(result, Value::Integer(30));
    }

    #[test]
    fn test_mul_integers() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(6)),
            OpCode::Push(Value::Integer(7)),
            OpCode::Mul,
        ]).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_div_integers() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(100)),
            OpCode::Push(Value::Integer(4)),
            OpCode::Div,
        ]).unwrap();
        assert_eq!(result, Value::Integer(25));
    }

    #[test]
    fn test_div_by_zero() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(42)),
            OpCode::Push(Value::Integer(0)),
            OpCode::Div,
        ]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_equal_true() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(42)),
            OpCode::Push(Value::Integer(42)),
            OpCode::Equal,
        ]).unwrap();
        assert!(matches!(result, Value::Tattva(crate::evaluator::TattvaState::Sat)));
    }

    #[test]
    fn test_equal_false() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(1)),
            OpCode::Push(Value::Integer(2)),
            OpCode::Equal,
        ]).unwrap();
        assert!(matches!(result, Value::Tattva(crate::evaluator::TattvaState::Asat)));
    }

    #[test]
    fn test_store_and_load_local() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(108)),
            OpCode::StoreLocal("x".into()),
            OpCode::LoadLocal("x".into()),
        ]).unwrap();
        assert_eq!(result, Value::Integer(108));
    }

    #[test]
    fn test_load_undefined_returns_shunya() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::LoadLocal("undefined_var".into()),
        ]).unwrap();
        assert_eq!(result, Value::Shunya);
    }

    #[test]
    fn test_jump() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(1)),  // 0: push 1
            OpCode::Jump(3),                   // 1: skip next
            OpCode::Push(Value::Integer(999)), // 2: should be skipped
            OpCode::Push(Value::Integer(2)),  // 3: push 2
            OpCode::Add,                       // 4: 1 + 2 = 3
        ]).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_jump_if_true() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(1)),   // truthy
            OpCode::JumpIfTrue(3),             // jump to index 3
            OpCode::Push(Value::Integer(999)), // skipped
            OpCode::Push(Value::Integer(42)),  // landed here
        ]).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_jump_if_false() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Shunya),       // falsy
            OpCode::JumpIfFalse(3),            // jump to index 3
            OpCode::Push(Value::Integer(999)), // skipped
            OpCode::Push(Value::Integer(7)),   // landed here
        ]).unwrap();
        assert_eq!(result, Value::Integer(7));
    }

    #[test]
    fn test_invoke_dhatu_yuj_strings() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Str("hello".into())),
            OpCode::Push(Value::Str(" world".into())),
            OpCode::InvokeDhatu {
                root: "युज्".into(),
                suffix: "ति".into(),
                arg_count: 2,
            },
        ]).unwrap();
        assert_eq!(result, Value::Str("hello world".into()));
    }

    #[test]
    fn test_invoke_dhatu_gan_multiply() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(6)),
            OpCode::Push(Value::Integer(9)),
            OpCode::InvokeDhatu {
                root: "गण्".into(),
                suffix: "ति".into(),
                arg_count: 2,
            },
        ]).unwrap();
        assert_eq!(result, Value::Integer(54));
    }

    #[test]
    fn test_invoke_dhatu_ma_length() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Str("kasturi".into())),
            OpCode::InvokeDhatu {
                root: "मा".into(),
                suffix: "ति".into(),
                arg_count: 1,
            },
        ]).unwrap();
        assert_eq!(result, Value::Integer(7));
    }

    #[test]
    fn test_contract_store_and_load() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(1000)),
            OpCode::ContractStore {
                contract_id: "SabhaDAO".into(),
                key: "total_supply".into(),
            },
            OpCode::ContractLoad {
                contract_id: "SabhaDAO".into(),
                key: "total_supply".into(),
            },
        ]).unwrap();
        assert_eq!(result, Value::Integer(1000));
    }

    #[test]
    fn test_emit_event() {
        let mut vm = KasturiVM::new();
        vm.execute(&[
            OpCode::Push(Value::Str("alice".into())),
            OpCode::Push(Value::Integer(100)),
            OpCode::EmitEvent {
                event_name: "Transfer".into(),
                arg_count: 2,
            },
        ]).unwrap();
        assert_eq!(vm.events.len(), 1);
        assert_eq!(vm.events[0].0, "Transfer");
        assert_eq!(vm.events[0].1.len(), 2);
    }

    #[test]
    fn test_nyaya_check_positive() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(10)),
            OpCode::NyayaCheck,
        ]).unwrap();
        // Positive integer > 0 threshold → Sat → returns the value
        assert_eq!(result, Value::Integer(10));
    }

    #[test]
    fn test_return_stops_execution() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(42)),
            OpCode::Return,
            OpCode::Push(Value::Integer(999)), // should not execute
        ]).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_string_concatenation_via_add() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::Push(Value::Str("Hare ".into())),
            OpCode::Push(Value::Str("Krishna".into())),
            OpCode::Add,
        ]).unwrap();
        assert_eq!(result, Value::Str("Hare Krishna".into()));
    }

    #[test]
    fn test_complex_arithmetic() {
        let mut vm = KasturiVM::new();
        // (10 + 20) * 3 = 90
        let result = vm.execute(&[
            OpCode::Push(Value::Integer(10)),
            OpCode::Push(Value::Integer(20)),
            OpCode::Add,
            OpCode::Push(Value::Integer(3)),
            OpCode::Mul,
        ]).unwrap();
        assert_eq!(result, Value::Integer(90));
    }

    #[test]
    fn test_stack_underflow_error() {
        let mut vm = KasturiVM::new();
        let result = vm.execute(&[
            OpCode::StoreLocal("x".into()), // nothing on stack
        ]);
        assert!(result.is_err());
    }
}
