/// # Quantum Lattice Cryptography
///
/// Post-Quantum Cryptography (PQC) using CRYSTALS-Dilithium (ML-DSA)
/// Provides lattice-based digital signatures resistant to quantum computer attacks.

use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};
use crate::shiva::nakshatra::Nakshatra;
use sha2::{Sha256, Digest};

pub struct QuantumSigner;

impl QuantumSigner {
    /// Generate a new Dilithium5 keypair
    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let (pk, sk) = keypair();
        (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
    }

    /// Sign data using Dilithium5 private key, entangled with Nakshatra
    pub fn sign(private_key: &[u8], data: &[u8], nakshatra_timestamp: u64) -> Result<Vec<u8>, String> {
        let sk = pqcrypto_dilithium::dilithium5::SecretKey::from_bytes(private_key)
            .map_err(|_| "Invalid Dilithium5 private key format".to_string())?;
        
        // 1. Core Post-Quantum Signature
        let sig = detached_sign(data, &sk);
        let mut sig_bytes = sig.as_bytes().to_vec();

        // 2. Cosmic Entanglement (Nakshatra-Lock)
        let naks = Nakshatra::current_from_time(nakshatra_timestamp);
        let mut hasher = Sha256::new();
        hasher.update((naks as u8).to_be_bytes());
        hasher.update(naks.frequency().to_be_bytes());
        let mask = hasher.finalize();

        // 3. Apply Nakshatra XOR mask to the signature
        for i in 0..sig_bytes.len() {
            sig_bytes[i] ^= mask[i % mask.len()];
        }

        Ok(sig_bytes)
    }

    /// Verify signature using Dilithium5 public key, unlocking with Nakshatra
    pub fn verify(public_key: &[u8], signature: &[u8], data: &[u8], nakshatra_timestamp: u64) -> bool {
        let pk = match pqcrypto_dilithium::dilithium5::PublicKey::from_bytes(public_key) {
            Ok(k) => k,
            Err(_) => return false,
        };
        
        let mut sig_bytes = signature.to_vec();

        // 1. Cosmic Disentanglement (Nakshatra-Unlock)
        let naks = Nakshatra::current_from_time(nakshatra_timestamp);
        let mut hasher = Sha256::new();
        hasher.update((naks as u8).to_be_bytes());
        hasher.update(naks.frequency().to_be_bytes());
        let mask = hasher.finalize();

        // 2. Remove Nakshatra XOR mask
        for i in 0..sig_bytes.len() {
            sig_bytes[i] ^= mask[i % mask.len()];
        }

        let sig = match pqcrypto_dilithium::dilithium5::DetachedSignature::from_bytes(&sig_bytes) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // 3. Post-Quantum Verification
        verify_detached_signature(&sig, data, &pk).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_keypair_generation() {
        let (pk, sk) = QuantumSigner::generate_keypair();
        // Dilithium5 keys are quite large
        assert!(pk.len() > 1000);
        assert!(sk.len() > 2000);
    }

    #[test]
    fn test_sign_and_verify_success() {
        let (pk, sk) = QuantumSigner::generate_keypair();
        let data = b"kasturi_quantum_transaction";
        let timestamp = 1680000000;

        let signature = QuantumSigner::sign(&sk, data, timestamp).unwrap();
        
        // Verification with same data and timestamp
        let valid = QuantumSigner::verify(&pk, &signature, data, timestamp);
        assert!(valid);
    }

    #[test]
    fn test_verify_fails_with_wrong_timestamp() {
        let (pk, sk) = QuantumSigner::generate_keypair();
        let data = b"kasturi_quantum_transaction";
        let sign_time = 1680000000;
        let verify_time = 1680000000 + 86400 * 5; // Different day/nakshatra

        let signature = QuantumSigner::sign(&sk, data, sign_time).unwrap();
        
        // Try verifying with different time
        let valid = QuantumSigner::verify(&pk, &signature, data, verify_time);
        // Sometimes it could be the same nakshatra if time diff is exactly multiple of 27 days, 
        // but 5 days is definitely a different nakshatra.
        assert!(!valid);
    }

    #[test]
    fn test_verify_fails_with_tampered_data() {
        let (pk, sk) = QuantumSigner::generate_keypair();
        let data = b"send 100 to alice";
        let timestamp = 1680000000;

        let signature = QuantumSigner::sign(&sk, data, timestamp).unwrap();
        
        let tampered_data = b"send 1000 to hacker";
        let valid = QuantumSigner::verify(&pk, &signature, tampered_data, timestamp);
        assert!(!valid);
    }

    #[test]
    fn test_verify_fails_with_wrong_public_key() {
        let (_, sk) = QuantumSigner::generate_keypair();
        let (wrong_pk, _) = QuantumSigner::generate_keypair();
        
        let data = b"some data";
        let timestamp = 1680000000;

        let signature = QuantumSigner::sign(&sk, data, timestamp).unwrap();
        
        let valid = QuantumSigner::verify(&wrong_pk, &signature, data, timestamp);
        assert!(!valid);
    }

    #[test]
    fn test_sign_fails_with_invalid_private_key() {
        let invalid_sk = vec![0u8; 32]; // Too short for Dilithium5
        let data = b"some data";
        let timestamp = 1680000000;

        let result = QuantumSigner::sign(&invalid_sk, data, timestamp);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_fails_with_invalid_public_key() {
        let invalid_pk = vec![0u8; 32]; // Too short
        let signature = vec![0u8; 4000];
        let data = b"some data";
        let timestamp = 1680000000;

        let valid = QuantumSigner::verify(&invalid_pk, &signature, data, timestamp);
        assert!(!valid);
    }
}
