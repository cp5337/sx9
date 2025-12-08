//! CTAS-7 USIM Blockchain Manager Module (Core)
//! Core blockchain manager structure
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};
use super::types::*;
use super::crypto::*;
use crate::usim_pgp_integration::UsimPgpKeyRef;

#[cfg(not(feature = "embedded-firefly"))]
use std::{collections::HashMap, vec::Vec};

#[cfg(feature = "embedded-firefly")]
use alloc::vec::Vec;
#[cfg(feature = "embedded-firefly")]
use heapless::FnvIndexMap as HashMap;

/// USIM Blockchain Manager
pub struct UsimBlockchainManager {
    pub blockchain: UsimBlockchain,
    pub pending_transactions: Vec<UsimTransaction>,
    pub key_registry: HashMap<String, UsimPgpKeyRef>,
}

/// Blockchain statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub key_registrations: usize,
    pub build_verifications: usize,
    pub current_difficulty: u8,
    pub chain_size_bytes: usize,
}

impl UsimBlockchainManager {
    /// Create new USIM blockchain
    pub fn new() -> Self {
        let config = ChainConfig::default();
        let genesis_block = Self::create_genesis_block();

        Self {
            blockchain: UsimBlockchain {
                blocks: vec![genesis_block],
                difficulty: 1,
                genesis_time: Self::get_timestamp(),
                config,
            },
            pending_transactions: Vec::new(),
            key_registry: HashMap::new(),
        }
    }

    /// Create genesis block
    fn create_genesis_block() -> UsimBlock {
        let timestamp = Self::get_timestamp();
        UsimBlock {
            index: 0,
            previous_hash: "0".repeat(64),
            timestamp,
            transactions: vec![],
            merkle_root: "genesis".to_string(),
            hash: calculate_hash(0, &"0".repeat(64), timestamp, &[], "genesis", 0),
            nonce: 0,
            miner: "CTAS-7-Genesis".to_string(),
        }
    }

    /// Create blockchain from configuration
    pub fn with_config(config: ChainConfig) -> Self {
        let genesis_block = Self::create_genesis_block();

        Self {
            blockchain: UsimBlockchain {
                blocks: vec![genesis_block],
                difficulty: config.min_difficulty,
                genesis_time: Self::get_timestamp(),
                config,
            },
            pending_transactions: Vec::new(),
            key_registry: HashMap::new(),
        }
    }

    /// Get blockchain configuration
    pub fn get_config(&self) -> &ChainConfig {
        &self.blockchain.config
    }

    /// Update blockchain configuration
    pub fn update_config(&mut self, config: ChainConfig) -> Result<(), String> {
        if config.min_difficulty > 10 {
            return Err("Difficulty too high for embedded systems".to_string());
        }

        if config.max_transactions_per_block == 0 {
            return Err("Must allow at least 1 transaction per block".to_string());
        }

        if config.max_chain_length < 10 {
            return Err("Chain length must be at least 10 blocks".to_string());
        }

        self.blockchain.config = config;
        Ok(())
    }

    /// Reset blockchain to genesis
    pub fn reset(&mut self) {
        let genesis_block = Self::create_genesis_block();
        self.blockchain.blocks = vec![genesis_block];
        self.blockchain.difficulty = self.blockchain.config.min_difficulty;
        self.blockchain.genesis_time = Self::get_timestamp();
        self.pending_transactions.clear();
        self.key_registry.clear();
    }

    /// Check if blockchain is empty (only genesis)
    pub fn is_empty(&self) -> bool {
        self.blockchain.blocks.len() <= 1
    }

    /// Get total transaction count across all blocks
    pub fn total_transactions(&self) -> usize {
        self.blockchain.blocks.iter()
            .map(|block| block.transactions.len())
            .sum()
    }

    /// Internal timestamp utility (made public for modules)
    pub fn get_timestamp() -> u64 {
        #[cfg(not(feature = "embedded-firefly"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }
        #[cfg(feature = "embedded-firefly")]
        {
            1732365200 // Fixed timestamp for embedded
        }
    }
}

impl Default for UsimBlockchainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let manager = UsimBlockchainManager::new();
        assert_eq!(manager.blockchain.blocks.len(), 1); // Genesis block
        assert_eq!(manager.blockchain.difficulty, 1);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_config_update() {
        let mut manager = UsimBlockchainManager::new();
        let mut config = ChainConfig::default();
        config.max_transactions_per_block = 5;

        assert!(manager.update_config(config).is_ok());
        assert_eq!(manager.get_config().max_transactions_per_block, 5);

        // Test invalid config
        let mut bad_config = ChainConfig::default();
        bad_config.min_difficulty = 15; // Too high
        assert!(manager.update_config(bad_config).is_err());
    }

    #[test]
    fn test_reset() {
        let mut manager = UsimBlockchainManager::new();
        manager.reset();
        assert!(manager.is_empty());
        assert_eq!(manager.total_transactions(), 0);
    }
}