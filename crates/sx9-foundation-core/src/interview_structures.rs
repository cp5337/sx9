//! Interview Data Structures
//! 
//! Unified data structures for crate and node interviews that work across
//! all database tiers (Supabase, SurrealDB, Sled, Legion ECS).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unified crate interview structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInterview {
    /// Unique identifier
    pub id: Uuid,
    
    /// Basic crate identity
    pub crate_identity: CrateIdentity,
    
    /// Trivariate hash system
    pub trivariate_hash: TrivariateHash,
    
    /// Crate voice (first-person narrative)
    pub crate_voice: CrateVoice,
    
    /// Dependencies and relationships
    pub dependencies: Vec<CrateDependency>,
    pub operational_relationships: OperationalRelationship,
    
    /// Node applications
    pub node_applications: Vec<NodeApplication>,
    
    /// Tool chain integration
    pub tool_chain_integration: ToolChainIntegration,
    
    /// EEI requirements
    pub eei_requirements: EEIRequirements,
    
    /// MCP integration
    pub mcp_integration: MCPIntegration,
    
    /// GNN and vector database integration
    pub gnn_vector_integration: GNNVectorIntegration,
    
    /// XSD validation
    pub xsd_validation: XSDValidation,
    
    /// Metadata
    pub metadata: InterviewMetadata,
}

/// Unified node interview structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInterview {
    /// Unique identifier
    pub id: Uuid,
    
    /// Node identity
    pub node_identity: NodeIdentity,
    
    /// Trivariate hash system
    pub trivariate_hash: TrivariateHash,
    
    /// Adversary voice (dual perspective)
    pub adversary_voice: AdversaryVoice,
    
    /// Cognigraph data
    pub cognigraph_data: CognigraphData,
    
    /// Operational intelligence
    pub operational_intelligence: OperationalIntelligence,
    
    /// Real world examples
    pub real_world_examples: RealWorldExamples,
    
    /// CTAS 7.0 integration
    pub ctas7_integration: CTAS7Integration,
    
    /// XSD validation
    pub xsd_validation: XSDValidation,
    
    /// Interdiction framework
    pub interdiction_framework: InterdictionFramework,
    
    /// Graph relationships
    pub graph_relationships: GraphRelationships,
    
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    
    /// Metadata
    pub metadata: InterviewMetadata,
}

/// Crate identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateIdentity {
    pub crate_id: String,
    pub title: String,
    pub crate_type: CrateType,
    pub hd4_phase: String,
    pub priority: Priority,
    pub version: String,
    pub description: String,
}

/// Node identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeIdentity {
    pub task_id: String,
    pub title: String,
    pub node_type: NodeType,
    pub hd4_phase: String,
    pub parent_node: Option<String>,
    pub priority: Priority,
    pub classification: String,
    pub ctas_version: String,
    pub shipyard_quality_score: u32,
}

/// Trivariate hash system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrivariateHash {
    /// Semantic Core Hash (16 positions)
    pub sch: String,
    /// Contextual Unique ID (32 positions)
    pub cuid: String,
    /// Standard UUID4
    pub uuid: String,
}

/// Crate voice (first-person narrative)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateVoice {
    pub primary_narrative: String,
    pub crate_identity: String,
    pub capabilities: Vec<Capability>,
    pub limitations: Vec<String>,
    pub supporting_tools: Vec<String>,
}

/// Adversary voice (dual perspective)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversaryVoice {
    pub first_person_narrative: String,
    pub second_person_commands: Vec<String>,
    pub actor_identity: String,
    pub capabilities: Vec<Capability>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub limitations: Vec<String>,
    pub supporting_tools: Vec<String>,
}

/// Capability structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub capability: String,
    pub description: String,
    pub sophistication_level: String,
    pub ctas_counter: Option<String>,
}

/// Vulnerability structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub vulnerability_id: String,
    pub vulnerability_type: String,
    pub description: String,
    pub exploitability: f32,
    pub detection_method: String,
    pub countermeasure: String,
    pub confidence: f32,
}

/// Crate dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateDependency {
    pub crate_id: String,
    pub dependency_type: DependencyType,
    pub version_constraint: String,
    pub purpose: String,
    pub critical: bool,
}

/// Operational relationship information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalRelationship {
    pub predecessor_crates: Vec<String>,
    pub follower_crates: Vec<String>,
    pub trigger_conditions: Vec<String>,
    pub execution_order: i32,
    pub parallel_execution: bool,
}

/// Node application information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeApplication {
    pub task_id: String,
    pub node_title: String,
    pub hd4_phase: String,
    pub application_type: ApplicationType,
    pub capability_match: String,
    pub integration_level: IntegrationLevel,
}

/// Tool chain integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChainIntegration {
    pub input_tool_chains: Vec<String>,
    pub output_tool_chains: Vec<String>,
    pub execution_modes: Vec<String>,
}

/// EEI requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEIRequirements {
    pub essential_elements: Vec<String>,
    pub intelligence_gaps: Vec<String>,
    pub collection_requirements: Vec<String>,
}

/// MCP integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPIntegration {
    pub model_contexts: Vec<String>,
    pub agent_capabilities: Vec<String>,
    pub coordination_requirements: Vec<String>,
}

/// GNN and vector database integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNVectorIntegration {
    pub graph_entities: Vec<String>,
    pub vector_embeddings: Vec<String>,
    pub semantic_relationships: Vec<String>,
}

/// Cognigraph data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognigraphData {
    pub element_symbol: String,
    pub atomic_number: i32,
    pub element_name: String,
    pub period: i32,
    pub group: i32,
    pub block: String,
    pub color_scheme: String,
    pub completion_rate: f32,
    pub technical_difficulty: f32,
    pub human_factor: f32,
    pub ctas_sensing_capability: String,
    pub progressive_disclosure: ProgressiveDisclosure,
}

/// Progressive disclosure levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveDisclosure {
    pub level_0: String,
    pub level_1: String,
    pub level_2: String,
    pub level_3: String,
}

/// Operational intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalIntelligence {
    pub eei_requirements: Vec<EEIRequirement>,
    pub tactics: Vec<String>,
    pub techniques: Vec<String>,
    pub procedures: Vec<String>,
    pub threat_indicators: Vec<String>,
}

/// EEI requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEIRequirement {
    pub eei_id: String,
    pub question: String,
    pub priority: Priority,
    pub collection_method: String,
    pub timeline: String,
}

/// Real world examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealWorldExamples {
    pub historical_precedents: Vec<String>,
    pub emerging_patterns: Vec<String>,
    pub threat_signatures: Vec<String>,
}

/// CTAS 7.0 integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTAS7Integration {
    pub continuous_hunter_triggers: Vec<String>,
    pub child_protection_measures: Vec<String>,
    pub national_security_implications: Vec<String>,
    pub shipyard_compliance: bool,
    pub quality_metrics: QualityMetrics,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub tesla_grade_standard: bool,
    pub foundation_dependency_only: bool,
    pub test_coverage: f32,
    pub security_audit_clean: bool,
}

/// XSD validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSDValidation {
    pub primitive_type: String,
    pub ctas_primitive: String,
    pub stix_mapping: STIXMapping,
    pub rdf_narrative: RDFNarrative,
    pub interdiction_point: bool,
}

/// STIX mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STIXMapping {
    pub technique_id: String,
    pub tactic: String,
    pub data_source: String,
}

/// RDF narrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RDFNarrative {
    pub actor: String,
    pub verb: String,
    pub object: String,
    pub attribute: String,
    pub context: String,
}

/// Interdiction framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterdictionFramework {
    pub interdiction_opportunities: Vec<String>,
    pub countermeasures: Vec<String>,
    pub legal_framework: Vec<String>,
}

/// Graph relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRelationships {
    pub parent_bonds: Vec<String>,
    pub child_bonds: Vec<String>,
    pub sibling_bonds: Vec<String>,
    pub catalyzes: Vec<String>,
    pub requires: Vec<String>,
    pub bond_type: String,
    pub ctas_correlations: Vec<String>,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub technical_resources: Vec<String>,
    pub human_resources: Vec<String>,
    pub environmental_context: Vec<String>,
}

/// Interview metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewMetadata {
    pub version: String,
    pub ctas_framework: String,
    pub created_date: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub review_cycle: String,
    pub distribution: String,
    pub author: String,
    pub reviewer: String,
    pub shipyard_status: String,
}

// Enums

/// Crate types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateType {
    Foundation,
    Core,
    Intelligence,
    Operations,
    Specialized,
    Tools,
    Infrastructure,
}

/// Node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Parent,
    Child,
    Leaf,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Dependency types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Required,
    Optional,
    Development,
}

/// Application types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationType {
    Primary,
    Secondary,
    Support,
}

/// Integration levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationLevel {
    Direct,
    Indirect,
    Optional,
}

// Conversion functions for backward compatibility

impl From<CrateInterview> for serde_json::Value {
    fn from(interview: CrateInterview) -> Self {
        serde_json::to_value(interview).unwrap_or_default()
    }
}

impl From<NodeInterview> for serde_json::Value {
    fn from(interview: NodeInterview) -> Self {
        serde_json::to_value(interview).unwrap_or_default()
    }
}

impl TryFrom<serde_json::Value> for CrateInterview {
    type Error = serde_json::Error;
    
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

impl TryFrom<serde_json::Value> for NodeInterview {
    type Error = serde_json::Error;
    
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

// Default implementations

impl Default for CrateInterview {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            crate_identity: CrateIdentity::default(),
            trivariate_hash: TrivariateHash::default(),
            crate_voice: CrateVoice::default(),
            dependencies: Vec::new(),
            operational_relationships: OperationalRelationship::default(),
            node_applications: Vec::new(),
            tool_chain_integration: ToolChainIntegration::default(),
            eei_requirements: EEIRequirements::default(),
            mcp_integration: MCPIntegration::default(),
            gnn_vector_integration: GNNVectorIntegration::default(),
            xsd_validation: XSDValidation::default(),
            metadata: InterviewMetadata::default(),
        }
    }
}

impl Default for NodeInterview {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            node_identity: NodeIdentity::default(),
            trivariate_hash: TrivariateHash::default(),
            adversary_voice: AdversaryVoice::default(),
            cognigraph_data: CognigraphData::default(),
            operational_intelligence: OperationalIntelligence::default(),
            real_world_examples: RealWorldExamples::default(),
            ctas7_integration: CTAS7Integration::default(),
            xsd_validation: XSDValidation::default(),
            interdiction_framework: InterdictionFramework::default(),
            graph_relationships: GraphRelationships::default(),
            resource_requirements: ResourceRequirements::default(),
            metadata: InterviewMetadata::default(),
        }
    }
}

impl Default for CrateIdentity {
    fn default() -> Self {
        Self {
            crate_id: String::new(),
            title: String::new(),
            crate_type: CrateType::Core,
            hd4_phase: "all_phases".to_string(),
            priority: Priority::Medium,
            version: "1.0.0".to_string(),
            description: String::new(),
        }
    }
}

impl Default for NodeIdentity {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            title: String::new(),
            node_type: NodeType::Leaf,
            hd4_phase: "hunt".to_string(),
            parent_node: None,
            priority: Priority::Medium,
            classification: "unclassified".to_string(),
            ctas_version: "7.0".to_string(),
            shipyard_quality_score: 90,
        }
    }
}

impl Default for TrivariateHash {
    fn default() -> Self {
        Self {
            sch: String::new(),
            cuid: String::new(),
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for CrateVoice {
    fn default() -> Self {
        Self {
            primary_narrative: String::new(),
            crate_identity: "operational_component".to_string(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
            supporting_tools: Vec::new(),
        }
    }
}

impl Default for AdversaryVoice {
    fn default() -> Self {
        Self {
            first_person_narrative: String::new(),
            second_person_commands: Vec::new(),
            actor_identity: String::new(),
            capabilities: Vec::new(),
            vulnerabilities: Vec::new(),
            limitations: Vec::new(),
            supporting_tools: Vec::new(),
        }
    }
}

impl Default for OperationalRelationship {
    fn default() -> Self {
        Self {
            predecessor_crates: Vec::new(),
            follower_crates: Vec::new(),
            trigger_conditions: Vec::new(),
            execution_order: 0,
            parallel_execution: false,
        }
    }
}

impl Default for ToolChainIntegration {
    fn default() -> Self {
        Self {
            input_tool_chains: Vec::new(),
            output_tool_chains: Vec::new(),
            execution_modes: Vec::new(),
        }
    }
}

impl Default for EEIRequirements {
    fn default() -> Self {
        Self {
            essential_elements: Vec::new(),
            intelligence_gaps: Vec::new(),
            collection_requirements: Vec::new(),
        }
    }
}

impl Default for MCPIntegration {
    fn default() -> Self {
        Self {
            model_contexts: Vec::new(),
            agent_capabilities: Vec::new(),
            coordination_requirements: Vec::new(),
        }
    }
}

impl Default for GNNVectorIntegration {
    fn default() -> Self {
        Self {
            graph_entities: Vec::new(),
            vector_embeddings: Vec::new(),
            semantic_relationships: Vec::new(),
        }
    }
}

impl Default for CognigraphData {
    fn default() -> Self {
        Self {
            element_symbol: String::new(),
            atomic_number: 0,
            element_name: String::new(),
            period: 1,
            group: 1,
            block: "intel".to_string(),
            color_scheme: "#6b7280".to_string(),
            completion_rate: 0.0,
            technical_difficulty: 0.0,
            human_factor: 0.0,
            ctas_sensing_capability: String::new(),
            progressive_disclosure: ProgressiveDisclosure::default(),
        }
    }
}

impl Default for ProgressiveDisclosure {
    fn default() -> Self {
        Self {
            level_0: String::new(),
            level_1: String::new(),
            level_2: String::new(),
            level_3: String::new(),
        }
    }
}

impl Default for OperationalIntelligence {
    fn default() -> Self {
        Self {
            eei_requirements: Vec::new(),
            tactics: Vec::new(),
            techniques: Vec::new(),
            procedures: Vec::new(),
            threat_indicators: Vec::new(),
        }
    }
}

impl Default for RealWorldExamples {
    fn default() -> Self {
        Self {
            historical_precedents: Vec::new(),
            emerging_patterns: Vec::new(),
            threat_signatures: Vec::new(),
        }
    }
}

impl Default for CTAS7Integration {
    fn default() -> Self {
        Self {
            continuous_hunter_triggers: Vec::new(),
            child_protection_measures: Vec::new(),
            national_security_implications: Vec::new(),
            shipyard_compliance: true,
            quality_metrics: QualityMetrics::default(),
        }
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            tesla_grade_standard: true,
            foundation_dependency_only: true,
            test_coverage: 0.9,
            security_audit_clean: true,
        }
    }
}

impl Default for XSDValidation {
    fn default() -> Self {
        Self {
            primitive_type: String::new(),
            ctas_primitive: String::new(),
            stix_mapping: STIXMapping::default(),
            rdf_narrative: RDFNarrative::default(),
            interdiction_point: false,
        }
    }
}

impl Default for STIXMapping {
    fn default() -> Self {
        Self {
            technique_id: String::new(),
            tactic: String::new(),
            data_source: String::new(),
        }
    }
}

impl Default for RDFNarrative {
    fn default() -> Self {
        Self {
            actor: String::new(),
            verb: String::new(),
            object: String::new(),
            attribute: String::new(),
            context: String::new(),
        }
    }
}

impl Default for InterdictionFramework {
    fn default() -> Self {
        Self {
            interdiction_opportunities: Vec::new(),
            countermeasures: Vec::new(),
            legal_framework: Vec::new(),
        }
    }
}

impl Default for GraphRelationships {
    fn default() -> Self {
        Self {
            parent_bonds: Vec::new(),
            child_bonds: Vec::new(),
            sibling_bonds: Vec::new(),
            catalyzes: Vec::new(),
            requires: Vec::new(),
            bond_type: "operational".to_string(),
            ctas_correlations: Vec::new(),
        }
    }
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            technical_resources: Vec::new(),
            human_resources: Vec::new(),
            environmental_context: Vec::new(),
        }
    }
}

impl Default for InterviewMetadata {
    fn default() -> Self {
        Self {
            version: "7.0".to_string(),
            ctas_framework: "7.0.1".to_string(),
            created_date: Utc::now(),
            last_modified: Utc::now(),
            review_cycle: "30 days".to_string(),
            distribution: "CTAS Operations Personnel Only".to_string(),
            author: "CTAS 7.0 Automated Interview Generator".to_string(),
            reviewer: "CTAS Intelligence Hub".to_string(),
            shipyard_status: "quality_approved".to_string(),
        }
    }
}

