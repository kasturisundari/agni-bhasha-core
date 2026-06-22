use crate::shiva::nakshatra::Nakshatra;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// Harmonic Bridge (EVM Setu Bridge)
/// 
/// Translates transactions from EVM networks (Hex format) into
/// harmonic resonance form understood by Vyakarana engine.
pub struct EvmBridge;

impl EvmBridge {
    /// Calculate cosmic resonance frequency for EVM Hex data
    pub fn solidity_to_sutra(hex_payload: &str) -> Result<EvmResonance, String> {
        // Strip 0x hex prefix
        let clean_hex = hex_payload.trim_start_matches("0x");
        
        let bytes = hex::decode(clean_hex)
            .map_err(|e| format!("Invalid hex payload: {}", e))?;
            
        // Calculate the hash (Nada) of the data
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let result = hasher.finalize();
        
        // Convert hash to geometric resonance frequency
        // We take first 8 bytes to construct base frequency
        let mut sum_freq: u64 = 0;
        for i in 0..8.min(result.len()) {
            sum_freq = sum_freq.wrapping_add((result[i] as u64) << (i * 8));
        }
        
        // Compute Nakshatra cosmic index (0 to 26)
        let nakshatra_index = (sum_freq % 27) as u8;
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        // Compare data resonance index with current cosmic Nakshatra
        let current_naks = Nakshatra::current_from_time(current_time);
        
        let is_resonant = (current_naks as u8) == nakshatra_index;

        Ok(EvmResonance {
            original_hex: hex_payload.to_string(),
            frequency: sum_freq,
            nakshatra_index,
            is_resonant,
        })
    }
}

/// Representation of the received payload resonance state
pub struct EvmResonance {
    pub original_hex: String,
    pub frequency: u64,
    pub nakshatra_index: u8,
    pub is_resonant: bool,
}
