use axum::extract::State;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub port: u16,
    pub health_status: String,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    pub capabilities: Vec<String>,
}

#[derive(Debug)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    client: Client,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            client: Client::new(),
        }
    }

    pub async fn register_service(&self, service: ServiceInfo) {
        let service_name = service.name.clone();
        let mut services = self.services.write().await;
        services.insert(service.id.clone(), service);
        info!("📝 Service registered: {}", service_name);
    }

    pub async fn get_all_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    pub async fn get_service(&self, service_id: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(service_id).cloned()
    }

    pub async fn route_to_service(
        &self,
        service_name: &str,
        path: &str,
        payload: Value,
    ) -> Result<Value, String> {
        let services = self.services.read().await;
        
        // Find service by name
        let service = services.values()
            .find(|s| s.name == service_name)
            .ok_or_else(|| format!("Service '{}' not found", service_name))?;

        let url = format!("http://{}:{}{}", service.endpoint, service.port, path);
        
        info!("🔄 Routing to {}: {}", service_name, url);

        match self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    let json_response: Value = response.json().await
                        .map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(json_response)
                } else {
                    Err(format!("Service returned error: {}", response.status()))
                }
            }
            Err(e) => {
                warn!("❌ Failed to route to service {}: {}", service_name, e);
                Err(format!("Failed to connect to service: {}", e))
            }
        }
    }

    pub async fn health_check_service(&self, service_id: &str) -> bool {
        let services = self.services.read().await;
        if let Some(service) = services.get(service_id) {
            let url = format!("http://{}:{}/health", service.endpoint, service.port);
            
            match self.client.get(&url).send().await {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub async fn initialize_core_services(&self) {
        info!("🚀 Initializing core CTAS-7.0 services...");

        let core_services = vec![
            ServiceInfo {
                id: "xsd-environment".to_string(),
                name: "xsd".to_string(),
                endpoint: "localhost".to_string(),
                port: 18102,
                health_status: "unknown".to_string(),
                last_health_check: chrono::Utc::now(),
                capabilities: vec!["schema_validation".to_string(), "inference".to_string()],
            },
            ServiceInfo {
                id: "port-manager".to_string(),
                name: "ports".to_string(),
                endpoint: "localhost".to_string(),
                port: 18103,
                health_status: "unknown".to_string(),
                last_health_check: chrono::Utc::now(),
                capabilities: vec!["port_allocation".to_string(), "deception".to_string()],
            },
            ServiceInfo {
                id: "universal-telemetry".to_string(),
                name: "telemetry".to_string(),
                endpoint: "localhost".to_string(),
                port: 18101,
                health_status: "unknown".to_string(),
                last_health_check: chrono::Utc::now(),
                capabilities: vec!["metrics".to_string(), "monitoring".to_string()],
            },
            ServiceInfo {
                id: "statistical-analysis".to_string(),
                name: "analysis".to_string(),
                endpoint: "localhost".to_string(),
                port: 18108,
                health_status: "unknown".to_string(),
                last_health_check: chrono::Utc::now(),
                capabilities: vec!["ml_analysis".to_string(), "statistics".to_string()],
            },
        ];

        for service in core_services {
            self.register_service(service).await;
        }

        info!("✅ Core services initialized");
    }
}
