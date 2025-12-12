//! Interdiction Point Analyzer
//!
//! Analyzes attack chains to find earliest interdiction points
//! (further left = earlier in attack chain = better)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::threat_reaction::glaf_correlation::GLAFClient;
use crate::threat_reaction::recognize::{ATTACKTechnique, DualTrivariateHash, GLAFGraph};

/// Interdiction point in attack chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterdictionPoint {
    pub step: AttackStep,
    pub position: usize,     // Position in attack chain (0 = earliest)
    pub leftness_score: f64, // Higher = further left = better
    pub technique_id: String,
    pub hash: DualTrivariateHash,
    pub unicode_op: char,
    pub interdiction_method: InterdictionMethod,
}

/// Attack step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackStep {
    pub step_id: String,
    pub name: String,
    pub trivariate_hash: DualTrivariateHash,
    pub unicode_operation: char,
    pub step_type: AttackStepType,
}

/// Attack step type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackStepType {
    Reconnaissance,
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    CommandAndControl,
    Exfiltration,
    Impact,
}

/// Interdiction method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterdictionMethod {
    Block,
    Quarantine,
    Alert,
    Redirect,
    Monitor,
}

/// Attack chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackChain {
    pub technique_id: String,
    pub steps: Vec<AttackStep>,
}

/// Interdiction Point Analyzer
pub struct InterdictionPointAnalyzer {
    glaf_client: GLAFClient,
    _attack_chain_analyzer: AttackChainAnalyzer,
}

impl Default for InterdictionPointAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl InterdictionPointAnalyzer {
    pub fn new() -> Self {
        Self {
            glaf_client: GLAFClient::new("http://localhost:8090".to_string()),
            _attack_chain_analyzer: AttackChainAnalyzer::new(),
        }
    }

    /// Analyze attack chain to find earliest interdiction point
    pub async fn analyze_chain(
        &self,
        execution_graph: &GLAFGraph,
    ) -> Result<Vec<InterdictionPoint>> {
        info!("Analyzing attack chain for interdiction points");

        // 1. Extract attack chain from GLAF graph
        let attack_chain = self.extract_attack_chain(execution_graph).await?;

        // 2. Identify potential interdiction points
        let mut interdiction_points = Vec::new();

        for (step_idx, step) in attack_chain.steps.iter().enumerate() {
            // Calculate "leftness" (earlier = higher score)
            let leftness = 1.0 - (step_idx as f64 / attack_chain.steps.len() as f64);

            // Check if step is interdiction-capable
            if self.can_interdict(step).await? {
                interdiction_points.push(InterdictionPoint {
                    step: step.clone(),
                    position: step_idx,
                    leftness_score: leftness,
                    technique_id: attack_chain.technique_id.clone(),
                    hash: step.trivariate_hash.clone(),
                    unicode_op: step.unicode_operation,
                    interdiction_method: self.determine_interdiction_method(step).await?,
                });
            }
        }

        // 3. Rank by leftness (further left = better)
        interdiction_points
            .sort_by(|a, b| b.leftness_score.partial_cmp(&a.leftness_score).unwrap());

        info!("Found {} interdiction points", interdiction_points.len());
        Ok(interdiction_points)
    }

    /// Extract attack chain from GLAF graph
    async fn extract_attack_chain(&self, graph: &GLAFGraph) -> Result<AttackChain> {
        // TODO: Implement actual chain extraction from GLAF graph
        // For now, create a mock chain
        let steps: Vec<AttackStep> = graph
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                AttackStep {
                    step_id: format!("step_{}", idx),
                    name: format!("Attack Step {}", idx + 1),
                    trivariate_hash: node.dual_hash.clone(),
                    unicode_operation: node.unicode_op,
                    step_type: AttackStepType::Execution, // Default
                }
            })
            .collect();

        Ok(AttackChain {
            technique_id: "T1003".to_string(), // Default
            steps,
        })
    }

    /// Find interdiction points for a technique
    pub async fn find_interdiction_points(
        &self,
        technique: &ATTACKTechnique,
    ) -> Result<Vec<InterdictionPoint>> {
        // Query GLAF for technique execution graph
        let execution_graph = self
            .glaf_client
            .query_technique_graph(&technique.technique_id)
            .await?;

        // Analyze chain
        self.analyze_chain(&execution_graph).await
    }

    /// Check if step can be interdicted
    async fn can_interdict(&self, _step: &AttackStep) -> Result<bool> {
        // TODO: Implement actual interdiction capability check
        // For now, allow interdiction for all steps
        Ok(true)
    }

    /// Determine interdiction method for step
    async fn determine_interdiction_method(
        &self,
        _step: &AttackStep,
    ) -> Result<InterdictionMethod> {
        // TODO: Implement actual method determination based on step type
        // For now, default to Block
        Ok(InterdictionMethod::Block)
    }

    /// Calculate leftness score (higher = earlier = better)
    pub fn calculate_leftness_score(&self, point: &InterdictionPoint) -> f64 {
        // Base score from position (earlier = higher)
        let position_score = 1.0 - (point.position as f64 / 100.0);

        // Bonus for hash/Unicode correlation strength
        let correlation_bonus = 0.1; // TODO: Calculate from actual correlation

        // Bonus for technique-specific early detection
        let technique_bonus = 0.05; // TODO: Calculate from technique metadata

        position_score + correlation_bonus + technique_bonus
    }

    /// Get correlation strength for point
    #[allow(dead_code)]
    fn get_correlation_strength(&self, _point: &InterdictionPoint) -> Option<f64> {
        // TODO: Implement actual correlation strength calculation
        Some(0.5)
    }

    /// Get technique bonus for early detection
    #[allow(dead_code)]
    fn get_technique_bonus(&self, _technique_id: &str) -> Option<f64> {
        // TODO: Implement technique-specific bonus calculation
        Some(0.05)
    }
}

/// Attack Chain Analyzer
pub struct AttackChainAnalyzer;

impl Default for AttackChainAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AttackChainAnalyzer {
    pub fn new() -> Self {
        Self
    }
}
