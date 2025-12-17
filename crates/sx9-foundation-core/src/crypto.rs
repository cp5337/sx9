//! CTAS-7 USIM Blockchain Cryptographic Module
//!
//! Implements RFC-9009 (Quantum Cryptographic Architecture) and RFC-9001 (Trivariate Hash)
//! Replaces legacy Blake3 with TrivariateMurmur for post-quantum readiness.
//!
//! Standards:
//! - Hash: TrivariateMurmur (SHA-3 + Murmur3-128 mix)
//! - Signatures: Dilithium (Placeholder for post-quantum)
//! - Transport: TransportProfile aware

use serde::{Deserialize, Serialize};
use super::types::{UsimTransaction, MiningResult};
use crate::trivariate_hash_v731::{TrivariateHashEngineV731};
use crate::neural_mux::TransportProfile;

#[cfg(not(feature = "embedded-firefly"))]
use std::time::Instant;

#[cfg(feature = "embedded-firefly")]
use alloc::{string::String, vec::Vec};

/// Crypto Provider Trait (RFC-9009)
pub trait CryptoProvider {
    fn hash(&self, data: &[u8]) -> String;
    fn sign(&self, data: &[u8], key: &[u8]) -> String;
    fn verify(&self, data: &[u8], signature: &str, key: &[u8]) -> bool;
    fn get_transport_profile(&self) -> TransportProfile;
}

/// Standard CTAS-7 Crypto Provider (Hybrid Mode)
pub struct StandardCryptoProvider {
    engine: TrivariateHashEngineV731,
    transport: TransportProfile,
}

impl StandardCryptoProvider {
    pub fn new(transport: TransportProfile) -> Self {
        Self {
            engine: TrivariateHashEngineV731::new(),
            transport,
        }
    }
}

impl CryptoProvider for StandardCryptoProvider {
    fn hash(&self, data: &[u8]) -> String {
        // RFC-9001: Use Trivariate Engine (Base96 128-bit)
        self.engine.generate_hash_from_bytes(data)
    }

    fn sign(&self, data: &[u8], _key: &[u8]) -> String {
        // RFC-9009: Placeholder for Dilithium signature
        // In Hybrid mode, we simulate quantum signature with extended hash
        let hash = self.hash(data);
        format!("dilithium_sig:{}", hash)
    }

    fn verify(&self, data: &[u8], signature: &str, _key: &[u8]) -> bool {
        let hash = self.hash(data);
        signature.ends_with(&hash)
    }

    fn get_transport_profile(&self) -> TransportProfile {
        self.transport
    }
}

/// Calculate block hash using Trivariate Hash (RFC-9001)
/// Replaces legacy Blake3 implementation
pub fn calculate_hash(
    index: u64,
    previous_hash: &str,
    timestamp: u64,
    transactions: &[UsimTransaction],
    merkle_root: &str,
    nonce: u32,
) -> String {
    let engine = TrivariateHashEngineV731::new();
    let mut buffer = Vec::new();

    buffer.extend_from_slice(&index.to_le_bytes());
    buffer.extend_from_slice(previous_hash.as_bytes());
    buffer.extend_from_slice(&timestamp.to_le_bytes());
    buffer.extend_from_slice(merkle_root.as_bytes());
    buffer.extend_from_slice(&nonce.to_le_bytes());

    // Serialize transactions to bytes for hashing
    for tx in transactions {
        match tx {
            UsimTransaction::KeyRegistration { key_ref, registrar, timestamp, .. } => {
                buffer.extend_from_slice(b"key_reg");
                buffer.extend_from_slice(key_ref.fingerprint.as_bytes());
                buffer.extend_from_slice(registrar.as_bytes());
                buffer.extend_from_slice(&timestamp.to_le_bytes());
            },
            UsimTransaction::BuildVerification { verifier, timestamp, .. } => {
                buffer.extend_from_slice(b"build_ver");
                buffer.extend_from_slice(verifier.as_bytes());
                buffer.extend_from_slice(&timestamp.to_le_bytes());
            },
            UsimTransaction::KeyRevocation { key_fingerprint, revoker, timestamp, .. } => {
                buffer.extend_from_slice(b"key_rev");
                buffer.extend_from_slice(key_fingerprint.as_bytes());
                buffer.extend_from_slice(revoker.as_bytes());
                buffer.extend_from_slice(&timestamp.to_le_bytes());
            },
            UsimTransaction::TrustEndorsement { endorser, endorsed, trust_level, timestamp } => {
                buffer.extend_from_slice(b"trust");
                buffer.extend_from_slice(endorser.as_bytes());
                buffer.extend_from_slice(endorsed.as_bytes());
                buffer.push(*trust_level);
                buffer.extend_from_slice(&timestamp.to_le_bytes());
            },
        }
    }

    // Generate Base96 Trivariate Hash
    engine.generate_hash_from_bytes(&buffer)
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

/// Mine block with Trivariate Proof-of-Work
/// Difficulty is checked against Base96 '0' prefix
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
    // Prefix target: Base96 '0' char repeated difficulty times
    let target_prefix = "0".repeat(difficulty as usize);

    loop {
        let hash = calculate_hash(index, previous_hash, timestamp, transactions, merkle_root, nonce);

        if hash.starts_with(&target_prefix) {
            let mining_time = start_time.elapsed().as_millis() as u64;
            return MiningResult::success(hash, nonce, mining_time, difficulty);
        }

        nonce += 1;

        // Prevent infinite loop (Safety Valve)
        if nonce > 1000000 {
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
    let target_prefix = "0".repeat(difficulty as usize);

    loop {
        let hash = calculate_hash(index, previous_hash, timestamp, transactions, merkle_root, nonce);

        if hash.starts_with(&target_prefix) {
            return MiningResult::success(hash, nonce, 0, difficulty);
        }

        nonce += 1;

        if nonce > 10000 {
            return MiningResult::failure();
        }
    }
}

/// Hash a single transaction using Trivariate Hash
fn hash_transaction(tx: &UsimTransaction) -> String {
    // Re-use logic or simplify
    // For single transaction, just wrap in list and hash with dummy block data
    // Or direct serialization
    let engine = TrivariateHashEngineV731::new();
    let mut buffer = Vec::new();
    
    // Quick serialization of tx
    match tx {
         UsimTransaction::KeyRegistration { key_ref, registrar, .. } => {
             buffer.extend_from_slice(key_ref.fingerprint.as_bytes());
             buffer.extend_from_slice(registrar.as_bytes());
         },
         // ... others simplified for brevity in this helper
         _ => buffer.extend_from_slice(b"tx_generic"),
    }
    
    engine.generate_hash_from_bytes(&buffer)
}

/// Hash a pair of strings
fn hash_pair(left: &str, right: &str) -> String {
    let engine = TrivariateHashEngineV731::new();
    let mut buffer = Vec::new();
    buffer.extend_from_slice(left.as_bytes());
    buffer.extend_from_slice(right.as_bytes());
    engine.generate_hash_from_bytes(&buffer)
}

/// Verify hash meets difficulty requirement
pub fn verify_difficulty(hash: &str, difficulty: u8) -> bool {
    hash.starts_with(&"0".repeat(difficulty as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivariate_hash_calculation() {
        let hash = calculate_hash(1, "prev", 12345, &[], "merkle", 0);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16); // Base96 128-bit = 16 chars
    }

    #[test]
    fn test_difficulty_verification() {
        // Base96 '0' is the first char
        assert!(verify_difficulty("000abc", 3));
        assert!(!verify_difficulty("00abc", 3));
    }
}
