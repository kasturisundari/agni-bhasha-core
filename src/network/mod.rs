pub mod gossip;
pub mod quantum;
pub mod sync;
pub mod setu_proxy;
pub mod daemon;
pub mod stealth;
pub mod sabha;
pub mod samparka;
pub mod merkle;
pub mod transaction;
pub mod tarka_zk;
pub mod account;
pub mod rakshasa;
pub mod p2p;
pub use gossip::{KalaSyncNode, P2PMessage, P2P_PORT};
pub use setu_proxy::SetuProxyNode;
pub use samparka::SamparkaGateway;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::transaction::Transaction;
    use crate::network::tarka_zk::TarkaZK;
    use crate::network::sabha::SabhaCouncil;
    use crate::network::account::{AccountManager, StakingAction};

    // End-to-End Integration test simulating a transaction passing through ZK rollup,
    // Sabha validation, Staking check, and eventually being ready for a block.
    #[test]
    fn test_e2e_transaction_lifecycle() {
        // 1. Transaction Creation
        let mut tx = Transaction::new(
            "alice_key".to_string(),
            "bob_key".to_string(),
            100.0,
            0,
            "Transfer 100".to_string(),
        );
        tx.sign(b"alice_private_key"); // Dummy sign

        // 2. ZK Privacy (Simulate Rollup)
        // Normally we'd use circuit, let's just generate a proof for the test
        let (pk, _) = TarkaZK::setup().unwrap();
        let private_amount = 100.0;
        let nullifier_hash = TarkaZK::generate_nullifier(&tx.sender, tx.nonce);
        let zk_proof = TarkaZK::generate_proof(&pk, private_amount, tx.nonce).unwrap();
        
        tx.zk_proof = Some(zk_proof);
        tx.zk_nullifier = Some(nullifier_hash);

        // 3. Staking Check (Does Alice have enough to even transact or is she a validator?)
        let mut accounts = AccountManager::new();
        // Alice deposits some stake to participate
        let res = accounts.handle_staking_action(&tx.sender, 500.0, StakingAction::Stake);
        assert!(res.is_ok());

        // 4. Sabha Validation (Governance rules)
        let sabha_score = SabhaCouncil::full_evaluation(&tx);
        // Valid transactions should pass Sabha
        assert!(sabha_score > 0);

        // 5. Verification before Block Assembly
        assert!(tx.verify());
    }

    #[test]
    fn test_e2e_slashing_malicious_actor() {
        let mut accounts = AccountManager::new();
        let validator = "malicious_node".to_string();
        
        accounts.handle_staking_action(&validator, 1000.0, StakingAction::Stake).unwrap();
        
        // Node does something bad (e.g. double signing), detected by Network
        let res = accounts.slash(&validator, 0.5); // Slash 50%
        assert!(res.is_ok());

        let state = accounts.get_state(&validator).unwrap();
        assert!((state.staked_amount - 500.0).abs() < f64::EPSILON);
        assert_eq!(state.slashing_history, 1);
    }
}
