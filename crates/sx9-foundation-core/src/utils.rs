//! CTAS-7 USIM Blockchain Utilities Module
//! Helper functions and blockchain maintenance
//! Follows CTAS-7 standards: ≤200 LOC

use super::types::*;
use super::crypto::*;
use super::manager::{UsimBlockchainManager, BlockchainStats};

#[cfg(not(feature = "embedded-firefly"))]
use std::collections::HashMap;

#[cfg(feature = "embedded-firefly")]
use heapless::FnvIndexMap as HashMap;

impl UsimBlockchainManager {
    /// Verify blockchain integrity
    pub fn verify_chain(&self) -> ChainVerificationResult {
        let mut result = ChainVerificationResult::new();
        result.block_count = self.blockchain.blocks.len();

        for i in 1..self.blockchain.blocks.len() {
            let current = &self.blockchain.blocks[i];
            let previous = &self.blockchain.blocks[i - 1];

            // Verify hash
            let calculated_hash = calculate_hash(
                current.index,
                &current.previous_hash,
                current.timestamp,
                &current.transactions,
                &current.merkle_root,
                current.nonce,
            );

            if calculated_hash != current.hash {
                result.add_error(current.index, format!("Invalid hash at block {}", current.index));
                continue;
            }

            // Verify chain linkage
            if current.previous_hash != previous.hash {
                result.add_error(current.index, format!("Broken chain at block {}", current.index));
                continue;
            }

            // Verify sequence
            if current.index != previous.index + 1 {
                result.add_error(current.index, format!("Invalid sequence at block {}", current.index));
                continue;
            }

            // Verify difficulty
            if !verify_difficulty(&current.hash, self.blockchain.difficulty) {
                result.add_error(current.index, format!("Hash doesn't meet difficulty at block {}", current.index));
                continue;
            }

            result.last_verified_block = current.index;
        }

        result
    }

    /// Get key from registry
    pub fn get_key(&self, fingerprint: &str) -> Option<&crate::usim_pgp_integration::UsimPgpKeyRef> {
        self.key_registry.get(fingerprint)
    }

    /// Get blockchain statistics
    pub fn get_stats(&self) -> BlockchainStats {
        let total_transactions = self.blockchain.blocks.iter()
            .map(|block| block.transactions.len())
            .sum();

        let key_registrations = self.blockchain.blocks.iter()
            .flat_map(|block| &block.transactions)
            .filter(|tx| matches!(tx, UsimTransaction::KeyRegistration { .. }))
            .count();

        let build_verifications = self.blockchain.blocks.iter()
            .flat_map(|block| &block.transactions)
            .filter(|tx| matches!(tx, UsimTransaction::BuildVerification { .. }))
            .count();

        BlockchainStats {
            total_blocks: self.blockchain.blocks.len(),
            total_transactions,
            key_registrations,
            build_verifications,
            current_difficulty: self.blockchain.difficulty,
            chain_size_bytes: self.estimate_chain_size(),
        }
    }

    /// Get block by index
    pub fn get_block(&self, index: u64) -> Option<&UsimBlock> {
        self.blockchain.blocks.iter().find(|block| block.index == index)
    }

    /// Get latest block
    pub fn get_latest_block(&self) -> Option<&UsimBlock> {
        self.blockchain.blocks.last()
    }

    /// Get chain height
    pub fn get_height(&self) -> u64 {
        self.blockchain.blocks.len() as u64 - 1 // Subtract 1 for genesis
    }

    /// Get difficulty
    pub fn get_difficulty(&self) -> u8 {
        self.blockchain.difficulty
    }

    // Private helper methods

    pub(super) fn adjust_difficulty(&mut self) {
        let recent_blocks = &self.blockchain.blocks[self.blockchain.blocks.len().saturating_sub(10)..];
        if recent_blocks.len() < 2 {
            return;
        }

        let time_taken = recent_blocks.last().unwrap().timestamp - recent_blocks.first().unwrap().timestamp;
        let target_time = self.blockchain.config.target_block_time * (recent_blocks.len() as u64 - 1);

        if time_taken < target_time / 2 && self.blockchain.difficulty < 4 {
            self.blockchain.difficulty += 1;
        } else if time_taken > target_time * 2 && self.blockchain.difficulty > 1 {
            self.blockchain.difficulty -= 1;
        }
    }

    pub(super) fn prune_old_blocks(&mut self) {
        let keep_count = self.blockchain.config.max_chain_length / 2;
        if self.blockchain.blocks.len() > keep_count {
            let remove_count = self.blockchain.blocks.len() - keep_count;
            self.blockchain.blocks.drain(1..remove_count + 1); // Keep genesis block

            // Update indices
            for (i, block) in self.blockchain.blocks.iter_mut().enumerate().skip(1) {
                block.index = i as u64;
            }
        }
    }

    pub(super) fn rebuild_key_registry(&mut self) {
        self.key_registry.clear();
        for block in &self.blockchain.blocks {
            for transaction in &block.transactions {
                if let UsimTransaction::KeyRegistration { key_ref, .. } = transaction {
                    self.key_registry.insert(key_ref.fingerprint.clone(), key_ref.clone());
                }
            }
        }
    }

    pub(super) fn estimate_chain_size(&self) -> usize {
        #[cfg(not(feature = "embedded-firefly"))]
        {
            serde_json::to_string(&self.blockchain).map(|s| s.len()).unwrap_or(0)
        }
        #[cfg(feature = "embedded-firefly")]
        {
            // Rough estimate for embedded
            self.blockchain.blocks.len() * 256 // ~256 bytes per block estimate
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_verification() {
        let manager = UsimBlockchainManager::new();
        let result = manager.verify_chain();
        assert!(result.valid);
        assert_eq!(result.block_count, 1);
    }

    #[test]
    fn test_blockchain_stats() {
        let manager = UsimBlockchainManager::new();
        let stats = manager.get_stats();
        assert_eq!(stats.total_blocks, 1);
        assert_eq!(stats.total_transactions, 0);
    }
}