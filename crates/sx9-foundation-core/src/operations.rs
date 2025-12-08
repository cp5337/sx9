//! CTAS-7 USIM Blockchain Operations Module
//! Blockchain transaction operations
//! Follows CTAS-7 standards: ≤200 LOC

use super::types::*;
use super::crypto::*;
use super::manager::UsimBlockchainManager;
use crate::usim_pgp_integration::{UsimPgpKeyRef, UsimBuildSignature, UsimVerificationResult};
use crate::unicode_key_compression::CompressedKey;

#[cfg(not(feature = "embedded-firefly"))]
use std::vec::Vec;

#[cfg(feature = "embedded-firefly")]
use alloc::vec::Vec;

impl UsimBlockchainManager {
    /// Add PGP key to blockchain
    pub fn register_key(&mut self, key_ref: UsimPgpKeyRef, compressed_key: CompressedKey, registrar: &str) -> Result<(), String> {
        let transaction = UsimTransaction::KeyRegistration {
            key_ref: key_ref.clone(),
            compressed_key,
            registrar: registrar.to_string(),
            timestamp: Self::get_timestamp(),
        };

        self.pending_transactions.push(transaction);
        self.key_registry.insert(key_ref.fingerprint.clone(), key_ref);

        // Auto-mine if we have enough transactions
        if self.pending_transactions.len() >= self.blockchain.config.max_transactions_per_block {
            self.mine_block(registrar)?;
        }

        Ok(())
    }

    /// Add build verification to blockchain
    pub fn add_build_verification(&mut self, signature: UsimBuildSignature, verification: UsimVerificationResult, verifier: &str) -> Result<(), String> {
        let transaction = UsimTransaction::BuildVerification {
            signature,
            verification,
            verifier: verifier.to_string(),
            timestamp: Self::get_timestamp(),
        };

        self.pending_transactions.push(transaction);

        // Auto-mine if we have enough transactions
        if self.pending_transactions.len() >= self.blockchain.config.max_transactions_per_block {
            self.mine_block(verifier)?;
        }

        Ok(())
    }

    /// Revoke a compromised key
    pub fn revoke_key(&mut self, key_fingerprint: String, reason: String, revoker: &str) -> Result<(), String> {
        let transaction = UsimTransaction::KeyRevocation {
            key_fingerprint,
            reason,
            revoker: revoker.to_string(),
            timestamp: Self::get_timestamp(),
        };

        self.pending_transactions.push(transaction);

        // Auto-mine if we have enough transactions
        if self.pending_transactions.len() >= self.blockchain.config.max_transactions_per_block {
            self.mine_block(revoker)?;
        }

        Ok(())
    }

    /// Add trust endorsement
    pub fn add_trust_endorsement(&mut self, endorser: String, endorsed: String, trust_level: u8, signer: &str) -> Result<(), String> {
        if trust_level > 100 {
            return Err("Trust level cannot exceed 100".to_string());
        }

        let transaction = UsimTransaction::TrustEndorsement {
            endorser,
            endorsed,
            trust_level,
            timestamp: Self::get_timestamp(),
        };

        self.pending_transactions.push(transaction);

        // Auto-mine if we have enough transactions
        if self.pending_transactions.len() >= self.blockchain.config.max_transactions_per_block {
            self.mine_block(signer)?;
        }

        Ok(())
    }

    /// Mine a new block (lightweight proof-of-work)
    pub fn mine_block(&mut self, miner: &str) -> Result<(), String> {
        if self.pending_transactions.is_empty() {
            return Err("No pending transactions to mine".to_string());
        }

        let previous_block = self.blockchain.blocks.last().unwrap();
        let index = previous_block.index + 1;
        let timestamp = Self::get_timestamp();
        let transactions = self.pending_transactions.drain(..).collect::<Vec<_>>();
        let merkle_root = calculate_merkle_root(&transactions);

        // Use crypto module for mining
        let mining_result = mine_block_pow(
            index,
            &previous_block.hash,
            timestamp,
            &transactions,
            &merkle_root,
            self.blockchain.difficulty,
        );

        if !mining_result.success {
            return Err("Mining failed - difficulty too high for embedded system".to_string());
        }

        let block = UsimBlock {
            index,
            previous_hash: previous_block.hash.clone(),
            timestamp,
            transactions,
            merkle_root,
            hash: mining_result.block_hash,
            nonce: mining_result.nonce,
            miner: miner.to_string(),
        };

        self.blockchain.blocks.push(block);

        // Adjust difficulty (very conservatively for embedded)
        if self.blockchain.blocks.len() % 10 == 0 {
            self.adjust_difficulty();
        }

        // Prune old blocks if chain gets too long for embedded
        if self.blockchain.blocks.len() > self.blockchain.config.max_chain_length {
            self.prune_old_blocks();
        }

        Ok(())
    }

    /// Force mine pending transactions (for testing)
    pub fn force_mine(&mut self, miner: &str) -> Result<(), String> {
        if self.pending_transactions.is_empty() {
            // Add a dummy transaction if none exist
            let transaction = UsimTransaction::TrustEndorsement {
                endorser: "system".to_string(),
                endorsed: "genesis".to_string(),
                trust_level: 100,
                timestamp: Self::get_timestamp(),
            };
            self.pending_transactions.push(transaction);
        }

        self.mine_block(miner)
    }

    /// Get pending transaction count
    pub fn pending_count(&self) -> usize {
        self.pending_transactions.len()
    }

    /// Clear all pending transactions (emergency)
    pub fn clear_pending(&mut self) {
        self.pending_transactions.clear();
    }

}