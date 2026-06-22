/// # Merkle Tree — कस्तूरी मर्कल वृक्ष
///
/// A proper binary Merkle Tree for KasturiChain blocks.
/// Every block contains a Merkle root that cryptographically commits
/// to all transactions in the block, enabling efficient inclusion proofs.

use sha2::{Sha256, Digest};

/// A node in the Merkle Tree
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

/// A proof element for Merkle inclusion verification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleProofStep {
    /// The sibling hash at this level
    pub hash: String,
    /// True if the sibling is on the left side
    pub is_left: bool,
}

/// The Merkle Tree
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    /// All leaf hashes (transaction hashes) in order
    pub leaves: Vec<String>,
}

impl MerkleTree {
    /// Compute SHA-256 hash with Domain Separation for Leaves (Prefix 0x00)
    pub fn hash_leaf(data: &[u8]) -> String {
        // --- THE APOCALYPSE PATCH: Merkle Tree Domain Separation ---
        // Prepend 0x00 to leaf nodes to mathematically distinguish them from internal nodes
        let mut hasher = Sha256::new();
        hasher.update(&[0x00]);
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Compute hash of two concatenated hashes (Internal Node, Prefix 0x01)
    pub fn hash_pair(left: &str, right: &str) -> String {
        // --- THE APOCALYPSE PATCH: Merkle Tree Domain Separation ---
        // Prepend 0x01 to internal nodes to prevent Second Preimage attacks (Leaf-Node Confusion)
        let combined = format!("{}{}", left, right);
        let mut hasher = Sha256::new();
        hasher.update(&[0x01]);
        hasher.update(combined.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Build a Merkle Tree from a list of transaction data (serialized as bytes)
    pub fn new(transactions: &[Vec<u8>]) -> Self {
        if transactions.is_empty() {
            return Self {
                root: None,
                leaves: Vec::new(),
            };
        }

        // Step 1: Hash all leaves
        let mut leaf_hashes: Vec<String> = transactions
            .iter()
            .map(|tx| Self::hash_leaf(tx))
            .collect();

        let leaves = leaf_hashes.clone();

        // Step 2: Build tree bottom-up
        let root = Self::build_tree(&mut leaf_hashes);

        Self {
            root: Some(root),
            leaves,
        }
    }

    /// Build tree recursively from a list of hashes
    fn build_tree(hashes: &mut Vec<String>) -> MerkleNode {
        if hashes.len() == 1 {
            return MerkleNode {
                hash: hashes[0].clone(),
                left: None,
                right: None,
            };
        }

        // If odd number of hashes, duplicate the last one
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut parent_hashes = Vec::new();
        let mut parent_nodes = Vec::new();

        for chunk in hashes.chunks(2) {
            let left_hash = &chunk[0];
            let right_hash = &chunk[1];
            let parent_hash = Self::hash_pair(left_hash, right_hash);

            parent_hashes.push(parent_hash.clone());
            parent_nodes.push((
                parent_hash,
                left_hash.clone(),
                right_hash.clone(),
            ));
        }

        if parent_hashes.len() == 1 {
            let (hash, left_h, right_h) = &parent_nodes[0];
            return MerkleNode {
                hash: hash.clone(),
                left: Some(Box::new(MerkleNode {
                    hash: left_h.clone(),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(MerkleNode {
                    hash: right_h.clone(),
                    left: None,
                    right: None,
                })),
            };
        }

        Self::build_tree(&mut parent_hashes)
    }

    /// Get the Merkle root hash
    pub fn root_hash(&self) -> String {
        match &self.root {
            Some(node) => node.hash.clone(),
            None => "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        }
    }

    /// Generate an inclusion proof for a given leaf index
    pub fn generate_proof(&self, leaf_index: usize) -> Option<Vec<MerkleProofStep>> {
        if leaf_index >= self.leaves.len() || self.leaves.is_empty() {
            return None;
        }

        let mut proof = Vec::new();
        let mut current_hashes = self.leaves.clone();
        let mut index = leaf_index;

        while current_hashes.len() > 1 {
            // Duplicate last if odd
            if current_hashes.len() % 2 != 0 {
                current_hashes.push(current_hashes.last().unwrap().clone());
            }

            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
            let is_left = index % 2 != 0; // sibling is on the left if we are on the right

            proof.push(MerkleProofStep {
                hash: current_hashes[sibling_index].clone(),
                is_left,
            });

            // Move up to parent level
            let mut next_level = Vec::new();
            for chunk in current_hashes.chunks(2) {
                next_level.push(Self::hash_pair(&chunk[0], &chunk[1]));
            }
            current_hashes = next_level;
            index /= 2;
        }

        Some(proof)
    }

    /// Verify an inclusion proof against a known root hash
    pub fn verify_proof(leaf_data: &[u8], proof: &[MerkleProofStep], root_hash: &str) -> bool {
        let mut current_hash = Self::hash_leaf(leaf_data);

        for step in proof {
            if step.is_left {
                current_hash = Self::hash_pair(&step.hash, &current_hash);
            } else {
                current_hash = Self::hash_pair(&current_hash, &step.hash);
            }
        }

        current_hash == root_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new(&[]);
        assert!(tree.root.is_none());
        assert_eq!(tree.root_hash(), "0000000000000000000000000000000000000000000000000000000000000000");
        assert!(tree.leaves.is_empty());
    }

    #[test]
    fn test_single_leaf() {
        let tx = vec![b"transaction_1".to_vec()];
        let tree = MerkleTree::new(&tx);
        assert!(tree.root.is_some());
        assert_eq!(tree.leaves.len(), 1);
        // Root should equal the hash of the single leaf
        let expected = MerkleTree::hash_leaf(b"transaction_1");
        assert_eq!(tree.root_hash(), expected);
    }

    #[test]
    fn test_two_leaves() {
        let txs = vec![b"tx_a".to_vec(), b"tx_b".to_vec()];
        let tree = MerkleTree::new(&txs);
        assert_eq!(tree.leaves.len(), 2);

        let h_a = MerkleTree::hash_leaf(b"tx_a");
        let h_b = MerkleTree::hash_leaf(b"tx_b");
        let expected_root = MerkleTree::hash_pair(&h_a, &h_b);
        assert_eq!(tree.root_hash(), expected_root);
    }

    #[test]
    fn test_odd_number_of_leaves() {
        let txs = vec![
            b"tx_1".to_vec(),
            b"tx_2".to_vec(),
            b"tx_3".to_vec(),
        ];
        let tree = MerkleTree::new(&txs);
        assert_eq!(tree.leaves.len(), 3);
        // Root should be deterministic
        let root = tree.root_hash();
        assert!(!root.is_empty());
        assert_ne!(root, "0000000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn test_four_leaves() {
        let txs = vec![
            b"alpha".to_vec(),
            b"beta".to_vec(),
            b"gamma".to_vec(),
            b"delta".to_vec(),
        ];
        let tree = MerkleTree::new(&txs);
        assert_eq!(tree.leaves.len(), 4);

        let h0 = MerkleTree::hash_leaf(b"alpha");
        let h1 = MerkleTree::hash_leaf(b"beta");
        let h2 = MerkleTree::hash_leaf(b"gamma");
        let h3 = MerkleTree::hash_leaf(b"delta");
        let h01 = MerkleTree::hash_pair(&h0, &h1);
        let h23 = MerkleTree::hash_pair(&h2, &h3);
        let expected_root = MerkleTree::hash_pair(&h01, &h23);
        assert_eq!(tree.root_hash(), expected_root);
    }

    #[test]
    fn test_proof_generation_and_verification() {
        let txs = vec![
            b"tx_0".to_vec(),
            b"tx_1".to_vec(),
            b"tx_2".to_vec(),
            b"tx_3".to_vec(),
        ];
        let tree = MerkleTree::new(&txs);
        let root = tree.root_hash();

        // Verify each leaf's inclusion proof
        for i in 0..txs.len() {
            let proof = tree.generate_proof(i).expect("proof should exist");
            assert!(
                MerkleTree::verify_proof(&txs[i], &proof, &root),
                "Proof verification failed for leaf {}", i
            );
        }
    }

    #[test]
    fn test_proof_fails_for_wrong_data() {
        let txs = vec![b"real_tx".to_vec(), b"other_tx".to_vec()];
        let tree = MerkleTree::new(&txs);
        let root = tree.root_hash();

        let proof = tree.generate_proof(0).unwrap();
        // Verify with tampered data should fail
        assert!(!MerkleTree::verify_proof(b"fake_tx", &proof, &root));
    }

    #[test]
    fn test_proof_fails_for_wrong_root() {
        let txs = vec![b"tx_a".to_vec(), b"tx_b".to_vec()];
        let tree = MerkleTree::new(&txs);

        let proof = tree.generate_proof(0).unwrap();
        let fake_root = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert!(!MerkleTree::verify_proof(&txs[0], &proof, fake_root));
    }

    #[test]
    fn test_deterministic_roots() {
        let txs = vec![b"hello".to_vec(), b"world".to_vec()];
        let tree1 = MerkleTree::new(&txs);
        let tree2 = MerkleTree::new(&txs);
        assert_eq!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn test_different_order_different_root() {
        let txs_a = vec![b"first".to_vec(), b"second".to_vec()];
        let txs_b = vec![b"second".to_vec(), b"first".to_vec()];
        let tree_a = MerkleTree::new(&txs_a);
        let tree_b = MerkleTree::new(&txs_b);
        assert_ne!(tree_a.root_hash(), tree_b.root_hash());
    }

    #[test]
    fn test_out_of_bounds_proof() {
        let txs = vec![b"only_one".to_vec()];
        let tree = MerkleTree::new(&txs);
        assert!(tree.generate_proof(5).is_none());
    }
}
