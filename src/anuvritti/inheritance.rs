/// #   — Context Inheritance (Anuvritti)
///
///       .

use super::scope::{Scope, ScopeKind, ScopeValue};

///   —  
#[derive(Clone)]
pub struct AnuvrittiManager {
    ///   ( = )
    scopes: Vec<Scope>,
}

impl AnuvrittiManager {
    pub fn new() -> Self {
        let global = Scope::new("विश्व", ScopeKind::General); // "" = Global
        Self {
            scopes: vec![global],
        }
    }

    ///    (  )
    pub fn enter_scope(&mut self, name: impl Into<String>, kind: ScopeKind) {
        let inherited = self.current_contexts();
        let mut scope = Scope::new(name, kind);
        scope.inherited_contexts = inherited;
        self.scopes.push(scope);
    }

    ///     
    pub fn enter_scope_with_context(
        &mut self,
        name: impl Into<String>,
        kind: ScopeKind,
        context: impl Into<String>,
    ) {
        let mut inherited = self.current_contexts();
        let ctx = context.into();
        if !inherited.contains(&ctx) {
            inherited.push(ctx);
        }
        let mut scope = Scope::new(name, kind);
        scope.inherited_contexts = inherited;
        self.scopes.push(scope);
    }

    ///    
    pub fn exit_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None //      
        }
    }

    ///     
    pub fn set(&mut self, key: impl Into<String>, value: ScopeValue) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.set(key, value);
        }
    }

    ///    (    — Anuvritti)
    pub fn get(&self, key: &str) -> Option<&ScopeValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(key) {
                return Some(val);
            }
        }
        None
    }

    ///    
    pub fn current_contexts(&self) -> Vec<String> {
        self.scopes
            .last()
            .map(|s| s.inherited_contexts.clone())
            .unwrap_or_default()
    }

    ///     
    pub fn has_context(&self, ctx: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|s| s.has_context(ctx))
    }

    ///   
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }

    ///   
    pub fn current_scope_name(&self) -> &str {
        self.scopes.last().map(|s| s.name.as_str()).unwrap_or("विश्व")
    }
}

impl Default for AnuvrittiManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_inheritance() {
        let mut mgr = AnuvrittiManager::new();
        mgr.enter_scope_with_context("auth", ScopeKind::Adhikara, "JWT");
        assert!(mgr.has_context("JWT"));

        mgr.enter_scope("users", ScopeKind::Prakarana);
        // Prakarana   JWT  Adhikara
        assert!(mgr.has_context("JWT"));

        mgr.exit_scope();
        mgr.exit_scope();
        assert!(!mgr.has_context("JWT"));
    }

    #[test]
    fn test_variable_lookup() {
        let mut mgr = AnuvrittiManager::new();
        mgr.set("x", ScopeValue::Integer(42));
        mgr.enter_scope("inner", ScopeKind::Sutra);
        //      (anuvritti)
        assert!(matches!(mgr.get("x"), Some(ScopeValue::Integer(42))));
    }
}
