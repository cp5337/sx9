//! CTAS-7 Auto-Discovery System
//! 
//! Automatic service discovery and health monitoring.

use std::collections::HashMap;
use tracing::{info, warn, error};
use chrono::Utc;
use reqwest::Client;
use tokio::time::{sleep, Duration};

/// Auto-Discovery System
#[derive(Debug)]
pub struct AutoDiscovery {
    pub discovered_services: HashMap<String, DiscoveredService>,
    pub discovery_interval: u64,
    pub health_check_interval: u64,
    pub client: Client,
}

/// Discovered Service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscoveredService {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub endpoint: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub last_discovered: chrono::DateTime<Utc>,
    pub last_health_check: chrono::DateTime<Utc>,
    pub health_status: HealthStatus,
    pub metadata: HashMap<String, String>,
}

/// Service Status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServiceStatus {
    Active,
    Inactive,
    Unknown,
}

/// Health Status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

impl AutoDiscovery {
    pub fn new() -> Self {
        Self {
            discovered_services: HashMap::new(),
            discovery_interval: 30, // 30 seconds
            health_check_interval: 10, // 10 seconds
            client: Client::new(),
        }
    }
    
    pub async fn start_discovery(&mut self) {
        info!("🔍 Starting auto-discovery for CTAS-7 services");
        
        // Discover core CTAS-7 services
        self.discover_core_services().await;
        
        // Start continuous discovery
        let mut discovery_interval = tokio::time::interval(Duration::from_secs(self.discovery_interval));
        let mut health_check_interval = tokio::time::interval(Duration::from_secs(self.health_check_interval));
        
        loop {
            tokio::select! {
                _ = discovery_interval.tick() => {
                    self.discover_services().await;
                }
                _ = health_check_interval.tick() => {
                    self.health_check_services().await;
                }
            }
        }
    }
    
    async fn discover_core_services(&mut self) {
        let core_services = vec![
            ("smart-cdn", "CTAS-7 Smart CDN Gateway", "CDN", "http://localhost:18100", 18100),
            ("port-manager", "CTAS-7 Real Port Manager", "PortManager", "http://localhost:18103", 18103),
            ("progress-system", "CTAS-7 Progress System", "Progress", "http://localhost:18105", 18105),
            ("xsd-environment", "CTAS-7 XSD Environment", "XSD", "http://localhost:18107", 18107),
            ("statistical-cdn", "CTAS-7 Statistical Analysis CDN", "Statistical", "http://localhost:18108", 18108),
        ];
        
        for (id, name, service_type, endpoint, port) in core_services {
            let service = DiscoveredService {
                service_id: id.to_string(),
                service_name: name.to_string(),
                service_type: service_type.to_string(),
                endpoint: endpoint.to_string(),
                port,
                status: ServiceStatus::Active,
                last_discovered: Utc::now(),
                last_health_check: Utc::now(),
                health_status: HealthStatus::Unknown,
                metadata: HashMap::new(),
            };
            
            self.discovered_services.insert(id.to_string(), service);
            info!("🔍 Discovered core service: {} on port {}", name, port);
        }
    }
    
    async fn discover_services(&mut self) {
        info!("🔍 Running service discovery cycle");
        
        // Check for new services on common ports
        let common_ports = vec![18100, 18101, 18103, 18105, 18107, 18108, 18109, 18110];
        
        for port in common_ports {
            let endpoint = format!("http://localhost:{}/health", port);
            
            // Check if service is already discovered
            let already_discovered = self.discovered_services
                .values()
                .any(|service| service.port == port);
            
            if !already_discovered {
                if let Ok(response) = self.client.get(&endpoint).timeout(Duration::from_secs(5)).send().await {
                    if response.status().is_success() {
                        let service = DiscoveredService {
                            service_id: format!("service-{}", port),
                            service_name: format!("Unknown Service on Port {}", port),
                            service_type: "Unknown".to_string(),
                            endpoint: endpoint.clone(),
                            port,
                            status: ServiceStatus::Active,
                            last_discovered: Utc::now(),
                            last_health_check: Utc::now(),
                            health_status: HealthStatus::Healthy,
                            metadata: HashMap::new(),
                        };
                        
                        self.discovered_services.insert(service.service_id.clone(), service);
                        info!("🔍 Discovered new service on port {}", port);
                    }
                }
            }
        }
    }
    
    async fn health_check_services(&mut self) {
        let endpoints: Vec<(String, String)> = self.discovered_services
            .iter()
            .map(|(id, service)| (id.clone(), service.endpoint.clone()))
            .collect();
            
        for (service_id, endpoint) in endpoints {
            let health_status = self.check_service_health(&endpoint).await;
            
            if let Some(service) = self.discovered_services.get_mut(&service_id) {
                service.health_status = health_status.clone();
                service.last_health_check = Utc::now();
                
                match health_status {
                    HealthStatus::Healthy => {
                        service.status = ServiceStatus::Active;
                    }
                    HealthStatus::Unhealthy => {
                        service.status = ServiceStatus::Inactive;
                        warn!("⚠️ Service {} is unhealthy", service_id);
                    }
                    HealthStatus::Unknown => {
                        service.status = ServiceStatus::Unknown;
                    }
                }
            }
        }
    }
    
    async fn check_service_health(&self, endpoint: &str) -> HealthStatus {
        match self.client.get(endpoint).timeout(Duration::from_secs(5)).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                }
            }
            Err(_) => HealthStatus::Unhealthy,
        }
    }
    
    pub fn get_discovered_services(&self) -> Vec<&DiscoveredService> {
        self.discovered_services.values().collect()
    }
    
    pub fn get_service(&self, service_id: &str) -> Option<&DiscoveredService> {
        self.discovered_services.get(service_id)
    }
    
    pub fn get_healthy_services(&self) -> Vec<&DiscoveredService> {
        self.discovered_services
            .values()
            .filter(|service| service.health_status == HealthStatus::Healthy)
            .collect()
    }
}
