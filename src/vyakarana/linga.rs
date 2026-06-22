/// Liṅgānuśāsanam Engine (लिङ्गानुशासनम्)
/// The Vedic Strong Type System mapping Grammatical Gender to Memory Security & Permissions.

#[derive(Debug, Clone, PartialEq)]
pub enum LingaType {
    /// Pumlinga (Masculine / Active): High privilege, mutable state, can initiate actions (Actors).
    Pumlinga,
    
    /// Strilinga (Feminine / Receptive): Storage state, data vaults, highly secure memory allocations (Smart Contract Data).
    Strilinga,
    
    /// Napumsakalinga (Neuter / Immutable): Constants, cryptographic hashes, zero-knowledge proofs.
    Napumsakalinga,
}

#[derive(Clone)]
pub struct LinganushasanamEngine;

impl LinganushasanamEngine {
    pub fn new() -> Self {
        LinganushasanamEngine
    }

    /// Determines the grammatical gender (memory permission type) of a variable based on its suffix.
    pub fn determine_type(&self, identifier: &str) -> LingaType {
        // Simplified heuristic based on Sanskrit morphology rules
        
        // Words ending in 'ā' (आ) or 'ī' (ई) are typically Feminine (Vaults/Storage)
        if identifier.ends_with("ā") || identifier.ends_with("ī") || identifier.ends_with("आ") || identifier.ends_with("ई") {
            return LingaType::Strilinga;
        }
        
        // Words ending in 'm' (म्) are typically Neuter (Immutable/Constants)
        if identifier.ends_with("m") || identifier.ends_with("म्") {
            return LingaType::Napumsakalinga;
        }
        
        // Default to Masculine (Active Actors/Pointers)
        LingaType::Pumlinga
    }

    /// Verifies if a given operation is permitted on the specific LingaType
    pub fn verify_permission(&self, linga: &LingaType, is_mutation: bool) -> Result<(), &'static str> {
        match linga {
            LingaType::Pumlinga => Ok(()), // Actors can mutate their own execution state
            LingaType::Strilinga => {
                // Vaults can only be mutated through specific consensus or authorized transactions
                if is_mutation {
                    Err("Security Exception: Cannot directly mutate Strilinga (Vault) memory. Requires consensus transaction.")
                } else {
                    Ok(())
                }
            },
            LingaType::Napumsakalinga => {
                // Constants are strictly immutable
                if is_mutation {
                    Err("Type Error: Napumsakalinga (Immutable) memory cannot be mutated.")
                } else {
                    Ok(())
                }
            }
        }
    }
}
