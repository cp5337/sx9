//! Unified System Integration
//! 
//! Connects XSD to all CTAS-7 systems for unified operation.

use std::collections::HashMap;
use tracing::{info, warn, error};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::foundation_integration::FoundationIntegration;
use crate::intelligence::XSDConsciousness;

/// Unified System Manager
#[derive(Debug)]
pub struct UnifiedSystem {
    pub foundation_integration: FoundationIntegration,
    pub xsd_consciousness: XSDConsciousness,
    pub connected_services: HashMap<String, ConnectedService>,
    pub client: Client,
    pub system_status: SystemStatus,
}

/// Connected Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedService {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub port: u16,
    pub endpoint: String,
    pub xsd_context: String,
    pub intelligence_level: String,
    pub status: ServiceStatus,
    pub last_health_check: chrono::DateTime<Utc>,
    pub xsd_integration: bool,
}

/// Service Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
    Disconnected,
}

/// System Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub total_services: usize,
    pub healthy_services: usize,
    pub xsd_integrated_services: usize,
    pub foundation_available: bool,
    pub integration_mode: String,
    pub last_sync: chrono::DateTime<Utc>,
}

impl UnifiedSystem {
    pub fn new() -> Self {
        Self {
            foundation_integration: FoundationIntegration::new(),
            xsd_consciousness: XSDConsciousness::new(),
            connected_services: HashMap::new(),
            client: Client::new(),
            system_status: SystemStatus {
                total_services: 0,
                healthy_services: 0,
                xsd_integrated_services: 0,
                foundation_available: false,
                integration_mode: "Initializing".to_string(),
                last_sync: Utc::now(),
            },
        }
    }
    
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧠 Initializing Unified System with XSD integration");
        
        // Initialize foundation integration
        let foundation_services = self.foundation_integration.register_foundation_services().await;
        
        // Register foundation services
        for service in foundation_services {
            let connected_service = ConnectedService {
                service_id: service.service_id.clone(),
                service_name: service.service_name,
                service_type: service.service_type,
                port: service.port,
                endpoint: format!("http://localhost:{}", service.port),
                xsd_context: service.xsd_context,
                intelligence_level: service.intelligence_level,
                status: ServiceStatus::Healthy,
                last_health_check: Utc::now(),
                xsd_integration: true,
            };
            
            self.connected_services.insert(service.service_id, connected_service);
        }
        
        // Create standalone services if needed
        let standalone_services = self.foundation_integration.create_standalone_services().await;
        for service in standalone_services {
            let connected_service = ConnectedService {
                service_id: service.service_id.clone(),
                service_name: service.service_name,
                service_type: service.service_type,
                port: service.port,
                endpoint: service.endpoint,
                xsd_context: "Standalone".to_string(),
                intelligence_level: "Basic".to_string(),
                status: ServiceStatus::Unknown,
                last_health_check: Utc::now(),
                xsd_integration: true,
            };
            
            self.connected_services.insert(service.service_id, connected_service);
        }
        
        // Register core CTAS-7 services
        self.register_core_services().await;
        
        // Update system status
        self.update_system_status().await;
        
        // Set integration mode
        self.system_status.integration_mode = if self.foundation_integration.is_foundation_available() {
            "Foundation Integrated".to_string()
        } else {
            "Standalone".to_string()
        };
        
        info!("🧠 Unified System initialized with {} services", self.connected_services.len());
        Ok(())
    }
    
    async fn register_core_services(&mut self) {
        let core_services = vec![
            ("port-manager", "CTAS-7 Real Port Manager", "PortManager", 18103, "Production", "Advanced"),
            ("statistical-cdn", "CTAS-7 Statistical Analysis CDN", "Statistical", 18108, "LLMOps", "Consciousness"),
            ("universal-telemetry", "CTAS-7 Universal Telemetry", "Telemetry", 18101, "DevOps", "Enhanced"),
            ("xsd-environment", "CTAS-7 XSD Environment", "XSD", 18107, "Production", "Consciousness"),
        ];
        
        for (id, name, service_type, port, context, intelligence) in core_services {
            let connected_service = ConnectedService {
                service_id: id.to_string(),
                service_name: name.to_string(),
                service_type: service_type.to_string(),
                port,
                endpoint: format!("http://localhost:{}", port),
                xsd_context: context.to_string(),
                intelligence_level: intelligence.to_string(),
                status: ServiceStatus::Unknown,
                last_health_check: Utc::now(),
                xsd_integration: true,
            };
            
            self.connected_services.insert(id.to_string(), connected_service);
            info!("🧠 Registered core service: {} on port {}", name, port);
        }
    }
    
    pub async fn sync_all_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧠 Syncing all services with XSD");
        
        // Sync with foundation if available
        if self.foundation_integration.is_foundation_available() {
            self.foundation_integration.sync_with_foundation().await?;
        }
        
        // Health check all services
        for (service_id, service) in self.connected_services.iter_mut() {
            let health_status = self.check_service_health(&service.endpoint).await;
            service.status = health_status;
            service.last_health_check = Utc::now();
            
            if service.xsd_integration {
                // Apply XSD intelligence to service
                self.apply_xsd_intelligence(service).await;
            }
        }
        
        self.update_system_status().await;
        self.system_status.last_sync = Utc::now();
        
        info!("🧠 Sync complete - {} healthy services", self.system_status.healthy_services);
        Ok(())
    }
    
    async fn check_service_health(&self, endpoint: &str) -> ServiceStatus {
        let health_url = format!("{}/health", endpoint);
        
        match self.client.get(&health_url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    ServiceStatus::Healthy
                } else {
                    ServiceStatus::Warning
                }
            }
            Err(_) => ServiceStatus::Disconnected,
        }
    }
    
    async fn apply_xsd_intelligence(&self, service: &mut ConnectedService) {
        // Apply XSD consciousness to service based on context and intelligence level
        let context_assessment = self.xsd_consciousness.assess_context(&ctas7_xsd_environment::environment::XSDEnvironmentAnnotation {
            operational_context: match service.xsd_context.as_str() {
                "Production" => ctas7_xsd_environment::environment::OperationalContext::Production,
                "LLMOps" => ctas7_xsd_environment::environment::OperationalContext::LLMOps,
                "DevOps" => ctas7_xsd_environment::environment::OperationalContext::DevOps,
                "DeceptionOps" => ctas7_xsd_environment::environment::OperationalContext::DeceptionOps,
                _ => ctas7_xsd_environment::environment::OperationalContext::Production,
            },
            intelligence_level: match service.intelligence_level.as_str() {
                "Consciousness" => ctas7_xsd_environment::environment::IntelligenceLevel::Consciousness,
                "Advanced" => ctas7_xsd_environment::environment::IntelligenceLevel::Advanced,
                "Enhanced" => ctas7_xsd_environment::environment::IntelligenceLevel::Enhanced,
                _ => ctas7_xsd_environment::environment::IntelligenceLevel::Basic,
            },
            security_posture: ctas7_xsd_environment::environment::SecurityPosture::High,
            automation_capability: ctas7_xsd_environment::environment::AutomationCapability::Full,
        });
        
        // Log intelligence recommendations
        if !context_assessment.is_empty() {
            info!("🧠 XSD intelligence for {}: {:?}", service.service_name, context_assessment);
        }
    }
    
    async fn update_system_status(&mut self) {
        self.system_status.total_services = self.connected_services.len();
        self.system_status.healthy_services = self.connected_services
            .values()
            .filter(|s| matches!(s.status, ServiceStatus::Healthy))
            .count();
        self.system_status.xsd_integrated_services = self.connected_services
            .values()
            .filter(|s| s.xsd_integration)
            .count();
        self.system_status.foundation_available = self.foundation_integration.is_foundation_available();
    }
    
    pub fn get_system_status(&self) -> &SystemStatus {
        &self.system_status
    }
    
    pub fn get_connected_services(&self) -> Vec<&ConnectedService> {
        self.connected_services.values().collect()
    }
    
    pub fn get_service(&self, service_id: &str) -> Option<&ConnectedService> {
        self.connected_services.get(service_id)
    }
    
    pub async fn analyze_request_with_xsd(&self, service_id: &str, request_data: &str) -> Result<XSDAnalysisResult, Box<dyn std::error::Error>> {
        if let Some(service) = self.connected_services.get(service_id) {
            // Use XSD consciousness to analyze request
            let threat_score = self.xsd_consciousness.analyze_threat_level(request_data);
            let recommendations = self.xsd_consciousness.assess_context(&ctas7_xsd_environment::environment::XSDEnvironmentAnnotation {
                operational_context: match service.xsd_context.as_str() {
                    "Production" => ctas7_xsd_environment::environment::OperationalContext::Production,
                    "LLMOps" => ctas7_xsd_environment::environment::OperationalContext::LLMOps,
                    "DevOps" => ctas7_xsd_environment::environment::OperationalContext::DevOps,
                    "DeceptionOps" => ctas7_xsd_environment::environment::OperationalContext::DeceptionOps,
                    _ => ctas7_xsd_environment::environment::OperationalContext::Production,
                },
                intelligence_level: match service.intelligence_level.as_str() {
                    "Consciousness" => ctas7_xsd_environment::environment::IntelligenceLevel::Consciousness,
                    "Advanced" => ctas7_xsd_environment::environment::IntelligenceLevel::Advanced,
                    "Enhanced" => ctas7_xsd_environment::environment::IntelligenceLevel::Enhanced,
                    _ => ctas7_xsd_environment::environment::IntelligenceLevel::Basic,
                },
                security_posture: ctas7_xsd_environment::environment::SecurityPosture::High,
                automation_capability: ctas7_xsd_environment::environment::AutomationCapability::Full,
            });
            
            Ok(XSDAnalysisResult {
                service_id: service_id.to_string(),
                threat_score,
                recommendations,
                xsd_context: service.xsd_context.clone(),
                intelligence_level: service.intelligence_level.clone(),
                analyzed_at: Utc::now(),
            })
        } else {
            Err(format!("Service {} not found", service_id).into())
        }
    }
}

/// XSD Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSDAnalysisResult {
    pub service_id: String,
    pub threat_score: f64,
    pub recommendations: Vec<String>,
    pub xsd_context: String,
    pub intelligence_level: String,
    pub analyzed_at: chrono::DateTime<Utc>,
}
