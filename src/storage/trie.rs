use sha2::{Sha256, Digest};
use crate::network::account::{ACCOUNT_DB, AccountState};

/// Cryptographically calculates the Global State Root (Merkle Patricia Trie equivalent)
/// By sorting all account states and building a deterministic hash.
pub fn compute_state_root() -> String {
    let account_db = ACCOUNT_DB.lock().unwrap();
    
    let mut accounts: Vec<AccountState> = Vec::new();
    for iter in account_db.db.iter() {
        if let Ok((_, value)) = iter {
            if let Ok(acc) = serde_json::from_slice::<AccountState>(&value) {
                accounts.push(acc);
            }
        }
    }
    
    // Sort to ensure deterministic hashing regardless of Sled insertion order
    accounts.sort_by(|a, b| a.address.cmp(&b.address));
    
    let mut hasher = Sha256::new();
    for acc in accounts {
        let payload = format!("{}:{}:{}", acc.address, acc.balance, acc.nonce);
        hasher.update(payload.as_bytes());
    }
    
    let result = hex::encode(hasher.finalize());
    if result.is_empty() {
        "0000000000000000000000000000000000000000".to_string()
    } else {
        result
    }
}
