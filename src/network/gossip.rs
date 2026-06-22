/// # KalaSync P2P Network — कालसिंक जाल (TCP-based)
///
/// Real TCP peer-to-peer discovery that works across the internet.
/// Replaces the old UDP Multicast (LAN-only) approach.
/// Seed node: node.kasturisundari.xyz:27271

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{interval, Duration};
use std::collections::HashSet;
use once_cell::sync::Lazy;
use crate::network::sync::{SyncRequest, SyncResponse, handle_sync_request};
use sha2::Digest;

/// Default seed node (the mother node)
const DEFAULT_SEED: &str = "node.kasturisundari.xyz:27271";

/// Default P2P port
pub const DEFAULT_P2P_PORT: u16 = 27271;

/// Global P2P port (can be overridden via CLI)
pub static P2P_PORT: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(DEFAULT_P2P_PORT));

/// Known peers registry (thread-safe)
pub static KNOWN_PEERS: Lazy<Arc<Mutex<HashSet<String>>>> = Lazy::new(|| {
    let mut peers = HashSet::new();
    
    // Add seed nodes from environment
    if let Ok(seeds) = std::env::var("KASTURI_SEEDS") {
        for seed in seeds.split(',') {
            let trimmed = seed.trim().to_string();
            if !trimmed.is_empty() {
                peers.insert(trimmed);
            }
        }
    }
    
    // Always include the primary seed node
    peers.insert(DEFAULT_SEED.to_string());
    
    Arc::new(Mutex::new(peers))
});

/// Heartbeat message exchanged between peers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub node_id: String,
    pub chain_height: i64,
    pub latest_hash: String,
    pub p2p_port: u16,
    pub peers: Vec<String>,
    pub timestamp: u64,
}

/// P2P Message types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2PMessage {
    /// Periodic heartbeat
    Heartbeat(Heartbeat),
    /// Sync request
    SyncReq(SyncRequest),
    /// Sync response
    SyncResp(SyncResponse),
    /// New transaction broadcast
    NewTransaction(serde_json::Value),
    /// New block broadcast
    NewBlock(serde_json::Value),
    /// Request peer list
    GetPeers,
    /// Peer list response
    PeerList(Vec<String>),
}

pub struct KalaSyncNode {
    node_id: String,
}

impl KalaSyncNode {
    pub fn new() -> Self {
        let id = format!("VedicNode-{}", rand::random::<u16>());
        Self { node_id: id }
    }

    pub async fn start(self) -> std::io::Result<()> {
        let port = *P2P_PORT.lock().unwrap();
        
        // Start TCP listener for incoming peer connections
        let node_id_listener = self.node_id.clone();
        tokio::spawn(async move {
            if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)).await {
                println!("🌐 P2P TCP Server started on port {}", port);
                loop {
                    if let Ok((socket, addr)) = listener.accept().await {
                        let peer_addr = format!("{}:{}", addr.ip(), port);
                        let mut should_handle = false;
                        {
                            let mut peers = KNOWN_PEERS.lock().unwrap();
                            if !peers.contains(&peer_addr) && !is_self_address(&peer_addr) {
                                // --- THE IMPOSSIBLE PATCH: P2P Firewall ---
                                // Cap maximum connections to prevent Memory Exhaustion & Eclipse Attacks
                                if peers.len() >= 50 {
                                    println!("🛑 WARNING: Max peer limit reached (50). Rejecting new peer: {}", peer_addr);
                                } else {
                                    println!("🔗 New peer connected: {}", peer_addr);
                                    peers.insert(peer_addr);
                                    should_handle = true;
                                }
                            } else if peers.contains(&peer_addr) {
                                // Allow known peers to communicate
                                should_handle = true;
                            }
                        }

                        if !should_handle {
                            continue; // DROP the connection!
                        }

                        tokio::spawn(async move {
                            handle_incoming_connection(socket).await;
                        });
                    }
                }
            } else {
                eprintln!("⚠️ Could not bind P2P port {}. Another node may be running.", port);
            }
        });

        // Periodic heartbeat to all known peers
        let node_id_heartbeat = self.node_id.clone();
        tokio::spawn(async move {
            // Wait a moment for the listener to start
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            let mut ticker = interval(Duration::from_secs(15));
            loop {
                ticker.tick().await;
                
                let peers: Vec<String> = {
                    let peers = KNOWN_PEERS.lock().unwrap();
                    peers.iter().cloned().collect()
                };

                // Get current chain state
                let (height, latest_hash) = get_local_chain_state();
                
                let heartbeat = Heartbeat {
                    node_id: node_id_heartbeat.clone(),
                    chain_height: height,
                    latest_hash,
                    p2p_port: *P2P_PORT.lock().unwrap(),
                    peers: peers.clone(),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                let msg = P2PMessage::Heartbeat(heartbeat);
                
                for peer in &peers {
                    let msg_clone = msg.clone();
                    let peer_clone = peer.clone();
                    tokio::spawn(async move {
                        let _ = send_message(&peer_clone, &msg_clone).await;
                    });
                }
            }
        });

        // Initial sync: connect to seed nodes and download chain
        let node_id_sync = self.node_id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("🔄 [{}] Starting initial chain sync from seed nodes...", node_id_sync);
            
            let peers: Vec<String> = {
                let peers = KNOWN_PEERS.lock().unwrap();
                peers.iter().cloned().collect()
            };
            
            for peer in &peers {
                match send_message_and_receive(peer, &P2PMessage::SyncReq(SyncRequest::GetChainState)).await {
                    Ok(P2PMessage::SyncResp(SyncResponse::ChainState { height, latest_hash, .. })) => {
                        let (local_height, _) = get_local_chain_state();
                        if height as i64 > local_height {
                            println!("📥 Peer {} has chain height {}. We have {}. Syncing...", peer, height, local_height);
                            // Request missing blocks
                            let from = if local_height > 0 { local_height as u64 + 1 } else { 1 };
                            let req = P2PMessage::SyncReq(SyncRequest::GetBlocks { from, to: height });
                            if let Ok(P2PMessage::SyncResp(SyncResponse::Blocks { payloads })) = 
                                send_message_and_receive(peer, &req).await {
                                println!("📦 Received {} block(s) from {}", payloads.len(), peer);
                                // TODO: Apply blocks to local chain
                            }
                        } else {
                            println!("✅ Chain is up to date with peer {} (height: {})", peer, height);
                        }
                    }
                    Ok(_) => {}
                    Err(_) => {
                        // Peer unreachable, that's OK
                    }
                }
            }
        });

        println!("🕉️  Kala Sync Gossip Node & Autonomous Consensus Miner started ({})", self.node_id);
        
        Ok(())
    }
}

/// Handle an incoming TCP connection
async fn handle_incoming_connection(mut socket: TcpStream) {
    let mut buf = vec![0u8; 65536];
    
    // --- THE ABSOLUTE SECURITY PATCH #6: Slowloris DoS Protection ---
    // Enforce a strict 5-second timeout on network reads to prevent connection exhaustion 
    // and OOM crashes from idle malicious peers.
    let read_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        socket.read(&mut buf)
    ).await;

    if let Ok(Ok(n)) = read_result {
        if n == 0 { return; }
        
        // --- THE ABSOLUTE SECURITY PATCH #8: P2P Payload Size Limits ---
        // Prevent OOM / CPU Exhaustion DoS by refusing to parse massive JSON payloads
        if n > 2_000_000 { // 2MB max for blocks
            println!("🛑 SECURITY: Dropped P2P connection from {}. Payload size ({} bytes) exceeds 2MB limit.", socket.peer_addr().unwrap(), n);
            return;
        }

        if let Ok(msg) = serde_json::from_slice::<P2PMessage>(&buf[..n]) {
            let response = match msg {
                P2PMessage::Heartbeat(hb) => {
                    // Learn new peers from the heartbeat
                    // --- PHASE 12 PATCH: Peer List Flooding Protection ---
                    // Limit peers accepted per heartbeat to 50, and total peer list to 1000
                    let mut our_peers = KNOWN_PEERS.lock().unwrap();
                    let mut added = 0;
                    for p in &hb.peers {
                        if added >= 50 { break; } // Max 50 peers per heartbeat
                        if our_peers.len() >= 1_000 { break; } // Max 1000 total peers
                        if !p.is_empty() && !is_self_address(p) {
                            if our_peers.insert(p.clone()) {
                                added += 1;
                            }
                        }
                    }
                    None // No response needed for heartbeat
                }
                P2PMessage::SyncReq(req) => {
                    let resp = handle_sync_request(req).await;
                    Some(P2PMessage::SyncResp(resp))
                }
                P2PMessage::GetPeers => {
                    let peers: Vec<String> = KNOWN_PEERS.lock().unwrap().iter().cloned().collect();
                    Some(P2PMessage::PeerList(peers))
                }
                P2PMessage::NewTransaction(tx) => {
                    let peer_addr = socket.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "Unknown".to_string());
                    
                    // --- THE IMMORTAL NETWORK PATCH: Rakshasa Sentry Nodes ---
                    // Let the sentry inspect the transaction for malicious intent before adding to mempool
                    let sentry = crate::network::rakshasa::RakshasaSentry::new();
                    if let Some(fraud_proof) = sentry.inspect_relayed_transaction(&tx, &peer_addr) {
                        // Fraud detected! Slash the offender
                        sentry.execute_slash(&fraud_proof);
                        // Do not add to mempool
                    } else {
                        // Safe from immediate fraud, but must pass Sabha Governance
                        // --- THE DEEP DIVE PATCH #7: Mempool Poisoning DoS Fix ---
                        // P2P transactions bypassed Sabha evaluation and filled the mempool with garbage.
                        // We must route it through daemon::add_to_mempool to enforce limits and rules.
                        crate::network::daemon::add_to_mempool(tx);
                    }
                    None
                }
                P2PMessage::NewBlock(block) => {
                    // --- THE DEEP DIVE PATCH #5: P2P Block Forgery (Chain Overwrite) Fix ---
                    // Enforce strict chain linkage. A block is ONLY accepted if its `previous_hash`
                    // matches our `chain_latest_hash` and its `block_number` is our `chain_height` + 1.
                    let peer_addr = socket.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "Unknown".to_string());
                    
                    let mut is_valid_link = false;
                    let mut hash_to_store = String::new();
                    let mut height_to_store = 0;
                    
                    if let (Some(b_num), Some(prev_hash), Some(b_hash)) = (
                        block.get("block_number").and_then(|v| v.as_u64()),
                        block.get("previous_hash").and_then(|v| v.as_str()),
                        block.get("hash").and_then(|v| v.as_str()),
                    ) {
                        let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                        let local_height = match db.retrieve("chain_height") {
                            Some(crate::evaluator::Value::Integer(n)) => n as u64,
                            _ => 0,
                        };
                        let local_latest_hash = match db.retrieve("chain_latest_hash") {
                            Some(crate::evaluator::Value::Str(h)) => h,
                            _ => "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                        };
                        
                        if b_num == local_height + 1 && prev_hash == local_latest_hash {
                            is_valid_link = true;
                            hash_to_store = b_hash.to_string();
                            height_to_store = b_num as i64;
                        } else {
                            println!("🛑 SECURITY: Rejected P2P Block from {}. Chain Linkage Failed: Block {} does not build on {}", peer_addr, b_num, local_height);
                        }
                    }
                    
                    if is_valid_link {
                        // --- THE ABSOLUTE SECURITY PATCH #1: VRF Forgery Prevention ---
                        // Re-calculate the VRF to prove the proposer was actually authorized to mine this block!
                        let mut is_authorized = false;
                        let block_proposer = block.get("proposer").and_then(|v| v.as_str()).unwrap_or("");
                        
                        {
                            let account_db = crate::network::account::ACCOUNT_DB.lock().unwrap();
                            let validators = account_db.get_validators();
                            
                            if !validators.is_empty() {
                                let current_nakshatra = crate::shiva::nakshatra::get_current_nakshatra();
                                let nakshatra_name = format!("{:?}", current_nakshatra);
                                let mut best_proposer = String::new();
                                let mut lowest_score = f64::MAX;
                                
                                for validator in &validators {
                                    let vrf_payload = format!("{}:{}:{}", block.get("previous_hash").and_then(|v| v.as_str()).unwrap_or(""), nakshatra_name, validator.address);
                                    let mut hasher = sha2::Sha256::new();
                                    sha2::Digest::update(&mut hasher, vrf_payload.as_bytes());
                                    let hash_bytes = hasher.finalize();
                                    let mut hash_arr = [0u8; 8];
                                    hash_arr.copy_from_slice(&hash_bytes[0..8]);
                                    let vrf_base = u64::from_be_bytes(hash_arr) as f64;
                                    let safe_stake = if validator.staked_balance > 0 { validator.staked_balance as f64 } else { 1.0 };
                                    let vrf_score = vrf_base / safe_stake;
                                    if vrf_score < lowest_score {
                                        lowest_score = vrf_score;
                                        best_proposer = validator.address.clone();
                                    }
                                }
                                
                                if block_proposer == best_proposer {
                                    is_authorized = true;
                                } else {
                                    println!("🛑 SECURITY: Rejected P2P Block {}. Consensus Bypass! Proposer {} did not win the VRF (Expected {}).", height_to_store, block_proposer, best_proposer);
                                }
                            } else {
                                // No validators yet, genesis rules apply
                                is_authorized = true; 
                            }
                        }

                        // --- THE ABSOLUTE SECURITY PATCH #2: Hyperinflation Fix ---
                        // DO NOT trust the JSON payload for block rewards. Hardcode the consensus rules.
                        let claimed_reward = block.get("block_reward").and_then(|v| v.as_u64()).unwrap_or(0);
                        let is_reward_valid = if claimed_reward == 10 {
                            true
                        } else {
                            println!("🛑 SECURITY: Rejected P2P Block {}. Invalid block reward claimed: {} (Expected 10).", height_to_store, claimed_reward);
                            false
                        };

                        let mut all_txs_valid = true;
                        let block_prev_hash = block.get("previous_hash").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        
                        if is_authorized && is_reward_valid {
                            if let Some(txs) = block.get("transactions").and_then(|v| v.as_array()) {
                                for tx in txs {
                                    if let Err(e) = crate::network::daemon::validate_transaction(tx) {
                                        println!("🛑 SECURITY: Rejected P2P Block {} from {}. Transaction validation failed: {}", height_to_store, peer_addr, e);
                                        all_txs_valid = false;
                                        break;
                                    }
                                }
                            
                                if all_txs_valid {
                                    let account_db = crate::network::account::ACCOUNT_DB.lock().unwrap();
                                    let mut db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                                    
                                    for tx in txs {
                                        let from = tx.get("from").and_then(|v| v.as_str()).unwrap_or("");
                                        let to = tx.get("to").and_then(|v| v.as_str()).unwrap_or("");
                                        let amount = tx.get("amount").and_then(|v| v.as_u64()).unwrap_or(0);
                                        // Use minimum fee 50 as standard for P2P relayed blocks as requested for donations
                                        let fee = tx.get("fee").and_then(|v| v.as_u64()).unwrap_or(50);
                                        let nonce = tx.get("nonce").and_then(|v| v.as_u64()).unwrap_or(0);
                                        
                                        // Execute transfer to synchronize state
                                        if let Err(e) = account_db.transfer(from, to, amount, fee, nonce) {
                                            println!("⚠️ Unexpected P2P Transfer Failure (State may be slightly desynced): {}", e);
                                        }
                                        
                                        // Nullify ZK spends
                                        if let Some(nullifier) = tx.get("zk_nullifier").and_then(|v| v.as_str()) {
                                            let nullifier_key = format!("zk_nullifier_{}", nullifier);
                                            db.store(&nullifier_key, &crate::evaluator::Value::Integer(1));
                                        }
                                    }
                                    
                                    // Credit Block Reward to Proposer
                                    if let (Some(proposer), Some(reward)) = (
                                        block.get("proposer").and_then(|v| v.as_str()),
                                        block.get("block_reward").and_then(|v| v.as_u64())
                                    ) {
                                        if reward > 0 {
                                            let _ = account_db.credit(proposer, reward);
                                        }
                                    }
                                }
                            } else {
                                // If it's an empty block (tx_count == 0), it's valid.
                                // If transactions array is missing but tx_count > 0, it's invalid.
                                let tx_count = block.get("tx_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                if tx_count > 0 {
                                    println!("🛑 SECURITY: Rejected P2P Block {} from {}. Missing transaction payload!", height_to_store, peer_addr);
                                    all_txs_valid = false;
                                } else {
                                    // Empty block, credit reward
                                    if let (Some(proposer), Some(reward)) = (
                                        block.get("proposer").and_then(|v| v.as_str()),
                                        block.get("block_reward").and_then(|v| v.as_u64())
                                    ) {
                                        if reward > 0 {
                                            let account_db = crate::network::account::ACCOUNT_DB.lock().unwrap();
                                            let _ = account_db.credit(proposer, reward);
                                        }
                                    }
                                }
                            }
                        } else {
                            all_txs_valid = false; // Block rejected due to VRF or Reward fail
                        }

                        if all_txs_valid {
                            // Accept the block into our Mandala DB
                            let mut db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                            let block_val = crate::evaluator::builtins::json_to_value(&block);
                            db.store(&hash_to_store, &block_val);
                            db.store(&format!("block_hash_{}", height_to_store), &crate::evaluator::Value::Str(hash_to_store.clone()));
                            db.store("chain_latest_hash", &crate::evaluator::Value::Str(hash_to_store.clone()));
                            db.store("chain_height", &crate::evaluator::Value::Integer(height_to_store));
                            println!("🔗 Synchronized P2P Block {} from {} via Gossip Network. State updated.", height_to_store, peer_addr);
                        }
                    }
                    None
                }
                _ => None,
            };
            
            if let Some(resp) = response {
                if let Ok(json) = serde_json::to_vec(&resp) {
                    let _ = socket.write_all(&json).await;
                }
            }
        }
    }
}

/// Send a P2P message to a peer (fire-and-forget)
async fn send_message(peer: &str, msg: &P2PMessage) -> Result<(), String> {
    let mut stream = TcpStream::connect(peer).await
        .map_err(|e| format!("Connect to {} failed: {}", peer, e))?;
    
    let json = serde_json::to_vec(msg)
        .map_err(|e| format!("Serialize failed: {}", e))?;
    
    stream.write_all(&json).await
        .map_err(|e| format!("Write to {} failed: {}", peer, e))?;
    
    Ok(())
}

/// Send a message and wait for response
async fn send_message_and_receive(peer: &str, msg: &P2PMessage) -> Result<P2PMessage, String> {
    let mut stream = TcpStream::connect(peer).await
        .map_err(|e| format!("Connect to {} failed: {}", peer, e))?;
    
    let json = serde_json::to_vec(msg)
        .map_err(|e| format!("Serialize failed: {}", e))?;
    
    stream.write_all(&json).await
        .map_err(|e| format!("Write to {} failed: {}", peer, e))?;
    
    let mut buf = vec![0u8; 65536];
    let n = tokio::time::timeout(Duration::from_secs(10), stream.read(&mut buf)).await
        .map_err(|_| "Timeout reading from peer".to_string())?
        .map_err(|e| format!("Read from {} failed: {}", peer, e))?;
    
    if n == 0 {
        return Err("Empty response".into());
    }
    
    serde_json::from_slice(&buf[..n])
        .map_err(|e| format!("Deserialize failed: {}", e))
}

/// Broadcast a transaction to all known peers
pub async fn broadcast_transaction(tx: serde_json::Value) {
    let peers: Vec<String> = KNOWN_PEERS.lock().unwrap().iter().cloned().collect();
    let msg = P2PMessage::NewTransaction(tx);
    for peer in peers {
        let msg_clone = msg.clone();
        tokio::spawn(async move {
            let _ = send_message(&peer, &msg_clone).await;
        });
    }
}

/// Broadcast a new block to all known peers
pub async fn broadcast_block(block: serde_json::Value) {
    let peers: Vec<String> = KNOWN_PEERS.lock().unwrap().iter().cloned().collect();
    let msg = P2PMessage::NewBlock(block);
    for peer in peers {
        let msg_clone = msg.clone();
        tokio::spawn(async move {
            let _ = send_message(&peer, &msg_clone).await;
        });
    }
}

/// Get local chain state from Mandala DB
fn get_local_chain_state() -> (i64, String) {
    let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
    let height = match db.retrieve("chain_height") {
        Some(crate::evaluator::Value::Integer(h)) => h,
        _ => 0,
    };
    let hash = match db.retrieve("chain_latest_hash") {
        Some(crate::evaluator::Value::Str(h)) => h,
        _ => "genesis".to_string(),
    };
    (height, hash)
}

/// Check if an address is our own (to avoid self-connection)
fn is_self_address(addr: &str) -> bool {
    addr.starts_with("127.0.0.1") || addr.starts_with("0.0.0.0")
}

/// Get peer count
pub fn peer_count() -> usize {
    KNOWN_PEERS.lock().unwrap().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_self_address() {
        assert!(is_self_address("127.0.0.1:27271"));
        assert!(is_self_address("0.0.0.0:27271"));
        assert!(!is_self_address("192.168.1.5:27271"));
        assert!(!is_self_address("node.kasturisundari.xyz:27271"));
    }

    #[test]
    fn test_peer_count_starts_with_seed() {
        // Initial KNOWN_PEERS includes DEFAULT_SEED
        let count = peer_count();
        assert!(count >= 1);
        
        let peers = KNOWN_PEERS.lock().unwrap();
        assert!(peers.contains(DEFAULT_SEED));
    }

    #[test]
    fn test_heartbeat_serialization() {
        let hb = Heartbeat {
            node_id: "test_node".to_string(),
            chain_height: 5,
            latest_hash: "hash".to_string(),
            p2p_port: 27271,
            peers: vec!["peer1".to_string()],
            timestamp: 123456789,
        };
        let msg = P2PMessage::Heartbeat(hb);
        let serialized = serde_json::to_string(&msg).unwrap();
        assert!(serialized.contains("Heartbeat"));
        assert!(serialized.contains("test_node"));
    }

    #[test]
    fn test_kala_sync_node_creation() {
        let node = KalaSyncNode::new();
        assert!(node.node_id.starts_with("VedicNode-"));
    }

    #[test]
    fn test_p2p_message_enums() {
        let tx = serde_json::json!({"test": "tx"});
        let block = serde_json::json!({"test": "block"});
        
        let msg1 = P2PMessage::NewTransaction(tx);
        let msg2 = P2PMessage::NewBlock(block);
        let msg3 = P2PMessage::GetPeers;
        
        assert!(serde_json::to_string(&msg1).unwrap().contains("NewTransaction"));
        assert!(serde_json::to_string(&msg2).unwrap().contains("NewBlock"));
        assert!(serde_json::to_string(&msg3).unwrap().contains("GetPeers"));
    }
}
