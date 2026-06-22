/// Mandala Storage Architecture
///
/// Replaces standard relational storage with a harmonic geometric database layout.
/// Data is categorized based on the resonance frequency of ShivaState using Katapayadi rules.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use sled::Db;

use crate::shiva::sutras::ShivaState;
use crate::shiva::frequency::FrequencyMatrix;
use crate::evaluator::{Value, TattvaState};
use crate::evaluator::builtins::{value_to_json, json_to_value};
use crate::storage::shard_router::{ShardRouter, ShardCoordinate};

lazy_static! {
    /// Global In-Memory Mandala database instance
    pub static ref MANDALA_DB: Arc<Mutex<MandalaNetwork>> = Arc::new(Mutex::new(MandalaNetwork::new()));
}

/// A single data point (Bindu)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bindu {
    pub key: String,
    pub frequency: f64,
    pub data_json: serde_json::Value,
}

/// Mandala Network representation using Sled KV Store
#[derive(Clone)]
pub struct MandalaNetwork {
    pub db: Db,
}

impl MandalaNetwork {
    pub fn new() -> Self {
        let db = sled::open("kasturi_mandala_db").expect("Failed to open Mandala Sled DB");
        Self { db }
    }

    /// Calculate resonance key frequency using simplified Katapayadi hash
    fn calculate_resonance(key: &str) -> (u8, f64) {
        let mut hash: u64 = 5381;
        for b in key.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64); // hash * 33 + c
        }
        
        let shiva_index = (hash % 14) as u8;
        
        let all_states = ShivaState::all();
        let state = &all_states[shiva_index as usize];
        
        let matrix = FrequencyMatrix::new();
        let freq = matrix.composite_frequency(&[state.clone()]);

        (shiva_index, freq)
    }

    /// Store value in Mandala network (√smṛ) and persist to disk
    pub fn store(&mut self, key: &str, value: &Value) {
        let coord = ShardRouter::calculate_coordinate(key);
        let coord_str = coord.to_string_key();
        
        let (_, freq) = Self::calculate_resonance(key);
        
        let bindu = Bindu {
            key: key.to_string(),
            frequency: freq,
            data_json: value_to_json(value),
        };

        // We prefix the key with the shard coordinate to group them logically
        let db_key = format!("{}:{}", coord_str, key);
        let db_val = serde_json::to_vec(&bindu).unwrap_or_default();
        
        let _ = self.db.insert(db_key.as_bytes(), db_val);
        let _ = self.db.flush();
    }

    /// Retrieve value from Mandala network (√dṛś)
    pub fn retrieve(&self, key: &str) -> Option<Value> {
        let coord = ShardRouter::calculate_coordinate(key);
        let coord_str = coord.to_string_key();
        
        let db_key = format!("{}:{}", coord_str, key);
        
        if let Ok(Some(ivec)) = self.db.get(db_key.as_bytes()) {
            if let Ok(bindu) = serde_json::from_slice::<Bindu>(&ivec) {
                return Some(json_to_value(&bindu.data_json));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use crate::evaluator::{Value, TattvaState};
    use crate::evaluator::builtins::{value_to_json, json_to_value};

    // Note: sled db tests could interfere with each other if using the same db name,
    // but in Rust tests run concurrently. Since `MandalaNetwork::new()` opens the 
    // real "kasturi_mandala_db", we should probably use a temporary db or unique prefixes.
    // To prevent total locking, we will just test against a local instance pointing 
    // to a temporary path or just serialize sequentially if possible, but actually 
    // sled allows multiple concurrent handles to the same db. We will just test through 
    // the methods.

    fn get_test_db() -> MandalaNetwork {
        // Because sled complains if we open the same path multiple times in the same process
        // we'll just use the global MANDALA_DB instance for tests or open a temp one.
        // Opening a temp db per test is safer.
        let temp_dir = std::env::temp_dir().join(format!("test_mandala_{}", rand::random::<u32>()));
        let db = sled::open(temp_dir).unwrap();
        MandalaNetwork { db }
    }

    #[test]
    fn test_mandala_store_and_retrieve_integer() {
        let mut network = get_test_db();
        network.store("test_int", &Value::Integer(42));
        
        let retrieved = network.retrieve("test_int").unwrap();
        assert_eq!(retrieved, Value::Integer(42));
    }

    #[test]
    fn test_mandala_store_and_retrieve_string() {
        let mut network = get_test_db();
        network.store("test_str", &Value::Str("Vedas".to_string()));
        
        let retrieved = network.retrieve("test_str").unwrap();
        assert_eq!(retrieved, Value::Str("Vedas".to_string()));
    }

    #[test]
    fn test_mandala_store_and_retrieve_float() {
        let mut network = get_test_db();
        network.store("test_float", &Value::Float(108.0));
        
        if let Value::Float(f) = network.retrieve("test_float").unwrap() {
            assert!((f - 108.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected float");
        }
    }

    #[test]
    fn test_mandala_store_and_retrieve_tattva() {
        let mut network = get_test_db();
        network.store("test_tattva", &Value::Tattva(TattvaState::Sat));
        
        assert_eq!(network.retrieve("test_tattva").unwrap(), Value::Tattva(TattvaState::Sat));
    }

    #[test]
    fn test_mandala_retrieve_nonexistent() {
        let network = get_test_db();
        let res = network.retrieve("this_key_does_not_exist");
        assert!(res.is_none());
    }

    #[test]
    fn test_mandala_resonance_calculation() {
        let (index1, freq1) = MandalaNetwork::calculate_resonance("karma");
        let (index2, freq2) = MandalaNetwork::calculate_resonance("dharma");
        
        // Different words likely have different indices/freqs
        // But more importantly, same word must have exactly same result
        let (index1_again, freq1_again) = MandalaNetwork::calculate_resonance("karma");
        
        assert_eq!(index1, index1_again);
        assert!((freq1 - freq1_again).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mandala_overwrite_key() {
        let mut network = get_test_db();
        network.store("overwrite_key", &Value::Integer(1));
        assert_eq!(network.retrieve("overwrite_key").unwrap(), Value::Integer(1));
        
        network.store("overwrite_key", &Value::Integer(2));
        assert_eq!(network.retrieve("overwrite_key").unwrap(), Value::Integer(2));
    }

    #[test]
    fn test_mandala_store_complex_object() {
        let mut network = get_test_db();
        // Since we don't have direct access to Map builder easily, we can just use json_to_value
        let json = serde_json::json!({
            "name": "Arjuna",
            "age": 30
        });
        let val = json_to_value(&json);
        network.store("complex_warrior", &val);
        
        let retrieved = network.retrieve("complex_warrior").unwrap();
        // Just verify it's a Map
        if let Value::Map(m) = retrieved {
            let map = m.lock().unwrap();
            assert_eq!(map.len(), 2);
            let name_key = Value::Str("name".to_string());
            assert_eq!(map.get(&name_key).unwrap().clone(), Value::Str("Arjuna".to_string()));
        } else {
            panic!("Expected Map");
        }
    }
}
