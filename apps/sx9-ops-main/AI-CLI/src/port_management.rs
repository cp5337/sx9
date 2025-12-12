//! Port Management for CTAS AI CLI
//! 
//! Integrates with existing port blocks and provides CLI commands for port management.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Port block definitions from the existing system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortBlock {
    pub name: String,
    pub base_port: u16,
    pub port_count: u16,
    pub purpose: String,
    pub sister_offset: Option<u16>,
}

/// Service status in a port block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Active { port: u16, response_time_ms: u64 },
    Inactive { port: u16 },
    Failed { port: u16, error: String },
    Fallback { primary_port: u16, fallback_port: u16 },
}

/// Port management for AI CLI
#[derive(Debug)]
pub struct PortManager {
    blocks: HashMap<String, PortBlock>,
    service_status: HashMap<String, ServiceStatus>,
}

impl PortManager {
    /// Create new port manager with standard CTAS blocks
    pub fn new() -> Self {
        let mut blocks = HashMap::new();
        
        // BLOCK-A: Frontend Services [17152-17167]
        blocks.insert("BLOCK-A".to_string(), PortBlock {
            name: "BLOCK-A".to_string(),
            base_port: 17152,
            port_count: 16,
            purpose: "Frontend Services".to_string(),
            sister_offset: Some(32768), // ALT-A at 49920
        });
        
        // BLOCK-B: AI/ML Services [17173-17188]
        blocks.insert("BLOCK-B".to_string(), PortBlock {
            name: "BLOCK-B".to_string(),
            base_port: 17173,
            port_count: 16,
            purpose: "AI/ML Services".to_string(),
            sister_offset: Some(32768),
        });
        
        // BLOCK-C: Database Cluster [17194-17209]
        blocks.insert("BLOCK-C".to_string(), PortBlock {
            name: "BLOCK-C".to_string(),
            base_port: 17194,
            port_count: 16,
            purpose: "Database Cluster".to_string(),
            sister_offset: Some(32768),
        });
        
        // BLOCK-D: MCP Services [17215-17230]
        blocks.insert("BLOCK-D".to_string(), PortBlock {
            name: "BLOCK-D".to_string(),
            base_port: 17215,
            port_count: 16,
            purpose: "MCP Services".to_string(),
            sister_offset: Some(32768),
        });
        
        Self {
            blocks,
            service_status: HashMap::new(),
        }
    }
    
    /// Get port for a service in a block
    pub fn get_service_port(&self, block_name: &str, service_index: u16) -> Result<u16> {
        let block = self.blocks.get(block_name)
            .ok_or_else(|| anyhow::anyhow!("Block {} not found", block_name))?;
            
        if service_index >= block.port_count {
            return Err(anyhow::anyhow!("Service index {} out of range for block {}", service_index, block_name));
        }
        
        Ok(block.base_port + service_index)
    }
    
    /// Get sister port for failover
    pub fn get_sister_port(&self, port: u16) -> Option<u16> {
        for block in self.blocks.values() {
            if port >= block.base_port && port < (block.base_port + block.port_count) {
                return block.sister_offset.map(|offset| port + offset);
            }
        }
        None
    }
    
    /// Check if port is in a valid block
    pub fn is_valid_port(&self, port: u16) -> bool {
        for block in self.blocks.values() {
            if port >= block.base_port && port < (block.base_port + block.port_count) {
                return true;
            }
        }
        false
    }
    
    /// Get block info for a port
    pub fn get_block_for_port(&self, port: u16) -> Option<&PortBlock> {
        for block in self.blocks.values() {
            if port >= block.base_port && port < (block.base_port + block.port_count) {
                return Some(block);
            }
        }
        None
    }
    
    /// List all blocks
    pub fn list_blocks(&self) -> Vec<&PortBlock> {
        self.blocks.values().collect()
    }
    
    /// Get block status summary
    pub fn get_block_status(&self, block_name: &str) -> Result<BlockStatus> {
        let block = self.blocks.get(block_name)
            .ok_or_else(|| anyhow::anyhow!("Block {} not found", block_name))?;
            
        let mut active_services = 0;
        let mut inactive_services = 0;
        let mut failed_services = 0;
        
        for i in 0..block.port_count {
            let port = block.base_port + i;
            let service_name = format!("{}-service-{}", block_name, i);
            
            match self.service_status.get(&service_name) {
                Some(ServiceStatus::Active { .. }) => active_services += 1,
                Some(ServiceStatus::Inactive { .. }) => inactive_services += 1,
                Some(ServiceStatus::Failed { .. }) => failed_services += 1,
                Some(ServiceStatus::Fallback { .. }) => active_services += 1,
                None => inactive_services += 1,
            }
        }
        
        Ok(BlockStatus {
            block_name: block_name.to_string(),
            total_ports: block.port_count,
            active_services,
            inactive_services,
            failed_services,
            sister_port: block.sister_offset.map(|offset| block.base_port + offset),
        })
    }
}

/// Block status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatus {
    pub block_name: String,
    pub total_ports: u16,
    pub active_services: u16,
    pub inactive_services: u16,
    pub failed_services: u16,
    pub sister_port: Option<u16>,
}

impl Default for PortManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_port_manager_creation() {
        let pm = PortManager::new();
        assert_eq!(pm.blocks.len(), 4);
        assert!(pm.blocks.contains_key("BLOCK-A"));
        assert!(pm.blocks.contains_key("BLOCK-B"));
    }
    
    #[test]
    fn test_get_service_port() {
        let pm = PortManager::new();
        let port = pm.get_service_port("BLOCK-A", 0).unwrap();
        assert_eq!(port, 17152);
        
        let port = pm.get_service_port("BLOCK-B", 5).unwrap();
        assert_eq!(port, 17178);
    }
    
    #[test]
    fn test_sister_port() {
        let pm = PortManager::new();
        let sister = pm.get_sister_port(17152);
        assert_eq!(sister, Some(49920));
    }
}
