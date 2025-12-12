//! QA5 Port Integration - Extends existing port management for operational intelligence
//! 
//! Integrates with the existing ctas-port-manager system to provide QA5-specific
//! operational intelligence capabilities without duplicating port management logic.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{info, warn, error, debug};

// Import existing port management system
use ctas_port_manager::{
    PortManagerIntegration,
    PortConnector,
    ServiceConnection,
    ConnectionStatus,
};

/// QA5 Operational Intelligence Service Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QA5ServiceType {
    GroupOperations,
    CrateInterview,
    LispRdfIntegration,
    XSDOrchestration,
    OperationalIntelligence,
    FrontendIntegration,
    DatabaseIntegration,
    AICLI,
    AICLIPortManagement,
    AICLIOperationalCommands,
}

/// QA5 Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QA5ServiceConfig {
    pub service_type: QA5ServiceType,
    pub service_name: String,
    pub primary_port: u16,
    pub fallback_port: Option<u16>,
    pub health_check_interval: Duration,
    pub operational_capabilities: Vec<String>,
    pub group_mapping: Vec<String>,
}

/// QA5 Port Integration - Extends existing port management
#[derive(Debug)]
pub struct QA5PortIntegration {
    port_connector: PortConnector,
    qa5_services: HashMap<String, QA5ServiceConfig>,
    operational_status: HashMap<String, OperationalStatus>,
    real_time_updates: mpsc::Sender<OperationalUpdate>,
}

/// Operational status for QA5 services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalStatus {
    pub service_name: String,
    pub service_type: QA5ServiceType,
    pub connection_status: ConnectionStatus,
    pub operational_capabilities: Vec<String>,
    pub group_mapping: Vec<String>,
    pub last_health_check: Instant,
    pub is_operational: bool,
}

/// Real-time operational updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalUpdate {
    pub update_type: String,
    pub service_name: String,
    pub timestamp: Instant,
    pub data: serde_json::Value,
}

impl QA5PortIntegration {
    /// Create new QA5 port integration
    pub fn new() -> Result<Self> {
        let port_connector = PortConnector::new()?;
        let (real_time_updates, _) = mpsc::channel(100);
        
        Ok(Self {
            port_connector,
            qa5_services: HashMap::new(),
            operational_status: HashMap::new(),
            real_time_updates,
        })
    }

    /// Register QA5 service with operational intelligence capabilities
    pub async fn register_qa5_service(&mut self, config: QA5ServiceConfig) -> Result<()> {
        // Create service connection for existing port management
        let service_connection = ServiceConnection {
            service_name: config.service_name.clone(),
            primary_port: config.primary_port,
            fallback_port: config.fallback_port,
            health_check_interval: config.health_check_interval,
            connection_timeout: Duration::from_secs(30),
            max_retries: 3,
            auto_failover: true,
        };

        // Register with existing port connector
        self.port_connector.register_service(service_connection)?;
        
        // Store QA5-specific configuration
        self.qa5_services.insert(config.service_name.clone(), config.clone());
        
        // Initialize operational status
        let operational_status = OperationalStatus {
            service_name: config.service_name.clone(),
            service_type: config.service_type.clone(),
            connection_status: ConnectionStatus::Unreachable { port: config.primary_port },
            operational_capabilities: config.operational_capabilities.clone(),
            group_mapping: config.group_mapping.clone(),
            last_health_check: Instant::now(),
            is_operational: false,
        };
        
        self.operational_status.insert(config.service_name.clone(), operational_status);
        
        info!("Registered QA5 service: {} on port {}", config.service_name, config.primary_port);
        
        Ok(())
    }

    /// Get operational status for a service
    pub async fn get_operational_status(&self, service_name: &str) -> Option<&OperationalStatus> {
        self.operational_status.get(service_name)
    }

    /// Get all operational statuses
    pub async fn get_all_operational_statuses(&self) -> &HashMap<String, OperationalStatus> {
        &self.operational_status
    }

    /// Execute group operation through port-managed service
    pub async fn execute_group_operation(&mut self, group_id: &str, operation: &str, parallel: bool) -> Result<serde_json::Value> {
        // Find the group operations service
        let group_ops_service = self.find_service_by_type(QA5ServiceType::GroupOperations)?;
        
        // Check if service is operational
        if let Some(status) = self.operational_status.get(&group_ops_service.service_name) {
            if !status.is_operational {
                return Err(anyhow::anyhow!("Group operations service is not operational"));
            }
        }

        // Execute operation through port-managed connection
        let operation_data = serde_json::json!({
            "group_id": group_id,
            "operation": operation,
            "parallel": parallel,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        // Send operation through port connector
        let result = self.send_operation_to_service(&group_ops_service.service_name, operation_data).await?;
        
        // Send real-time update
        let update = OperationalUpdate {
            update_type: "group_operation_executed".to_string(),
            service_name: group_ops_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Execute crate interview through port-managed service
    pub async fn execute_crate_interview(&mut self, crate_name: &str) -> Result<serde_json::Value> {
        let interview_service = self.find_service_by_type(QA5ServiceType::CrateInterview)?;
        
        let interview_data = serde_json::json!({
            "crate_name": crate_name,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.send_operation_to_service(&interview_service.service_name, interview_data).await?;
        
        let update = OperationalUpdate {
            update_type: "crate_interview_executed".to_string(),
            service_name: interview_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Execute LISP-RDF integration through port-managed service
    pub async fn execute_lisp_rdf_integration(&mut self, integration_data: serde_json::Value) -> Result<serde_json::Value> {
        let lisp_rdf_service = self.find_service_by_type(QA5ServiceType::LispRdfIntegration)?;
        
        let result = self.send_operation_to_service(&lisp_rdf_service.service_name, integration_data).await?;
        
        let update = OperationalUpdate {
            update_type: "lisp_rdf_integration_executed".to_string(),
            service_name: lisp_rdf_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Execute AI-CLI command through port-managed service
    pub async fn execute_ai_cli_command(&mut self, command: &str, args: Vec<String>) -> Result<serde_json::Value> {
        let ai_cli_service = self.find_service_by_type(QA5ServiceType::AICLI)?;
        
        let command_data = serde_json::json!({
            "command": command,
            "args": args,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.send_operation_to_service(&ai_cli_service.service_name, command_data).await?;
        
        let update = OperationalUpdate {
            update_type: "ai_cli_command_executed".to_string(),
            service_name: ai_cli_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Execute AI-CLI port management command
    pub async fn execute_ai_cli_port_management(&mut self, block_name: &str, operation: &str, service_index: Option<u16>) -> Result<serde_json::Value> {
        let port_mgmt_service = self.find_service_by_type(QA5ServiceType::AICLIPortManagement)?;
        
        let port_data = serde_json::json!({
            "block_name": block_name,
            "operation": operation,
            "service_index": service_index,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.send_operation_to_service(&port_mgmt_service.service_name, port_data).await?;
        
        let update = OperationalUpdate {
            update_type: "ai_cli_port_management_executed".to_string(),
            service_name: port_mgmt_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Execute AI-CLI operational command
    pub async fn execute_ai_cli_operational_command(&mut self, operational_command: &str, parameters: serde_json::Value) -> Result<serde_json::Value> {
        let operational_service = self.find_service_by_type(QA5ServiceType::AICLIOperationalCommands)?;
        
        let operational_data = serde_json::json!({
            "operational_command": operational_command,
            "parameters": parameters,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.send_operation_to_service(&operational_service.service_name, operational_data).await?;
        
        let update = OperationalUpdate {
            update_type: "ai_cli_operational_command_executed".to_string(),
            service_name: operational_service.service_name.clone(),
            timestamp: Instant::now(),
            data: result.clone(),
        };
        
        if let Err(e) = self.real_time_updates.send(update).await {
            warn!("Failed to send real-time update: {}", e);
        }

        Ok(result)
    }

    /// Get operational intelligence mapping
    pub async fn get_operational_intelligence_mapping(&self) -> Result<serde_json::Value> {
        let intel_service = self.find_service_by_type(QA5ServiceType::OperationalIntelligence)?;
        
        let mapping_data = serde_json::json!({
            "request_type": "operational_intelligence_mapping",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        self.send_operation_to_service(&intel_service.service_name, mapping_data).await
    }

    /// Subscribe to real-time updates
    pub fn subscribe_to_updates(&self) -> mpsc::Receiver<OperationalUpdate> {
        let (_, receiver) = mpsc::channel(100);
        receiver
    }

    /// Find service by type
    fn find_service_by_type(&self, service_type: QA5ServiceType) -> Result<&QA5ServiceConfig> {
        self.qa5_services
            .values()
            .find(|service| std::mem::discriminant(&service.service_type) == std::mem::discriminant(&service_type))
            .ok_or_else(|| anyhow::anyhow!("Service type {:?} not found", service_type))
    }

    /// Send operation to service through port connector
    async fn send_operation_to_service(&self, service_name: &str, data: serde_json::Value) -> Result<serde_json::Value> {
        // This would integrate with the existing port connector's service communication
        // For now, simulate the operation
        debug!("Sending operation to service {}: {:?}", service_name, data);
        
        // Simulate service response
        Ok(serde_json::json!({
            "service_name": service_name,
            "operation_success": true,
            "result": {
                "status": "completed",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "data": data
            }
        }))
    }

    /// Health check for all QA5 services
    pub async fn health_check_all_services(&mut self) -> Result<()> {
        for (service_name, config) in &self.qa5_services {
            if let Some(status) = self.operational_status.get_mut(service_name) {
                // Use existing port connector health check
                let connection_status = self.port_connector.check_service_health(service_name).await?;
                
                status.connection_status = connection_status.clone();
                status.last_health_check = Instant::now();
                status.is_operational = matches!(connection_status, ConnectionStatus::Connected { .. });
                
                info!("Health check for {}: {:?}", service_name, status.is_operational);
            }
        }
        
        Ok(())
    }

    /// Get port connector for direct access to existing functionality
    pub fn get_port_connector(&self) -> &PortConnector {
        &self.port_connector
    }

    /// Get mutable port connector for direct access to existing functionality
    pub fn get_port_connector_mut(&mut self) -> &mut PortConnector {
        &mut self.port_connector
    }
}

impl Default for QA5PortIntegration {
    fn default() -> Self {
        Self::new().expect("Failed to create default QA5 port integration")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_qa5_port_integration_creation() {
        let integration = QA5PortIntegration::new().unwrap();
        assert!(integration.get_all_operational_statuses().await.is_empty());
    }

    #[tokio::test]
    async fn test_qa5_service_registration() {
        let mut integration = QA5PortIntegration::new().unwrap();
        
        let config = QA5ServiceConfig {
            service_type: QA5ServiceType::GroupOperations,
            service_name: "test-group-ops".to_string(),
            primary_port: 8080,
            fallback_port: Some(8081),
            health_check_interval: Duration::from_secs(30),
            operational_capabilities: vec!["group_operations".to_string()],
            group_mapping: vec!["foundation".to_string()],
        };
        
        assert!(integration.register_qa5_service(config).await.is_ok());
        assert_eq!(integration.get_all_operational_statuses().await.len(), 1);
    }
}
