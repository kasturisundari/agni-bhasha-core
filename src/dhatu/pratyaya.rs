/// # Pratyaya Suffix Engine
/// Radha Krishna Language

use std::collections::HashMap;

/// Single Pratyaya Suffix
#[derive(Debug, Clone)]
pub struct PratyayaSuffix {
    pub name: String,
    pub devanagari: String,
    pub meaning: String,
    pub computational_effect: PratyayaEffect,
    pub description: String,
}

/// Computational effect of applying a suffix
#[derive(Debug, Clone, PartialEq)]
pub enum PratyayaEffect {
    SyncExecute,
    AsyncExecute,
    MustExecute,
    Optional,
    Interface,
    Collection,
    Stream,
    Completed,
    Callback,
    Event,
    Owner,
    TypeOf,
}

/// Pratyaya registry
#[derive(Clone)]
pub struct PratyayaRegistry {
    suffixes: HashMap<String, PratyayaSuffix>,
}

impl PratyayaRegistry {
    pub fn new() -> Self {
        let mut r = Self { suffixes: HashMap::new() };
        r.register_builtins();
        r
    }

    fn register_builtins(&mut self) {
        let items = vec![
            ("ति", "ति", "Sync Execution", PratyayaEffect::SyncExecute, "Sync execution"),
            ("स्यति", "स्यति", "Async Execution", PratyayaEffect::AsyncExecute, "Async/promise"),
            ("तव्य", "तव्य", "Required Execution", PratyayaEffect::MustExecute, "Must execute"),
            ("शक्य", "शक्य", "Optional Option", PratyayaEffect::Optional, "Optional/nullable"),
            ("त्र", "त्र", "Interface / Adapter", PratyayaEffect::Interface, "Interface/tool"),
            ("आः", "आः", "Collection / Array", PratyayaEffect::Collection, "Collection/array"),
            ("मान", "मान", "Stream / Flowing", PratyayaEffect::Stream, "Stream/ongoing"),
            ("क्त", "क्त", "Completed / Output", PratyayaEffect::Completed, "Completed/result"),
            ("तुम्", "तुम्", "Callback / Callable", PratyayaEffect::Callback, "Callback"),
            ("णम्", "णम्", "Action / Event", PratyayaEffect::Event, "Event/action"),
            ("इन्", "इन्", "Owner / Holder", PratyayaEffect::Owner, "Owner"),
            ("त्व", "त्व", "Identity / Type", PratyayaEffect::TypeOf, "TypeOf"),
        ];

        for (name, dev, mean, effect, desc) in items {
            self.suffixes.insert(name.to_string(), PratyayaSuffix {
                name: name.into(),
                devanagari: dev.into(),
                meaning: mean.into(),
                computational_effect: effect,
                description: desc.into(),
            });
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&PratyayaSuffix> {
        self.suffixes.get(name)
    }

    pub fn all(&self) -> Vec<&PratyayaSuffix> {
        self.suffixes.values().collect()
    }

    pub fn by_effect(&self, effect: &PratyayaEffect) -> Vec<&PratyayaSuffix> {
        self.suffixes.values().filter(|s| &s.computational_effect == effect).collect()
    }
}

impl Default for PratyayaRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ti() {
        let reg = PratyayaRegistry::new();
        let ti = reg.lookup("ति").unwrap();
        assert_eq!(ti.computational_effect, PratyayaEffect::SyncExecute);
    }

    #[test]
    fn test_all_suffixes() {
        let reg = PratyayaRegistry::new();
        assert!(reg.all().len() >= 10);
    }
}
