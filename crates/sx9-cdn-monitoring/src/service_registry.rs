//! Service Registry for Gateway CDN
//!
//! Manages registration and discovery of CTAS services
//! for the Gateway CDN ecosystem.

use chrono::Utc;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

use crate::gateway_cdn::{register_gateway_service, ECSService, ServiceStatus, ServiceType};

/// Core CTAS services configuration
pub struct ServiceRegistry {
    services: HashMap<String, ECSService>,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register core CTAS services
    pub async fn register_core_services(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        let services = vec![
            ECSService {
                id: Uuid::new_v4(),
                name: "core-foundation".to_string(),
                service_type: ServiceType::CoreFoundation,
                port: 18100,
                health_endpoint: "/health".to_string(),
                status: ServiceStatus::Healthy,
                last_health_check: Utc::now(),
                cyber_ops_enabled: true,
                traffic_analysis: true,
            },
            ECSService {
                id: Uuid::new_v4(),
                name: "interface-foundation".to_string(),
                service_type: ServiceType::InterfaceFoundation,
                port: 18101,
                health_endpoint: "/health".to_string(),
                status: ServiceStatus::Healthy,
                last_health_check: Utc::now(),
                cyber_ops_enabled: true,
                traffic_analysis: true,
            },
            ECSService {
                id: Uuid::new_v4(),
                name: "data-foundation".to_string(),
                service_type: ServiceType::DataFoundation,
                port: 18102,
                health_endpoint: "/health".to_string(),
                status: ServiceStatus::Healthy,
                last_health_check: Utc::now(),
                cyber_ops_enabled: true,
                traffic_analysis: true,
            },
            ECSService {
                id: Uuid::new_v4(),
                name: "shipyard-manager".to_string(),
                service_type: ServiceType::ShipyardManager,
                port: 18111,
                health_endpoint: "/health".to_string(),
                status: ServiceStatus::Healthy,
                last_health_check: Utc::now(),
                cyber_ops_enabled: true,
                traffic_analysis: true,
            },
            ECSService {
                id: Uuid::new_v4(),
                name: "cyber-ops".to_string(),
                service_type: ServiceType::CyberOps,
                port: 18108,
                health_endpoint: "/health".to_string(),
                status: ServiceStatus::CyberOpsActive,
                last_health_check: Utc::now(),
                cyber_ops_enabled: true,
                traffic_analysis: true,
            },
        ];

        let mut registered_count = 0;
        for service in services {
            if let Err(e) = register_gateway_service(service.clone()).await {
                error!("Failed to register service {}: {}", service.name, e);
            } else {
                self.services.insert(service.name.clone(), service);
                registered_count += 1;
            }
        }

        info!("✅ Registered {} core services", registered_count);
        Ok(registered_count)
    }

    /// Get service by name
    pub fn get_service(&self, name: &str) -> Option<&ECSService> {
        self.services.get(name)
    }

    /// Get all services
    pub fn get_all_services(&self) -> Vec<&ECSService> {
        self.services.values().collect()
    }

    /// Get services by type
    pub fn get_services_by_type(&self, service_type: ServiceType) -> Vec<&ECSService> {
        self.services
            .values()
            .filter(|service| service.service_type == service_type)
            .collect()
    }

    /// Get services with cyber ops enabled
    pub fn get_cyber_ops_services(&self) -> Vec<&ECSService> {
        self.services
            .values()
            .filter(|service| service.cyber_ops_enabled)
            .collect()
    }

    /// Get healthy services
    pub fn get_healthy_services(&self) -> Vec<&ECSService> {
        self.services
            .values()
            .filter(|service| {
                service.status == ServiceStatus::Healthy
                    || service.status == ServiceStatus::CyberOpsActive
            })
            .collect()
    }
}
