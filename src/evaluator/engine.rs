/// # Evaluation Engine
///
/// Walks the AST and executes the program.
/// Braj Bhasha RK

use crate::parser::ast::*;
use crate::dhatu::DerivationEngine;
use crate::anuvritti::{AnuvrittiManager, ScopeKind};
use super::environment::{Environment, Value};
use super::builtins;
use async_recursion::async_recursion;
use std::collections::HashMap;
use crate::vyakarana::{LinganushasanamEngine, UnadiEngine};

/// Main execution engine
#[derive(Clone)]
pub struct Engine {
    pub env: Environment,
    pub anuvritti: AnuvrittiManager,
    pub derivation: DerivationEngine,
    /// Defined functions registry
    functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
    /// Security Sandbox Mode
    pub is_sandboxed: bool,
    /// Gas/Resonance limit per execution (None = unlimited)
    pub resonance_limit: Option<usize>,
    /// Gas/Resonance used so far
    pub resonance_used: usize,
    /// Panini Grammar Engine: tracks structural depth for resonance discount
    pub ast_depth: usize,
    /// Linganushasanam Engine for strong typing and permissions
    pub linga: LinganushasanamEngine,
    /// Unadi Engine for Asset Factory tokenization
    pub unadi: UnadiEngine,
    /// Recursion depth counter (Phase 12: Stack Overflow Protection)
    pub call_depth: usize,
}

use crate::evaluator::chatushkoti::ChatushkotiEngine;

impl Engine {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            anuvritti: AnuvrittiManager::new(),
            derivation: DerivationEngine::new(),
            functions: HashMap::new(),
            is_sandboxed: false,
            resonance_limit: None, // By default unlimited (node runner can restrict)
            resonance_used: 0,
            ast_depth: 0,
            linga: LinganushasanamEngine::new(),
            unadi: UnadiEngine::new(),
            call_depth: 0,
        }
    }

    /// Set execution limits
    pub fn with_resonance_limit(mut self, limit: usize) -> Self {
        self.resonance_limit = Some(limit);
        self
    }

    /// Execute a complete program
    pub async fn execute(&mut self, program: &Program) -> Result<Value, RuntimeError> {
        let mut last = Value::Shunya;
        for stmt in &program.statements {
            last = self.execute_statement(stmt).await?;
        }
        Ok(last)
    }

    /// Consume gas/resonance (Panini Grammar Engine logic included)
    fn consume_resonance(&mut self, mut amount: usize) -> Result<(), RuntimeError> {
        // Panini Grammar Engine: Mantra Score Discount
        // Beautiful, linear code (low depth) resonates better and costs less gas.
        // Deeply nested, chaotic code (high depth) causes dissonance and costs more.
        if self.ast_depth <= 2 {
            // High resonance! 50% gas discount (Mantra Score high)
            amount = amount.saturating_sub(amount / 2);
        } else if self.ast_depth > 5 {
            // Dissonance! 50% gas penalty
            amount = amount.saturating_add(amount / 2);
        }
        
        // Never go below 1 unless amount was 0
        if amount == 0 && self.ast_depth <= 2 { amount = 1; }

        self.resonance_used += amount;
        if let Some(limit) = self.resonance_limit {
            if self.resonance_used > limit {
                return Err(RuntimeError::ResonanceExhausted(self.resonance_used, limit));
            }
        }
        Ok(())
    }

    /// Execute a single statement
    #[async_recursion]
    pub async fn execute_statement(&mut self, stmt: &Statement) -> Result<Value, RuntimeError> {
        self.consume_resonance(1)?; // 1 unit per statement
        self.ast_depth += 1;
        
        // --- THE DEEP DIVE PATCH #4: Vritti Memory Leak Prevention ---
        // In Sandbox mode, cap the Vritti memory ribbon to prevent memory exhaustion.
        // Each variable mutation appends a new record without deleting old ones (append-only model).
        // A malicious contract could fill RAM with 10,000+ records per execution.
        if self.is_sandboxed && self.env.vritti.len() > 50_000 {
            return Err(RuntimeError::General(
                "🛑 SANDBOX: Vritti memory ribbon exceeded 50,000 records. Possible memory exhaustion attack.".into()
            ));
        }
        
        let result = match stmt {
            Statement::Expression(expr) => self.evaluate_expr(expr).await,

            Statement::Assignment { name, value } => {
                // --- THE DEEP DIVE PATCH #7: System Variable Protection ---
                // Prevent smart contracts from spoofing their identity by overwriting system injected variables
                if name.starts_with("यन्त्र_") || name.starts_with("system_") {
                    return Err(RuntimeError::General(format!("🛑 SANDBOX: Cannot mutate critical system variable '{}'", name)));
                }

                let val = self.evaluate_expr(value).await?;
                
                // Linganushasanam Security Enforcer (Zero-Exploit Vaults)
                let is_mutation = self.env.get(&name).is_some();
                let linga_type = self.linga.determine_type(&name);
                
                if let Err(security_error) = self.linga.verify_permission(&linga_type, is_mutation) {
                    return Err(RuntimeError::TypeError(security_error.to_string()));
                }

                if !self.env.set(&name, val.clone()) {
                    self.env.define(name.clone(), val.clone());
                }
                Ok(val)
            }

            Statement::IfElse { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate_expr(condition).await?;
                if cond_val.is_truthy() {
                    let mut last = Value::Shunya;
                    for s in then_branch {
                        last = self.execute_statement(s).await?;
                    }
                    Ok(last)
                } else if let Some(els) = else_branch {
                    let mut last = Value::Shunya;
                    for s in els {
                        last = self.execute_statement(s).await?;
                    }
                    Ok(last)
                } else {
                    Ok(Value::Shunya)
                }
            }

            Statement::SutraRule(rule) => self.execute_sutra_rule(rule).await,

            Statement::Import(path) => {
                // --- THE DEEP DIVE PATCH #2: Path Traversal & Sandbox Import Prevention ---
                // In Sandbox mode (RPC/Smart Contracts), Import is COMPLETELY FORBIDDEN
                if self.is_sandboxed {
                    return Err(RuntimeError::General(
                        "🛑 SANDBOX: Import (आहर) is FORBIDDEN in sandboxed execution. Cannot load external files.".into()
                    ));
                }
                
                // Even outside sandbox, prevent directory traversal attacks
                if path.contains("..") {
                    return Err(RuntimeError::General(
                        format!("🛑 SECURITY: Path traversal detected in import '{}'. The '..' sequence is forbidden.", path)
                    ));
                }
                
                // Only allow .sutra file imports
                if !path.ends_with(".sutra") {
                    return Err(RuntimeError::General(
                        format!("🛑 SECURITY: Only .sutra files can be imported. Rejected: '{}'", path)
                    ));
                }

                // Read and execute imported file
                let content = match tokio::fs::read_to_string(path).await {
                    Ok(s) => s,
                    Err(e) => return Err(RuntimeError::General(format!("आयात दोष: '{}' पठितुं न शक्यते: {}", path, e))),
                };
                
                let mut scanner = crate::lexer::Scanner::new(&content);
                let tokens = scanner.scan_tokens();
                let mut parser = crate::parser::SutraParser::new(tokens);
                let program = parser.parse().map_err(|e| RuntimeError::General(e.to_string()))?;

                let mut last = Value::Shunya;
                for s in &program.statements {
                    last = Box::pin(self.execute_statement(s)).await?;
                }
                Ok(last)
            }

            Statement::Adhikara { context, body } => {
                let ctx_val = self.evaluate_expr(context).await?;
                let ctx_name = format!("{}", ctx_val);
                self.anuvritti.enter_scope_with_context(
                    ctx_name.clone(), ScopeKind::Adhikara, ctx_name,
                );
                self.env.push_scope();

                let mut last = Value::Shunya;
                for s in body {
                    last = self.execute_statement(s).await?;
                }

                self.env.pop_scope();
                self.anuvritti.exit_scope();
                Ok(last)
            }

            Statement::Prakarana { context, body } => {
                let ctx_val = self.evaluate_expr(context).await?;
                let ctx_name = format!("{}", ctx_val);
                self.anuvritti.enter_scope(ctx_name, ScopeKind::Prakarana);
                self.env.push_scope();

                let mut last = Value::Shunya;
                for s in body {
                    last = self.execute_statement(s).await?;
                }

                self.env.pop_scope();
                self.anuvritti.exit_scope();
                Ok(last)
            }

            Statement::SutraDefinition { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                self.env.define(name.clone(), Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body_source: format!("<Sutra {} — {} params>", name, params.len()),
                });
                Ok(Value::Shunya)
            }

            Statement::Ashtadhyayi { body } => {
                // Term Rewriting Engine (Ashtadhyayi)
                // No new scope here — transformations persist on the Vritti memory ribbon
                let mut last = Value::Shunya;
                let mut changed = true;
                let mut iterations = 0;
                
                while changed && iterations < 1000 { // infinite loop guard
                    changed = false;
                    iterations += 1;
                    
                    // Vipratishedhe Param Karyam
                    // On conflict, later rules take precedence.
                    // Evaluate rules from last to first
                    for s in body.iter().rev() {
                        let pre_len = self.env.vritti.len();
                        
                        match self.execute_statement(s).await {
                            Ok(val) => {
                                last = val;
                                // If Vritti memory ribbon was modified, rule was applied — retry
                                if self.env.vritti.len() != pre_len {
                                    changed = true;
                                    break; // Break to re-apply rules on new state
                                }
                            },
                            Err(e) => return Err(e),
                        }
                    }
                }
                
                Ok(last)
            }

            Statement::ForEach { item, collection, body } => {
                let coll_val = self.evaluate_expr(collection).await?;
                let mut last = Value::Shunya;

                let iter_items = match coll_val {
                    Value::List(l) => l,
                    Value::Dict(d) => d.into_iter().map(|(k, v)| {
                        let mut pair = HashMap::new();
                        pair.insert("key".to_string(), Value::Str(k));
                        pair.insert("value".to_string(), v);
                        Value::Dict(pair)
                    }).collect(),
                    _ => return Err(RuntimeError::TypeError("Cannot iterate over non-collection".into())),
                };

                for val in iter_items {
                    self.env.define(item.clone(), val);
                    for s in body {
                        last = self.execute_statement(s).await?;
                    }
                }

                Ok(last)
            }

            Statement::TryCatch { try_block, error_var, catch_block } => {
                self.env.push_scope();
                let mut success = true;
                let mut last = Value::Shunya;
                let mut err_msg = String::new();

                for s in try_block {
                    match self.execute_statement(s).await {
                        Ok(val) => last = val,
                        Err(e) => {
                            success = false;
                            err_msg = format!("{}", e);
                            break;
                        }
                    }
                }
                self.env.pop_scope();

                if !success {
                    self.env.push_scope();
                    self.env.define(error_var.clone(), Value::Str(err_msg));
                    for s in catch_block {
                        last = self.execute_statement(s).await?;
                    }
                    self.env.pop_scope();
                }

                Ok(last)
            }

            Statement::StructDef { name, fields } => {
                // We store struct definition as a special function that acts as constructor
                let body = Vec::new();
                self.functions.insert(name.clone(), (fields.clone(), body));
                self.env.define(name.clone(), Value::Function {
                    name: name.clone(),
                    params: fields.clone(),
                    body_source: format!("<स्वरूप {}>", name),
                });
                Ok(Value::Shunya)
            }

            Statement::WhileLoop { condition, body } => {
                let mut last = Value::Shunya;
                let mut iterations = 0;
                loop {
                    let cond_val = self.evaluate_expr(condition).await?;
                    if !cond_val.is_truthy() {
                        break;
                    }
                    iterations += 1;
                    // --- PHASE 12 PATCH: Sandbox-aware loop limit ---
                    let max_iterations = if self.is_sandboxed { 1_000 } else { 100_000 };
                    if iterations > max_iterations {
                        return Err(RuntimeError::General(format!(
                            "यावत् अनन्त (while loop exceeded {} iterations)", max_iterations
                        )));
                    }
                    for s in body {
                        match self.execute_statement(s).await {
                            Ok(val) => last = val,
                            Err(RuntimeError::ReturnValue(v)) => return Err(RuntimeError::ReturnValue(v)),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Ok(last)
            }

            Statement::Return(expr) => {
                let val = self.evaluate_expr(expr).await?;
                Err(RuntimeError::ReturnValue(val))
            }

            Statement::Match { target, arms, default } => {
                let target_val = self.evaluate_expr(target).await?;
                for (pattern, body) in arms {
                    let pattern_val = self.evaluate_expr(pattern).await?;
                    if self.values_equal(&target_val, &pattern_val) {
                        let mut last = Value::Shunya;
                        for s in body {
                            last = self.execute_statement(s).await?;
                        }
                        return Ok(last);
                    }
                }
                // No arm matched, try default
                if let Some(default_body) = default {
                    let mut last = Value::Shunya;
                    for s in default_body {
                        last = self.execute_statement(s).await?;
                    }
                    return Ok(last);
                }
                Ok(Value::Shunya)
            }
        };
        
        self.ast_depth -= 1;
        result
    }

    /// Execute a declarative Sutra rule
    #[async_recursion]
    async fn execute_sutra_rule(&mut self, rule: &SutraRule) -> Result<Value, RuntimeError> {
        let source_val = self.evaluate_dhatu(&rule.source).await?;

        if let Some(cond) = &rule.condition {
            let cond_val = self.evaluate_expr(cond).await?;
            if !cond_val.is_truthy() {
                return Ok(Value::Shunya);
            }
        }

        if let Some(transform) = &rule.transform {
            let _transform_val = self.evaluate_dhatu(transform).await?;
        }

        if let Some(result) = &rule.result {
            return self.evaluate_expr(result).await;
        }

        Ok(source_val)
    }

    /// Evaluate a Dhatu root expression
    #[async_recursion]
    async fn evaluate_dhatu(&mut self, dhatu: &DhatuExpr) -> Result<Value, RuntimeError> {
        let root = &dhatu.root;
        let suffix = dhatu.suffix.as_deref().unwrap_or("ti");

        let mut param_values = Vec::new();
        for param in &dhatu.params {
            param_values.push(self.evaluate_expr(param).await?);
        }

        // --- PHASE 12 PATCH: TOTAL Sandbox Builtin Firewall ---
        // Block ALL dangerous operations in BOTH Sanskrit AND Latin transliteration forms.
        // Previous firewall only blocked Sanskrit names, allowing bypass via Latin roots!
        if self.is_sandboxed {
            match root.as_str() {
                // File I/O — BOTH forms
                "पठ्" | "paṭh" => return Err(RuntimeError::General("🛑 SANDBOX: File read (पठ्/paṭh) is FORBIDDEN in sandboxed execution.".into())),
                "लिख्" | "likh" => return Err(RuntimeError::General("🛑 SANDBOX: File write (लिख्/likh) is FORBIDDEN in sandboxed execution.".into())),
                // HTTP requests — BOTH forms
                "क्षिप्" | "kṣip" => return Err(RuntimeError::General("🛑 SANDBOX: HTTP requests (क्षिप्) are FORBIDDEN in sandboxed execution.".into())),
                // Server spawning — BOTH forms
                "स्था" | "sthā" => return Err(RuntimeError::General("🛑 SANDBOX: Server spawning (स्था/sthā) is FORBIDDEN in sandboxed execution.".into())),
                // Shell execution — BOTH forms
                "क्षेत्र" | "kṣetra" => return Err(RuntimeError::General("🛑 SANDBOX: Shell execution (क्षेत्र/kṣetra) is FORBIDDEN in sandboxed execution.".into())),
                // HTTP client — BOTH forms
                "याच्" | "yāc" => return Err(RuntimeError::General("🛑 SANDBOX: HTTP client (याच्/yāc) is FORBIDDEN in sandboxed execution.".into())),
                // Import/require — BOTH forms
                "आहृ" | "āhṛ" => return Err(RuntimeError::General("🛑 SANDBOX: Import (आहृ/āhṛ) is FORBIDDEN in sandboxed execution.".into())),
                // Global DB write — CRITICAL: contracts must use √sañci instead!
                "स्मृ" | "smṛ" => return Err(RuntimeError::General("🛑 SANDBOX: Global DB write (स्मृ/smṛ) is FORBIDDEN. Use √sañci for contract storage.".into())),
                // Global DB read — CRITICAL: contracts must use √labh instead!
                "दृश्" | "dṛś" => return Err(RuntimeError::General("🛑 SANDBOX: Global DB read (दृश्/dṛś) is FORBIDDEN. Use √labh for contract storage.".into())),
                // Content-Addressable Storage — BLOCKED in sandbox
                "आकाश" => return Err(RuntimeError::General("🛑 SANDBOX: Akasha CAS (आकाश) is FORBIDDEN in sandboxed execution.".into())),
                _ => {} // Other builtins are safe
            }
        }

        // Async built-in roots
        if let Some(result) = builtins::execute_builtin(root, suffix, &param_values).await {
            return Ok(result);
        }

        // Phase 7 path (developer experience & standard library)
        match root.as_str() {
            "āhṛ" | "आहृ" => return self.builtin_ahr(&param_values).await,
            "paṭh" | "पठ्" => return self.builtin_path(&param_values).await,
            "likh" | "लिख्" => return self.builtin_likh(&param_values).await,
            "yāc" | "याच्" => return self.builtin_yac(&param_values).await,
            "sthā" | "स्था" => return self.builtin_stha(&param_values).await,
            "khan" | "खन्" => return self.builtin_khan(&param_values).await,
            "kṣetra" | "क्षेत्र" => return self.builtin_ksetra(&param_values).await,
            "guh" | "गुह्" => return self.builtin_guh(&param_values).await,
            "kāla" | "काल" => return self.builtin_kala(&param_values).await,
            "nakṣatra" | "नक्षत्र" => return self.builtin_nakshatra(&param_values).await,
            "nāda" | "नाद" => return self.builtin_nada(&param_values).await,
            "tapas" | "तपस्" => return self.builtin_tapas(&param_values).await,
            "setu" | "सेतु" => return self.builtin_setu(&param_values).await,
            "mukh" | "मुख" => return self.builtin_mukh(&param_values).await,
            "mārg" | "मार्ग" => return self.builtin_marg(&param_values).await,
            "kuñc" | "कुञ्च्" => return self.builtin_kunc(&param_values).await,
            "cihna" | "चिह्न" => return self.builtin_cihna(&param_values).await,
            "parīkṣ" | "परीक्ष्" => return self.builtin_pariksh(&param_values).await,
            "saṁvad" | "संवाद" => return self.builtin_samvad(&param_values).await,
            "yant" | "यन्त्र" => return self.builtin_yant(&param_values).await,
            
            // Vedic AI Cognitive Contracts (Chatushkoti)
            "bodh" | "बोध्" => {
                let input = param_values.get(0).unwrap_or(&Value::Shunya);
                let threshold = if param_values.len() > 1 {
                    match &param_values[1] {
                        Value::Integer(n) => *n as f64,
                        Value::Float(f) => *f,
                        _ => 0.0,
                    }
                } else {
                    0.0
                };
                let decision = ChatushkotiEngine::evaluate_cognition(input, threshold);
                // Return decision wrapped in Dict for script consumption
                let mut map = std::collections::HashMap::new();
                match decision {
                    crate::evaluator::chatushkoti::CognitiveDecision::Sat(v) => {
                        map.insert("state".into(), Value::Str("Sat".into()));
                        map.insert("value".into(), v);
                    }
                    crate::evaluator::chatushkoti::CognitiveDecision::Asat(v) => {
                        map.insert("state".into(), Value::Str("Asat".into()));
                        map.insert("value".into(), v);
                    }
                    crate::evaluator::chatushkoti::CognitiveDecision::Sadasat(v) => {
                        map.insert("state".into(), Value::Str("Sadasat".into()));
                        map.insert("value".into(), v);
                    }
                    crate::evaluator::chatushkoti::CognitiveDecision::Avyaktam => {
                        map.insert("state".into(), Value::Str("Avyaktam".into()));
                    }
                }
                return Ok(Value::Dict(map));
            }
            "nirṇay" | "निर्णय्" => {
                let input = param_values.get(0).unwrap_or(&Value::Shunya);
                if let Value::Dict(map) = input {
                    let state_str = match map.get("state") {
                        Some(Value::Str(s)) => s.as_str(),
                        _ => return Err(RuntimeError::TypeError("Invalid cognitive decision dict".into())),
                    };
                    let val = map.get("value").cloned().unwrap_or(Value::Shunya);
                    
                    let decision = match state_str {
                        "Sat" => crate::evaluator::chatushkoti::CognitiveDecision::Sat(val),
                        "Asat" => crate::evaluator::chatushkoti::CognitiveDecision::Asat(val),
                        "Sadasat" => crate::evaluator::chatushkoti::CognitiveDecision::Sadasat(val),
                        _ => crate::evaluator::chatushkoti::CognitiveDecision::Avyaktam,
                    };
                    return ChatushkotiEngine::resolve_decision(decision);
                }
                return Err(RuntimeError::TypeError("√nirṇay requires a CognitiveDecision dict from √bodh".into()));
            }
            "sañci" | "सञ्चि" => return self.builtin_sanci(&param_values).await,
            "labh" | "लभ्" => return self.builtin_labh(&param_values).await,
            "ghaṭ" | "घट्" => return self.builtin_ghat(&param_values).await,
            "smṛ" | "स्मृ" => return self.builtin_smr(&param_values).await,
            "dṛś" | "दृश्" => return self.builtin_drs(&param_values).await,
            _ => {}
        }

        // Named function call
        if let Some((params, body)) = self.functions.get(root).cloned() {
            let pratyaya_reg = crate::dhatu::pratyaya::PratyayaRegistry::new();
            let effect = pratyaya_reg.lookup(suffix)
                .map(|p| p.computational_effect.clone())
                .unwrap_or(crate::dhatu::pratyaya::PratyayaEffect::SyncExecute);
                
            // Phase 1: Vedic Concurrency (Async Promise)
            if suffix == "tum" || suffix == "तुम्" || effect == crate::dhatu::pratyaya::PratyayaEffect::AsyncExecute {
                // --- PHASE 12 PATCH: Block async fork bombs in sandbox ---
                if self.is_sandboxed {
                    return Err(RuntimeError::General("🛑 SANDBOX: Async task spawning (तुम्/tum) is FORBIDDEN in sandboxed execution. Use synchronous calls.".into()));
                }
                let mut engine_clone = self.clone();
                let params_owned = params.clone();
                let body_owned = body.clone();
                let param_values_owned = param_values.clone();
                
                tokio::spawn(async move {
                    // Detached async execution
                    let _ = engine_clone.call_function(&params_owned, &body_owned, &param_values_owned).await;
                });
                
                // Return a Promise placeholder
                return Ok(Value::Str(format!("<Pratijna Promise: {}>", root)));
            }

            return self.call_function(&params, &body, &param_values).await;
        }

        let sandhi_name = crate::dhatu::apply_sandhi(root, suffix);
        Ok(Value::Str(sandhi_name))
    }

    /// Evaluate a general expression
    #[async_recursion]
    pub async fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(lit) => Ok(match lit {
                Literal::Integer(n) => Value::Integer(*n),
                Literal::Float(n) => Value::Float(*n),
                Literal::Str(s) => Value::Str(s.clone()),
                Literal::Tattva(t) => Value::Tattva(*t),
                Literal::Shunya => Value::Shunya,
            }),

            Expr::Identifier(name) => {
                self.env.get(name).cloned().ok_or_else(|| {
                    RuntimeError::UndefinedVariable(name.clone())
                })
            }

            Expr::Dhatu(dhatu) => self.evaluate_dhatu(dhatu).await,

            Expr::Binary { left, operator, right } => {
                let l = self.evaluate_expr(left).await?;
                let r = self.evaluate_expr(right).await?;
                self.evaluate_binary(l, operator, r).await
            }

            Expr::Unary { operator, operand } => {
                let val = self.evaluate_expr(operand).await?;
                match operator {
                    UnaryOp::Negate => match val {
                        Value::Integer(n) => Ok(Value::Integer(-n)),
                        Value::Float(n) => Ok(Value::Float(-n)),
                        _ => Err(RuntimeError::TypeError("negate non-number".into())),
                    },
                    UnaryOp::Not => Ok(Value::Tattva(if val.is_truthy() { crate::evaluator::TattvaState::Asat } else { crate::evaluator::TattvaState::Sat })),
                }
            }

            Expr::Call { callee, args } => {
                let func = self.evaluate_expr(callee).await?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate_expr(arg).await?);
                }
                match &func {
                    Value::Function { name, params, body_source } => {
                        if body_source.starts_with("<स्वरूप") {
                            // This is a struct instantiation
                            let mut fields = HashMap::new();
                            for (i, param_name) in params.iter().enumerate() {
                                let val = arg_vals.get(i).cloned().unwrap_or(Value::Shunya);
                                fields.insert(param_name.clone(), val);
                            }
                            Ok(Value::StructInstance {
                                name: name.clone(),
                                fields,
                            })
                        } else if let Some((func_params, body)) = self.functions.get(name).cloned() {
                            self.call_function(&func_params, &body, &arg_vals).await
                        } else {
                            Ok(Value::Shunya)
                        }
                    }
                    Value::Lambda { params, body } => {
                        self.call_function(params, body, &arg_vals).await
                    }
                    _ => Err(RuntimeError::TypeError("not callable".into())),
                }
            }

            Expr::CurrentElement => {
                self.env.get("◈").cloned().ok_or_else(|| {
                    RuntimeError::UndefinedVariable("◈".into())
                })
            }

            Expr::Lambda { params, body } => {
                Ok(Value::Lambda {
                    params: params.clone(),
                    body: body.clone(),
                })
            }

            Expr::Range { start, end } => {
                let s = self.evaluate_expr(start).await?;
                let e = self.evaluate_expr(end).await?;
                match (s, e) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        // --- PHASE 12 PATCH: Range Memory Bomb Protection ---
                        // Prevent allocation of billions of elements via `1..999999999`
                        let max_range = if self.is_sandboxed { 10_000 } else { 100_000 };
                        let range_size = if b >= a { (b - a + 1) as usize } else { 0 };
                        if range_size > max_range {
                            return Err(RuntimeError::General(format!(
                                "Range too large: {} elements exceeds limit of {}. Use pagination.",
                                range_size, max_range
                            )));
                        }
                        let range: Vec<Value> = (a..=b).map(Value::Integer).collect();
                        Ok(Value::List(range))
                    }
                    _ => Err(RuntimeError::TypeError("range requires integers".into())),
                }
            }

            Expr::ParamRef(name) => {
                self.env.get(name).cloned().ok_or_else(|| {
                    RuntimeError::UndefinedVariable(format!(":{}", name))
                })
            }

            Expr::SutraExpr(rule) => self.execute_sutra_rule(rule).await,

            Expr::Dict(pairs) => {
                let mut map = HashMap::new();
                for (k_expr, v_expr) in pairs {
                    let k_val = self.evaluate_expr(k_expr).await?;
                    let k_str = format!("{}", k_val);
                    let v_val = self.evaluate_expr(v_expr).await?;
                    map.insert(k_str, v_val);
                }
                Ok(Value::Dict(map))
            }

            Expr::IndexAccess { object, index } => {
                let obj_val = self.evaluate_expr(object).await?;
                let idx_val = self.evaluate_expr(index).await?;
                match obj_val {
                    Value::List(l) => {
                        if let Value::Integer(i) = idx_val {
                            if i >= 0 && (i as usize) < l.len() {
                                return Ok(l[i as usize].clone());
                            }
                        }
                        Ok(Value::Shunya)
                    }
                    Value::Dict(d) => {
                        let key = format!("{}", idx_val);
                        Ok(d.get(&key).cloned().unwrap_or(Value::Shunya))
                    }
                    _ => Err(RuntimeError::TypeError("Cannot index non-collection".into())),
                }
            }

            Expr::PropertyAccess { object, property } => {
                let obj_val = self.evaluate_expr(object).await?;
                match obj_val {
                    Value::StructInstance { fields, .. } => {
                        Ok(fields.get(property).cloned().unwrap_or(Value::Shunya))
                    }
                    Value::Dict(d) => {
                        Ok(d.get(property).cloned().unwrap_or(Value::Shunya))
                    }
                    _ => Err(RuntimeError::TypeError(format!("Cannot access property on type {}", obj_val.type_name()))),
                }
            }
        }
    }

    /// Evaluate a binary operation
    async fn evaluate_binary(&self, l: Value, op: &BinaryOp, r: Value) -> Result<Value, RuntimeError> {
        match op {
            BinaryOp::Add => match (l, r) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + b as f64)),
                (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                _ => Err(RuntimeError::TypeError("cannot add".into())),
            },
            BinaryOp::Subtract => match (l, r) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                _ => Err(RuntimeError::TypeError("cannot subtract".into())),
            },
            BinaryOp::Multiply => match (l, r) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(crate::math::vedic::urdhva_tiryagbhyam(a, b))),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(crate::math::vedic::urdhva_tiryagbhyam_float(a, b))),
                _ => Err(RuntimeError::TypeError("cannot multiply".into())),
            },
            BinaryOp::Divide => match (l, r) {
                (Value::Integer(a), Value::Integer(b)) if b != 0 => Ok(Value::Integer(a / b)),
                (Value::Float(a), Value::Float(b)) if b != 0.0 => Ok(Value::Float(a / b)),
                _ => Err(RuntimeError::TypeError("division error".into())),
            },
            BinaryOp::Modulo => match (l, r) {
                (Value::Integer(a), Value::Integer(b)) if b != 0 => Ok(Value::Integer(a % b)),
                _ => Err(RuntimeError::TypeError("modulo error".into())),
            },
            BinaryOp::Equal => Ok(Value::Tattva(if self.values_equal(&l, &r) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            BinaryOp::NotEqual => Ok(Value::Tattva(if !self.values_equal(&l, &r) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            BinaryOp::Less => self.compare_values(&l, &r, |a, b| a < b),
            BinaryOp::LessEqual => self.compare_values(&l, &r, |a, b| a <= b),
            BinaryOp::Greater => self.compare_values(&l, &r, |a, b| a > b),
            BinaryOp::GreaterEqual => self.compare_values(&l, &r, |a, b| a >= b),
            BinaryOp::Join => {
                builtins::execute_builtin("yuj", "ति", &[l, r]).await
                    .ok_or_else(|| RuntimeError::TypeError("cannot join".into()))
            }
            BinaryOp::And => {
                // Short-circuit: if left is falsy, return left
                if !l.is_truthy() {
                    Ok(l)
                } else {
                    Ok(r)
                }
            }
            BinaryOp::Or => {
                // Short-circuit: if left is truthy, return left
                if l.is_truthy() {
                    Ok(l)
                } else {
                    Ok(r)
                }
            }
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => (x - y).abs() < f64::EPSILON,
            (Value::Str(x), Value::Str(y)) => x == y,
            (Value::Tattva(x), Value::Tattva(y)) => x == y,
            (Value::Shunya, Value::Shunya) => true,
            _ => false,
        }
    }

    fn compare_values<F>(&self, a: &Value, b: &Value, f: F) -> Result<Value, RuntimeError>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Tattva(if f(*x as f64, *y as f64) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Tattva(if f(*x, *y) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Tattva(if f(*x as f64, *y) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Tattva(if f(*x, *y as f64) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat })),
            _ => Err(RuntimeError::TypeError("cannot compare".into())),
        }
    }

    /// Call a function with arguments
    #[async_recursion]
    async fn call_function(
        &mut self,
        params: &[String],
        body: &[Statement],
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // --- PHASE 12 PATCH: Recursion Depth Limit ---
        // Prevent stack overflow from infinite recursion (e.g. f(n) { f(n) })
        let max_depth = if self.is_sandboxed { 64 } else { 256 };
        if self.call_depth >= max_depth {
            return Err(RuntimeError::General(format!(
                "🛑 Stack overflow: recursion depth {} exceeds limit of {}", self.call_depth, max_depth
            )));
        }
        self.call_depth += 1;
        self.env.push_scope();
        self.anuvritti.enter_scope("सूत्र_call", ScopeKind::Sutra);

        for (i, param) in params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Shunya);
            self.env.define(param.clone(), val);
        }

        let mut last = Value::Shunya;
        for stmt in body {
            match self.execute_statement(stmt).await {
                Ok(val) => last = val,
                Err(RuntimeError::ReturnValue(v)) => {
                    self.anuvritti.exit_scope();
                    self.env.pop_scope();
                    self.call_depth -= 1;
                    return Ok(v);
                }
                Err(e) => {
                    self.anuvritti.exit_scope();
                    self.env.pop_scope();
                    self.call_depth -= 1;
                    return Err(e);
                }
            }
        }

        self.anuvritti.exit_scope();
        self.env.pop_scope();
        self.call_depth -= 1;
        Ok(last)
    }

    /// √sthā — Web Server (Ether-Establish)
    pub async fn builtin_stha(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if self.is_sandboxed {
            return Err(RuntimeError::General("⛔ Sandbox Violation: Server spawning (√sthā) is blocked inside smart contracts.".into()));
        }
        if params.len() < 2 {
            return Err(RuntimeError::General("√sthā requires port number and handler function name".into()));
        }

        let port = match params[0] {
            Value::Integer(n) => n,
            _ => return Err(RuntimeError::TypeError("Port must be an integer".into())),
        };

        let handler_name = match &params[1] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Handler name must be a string".into())),
        };

        if !self.functions.contains_key(&handler_name) {
            return Err(RuntimeError::UndefinedVariable(handler_name));
        }

        use std::sync::Arc;
        use tokio::sync::Mutex;
        use axum::{routing::{get, post, put, delete, patch, options, head, trace}, Router, extract::{State, Request}, response::IntoResponse};
        use http_body_util::BodyExt;

        let engine_clone = self.clone();
        let state = Arc::new(Mutex::new((engine_clone, handler_name.clone())));

        async fn dynamic_handler(
            State(state): State<Arc<Mutex<(Engine, String)>>>,
            req: Request<axum::body::Body>,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let method_str = parts.method.as_str().to_string();
            let path = parts.uri.path().to_string();
            let path = percent_encoding::percent_decode_str(&path).decode_utf8_lossy().into_owned();
            
            let mut headers_dict = std::collections::HashMap::new();
            for (k, v) in parts.headers.iter() {
                if let Ok(val_str) = v.to_str() {
                    headers_dict.insert(k.as_str().to_string(), Value::Str(val_str.to_string()));
                }
            }

            let bytes = body.collect().await.unwrap().to_bytes();

            let body_str = String::from_utf8_lossy(&bytes).into_owned();

            let mut request_dict = std::collections::HashMap::new();
            request_dict.insert("मार्ग".into(), Value::Str(method_str)); // Route/Method
            request_dict.insert("पथ".into(), Value::Str(path)); // Path
            request_dict.insert("शिरस्".into(), Value::Dict(headers_dict)); // Headers
            
            // Try parse JSON body, else string
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&body_str) {
                request_dict.insert("देह".into(), crate::evaluator::builtins::json_to_value(&json_val));
            } else {
                request_dict.insert("देह".into(), Value::Str(body_str));
            }

            let request_val = Value::Dict(request_dict);

            let mut guard = state.lock().await;
            let handler = guard.1.clone();
            let engine = &mut guard.0;

            if let Some((params, body)) = engine.functions.get(&handler).cloned() {
                match engine.call_function(&params, &body, &[request_val]).await {
                    Ok(Value::Dict(resp_dict)) => {
                        let status = match resp_dict.get("स्थिति") {
                            Some(Value::Integer(n)) => axum::http::StatusCode::from_u16(*n as u16).unwrap_or(axum::http::StatusCode::OK),
                            _ => axum::http::StatusCode::OK,
                        };
                        
                        let body = match resp_dict.get("देह") {
                            Some(val) => {
                                let json = crate::evaluator::builtins::value_to_json(val);
                                serde_json::to_string(&json).unwrap_or_default()
                            }
                            None => String::new(),
                        };

                        (status, body).into_response()
                    }
                    Ok(other) => {
                        println!("Sutra returned non-dict: {}", other);
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Invalid response from sutra handler").into_response()
                    }
                    Err(e) => {
                        println!("Sutra handler error: {:?}", e);
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Invalid response from sutra handler").into_response()
                    }
                }
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Handler not found").into_response()
            }
        }

        async fn explorer_handler() -> axum::response::Response {
            let db_lock = crate::storage::mandala::MANDALA_DB.lock().unwrap();
            let mut total_bindus = 0;
            let mut nodes = Vec::new();

            for item in db_lock.db.iter() {
                if let Ok((k_ivec, v_ivec)) = item {
                    total_bindus += 1;
                    if let Ok(bindu) = serde_json::from_slice::<crate::storage::mandala::Bindu>(&v_ivec) {
                        nodes.push(serde_json::json!({
                            "key": bindu.key,
                            "frequency": bindu.frequency,
                            "data": bindu.data_json
                        }));
                    }
                }
            }

            let resp = serde_json::json!({
                "status": "online",
                "total_shards": 4536,
                "total_bindus": total_bindus,
                "data": nodes
            });

            use axum::response::IntoResponse;
            (axum::http::StatusCode::OK, serde_json::to_string(&resp).unwrap_or_default()).into_response()
        }

        let app = Router::new()
            .route("/explorer/state", get(explorer_handler))
            .fallback(get(dynamic_handler)
                .post(dynamic_handler)
                .put(dynamic_handler)
                .delete(dynamic_handler)
                .patch(dynamic_handler)
                .options(dynamic_handler)
                .head(dynamic_handler)
                .trace(dynamic_handler))
            .with_state(state)
            .layer(tower_http::cors::CorsLayer::permissive());

        println!("ॐ Web server (आकाश-स्था) listening on port {}", port);
        
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| RuntimeError::General(format!("Failed to bind port: {}", e)))?;
        
        // This will block the executor until stopped!
        if let Err(e) = axum::serve(listener, app).await {
            return Err(RuntimeError::General(format!("Server error: {}", e)));
        }

        Ok(Value::Shunya)
    }

    /// √khan — खन् (Database operations - DEPRECATED)
    pub async fn builtin_khan(&mut self, _params: &[Value]) -> Result<Value, RuntimeError> {
        Err(RuntimeError::General("√khan is deprecated in Kasturichain. Use √smṛ (Store) or √dṛś (Retrieve) in Mandala storage.".into()))
    }

    /// √kṣetra — System Environment / Shell
    pub async fn builtin_ksetra(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if self.is_sandboxed {
            return Err(RuntimeError::General("⛔ Sandbox Violation: Shell execution (√kṣetra) is strictly blocked inside smart contracts.".into()));
        }
        if params.is_empty() {
            return Err(RuntimeError::General("√kṣetra requires a command".into()));
        }

        let cmd = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Command must be a string".into())),
        };

        if cmd == "env" {
            if params.len() > 1 {
                if let Value::Str(var_name) = &params[1] {
                    match std::env::var(var_name) {
                        Ok(v) => return Ok(Value::Str(v)),
                        Err(_) => return Ok(Value::Shunya),
                    }
                }
            }
            return Ok(Value::Shunya);
        }

        // Run shell command
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .arg("/C")
                .arg(&cmd)
                .output()
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
                Ok(Value::Str(stdout.trim().to_string()))
            }
            Err(e) => Err(RuntimeError::General(format!("Command execution failed: {}", e))),
        }
    }

    /// √āhṛ — Module Import System
    pub async fn builtin_ahr(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√āhṛ requires a file path to import".into()));
        }
        if let Value::Str(path) = &params[0] {
            let content_res = tokio::fs::read_to_string(path).await;
            let final_content = match content_res {
                Ok(c) => Ok(c),
                Err(e) => {
                    let sangraha_path = format!("_sangraha/{}", path);
                    tokio::fs::read_to_string(&sangraha_path).await.map_err(|_| e)
                }
            };

            match final_content {
                Ok(source) => {
                    let mut scanner = crate::lexer::Scanner::new(&source);
                    let tokens = scanner.scan_tokens();
                    let mut parser = crate::parser::SutraParser::new(tokens);
                    match parser.parse() {
                        Ok(prog) => {
                            let mut module_engine = Engine::new();
                            // Optional: pre-load context, but isolation is preferred.
                            module_engine.execute(&prog).await?;
                            
                            // Collect exported functions and variables.
                            // Currently, everything defined in the global scope of the module is exported.
                            let mut exports = HashMap::new();
                            
                            // Export functions
                            // Export functions
                            for (name, (func_params, body_source)) in module_engine.functions.iter() {
                                let func_val = Value::Function {
                                    name: name.clone(),
                                    params: func_params.clone(),
                                    body_source: format!("<Sutra {} — {} params>", name, func_params.len()),
                                };
                                exports.insert(name.clone(), func_val.clone());
                                // INJECT INTO GLOBAL SCOPE
                                self.functions.insert(name.clone(), (func_params.clone(), body_source.clone()));
                                self.env.define(name.clone(), func_val);
                            }
                            
                            // Export global variables (from Vritti memory)
                            for pada in &module_engine.env.vritti {
                                // Since we ran the whole script in the global scope of this module_engine, 
                                // all variables in vritti are module globals.
                                exports.insert(pada.name.clone(), pada.value.clone());
                            }
                            
                            return Ok(Value::Dict(exports));
                        }
                        Err(e) => {
                            return Err(RuntimeError::General(format!("Parse error in imported file: {:?}", e)));
                        }
                    }
                }
                Err(e) => return Err(RuntimeError::General(format!("Cannot read file '{}': {}", path, e))),
            }
        }
        Err(RuntimeError::TypeError("Import path must be a string".into()))
    }

    /// √paṭh — File Read
    pub async fn builtin_path(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if self.is_sandboxed {
            return Err(RuntimeError::General("⛔ Sandbox Violation: File I/O (√paṭh) is blocked inside smart contracts.".into()));
        }
        if params.is_empty() {
            return Err(RuntimeError::General("√paṭh requires a file path".into()));
        }
        if let Value::Str(path) = &params[0] {
            match tokio::fs::read_to_string(path).await {
                Ok(content) => return Ok(Value::Str(content)),
                Err(e) => return Err(RuntimeError::General(format!("File read error: {}", e))),
            }
        }
        Err(RuntimeError::TypeError("File path must be a string".into()))
    }

    /// √likh — File Write
    pub async fn builtin_likh(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if self.is_sandboxed {
            return Err(RuntimeError::General("⛔ Sandbox Violation: File Write (√likh) is blocked inside smart contracts.".into()));
        }
        if params.len() < 2 {
            return Err(RuntimeError::General("√likh requires file path and content".into()));
        }
        if let Value::Str(path) = &params[0] {
            let content = match &params[1] {
                Value::Str(s) => s.clone(),
                v => format!("{}", v),
            };
            match tokio::fs::write(path, content).await {
                Ok(_) => return Ok(Value::Tattva(crate::evaluator::TattvaState::Sat)),
                Err(e) => return Err(RuntimeError::General(format!("File write error: {}", e))),
            }
        }
        Err(RuntimeError::TypeError("File path must be a string".into()))
    }

    /// √yāc — HTTP Client Request
    pub async fn builtin_yac(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if self.is_sandboxed {
            return Err(RuntimeError::General("⛔ Sandbox Violation: HTTP requests (√yāc) are blocked inside smart contracts.".into()));
        }
        if params.is_empty() {
            return Err(RuntimeError::General("√yāc requires a URL".into()));
        }
        if let Value::Str(url) = &params[0] {
            let client = reqwest::Client::new();
            match client.get(url).send().await {
                Ok(res) => {
                    let status = res.status().as_u16();
                    let text = res.text().await.unwrap_or_default();
                    let mut dict = std::collections::HashMap::new();
                    dict.insert("status".to_string(), Value::Integer(status as i64));
                    dict.insert("body".to_string(), Value::Str(text));
                    return Ok(Value::Dict(dict));
                }
                Err(e) => return Err(RuntimeError::General(format!("HTTP request failed: {}", e))),
            }
        }
        Err(RuntimeError::TypeError("URL must be a string".into()))
    }

    /// √guh — SHA-256 Hashing
    pub async fn builtin_guh(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√guh requires text to hash".into()));
        }
        use sha2::{Sha256, Digest};
        let input = match &params[0] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Ok(Value::Str(hex::encode(result)))
    }

    /// √kāla — Unix Timestamp
    pub async fn builtin_kala(&mut self, _params: &[Value]) -> Result<Value, RuntimeError> {
        let start = std::time::SystemTime::now();
        let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(Value::Integer(since_the_epoch.as_secs() as i64))
    }

    /// √nakṣatra — नक्षत्र (Current Nakshatra info based on Cosmic Time)
    pub async fn builtin_nakshatra(&mut self, _params: &[Value]) -> Result<Value, RuntimeError> {
        let start = std::time::SystemTime::now();
        let timestamp_ms = start.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards").as_millis() as u64;
        
        let nakshatra = crate::shiva::Nakshatra::current_from_time(timestamp_ms);
        let mut dict = std::collections::HashMap::new();
        dict.insert("नाम".to_string(), Value::Str(nakshatra.sanskrit_name().to_string()));
        dict.insert("सूचकाङ्क".to_string(), Value::Integer(nakshatra as u32 as i64));
        dict.insert("तरङ्ग".to_string(), Value::Float(nakshatra.frequency()));
        Ok(Value::Dict(dict))
    }

    /// √nāda — नाद (Resonance Frequency of Data)
    pub async fn builtin_nada(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√nāda requires data".into()));
        }
        let input = match &params[0] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };
        // Simple resonance calculation: sum of bytes modulo 27 + 1
        let sum: usize = input.as_bytes().iter().map(|&b| b as usize).sum();
        let resonance = (sum % 27) + 1;
        Ok(Value::Integer(resonance as i64))
    }

    /// √tapas — तपस् (Astromechanical Quantum Proof of Work)
    pub async fn builtin_tapas(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 4 {
            return Err(RuntimeError::General("√tapas requires: block_data, private_key_hex, block_height, nakshatra_index".into()));
        }

        let block_data = match &params[0] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };

        let private_key_hex = match &params[1] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("private_key_hex must be string".into())),
        };

        let block_height = match &params[2] {
            Value::Integer(n) => *n as u64,
            _ => return Err(RuntimeError::TypeError("block_height must be integer".into())),
        };

        let nakshatra_index = match &params[3] {
            Value::Integer(n) => *n as u64,
            _ => return Err(RuntimeError::TypeError("nakshatra_index must be integer".into())),
        };

        // Offload heavy computation to blocking thread
        let result = tokio::task::spawn_blocking(move || {
            use crate::network::quantum::QuantumSigner;
            use sha2::{Sha256, Digest};
            
            let pk_bytes = match hex::decode(&private_key_hex) {
                Ok(b) => b,
                Err(_) => return Err("Invalid hex private key".to_string()),
            };

            let base_difficulty = block_height / 108;
            let cosmic_modifier = (13i64 - (nakshatra_index as i64)).abs() as u64;
            let total_difficulty = base_difficulty + (cosmic_modifier / 3);
            
            // To prevent hanging forever during demo, cap the difficulty artificially
            let effective_difficulty = std::cmp::min(total_difficulty, 2); // Cap at 2 hex zeros for quick testing, but scales logically.
            let target_prefix = "0".repeat(effective_difficulty as usize);

            let mut nonce: u64 = 0;
            let max_attempts = 50000; // Circuit breaker

            loop {
                let payload = format!("{}_{}_{}", block_data, nonce, nakshatra_index);
                let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                
                // Quantum Sign the payload (Extremely heavy)
                if let Ok(signature) = QuantumSigner::sign(&pk_bytes, payload.as_bytes(), current_time) {
                    let mut hasher = Sha256::new();
                    hasher.update(&signature);
                    let hash_result = hex::encode(hasher.finalize());

                    if hash_result.starts_with(&target_prefix) {
                        return Ok((nonce, hash_result, hex::encode(signature)));
                    }
                }

                nonce += 1;
                if nonce > max_attempts {
                    return Err("Mining failed: Resonance not found within limits".to_string());
                }
            }
        }).await.unwrap();

        match result {
            Ok((nonce, hash, sig)) => {
                let mut dict = std::collections::HashMap::new();
                dict.insert("अङ्".to_string(), Value::Integer(nonce as i64)); // nonce
                dict.insert("मुद्रा".to_string(), Value::Str(hash)); // hash
                dict.insert("हस्ताक्षर".to_string(), Value::Str(sig)); // signature
                Ok(Value::Dict(dict))
            }
            Err(_) => Ok(Value::Shunya),
        }
    }

    /// √setu — EVM Setu Bridge
    pub async fn builtin_setu(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√setu requires EVM hex payload".into()));
        }
        
        let hex_input = match &params[0] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };
        
        match crate::shiva::EvmBridge::solidity_to_sutra(&hex_input) {
            Ok(resonance) => {
                let mut dict = std::collections::HashMap::new();
                dict.insert("मूल_दत्तांश".to_string(), Value::Str(resonance.original_hex));
                dict.insert("तरङ्ग".to_string(), Value::Integer(resonance.frequency as i64));
                dict.insert("नक्षत्र_सूचकाङ्क".to_string(), Value::Integer(resonance.nakshatra_index as i64));
                dict.insert("समस्वर".to_string(), Value::Tattva(
                    if resonance.is_resonant { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat }
                ));
                Ok(Value::Dict(dict))
            }
            Err(e) => Err(RuntimeError::General(format!("Bridge translation failed: {}", e))),
        }
    }

    /// √mukh — Starts With check
    pub async fn builtin_mukh(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 2 {
            return Err(RuntimeError::General("√mukh requires text and prefix".into()));
        }
        if let (Value::Str(text), Value::Str(prefix)) = (&params[0], &params[1]) {
            Ok(Value::Tattva(if text.starts_with(prefix) { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat }))
        } else {
            Err(RuntimeError::TypeError("√mukh requires two string values".into()))
        }
    }

    /// √mārg — Range generator for loops
    pub async fn builtin_marg(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if let Some(Value::Integer(end)) = params.first() {
            let list = (0..*end).map(|i| Value::Integer(i)).collect();
            Ok(Value::List(list))
        } else {
            Ok(Value::Shunya)
        }
    }
    /// √kuñc — Key Generation (Post-Quantum Dilithium5)
    pub async fn builtin_kunc(&mut self, _params: &[Value]) -> Result<Value, RuntimeError> {
        let (pk, sk) = crate::network::quantum::QuantumSigner::generate_keypair();

        let private_hex = hex::encode(sk);
        let public_hex = hex::encode(pk);

        let mut dict = std::collections::HashMap::new();
        dict.insert("गुप्तकुञ्चिका".to_string(), Value::Str(private_hex));
        dict.insert("प्रकटकुञ्चिका".to_string(), Value::Str(public_hex));
        Ok(Value::Dict(dict))
    }

    /// √cihna — Signing (Post-Quantum Dilithium5 with Nakshatra Entanglement)
    pub async fn builtin_cihna(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 3 {
            return Err(RuntimeError::General("√cihna requires private key, data, and timestamp".into()));
        }

        let private_hex = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Private key must be text".into())),
        };
        let data = match &params[1] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };
        let timestamp = match &params[2] {
            Value::Integer(t) => *t as u64,
            _ => return Err(RuntimeError::TypeError("Timestamp must be an integer".into())),
        };

        let key_bytes = hex::decode(&private_hex)
            .map_err(|e| RuntimeError::General(format!("Invalid key hex: {}", e)))?;
            
        let signature = crate::network::quantum::QuantumSigner::sign(&key_bytes, data.as_bytes(), timestamp)
            .map_err(|e| RuntimeError::General(e))?;
            
        Ok(Value::Str(hex::encode(signature)))
    }

    /// √parīkṣ — Verification (Post-Quantum Dilithium5 with Nakshatra Entanglement)
    pub async fn builtin_pariksh(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 4 {
            return Err(RuntimeError::General("√parīkṣ requires public key, signature, data, and timestamp".into()));
        }

        let public_hex = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Public key must be text".into())),
        };
        let sig_hex = match &params[1] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Signature must be text".into())),
        };
        let data = match &params[2] {
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };
        let timestamp = match &params[3] {
            Value::Integer(t) => *t as u64,
            _ => return Err(RuntimeError::TypeError("Timestamp must be an integer".into())),
        };

        let pub_bytes = hex::decode(&public_hex)
            .map_err(|e| RuntimeError::General(format!("Invalid public key hex: {}", e)))?;
            
        let sig_bytes = hex::decode(&sig_hex)
            .map_err(|e| RuntimeError::General(format!("Invalid signature hex: {}", e)))?;

        let is_valid = crate::network::quantum::QuantumSigner::verify(&pub_bytes, &sig_bytes, data.as_bytes(), timestamp);
        
        Ok(Value::Tattva(if is_valid { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat }))
    }

    /// √saṁvad — संवाद (HTTP POST to peer for gossip)
    pub async fn builtin_samvad(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 2 {
            return Err(RuntimeError::General("√saṁvad requires URL and data".into()));
        }
        let url = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("URL must be text".into())),
        };
        let body = match &params[1] {
            Value::Dict(d) => {
                let json = crate::evaluator::builtins::value_to_json(&Value::Dict(d.clone()));
                serde_json::to_string(&json).unwrap_or_default()
            },
            Value::Str(s) => s.clone(),
            v => format!("{}", v),
        };

        let client = reqwest::Client::new();
        match client.post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await {
            Ok(res) => {
                let status = res.status().as_u16();
                let text = res.text().await.unwrap_or_default();
                let mut dict = std::collections::HashMap::new();
                dict.insert("स्थिति".to_string(), Value::Integer(status as i64));
                dict.insert("देह".to_string(), Value::Str(text));
                Ok(Value::Dict(dict))
            },
            Err(e) => Err(RuntimeError::General(format!("संवाद विफल: {}", e))),
        }
    }

    /// √yant — यन्त्र (Sutra Yantra - Dynamic Execution)
    pub async fn builtin_yant(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√yant requires code to execute".into()));
        }
        let code = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Contract code must be text".into())),
        };

        // --- PHASE 12 PATCH: Contract Size Limit ---
        // Prevent parser memory exhaustion from massive dynamic contracts
        if code.len() > 262_144 { // 256KB
            return Err(RuntimeError::General(format!("Contract code too large: {} bytes exceeds 256KB limit", code.len())));
        }

        let mut scanner = crate::lexer::Scanner::new(&code);
        let tokens = scanner.scan_tokens();
        let mut parser = crate::parser::SutraParser::new(tokens);
        
        match parser.parse() {
            Ok(prog) => {
                self.env.push_scope();
                
                // Snapshot Security Context
                let previous_sandbox = self.is_sandboxed;
                let previous_gas_limit = self.resonance_limit;
                
                // Enforce strict VM rules
                self.is_sandboxed = true;
                self.resonance_limit = Some(100_000); // 100k Gas limit for contracts
                
                // Inject Context (e.g. sender, contract address, call data)
                if params.len() > 1 {
                    if let Value::Dict(ctx) = &params[1] {
                        for (k, v) in ctx {
                            self.env.define(k.clone(), v.clone());
                        }
                    }
                }

                // Inject Contract Address into environment specifically for storage roots
                if let Some(Value::Str(_addr)) = self.env.get("यन्त्र_पता") {
                    // It's already in the context
                } else {
                    return Err(RuntimeError::General("निष्पादनसन्दर्भे 'यन्त्र_पता' (सन्धिपत्रसङ्केतः) आवश्यकम् अस्ति".into()));
                }

                let val = match self.execute(&prog).await {
                    Ok(v) => v,
                    Err(e) => {
                        self.env.pop_scope();
                        self.is_sandboxed = previous_sandbox;
                        self.resonance_limit = previous_gas_limit;
                        return Err(e);
                    }
                };
                self.env.pop_scope();
                self.is_sandboxed = previous_sandbox;
                self.resonance_limit = previous_gas_limit;
                Ok(val)
            }
            Err(e) => Err(RuntimeError::General(format!("Contract parse error: {:?}", e))),
        }
    }

    /// √sañci — सञ्चि (Store in Contract Storage)
    pub async fn builtin_sanci(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 2 {
            return Err(RuntimeError::General("√sañci requires key and value".into()));
        }
        let key = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Key must be text".into())),
        };
        // --- PHASE 12 PATCH: Key Length Validation ---
        if key.len() > 512 {
            return Err(RuntimeError::General("Contract storage key too long: maximum 512 bytes.".into()));
        }
        let value = &params[1];

        // --- THE DEEP DIVE PATCH #8: Storage Payload Bloat Protection ---
        let serialized_value = serde_json::to_string(&crate::evaluator::builtins::value_to_json(value)).unwrap_or_default();
        if serialized_value.len() > 65536 {
            return Err(RuntimeError::General("🛑 SANDBOX: Storage payload exceeds 64KB limit. State bloat prevented.".into()));
        }

        let contract_addr = match self.env.get("यन्त्र_पता") {
            Some(Value::Str(addr)) => addr,
            _ => return Err(RuntimeError::General("सन्धिपत्रसन्दर्भः नास्ति (यन्त्र_पता लुप्तम्)".into())),
        };

        let db_key = format!("भण्डार_{}_{}", contract_addr, key);
        let mut db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
        db.store(&db_key, value);

        Ok(Value::Tattva(crate::evaluator::TattvaState::Sat))
    }

    /// √labh — लभ् (Retrieve from Contract Storage)
    pub async fn builtin_labh(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.is_empty() {
            return Err(RuntimeError::General("√labh requires key".into()));
        }
        let key = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Key must be text".into())),
        };
        // --- PHASE 12 PATCH: Key Length Validation ---
        if key.len() > 512 {
            return Err(RuntimeError::General("Contract storage key too long: maximum 512 bytes.".into()));
        }

        let contract_addr = match self.env.get("यन्त्र_पता") {
            Some(Value::Str(addr)) => addr,
            _ => return Err(RuntimeError::General("सन्धिपत्रसन्दर्भः नास्ति (यन्त्र_पता लुप्तम्)".into())),
        };

        let db_key = format!("भण्डार_{}_{}", contract_addr, key);
        let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
        
        if let Some(val) = db.retrieve(&db_key) {
            Ok(val)
        } else {
            Ok(Value::Shunya)
        }
    }

    /// √ghaṭ — घट् (Emit Event)
    pub async fn builtin_ghat(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        if params.len() < 2 {
            return Err(RuntimeError::General("√ghaṭ requires topic and data".into()));
        }
        let topic = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Topic must be text".into())),
        };
        let data = &params[1];

        let contract_addr = match self.env.get("यन्त्र_पता") {
            Some(Value::Str(addr)) => addr,
            _ => return Err(RuntimeError::General("सन्धिपत्रसन्दर्भः नास्ति (यन्त्र_पता लुप्तम्)".into())),
        };

        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;

        let event = Value::Dict(std::collections::HashMap::from([
            ("सन्धिपत्र_पता".to_string(), Value::Str(contract_addr.clone())),
            ("विषय".to_string(), Value::Str(topic)),
            ("दत्तांश".to_string(), data.clone()),
            ("काल".to_string(), Value::Integer(time)),
        ]));

        let mut db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
        
        let events_key = "घटना_सूची".to_string();
        let mut events_list = match db.retrieve(&events_key) {
            Some(Value::List(l)) => l,
            _ => Vec::new(),
        };
        // --- PHASE 12 PATCH: Event Log Overflow Protection ---
        // Cap at 10,000 events to prevent unbounded database growth from spamming contracts
        if events_list.len() >= 10_000 {
            return Err(RuntimeError::General("🛑 Event log overflow: maximum 10,000 events reached. Prune old events first.".into()));
        }
        events_list.push(event);
        db.store(&events_key, &Value::List(events_list));

        Ok(Value::Tattva(crate::evaluator::TattvaState::Sat))
    }
    /// √smṛ — स्मृ (Store in Mandala)
    pub async fn builtin_smr(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        // --- PHASE 12 PATCH: Block global DB access from sandbox ---
        if self.is_sandboxed {
            return Err(RuntimeError::General("🛑 SANDBOX: √smṛ (global DB write) is FORBIDDEN in contracts. Use √sañci for contract-scoped storage.".into()));
        }
        if params.len() < 2 {
            return Err(RuntimeError::General("√smṛ requires key and value".into()));
        }
        let key = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Key must be text".into())),
        };
        // --- PHASE 12 PATCH: Key length validation ---
        if key.len() > 512 {
            return Err(RuntimeError::General("Key too long: maximum 512 bytes.".into()));
        }
        let value = &params[1];

        let mut db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
        db.store(&key, value);

        Ok(Value::Tattva(crate::evaluator::TattvaState::Sat))
    }

    /// √dṛś — दृश् (Retrieve from Mandala)
    pub async fn builtin_drs(&mut self, params: &[Value]) -> Result<Value, RuntimeError> {
        // --- PHASE 12 PATCH: Block global DB access from sandbox ---
        if self.is_sandboxed {
            return Err(RuntimeError::General("🛑 SANDBOX: √dṛś (global DB read) is FORBIDDEN in contracts. Use √labh for contract-scoped storage.".into()));
        }
        if params.is_empty() {
            return Err(RuntimeError::General("√dṛś requires key".into()));
        }
        let key = match &params[0] {
            Value::Str(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Key must be text".into())),
        };

        let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
        if let Some(val) = db.retrieve(&key) {
            Ok(val)
        } else {
            Ok(Value::Shunya)
        }
    }
}

impl Default for Engine {
    fn default() -> Self { Self::new() }
}

/// Runtime Errors
#[derive(Debug, Clone)]
pub enum RuntimeError {
    UndefinedVariable(String),
    TypeError(String),
    DivisionByZero,
    General(String),
    /// Control flow: early return from function
    ReturnValue(Value),
    /// Gas/Resonance Limit Exhausted
    ResonanceExhausted(usize, usize),
}

impl RuntimeError {
    pub fn to_diagnostic(&self) -> crate::error::Diagnostic {
        use crate::error::Diagnostic;
        match self {
            RuntimeError::UndefinedVariable(n) => Diagnostic::error(format!("अपरिभाषित '{}' (undefined variable)", n))
                .with_hint(format!("Variable '{}' not found. Check Anuvritti scope or define it via √sṛj", n)),
            RuntimeError::TypeError(msg) => Diagnostic::error(format!("प्रकार '{}' (type error)", msg))
                .with_hint("Operation is incompatible with the current data type"),
            RuntimeError::DivisionByZero => Diagnostic::error("शून्यभाजन (division by zero)")
                .with_hint("Division by zero is undefined"),
            RuntimeError::General(msg) => Diagnostic::error(msg.clone()),
            RuntimeError::ReturnValue(_) => Diagnostic::error("प्रतिदा outside function context"),
            RuntimeError::ResonanceExhausted(used, limit) => Diagnostic::error(format!("प्राणः समाप्तः (उपयोगः: {}, सीमा: {})", used, limit))
                .with_hint("अस्य कार्यस्य कृते अधिकप्राणस्य (Gas) आवश्यकता अस्ति। स्वस्य सन्धिपत्रम् अनुकूलं कुरुतु।"),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_diagnostic().message)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::*;

    fn eval_expr(expr: Expr) -> Result<Value, RuntimeError> {
        let mut engine = Evaluator::new();
        engine.evaluate_expression(&expr)
    }

    fn eval_stmt(stmt: Statement) -> Result<(), RuntimeError> {
        let mut engine = Evaluator::new();
        engine.evaluate_statement(&stmt)
    }

    #[test]
    fn test_eval_literal_int() {
        let expr = Expr::Literal(Literal::Integer(42));
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(42));
    }

    #[test]
    fn test_eval_literal_float() {
        let expr = Expr::Literal(Literal::Float(3.14));
        assert_eq!(eval_expr(expr).unwrap(), Value::Float(3.14));
    }

    #[test]
    fn test_eval_literal_str() {
        let expr = Expr::Literal(Literal::Str("sanskrit".to_string()));
        assert_eq!(eval_expr(expr).unwrap(), Value::Str("sanskrit".to_string()));
    }

    #[test]
    fn test_eval_literal_bool() {
        let expr = Expr::Literal(Literal::Boolean(true));
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_literal_null() {
        let expr = Expr::Literal(Literal::Null);
        assert_eq!(eval_expr(expr).unwrap(), Value::Null);
    }

    #[test]
    fn test_eval_addition() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Integer(20))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(30));
    }

    #[test]
    fn test_eval_subtraction() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(50))),
            operator: BinaryOp::Subtract,
            right: Box::new(Expr::Literal(Literal::Integer(20))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(30));
    }

    #[test]
    fn test_eval_multiplication() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(5))),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Literal(Literal::Integer(6))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(30));
    }

    #[test]
    fn test_eval_division() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(100))),
            operator: BinaryOp::Divide,
            right: Box::new(Expr::Literal(Literal::Integer(4))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(25));
    }

    #[test]
    fn test_eval_division_by_zero() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(100))),
            operator: BinaryOp::Divide,
            right: Box::new(Expr::Literal(Literal::Integer(0))),
        };
        assert!(eval_expr(expr).is_err());
    }

    #[test]
    fn test_eval_equality() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(100))),
            operator: BinaryOp::Equal,
            right: Box::new(Expr::Literal(Literal::Integer(100))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_inequality() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(100))),
            operator: BinaryOp::NotEqual,
            right: Box::new(Expr::Literal(Literal::Integer(50))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_greater_than() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(100))),
            operator: BinaryOp::Greater,
            right: Box::new(Expr::Literal(Literal::Integer(50))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_less_than() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Integer(50))),
            operator: BinaryOp::Less,
            right: Box::new(Expr::Literal(Literal::Integer(100))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_unary_negate() {
        let expr = Expr::Unary {
            operator: UnaryOp::Negate,
            operand: Box::new(Expr::Literal(Literal::Integer(50))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Integer(-50));
    }

    #[test]
    fn test_eval_unary_not() {
        let expr = Expr::Unary {
            operator: UnaryOp::Not,
            operand: Box::new(Expr::Literal(Literal::Boolean(true))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_eval_logical_and() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Boolean(true))),
            operator: BinaryOp::And,
            right: Box::new(Expr::Literal(Literal::Boolean(false))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_eval_logical_or() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Boolean(true))),
            operator: BinaryOp::Or,
            right: Box::new(Expr::Literal(Literal::Boolean(false))),
        };
        assert_eq!(eval_expr(expr).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_array_literal() {
        let expr = Expr::Array(vec![
            Expr::Literal(Literal::Integer(1)),
            Expr::Literal(Literal::Integer(2)),
            Expr::Literal(Literal::Integer(3)),
        ]);
        let val = eval_expr(expr).unwrap();
        if let Value::Array(items) = val {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].lock().unwrap().clone(), Value::Integer(1));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_eval_dict_literal() {
        let expr = Expr::Dict(vec![
            (Expr::Literal(Literal::Str("key1".to_string())), Expr::Literal(Literal::Integer(42))),
        ]);
        let val = eval_expr(expr).unwrap();
        if let Value::Map(map) = val {
            let inner = map.lock().unwrap();
            assert_eq!(inner.len(), 1);
            let k = Value::Str("key1".to_string());
            assert_eq!(inner.get(&k).unwrap().clone(), Value::Integer(42));
        } else {
            panic!("Expected dictionary");
        }
    }

    #[test]
    fn test_variable_declaration_and_access() {
        let mut engine = Evaluator::new();
        let decl = Statement::VarDecl {
            name: "test_var".to_string(),
            type_hint: None,
            initializer: Some(Expr::Literal(Literal::Integer(999))),
        };
        engine.evaluate_statement(&decl).unwrap();

        let access = Expr::Identifier("test_var".to_string());
        assert_eq!(engine.evaluate_expression(&access).unwrap(), Value::Integer(999));
    }

    #[test]
    fn test_variable_reassignment() {
        let mut engine = Evaluator::new();
        let decl = Statement::VarDecl {
            name: "test_var".to_string(),
            type_hint: None,
            initializer: Some(Expr::Literal(Literal::Integer(100))),
        };
        engine.evaluate_statement(&decl).unwrap();

        let assign = Expr::Assignment {
            name: "test_var".to_string(),
            value: Box::new(Expr::Literal(Literal::Integer(200))),
        };
        engine.evaluate_expression(&assign).unwrap();

        let access = Expr::Identifier("test_var".to_string());
        assert_eq!(engine.evaluate_expression(&access).unwrap(), Value::Integer(200));
    }

    #[test]
    fn test_undefined_variable_access_fails() {
        let expr = Expr::Identifier("unknown_var".to_string());
        assert!(eval_expr(expr).is_err());
    }

    #[test]
    fn test_if_statement_true_branch() {
        let mut engine = Evaluator::new();
        let stmt = Statement::If {
            condition: Expr::Literal(Literal::Boolean(true)),
            then_branch: Box::new(Statement::Block(vec![
                Statement::VarDecl {
                    name: "result".to_string(),
                    type_hint: None,
                    initializer: Some(Expr::Literal(Literal::Integer(1))),
                }
            ])),
            else_branch: None,
        };
        engine.evaluate_statement(&stmt).unwrap();
        // Since block scope drops variables, we need to declare it outside first to test properly
    }

    #[test]
    fn test_while_loop() {
        let mut engine = Evaluator::new();
        // i = 0
        engine.evaluate_statement(&Statement::VarDecl {
            name: "i".to_string(),
            type_hint: None,
            initializer: Some(Expr::Literal(Literal::Integer(0))),
        }).unwrap();

        // while i < 3 { i = i + 1 }
        let stmt = Statement::While {
            condition: Expr::Binary {
                left: Box::new(Expr::Identifier("i".to_string())),
                operator: BinaryOp::Less,
                right: Box::new(Expr::Literal(Literal::Integer(3))),
            },
            body: Box::new(Statement::Block(vec![
                Statement::Expression(Expr::Assignment {
                    name: "i".to_string(),
                    value: Box::new(Expr::Binary {
                        left: Box::new(Expr::Identifier("i".to_string())),
                        operator: BinaryOp::Add,
                        right: Box::new(Expr::Literal(Literal::Integer(1))),
                    }),
                })
            ])),
        };
        engine.evaluate_statement(&stmt).unwrap();

        assert_eq!(engine.evaluate_expression(&Expr::Identifier("i".to_string())).unwrap(), Value::Integer(3));
    }
}
