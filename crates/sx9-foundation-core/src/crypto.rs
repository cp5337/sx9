//! CTAS-7 USIM Blockchain Cryptographic Module
//! Hash calculation and cryptographic utilities
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};
use super::types::{UsimTransaction, MiningResult};

#[cfg(not(feature = "embedded-firefly"))]
use std::time::Instant;

#[cfg(feature = "embedded-firefly")]
use alloc::{string::String, vec::Vec};

/// Calculate block hash using Blake3 (Tesla/SpaceX grade)
pub fn calculate_hash(
    index: u64,
    previous_hash: &str,
    timestamp: u64,
    transactions: &[UsimTransaction],
    merkle_root: &str,
    nonce: u32,
) -> String {
    let mut hasher = HashEngine::new();

    hasher.update(&index.to_le_bytes());
    hasher.update(previous_hash.as_bytes());
    hasher.update(&timestamp.to_le_bytes());
    hasher.update(merkle_root.as_bytes());
    hasher.update(&nonce.to_le_bytes());

    // Hash transaction data
    for tx in transactions {
        match tx {
            UsimTransaction::KeyRegistration { key_ref, registrar, timestamp, .. } => {
                hasher.update(b"key_reg");
                hasher.update(key_ref.fingerprint.as_bytes());
                hasher.update(registrar.as_bytes());
                hasher.update(&timestamp.to_le_bytes());
            },
            UsimTransaction::BuildVerification { verifier, timestamp, .. } => {
                hasher.update(b"build_ver");
                hasher.update(verifier.as_bytes());
                hasher.update(&timestamp.to_le_bytes());
            },
            UsimTransaction::KeyRevocation { key_fingerprint, revoker, timestamp, .. } => {
                hasher.update(b"key_rev");
                hasher.update(key_fingerprint.as_bytes());
                hasher.update(revoker.as_bytes());
                hasher.update(&timestamp.to_le_bytes());
            },
            UsimTransaction::TrustEndorsement { endorser, endorsed, trust_level, timestamp } => {
                hasher.update(b"trust");
                hasher.update(endorser.as_bytes());
                hasher.update(endorsed.as_bytes());
                hasher.update(&[*trust_level]);
                hasher.update(&timestamp.to_le_bytes());
            },
        }
    }

    format!("{}", hasher.finalize().to_hex())
}

/// Calculate Merkle root of transactions
pub fn calculate_merkle_root(transactions: &[UsimTransaction]) -> String {
    if transactions.is_empty() {
        return "empty".to_string();
    }

    if transactions.len() == 1 {
        return hash_transaction(&transactions[0]);
    }

    let mut hashes: Vec<String> = transactions.iter().map(hash_transaction).collect();

    while hashes.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in hashes.chunks(2) {
            if chunk.len() == 2 {
                next_level.push(hash_pair(&chunk[0], &chunk[1]));
            } else {
                next_level.push(chunk[0].clone());
            }
        }

        hashes = next_level;
    }

    hashes[0].clone()
}

/// Mine block with lightweight proof-of-work
#[cfg(not(feature = "embedded-firefly"))]
pub fn mine_block_pow(
    index: u64,
    previous_hash: &str,
    timestamp: u64,
    transactions: &[UsimTransaction],
    merkle_root: &str,
    difficulty: u8,
) -> MiningResult {
    let start_time = Instant::now();
    let mut nonce = 0u32;

    loop {
        let hash = calculate_hash(index, previous_hash, timestamp, transactions, merkle_root, nonce);

        // Very lightweight difficulty - just check for leading zeros
        if hash.starts_with(&"0".repeat(difficulty as usize)) {
            let mining_time = start_time.elapsed().as_millis() as u64;
            return MiningResult::success(hash, nonce, mining_time, difficulty);
        }

        nonce += 1;

        // Prevent infinite loop in embedded systems
        if nonce > 100000 {
            return MiningResult::failure();
        }
    }
}

/// Embedded mining (simplified)
#[cfg(feature = "embedded-firefly")]
pub fn mine_block_pow(
    index: u64,
    previous_hash: &str,
    timestamp: u64,
    transactions: &[UsimTransaction],
    merkle_root: &str,
    difficulty: u8,
) -> MiningResult {
    let mut nonce = 0u32;

    loop {
        let hash = calculate_hash(index, previous_hash, timestamp, transactions, merkle_root, nonce);

        if hash.starts_with(&"0".repeat(difficulty as usize)) {
            return MiningResult::success(hash, nonce, 0, difficulty);
        }

        nonce += 1;

        if nonce > 10000 { // Lower limit for embedded
            return MiningResult::failure();
        }
    }
}

/// Hash a single transaction
fn hash_transaction(tx: &UsimTransaction) -> String {
    let mut hasher = HashEngine::new();

    match tx {
        UsimTransaction::KeyRegistration { key_ref, registrar, timestamp, .. } => {
            hasher.update(b"key_reg");
            hasher.update(key_ref.fingerprint.as_bytes());
            hasher.update(registrar.as_bytes());
            hasher.update(&timestamp.to_le_bytes());
        },
        UsimTransaction::BuildVerification { verifier, timestamp, .. } => {
            hasher.update(b"build_ver");
            hasher.update(verifier.as_bytes());
            hasher.update(&timestamp.to_le_bytes());
        },
        UsimTransaction::KeyRevocation { key_fingerprint, revoker, timestamp, .. } => {
            hasher.update(b"key_rev");
            hasher.update(key_fingerprint.as_bytes());
            hasher.update(revoker.as_bytes());
            hasher.update(&timestamp.to_le_bytes());
        },
        UsimTransaction::TrustEndorsement { endorser, endorsed, trust_level, timestamp } => {
            hasher.update(b"trust");
            hasher.update(endorser.as_bytes());
            hasher.update(endorsed.as_bytes());
            hasher.update(&[*trust_level]);
            hasher.update(&timestamp.to_le_bytes());
        },
    }

    format!("{}", hasher.finalize().to_hex())
}

/// Hash a pair of strings
fn hash_pair(left: &str, right: &str) -> String {
    let mut hasher = HashEngine::new();
    hasher.update(left.as_bytes());
    hasher.update(right.as_bytes());
    format!("{}", hasher.finalize().to_hex())
}

/// Verify hash meets difficulty requirement
pub fn verify_difficulty(hash: &str, difficulty: u8) -> bool {
    hash.starts_with(&"0".repeat(difficulty as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_calculation() {
        let hash = calculate_hash(1, "prev", 12345, &[], "merkle", 0);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // Blake3 produces 32 bytes = 64 hex chars
    }

    #[test]
    fn test_difficulty_verification() {
        assert!(verify_difficulty("000abc", 3));
        assert!(!verify_difficulty("00abc", 3));
        assert!(verify_difficulty("0abc", 1));
    }

    #[test]
    fn test_merkle_root_empty() {
        let root = calculate_merkle_root(&[]);
        assert_eq!(root, "empty");
    }
}