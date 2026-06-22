use sled::Db;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    /// Global instance of the Sled database for indexing
    pub static ref INDEX_DB: Arc<IndexerDb> = Arc::new(IndexerDb::new());
}

pub struct IndexerDb {
    db: Db,
}

impl IndexerDb {
    pub fn new() -> Self {
        let db = sled::open("mandala_indexer_db").expect("Failed to open Indexer Database");
        Self { db }
    }

    /// Record a transaction hash for a specific address
    pub fn add_history(&self, address: &str, tx_hash: &str) {
        let key = address.as_bytes();
        let existing = self.db.get(key).unwrap_or(None);
        
        let mut history: Vec<String> = match existing {
            Some(bytes) => serde_json::from_slice(&bytes).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };

        // Prepend to keep latest first
        history.insert(0, tx_hash.to_string());
        
        // Keep only last 100 to prevent unbounded growth in MVP
        if history.len() > 100 {
            history.truncate(100);
        }

        if let Ok(serialized) = serde_json::to_vec(&history) {
            let _ = self.db.insert(key, serialized);
        }
    }

    /// Retrieve transaction history for an address
    pub fn get_history(&self, address: &str) -> Vec<String> {
        let key = address.as_bytes();
        if let Ok(Some(bytes)) = self.db.get(key) {
            serde_json::from_slice(&bytes).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        }
    }
}
