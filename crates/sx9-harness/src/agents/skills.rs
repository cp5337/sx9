//! Skill Registry for Agent Capabilities
//!
//! Formalizes agent skills with:
//! - Input/output JSON schemas for validation
//! - SLO (Service Level Objective) definitions
//! - Prerequisite skill chains
//! - Runtime discovery and composition
//!
//! ## RFC-9141 Alignment
//!
//! "Prompts are assembled, not authored. Variable selection precedes generation."
//!
//! Skills are the atomic units that agents compose into workflows.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use sx9_foundation_core::data::{DateTime, Utc, Uuid};

use super::{Agent, AgentCapability};

// ============================================================================
// SKILL DEFINITION
// ============================================================================

/// A formal skill that an agent can execute.
///
/// Skills are the atomic units of agent capability. They have:
/// - Typed input/output schemas
/// - Performance SLOs
/// - Prerequisite dependencies
/// - Execution constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique skill identifier
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Detailed description
    pub description: String,

    /// Skill category for grouping
    pub category: SkillCategory,

    /// JSON Schema for input validation
    pub input_schema: JsonValue,

    /// JSON Schema for output validation
    pub output_schema: JsonValue,

    /// Service Level Objective
    pub slo: SkillSlo,

    /// Required skills that must execute before this one
    pub prerequisites: Vec<String>,

    /// Skills that can run in parallel with this one
    pub parallelizable_with: Vec<String>,

    /// Tags for discovery
    pub tags: Vec<String>,

    /// Whether skill is enabled
    pub enabled: bool,

    /// Skill version (semver)
    pub version: String,

    /// When skill was registered
    pub registered_at: DateTime<Utc>,
}

/// Skill categories aligned with agent capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillCategory {
    /// Code generation skills
    CodeGeneration,
    /// Code analysis and review
    CodeAnalysis,
    /// Testing and QA
    Testing,
    /// Security scanning
    Security,
    /// Documentation generation
    Documentation,
    /// Research and information gathering
    Research,
    /// Architecture and design
    Architecture,
    /// Infrastructure and DevOps
    Infrastructure,
    /// Data transformation
    DataTransform,
    /// Notification and communication
    Notification,
}

impl From<AgentCapability> for SkillCategory {
    fn from(cap: AgentCapability) -> Self {
        match cap {
            AgentCapability::CodeGeneration => SkillCategory::CodeGeneration,
            AgentCapability::CodeReview => SkillCategory::CodeAnalysis,
            AgentCapability::Architecture => SkillCategory::Architecture,
            AgentCapability::Security => SkillCategory::Security,
            AgentCapability::Research => SkillCategory::Research,
            AgentCapability::Documentation => SkillCategory::Documentation,
            AgentCapability::Analysis => SkillCategory::CodeAnalysis,
            AgentCapability::Planning => SkillCategory::Architecture,
            AgentCapability::Infrastructure => SkillCategory::Infrastructure,
            AgentCapability::Custom(_) => SkillCategory::DataTransform,
        }
    }
}

/// Service Level Objective for skill execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSlo {
    /// Target response time in milliseconds
    pub target_latency_ms: u64,

    /// Maximum allowed latency (timeout)
    pub max_latency_ms: u64,

    /// Expected success rate (0.0 - 1.0)
    pub target_success_rate: f32,

    /// Maximum retries before failure
    pub max_retries: u32,

    /// Retry backoff strategy
    pub retry_strategy: RetryStrategy,
}

impl Default for SkillSlo {
    fn default() -> Self {
        Self {
            target_latency_ms: 5000,
            max_latency_ms: 30000,
            target_success_rate: 0.95,
            max_retries: 3,
            retry_strategy: RetryStrategy::ExponentialBackoff {
                base_ms: 1000,
                max_ms: 10000,
            },
        }
    }
}

/// Retry strategy for failed skill executions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryStrategy {
    /// No retries
    None,
    /// Fixed delay between retries
    FixedDelay { delay_ms: u64 },
    /// Exponential backoff
    ExponentialBackoff { base_ms: u64, max_ms: u64 },
    /// Linear backoff
    LinearBackoff { increment_ms: u64, max_ms: u64 },
}

// ============================================================================
// SKILL EXECUTION
// ============================================================================

/// Input for skill execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInput {
    /// Skill ID to execute
    pub skill_id: String,

    /// Input parameters (validated against skill's input_schema)
    pub params: JsonValue,

    /// Execution context
    pub context: SkillContext,

    /// Correlation ID for tracing
    pub correlation_id: Uuid,
}

/// Execution context for skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillContext {
    /// Agent executing the skill
    pub agent_id: Uuid,

    /// Parent task ID (if part of a chain)
    pub parent_task_id: Option<Uuid>,

    /// Linear issue ID (if task-driven)
    pub linear_issue_id: Option<String>,

    /// Working directory for file operations
    pub working_dir: Option<String>,

    /// Environment variables to inject
    pub env_vars: HashMap<String, String>,

    /// Timeout override (uses SLO if not set)
    pub timeout_ms: Option<u64>,
}

/// Output from skill execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillOutput {
    /// Skill that was executed
    pub skill_id: String,

    /// Execution succeeded
    pub success: bool,

    /// Output data (validated against skill's output_schema)
    pub result: JsonValue,

    /// Artifacts produced (files, code, etc.)
    pub artifacts: Vec<SkillArtifact>,

    /// Execution duration in milliseconds
    pub duration_ms: u64,

    /// Error message if failed
    pub error: Option<String>,

    /// Correlation ID for tracing
    pub correlation_id: Uuid,

    /// Timestamp
    pub completed_at: DateTime<Utc>,
}

/// Artifact produced by skill execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillArtifact {
    /// Artifact name
    pub name: String,

    /// MIME type
    pub mime_type: String,

    /// File path (if written to disk)
    pub path: Option<String>,

    /// Inline content (for small artifacts)
    pub content: Option<String>,

    /// Content hash for verification
    pub sha256: Option<String>,

    /// Size in bytes
    pub size_bytes: u64,
}

// ============================================================================
// SKILL REGISTRY
// ============================================================================

/// Registry of available skills
#[derive(Debug, Default)]
pub struct SkillRegistry {
    /// Registered skills by ID
    skills: HashMap<String, Skill>,

    /// Skills by category for discovery
    by_category: HashMap<SkillCategory, Vec<String>>,

    /// Skills by tag for discovery
    by_tag: HashMap<String, Vec<String>>,

    /// Agent to skill mappings
    agent_skills: HashMap<Uuid, Vec<String>>,
}

impl SkillRegistry {
    /// Create a new skill registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create registry with built-in skills
    pub fn with_builtin_skills() -> Self {
        let mut registry = Self::new();
        registry.register_builtin_skills();
        registry
    }

    /// Register a skill
    pub fn register(&mut self, skill: Skill) -> Result<(), SkillRegistryError> {
        if self.skills.contains_key(&skill.id) {
            return Err(SkillRegistryError::AlreadyExists(skill.id));
        }

        // Validate prerequisites exist
        for prereq in &skill.prerequisites {
            if !self.skills.contains_key(prereq) {
                return Err(SkillRegistryError::MissingPrerequisite {
                    skill: skill.id.clone(),
                    prereq: prereq.clone(),
                });
            }
        }

        // Index by category
        self.by_category
            .entry(skill.category)
            .or_default()
            .push(skill.id.clone());

        // Index by tags
        for tag in &skill.tags {
            self.by_tag
                .entry(tag.clone())
                .or_default()
                .push(skill.id.clone());
        }

        self.skills.insert(skill.id.clone(), skill);
        Ok(())
    }

    /// Get a skill by ID
    pub fn get(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }

    /// Get all skills for a category
    pub fn by_category(&self, category: SkillCategory) -> Vec<&Skill> {
        self.by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.skills.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all skills with a tag
    pub fn by_tag(&self, tag: &str) -> Vec<&Skill> {
        self.by_tag
            .get(tag)
            .map(|ids| ids.iter().filter_map(|id| self.skills.get(id)).collect())
            .unwrap_or_default()
    }

    /// Search skills by keyword
    pub fn search(&self, query: &str) -> Vec<&Skill> {
        let query_lower = query.to_lowercase();
        self.skills
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Assign skills to an agent
    pub fn assign_to_agent(&mut self, agent_id: Uuid, skill_ids: Vec<String>) {
        self.agent_skills.insert(agent_id, skill_ids);
    }

    /// Get skills assigned to an agent
    pub fn agent_skills(&self, agent_id: &Uuid) -> Vec<&Skill> {
        self.agent_skills
            .get(agent_id)
            .map(|ids| ids.iter().filter_map(|id| self.skills.get(id)).collect())
            .unwrap_or_default()
    }

    /// List all registered skills
    pub fn list_all(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }

    /// Validate skill input against schema
    pub fn validate_input(&self, skill_id: &str, input: &JsonValue) -> Result<(), ValidationError> {
        let skill = self.get(skill_id)
            .ok_or_else(|| ValidationError::SkillNotFound(skill_id.to_string()))?;

        // TODO: Use jsonschema crate for full validation
        // For now, just check required fields exist
        if let Some(required) = skill.input_schema.get("required") {
            if let Some(required_fields) = required.as_array() {
                for field in required_fields {
                    if let Some(field_name) = field.as_str() {
                        if input.get(field_name).is_none() {
                            return Err(ValidationError::MissingField(field_name.to_string()));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate skill output against schema
    pub fn validate_output(&self, skill_id: &str, output: &JsonValue) -> Result<(), ValidationError> {
        let skill = self.get(skill_id)
            .ok_or_else(|| ValidationError::SkillNotFound(skill_id.to_string()))?;

        // TODO: Use jsonschema crate for full validation
        if let Some(required) = skill.output_schema.get("required") {
            if let Some(required_fields) = required.as_array() {
                for field in required_fields {
                    if let Some(field_name) = field.as_str() {
                        if output.get(field_name).is_none() {
                            return Err(ValidationError::MissingField(field_name.to_string()));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Register built-in skills
    fn register_builtin_skills(&mut self) {
        let now = Utc::now();

        // Code Generation skill
        let _ = self.register(Skill {
            id: "code.generate".to_string(),
            name: "Generate Code".to_string(),
            description: "Generate source code from a specification".to_string(),
            category: SkillCategory::CodeGeneration,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["language", "specification"],
                "properties": {
                    "language": { "type": "string", "enum": ["rust", "typescript", "python", "go"] },
                    "specification": { "type": "string" },
                    "framework": { "type": "string" },
                    "target_path": { "type": "string" }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["files"],
                "properties": {
                    "files": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "content": { "type": "string" }
                            }
                        }
                    },
                    "summary": { "type": "string" }
                }
            }),
            slo: SkillSlo {
                target_latency_ms: 10000,
                max_latency_ms: 60000,
                target_success_rate: 0.90,
                max_retries: 2,
                retry_strategy: RetryStrategy::ExponentialBackoff {
                    base_ms: 2000,
                    max_ms: 15000,
                },
            },
            prerequisites: vec![],
            parallelizable_with: vec!["docs.generate".to_string()],
            tags: vec!["code".to_string(), "generation".to_string(), "core".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Code Review skill
        let _ = self.register(Skill {
            id: "code.review".to_string(),
            name: "Review Code".to_string(),
            description: "Analyze code for quality, security, and best practices".to_string(),
            category: SkillCategory::CodeAnalysis,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["files"],
                "properties": {
                    "files": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "focus_areas": {
                        "type": "array",
                        "items": { "type": "string", "enum": ["security", "performance", "style", "correctness"] }
                    }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["findings"],
                "properties": {
                    "findings": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "file": { "type": "string" },
                                "line": { "type": "integer" },
                                "severity": { "type": "string" },
                                "message": { "type": "string" },
                                "suggestion": { "type": "string" }
                            }
                        }
                    },
                    "summary": { "type": "string" },
                    "quality_score": { "type": "number" }
                }
            }),
            slo: SkillSlo {
                target_latency_ms: 5000,
                max_latency_ms: 30000,
                target_success_rate: 0.95,
                max_retries: 2,
                retry_strategy: RetryStrategy::ExponentialBackoff {
                    base_ms: 1000,
                    max_ms: 8000,
                },
            },
            prerequisites: vec![],
            parallelizable_with: vec!["security.scan".to_string()],
            tags: vec!["code".to_string(), "review".to_string(), "quality".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Test Generation skill
        let _ = self.register(Skill {
            id: "test.generate".to_string(),
            name: "Generate Tests".to_string(),
            description: "Generate unit and integration tests for code".to_string(),
            category: SkillCategory::Testing,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["source_files"],
                "properties": {
                    "source_files": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "test_framework": { "type": "string" },
                    "coverage_target": { "type": "number" }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["test_files"],
                "properties": {
                    "test_files": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "content": { "type": "string" }
                            }
                        }
                    },
                    "test_count": { "type": "integer" }
                }
            }),
            slo: SkillSlo::default(),
            prerequisites: vec!["code.generate".to_string()],
            parallelizable_with: vec![],
            tags: vec!["test".to_string(), "generation".to_string(), "quality".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Security Scan skill
        let _ = self.register(Skill {
            id: "security.scan".to_string(),
            name: "Security Scan".to_string(),
            description: "Scan code for security vulnerabilities".to_string(),
            category: SkillCategory::Security,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["target_path"],
                "properties": {
                    "target_path": { "type": "string" },
                    "scan_types": {
                        "type": "array",
                        "items": { "type": "string", "enum": ["secrets", "dependencies", "sast", "container"] }
                    }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["findings", "passed"],
                "properties": {
                    "findings": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "type": { "type": "string" },
                                "severity": { "type": "string" },
                                "location": { "type": "string" },
                                "description": { "type": "string" }
                            }
                        }
                    },
                    "passed": { "type": "boolean" },
                    "cato_status": { "type": "string" }
                }
            }),
            slo: SkillSlo {
                target_latency_ms: 15000,
                max_latency_ms: 120000,
                target_success_rate: 0.99,
                max_retries: 1,
                retry_strategy: RetryStrategy::FixedDelay { delay_ms: 5000 },
            },
            prerequisites: vec![],
            parallelizable_with: vec!["code.review".to_string()],
            tags: vec!["security".to_string(), "scan".to_string(), "devsecops".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Documentation Generation skill
        let _ = self.register(Skill {
            id: "docs.generate".to_string(),
            name: "Generate Documentation".to_string(),
            description: "Generate documentation from code and specifications".to_string(),
            category: SkillCategory::Documentation,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["source_path"],
                "properties": {
                    "source_path": { "type": "string" },
                    "doc_format": { "type": "string", "enum": ["markdown", "rustdoc", "jsdoc", "openapi"] },
                    "include_examples": { "type": "boolean" }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["documentation"],
                "properties": {
                    "documentation": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "content": { "type": "string" }
                            }
                        }
                    }
                }
            }),
            slo: SkillSlo::default(),
            prerequisites: vec![],
            parallelizable_with: vec!["code.generate".to_string()],
            tags: vec!["docs".to_string(), "documentation".to_string(), "generation".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Slack Notification skill
        let _ = self.register(Skill {
            id: "notify.slack".to_string(),
            name: "Slack Notification".to_string(),
            description: "Send notifications to Slack channels".to_string(),
            category: SkillCategory::Notification,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["channel", "message"],
                "properties": {
                    "channel": { "type": "string" },
                    "message": { "type": "string" },
                    "blocks": { "type": "array" },
                    "thread_ts": { "type": "string" }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["sent"],
                "properties": {
                    "sent": { "type": "boolean" },
                    "message_ts": { "type": "string" }
                }
            }),
            slo: SkillSlo {
                target_latency_ms: 1000,
                max_latency_ms: 5000,
                target_success_rate: 0.99,
                max_retries: 3,
                retry_strategy: RetryStrategy::ExponentialBackoff {
                    base_ms: 500,
                    max_ms: 3000,
                },
            },
            prerequisites: vec![],
            parallelizable_with: vec![],
            tags: vec!["notification".to_string(), "slack".to_string(), "messaging".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });

        // Linear Issue Creation skill
        let _ = self.register(Skill {
            id: "linear.create_issue".to_string(),
            name: "Create Linear Issue".to_string(),
            description: "Create a new issue in Linear".to_string(),
            category: SkillCategory::Infrastructure,
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["title", "description"],
                "properties": {
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "team_id": { "type": "string" },
                    "parent_id": { "type": "string" },
                    "labels": { "type": "array", "items": { "type": "string" } },
                    "priority": { "type": "integer" }
                }
            }),
            output_schema: serde_json::json!({
                "type": "object",
                "required": ["issue_id"],
                "properties": {
                    "issue_id": { "type": "string" },
                    "url": { "type": "string" }
                }
            }),
            slo: SkillSlo {
                target_latency_ms: 2000,
                max_latency_ms: 10000,
                target_success_rate: 0.99,
                max_retries: 2,
                retry_strategy: RetryStrategy::ExponentialBackoff {
                    base_ms: 1000,
                    max_ms: 5000,
                },
            },
            prerequisites: vec![],
            parallelizable_with: vec![],
            tags: vec!["linear".to_string(), "issue".to_string(), "project-management".to_string()],
            enabled: true,
            version: "1.0.0".to_string(),
            registered_at: now,
        });
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Skill registry errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum SkillRegistryError {
    #[error("Skill already exists: {0}")]
    AlreadyExists(String),

    #[error("Missing prerequisite skill '{prereq}' for skill '{skill}'")]
    MissingPrerequisite { skill: String, prereq: String },

    #[error("Skill not found: {0}")]
    NotFound(String),
}

/// Validation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid field type: {field} expected {expected}")]
    InvalidType { field: String, expected: String },

    #[error("Schema validation failed: {0}")]
    SchemaError(String),
}

// ============================================================================
// SKILL EXECUTOR (Trait)
// ============================================================================

/// Trait for executing skills
#[async_trait::async_trait]
pub trait SkillExecutor: Send + Sync {
    /// Execute a skill with given input
    async fn execute(&self, input: SkillInput) -> Result<SkillOutput, SkillExecutionError>;

    /// Check if this executor can handle a skill
    fn can_execute(&self, skill_id: &str) -> bool;
}

/// Skill execution errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum SkillExecutionError {
    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Input validation failed: {0}")]
    ValidationFailed(String),

    #[error("Execution timeout after {0}ms")]
    Timeout(u64),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_registry_creation() {
        let registry = SkillRegistry::with_builtin_skills();
        assert!(registry.get("code.generate").is_some());
        assert!(registry.get("code.review").is_some());
        assert!(registry.get("security.scan").is_some());
    }

    #[test]
    fn test_skill_by_category() {
        let registry = SkillRegistry::with_builtin_skills();
        let code_gen_skills = registry.by_category(SkillCategory::CodeGeneration);
        assert!(!code_gen_skills.is_empty());
    }

    #[test]
    fn test_skill_search() {
        let registry = SkillRegistry::with_builtin_skills();
        let results = registry.search("security");
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.id == "security.scan"));
    }

    #[test]
    fn test_skill_by_tag() {
        let registry = SkillRegistry::with_builtin_skills();
        let core_skills = registry.by_tag("core");
        assert!(!core_skills.is_empty());
    }

    #[test]
    fn test_validate_input() {
        let registry = SkillRegistry::with_builtin_skills();

        // Valid input
        let valid_input = serde_json::json!({
            "language": "rust",
            "specification": "Create a hello world function"
        });
        assert!(registry.validate_input("code.generate", &valid_input).is_ok());

        // Missing required field
        let invalid_input = serde_json::json!({
            "language": "rust"
        });
        assert!(registry.validate_input("code.generate", &invalid_input).is_err());
    }

    #[test]
    fn test_slo_defaults() {
        let slo = SkillSlo::default();
        assert_eq!(slo.target_latency_ms, 5000);
        assert_eq!(slo.max_retries, 3);
    }
}
