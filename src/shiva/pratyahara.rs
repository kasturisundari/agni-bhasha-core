/// # Pratyahara System — State Abbreviation System
///
/// Similar to Panini's Pratyaharas for sound abbreviations (e.g. "aC" for vowels),
/// this module defines state abbreviation collections.
///
/// Rule: First sound of first state + Anubandha suffix of last state = Pratyahara name.

use super::sutras::ShivaState;
use std::collections::HashMap;

/// Single Pratyahara definition
#[derive(Debug, Clone, PartialEq)]
pub struct Pratyahara {
    /// Short name (e.g. "aC", "haL")
    pub name: String,
    /// Included states
    pub states: Vec<ShivaState>,
    /// Collection description
    pub description: String,
}

/// Pratyahara Registry holding all shorthand symbols
pub struct PratyaharaRegistry {
    entries: HashMap<String, Pratyahara>,
}

impl PratyaharaRegistry {
    /// Create registry containing builtin rules
    pub fn new() -> Self {
        let mut registry = Self {
            entries: HashMap::new(),
        };
        registry.register_builtins();
        registry
    }

    /// Register builtin core Pratyaharas
    fn register_builtins(&mut self) {
        // aC = Creation -> Expansion (Primitive operations)
        self.register(Pratyahara {
            name: "aC".to_string(),
            states: vec![
                ShivaState::Creation,
                ShivaState::Flow,
                ShivaState::Harmony,
                ShivaState::Expansion,
            ],
            description: "Primitive operations: creation, flow, harmony, expansion".to_string(),
        });

        // haL = Motion -> Dissolution (Executive operations)
        self.register(Pratyahara {
            name: "haL".to_string(),
            states: vec![
                ShivaState::Motion,
                ShivaState::Binding,
                ShivaState::Resonance,
                ShivaState::Force,
                ShivaState::Transform,
                ShivaState::Structure,
                ShivaState::Protection,
                ShivaState::Causation,
                ShivaState::Purification,
                ShivaState::Dissolution,
            ],
            description: "Executive operations: motion through dissolution".to_string(),
        });

        // aL = Creation -> Dissolution (Complete system)
        self.register(Pratyahara {
            name: "aL".to_string(),
            states: ShivaState::all().to_vec(),
            description: "Complete system: all 14 states".to_string(),
        });

        // aK = Creation -> Flow (Primordial origins)
        self.register(Pratyahara {
            name: "aK".to_string(),
            states: vec![ShivaState::Creation, ShivaState::Flow],
            description: "Primordial origins: creation and flow".to_string(),
        });

        // eṄ = Harmony only (Unification)
        self.register(Pratyahara {
            name: "eṄ".to_string(),
            states: vec![ShivaState::Harmony],
            description: "Unification: harmony operations only".to_string(),
        });

        // haṬ = Motion only (Pure motion)
        self.register(Pratyahara {
            name: "haṬ".to_string(),
            states: vec![ShivaState::Motion],
            description: "Pure motion: transfer and movement".to_string(),
        });

        // yaR = Motion -> Purification (Intermediate operations)
        self.register(Pratyahara {
            name: "yaR".to_string(),
            states: vec![
                ShivaState::Motion,
                ShivaState::Binding,
                ShivaState::Resonance,
                ShivaState::Force,
                ShivaState::Transform,
                ShivaState::Structure,
                ShivaState::Protection,
                ShivaState::Causation,
                ShivaState::Purification,
            ],
            description: "Intermediate operations: motion through purification".to_string(),
        });

        // jaŚ = Structure only (Pure structure)
        self.register(Pratyahara {
            name: "jaŚ".to_string(),
            states: vec![ShivaState::Structure],
            description: "Pure structure: definition and organization".to_string(),
        });

        // khaV = Protection only (Pure protection)
        self.register(Pratyahara {
            name: "khaV".to_string(),
            states: vec![ShivaState::Protection],
            description: "Pure protection: validation and guarding".to_string(),
        });
    }

    /// Register a new Pratyahara
    pub fn register(&mut self, p: Pratyahara) {
        self.entries.insert(p.name.clone(), p);
    }

    /// Find Pratyahara by name
    pub fn lookup(&self, name: &str) -> Option<&Pratyahara> {
        self.entries.get(name)
    }

    /// Construct dynamic Pratyahara range from start to end state
    pub fn create_range(&self, from: ShivaState, to: ShivaState) -> Pratyahara {
        let from_idx = from.index();
        let to_idx = to.index();

        let states: Vec<ShivaState> = if from_idx <= to_idx {
            (from_idx..=to_idx)
                .filter_map(ShivaState::from_index)
                .collect()
        } else {
            Vec::new()
        };

        let name = format!(
            "{}{}",
            from.phonemes().first().unwrap_or(&"?"),
            to.anubandha()
        );

        Pratyahara {
            name,
            states,
            description: format!("Dynamic range: {:?} → {:?}", from, to),
        }
    }

    /// Check if target state belongs to named Pratyahara
    pub fn state_in_pratyahara(&self, state: ShivaState, pratyahara_name: &str) -> bool {
        self.entries
            .get(pratyahara_name)
            .map(|p| p.states.contains(&state))
            .unwrap_or(false)
    }

    /// Retrieve all registered names
    pub fn all_names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PratyaharaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ac_contains_first_four() {
        let reg = PratyaharaRegistry::new();
        let ac = reg.lookup("aC").unwrap();
        assert_eq!(ac.states.len(), 4);
        assert!(ac.states.contains(&ShivaState::Creation));
        assert!(ac.states.contains(&ShivaState::Expansion));
        assert!(!ac.states.contains(&ShivaState::Motion));
    }

    #[test]
    fn test_al_contains_all() {
        let reg = PratyaharaRegistry::new();
        let al = reg.lookup("aL").unwrap();
        assert_eq!(al.states.len(), 14);
    }

    #[test]
    fn test_dynamic_range() {
        let reg = PratyaharaRegistry::new();
        let range = reg.create_range(ShivaState::Force, ShivaState::Structure);
        assert_eq!(range.states.len(), 3);
        assert!(range.states.contains(&ShivaState::Force));
        assert!(range.states.contains(&ShivaState::Transform));
        assert!(range.states.contains(&ShivaState::Structure));
    }
}
