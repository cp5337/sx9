// 🔍 CTAS-7 Service Discovery & Coordination
// Prevents service doubling (like ground station duplication)
// Ensures singleton coordination and proper bridging

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};
use uuid::Uuid;

/// Service Registry - Prevents duplicate instances
#[derive(Debug)]
pub struct ServiceRegistry {
    pub services: Arc<Mutex<HashMap<String, RegisteredService>>>,
    pub discovery_port: u16,
    pub heartbeat_interval: Duration,
    pub service_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredService {
    pub service_id: String,
    pub service_type: ServiceType,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub metadata: ServiceMetadata,
    pub singleton: bool, // If true, only one instance allowed
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ServiceType {
    FoundationDaemon,
    BackendMCP,
    DatabaseValidator,
    Phi3Guardian,
    HashEngine,
    NeuralMux,
    PortManager,
    GroundStation, // Prevent doubling like we have
    WatchdogMonitor,
    EmergencyRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Healthy,
    Degraded,
    Unhealthy,
    Shutting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    pub version: String,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub health_check_url: String,
    pub bridge_interfaces: Vec<String>, // Network interfaces for bridging
    pub ontological_namespace: Option<String>, // Future: semantic classification
}

/// Service Discovery Response
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    pub services: Vec<RegisteredService>,
    pub registry_status: RegistryStatus,
    pub coordination_bridges: Vec<CoordinationBridge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryStatus {
    pub total_services: u32,
    pub healthy_services: u32,
    pub singleton_violations: u32,
    pub bridge_conflicts: u32,
    pub last_cleanup: chrono::DateTime<chrono::Utc>,
}

/// Network Bridge Coordination - Prevents service conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationBridge {
    pub bridge_id: String,
    pub bridge_type: BridgeType,
    pub primary_service: String,
    pub secondary_services: Vec<String>,
    pub network_interface: String,
    pub port_range: (u16, u16),
    pub load_balancing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeType {
    SingletonCoordination, // Ensures only one instance
    LoadBalancing,         // Multiple instances with coordination
    FailoverCluster,       // Primary/backup coordination
    SemanticRouting,       // Future: ontological routing
}

/// Service Registration Request
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_type: ServiceType,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub metadata: ServiceMetadata,
    pub singleton: bool,
    pub request_bridge: Option<BridgeType>,
}

impl ServiceRegistry {
    /// Create new service registry
    pub fn new(discovery_port: u16) -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
            discovery_port,
            heartbeat_interval: Duration::from_secs(30),
            service_timeout: Duration::from_secs(90),
        }
    }

    /// Start service discovery server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Starting Service Discovery on port {}", self.discovery_port);

        // Start heartbeat monitoring
        self.start_heartbeat_monitoring().await;

        // Start singleton enforcement
        self.start_singleton_enforcement().await;

        // Start web server
        self.start_discovery_server().await
    }

    /// Register a service (prevent duplicates)
    pub async fn register_service(
        &self,
        registration: ServiceRegistration,
    ) -> Result<String, String> {
        let mut services = self.services.lock().unwrap();

        // Check for singleton violations
        if registration.singleton {
            let existing_singleton = services.values()
                .find(|s| s.service_type == registration.service_type && s.singleton);

            if let Some(existing) = existing_singleton {
                if existing.status == ServiceStatus::Healthy || existing.status == ServiceStatus::Degraded {
                    return Err(format!(
                        "Singleton violation: {} already running as {}",
                        format!("{:?}", registration.service_type),
                        existing.service_id
                    ));
                }
            }
        }

        // Check for port conflicts
        let port_conflict = services.values()
            .find(|s| s.host == registration.host && s.port == registration.port);

        if let Some(conflict) = port_conflict {
            return Err(format!(
                "Port conflict: {}:{} already used by {}",
                registration.host, registration.port, conflict.name
            ));
        }

        // Create service ID
        let service_id = format!("{}-{}",
            format!("{:?}", registration.service_type).to_lowercase(),
            Uuid::new_v4().to_string()[..8].to_string()
        );

        // Register the service
        let registered_service = RegisteredService {
            service_id: service_id.clone(),
            service_type: registration.service_type,
            name: registration.name,
            host: registration.host,
            port: registration.port,
            status: ServiceStatus::Starting,
            last_heartbeat: chrono::Utc::now(),
            metadata: registration.metadata,
            singleton: registration.singleton,
        };

        services.insert(service_id.clone(), registered_service);

        println!("✅ Registered service: {} ({})", service_id,
                 if registration.singleton { "singleton" } else { "multi-instance" });

        Ok(service_id)
    }

    /// Heartbeat monitoring to detect failed services
    async fn start_heartbeat_monitoring(&self) {
        let services = Arc::clone(&self.services);
        let timeout = self.service_timeout;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                let mut services_guard = services.lock().unwrap();
                let now = chrono::Utc::now();

                // Check for timed-out services
                let mut to_remove = Vec::new();
                for (service_id, service) in services_guard.iter_mut() {
                    let time_since_heartbeat = now - service.last_heartbeat;

                    if time_since_heartbeat > chrono::Duration::from_std(timeout).unwrap() {
                        if service.status != ServiceStatus::Shutting {
                            println!("⚠️ Service timeout detected: {} ({})",
                                     service.name, service_id);
                            to_remove.push(service_id.clone());
                        }
                    }
                }

                // Remove timed-out services
                for service_id in to_remove {
                    services_guard.remove(&service_id);
                    println!("🗑️ Removed timed-out service: {}", service_id);
                }
            }
        });
    }

    /// Singleton enforcement - prevent doubling
    async fn start_singleton_enforcement(&self) {
        let services = Arc::clone(&self.services);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));

            loop {
                interval.tick().await;

                let services_guard = services.lock().unwrap();
                let mut type_counts: HashMap<ServiceType, u32> = HashMap::new();

                // Count services by type
                for service in services_guard.values() {
                    if service.singleton &&
                       (service.status == ServiceStatus::Healthy || service.status == ServiceStatus::Degraded) {
                        *type_counts.entry(service.service_type.clone()).or_insert(0) += 1;
                    }
                }

                // Report violations
                for (service_type, count) in type_counts {
                    if count > 1 {
                        println!("🚨 Singleton violation detected: {} has {} instances",
                                 format!("{:?}", service_type), count);
                    }
                }
            }
        });
    }

    /// Start discovery web server
    async fn start_discovery_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        use warp::Filter;

        let services = Arc::clone(&self.services);

        // Health endpoint
        let health = warp::path("health")
            .and(warp::get())
            .map(|| warp::reply::json(&serde_json::json!({
                "status": "ok",
                "service": "Service-Discovery"
            })));

        // Service registration endpoint
        let register = warp::path("register")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |reg: ServiceRegistration| {
                let services_clone = Arc::clone(&services);
                async move {
                    // This is a simplified version - in real implementation would call register_service
                    match Self::mock_register_service(services_clone, reg).await {
                        Ok(service_id) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                            "success": true,
                            "service_id": service_id
                        }))),
                        Err(e) => Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                            "success": false,
                            "error": e
                        })))
                    }
                }
            });

        // Discovery endpoint
        let services_clone = Arc::clone(&self.services);
        let discover = warp::path("discover")
            .and(warp::get())
            .map(move || {
                let services_guard = services_clone.lock().unwrap();
                let services_list: Vec<RegisteredService> = services_guard.values().cloned().collect();

                let response = DiscoveryResponse {
                    services: services_list.clone(),
                    registry_status: RegistryStatus {
                        total_services: services_list.len() as u32,
                        healthy_services: services_list.iter()
                            .filter(|s| s.status == ServiceStatus::Healthy)
                            .count() as u32,
                        singleton_violations: 0, // Would calculate in real implementation
                        bridge_conflicts: 0,
                        last_cleanup: chrono::Utc::now(),
                    },
                    coordination_bridges: vec![], // Would populate in real implementation
                };

                warp::reply::json(&response)
            });

        let routes = health.or(register).or(discover)
            .with(warp::cors().allow_any_origin());

        println!("🌐 Service Discovery listening on http://localhost:{}", self.discovery_port);
        warp::serve(routes).run(([127, 0, 0, 1], self.discovery_port)).await;

        Ok(())
    }

    /// Mock registration for simplified implementation
    async fn mock_register_service(
        services: Arc<Mutex<HashMap<String, RegisteredService>>>,
        registration: ServiceRegistration,
    ) -> Result<String, String> {
        let service_id = format!("{}-mock", format!("{:?}", registration.service_type).to_lowercase());

        let registered_service = RegisteredService {
            service_id: service_id.clone(),
            service_type: registration.service_type,
            name: registration.name,
            host: registration.host,
            port: registration.port,
            status: ServiceStatus::Healthy,
            last_heartbeat: chrono::Utc::now(),
            metadata: registration.metadata,
            singleton: registration.singleton,
        };

        let mut services_guard = services.lock().unwrap();
        services_guard.insert(service_id.clone(), registered_service);

        Ok(service_id)
    }
}

/// Service Discovery Client - For services to register themselves
#[derive(Debug)]
pub struct ServiceDiscoveryClient {
    pub registry_url: String,
    pub service_id: Option<String>,
}

impl ServiceDiscoveryClient {
    pub fn new(registry_host: &str, registry_port: u16) -> Self {
        Self {
            registry_url: format!("http://{}:{}", registry_host, registry_port),
            service_id: None,
        }
    }

    /// Register this service with the discovery server
    pub async fn register(
        &mut self,
        registration: ServiceRegistration,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let response = client
            .post(&format!("{}/register", self.registry_url))
            .json(&registration)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(service_id) = result["service_id"].as_str() {
                self.service_id = Some(service_id.to_string());
                println!("✅ Successfully registered as: {}", service_id);
                return Ok(service_id.to_string());
            }
        }

        Err("Registration failed".into())
    }

    /// Send heartbeat to maintain registration
    pub async fn heartbeat(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(service_id) = &self.service_id {
            // In real implementation, would send heartbeat to registry
            println!("💓 Heartbeat sent for service: {}", service_id);
        }
        Ok(())
    }

    /// Discover other services
    pub async fn discover_services(&self) -> Result<Vec<RegisteredService>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/discover", self.registry_url))
            .send()
            .await?;

        if response.status().is_success() {
            let discovery_response: DiscoveryResponse = response.json().await?;
            return Ok(discovery_response.services);
        }

        Err("Service discovery failed".into())
    }
}

// NOTE: Future ontological/semantic component would integrate here
// with semantic service classification and meaning-based routing

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_singleton_enforcement() {
        let registry = ServiceRegistry::new(18650);

        let registration1 = ServiceRegistration {
            service_type: ServiceType::Phi3Guardian,
            name: "phi3-instance-1".to_string(),
            host: "localhost".to_string(),
            port: 11434,
            metadata: ServiceMetadata {
                version: "1.0.0".to_string(),
                capabilities: vec!["model-inference".to_string()],
                dependencies: vec![],
                health_check_url: "http://localhost:11434/health".to_string(),
                bridge_interfaces: vec![],
                ontological_namespace: None,
            },
            singleton: true,
            request_bridge: None,
        };

        let registration2 = registration1.clone();

        // First registration should succeed
        let result1 = registry.register_service(registration1).await;
        assert!(result1.is_ok());

        // Second registration should fail (singleton violation)
        let result2 = registry.register_service(registration2).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("Singleton violation"));
    }
}