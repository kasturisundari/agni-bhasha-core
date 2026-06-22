/// The Ashtadhyayi Engine (अष्टाध्यायी)
/// Represents Panini's 4000 Sutras of Sanskrit Grammar translated into Computational Architecture.
/// This engine will act as the core vyakarana (grammar) resolver for the Sutra Language.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents a single Paninian Sutra (Rule)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaniniSutra {
    pub sutra_id: String,     // e.g. "1.1.1"
    pub original_text: String, // e.g. "वृद्धिरादैच्"
    pub description: String,   // Meaning of the rule
    pub adhyaya: u8,          // Chapter (1-8)
}

/// The Engine orchestrating all 3959 (roughly 4000) Sutras
pub struct AshtadhyayiEngine {
    pub rules: HashMap<String, PaniniSutra>,
}

impl AshtadhyayiEngine {
    pub fn new() -> Self {
        // Compile-time ingestion of all 3959 Cosmic Rules
        let sutras_json = include_str!("sutras.json");
        let rules: HashMap<String, PaniniSutra> = serde_json::from_str(sutras_json)
            .expect("Failed to parse the 3959 Panini Sutras");

        AshtadhyayiEngine { rules }
    }



    /// Retrieve a Sutra by its canonical ID (e.g., "1.1.1")
    pub fn get_sutra(&self, id: &str) -> Option<&PaniniSutra> {
        self.rules.get(id)
    }

    /// Total active rules loaded in the engine
    pub fn total_rules(&self) -> usize {
        self.rules.len()
    }
}
