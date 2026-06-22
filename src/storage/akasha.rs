/// Akasha Content-Addressable Storage (CAS)
///
/// An immutable, eternal storage layer for KasturiChain.
/// Data is hashed via SHA-256 to generate a Cosmic Identifier (CID).
/// The CID guarantees the exactness and integrity of the sacred knowledge.

use sled::Db;
use sha2::{Sha256, Digest};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::evaluator::Value;
use crate::evaluator::builtins::{value_to_json, json_to_value};

lazy_static! {
    /// Global In-Memory Akasha database instance
    pub static ref AKASHA_DB: Arc<Mutex<AkashaNetwork>> = Arc::new(Mutex::new(AkashaNetwork::new()));
}

/// Akasha Network representation using Sled KV Store
#[derive(Clone)]
pub struct AkashaNetwork {
    pub db: Db,
}

impl AkashaNetwork {
    pub fn new() -> Self {
        let db = sled::open("kasturi_akasha_db").expect("Failed to open Akasha Sled DB");
        Self { db }
    }

    /// Calculate Cosmic Identifier (CID) using SHA-256
    pub fn calculate_cid(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Store value immutably in Akasha (√ākāśa+ति)
    /// Returns the CID (Hash) of the content
    pub fn store(&mut self, value: &Value) -> String {
        let json_data = value_to_json(value);
        let serialized_data = serde_json::to_vec(&json_data).unwrap_or_default();
        
        let cid = Self::calculate_cid(&serialized_data);
        
        // Only insert if it doesn't already exist (immutable property)
        if !self.db.contains_key(cid.as_bytes()).unwrap_or(false) {
            let _ = self.db.insert(cid.as_bytes(), serialized_data);
            let _ = self.db.flush();
        }
        
        cid
    }

    /// Retrieve value from Akasha using CID (√ākāśa+णम्)
    pub fn retrieve(&self, cid: &str) -> Option<Value> {
        if let Ok(Some(ivec)) = self.db.get(cid.as_bytes()) {
            if let Ok(json_data) = serde_json::from_slice::<serde_json::Value>(&ivec) {
                return Some(json_to_value(&json_data));
            }
        }
        None
    }
}
