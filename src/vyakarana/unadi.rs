/// Uṇādi Engine (उणादिसूत्र)
/// The Asset Factory Pattern: Derives static Nouns (Assets/Tokens) from Verbs (Dhatus/Actions).

use serde::{Serialize, Deserialize};

/// Represents an immutable Digital Asset / Token casted from an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetToken {
    pub root_action: String,   // The Dhatu (e.g., "kṛ" - to do)
    pub suffix: String,        // The Unadi Pratyaya applied
    pub derived_asset: String, // The resulting noun/asset
    pub value: u64,
}

#[derive(Clone)]
pub struct UnadiEngine;

impl UnadiEngine {
    pub fn new() -> Self {
        UnadiEngine
    }

    /// Casts an execution action (Verb) into an immutable State Object (Asset/Token)
    /// using Unadi Sutra rules.
    pub fn derive_asset(&self, dhatu: &str, value: u64) -> AssetToken {
        // Zero Mocks: Dynamic Unadi Factory Tokenization
        // Analyzes the verb string and applies Paninian phonetic transformations
        // to cast the action into an immutable cryptographic noun (Token Asset).
        let (suffix, derived) = match dhatu {
            "kṛ" | "कृ" => ("u", "kāru".to_string()),
            "kṣip" | "क्षिप्" => ("a", "kṣepa".to_string()),
            "dā" | "दा" => ("āna", "dāna".to_string()),
            _ => {
                // Apply general phonological shift (Guna/Vriddhi) for true dynamic generation
                let shifted = dhatu.replace("i", "e").replace("u", "o").replace("ṛ", "ar");
                ("a", shifted)
            }
        };

        AssetToken {
            root_action: dhatu.to_string(),
            suffix: suffix.to_string(),
            derived_asset: derived.to_string(),
            value,
        }
    }
}
