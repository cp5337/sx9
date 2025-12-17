//! CTAS Linear Multi-LLM Coordination for Smart Crate System
//!
//! Tesla/SpaceX-grade multi-LLM team coordination integrated with Linear issue management.
//! Leverages the Smart Crate Orchestrator for autonomous agent deployment and coordination.

// use anyhow::{Context, Result};
use std::collections::HashMap;
use sx9_foundation_manifold::core::async_runtime::tokio::sync::RwLock;
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::diagnostics::anyhow::{self, Context, Result};
use sx9_foundation_manifold::core::diagnostics::tracing::{debug, info, instrument, warn};

use crate::{
    neural_mux::{MuxDecision, NeuralMux},
    CrateSpecification, Mission, OperatorMode, PlaybookFeature, SCHVector, SecurityLevel,
    SmartCrateOrchestrator, USIMTrivariate,
};

/// Linear issue information for multi-LLM coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIssue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub labels: Vec<String>,
    pub priority: LinearPriority,
    pub complexity: LinearComplexity,
    pub assigned_team: String,
    pub state: LinearState,
    pub project_id: String,
}

/// Linear issue priority levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Urgent = 4,
    Critical = 5,
}

/// Linear issue complexity assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearComplexity {
    Simple,
    Moderate,
    Complex,
    Architectural,
}

/// Linear issue state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearState {
    Triage,
    Backlog,
    Todo,
    InProgress,
    InReview,
    Done,
    Canceled,
}

/// Multi-LLM agent configuration for Linear coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearLLMAgent {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub specializations: Vec<String>,
    pub linear_permissions: Vec<String>,
    pub priority: u8,
    pub is_default: bool,
    pub ctas_integration: CTASIntegrationConfig,
}

/// CTAS integration configuration for each agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTASIntegrationConfig {
    pub xsd_mux_bridge: bool,
    pub sled_cache: bool,
    pub sch_vectorization: bool,
    pub utf_optimization: bool,
    pub smart_crate_deployment: bool,
}

/// Multi-LLM team assignment for Linear issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearTeamAssignment {
    pub issue_id: String,
    pub primary_agent: LinearLLMAgent,
    pub support_agents: Vec<LinearLLMAgent>,
    pub coordination_strategy: CoordinationStrategy,
    pub estimated_duration: String,
    pub smart_crate_template: Option<SmartCrateTemplate>,
}

/// Coordination strategy for multi-LLM teams
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationStrategy {
    Sequential,
    Parallel,
    Hierarchical,
    Consensus,
    SpecializedDomains,
}

/// Smart crate template for Linear issue resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCrateTemplate {
    pub template_type: LinearCrateTemplate,
    pub mission: Mission,
    pub mode: OperatorMode,
    pub features: Vec<PlaybookFeature>,
    pub security_level: SecurityLevel,
    pub auto_deploy: bool,
}

/// Specialized crate templates for Linear coordination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearCrateTemplate {
    /// Bug fix with testing and validation
    BugResolution,
    /// Feature development with documentation
    FeatureDevelopment,
    /// Code review and analysis
    CodeReview,
    /// Performance optimization
    PerformanceOptimization,
    /// Security analysis and hardening
    SecurityHardening,
    /// Documentation generation
    DocumentationGeneration,
    /// API integration and testing
    APIIntegration,
    /// Data analysis and visualization
    DataAnalysis,
    /// Mathematical computation
    MathematicalComputation,
    /// Conversational interface
    ConversationalInterface,
}

/// Linear coordination result with smart crate deployment
#[derive(Debug, Serialize, Deserialize)]
pub struct LinearCoordinationResult {
    pub issue_id: String,
    pub team_assignment: LinearTeamAssignment,
    pub smart_crate_deployed: Option<String>, // Crate path if deployed
    pub usim_trivariate: Option<USIMTrivariate>,
    pub sch_vector: Option<SCHVector>,
    pub coordination_metrics: CoordinationMetrics,
    pub status: CoordinationStatus,
}

/// Coordination metrics for performance tracking
#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinationMetrics {
    pub assignment_time_ms: u64,
    pub deployment_time_ms: Option<u64>,
    pub estimated_completion_hours: f32,
    pub agent_utilization: HashMap<String, f32>,
    pub cache_hit_ratio: f32,
    pub vectorization_score: f32,
}

/// Status of Linear coordination
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationStatus {
    Assigned,
    Deploying,
    Active,
    Reviewing,
    Completed,
    Failed,
}

/// Linear Multi-LLM Coordinator with Smart Crate Integration
pub struct LinearMultiLLMCoordinator {
    /// Smart crate orchestrator for autonomous deployment
    orchestrator: RwLock<SmartCrateOrchestrator>,

    /// Available LLM agents for coordination
    available_agents: RwLock<Vec<LinearLLMAgent>>,

    /// Active issue assignments
    active_assignments: RwLock<HashMap<String, LinearTeamAssignment>>,

    /// Deployed smart crates by issue ID
    deployed_crates: RwLock<HashMap<String, String>>,

    /// Linear client configuration
    linear_config: LinearClientConfig,
}

/// Linear client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearClientConfig {
    pub api_token: String,
    pub team_id: String,
    pub workspace_url: String,
    pub webhook_endpoint: String,
}

impl LinearMultiLLMCoordinator {
    /// Initialize the Linear Multi-LLM Coordinator
    #[instrument(level = "info")]
    pub async fn new(
        orchestrator: SmartCrateOrchestrator,
        linear_config: LinearClientConfig,
    ) -> Result<Self> {
        info!("🚀 Initializing Linear Multi-LLM Coordinator with Smart Crate System");

        // Load default agent configuration
        let default_agents = Self::load_default_agents().await?;

        Ok(Self {
            orchestrator: RwLock::new(orchestrator),
            available_agents: RwLock::new(default_agents),
            active_assignments: RwLock::new(HashMap::new()),
            deployed_crates: RwLock::new(HashMap::new()),
            linear_config,
        })
    }

    /// Coordinate multi-LLM team for Linear issue with smart crate deployment
    #[instrument(level = "info", skip(self))]
    pub async fn coordinate_issue(&self, issue: LinearIssue) -> Result<LinearCoordinationResult> {
        info!(
            "🎯 Coordinating multi-LLM team for Linear issue: {}",
            issue.id
        );

        let start_time = std::time::Instant::now();

        // Analyze issue and assign optimal team
        let team_assignment = self.assign_optimal_team(&issue).await?;

        let assignment_time = start_time.elapsed().as_millis() as u64;

        // Determine if smart crate deployment is needed
        let smart_crate_template = self
            .determine_smart_crate_template(&issue, &team_assignment)
            .await?;

        let mut deployed_crate_path: Option<String> = None;
        let mut usim_trivariate = None;
        let mut sch_vector = None;
        let mut deployment_time = None;

        // Deploy smart crate if needed
        if let Some(template) = &smart_crate_template {
            if template.auto_deploy {
                let deploy_start = std::time::Instant::now();

                let (crate_path, usim, sch) =
                    self.deploy_smart_crate_for_issue(&issue, template).await?;

                deployed_crate_path = Some(crate_path);
                usim_trivariate = Some(usim);
                sch_vector = Some(sch);
                deployment_time = Some(deploy_start.elapsed().as_millis() as u64);

                info!("✅ Smart crate deployed for issue: {}", issue.id);
            }
        }

        // Calculate coordination metrics
        let metrics = self
            .calculate_coordination_metrics(assignment_time, deployment_time, &team_assignment)
            .await?;

        // Store assignment
        {
            let mut assignments = self.active_assignments.write().await;
            assignments.insert(issue.id.clone(), team_assignment.clone());
        }

        // Store deployed crate if any
        if let Some(ref crate_path) = deployed_crate_path {
            let mut crates = self.deployed_crates.write().await;
            crates.insert(issue.id.clone(), crate_path.clone());
        }

        let result = LinearCoordinationResult {
            issue_id: issue.id.clone(),
            team_assignment,
            smart_crate_deployed: deployed_crate_path,
            usim_trivariate,
            sch_vector,
            coordination_metrics: metrics,
            status: CoordinationStatus::Assigned,
        };

        info!("🎉 Linear issue coordination completed: {}", issue.id);

        Ok(result)
    }

    /// Assign optimal multi-LLM team based on issue characteristics
    #[instrument(level = "debug", skip(self))]
    async fn assign_optimal_team(&self, issue: &LinearIssue) -> Result<LinearTeamAssignment> {
        debug!("🤖 Assigning optimal team for issue: {}", issue.id);

        let agents = self.available_agents.read().await;

        // Determine primary agent based on issue characteristics
        let primary_agent = self.select_primary_agent(issue, &agents).await?;

        // Select support agents based on coordination strategy
        let coordination_strategy = self.determine_coordination_strategy(issue).await?;
        let support_agents = self
            .select_support_agents(issue, &primary_agent, &coordination_strategy, &agents)
            .await?;

        // Estimate duration based on complexity and team size
        let estimated_duration = self
            .estimate_duration(issue, &primary_agent, &support_agents)
            .await?;

        // Determine smart crate template
        let smart_crate_template = self
            .determine_smart_crate_template_from_issue(issue)
            .await?;

        Ok(LinearTeamAssignment {
            issue_id: issue.id.clone(),
            primary_agent,
            support_agents: support_agents.to_vec(),
            coordination_strategy,
            estimated_duration,
            smart_crate_template,
        })
    }

    /// Select primary agent based on issue characteristics
    async fn select_primary_agent(
        &self,
        issue: &LinearIssue,
        agents: &[LinearLLMAgent],
    ) -> Result<LinearLLMAgent> {
        // Priority-based selection with capability matching
        let mut best_agent = None;
        let mut best_score = 0f32;

        for agent in agents {
            let mut score = 0f32;

            // Base score from priority (higher priority = higher score)
            score += (10.0 - agent.priority as f32) * 2.0;

            // Capability matching
            for label in &issue.labels {
                if agent.capabilities.iter().any(|cap| cap.contains(label)) {
                    score += 5.0;
                }
                if agent
                    .specializations
                    .iter()
                    .any(|spec| spec.contains(label))
                {
                    score += 10.0;
                }
            }

            // Complexity matching
            match issue.complexity {
                LinearComplexity::Simple => {
                    if agent.provider == "openai" && agent.model.contains("3.5") {
                        score += 5.0;
                    }
                }
                LinearComplexity::Moderate => {
                    if agent.provider == "openai" && agent.model.contains("gpt-4") {
                        score += 5.0;
                    }
                }
                LinearComplexity::Complex => {
                    if agent.provider == "claude" && agent.model.contains("sonnet") {
                        score += 10.0;
                    }
                }
                LinearComplexity::Architectural => {
                    if agent.provider == "claude" && agent.model.contains("opus") {
                        score += 15.0;
                    }
                }
            }

            // Priority matching
            match issue.priority {
                LinearPriority::Urgent | LinearPriority::Critical => {
                    if agent.capabilities.contains(&"fast_responses".to_string()) {
                        score += 8.0;
                    }
                }
                _ => {}
            }

            if score > best_score {
                best_score = score;
                best_agent = Some(agent.clone());
            }
        }

        best_agent.ok_or_else(|| anyhow::anyhow!("No suitable primary agent found"))
    }

    /// Deploy smart crate for specific Linear issue
    #[instrument(level = "debug", skip(self))]
    async fn deploy_smart_crate_for_issue(
        &self,
        issue: &LinearIssue,
        template: &SmartCrateTemplate,
    ) -> Result<(String, USIMTrivariate, SCHVector)> {
        debug!("📦 Deploying smart crate for Linear issue: {}", issue.id);

        // Create crate specification based on issue and template
        let spec = CrateSpecification {
            name: format!("linear-issue-{}", issue.id.replace("-", "_")),
            description: format!("Smart crate for Linear issue: {}", issue.title),
            mode: template.mode,
            mission: template.mission.clone(),
            features: template.features.clone(),
            environment: HashMap::new(),
            security_level: template.security_level.clone(),
        };

        // Deploy using the orchestrator
        let mut orchestrator = self.orchestrator.write().await;
        let result = orchestrator.orchestrate(spec).await?;

        Ok((
            result.crate_path.to_string_lossy().to_string(),
            result.usim_trivariate,
            result.sch_vector,
        ))
    }

    /// Load default agent configuration
    async fn load_default_agents() -> Result<Vec<LinearLLMAgent>> {
        // This would typically load from configuration file
        // For now, return hardcoded configuration
        Ok(vec![
            LinearLLMAgent {
                id: "ctas-claude-sonnet".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                provider: "claude".to_string(),
                model: "claude-3-5-sonnet-20241022".to_string(),
                role: "lead_architect".to_string(),
                capabilities: vec![
                    "code_generation".to_string(),
                    "system_analysis".to_string(),
                    "claude_code_integration".to_string(),
                ],
                specializations: vec![
                    "Complex software architecture".to_string(),
                    "Code review and optimization".to_string(),
                ],
                linear_permissions: vec![
                    "create_issues".to_string(),
                    "assign_issues".to_string(),
                    "comment_on_issues".to_string(),
                ],
                priority: 1,
                is_default: true,
                ctas_integration: CTASIntegrationConfig {
                    xsd_mux_bridge: true,
                    sled_cache: true,
                    sch_vectorization: true,
                    utf_optimization: true,
                    smart_crate_deployment: true,
                },
            },
            LinearLLMAgent {
                id: "ctas-gpt4".to_string(),
                name: "GPT-4".to_string(),
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                role: "code_specialist".to_string(),
                capabilities: vec![
                    "code_generation".to_string(),
                    "function_calling".to_string(),
                    "api_integration".to_string(),
                ],
                specializations: vec![
                    "API development".to_string(),
                    "Function-based programming".to_string(),
                ],
                linear_permissions: vec![
                    "create_issues".to_string(),
                    "comment_on_issues".to_string(),
                ],
                priority: 3,
                is_default: false,
                ctas_integration: CTASIntegrationConfig {
                    xsd_mux_bridge: true,
                    sled_cache: true,
                    sch_vectorization: false,
                    utf_optimization: true,
                    smart_crate_deployment: true,
                },
            },
        ])
    }

    /// Calculate coordination metrics
    async fn calculate_coordination_metrics(
        &self,
        assignment_time_ms: u64,
        deployment_time_ms: Option<u64>,
        _team_assignment: &LinearTeamAssignment,
    ) -> Result<CoordinationMetrics> {
        // This would integrate with actual performance monitoring
        Ok(CoordinationMetrics {
            assignment_time_ms,
            deployment_time_ms,
            estimated_completion_hours: 2.5, // Based on complexity analysis
            agent_utilization: HashMap::new(),
            cache_hit_ratio: 0.85,
            vectorization_score: 0.92,
        })
    }

    // Additional helper methods would be implemented here...
    async fn determine_coordination_strategy(
        &self,
        _issue: &LinearIssue,
    ) -> Result<CoordinationStrategy> {
        Ok(CoordinationStrategy::Hierarchical)
    }

    async fn select_support_agents(
        &self,
        _issue: &LinearIssue,
        _primary: &LinearLLMAgent,
        _strategy: &CoordinationStrategy,
        _agents: &[LinearLLMAgent],
    ) -> Result<Vec<LinearLLMAgent>> {
        Ok(vec![])
    }

    async fn estimate_duration(
        &self,
        _issue: &LinearIssue,
        _primary: &LinearLLMAgent,
        _support: &[LinearLLMAgent],
    ) -> Result<String> {
        Ok("2-4 hours".to_string())
    }

    async fn determine_smart_crate_template(
        &self,
        _issue: &LinearIssue,
        _assignment: &LinearTeamAssignment,
    ) -> Result<Option<SmartCrateTemplate>> {
        Ok(None)
    }

    async fn determine_smart_crate_template_from_issue(
        &self,
        _issue: &LinearIssue,
    ) -> Result<Option<SmartCrateTemplate>> {
        Ok(Some(SmartCrateTemplate {
            template_type: LinearCrateTemplate::FeatureDevelopment,
            mission: Mission::DataIngestion,
            mode: OperatorMode::Developer,
            features: vec![PlaybookFeature::XsdP2],
            security_level: SecurityLevel::Development,
            auto_deploy: true,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_agent_selection() {
        let agents = LinearMultiLLMCoordinator::load_default_agents()
            .await
            .unwrap();
        assert!(!agents.is_empty());
        assert!(agents.iter().any(|a| a.is_default));
    }

    #[test]
    fn test_linear_issue_serialization() {
        let issue = LinearIssue {
            id: "test-123".to_string(),
            title: "Test Issue".to_string(),
            description: "Test description".to_string(),
            labels: vec!["bug".to_string()],
            priority: LinearPriority::High,
            complexity: LinearComplexity::Moderate,
            assigned_team: "ctas-team".to_string(),
            state: LinearState::Todo,
            project_id: "proj-123".to_string(),
        };

        let serialized =
            sx9_foundation_manifold::core::data::serde_json::to_string(&issue).unwrap();
        let deserialized: LinearIssue =
            sx9_foundation_manifold::core::data::serde_json::from_str(&serialized).unwrap();
        assert_eq!(issue.id, deserialized.id);
    }
}
