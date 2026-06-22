/// # Transaction — कर्मबिन्दु (Karmabindu)
///
/// Proper transaction structure with digital signatures.
/// Uses CRYSTALS-Dilithium5 (Post-Quantum) for signing and verification.
/// Includes nonce-based replay protection.

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// A signed transaction on KasturiChain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender address (hex-encoded public key hash)
    pub from: String,
    /// Receiver address
    pub to: String,
    /// Amount of Pyar to transfer
    pub amount: u64,
    /// Sequential nonce for replay protection
    pub nonce: u64,
    /// Timestamp (Julian Date for cosmic alignment)
    pub timestamp: f64,
    /// Transaction data/memo (optional)
    pub data: String,
    /// Dilithium5 digital signature (hex-encoded)
    pub signature: String,
    /// Sender's public key (hex-encoded, for verification)
    pub public_key: String,
}

/// Result of transaction validation
#[derive(Debug, Clone)]
pub enum TxValidationResult {
    Valid,
    InvalidSignature,
    InsufficientBalance { available: u64, requested: u64 },
    InvalidNonce { expected: u64, got: u64 },
    SelfTransfer,
    ZeroAmount,
    MissingFields(String),
}

impl std::fmt::Display for TxValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxValidationResult::Valid => write!(f, "Valid"),
            TxValidationResult::InvalidSignature => write!(f, "Invalid digital signature"),
            TxValidationResult::InsufficientBalance { available, requested } =>
                write!(f, "Insufficient balance: have {}, need {}", available, requested),
            TxValidationResult::InvalidNonce { expected, got } =>
                write!(f, "Invalid nonce: expected {}, got {}", expected, got),
            TxValidationResult::SelfTransfer => write!(f, "Self-transfer not allowed"),
            TxValidationResult::ZeroAmount => write!(f, "Transfer amount must be > 0"),
            TxValidationResult::MissingFields(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl Transaction {
    /// Create a new unsigned transaction
    pub fn new(from: String, to: String, amount: u64, nonce: u64, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Self {
            from,
            to,
            amount,
            nonce,
            timestamp,
            data,
            signature: String::new(),
            public_key: String::new(),
        }
    }

    /// Compute the hash of the transaction (excluding signature for signing)
    pub fn hash(&self) -> String {
        // --- THE APOCALYPSE PATCH: Cross-Chain Replay Protection ---
        // By embedding a strict CHAIN_ID, transactions signed on testnet 
        // will have a completely different hash than on mainnet.
        let chain_id = "kasturichain-mainnet-v1";
        let signable = format!(
            "{}:{}:{}:{}:{}:{}:{}",
            chain_id, self.from, self.to, self.amount, self.nonce, self.timestamp, self.data
        );
        let mut hasher = Sha256::new();
        hasher.update(signable.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Compute address from public key (SHA-256 hash of public key, first 40 chars)
    pub fn address_from_pubkey(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let hash = hex::encode(hasher.finalize());
        format!("0x{}", &hash[..40])
    }

    /// Sign the transaction using Dilithium5 private key
    pub fn sign(&mut self, private_key: &[u8], public_key: &[u8]) -> Result<(), String> {
        use pqcrypto_dilithium::dilithium5::*;
        use pqcrypto_traits::sign::{SecretKey, DetachedSignature as DetachedSignatureTrait};

        let sk = pqcrypto_dilithium::dilithium5::SecretKey::from_bytes(private_key)
            .map_err(|_| "Invalid Dilithium5 private key".to_string())?;

        // Set from and public_key BEFORE computing hash so verify will match
        self.public_key = hex::encode(public_key);
        self.from = Self::address_from_pubkey(public_key);

        let tx_hash = self.hash();
        let sig = detached_sign(tx_hash.as_bytes(), &sk);
        
        self.signature = hex::encode(sig.as_bytes());

        Ok(())
    }

    /// Verify the transaction's digital signature
    pub fn verify_signature(&self) -> bool {
        use pqcrypto_dilithium::dilithium5::*;
        use pqcrypto_traits::sign::{PublicKey, DetachedSignature};

        if self.signature.is_empty() || self.public_key.is_empty() {
            return false;
        }

        let pk_bytes = match hex::decode(&self.public_key) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let sig_bytes = match hex::decode(&self.signature) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let pk = match pqcrypto_dilithium::dilithium5::PublicKey::from_bytes(&pk_bytes) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let sig = match pqcrypto_dilithium::dilithium5::DetachedSignature::from_bytes(&sig_bytes) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let tx_hash = self.hash();
        verify_detached_signature(&sig, tx_hash.as_bytes(), &pk).is_ok()
    }

    /// Verify the sender address matches the public key
    pub fn verify_sender(&self) -> bool {
        if self.public_key.is_empty() {
            return false;
        }
        match hex::decode(&self.public_key) {
            Ok(pk_bytes) => {
                let derived_address = Self::address_from_pubkey(&pk_bytes);
                self.from == derived_address
            }
            Err(_) => false,
        }
    }

    /// Full structural validation (does NOT check balance or nonce against state)
    pub fn validate_structure(&self) -> TxValidationResult {
        if self.from.is_empty() {
            return TxValidationResult::MissingFields("from".into());
        }
        if self.to.is_empty() {
            return TxValidationResult::MissingFields("to".into());
        }
        if self.signature.is_empty() {
            return TxValidationResult::MissingFields("signature".into());
        }
        if self.public_key.is_empty() {
            return TxValidationResult::MissingFields("public_key".into());
        }
        if self.amount == 0 {
            return TxValidationResult::ZeroAmount;
        }
        if self.from == self.to {
            return TxValidationResult::SelfTransfer;
        }
        if !self.verify_sender() {
            return TxValidationResult::InvalidSignature;
        }
        if !self.verify_signature() {
            return TxValidationResult::InvalidSignature;
        }
        TxValidationResult::Valid
    }
}

/// Generate a fresh Dilithium5 keypair and return (public_key, secret_key) as byte vectors
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    use pqcrypto_dilithium::dilithium5::keypair;
    use pqcrypto_traits::sign::{PublicKey, SecretKey};
    let (pk, sk) = keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_signed_tx() -> (Transaction, Vec<u8>, Vec<u8>) {
        let (pk, sk) = generate_keypair();
        let to_addr = "0xrecipient_address_placeholder_for_testing_00";
        let mut tx = Transaction::new(
            String::new(), // will be filled by sign
            to_addr.to_string(),
            100,
            1,
            "test transfer".into(),
        );
        tx.sign(&sk, &pk).expect("signing should succeed");
        (tx, pk, sk)
    }

    #[test]
    fn test_transaction_hash_deterministic() {
        let tx = Transaction::new("alice".into(), "bob".into(), 50, 0, "memo".into());
        let h1 = tx.hash();
        let h2 = tx.hash();
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // SHA-256 hex = 64 chars
    }

    #[test]
    fn test_transaction_hash_changes_with_data() {
        let tx1 = Transaction::new("alice".into(), "bob".into(), 50, 0, "memo1".into());
        let tx2 = Transaction::new("alice".into(), "bob".into(), 50, 0, "memo2".into());
        assert_ne!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_sign_and_verify() {
        let (tx, _, _) = create_signed_tx();
        assert!(!tx.signature.is_empty());
        assert!(!tx.public_key.is_empty());
        assert!(tx.verify_signature(), "Valid signature should verify");
    }

    #[test]
    fn test_verify_sender_address() {
        let (tx, _, _) = create_signed_tx();
        assert!(tx.verify_sender(), "Sender address should match public key");
    }

    #[test]
    fn test_tampered_amount_fails_verification() {
        let (mut tx, _, _) = create_signed_tx();
        tx.amount = 999999; // Tamper with the amount
        assert!(!tx.verify_signature(), "Tampered tx should fail verification");
    }

    #[test]
    fn test_tampered_recipient_fails_verification() {
        let (mut tx, _, _) = create_signed_tx();
        tx.to = "0xattacker_address".to_string();
        assert!(!tx.verify_signature(), "Tampered recipient should fail");
    }

    #[test]
    fn test_empty_signature_fails() {
        let tx = Transaction::new("alice".into(), "bob".into(), 50, 0, "".into());
        assert!(!tx.verify_signature());
    }

    #[test]
    fn test_validate_structure_valid() {
        let (tx, _, _) = create_signed_tx();
        match tx.validate_structure() {
            TxValidationResult::Valid => {} // OK
            other => panic!("Expected Valid, got: {:?}", other),
        }
    }

    #[test]
    fn test_validate_zero_amount() {
        let (pk, sk) = generate_keypair();
        let mut tx = Transaction::new(String::new(), "0xbob".into(), 0, 0, "".into());
        tx.sign(&sk, &pk).unwrap();
        match tx.validate_structure() {
            TxValidationResult::ZeroAmount => {} // Expected
            other => panic!("Expected ZeroAmount, got: {:?}", other),
        }
    }

    #[test]
    fn test_validate_self_transfer() {
        let (pk, sk) = generate_keypair();
        let addr = Transaction::address_from_pubkey(&pk);
        let mut tx = Transaction::new(String::new(), addr.clone(), 50, 0, "".into());
        tx.sign(&sk, &pk).unwrap();
        match tx.validate_structure() {
            TxValidationResult::SelfTransfer => {} // Expected
            other => panic!("Expected SelfTransfer, got: {:?}", other),
        }
    }

    #[test]
    fn test_address_from_pubkey_deterministic() {
        let (pk, _) = generate_keypair();
        let addr1 = Transaction::address_from_pubkey(&pk);
        let addr2 = Transaction::address_from_pubkey(&pk);
        assert_eq!(addr1, addr2);
        assert!(addr1.starts_with("0x"));
        assert_eq!(addr1.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    fn test_different_keys_different_addresses() {
        let (pk1, _) = generate_keypair();
        let (pk2, _) = generate_keypair();
        let addr1 = Transaction::address_from_pubkey(&pk1);
        let addr2 = Transaction::address_from_pubkey(&pk2);
        assert_ne!(addr1, addr2);
    }

    #[test]
    fn test_tx_hash_deterministic() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx1 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 500, 1, "test".to_string());
        tx1.sign(&sk, &pk).unwrap();

        let mut tx2 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 500, 1, "test".to_string());
        tx2.timestamp = tx1.timestamp; // Force same timestamp for hash match
        tx2.sign(&sk, &pk).unwrap();

        assert_eq!(tx1.hash, tx2.hash);
    }

    #[test]
    fn test_tx_hash_changes_with_amount() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx1 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 500, 1, "".to_string());
        tx1.sign(&sk, &pk).unwrap();

        let mut tx2 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 501, 1, "".to_string());
        tx2.timestamp = tx1.timestamp;
        tx2.sign(&sk, &pk).unwrap();

        assert_ne!(tx1.hash, tx2.hash);
    }

    #[test]
    fn test_tx_hash_changes_with_nonce() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx1 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 500, 1, "".to_string());
        tx1.sign(&sk, &pk).unwrap();

        let mut tx2 = Transaction::new(from_addr.clone(), "0xrecipient".to_string(), 500, 2, "".to_string());
        tx2.timestamp = tx1.timestamp;
        tx2.sign(&sk, &pk).unwrap();

        assert_ne!(tx1.hash, tx2.hash);
    }

    #[test]
    fn test_tx_hash_changes_with_recipient() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx1 = Transaction::new(from_addr.clone(), "0xrecipient1".to_string(), 500, 1, "".to_string());
        tx1.sign(&sk, &pk).unwrap();

        let mut tx2 = Transaction::new(from_addr.clone(), "0xrecipient2".to_string(), 500, 1, "".to_string());
        tx2.timestamp = tx1.timestamp;
        tx2.sign(&sk, &pk).unwrap();

        assert_ne!(tx1.hash, tx2.hash);
    }

    #[test]
    fn test_tx_verify_fails_with_tampered_amount() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx = Transaction::new(from_addr, "0xrecipient".to_string(), 500, 1, "".to_string());
        tx.sign(&sk, &pk).unwrap();

        // Tamper
        tx.amount = 1000;
        
        match tx.verify() {
            TxValidationResult::InvalidSignature => {} // Expected
            other => panic!("Expected InvalidSignature due to tampered amount, got {:?}", other),
        }
    }

    #[test]
    fn test_tx_verify_fails_with_tampered_recipient() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx = Transaction::new(from_addr, "0xrecipient".to_string(), 500, 1, "".to_string());
        tx.sign(&sk, &pk).unwrap();

        // Tamper
        tx.to = "0xhacker".to_string();
        
        match tx.verify() {
            TxValidationResult::InvalidSignature => {} // Expected
            other => panic!("Expected InvalidSignature due to tampered recipient, got {:?}", other),
        }
    }

    #[test]
    fn test_tx_serialize_deserialize() {
        let (pk, sk) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let mut tx = Transaction::new(from_addr, "0xrecipient".to_string(), 1234, 42, "hello".to_string());
        tx.sign(&sk, &pk).unwrap();

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.hash, deserialized.hash);
        assert_eq!(tx.signature, deserialized.signature);
        assert_eq!(tx.amount, deserialized.amount);
        assert_eq!(tx.nonce, deserialized.nonce);
        
        match deserialized.verify() {
            TxValidationResult::Valid => {}
            other => panic!("Deserialized transaction failed verification: {:?}", other),
        }
    }

    #[test]
    fn test_unsigned_tx_fails_verification() {
        let (pk, _) = generate_keypair();
        let from_addr = Transaction::address_from_pubkey(&pk);
        let tx = Transaction::new(from_addr, "0xrecipient".to_string(), 500, 1, "".to_string());
        
        match tx.verify() {
            TxValidationResult::MissingSignature => {}
            other => panic!("Expected MissingSignature, got {:?}", other),
        }
    }
}
