//! CTAS-7 Real Port Manager Implementation
//!
//! Core port management logic with major port blocks, mirror blocks, and deception.

use crate::types::*;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{error, info, warn};
use uuid::Uuid;

impl PortManager {
    pub fn new() -> Self {
        let mut manager = Self {
            allocations: HashMap::new(),
            reserved_ports: vec![18103, 18108, 18111], // Core CTAS-7 ports
            mirror_blocks: Vec::new(),
            deception_settings: DeceptionSettings {
                stealth_mode: true,
                fake_ports: vec![18150, 18151, 18152],
                decoy_services: vec!["fake-service-1".to_string(), "fake-service-2".to_string()],
                traffic_obfuscation: true,
                port_randomization: true,
            },
            cyber_ops_enabled: true,
        };

        // Initialize mirror blocks
        manager.initialize_mirror_blocks();
        manager
    }

    fn initialize_mirror_blocks(&mut self) {
        // CDN Mirror Block (18140-18159)
        self.mirror_blocks.push(MirrorBlock {
            primary_port: 18140,
            mirror_ports: vec![18141, 18142],
            mirror_type: MirrorType::LoadBalancing,
            active: true,
        });

        // Orbital Services Mirror Block (18120-18139)
        self.mirror_blocks.push(MirrorBlock {
            primary_port: 18120,
            mirror_ports: vec![18121, 18122],
            mirror_type: MirrorType::Failover,
            active: true,
        });

        // Neural/Memory Mesh Mirror Block (18160-18179)
        self.mirror_blocks.push(MirrorBlock {
            primary_port: 18160,
            mirror_ports: vec![18161, 18162],
            mirror_type: MirrorType::LoadBalancing,
            active: true,
        });

        // Deception Mirror Block
        self.mirror_blocks.push(MirrorBlock {
            primary_port: 18150,
            mirror_ports: vec![18151, 18152],
            mirror_type: MirrorType::Deception,
            active: true,
        });
    }

    pub async fn allocate_port(
        &mut self,
        port: u16,
        service_name: &str,
        service_type: ServiceType,
    ) -> Result<PortAllocation, PortManagerError> {
        // Check port range (18100-18199)
        if port < 18100 || port > 18199 {
            return Err(PortManagerError::PortOutOfRange(port));
        }

        // Check if port is reserved
        if self.reserved_ports.contains(&port) {
            return Err(PortManagerError::PortReserved(port));
        }

        // Check if port is already allocated
        if self.allocations.contains_key(&port) {
            return Err(PortManagerError::PortAlreadyAllocated(port));
        }

        // Find mirror ports for this service
        let mirror_ports = self.get_mirror_ports(port);

        // Create allocation
        let allocation = PortAllocation {
            port,
            service_name: service_name.to_string(),
            service_type,
            allocated_at: Utc::now(),
            cyber_ops_enabled: self.cyber_ops_enabled,
            mirror_ports: mirror_ports.clone(),
            deception_active: self.deception_settings.stealth_mode,
            allocation_id: Uuid::new_v4().to_string(),
        };

        // Store allocation
        self.allocations.insert(port, allocation.clone());

        info!(
            "📡 Allocated port {} for service: {} (mirrors: {:?})",
            port, service_name, mirror_ports
        );

        Ok(allocation)
    }

    pub async fn release_port(&mut self, port: u16) -> Result<(), PortManagerError> {
        if let Some(allocation) = self.allocations.remove(&port) {
            info!(
                "🔓 Released port {} from service: {}",
                port, allocation.service_name
            );
            Ok(())
        } else {
            Err(PortManagerError::ServiceNotFound(format!("Port {}", port)))
        }
    }

    pub fn get_mirror_ports(&self, port: u16) -> Vec<u16> {
        for mirror_block in &self.mirror_blocks {
            if mirror_block.primary_port == port {
                return mirror_block.mirror_ports.clone();
            }
        }
        Vec::new()
    }

    pub fn get_all_allocations(&self) -> Vec<&PortAllocation> {
        self.allocations.values().collect()
    }

    pub fn get_port_allocation(&self, port: u16) -> Option<&PortAllocation> {
        self.allocations.get(&port)
    }

    pub fn get_mirror_blocks(&self) -> &Vec<MirrorBlock> {
        &self.mirror_blocks
    }

    pub fn get_deception_settings(&self) -> &DeceptionSettings {
        &self.deception_settings
    }

    /// Allocate port for orbital services (18120-18139 block)
    pub async fn allocate_orbital_port(
        &mut self,
        service_name: &str,
    ) -> Result<PortAllocation, PortManagerError> {
        let orbital_block_start = 18120;
        let orbital_block_end = 18139;

        // Find next available port in orbital block
        for port in orbital_block_start..=orbital_block_end {
            if !self.allocations.contains_key(&port) && !self.reserved_ports.contains(&port) {
                return self
                    .allocate_port(port, service_name, ServiceType::Orbital)
                    .await;
            }
        }

        Err(PortManagerError::NoPortsAvailable(
            "Orbital services block (18120-18139) is full".to_string(),
        ))
    }

    /// Allocate port for CDN services (18140-18159 block)
    pub async fn allocate_cdn_port(
        &mut self,
        service_name: &str,
    ) -> Result<PortAllocation, PortManagerError> {
        let cdn_block_start = 18140;
        let cdn_block_end = 18159;

        // Find next available port in CDN block
        for port in cdn_block_start..=cdn_block_end {
            if !self.allocations.contains_key(&port) && !self.reserved_ports.contains(&port) {
                return self
                    .allocate_port(port, service_name, ServiceType::CDN)
                    .await;
            }
        }

        Err(PortManagerError::NoPortsAvailable(
            "CDN services block (18140-18159) is full".to_string(),
        ))
    }

    /// Allocate port for Neural/Memory Mesh services (18160-18179 block)
    pub async fn allocate_neural_port(
        &mut self,
        service_name: &str,
    ) -> Result<PortAllocation, PortManagerError> {
        let neural_block_start = 18160;
        let neural_block_end = 18179;

        // Find next available port in neural block
        for port in neural_block_start..=neural_block_end {
            if !self.allocations.contains_key(&port) && !self.reserved_ports.contains(&port) {
                return self
                    .allocate_port(port, service_name, ServiceType::XSD)
                    .await;
            }
        }

        Err(PortManagerError::NoPortsAvailable(
            "Neural/Memory Mesh block (18160-18179) is full".to_string(),
        ))
    }

    /// Allocate specific ports for known orbital services
    pub async fn allocate_orbital_services(
        &mut self,
    ) -> Result<Vec<PortAllocation>, PortManagerError> {
        let mut allocations = Vec::new();

        // Core orbital services with preferred ports
        let orbital_services = vec![
            ("ctas7-groundstations-hft", 18120),
            ("ctas7-orbital-mechanics", 18121),
            ("ctas7-enhanced-geolocation", 18122),
            ("ctas7-orbital-ingest", 18123),
            ("ctas7-laserlight-constellation", 18124),
            ("ctas7-mcp-laser-light", 18125),
            ("ctas7-space-world-foundation-bridge", 18126),
        ];

        for (service_name, preferred_port) in orbital_services {
            match self
                .allocate_port(preferred_port, service_name, ServiceType::Orbital)
                .await
            {
                Ok(allocation) => {
                    allocations.push(allocation);
                    info!(
                        "🛰️ Allocated orbital port {} for {}",
                        preferred_port, service_name
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to allocate preferred port {} for {}: {:?}",
                        preferred_port, service_name, e
                    );
                    // Try to allocate any available port in orbital block
                    match self.allocate_orbital_port(service_name).await {
                        Ok(allocation) => {
                            let port = allocation.port;
                            allocations.push(allocation);
                            info!(
                                "🛰️ Allocated fallback orbital port {} for {}",
                                port, service_name
                            );
                        }
                        Err(fallback_err) => {
                            error!(
                                "Failed to allocate any orbital port for {}: {:?}",
                                service_name, fallback_err
                            );
                        }
                    }
                }
            }
        }

        Ok(allocations)
    }
}

/// Port Manager
#[derive(Debug)]
pub struct PortManager {
    pub allocations: HashMap<u16, PortAllocation>,
    pub reserved_ports: Vec<u16>,
    pub mirror_blocks: Vec<MirrorBlock>,
    pub deception_settings: DeceptionSettings,
    pub cyber_ops_enabled: bool,
}
