//! # Cannon Plug API Connector
//!
//! Orchestrates all CTAS 7.0 microservices through the Cannon Plug API

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::{info, warn, error, debug};

/// Connector for the Cannon Plug API orchestration system
#[derive(Debug)]
pub struct CannonPlugConnector {
    client: Client,
    base_url: String,
    registered_services: Arc<RwLock<HashMap<String, ServiceRegistration>>>,
    health_monitor: HealthMonitor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub port: u16,
    pub capabilities: Vec<String>,
    pub health_status: ServiceHealth,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug)]
pub struct HealthMonitor {
    check_interval_secs: u64,
    unhealthy_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroserviceHealth {
    pub total_services: usize,
    pub healthy_services: usize,
    pub degraded_services: usize,
    pub unhealthy_services: usize,
    pub services: HashMap<String, ServiceHealth>,
}

impl CannonPlugConnector {
    /// Create new Cannon Plug connector
    pub async fn new(base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();
        let registered_services = Arc::new(RwLock::new(HashMap::new()));

        let health_monitor = HealthMonitor {
            check_interval_secs: 30,
            unhealthy_threshold: 3,
        };

        let connector = Self {
            client,
            base_url,
            registered_services,
            health_monitor,
        };

        // Register with cannon plug
        connector.register_unified_engine().await?;

        Ok(connector)
    }

    /// Register the unified knowledge engine with Cannon Plug
    async fn register_unified_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        let registration = serde_json::json!({
            "id": "unified-knowledge-engine",
            "name": "CTAS Unified Knowledge Engine",
            "endpoint": "localhost",
            "port": 8080,
            "capabilities": [
                "unified_intelligence",
                "multi_database_integration",
                "threat_analysis",
                "ai_inference",
                "xsd_validation",
                "crate_analysis",
                "knowledge_synthesis",
                "lightspeed_decisions"
            ],
            "health_status": "healthy",
            "last_health_check": chrono::Utc::now().to_rfc3339()
        });

        let response = self.client
            .post(&format!("{}/cannon/plug", self.base_url))
            .json(&registration)
            .send()
            .await?;

        if response.status().is_success() {
            info!("✅ Unified Knowledge Engine registered with Cannon Plug");
        } else {
            warn!("⚠️ Failed to register with Cannon Plug: {}", response.status());
        }

        Ok(())
    }

    /// Discover and connect to all available microservices
    pub async fn discover_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Discovering available microservices...");

        let response = self.client
            .get(&format!("{}/services", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let services: Vec<ServiceRegistration> = response.json().await?;
            let mut registered = self.registered_services.write().await;

            for service in services {
                info!("📡 Discovered service: {} ({}:{})", service.name, service.endpoint, service.port);
                registered.insert(service.id.clone(), service);
            }

            info!("✅ Discovered {} microservices", registered.len());
        }

        Ok(())
    }

    /// Check health of all registered microservices
    pub async fn check_all_health(&self) -> Result<MicroserviceHealth, Box<dyn std::error::Error>> {
        let services = self.registered_services.read().await;
        let mut health_status = HashMap::new();
        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;

        for (service_id, service) in services.iter() {
            let health = self.check_service_health(service).await;

            match health {
                ServiceHealth::Healthy => healthy_count += 1,
                ServiceHealth::Degraded => degraded_count += 1,
                ServiceHealth::Unhealthy => unhealthy_count += 1,
                ServiceHealth::Unknown => unhealthy_count += 1,
            }

            health_status.insert(service_id.clone(), health);
        }

        Ok(MicroserviceHealth {
            total_services: services.len(),
            healthy_services: healthy_count,
            degraded_services: degraded_count,
            unhealthy_services: unhealthy_count,
            services: health_status,
        })
    }

    /// Check health of specific microservice
    async fn check_service_health(&self, service: &ServiceRegistration) -> ServiceHealth {
        let health_url = format!("http://{}:{}/health", service.endpoint, service.port);

        match self.client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                debug!("✅ {} is healthy", service.name);
                ServiceHealth::Healthy
            }
            Ok(response) => {
                warn!("⚠️ {} returned status: {}", service.name, response.status());
                ServiceHealth::Degraded
            }
            Err(e) => {
                error!("❌ {} health check failed: {}", service.name, e);
                ServiceHealth::Unhealthy
            }
        }
    }

    /// Send analysis request to CTAS Analyzer microservice
    pub async fn analyze_crate(&self, crate_path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("🔍 Sending crate analysis request for: {}", crate_path);

        let request = serde_json::json!({
            "crate_path": crate_path,
            "analysis_type": "unified",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let response = self.client
            .post("http://localhost:18109/analyze")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            info!("✅ Crate analysis completed");
            Ok(result)
        } else {
            error!("❌ Crate analysis failed: {}", response.status());
            Err(format!("Analysis failed with status: {}", response.status()).into())
        }
    }

    /// Send request to Statistical Analysis CDN
    pub async fn request_statistical_analysis(&self, data: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("📊 Requesting statistical analysis");

        let response = self.client
            .post("http://localhost:18108/analyze")
            .json(&data)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            info!("✅ Statistical analysis completed");
            Ok(result)
        } else {
            error!("❌ Statistical analysis failed: {}", response.status());
            Err(format!("Statistical analysis failed with status: {}", response.status()).into())
        }
    }

    /// Request hash computation from Hashing Engine
    pub async fn compute_hash(&self, data: &str, algorithm: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "data": data,
            "algorithm": algorithm,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let response = self.client
            .post("http://localhost:18105/compute")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(hash) = result.get("hash").and_then(|h| h.as_str()) {
                Ok(hash.to_string())
            } else {
                Err("Hash not found in response".into())
            }
        } else {
            Err(format!("Hash computation failed with status: {}", response.status()).into())
        }
    }

    /// Get port allocation from Port Manager
    pub async fn allocate_port(&self, service_name: &str) -> Result<u16, Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "service_name": service_name,
            "requested_by": "unified-knowledge-engine"
        });

        let response = self.client
            .post("http://localhost:18103/allocate")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(port) = result.get("port").and_then(|p| p.as_u64()) {
                Ok(port as u16)
            } else {
                Err("Port not found in response".into())
            }
        } else {
            Err(format!("Port allocation failed with status: {}", response.status()).into())
        }
    }

    /// Validate XSD through XSD Environment service
    pub async fn validate_xsd(&self, content: &str, schema: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "content": content,
            "schema": schema,
            "validation_type": "strict"
        });

        let response = self.client
            .post("http://localhost:18102/validate")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(valid) = result.get("valid").and_then(|v| v.as_bool()) {
                Ok(valid)
            } else {
                Ok(false)
            }
        } else {
            Err(format!("XSD validation failed with status: {}", response.status()).into())
        }
    }

    /// Get overall system status
    pub async fn get_system_status(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self.client
            .get(&format!("{}/status", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(format!("Failed to get system status: {}", response.status()).into())
        }
    }

    /// Start health monitoring for all services
    pub async fn start_health_monitoring(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Starting health monitoring for all microservices");

        let connector = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(connector.health_monitor.check_interval_secs)
            );

            loop {
                interval.tick().await;

                if let Err(e) = connector.check_all_health().await {
                    error!("Health monitoring error: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Get health status for dashboard
    pub async fn get_health_status(&self) -> Result<MicroserviceHealth, Box<dyn std::error::Error>> {
        self.check_all_health().await
    }
}

impl Clone for CannonPlugConnector {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            registered_services: self.registered_services.clone(),
            health_monitor: HealthMonitor {
                check_interval_secs: self.health_monitor.check_interval_secs,
                unhealthy_threshold: self.health_monitor.unhealthy_threshold,
            },
        }
    }
}