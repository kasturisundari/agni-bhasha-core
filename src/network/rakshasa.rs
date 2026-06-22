/// # Rakshasa (राक्षस) Sentry Nodes
///
/// Silent observers on the Gossip Network. Rakshasa sentries do not produce blocks.
/// They monitor the Mempool and incoming transactions for fraud (e.g. Double Spends,
/// Invalid Nonces, Invalid Signatures).
///
/// If a node broadcasts a transaction that fails critical checks but was relayed by a validator,
/// the Rakshasa creates a FraudProof and submits it to the Sabha to slash the offender's stake.

use crate::network::account::{ACCOUNT_DB, StakingAction};
use crate::network::daemon::NONCE_TRACKER;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FraudProof {
    pub offender_node_id: String,
    pub malicious_tx: serde_json::Value,
    pub reason: String,
    pub sentry_id: String,
    pub timestamp: u64,
}

pub struct RakshasaSentry {
    pub sentry_id: String,
}

impl RakshasaSentry {
    pub fn new() -> Self {
        let id = format!("Rakshasa-{}", rand::random::<u16>());
        Self { sentry_id: id }
    }

    /// Monitors an incoming transaction relayed by a specific peer.
    /// If the transaction is fundamentally flawed (e.g. signature bypass attempt),
    /// generates a FraudProof to slash the peer.
    pub fn inspect_relayed_transaction(&self, tx: &serde_json::Value, peer_id: &str) -> Option<FraudProof> {
        let from = tx.get("from").and_then(|v| v.as_str()).unwrap_or("");
        let amount = tx.get("amount").and_then(|v| v.as_u64()).unwrap_or(0);
        let nonce = tx.get("nonce").and_then(|v| v.as_u64()).unwrap_or(0);
        
        let mut is_fraudulent = false;
        let mut fraud_reason = String::new();

        // 1. Signature Bypass Check
        let signature = tx.get("signature").and_then(|v| v.as_str()).unwrap_or("");
        if signature.is_empty() {
            is_fraudulent = true;
            fraud_reason = "Relayed transaction missing mandatory signature".to_string();
        }

        // 2. Double Spend Check (Balance vs Amount)
        if !is_fraudulent {
            let account_db = ACCOUNT_DB.lock().unwrap();
            let balance = account_db.get_balance(from);
            if balance < amount {
                is_fraudulent = true;
                fraud_reason = format!("Relayed double-spend transaction: {} < {}", balance, amount);
            }
        }

        // 3. Stagnant Nonce Check
        if !is_fraudulent {
            let nonce_map = NONCE_TRACKER.lock().unwrap();
            if let Some(&last_nonce) = nonce_map.get(from) {
                if nonce <= last_nonce {
                    is_fraudulent = true;
                    fraud_reason = format!("Relayed stagnant nonce transaction: {} <= {}", nonce, last_nonce);
                }
            }
        }

        if is_fraudulent {
            println!("👁️  राक्षس (Rakshasa): FRAUD DETECTED from Peer {}!", peer_id);
            println!("   ⮑ Reason: {}", fraud_reason);
            
            return Some(FraudProof {
                offender_node_id: peer_id.to_string(),
                malicious_tx: tx.clone(),
                reason: fraud_reason,
                sentry_id: self.sentry_id.clone(),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            });
        }

        None
    }

    /// Submits a FraudProof to the Network (Sabha) for immediate slashing
    pub fn execute_slash(&self, proof: &FraudProof) {
        println!("⚔️  राक्षس (Rakshasa): EXECUTING SLASH ON {}...", proof.offender_node_id);
        let mut db = ACCOUNT_DB.lock().unwrap();
        
        // Slash 50% of the offender's staked balance as penalty
        // We assume offender_node_id corresponds to a registered staking public key
        // In a real network, the peer must have a staked identity
        let current_stake = db.get_balance(&proof.offender_node_id);
        if current_stake > 0 {
            // --- THE DEEP DIVE PATCH #8: Rakshasa Sentry Crash Fix ---
            // The previous API was non-existent. We correctly use `slash` and pass the 50% penalty.
            match db.slash(&proof.offender_node_id, 50) {
                Ok(slashed_amount) => {
                    println!("🔥 SLAUGHTERED: Node {} lost {} Pyar for relaying malicious data!", proof.offender_node_id, slashed_amount);
                },
                Err(e) => {
                    println!("⚠️ Slash execution failed: {}", e);
                }
            }
        } else {
            println!("⚠️ Offender {} has no stake to slash, banning peer...", proof.offender_node_id);
            // In a full P2P implementation, we would also drop the TCP connection here
        }
    }
}
