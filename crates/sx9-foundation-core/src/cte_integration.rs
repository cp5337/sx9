//! CTE Integration - Health Bridge and Agent Registry
//!
//! Integrates with Cognitive Tactics Engine for health monitoring,
//! agent coordination, and rollcall system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CTE Health Bridge for foundation integration
#[derive(Debug, Clone)]
pub struct CTEHealthBridge {
    pub agent_registry: AgentRegistry,
    pub port_manager: PortManager,
    pub rollcall_integration: RollcallBridge,
    pub health_endpoints: Vec<HealthEndpoint>,
    pub connected: bool,
}

/// Agent Registry from CTE agents.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistry {
    pub registry_version: String,
    pub agents: Vec<CTEAgent>,
}

/// CTE Agent definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTEAgent {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub ea_code: String,
    pub xsd_symbol: String,
    pub xsd_ref: String,
    pub status: AgentStatus,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Online,
    Offline,
    Unknown,
}

/// Port Manager for CTE system
#[derive(Debug, Clone)]
pub struct PortManager {
    pub primary_ports: HashMap<String, u16>,
    pub failover_ports: HashMap<String, Vec<u16>>,
    pub port_health: HashMap<u16, bool>,
}

/// Rollcall Bridge for health checks
#[derive(Debug, Clone)]
pub struct RollcallBridge {
    pub rollcall_script_path: String,
    pub last_rollcall: Option<RollcallResult>,
    pub auto_rollcall: bool,
    pub rollcall_interval_seconds: u64,
}

/// Rollcall result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollcallResult {
    pub timestamp: String,
    pub agents_checked: u32,
    pub agents_online: u32,
    pub agents_offline: u32,
    pub mux_status: MuxStatus,
    pub overall_health: OverallHealth,
}

/// Mux status from rollcall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuxStatus {
    pub mux_url: String,
    pub mux_responding: bool,
    pub response_time_ms: u32,
}

/// Overall health assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverallHealth {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// Health endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEndpoint {
    pub name: String,
    pub url: String,
    pub method: String,
    pub expected_status: u16,
    pub timeout_ms: u32,
    pub last_check: Option<HealthCheckResult>,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub timestamp: String,
    pub status_code: u16,
    pub response_time_ms: u32,
    pub healthy: bool,
    pub error_message: Option<String>,
}

impl CTEHealthBridge {
    pub fn new() -> Self {
        Self {
            agent_registry: AgentRegistry::new(),
            port_manager: PortManager::new(),
            rollcall_integration: RollcallBridge::new(),
            health_endpoints: Vec::new(),
            connected: false,
        }
    }

    /// Connect to CTE system
    pub async fn connect_to_cte(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Connecting to Cognitive Tactics Engine...");

        // Load agent registry
        self.load_agent_registry().await?;

        // Initialize port manager
        self.port_manager.initialize_ports();

        // Setup health endpoints
        self.setup_health_endpoints();

        // Test connection
        self.test_cte_connection().await?;

        self.connected = true;
        println!("✅ CTE Health Bridge: Connected");

        Ok(())
    }

    /// Load agent registry from CTE
    async fn load_agent_registry(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would load from agents.json
        self.agent_registry = AgentRegistry {
            registry_version: "1.0".to_string(),
            agents: vec![
                CTEAgent {
                    id: "repoagent".to_string(),
                    name: "RepoAgent".to_string(),
                    base_url: "http://localhost:15180".to_string(),
                    ea_code: "EA-FND-CORE".to_string(),
                    xsd_symbol: "⟂RS".to_string(),
                    xsd_ref: "xsd://ctas7/repo/status@1.2.0".to_string(),
                    status: AgentStatus::Unknown,
                },
                CTEAgent {
                    id: "cove".to_string(),
                    name: "Lachlan 'Cove' Harris".to_string(),
                    base_url: "http://localhost:15180".to_string(),
                    ea_code: "EA-ARCH".to_string(),
                    xsd_symbol: "⚙︎CX".to_string(),
                    xsd_ref: "xsd://ctas7/playbook-recursive.v1".to_string(),
                    status: AgentStatus::Unknown,
                },
            ],
        };

        println!("📋 Agent Registry loaded: {} agents", self.agent_registry.agents.len());
        Ok(())
    }

    /// Setup health endpoints
    fn setup_health_endpoints(&mut self) {
        self.health_endpoints = vec![
            HealthEndpoint {
                name: "RepoAgent Health".to_string(),
                url: "http://localhost:15180/health".to_string(),
                method: "GET".to_string(),
                expected_status: 200,
                timeout_ms: 5000,
                last_check: None,
            },
            HealthEndpoint {
                name: "RepoAgent Status".to_string(),
                url: "http://localhost:15180/repo/status".to_string(),
                method: "GET".to_string(),
                expected_status: 200,
                timeout_ms: 5000,
                last_check: None,
            },
            HealthEndpoint {
                name: "Playbook Mux".to_string(),
                url: "http://localhost:15180/mux/playbook?id=demo-ready&fmt=xml&ver=latest".to_string(),
                method: "GET".to_string(),
                expected_status: 200,
                timeout_ms: 5000,
                last_check: None,
            },
        ];

        println!("🏥 Health endpoints configured: {}", self.health_endpoints.len());
    }

    /// Test CTE connection
    async fn test_cte_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧪 Testing CTE connection...");

        // Test primary port
        if self.port_manager.check_port_health(15180).await {
            println!("✅ Primary port 15180: Available");
        } else {
            println!("❌ Primary port 15180: Unavailable");
        }

        // Update agent statuses
        let agents_clone = self.agent_registry.agents.clone();
        for (index, agent) in agents_clone.iter().enumerate() {
            self.agent_registry.agents[index].status = if self.test_agent_connection(&agent.base_url).await {
                AgentStatus::Online
            } else {
                AgentStatus::Offline
            };
        }

        Ok(())
    }

    /// Test individual agent connection
    async fn test_agent_connection(&self, base_url: &str) -> bool {
        // In a real implementation, this would make HTTP requests
        // For now, simulate connection test
        println!("🔍 Testing connection to: {}", base_url);
        false // Simulate offline for now
    }

    /// Perform rollcall check
    pub async fn perform_rollcall(&mut self) -> Result<RollcallResult, Box<dyn std::error::Error>> {
        println!("📞 Performing CTE rollcall...");

        let mut agents_online = 0;
        let mut agents_offline = 0;

        // Clone to avoid borrow checker issues
        let agents_clone = self.agent_registry.agents.clone();
        for (index, agent) in agents_clone.iter().enumerate() {
            if self.test_agent_connection(&agent.base_url).await {
                self.agent_registry.agents[index].status = AgentStatus::Online;
                agents_online += 1;
            } else {
                self.agent_registry.agents[index].status = AgentStatus::Offline;
                agents_offline += 1;
            }
        }

        let mux_status = MuxStatus {
            mux_url: "http://localhost:15180/mux/playbook?id=demo-ready&fmt=xml&ver=latest".to_string(),
            mux_responding: false,
            response_time_ms: 0,
        };

        let overall_health = if agents_online > 0 {
            OverallHealth::Degraded
        } else {
            OverallHealth::Critical
        };

        let result = RollcallResult {
            timestamp: chrono::Utc::now().to_rfc3339(),
            agents_checked: self.agent_registry.agents.len() as u32,
            agents_online,
            agents_offline,
            mux_status,
            overall_health,
        };

        self.rollcall_integration.last_rollcall = Some(result.clone());

        println!("📊 Rollcall complete: {}/{} agents online", agents_online, agents_online + agents_offline);

        Ok(result)
    }

    /// Get overall health status
    pub fn get_health_status(&self) -> OverallHealth {
        if !self.connected {
            return OverallHealth::Critical;
        }

        if let Some(rollcall) = &self.rollcall_integration.last_rollcall {
            rollcall.overall_health.clone()
        } else {
            OverallHealth::Unknown
        }
    }

    /// Generate health report
    pub fn generate_health_report(&self) -> String {
        let status = match self.get_health_status() {
            OverallHealth::Healthy => "🟢 HEALTHY",
            OverallHealth::Degraded => "🟡 DEGRADED",
            OverallHealth::Critical => "🔴 CRITICAL",
            OverallHealth::Unknown => "⚪ UNKNOWN",
        };

        format!(
            "CTE Health Bridge Report:\n\
             Connection Status: {}\n\
             Overall Health: {}\n\
             Registered Agents: {}\n\
             Health Endpoints: {}",
            if self.connected { "CONNECTED" } else { "DISCONNECTED" },
            status,
            self.agent_registry.agents.len(),
            self.health_endpoints.len()
        )
    }
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            registry_version: "1.0".to_string(),
            agents: Vec::new(),
        }
    }
}

impl PortManager {
    pub fn new() -> Self {
        Self {
            primary_ports: HashMap::new(),
            failover_ports: HashMap::new(),
            port_health: HashMap::new(),
        }
    }

    fn initialize_ports(&mut self) {
        // CTE port configuration
        self.primary_ports.insert("repoagent".to_string(), 15180);
        self.primary_ports.insert("api_gateway".to_string(), 18103);
        self.primary_ports.insert("mcp_forge".to_string(), 3000);
        self.primary_ports.insert("monitoring".to_string(), 18108);

        // Failover ports
        self.failover_ports.insert("repoagent".to_string(), vec![15181, 15182]);
        self.failover_ports.insert("api_gateway".to_string(), vec![18104, 18105]);
        self.failover_ports.insert("mcp_forge".to_string(), vec![3001, 3002]);
        self.failover_ports.insert("monitoring".to_string(), vec![18109, 18110]);

        println!("🔌 Port configuration initialized");
    }

    async fn check_port_health(&mut self, port: u16) -> bool {
        // In a real implementation, this would check if port is available
        // For now, simulate port check
        self.port_health.insert(port, false);
        false
    }
}

impl RollcallBridge {
    pub fn new() -> Self {
        Self {
            rollcall_script_path: "./rollcall.sh".to_string(),
            last_rollcall: None,
            auto_rollcall: true,
            rollcall_interval_seconds: 300, // 5 minutes
        }
    }
}

impl Default for CTEHealthBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cte_bridge_creation() {
        let bridge = CTEHealthBridge::new();
        assert!(!bridge.connected);
        assert_eq!(bridge.agent_registry.agents.len(), 0);
    }

    #[tokio::test]
    async fn test_agent_registry_loading() {
        let mut bridge = CTEHealthBridge::new();
        bridge.load_agent_registry().await.unwrap();

        assert_eq!(bridge.agent_registry.agents.len(), 2);
        assert_eq!(bridge.agent_registry.agents[0].id, "repoagent");
    }

    #[test]
    fn test_port_manager_initialization() {
        let mut port_manager = PortManager::new();
        port_manager.initialize_ports();

        assert_eq!(port_manager.primary_ports.get("repoagent"), Some(&15180));
        assert_eq!(port_manager.primary_ports.get("mcp_forge"), Some(&3000));
    }

    #[tokio::test]
    async fn test_rollcall_execution() {
        let mut bridge = CTEHealthBridge::new();
        bridge.load_agent_registry().await.unwrap();

        let result = bridge.perform_rollcall().await.unwrap();
        assert_eq!(result.agents_checked, 2);
        assert!(bridge.rollcall_integration.last_rollcall.is_some());
    }
}