use crate::types::*;
use crate::errors::EVMError;
use crate::docker_borg_assimilator::{DockerBorgAssimilator, BorgConfig};
use crate::scanning_manifold::ScanningManifold;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use std::collections::HashMap;
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use std::path::PathBuf;
use tokio::time::{timeout, Duration, Instant};
use reqwest;

/// Caldera Integration - MITRE ATT&CK adversary emulation platform
/// Integrates with our Docker Borg Assimilator for red team operations
#[derive(Debug)]
pub struct CalderaIntegration {
    integration_id: Uuid,
    config: CalderaConfig,
    borg_assimilator: DockerBorgAssimilator,
    manifold: ScanningManifold,
    caldera_client: reqwest::Client,
    active_operations: Arc<tokio::sync::RwLock<HashMap<String, CalderaOperation>>>,
    adversary_profiles: Arc<tokio::sync::RwLock<HashMap<String, AdversaryProfile>>>,
    ability_database: Arc<tokio::sync::RwLock<HashMap<String, CalderaAbility>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaConfig {
    pub caldera_url: String,
    pub api_key: String,
    pub docker_integration: bool,
    pub borg_designation: String,
    pub workspace_dir: PathBuf,
    pub enable_att_ck_mapping: bool,
    pub auto_agent_deployment: bool,
    pub max_concurrent_operations: usize,
    pub operation_timeout: u64,
    pub fact_source_integration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaOperation {
    pub operation_id: String,
    pub name: String,
    pub adversary: String,
    pub group: String,
    pub state: OperationState,
    pub agents: Vec<CalderaAgent>,
    pub chain: Vec<CalderaLink>,
    pub facts: HashMap<String, CalderaFact>,
    pub planner: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration: Option<Duration>,
    pub borg_integration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationState {
    Planning,
    Running,
    RunOneLink,
    Paused,
    Finished,
    Cleanup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaAgent {
    pub paw: String,
    pub group: String,
    pub architecture: String,
    pub platform: String,
    pub location: String,
    pub pid: u32,
    pub ppid: u32,
    pub privilege: String,
    pub username: String,
    pub host: String,
    pub contact: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub sleep_min: u32,
    pub sleep_max: u32,
    pub watchdog: u32,
    pub trusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaLink {
    pub id: String,
    pub operation: String,
    pub paw: String,
    pub ability: String,
    pub command: String,
    pub status: LinkStatus,
    pub score: i32,
    pub jitter: String,
    pub decide: chrono::DateTime<chrono::Utc>,
    pub collect: Option<chrono::DateTime<chrono::Utc>>,
    pub finish: Option<chrono::DateTime<chrono::Utc>>,
    pub pin: i32,
    pub output: String,
    pub stderr: String,
    pub agent_reported_time: String,
    pub facts: Vec<CalderaFact>,
    pub unique: String,
    pub cleanup: i32,
    pub visibility: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkStatus {
    Success = 0,
    Failure = -1,
    Timeout = -2,
    Collected = -3,
    Untrusted = -4,
    Pause = 124,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaFact {
    pub unique: String,
    pub trait_name: String,
    pub value: String,
    pub score: i32,
    pub collected_by: Vec<String>,
    pub technique_id: Option<String>,
    pub source: String,
    pub origin_type: i32,
    pub created: chrono::DateTime<chrono::Utc>,
    pub limit_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversaryProfile {
    pub adversary_id: String,
    pub name: String,
    pub description: String,
    pub atomic_ordering: Vec<String>,
    pub objective: String,
    pub tags: Vec<String>,
    pub plugin: String,
    pub has_repeatable_abilities: bool,
    pub buckets: HashMap<String, Vec<String>>,
    pub borg_enhanced: bool,
    pub att_ck_mapping: Vec<AttackTechnique>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
    pub subtechniques: Vec<String>,
    pub platforms: Vec<String>,
    pub data_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaAbility {
    pub ability_id: String,
    pub tactic: String,
    pub technique_id: String,
    pub technique_name: String,
    pub name: String,
    pub description: String,
    pub cleanup: Vec<String>,
    pub executors: Vec<CalderaExecutor>,
    pub requirements: Vec<CalderaRequirement>,
    pub privilege: Option<String>,
    pub timeout: u32,
    pub repeatable: bool,
    pub buckets: Vec<String>,
    pub access: HashMap<String, Vec<String>>,
    pub additional_info: HashMap<String, serde_json::Value>,
    pub plugin: String,
    pub borg_wrapped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaExecutor {
    pub name: String,
    pub platform: String,
    pub command: String,
    pub code: Option<String>,
    pub build_target: Option<String>,
    pub language: Option<String>,
    pub payloads: Vec<String>,
    pub uploads: Vec<String>,
    pub timeout: u32,
    pub parsers: Vec<CalderaParser>,
    pub cleanup: Vec<String>,
    pub variations: Vec<CalderaVariation>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaParser {
    pub module: String,
    pub parserconfigs: Vec<CalderaParserConfig>,
    pub custom_parser_vals: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaParserConfig {
    pub source: String,
    pub edge: String,
    pub target: String,
    pub custom_parser_vals: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaVariation {
    pub description: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaRequirement {
    pub module: String,
    pub relationship_match: Vec<CalderaRelationshipMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaRelationshipMatch {
    pub source: String,
    pub edge: String,
    pub target: String,
}

impl CalderaIntegration {
    /// Create new Caldera integration with Borg assimilator
    pub async fn new(config: CalderaConfig) -> Result<Self, EVMError> {
        let integration_id = Uuid::new_v4();
        
        info!("🏴‍☠️ Initializing Caldera Integration {} - MITRE ATT&CK adversary emulation", integration_id);
        
        // Initialize Borg assimilator for Docker containerization
        let borg_config = BorgConfig {
            borg_namespace: config.borg_designation.clone(),
            ..Default::default()
        };
        let borg_assimilator = DockerBorgAssimilator::new(borg_config).await?;
        
        // Initialize manifold for tool execution
        let manifold = ScanningManifold::new(Default::default()).await?;
        
        // Create HTTP client for Caldera API
        let caldera_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| EVMError::IntegrationError(format!("Failed to create HTTP client: {}", e)))?;
        
        let integration = Self {
            integration_id,
            config,
            borg_assimilator,
            manifold,
            caldera_client,
            active_operations: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            adversary_profiles: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            ability_database: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        };
        
        // Test Caldera connectivity
        integration.test_caldera_connection().await?;
        
        // Load adversary profiles and abilities
        integration.load_adversary_profiles().await?;
        integration.load_ability_database().await?;
        
        // Assimilate Caldera into Borg collective
        integration.assimilate_caldera_into_borg().await?;
        
        info!("✅ Caldera Integration initialized - Ready for adversary emulation");
        Ok(integration)
    }
    
    /// Test connection to Caldera server
    async fn test_caldera_connection(&self) -> Result<(), EVMError> {
        info!("🔗 Testing connection to Caldera server: {}", self.config.caldera_url);
        
        let response = self.caldera_client
            .get(&format!("{}/api/v2/health", self.config.caldera_url))
            .header("KEY", &self.config.api_key)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to connect to Caldera: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(EVMError::IntegrationError(
                format!("Caldera health check failed with status: {}", response.status())
            ));
        }
        
        let health_data: serde_json::Value = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse health response: {}", e)))?;
        
        info!("✅ Caldera connection successful - Server health: {:?}", health_data);
        Ok(())
    }
    
    /// Load adversary profiles from Caldera
    async fn load_adversary_profiles(&self) -> Result<(), EVMError> {
        info!("📚 Loading adversary profiles from Caldera");
        
        let response = self.caldera_client
            .get(&format!("{}/api/v2/adversaries", self.config.caldera_url))
            .header("KEY", &self.config.api_key)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to fetch adversaries: {}", e)))?;
        
        let adversaries: Vec<serde_json::Value> = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse adversaries: {}", e)))?;
        
        let mut profiles = self.adversary_profiles.write().await;
        
        for adv_data in adversaries {
            let adversary = self.parse_adversary_profile(adv_data)?;
            profiles.insert(adversary.adversary_id.clone(), adversary);
        }
        
        info!("✅ Loaded {} adversary profiles", profiles.len());
        Ok(())
    }
    
    /// Load ability database from Caldera
    async fn load_ability_database(&self) -> Result<(), EVMError> {
        info!("⚡ Loading ability database from Caldera");
        
        let response = self.caldera_client
            .get(&format!("{}/api/v2/abilities", self.config.caldera_url))
            .header("KEY", &self.config.api_key)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to fetch abilities: {}", e)))?;
        
        let abilities: Vec<serde_json::Value> = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse abilities: {}", e)))?;
        
        let mut db = self.ability_database.write().await;
        
        for ability_data in abilities {
            let ability = self.parse_caldera_ability(ability_data)?;
            db.insert(ability.ability_id.clone(), ability);
        }
        
        info!("✅ Loaded {} abilities into database", db.len());
        Ok(())
    }
    
    /// Assimilate Caldera into Borg collective
    async fn assimilate_caldera_into_borg(&self) -> Result<(), EVMError> {
        info!("🤖 Assimilating Caldera into Borg collective - Resistance is futile");
        
        // Create Borg-wrapped Caldera container
        let caldera_dockerfile = self.generate_caldera_borg_dockerfile();
        let dockerfile_path = PathBuf::from("/tmp/Dockerfile.caldera-borg");
        
        tokio::fs::write(&dockerfile_path, caldera_dockerfile).await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to write Caldera Dockerfile: {}", e)))?;
        
        // Build Borg Caldera image
        let build_result = Command::new("docker")
            .args(&[
                "build",
                "-t", &format!("{}/caldera-borg:latest", self.config.borg_designation),
                "-f", dockerfile_path.to_str().unwrap(),
                "/tmp",
            ])
            .output()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to build Caldera Borg image: {}", e)))?;
        
        if !build_result.status.success() {
            let stderr = String::from_utf8_lossy(&build_result.stderr);
            return Err(EVMError::IntegrationError(format!("Caldera Borg build failed: {}", stderr)));
        }
        
        info!("✅ Caldera successfully assimilated into Borg collective");
        Ok(())
    }
    
    /// Generate Dockerfile for Borg-integrated Caldera
    fn generate_caldera_borg_dockerfile(&self) -> String {
        format!(r#"
FROM {}/borg-base:latest

# Caldera Borg Assimilation
LABEL org.ctas.borg.tool="caldera"
LABEL org.ctas.borg.designation="BORG-CALDERA-001"
LABEL org.ctas.borg.type="adversary_emulation"

# Install Python and dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    git \
    curl \
    && apt-get clean

# Clone Caldera
RUN git clone --recurse-submodules https://github.com/mitre/caldera.git /opt/caldera
WORKDIR /opt/caldera

# Install Caldera dependencies
RUN pip3 install -r requirements.txt

# Install additional plugins for enhanced capabilities
RUN pip3 install docker requests

# Create Borg integration layer
COPY caldera-borg-wrapper.py /opt/borg/
RUN chmod +x /opt/borg/caldera-borg-wrapper.py

# Set Caldera configuration for Borg integration
ENV CALDERA_URL="{}"
ENV BORG_CALDERA_MODE="true"
ENV BORG_HASH_INTEGRATION="enabled"

# Create startup script
RUN echo '#!/bin/bash' > /opt/borg/caldera-startup.sh && \
    echo 'cd /opt/caldera' >> /opt/borg/caldera-startup.sh && \
    echo 'python3 server.py --insecure &' >> /opt/borg/caldera-startup.sh && \
    echo 'sleep 10' >> /opt/borg/caldera-startup.sh && \
    echo 'python3 /opt/borg/caldera-borg-wrapper.py "$@"' >> /opt/borg/caldera-startup.sh && \
    chmod +x /opt/borg/caldera-startup.sh

EXPOSE 8888 8443 7010 7011 7012

ENTRYPOINT ["/opt/borg/caldera-startup.sh"]
CMD ["start"]
"#,
            self.config.borg_designation,
            self.config.caldera_url
        )
    }
    
    /// Create new adversary operation
    pub async fn create_operation(
        &self,
        name: &str,
        adversary_id: &str,
        group: &str,
        planner: Option<&str>,
    ) -> Result<String, EVMError> {
        info!("🎯 Creating Caldera operation: {} with adversary: {}", name, adversary_id);
        
        let operation_data = serde_json::json!({
            "name": name,
            "adversary": {"adversary_id": adversary_id},
            "group": group,
            "planner": {"id": planner.unwrap_or("atomic")},
            "state": "planning",
            "autonomous": 1,
            "auto_close": false,
            "visibility": "51"
        });
        
        let response = self.caldera_client
            .put(&format!("{}/api/v2/operations", self.config.caldera_url))
            .header("KEY", &self.config.api_key)
            .json(&operation_data)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to create operation: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(EVMError::IntegrationError(
                format!("Operation creation failed with status: {}", response.status())
            ));
        }
        
        let operation: serde_json::Value = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse operation response: {}", e)))?;
        
        let operation_id = operation["id"].as_str()
            .ok_or_else(|| EVMError::IntegrationError("No operation ID returned".to_string()))?
            .to_string();
        
        // Store operation in our tracking system
        let caldera_op = CalderaOperation {
            operation_id: operation_id.clone(),
            name: name.to_string(),
            adversary: adversary_id.to_string(),
            group: group.to_string(),
            state: OperationState::Planning,
            agents: vec![],
            chain: vec![],
            facts: HashMap::new(),
            planner: planner.unwrap_or("atomic").to_string(),
            start_time: chrono::Utc::now(),
            duration: None,
            borg_integration: true,
        };
        
        {
            let mut operations = self.active_operations.write().await;
            operations.insert(operation_id.clone(), caldera_op);
        }
        
        info!("✅ Operation created with ID: {}", operation_id);
        Ok(operation_id)
    }
    
    /// Start adversary operation
    pub async fn start_operation(&self, operation_id: &str) -> Result<(), EVMError> {
        info!("🚀 Starting Caldera operation: {}", operation_id);
        
        let response = self.caldera_client
            .patch(&format!("{}/api/v2/operations/{}", self.config.caldera_url, operation_id))
            .header("KEY", &self.config.api_key)
            .json(&serde_json::json!({"state": "running"}))
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to start operation: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(EVMError::IntegrationError(
                format!("Operation start failed with status: {}", response.status())
            ));
        }
        
        // Update local tracking
        {
            let mut operations = self.active_operations.write().await;
            if let Some(op) = operations.get_mut(operation_id) {
                op.state = OperationState::Running;
            }
        }
        
        info!("✅ Operation {} started successfully", operation_id);
        Ok(())
    }
    
    /// Get operation status and results
    pub async fn get_operation_status(&self, operation_id: &str) -> Result<CalderaOperationStatus, EVMError> {
        let response = self.caldera_client
            .get(&format!("{}/api/v2/operations/{}", self.config.caldera_url, operation_id))
            .header("KEY", &self.config.api_key)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to get operation status: {}", e)))?;
        
        let operation_data: serde_json::Value = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse operation status: {}", e)))?;
        
        let status = CalderaOperationStatus {
            operation_id: operation_id.to_string(),
            state: self.parse_operation_state(operation_data["state"].as_str().unwrap_or("unknown")),
            chain_length: operation_data["chain"].as_array().map(|c| c.len()).unwrap_or(0),
            agent_count: operation_data["host_group"].as_array().map(|h| h.len()).unwrap_or(0),
            facts_discovered: operation_data["facts"].as_array().map(|f| f.len()).unwrap_or(0),
            start_time: chrono::Utc::now(), // Would parse from response
            elapsed_time: Duration::from_secs(0), // Would calculate from response
            borg_enhanced: true,
        };
        
        Ok(status)
    }
    
    /// Execute single technique through Borg integration
    pub async fn execute_technique_borg(
        &self,
        ability_id: &str,
        agent_paw: &str,
        facts: Option<HashMap<String, String>>,
        hash: Option<&str>,
    ) -> Result<CalderaBorgResult, EVMError> {
        info!("⚡ Executing technique {} on agent {} through Borg", ability_id, agent_paw);
        
        // Get ability details
        let ability = {
            let db = self.ability_database.read().await;
            db.get(ability_id).cloned()
                .ok_or_else(|| EVMError::IntegrationError(format!("Ability {} not found", ability_id)))?
        };
        
        // If Borg-wrapped, execute through Docker
        if ability.borg_wrapped {
            return self.execute_borg_wrapped_technique(ability_id, agent_paw, facts, hash).await;
        }
        
        // Otherwise, execute through standard Caldera API
        let manual_command = serde_json::json!({
            "paw": agent_paw,
            "ability": {"ability_id": ability_id},
            "facts": facts.unwrap_or_default()
        });
        
        let response = self.caldera_client
            .put(&format!("{}/api/v2/operations/{}/potential-links", self.config.caldera_url, "manual"))
            .header("KEY", &self.config.api_key)
            .json(&manual_command)
            .send()
            .await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to execute technique: {}", e)))?;
        
        let link_data: serde_json::Value = response.json().await
            .map_err(|e| EVMError::IntegrationError(format!("Failed to parse technique response: {}", e)))?;
        
        let result = CalderaBorgResult {
            technique_id: ability.technique_id,
            ability_id: ability_id.to_string(),
            agent_paw: agent_paw.to_string(),
            execution_time: Duration::from_secs(0), // Would measure actual time
            success: response.status().is_success(),
            output: link_data["output"].as_str().unwrap_or("").to_string(),
            facts_collected: vec![], // Would parse from response
            borg_enhanced: ability.borg_wrapped,
            hash_used: hash.map(String::from),
        };
        
        info!("✅ Technique execution completed - Success: {}", result.success);
        Ok(result)
    }
    
    /// Execute Borg-wrapped technique
    async fn execute_borg_wrapped_technique(
        &self,
        ability_id: &str,
        agent_paw: &str,
        facts: Option<HashMap<String, String>>,
        hash: Option<&str>,
    ) -> Result<CalderaBorgResult, EVMError> {
        info!("🤖 Executing Borg-wrapped technique: {}", ability_id);
        
        // Execute through Borg assimilator
        let borg_result = self.borg_assimilator.borg_execute(
            "caldera",
            hash,
            vec![
                "execute-technique".to_string(),
                ability_id.to_string(),
                agent_paw.to_string(),
            ]
        ).await?;
        
        let result = CalderaBorgResult {
            technique_id: ability_id.to_string(),
            ability_id: ability_id.to_string(),
            agent_paw: agent_paw.to_string(),
            execution_time: borg_result.execution_time,
            success: borg_result.exit_code == 0,
            output: borg_result.stdout,
            facts_collected: vec![], // Would parse from Borg output
            borg_enhanced: true,
            hash_used: hash.map(String::from),
        };
        
        Ok(result)
    }
    
    // Helper parsing methods
    
    fn parse_adversary_profile(&self, data: serde_json::Value) -> Result<AdversaryProfile, EVMError> {
        // Simplified parsing - would implement full JSON deserialization
        Ok(AdversaryProfile {
            adversary_id: data["adversary_id"].as_str().unwrap_or("unknown").to_string(),
            name: data["name"].as_str().unwrap_or("Unknown").to_string(),
            description: data["description"].as_str().unwrap_or("").to_string(),
            atomic_ordering: vec![], // Would parse from data
            objective: data["objective"].as_str().unwrap_or("").to_string(),
            tags: vec![], // Would parse from data
            plugin: data["plugin"].as_str().unwrap_or("").to_string(),
            has_repeatable_abilities: false,
            buckets: HashMap::new(),
            borg_enhanced: true,
            att_ck_mapping: vec![], // Would parse ATT&CK techniques
        })
    }
    
    fn parse_caldera_ability(&self, data: serde_json::Value) -> Result<CalderaAbility, EVMError> {
        // Simplified parsing - would implement full JSON deserialization
        Ok(CalderaAbility {
            ability_id: data["ability_id"].as_str().unwrap_or("unknown").to_string(),
            tactic: data["tactic"].as_str().unwrap_or("").to_string(),
            technique_id: data["technique_id"].as_str().unwrap_or("").to_string(),
            technique_name: data["technique_name"].as_str().unwrap_or("").to_string(),
            name: data["name"].as_str().unwrap_or("").to_string(),
            description: data["description"].as_str().unwrap_or("").to_string(),
            cleanup: vec![], // Would parse from data
            executors: vec![], // Would parse from data
            requirements: vec![], // Would parse from data
            privilege: None,
            timeout: 60,
            repeatable: false,
            buckets: vec![],
            access: HashMap::new(),
            additional_info: HashMap::new(),
            plugin: data["plugin"].as_str().unwrap_or("").to_string(),
            borg_wrapped: false, // Would determine based on analysis
        })
    }
    
    fn parse_operation_state(&self, state_str: &str) -> OperationState {
        match state_str {
            "planning" => OperationState::Planning,
            "running" => OperationState::Running,
            "run-one-link" => OperationState::RunOneLink,
            "paused" => OperationState::Paused,
            "finished" => OperationState::Finished,
            "cleanup" => OperationState::Cleanup,
            _ => OperationState::Planning,
        }
    }
}

use std::sync::Arc;

// Result and status types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaOperationStatus {
    pub operation_id: String,
    pub state: OperationState,
    pub chain_length: usize,
    pub agent_count: usize,
    pub facts_discovered: usize,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub elapsed_time: Duration,
    pub borg_enhanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalderaBorgResult {
    pub technique_id: String,
    pub ability_id: String,
    pub agent_paw: String,
    pub execution_time: Duration,
    pub success: bool,
    pub output: String,
    pub facts_collected: Vec<CalderaFact>,
    pub borg_enhanced: bool,
    pub hash_used: Option<String>,
}

impl Default for CalderaConfig {
    fn default() -> Self {
        Self {
            caldera_url: "http://localhost:8888".to_string(),
            api_key: "admin123".to_string(),
            docker_integration: true,
            borg_designation: "ctas-borg".to_string(),
            workspace_dir: PathBuf::from("/tmp/caldera-borg"),
            enable_att_ck_mapping: true,
            auto_agent_deployment: false,
            max_concurrent_operations: 5,
            operation_timeout: 3600,
            fact_source_integration: true,
        }
    }
}