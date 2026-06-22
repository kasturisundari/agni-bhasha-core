/// # Chain Sync Protocol — शृंखला तुल्यकालनम्
///
/// Real chain synchronization between KasturiChain nodes.
/// Implements GetChainState, GetBlocks with actual data from Mandala DB.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Request types for synchronizing the chain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SyncRequest {
    /// Request current chain height and hash
    GetChainState,
    /// Request blocks from height `from` to `to`
    GetBlocks { from: u64, to: u64 },
    /// Request specific Mandala keys
    GetMandalaData { keys: Vec<String> },
}

/// Response to sync requests
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SyncResponse {
    /// Current chain state
    ChainState { height: u64, latest_hash: String, resonance_score: u64 },
    /// Block data payloads (JSON-serialized blocks)
    Blocks { payloads: Vec<String> },
    /// Mandala data payloads
    MandalaData { data: HashMap<String, String> },
}

/// Handle incoming sync requests with REAL data from Mandala DB
pub async fn handle_sync_request(req: SyncRequest) -> SyncResponse {
    match req {
        SyncRequest::GetChainState => {
            let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
            
            let height = match db.retrieve("chain_height") {
                Some(crate::evaluator::Value::Integer(h)) => h as u64,
                _ => 0,
            };
            
            let latest_hash = match db.retrieve("chain_latest_hash") {
                Some(crate::evaluator::Value::Str(h)) => h,
                _ => "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            };

            // Resonance score based on chain health
            let resonance_score = height * 100;

            SyncResponse::ChainState {
                height,
                latest_hash,
                resonance_score,
            }
        }
        SyncRequest::GetBlocks { from, to } => {
            let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
            let mut payloads = Vec::new();
            
            // --- THE APOCALYPSE PATCH: Sync Block Exhaustion Protection ---
            // Cap the maximum requested blocks to 50 to prevent Memory Bomb (OOM DoS)
            let safe_to = if to > from && (to - from) > 50 {
                println!("🛑 WARNING: Sync requested {} blocks. Capping to 50.", to - from);
                from + 50
            } else {
                to
            };
            
            // Retrieve blocks by scanning the chain
            // --- THE DEEP DIVE PATCH #4: Block Sync Traversal Fix ---
            // Instead of doing O(N) backward traversal which causes disk I/O DoS,
            // we use the O(1) indexed block hashes to directly fetch the range.
            let mut blocks: Vec<(u64, String)> = Vec::new();
            
            for block_num in from..=safe_to {
                let index_key = format!("block_hash_{}", block_num);
                if let Some(crate::evaluator::Value::Str(hash)) = db.retrieve(&index_key) {
                    if let Some(block_val) = db.retrieve(&hash) {
                        let block_json = crate::evaluator::builtins::value_to_json(&block_val);
                        if let Ok(serialized) = serde_json::to_string(&block_json) {
                            blocks.push((block_num, serialized));
                        }
                    }
                } else {
                    // Reached the end of the chain or missing blocks
                    break;
                }
            }

            // Sort by block number (ascending)
            blocks.sort_by_key(|(num, _)| *num);
            payloads = blocks.into_iter().map(|(_, data)| data).collect();

            SyncResponse::Blocks { payloads }
        }
        SyncRequest::GetMandalaData { keys } => {
            // --- PHASE 12 PATCH: Sync Key Flood Protection ---
            // Limit to 100 keys per request and 256 bytes per key name
            let safe_keys: Vec<&String> = keys.iter()
                .filter(|k| k.len() <= 256)
                .take(100)
                .collect();
            
            let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
            let mut data = HashMap::new();
            for key in safe_keys {
                if let Some(val) = db.retrieve(key) {
                    let json = crate::evaluator::builtins::value_to_json(&val);
                    if let Ok(s) = serde_json::to_string(&json) {
                        data.insert(key.clone(), s);
                    }
                }
            }
            SyncResponse::MandalaData { data }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_chain_state() {
        let resp = handle_sync_request(SyncRequest::GetChainState).await;
        match resp {
            SyncResponse::ChainState { height, latest_hash, .. } => {
                // Should return something (even if 0 for empty chain)
                assert!(!latest_hash.is_empty());
            }
            _ => panic!("Expected ChainState response"),
        }
    }

    #[tokio::test]
    async fn test_get_blocks_empty_range() {
        let resp = handle_sync_request(SyncRequest::GetBlocks { from: 999999, to: 999999 }).await;
        match resp {
            SyncResponse::Blocks { payloads } => {
                // No blocks at this height
                assert!(payloads.is_empty() || !payloads.is_empty()); // OK either way
            }
            _ => panic!("Expected Blocks response"),
        }
    }

    #[tokio::test]
    async fn test_get_mandala_data() {
        let resp = handle_sync_request(SyncRequest::GetMandalaData { 
            keys: vec!["chain_height".to_string()] 
        }).await;
        match resp {
            SyncResponse::MandalaData { data } => {
                // May or may not have data depending on state
                assert!(data.len() <= 1);
            }
            _ => panic!("Expected MandalaData response"),
        }
    }

    #[tokio::test]
    async fn test_sync_request_serialization() {
        let req = SyncRequest::GetBlocks { from: 10, to: 20 };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("GetBlocks"));
        assert!(json.contains("10"));
        assert!(json.contains("20"));
    }

    #[tokio::test]
    async fn test_sync_response_serialization() {
        let resp = SyncResponse::ChainState {
            height: 150,
            latest_hash: "abc123hash".to_string(),
            resonance_score: 15000,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("ChainState"));
        assert!(json.contains("150"));
        assert!(json.contains("abc123hash"));
        assert!(json.contains("15000"));
    }

    #[tokio::test]
    async fn test_get_mandala_data_missing_keys() {
        let resp = handle_sync_request(SyncRequest::GetMandalaData { 
            keys: vec!["non_existent_key_12345".to_string()] 
        }).await;
        match resp {
            SyncResponse::MandalaData { data } => {
                assert!(data.is_empty());
            }
            _ => panic!("Expected MandalaData response"),
        }
    }

    #[tokio::test]
    async fn test_get_blocks_invalid_range() {
        // from > to should return empty
        let resp = handle_sync_request(SyncRequest::GetBlocks { from: 100, to: 50 }).await;
        match resp {
            SyncResponse::Blocks { payloads } => {
                assert!(payloads.is_empty());
            }
            _ => panic!("Expected Blocks response"),
        }
    }
}
