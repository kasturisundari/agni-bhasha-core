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
        // Mocking the Unadi factory logic.
        // E.g., The root 'kṛ' (to do/make) + 'u' suffix = 'kāru' (artisan/maker token).
        // For blockchain, 'kṣip' (to send) + Unadi = 'Kṣepa' (Transaction Receipt Asset).
        
        let (suffix, derived) = match dhatu {
            "kṛ" | "कृ" => ("u", "kāru"),
            "kṣip" | "क्षिप्" => ("a", "kṣepa"),
            "dā" | "दा" => ("āna", "dāna"),
            _ => ("a", dhatu), // Default transformation
        };

        AssetToken {
            root_action: dhatu.to_string(),
            suffix: suffix.to_string(),
            derived_asset: derived.to_string(),
            value,
        }
    }
}
