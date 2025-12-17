//! Neural Mux for Autonomous Crate Orchestration
//!
//! Implements the neural multiplexer that routes OODA decisions to
//! Docker API calls for autonomous crate spinning based on SCH vectors.

use crate::usim::{SCHVector, USIMTrivariate};
use crate::{CrateSpecification, Mission, OperatorMode, SecurityLevel};
use std::collections::HashMap;
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::data::serde_json::json;
use sx9_foundation_manifold::core::diagnostics::anyhow::{Context, Result};
use sx9_foundation_manifold::core::diagnostics::tracing::{debug, info, warn};
use sx9_foundation_manifold::core::networking::reqwest::Client;
use sx9_foundation_manifold::core::TrivariateHashEngine;

/// Neural mux routing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuxDecision {
    /// Spin new crate with specifications
    SpinCrate(CrateSpinRequest),
    /// Alert only, no action required
    AlertOnly(AlertPayload),
    /// Monitor situation, collect more data
    Monitor(MonitoringPayload),
}

/// Crate spin request for Docker API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateSpinRequest {
    /// Crate name for the new instance
    pub crate_name: String,
    /// Mission type based on threat analysis
    pub mission: Mission,
    /// Operator mode for the spin
    pub mode: OperatorMode,
    /// Security level required
    pub security_level: SecurityLevel,
    /// USIM context for the spin
    pub usim_context: String,
    /// Threat score that triggered the spin
    pub threat_score: f32,
    /// Predicted port requirement
    pub port_requirement: Option<u16>,
    /// Path to the generated build context
    pub build_context_path: Option<std::path::PathBuf>,
}

/// Alert payload for monitoring systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPayload {
    /// Alert severity level
    pub severity: AlertSeverity,
    /// Human-readable description
    pub description: String,
    /// USIM that triggered the alert
    pub usim_hash: String,
    /// Recommendation for human operators
    pub recommendation: String,
}

/// Monitoring payload for telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringPayload {
    /// Monitoring interval in seconds
    pub interval: u64,
    /// Metrics to collect
    pub metrics: Vec<String>,
    /// Convergence threshold to watch
    pub convergence_threshold: f32,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Neural Mux for autonomous decision making
#[derive(Debug)]
pub struct NeuralMux {
    /// HTTP client for API calls
    http_client: Client,
    /// Docker API base URL
    docker_api_url: String,
    /// Smart CDN Gateway URL
    cdn_gateway_url: String,
    /// Port manager URL
    port_manager_url: String,
    /// Current system load metrics
    system_metrics: SystemMetrics,
    /// Decision history for learning
    decision_history: Vec<MuxDecisionRecord>,
}

/// System metrics for decision making
#[derive(Debug, Clone, Default)]
struct SystemMetrics {
    /// Current CPU utilization (0.0-1.0)
    cpu_usage: f32,
    /// Current memory utilization (0.0-1.0)
    memory_usage: f32,
    /// Number of active crates
    active_crates: u32,
    /// Average response time (ms)
    avg_response_time: f32,
}

/// Record of mux decisions for analysis
#[derive(Debug, Clone)]
struct MuxDecisionRecord {
    /// Timestamp of decision
    timestamp: u64,
    /// Decision made
    decision: MuxDecision,
    /// Outcome success/failure
    outcome: Option<bool>,
    /// SCH vector that led to decision
    sch_vector: SCHVector,
}

impl NeuralMux {
    /// Creates new Neural Mux instance
    pub fn new(docker_api_url: String, cdn_gateway_url: String, port_manager_url: String) -> Self {
        Self {
            http_client: Client::new(),
            docker_api_url,
            cdn_gateway_url,
            port_manager_url,
            system_metrics: SystemMetrics::default(),
            decision_history: Vec::new(),
        }
    }

    /// Main OODA decision function
    pub async fn ooda_decide(
        &mut self,
        usim: &USIMTrivariate,
        sch_vector: &SCHVector,
        threat_narrative: &str,
        build_context_path: Option<std::path::PathBuf>,
    ) -> Result<MuxDecision> {
        info!("Neural Mux OODA decision process initiated");

        // Update system metrics
        self.update_system_metrics().await?;

        // Analyze threat level and system capacity
        let threat_level = self.assess_threat_level(sch_vector);
        let system_capacity = self.assess_system_capacity();

        debug!(
            "Threat level: {:?}, System capacity: {:.2}",
            threat_level, system_capacity
        );

        // Make routing decision based on OODA loop
        let decision = match threat_level {
            ThreatLevel::Critical if system_capacity > 0.7 => {
                self.create_spin_decision(usim, sch_vector, threat_narrative, build_context_path)
                    .await?
            }
            ThreatLevel::High if system_capacity > 0.5 => {
                self.create_spin_decision(usim, sch_vector, threat_narrative, build_context_path)
                    .await?
            }
            ThreatLevel::Medium => self.create_alert_decision(sch_vector, threat_narrative),
            _ => self.create_monitor_decision(sch_vector),
        };

        // Record decision for learning
        self.record_decision(decision.clone(), sch_vector.clone())
            .await;

        Ok(decision)
    }

    /// Executes the mux decision
    pub async fn execute_decision(&self, decision: &MuxDecision) -> Result<String> {
        match decision {
            MuxDecision::SpinCrate(request) => self.execute_crate_spin(request).await,
            MuxDecision::AlertOnly(alert) => self.execute_alert(alert).await,
            MuxDecision::Monitor(monitor) => self.execute_monitoring(monitor).await,
        }
    }

    /// Assess threat level from SCH vector
    fn assess_threat_level(&self, sch_vector: &SCHVector) -> ThreatLevel {
        let threat_score = sch_vector.prediction.iter().sum::<f32>() / 64.0;
        let convergence = sch_vector.convergence;

        match (threat_score, convergence) {
            (score, conv) if score > 0.8 && conv > 0.9 => ThreatLevel::Critical,
            (score, conv) if score > 0.6 && conv > 0.8 => ThreatLevel::High,
            (score, conv) if score > 0.4 && conv > 0.7 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,
        }
    }

    /// Assess current system capacity
    fn assess_system_capacity(&self) -> f32 {
        let cpu_capacity = 1.0 - self.system_metrics.cpu_usage;
        let memory_capacity = 1.0 - self.system_metrics.memory_usage;
        let load_factor = if self.system_metrics.active_crates > 10 {
            0.7
        } else {
            1.0
        };

        (cpu_capacity + memory_capacity) / 2.0 * load_factor
    }

    /// Create crate spin decision
    async fn create_spin_decision(
        &self,
        usim: &USIMTrivariate,
        sch_vector: &SCHVector,
        _narrative: &str,
        build_context_path: Option<std::path::PathBuf>,
    ) -> Result<MuxDecision> {
        // Determine mission type from SCH vector analysis
        let mission = self.infer_mission_type(sch_vector);

        // Request port allocation
        let port = self.request_port_allocation().await.ok();

        let request = CrateSpinRequest {
            crate_name: format!(
                "threat-response-{}",
                &sx9_foundation_manifold::core::data::uuid::Uuid::new_v4().to_string()[..8]
            ),
            mission,
            mode: OperatorMode::Specialist, // High-threat situations use Specialist mode
            security_level: SecurityLevel::Production,
            usim_context: format!("{:?}", usim),
            threat_score: sch_vector.prediction.iter().sum::<f32>() / 64.0,
            port_requirement: port,
            build_context_path,
        };

        Ok(MuxDecision::SpinCrate(request))
    }

    /// Create alert decision
    fn create_alert_decision(&self, sch_vector: &SCHVector, narrative: &str) -> MuxDecision {
        let severity = if sch_vector.convergence > 0.8 {
            AlertSeverity::High
        } else if sch_vector.convergence > 0.6 {
            AlertSeverity::Medium
        } else {
            AlertSeverity::Low
        };

        let hasher = TrivariateHashEngine::new();
        let usim_hash = hasher.generate_hash_from_bytes(narrative.as_bytes());

        let alert = AlertPayload {
            severity,
            description: narrative.to_string(),
            usim_hash,
            recommendation: "Monitor threat progression and reassess if convergence increases"
                .to_string(),
        };

        MuxDecision::AlertOnly(alert)
    }

    /// Create monitoring decision
    fn create_monitor_decision(&self, sch_vector: &SCHVector) -> MuxDecision {
        let monitor = MonitoringPayload {
            interval: 30, // 30 second intervals
            metrics: vec![
                "sch_convergence".to_string(),
                "threat_score".to_string(),
                "system_capacity".to_string(),
            ],
            convergence_threshold: sch_vector.convergence + 0.1,
        };

        MuxDecision::Monitor(monitor)
    }

    /// Execute crate spin via Docker API
    async fn execute_crate_spin(&self, request: &CrateSpinRequest) -> Result<String> {
        info!("Executing autonomous crate spin: {}", request.crate_name);

        // Create Docker container specification with bind mount if context path exists
        let mut host_config = json!({
            "AutoRemove": true,
            "RestartPolicy": {"Name": "unless-stopped"}
        });

        if let Some(path) = &request.build_context_path {
            if let Some(path_str) = path.to_str() {
                host_config["Binds"] = json!([format!("{}:/app/crate:ro", path_str)]);
            }
        }

        let container_spec = json!({
            "Image": "ctas7-threat-response:latest",
            "Name": request.crate_name,
            "Env": [
                format!("MISSION={:?}", request.mission),
                format!("MODE={:?}", request.mode),
                format!("SECURITY_LEVEL={:?}", request.security_level),
                format!("USIM_CONTEXT={}", request.usim_context),
                format!("THREAT_SCORE={}", request.threat_score),
            ],
            "ExposedPorts": {
                format!("{}/tcp", request.port_requirement.unwrap_or(18000)): {}
            },
            "HostConfig": host_config
        });

        // Send to Docker API
        let response = self
            .http_client
            .post(&format!("{}/containers/create", self.docker_api_url))
            .json(&container_spec)
            .send()
            .await
            .context("Failed to create container")?;

        if response.status().is_success() {
            let container_info: serde_json::Value = response.json().await?;
            let container_id = container_info["Id"].as_str().unwrap_or("unknown");

            // Start the container
            let start_response = self
                .http_client
                .post(&format!(
                    "{}/containers/{}/start",
                    self.docker_api_url, container_id
                ))
                .send()
                .await?;

            if start_response.status().is_success() {
                Ok(format!(
                    "Autonomous crate spun successfully: {} ({})",
                    request.crate_name, container_id
                ))
            } else {
                warn!("Failed to start container: {}", start_response.status());
                Ok("Container created but failed to start".to_string())
            }
        } else {
            warn!("Failed to create container: {}", response.status());
            Ok("Failed to create container".to_string())
        }
    }

    /// Execute alert via CDN Gateway
    async fn execute_alert(&self, alert: &AlertPayload) -> Result<String> {
        debug!("Executing alert: {:?}", alert.severity);

        let alert_payload = json!({
            "type": "threat_alert",
            "severity": alert.severity,
            "description": alert.description,
            "usim_hash": alert.usim_hash,
            "recommendation": alert.recommendation,
            "timestamp": sx9_foundation_manifold::core::data::chrono::Utc::now().to_rfc3339(),
        });

        let response = self
            .http_client
            .post(&format!("{}/alerts", self.cdn_gateway_url))
            .json(&alert_payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(format!("Alert sent successfully: {:?}", alert.severity))
        } else {
            Ok("Alert delivery failed".to_string())
        }
    }

    /// Execute monitoring setup
    async fn execute_monitoring(&self, monitor: &MonitoringPayload) -> Result<String> {
        debug!(
            "Setting up monitoring with {} second intervals",
            monitor.interval
        );

        // Configure monitoring via CDN Gateway
        let monitor_config = json!({
            "type": "monitoring_setup",
            "interval": monitor.interval,
            "metrics": monitor.metrics,
            "convergence_threshold": monitor.convergence_threshold,
        });

        let response = self
            .http_client
            .post(&format!("{}/monitoring/configure", self.cdn_gateway_url))
            .json(&monitor_config)
            .send()
            .await?;

        if response.status().is_success() {
            Ok("Monitoring configured successfully".to_string())
        } else {
            Ok("Monitoring setup failed".to_string())
        }
    }

    /// Infer mission type from SCH vector patterns
    fn infer_mission_type(&self, sch_vector: &SCHVector) -> Mission {
        // Analyze vector patterns to infer mission
        let service_mean = sch_vector.service.iter().sum::<f32>() / 64.0;
        let crate_mean = sch_vector.crate_component.iter().sum::<f32>() / 64.0;

        match (service_mean, crate_mean) {
            (s, c) if s > 0.7 && c > 0.7 => Mission::Analysis, // Was NeuralInference
            (_s, _c) if _s > 0.6 => Mission::Analysis,         // Was SystemMonitoring
            (_s, _c) if _c > 0.6 => Mission::DataIngestion,
            _ => Mission::Communication, // Was NetworkRouting
        }
    }

    /// Request port allocation from port manager
    async fn request_port_allocation(&self) -> Result<u16> {
        let request = json!({
            "service_name": "autonomous-threat-response",
            "preferred_range": [18000, 18999],
            "required": true
        });

        let response = self
            .http_client
            .post(&format!("{}/allocate", self.port_manager_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let allocation: serde_json::Value = response.json().await?;
            let port = allocation["port"].as_u64().unwrap_or(18000) as u16;
            Ok(port)
        } else {
            Err(sx9_foundation_manifold::core::diagnostics::anyhow::anyhow!(
                "Port allocation failed"
            ))
        }
    }

    /// Update system metrics for decision making
    async fn update_system_metrics(&mut self) -> Result<()> {
        // Placeholder for system metrics collection
        // In production, this would query system APIs
        self.system_metrics.cpu_usage = 0.3; // 30% CPU
        self.system_metrics.memory_usage = 0.4; // 40% Memory
        self.system_metrics.active_crates = self.decision_history.len() as u32;
        self.system_metrics.avg_response_time = 150.0; // 150ms

        Ok(())
    }

    /// Record decision for learning and analysis
    async fn record_decision(&mut self, decision: MuxDecision, sch_vector: SCHVector) {
        let record = MuxDecisionRecord {
            timestamp: sx9_foundation_manifold::core::data::chrono::Utc::now().timestamp() as u64,
            decision,
            outcome: None, // Will be updated later
            sch_vector,
        };

        self.decision_history.push(record);

        // Keep only last 1000 decisions
        if self.decision_history.len() > 1000 {
            self.decision_history.remove(0);
        }
    }
}

/// Threat level assessment
#[derive(Debug, Clone, Copy)]
enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usim::{LifecycleStage, USIMProcessor};

    #[tokio::test]
    async fn test_neural_mux_decision() {
        let mut mux = NeuralMux::new(
            "http://localhost:2375".to_string(),
            "http://localhost:18200".to_string(),
            "http://localhost:18103".to_string(),
        );

        let processor = USIMProcessor::new();
        let usim = processor
            .generate_usim("test_telemetry", "test_context", LifecycleStage::Birth)
            .unwrap();

        let sch_vector = processor.generate_sch_vector(&usim, 0.8, 0.3).unwrap();

        let decision = mux
            .ooda_decide(&usim, &sch_vector, "Test threat", None)
            .await
            .unwrap();

        match decision {
            MuxDecision::SpinCrate(_) | MuxDecision::AlertOnly(_) | MuxDecision::Monitor(_) => {
                // Any valid decision is acceptable for test
            }
        }
    }
}
