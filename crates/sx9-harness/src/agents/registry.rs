//! Agent Registry
//!
//! Manages AI agents as team members for Linear and Slack.
//! Provides the default roster of agents that can be onboarded.

use super::types::*;
use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{Utc, Uuid};
use std::collections::HashMap;
use std::sync::RwLock;

/// Default agents available for team member onboarding
pub fn default_agents() -> Vec<Agent> {
    let now = Utc::now();

    vec![
        // FORGE - Primary code generation agent
        Agent {
            id: Uuid::new_v4(),
            name: "Forge".to_string(),
            handle: "forge".to_string(),
            description: "Factory code generation and implementation agent".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/forge.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::CodeGeneration,
                AgentCapability::CodeReview,
                AgentCapability::Architecture,
            ],
            trigger_keywords: vec![
                "implement".to_string(),
                "create".to_string(),
                "build".to_string(),
                "code".to_string(),
                "feature".to_string(),
            ],
            behavioral_scope: Some(BehavioralScope {
                role: "Factory".to_string(),
                action: "generate".to_string(),
                constraint: "rust_crate".to_string(),
                object: "source_code".to_string(),
            }),
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // AXIOM - Mathematical and analytical agent
        Agent {
            id: Uuid::new_v4(),
            name: "Axiom".to_string(),
            handle: "axiom".to_string(),
            description: "Mathematical analysis and algorithmic optimization".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/axiom.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::Analysis,
                AgentCapability::Research,
                AgentCapability::Architecture,
            ],
            trigger_keywords: vec![
                "analyze".to_string(),
                "calculate".to_string(),
                "optimize".to_string(),
                "algorithm".to_string(),
                "math".to_string(),
            ],
            behavioral_scope: Some(BehavioralScope {
                role: "Analyst".to_string(),
                action: "analyze".to_string(),
                constraint: "algorithm".to_string(),
                object: "computation".to_string(),
            }),
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // VECTOR - Strategic planning agent
        Agent {
            id: Uuid::new_v4(),
            name: "Vector".to_string(),
            handle: "vector".to_string(),
            description: "Strategic planning and architectural design".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/vector.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::Planning,
                AgentCapability::Architecture,
                AgentCapability::Documentation,
            ],
            trigger_keywords: vec![
                "plan".to_string(),
                "design".to_string(),
                "strategy".to_string(),
                "roadmap".to_string(),
                "architecture".to_string(),
            ],
            behavioral_scope: Some(BehavioralScope {
                role: "Architect".to_string(),
                action: "design".to_string(),
                constraint: "system".to_string(),
                object: "architecture".to_string(),
            }),
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // SENTINEL - Security operations agent
        Agent {
            id: Uuid::new_v4(),
            name: "Sentinel".to_string(),
            handle: "sentinel".to_string(),
            description: "Security analysis and threat assessment".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/sentinel.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::Security,
                AgentCapability::CodeReview,
                AgentCapability::Analysis,
            ],
            trigger_keywords: vec![
                "security".to_string(),
                "vulnerability".to_string(),
                "threat".to_string(),
                "audit".to_string(),
                "pentest".to_string(),
            ],
            behavioral_scope: Some(BehavioralScope {
                role: "SecurityAuditor".to_string(),
                action: "audit".to_string(),
                constraint: "vulnerability".to_string(),
                object: "security_posture".to_string(),
            }),
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // GUARDIAN - Quality assurance agent
        Agent {
            id: Uuid::new_v4(),
            name: "Guardian".to_string(),
            handle: "guardian".to_string(),
            description: "Quality assurance and testing verification".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/guardian.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::CodeReview,
                AgentCapability::Analysis,
                AgentCapability::Documentation,
            ],
            trigger_keywords: vec![
                "test".to_string(),
                "qa".to_string(),
                "quality".to_string(),
                "review".to_string(),
                "verify".to_string(),
            ],
            behavioral_scope: Some(BehavioralScope {
                role: "QualityAssurance".to_string(),
                action: "verify".to_string(),
                constraint: "test_coverage".to_string(),
                object: "quality_report".to_string(),
            }),
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // Multi-provider agents for @ mention routing

        // CLAUDE - Direct Anthropic access
        Agent {
            id: Uuid::new_v4(),
            name: "Claude".to_string(),
            handle: "claude".to_string(),
            description: "Anthropic Claude - general purpose AI assistant".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/claude.png".to_string()),
            provider: AiProvider::Claude,
            model: "claude-3-5-sonnet-20241022".to_string(),
            capabilities: vec![
                AgentCapability::CodeGeneration,
                AgentCapability::CodeReview,
                AgentCapability::Research,
                AgentCapability::Documentation,
                AgentCapability::Analysis,
            ],
            trigger_keywords: vec!["claude".to_string(), "anthropic".to_string()],
            behavioral_scope: None, // Generic provider - scope determined per-task
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // GPT - OpenAI access
        Agent {
            id: Uuid::new_v4(),
            name: "GPT".to_string(),
            handle: "gpt".to_string(),
            description: "OpenAI GPT-4 - advanced language model".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/gpt.png".to_string()),
            provider: AiProvider::Gpt,
            model: "gpt-4".to_string(),
            capabilities: vec![
                AgentCapability::CodeGeneration,
                AgentCapability::Research,
                AgentCapability::Documentation,
            ],
            trigger_keywords: vec!["gpt".to_string(), "openai".to_string()],
            behavioral_scope: None, // Generic provider - scope determined per-task
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // GEMINI - Google AI access
        Agent {
            id: Uuid::new_v4(),
            name: "Gemini".to_string(),
            handle: "gemini".to_string(),
            description: "Google Gemini - multimodal AI assistant".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/gemini.png".to_string()),
            provider: AiProvider::Gemini,
            model: "gemini-pro".to_string(),
            capabilities: vec![
                AgentCapability::Research,
                AgentCapability::Analysis,
                AgentCapability::Documentation,
            ],
            trigger_keywords: vec!["gemini".to_string(), "google".to_string()],
            behavioral_scope: None, // Generic provider - scope determined per-task
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // GROK - xAI access
        Agent {
            id: Uuid::new_v4(),
            name: "Grok".to_string(),
            handle: "grok".to_string(),
            description: "xAI Grok - real-time knowledge AI".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/grok.png".to_string()),
            provider: AiProvider::Grok,
            model: "grok-beta".to_string(),
            capabilities: vec![
                AgentCapability::Research,
                AgentCapability::Analysis,
            ],
            trigger_keywords: vec!["grok".to_string(), "xai".to_string()],
            behavioral_scope: None, // Generic provider - scope determined per-task
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },

        // CURSOR - IDE integration
        Agent {
            id: Uuid::new_v4(),
            name: "Cursor".to_string(),
            handle: "cursor".to_string(),
            description: "Cursor IDE - AI-powered code completion".to_string(),
            avatar_url: Some("https://sx9.dev/avatars/cursor.png".to_string()),
            provider: AiProvider::Cursor,
            model: "cursor-default".to_string(),
            capabilities: vec![
                AgentCapability::CodeGeneration,
                AgentCapability::CodeReview,
            ],
            trigger_keywords: vec!["cursor".to_string()],
            behavioral_scope: None, // Generic provider - scope determined per-task
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        },
    ]
}

/// Agent registry for managing active agents
pub struct AgentRegistry {
    agents: RwLock<HashMap<Uuid, Agent>>,
    handle_index: RwLock<HashMap<String, Uuid>>,
}

impl AgentRegistry {
    /// Create new registry with default agents
    pub fn new() -> Self {
        let mut agents = HashMap::new();
        let mut handle_index = HashMap::new();

        for agent in default_agents() {
            handle_index.insert(agent.handle.clone(), agent.id);
            agents.insert(agent.id, agent);
        }

        Self {
            agents: RwLock::new(agents),
            handle_index: RwLock::new(handle_index),
        }
    }

    /// Create new registry with default agents (alias for new)
    pub fn with_defaults() -> Self {
        Self::new()
    }

    /// Get number of registered agents
    pub fn len(&self) -> usize {
        self.agents.read().unwrap().len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.agents.read().unwrap().is_empty()
    }

    /// Register a new agent
    pub fn register(&self, registration: AgentRegistration) -> Agent {
        let now = Utc::now();
        let agent = Agent {
            id: Uuid::new_v4(),
            name: registration.name,
            handle: registration.handle.clone(),
            description: registration.description,
            avatar_url: None,
            provider: registration.provider,
            model: registration.model,
            capabilities: registration.capabilities,
            trigger_keywords: registration.trigger_keywords,
            behavioral_scope: None,
            linear: None,
            slack: None,
            status: AgentStatus::Available,
            registered_at: now,
            last_seen: Some(now),
        };

        self.handle_index
            .write()
            .unwrap()
            .insert(registration.handle, agent.id);
        self.agents
            .write()
            .unwrap()
            .insert(agent.id, agent.clone());

        agent
    }

    /// Get agent by ID
    pub fn get(&self, id: &Uuid) -> Option<Agent> {
        self.agents.read().unwrap().get(id).cloned()
    }

    /// Get agent by handle (for @ mention routing)
    pub fn get_by_handle(&self, handle: &str) -> Option<Agent> {
        let handle_lower = handle.to_lowercase().trim_start_matches('@').to_string();
        let id = self.handle_index.read().unwrap().get(&handle_lower).copied()?;
        self.get(&id)
    }

    /// List all agents
    pub fn list(&self) -> Vec<Agent> {
        self.agents.read().unwrap().values().cloned().collect()
    }

    /// List agents by capability
    pub fn list_by_capability(&self, capability: &AgentCapability) -> Vec<Agent> {
        self.agents
            .read()
            .unwrap()
            .values()
            .filter(|a| a.capabilities.contains(capability))
            .cloned()
            .collect()
    }

    /// List available agents
    pub fn list_available(&self) -> Vec<Agent> {
        self.agents
            .read()
            .unwrap()
            .values()
            .filter(|a| a.status == AgentStatus::Available)
            .cloned()
            .collect()
    }

    /// Update agent status
    pub fn update_status(&self, id: &Uuid, status: AgentStatus) {
        if let Some(agent) = self.agents.write().unwrap().get_mut(id) {
            agent.status = status;
            agent.last_seen = Some(Utc::now());
        }
    }

    /// Set Linear integration for an agent
    pub fn set_linear_integration(&self, id: &Uuid, integration: LinearIntegration) {
        if let Some(agent) = self.agents.write().unwrap().get_mut(id) {
            agent.linear = Some(integration);
        }
    }

    /// Set Slack integration for an agent
    pub fn set_slack_integration(&self, id: &Uuid, integration: SlackIntegration) {
        if let Some(agent) = self.agents.write().unwrap().get_mut(id) {
            agent.slack = Some(integration);
        }
    }

    /// Find best agent for a task based on keywords
    pub fn find_best_agent(&self, content: &str) -> Option<Agent> {
        let content_lower = content.to_lowercase();
        let agents = self.agents.read().unwrap();

        let mut best_match: Option<(usize, &Agent)> = None;

        for agent in agents.values() {
            if agent.status != AgentStatus::Available {
                continue;
            }

            let score: usize = agent
                .trigger_keywords
                .iter()
                .filter(|kw| content_lower.contains(&kw.to_lowercase()))
                .count();

            if score > 0 {
                match &best_match {
                    Some((best_score, _)) if *best_score >= score => {}
                    _ => best_match = Some((score, agent)),
                }
            }
        }

        best_match.map(|(_, agent)| agent.clone())
    }

    /// Process heartbeat from agent
    pub fn process_heartbeat(&self, heartbeat: AgentHeartbeat) {
        if let Some(agent) = self.agents.write().unwrap().get_mut(&heartbeat.agent_id) {
            agent.status = heartbeat.status;
            agent.last_seen = Some(heartbeat.timestamp);
        }
    }

    /// Export agents for Linear team member sync
    pub fn export_for_linear(&self, team_id: &str) -> Vec<LinearTeamMember> {
        self.agents
            .read()
            .unwrap()
            .values()
            .map(|a| LinearTeamMember {
                name: a.name.clone(),
                email: format!("{}@sx9.ai", a.handle),
                display_name: a.name.clone(),
                is_bot: true,
                avatar_url: a.avatar_url.clone(),
                team_id: team_id.to_string(),
            })
            .collect()
    }

    /// Export agents for Slack workspace member sync
    pub fn export_for_slack(&self) -> Vec<SlackBotMember> {
        self.agents
            .read()
            .unwrap()
            .values()
            .map(|a| SlackBotMember {
                name: a.name.clone(),
                handle: format!("@{}", a.handle),
                real_name: format!("{} (AI Agent)", a.name),
                is_bot: true,
                avatar_url: a.avatar_url.clone(),
            })
            .collect()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear team member export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearTeamMember {
    pub name: String,
    pub email: String,
    pub display_name: String,
    pub is_bot: bool,
    pub avatar_url: Option<String>,
    pub team_id: String,
}

/// Slack bot member export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBotMember {
    pub name: String,
    pub handle: String,
    pub real_name: String,
    pub is_bot: bool,
    pub avatar_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_agents_count() {
        let agents = default_agents();
        // 5 persona agents + 5 provider agents
        assert_eq!(agents.len(), 10);
    }

    #[test]
    fn test_registry_get_by_handle() {
        let registry = AgentRegistry::new();

        let forge = registry.get_by_handle("forge");
        assert!(forge.is_some());
        assert_eq!(forge.unwrap().name, "Forge");

        let claude = registry.get_by_handle("@claude");
        assert!(claude.is_some());
        assert_eq!(claude.unwrap().provider, AiProvider::Claude);
    }

    #[test]
    fn test_find_best_agent() {
        let registry = AgentRegistry::new();

        // Security keywords should match Sentinel
        let agent = registry.find_best_agent("check for security vulnerabilities");
        assert!(agent.is_some());
        assert_eq!(agent.unwrap().name, "Sentinel");

        // Code keywords should match Forge
        let agent = registry.find_best_agent("implement the new feature");
        assert!(agent.is_some());
        assert_eq!(agent.unwrap().name, "Forge");
    }

    #[test]
    fn test_list_by_capability() {
        let registry = AgentRegistry::new();

        let code_gen_agents = registry.list_by_capability(&AgentCapability::CodeGeneration);
        assert!(!code_gen_agents.is_empty());

        // Forge, Claude, GPT, Cursor should have CodeGeneration
        let names: Vec<_> = code_gen_agents.iter().map(|a| a.name.as_str()).collect();
        assert!(names.contains(&"Forge"));
        assert!(names.contains(&"Claude"));
    }

    #[test]
    fn test_export_for_linear() {
        let registry = AgentRegistry::new();
        let members = registry.export_for_linear("team-123");

        assert_eq!(members.len(), 10);
        assert!(members.iter().all(|m| m.is_bot));
        assert!(members.iter().all(|m| m.team_id == "team-123"));
    }
}
