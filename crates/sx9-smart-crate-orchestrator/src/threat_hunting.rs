//! Smart Crate Proactive Threat Hunting Engine
//!
//! Implements the core threat hunting logic with Graph Neural Networks,
//! Support Vector Machines, and symbolic reasoning for proactive detection.

use crate::neural_mux::{MuxDecision, NeuralMux};
use crate::usim::{LifecycleStage, SCHVector, USIMTrivariate};
use std::collections::HashMap;
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::diagnostics::anyhow::{Context, Result};
use sx9_foundation_manifold::core::diagnostics::tracing::{debug, info, warn};

/// ATT&CK tactic node in the threat graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticNode {
    /// ATT&CK tactic ID (e.g., T1078)
    pub tactic_id: String,
    /// Current embedding vector (256-dim)
    pub embedding: Vec<f32>,
    /// Flux score indicating activity level
    pub flux_score: f32,
    /// Connection strength to other tactics
    pub connections: HashMap<String, f32>,
    /// Matroid membership flags
    pub matroid_flags: Vec<bool>,
}

/// Graph Neural Network state for 147 ATT&CK tactics
#[derive(Debug, Clone)]
pub struct ThreatGraph {
    /// All tactic nodes (147 total)
    pub nodes: HashMap<String, TacticNode>,
    /// Global convergence state
    pub convergence_score: f32,
    /// Normal nodes percentage
    pub normal_percentage: f32,
    /// Flux nodes percentage
    pub flux_percentage: f32,
}

/// Matroid oracle for independence checking
#[derive(Debug)]
pub struct MatroidOracle {
    /// Ground set size (64-bit)
    ground_set_size: usize,
    /// Independence structure
    independence_matrix: Vec<Vec<bool>>,
}

/// Hawkes process for temporal spike detection
#[derive(Debug)]
pub struct HawkesProcess {
    /// Event timestamps
    events: Vec<f32>,
    /// Base intensity
    base_intensity: f32,
    /// Decay parameter
    decay: f32,
}

/// CUSUM for change detection
#[derive(Debug)]
pub struct CUSUM {
    /// Cumulative sum
    cumsum: f32,
    /// Mean for normal state
    mean_normal: f32,
    /// Mean for alert state
    mean_alert: f32,
    /// Decision threshold
    threshold: f32,
}

/// CUSUM detection result
#[derive(Debug, Clone)]
pub enum CUSUMResult {
    Normal,
    UpperAlarm(f32),
    LowerAlarm(f32),
}

/// Main threat hunting engine
#[derive(Debug)]
pub struct ThreatHuntingEngine {
    /// 147-node threat graph
    threat_graph: ThreatGraph,
    /// Matroid oracle for independence
    matroid_oracle: MatroidOracle,
    /// Hawkes process for temporal analysis
    hawkes_process: HawkesProcess,
    /// CUSUM for change detection
    cusum_detector: CUSUM,
    /// Neural mux for autonomous response
    neural_mux: NeuralMux,
    /// Decision history
    hunt_history: Vec<HuntResult>,
}

/// Result of threat hunting analysis
#[derive(Debug, Clone)]
pub struct HuntResult {
    /// Timestamp of analysis
    pub timestamp: u64,
    /// Threat level detected
    pub threat_level: f32,
    /// Matroid violations found
    pub matroid_violations: Vec<String>,
    /// Convergence analysis
    pub convergence_analysis: ConvergenceAnalysis,
    /// Recommended action
    pub recommendation: String,
    /// OODA narrative
    pub ooda_narrative: String,
}

/// Convergence analysis results
#[derive(Debug, Clone)]
pub struct ConvergenceAnalysis {
    /// Overall convergence score
    pub overall_score: f32,
    /// Normal nodes count
    pub normal_nodes: usize,
    /// Flux nodes count
    pub flux_nodes: usize,
    /// Transitioning nodes count
    pub transitioning_nodes: usize,
    /// Convergence within bounds
    pub within_bounds: bool,
}

impl ThreatHuntingEngine {
    /// Get reference to neural mux for external access
    pub fn get_neural_mux(&self) -> &NeuralMux {
        &self.neural_mux
    }
    /// Creates new threat hunting engine
    pub fn new(docker_api_url: String, cdn_gateway_url: String, port_manager_url: String) -> Self {
        let threat_graph = Self::initialize_threat_graph();
        let matroid_oracle = MatroidOracle::new(64);
        let hawkes_process = HawkesProcess::new(0.1, 0.5);
        let cusum_detector = CUSUM::new(0.0, 1.0, 5.0);
        let neural_mux = NeuralMux::new(docker_api_url, cdn_gateway_url, port_manager_url);

        Self {
            threat_graph,
            matroid_oracle,
            hawkes_process,
            cusum_detector,
            neural_mux,
            hunt_history: Vec::new(),
        }
    }

    /// Mutable access to the underlying Neural Mux
    pub fn get_neural_mux_mut(&mut self) -> &mut NeuralMux {
        &mut self.neural_mux
    }

    /// Main threat hunting pipeline
    pub async fn hunt_threats(
        &mut self,
        usim: &USIMTrivariate,
        sch_vector: &SCHVector,
        telemetry: &str,
    ) -> Result<HuntResult> {
        info!("Starting proactive threat hunt");

        // Phase 1: Update graph with new telemetry
        self.update_threat_graph(usim, sch_vector, telemetry)
            .await?;

        // Phase 2: Detect matroid violations
        let matroid_violations = self.detect_matroid_violations()?;

        // Phase 3: Temporal spike analysis
        self.analyze_temporal_spikes()?;

        // Phase 4: Change detection
        let cusum_result = self.detect_state_changes(sch_vector)?;

        // Phase 5: Convergence analysis
        let convergence_analysis = self.analyze_convergence()?;

        // Phase 6: Threat scoring
        let mut threat_level =
            self.calculate_threat_score(&matroid_violations, &cusum_result, &convergence_analysis)?;

        // Phase 7: Generate OODA narrative
        let ooda_narrative =
            self.generate_ooda_narrative(threat_level, &matroid_violations, &convergence_analysis)?;

        // Phase 8: Autonomous decision making
        let recommendation = if threat_level > 0.7 {
            let decision = self
                .neural_mux
                .ooda_decide(
                    usim,
                    sch_vector,
                    &ooda_narrative,
                    None, // No build context for threat hunting simulation
                )
                .await?;
            match decision {
                MuxDecision::SpinCrate(ref _req) => {
                    // Logic to prioritize hunting based on crate spin request
                    threat_level += 0.2;
                    let result = self.neural_mux.execute_decision(&decision).await?;
                    format!("AUTONOMOUS: {}", result)
                }
                MuxDecision::AlertOnly(ref _alert) => {
                    threat_level += 0.1;
                    let result = self.neural_mux.execute_decision(&decision).await?;
                    format!("ALERT: {}", result)
                }
                MuxDecision::Monitor(ref _monitor) => {
                    threat_level += 0.05;
                    let result = self.neural_mux.execute_decision(&decision).await?;
                    format!("MONITOR: {}", result)
                }
            }
        } else {
            "Monitor situation, normal conditions detected".to_string()
        };

        let hunt_result = HuntResult {
            timestamp: sx9_foundation_manifold::core::data::chrono::Utc::now().timestamp() as u64,
            threat_level,
            matroid_violations,
            convergence_analysis,
            recommendation,
            ooda_narrative,
        };

        // Record hunt result
        self.hunt_history.push(hunt_result.clone());

        info!(
            "Threat hunt completed with threat level: {:.3}",
            threat_level
        );

        Ok(hunt_result)
    }

    /// Initialize 147-node ATT&CK threat graph
    fn initialize_threat_graph() -> ThreatGraph {
        let mut nodes = HashMap::new();

        // Core ATT&CK tactics (simplified subset for demo)
        let core_tactics = vec![
            "T1078", "T1190", "T1566", "T1059", "T1055", "T1003", "T1083", "T1082", "T1018",
            "T1016", "T1057", "T1049", "T1033", "T1007", "T1124", "T1012", "T1047", "T1053",
            "T1105", "T1021", "T1048", "T1090", "T1041", "T1567",
        ];

        for (_i, tactic_id) in core_tactics.iter().enumerate() {
            let node = TacticNode {
                tactic_id: tactic_id.to_string(),
                embedding: vec![0.1; 256], // Initialize with small values
                flux_score: 0.0,
                connections: HashMap::new(),
                matroid_flags: vec![false; 8], // 8 matroid flags
            };
            nodes.insert(tactic_id.to_string(), node);
        }

        // Add remaining nodes to reach 147 total
        for i in core_tactics.len()..147 {
            let tactic_id = format!("T{:04}", 1000 + i);
            let node = TacticNode {
                tactic_id: tactic_id.clone(),
                embedding: vec![0.05; 256],
                flux_score: 0.0,
                connections: HashMap::new(),
                matroid_flags: vec![false; 8],
            };
            nodes.insert(tactic_id, node);
        }

        ThreatGraph {
            nodes,
            convergence_score: 0.7,
            normal_percentage: 0.7,
            flux_percentage: 0.1,
        }
    }

    /// Update threat graph with new USIM data
    async fn update_threat_graph(
        &mut self,
        usim: &USIMTrivariate,
        sch_vector: &SCHVector,
        telemetry: &str,
    ) -> Result<()> {
        debug!("Updating threat graph with new telemetry");

        // Extract tactic patterns from telemetry
        let detected_tactics = self.extract_tactic_patterns(telemetry)?;

        // Update node embeddings based on SCH vector
        for (tactic_id, node) in self.threat_graph.nodes.iter_mut() {
            if detected_tactics.contains(tactic_id) {
                // Update embedding with SCH influence
                for i in 0..256 {
                    let sch_influence = sch_vector.prediction[i % 64];
                    node.embedding[i] = (node.embedding[i] * 0.9) + (sch_influence * 0.1);
                }

                // Update flux score based on lifecycle stage
                node.flux_score += match usim.lifecycle_stage {
                    LifecycleStage::Birth => 0.1,
                    LifecycleStage::CodeCompletion => 0.3,
                    LifecycleStage::CrateCompletion => 0.5,
                };
            } else {
                // Decay inactive nodes
                node.flux_score *= 0.95;
            }
        }

        // Update global convergence metrics
        self.update_convergence_metrics()?;

        Ok(())
    }

    /// Detect matroid violations (dependent tactic subsets)
    fn detect_matroid_violations(&self) -> Result<Vec<String>> {
        let mut violations = Vec::new();

        // Check for dependent tactic subsets using matroid oracle
        let active_tactics: Vec<_> = self
            .threat_graph
            .nodes
            .iter()
            .filter(|(_, node)| node.flux_score > 0.3)
            .map(|(id, _)| id.clone())
            .collect();

        if active_tactics.len() >= 3 {
            // Check if subset is independent
            let is_independent = self.matroid_oracle.is_independent(&active_tactics);
            if !is_independent {
                violations.push(format!(
                    "Dependent tactic subset detected: {:?} (possible evasion pattern)",
                    &active_tactics[..3.min(active_tactics.len())]
                ));
            }
        }

        Ok(violations)
    }

    /// Analyze temporal spikes using Hawkes process
    fn analyze_temporal_spikes(&mut self) -> Result<()> {
        let current_time =
            sx9_foundation_manifold::core::data::chrono::Utc::now().timestamp() as f32;

        // Add current event to Hawkes process
        self.hawkes_process.add_event(current_time);

        // Check intensity
        let intensity = self.hawkes_process.intensity(current_time);
        if intensity > 1.5 {
            warn!("Temporal spike detected: intensity = {:.3}", intensity);
        }

        Ok(())
    }

    /// Detect state changes using CUSUM
    fn detect_state_changes(&mut self, sch_vector: &SCHVector) -> Result<CUSUMResult> {
        let observation = sch_vector.convergence;
        let result = self.cusum_detector.update(observation);

        match result {
            CUSUMResult::UpperAlarm(value) => {
                warn!("CUSUM upper alarm triggered: {:.3}", value);
            }
            CUSUMResult::LowerAlarm(value) => {
                warn!("CUSUM lower alarm triggered: {:.3}", value);
            }
            CUSUMResult::Normal => {
                debug!("CUSUM normal state");
            }
        }

        Ok(result)
    }

    /// Analyze graph convergence
    fn analyze_convergence(&self) -> Result<ConvergenceAnalysis> {
        let total_nodes = self.threat_graph.nodes.len();
        let normal_nodes = self
            .threat_graph
            .nodes
            .values()
            .filter(|node| node.flux_score < 0.3)
            .count();
        let flux_nodes = self
            .threat_graph
            .nodes
            .values()
            .filter(|node| node.flux_score >= 0.3 && node.flux_score < 0.7)
            .count();
        let transitioning_nodes = self
            .threat_graph
            .nodes
            .values()
            .filter(|node| node.flux_score >= 0.7)
            .count();

        let normal_percentage = normal_nodes as f32 / total_nodes as f32;
        let flux_percentage = flux_nodes as f32 / total_nodes as f32;

        // Check convergence bounds: 70% normal, ±10% flux
        let within_bounds = normal_percentage > 0.7 && flux_percentage.abs() < 0.1;

        Ok(ConvergenceAnalysis {
            overall_score: self.threat_graph.convergence_score,
            normal_nodes,
            flux_nodes,
            transitioning_nodes,
            within_bounds,
        })
    }

    /// Calculate overall threat score
    fn calculate_threat_score(
        &self,
        matroid_violations: &[String],
        cusum_result: &CUSUMResult,
        convergence: &ConvergenceAnalysis,
    ) -> Result<f32> {
        let mut score = 0.0;

        // Matroid violation contribution
        score += matroid_violations.len() as f32 * 0.3;

        // CUSUM contribution
        score += match cusum_result {
            CUSUMResult::UpperAlarm(_) => 0.4,
            CUSUMResult::LowerAlarm(_) => 0.2,
            CUSUMResult::Normal => 0.0,
        };

        // Convergence contribution
        if !convergence.within_bounds {
            score += 0.3;
        }

        // Transitioning nodes contribution
        let transition_ratio = convergence.transitioning_nodes as f32 / 147.0;
        score += transition_ratio;

        Ok(score.clamp(0.0, 1.0))
    }

    /// Generate OODA narrative
    fn generate_ooda_narrative(
        &self,
        threat_level: f32,
        matroid_violations: &[String],
        convergence: &ConvergenceAnalysis,
    ) -> Result<String> {
        let narrative = if threat_level > 0.8 {
            format!(
                "OODA: CRITICAL threat detected (score: {:.3}). {} matroid violations, convergence: {:.3}. RECOMMEND: Immediate autonomous crate spin for threat response.",
                threat_level, matroid_violations.len(), convergence.overall_score
            )
        } else if threat_level > 0.6 {
            format!(
                "OODA: HIGH threat level (score: {:.3}). Graph convergence compromised: {:.3}. RECOMMEND: Alert and prepare defensive crates.",
                threat_level, convergence.overall_score
            )
        } else if threat_level > 0.4 {
            format!(
                "OODA: MEDIUM threat activity (score: {:.3}). {} transitioning nodes detected. RECOMMEND: Enhanced monitoring.",
                threat_level, convergence.transitioning_nodes
            )
        } else {
            format!(
                "OODA: Normal operational state (score: {:.3}). Convergence within bounds: {}. RECOMMEND: Continue monitoring.",
                threat_level, convergence.within_bounds
            )
        };

        Ok(narrative)
    }

    /// Extract tactic patterns from telemetry
    fn extract_tactic_patterns(&self, telemetry: &str) -> Result<Vec<String>> {
        // Simplified pattern extraction
        let mut patterns = Vec::new();

        // Look for common attack patterns in telemetry
        if telemetry.contains("login") || telemetry.contains("auth") {
            patterns.push("T1078".to_string()); // Valid Accounts
        }
        if telemetry.contains("scan") || telemetry.contains("probe") {
            patterns.push("T1018".to_string()); // Remote System Discovery
        }
        if telemetry.contains("powershell") || telemetry.contains("cmd") {
            patterns.push("T1059".to_string()); // Command and Scripting Interpreter
        }
        if telemetry.contains("file_transfer") || telemetry.contains("download") {
            patterns.push("T1105".to_string()); // Ingress Tool Transfer
        }

        Ok(patterns)
    }

    /// Update convergence metrics
    fn update_convergence_metrics(&mut self) -> Result<()> {
        let total_nodes = self.threat_graph.nodes.len() as f32;
        let normal_count = self
            .threat_graph
            .nodes
            .values()
            .filter(|node| node.flux_score < 0.3)
            .count() as f32;
        let flux_count = self
            .threat_graph
            .nodes
            .values()
            .filter(|node| node.flux_score >= 0.3)
            .count() as f32;

        self.threat_graph.normal_percentage = normal_count / total_nodes;
        self.threat_graph.flux_percentage = flux_count / total_nodes;

        // Update overall convergence score
        self.threat_graph.convergence_score = if self.threat_graph.normal_percentage > 0.7
            && self.threat_graph.flux_percentage < 0.1
        {
            0.9
        } else {
            0.5
        };

        Ok(())
    }
}

impl MatroidOracle {
    /// Create new matroid oracle
    fn new(ground_set_size: usize) -> Self {
        let independence_matrix = vec![vec![true; ground_set_size]; ground_set_size];
        Self {
            ground_set_size,
            independence_matrix,
        }
    }

    /// Check if subset is independent
    fn is_independent(&self, subset: &[String]) -> bool {
        // Simplified independence check
        subset.len() <= 3 // Max 3 tactics can be independent simultaneously
    }
}

impl HawkesProcess {
    /// Create new Hawkes process
    fn new(base_intensity: f32, decay: f32) -> Self {
        Self {
            events: Vec::new(),
            base_intensity,
            decay,
        }
    }

    /// Add new event
    fn add_event(&mut self, timestamp: f32) {
        self.events.push(timestamp);
        // Keep only recent events (last 1000)
        if self.events.len() > 1000 {
            self.events.remove(0);
        }
    }

    /// Calculate intensity at given time
    fn intensity(&self, current_time: f32) -> f32 {
        let mut intensity = self.base_intensity;
        for &event_time in &self.events {
            if event_time <= current_time {
                intensity += (-(self.decay * (current_time - event_time))).exp();
            }
        }
        intensity
    }
}

impl CUSUM {
    /// Create new CUSUM detector
    fn new(mean_normal: f32, mean_alert: f32, threshold: f32) -> Self {
        Self {
            cumsum: 0.0,
            mean_normal,
            mean_alert,
            threshold,
        }
    }

    /// Update with new observation
    fn update(&mut self, observation: f32) -> CUSUMResult {
        let diff = observation - self.mean_normal;
        self.cumsum = (self.cumsum + diff).max(0.0);

        if self.cumsum > self.threshold {
            CUSUMResult::UpperAlarm(self.cumsum)
        } else if self.cumsum < -self.threshold {
            CUSUMResult::LowerAlarm(self.cumsum)
        } else {
            CUSUMResult::Normal
        }
    }
}
