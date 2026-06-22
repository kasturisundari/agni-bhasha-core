/// #   — Scope Manager (Anuvritti)
///
///      .

use std::collections::HashMap;

///    
#[derive(Debug, Clone)]
pub enum ScopeValue {
    String(String),
    Integer(i64),
    Float(f64),
    Tattva(crate::evaluator::TattvaState),
    List(Vec<ScopeValue>),
    Null,
}

///     
#[derive(Debug, Clone)]
pub struct Scope {
    ///  
    pub name: String,
    ///  
    pub kind: ScopeKind,
    ///  
    pub values: HashMap<String, ScopeValue>,
    ///   ( adhikāra )
    pub inherited_contexts: Vec<String>,
}

///   (   Anuvritti)
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    ///  —  
    Adhikara,
    ///  —  
    Prakarana,
    ///  —  
    Sutra,
    ///  
    General,
}

impl Scope {
    pub fn new(name: impl Into<String>, kind: ScopeKind) -> Self {
        Self {
            name: name.into(),
            kind,
            values: HashMap::new(),
            inherited_contexts: Vec::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: ScopeValue) {
        self.values.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&ScopeValue> {
        self.values.get(key)
    }

    pub fn add_context(&mut self, ctx: impl Into<String>) {
        self.inherited_contexts.push(ctx.into());
    }

    pub fn has_context(&self, ctx: &str) -> bool {
        self.inherited_contexts.iter().any(|c| c == ctx)
    }
}
