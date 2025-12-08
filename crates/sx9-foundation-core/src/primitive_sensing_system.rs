//! Primitive Sensing System - Node-based environmental sensing and hash-triggered tool execution
//!
//! Implements the Universal Cognigraph B₁-B₁₀ sensing primitives mapped to environmental
//! sensing capabilities (see, smell, hear, touch, taste) that generate hash triggers for tool execution.

use bevy::prelude::*;
use crate::slotgraph_integration::*;
use crate::slotgraph_task_tool_mapper::HD4Phase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Primitive sensing capabilities - the five senses mapped to network/digital sensing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveSense {
    // Visual sensing - network topology, visual indicators, status displays
    See {
        visual_range: f32,
        clarity: f32,
        spectrum: VisualSpectrum,
    },
    
    // Auditory sensing - network traffic, protocol analysis, signal detection
    Hear {
        frequency_range: (f32, f32),
        sensitivity: f32,
        discrimination: f32,
    },
    
    // Olfactory sensing - anomaly detection, pattern recognition, signature analysis
    Smell {
        chemical_sensitivity: f32,
        signature_database: Vec<String>,
        detection_threshold: f32,
    },
    
    // Tactile sensing - network probing, port scanning, service fingerprinting
    Touch {
        pressure_sensitivity: f32,
        texture_analysis: bool,
        response_time: f32,
    },
    
    // Gustatory sensing - data quality assessment, content analysis, payload inspection
    Taste {
        flavor_profiles: Vec<String>,
        palatability_threshold: f32,
        toxicity_detection: bool,
    },
}

/// Visual spectrum for "See" sensing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VisualSpectrum {
    Visible,        // Normal network visibility
    Infrared,       // Heat signatures, performance metrics
    Ultraviolet,    // Hidden protocols, covert channels
    XRay,          // Deep packet inspection, internal structures
    Thermal,       // System load, resource utilization
}

/// Node sensing component - attaches to Universal Node Types
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NodeSensing {
    pub node_id: Uuid,
    pub universal_type: crate::slotgraph_task_tool_mapper::SlotGraphNodeType,
    pub primary_senses: Vec<PrimitiveSense>,
    pub sensing_range: f32,
    pub sensitivity: f32,
    pub environmental_awareness: EnvironmentalAwareness,
    pub sensing_history: Vec<SensingEvent>,
    pub hash_triggers: Vec<HashTrigger>,
}

/// Environmental awareness context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalAwareness {
    pub metoc_sensitive: bool,      // Weather/environmental conditions
    pub traffic_aware: bool,        // Network traffic patterns
    pub illumination_dependent: bool, // Visibility conditions
    pub resource_constrained: bool, // System resource availability
    pub time_sensitive: bool,       // Temporal context awareness
}

/// Sensing event recorded by node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensingEvent {
    pub event_id: Uuid,
    pub timestamp: f64,
    pub sense_type: PrimitiveSense,
    pub stimulus_source: String,
    pub stimulus_strength: f32,
    pub interpretation: StimulusInterpretation,
    pub hash_generated: Option<String>,
}

/// Interpretation of sensed stimulus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StimulusInterpretation {
    Threat {
        severity: ThreatSeverity,
        confidence: f32,
        indicators: Vec<String>,
    },
    Opportunity {
        potential: f32,
        requirements: Vec<String>,
        expected_outcome: String,
    },
    Environmental {
        condition_type: String,
        impact_level: f32,
        duration_estimate: f32,
    },
    Unknown {
        anomaly_score: f32,
        patterns: Vec<String>,
        investigation_priority: f32,
    },
}

/// Threat severity levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Hash trigger generated from sensing events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashTrigger {
    pub trigger_id: Uuid,
    pub blake3_hash: String,
    pub trigger_condition: TriggerCondition,
    pub tool_requirements: ToolRequirements,
    pub execution_parameters: HashMap<String, String>,
    pub priority: HD4Phase,
    pub urgency: f32,
    pub ttl: f32, // Time to live in seconds
}

/// Trigger condition that creates hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    // Threshold-based triggers
    ThresholdExceeded {
        metric: String,
        threshold: f32,
        current_value: f32,
    },
    
    // Pattern-based triggers
    PatternDetected {
        pattern_type: String,
        confidence: f32,
        signature: String,
    },
    
    // Anomaly-based triggers
    AnomalyDetected {
        baseline_deviation: f32,
        anomaly_type: String,
        severity: f32,
    },
    
    // Event-based triggers
    EventSequence {
        events: Vec<String>,
        timing_window: f32,
        correlation_strength: f32,
    },
    
    // Composite triggers
    CompositeTrigger {
        sub_triggers: Vec<HashTrigger>,
        logic_operator: LogicOperator,
        aggregation_method: AggregationMethod,
    },
}

/// Logic operators for composite triggers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicOperator {
    And,
    Or,
    Not,
    Xor,
    Nand,
    Nor,
}

/// Aggregation methods for composite triggers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AggregationMethod {
    Sum,
    Average,
    Maximum,
    Minimum,
    WeightedSum,
    BayesianFusion,
}

/// Tool requirements specified by trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequirements {
    pub required_tools: Vec<String>,
    pub preferred_tools: Vec<String>,
    pub tool_sequence: Option<Vec<String>>,
    pub execution_constraints: ExecutionConstraints,
    pub success_criteria: Vec<String>,
}

/// Execution constraints for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConstraints {
    pub max_execution_time: f32,
    pub resource_limits: ResourceLimits,
    pub environmental_constraints: Vec<String>,
    pub stealth_requirements: StealthRequirements,
    pub safety_parameters: SafetyParameters,
}

/// Resource limits for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_usage: f32,
    pub max_memory_mb: u64,
    pub max_network_bandwidth_mbps: u32,
    pub max_storage_mb: u64,
    pub max_concurrent_operations: u32,
}

/// Stealth requirements for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthRequirements {
    pub detection_avoidance_level: f32, // 0.0 = no concern, 1.0 = maximum stealth
    pub traffic_obfuscation: bool,
    pub timing_randomization: bool,
    pub source_spoofing: bool,
    pub artifact_cleanup: bool,
}

/// Safety parameters for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyParameters {
    pub damage_prevention_level: f32,
    pub reversibility_required: bool,
    pub backup_requirements: Vec<String>,
    pub rollback_procedures: Vec<String>,
    pub emergency_stop_conditions: Vec<String>,
}

/// Sensing system that processes environmental inputs and generates hash triggers
pub struct PrimitiveSensingSystem {
    pub sensing_entities: Vec<Entity>,
    pub environmental_monitors: HashMap<String, EnvironmentalMonitor>,
    pub hash_generator: HashTriggerGenerator,
    pub tool_dispatcher: ToolDispatcher,
}

/// Environmental monitor for specific sensing domains
#[derive(Debug, Clone)]
pub struct EnvironmentalMonitor {
    pub monitor_id: Uuid,
    pub domain: SensingDomain,
    pub active_sensors: Vec<SensorConfig>,
    pub baseline_metrics: HashMap<String, f32>,
    pub current_metrics: HashMap<String, f32>,
    pub anomaly_thresholds: HashMap<String, f32>,
}

/// Sensing domains mapped to tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SensingDomain {
    Network {
        interfaces: Vec<String>,
        protocols: Vec<String>,
        port_ranges: Vec<(u16, u16)>,
    },
    System {
        processes: Vec<String>,
        files: Vec<String>,
        registry_keys: Vec<String>,
    },
    Application {
        web_services: Vec<String>,
        apis: Vec<String>,
        databases: Vec<String>,
    },
    External {
        feeds: Vec<String>,
        apis: Vec<String>,
        sensors: Vec<String>,
    },
}

/// Sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorConfig {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub sampling_rate: f32,
    pub sensitivity: f32,
    pub filter_config: FilterConfig,
}

/// Sensor types for different primitives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SensorType {
    // Network sensors (See, Hear, Touch)
    PacketCapture { interface: String, bpf_filter: String },
    PortScan { target_ranges: Vec<String>, scan_type: String },
    ServiceFingerprint { ports: Vec<u16>, protocols: Vec<String> },
    TrafficAnalysis { flow_timeout: u32, aggregation_window: u32 },
    
    // System sensors (Smell, Taste)
    ProcessMonitor { process_patterns: Vec<String> },
    FileSystemWatch { paths: Vec<String>, events: Vec<String> },
    PerformanceCounter { metrics: Vec<String>, interval: u32 },
    LogAnalysis { log_sources: Vec<String>, patterns: Vec<String> },
    
    // Application sensors (See, Hear, Smell)
    ApiMonitor { endpoints: Vec<String>, response_analysis: bool },
    DatabaseMonitor { connections: Vec<String>, query_analysis: bool },
    WebCrawler { targets: Vec<String>, depth: u32 },
    ContentAnalysis { sources: Vec<String>, nlp_enabled: bool },
}

/// Filter configuration for sensors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub priority_patterns: Vec<String>,
    pub noise_reduction: f32,
}

/// Hash trigger generator - creates Blake3 hashes from sensing events
#[derive(Debug, Clone)]
pub struct HashTriggerGenerator {
    pub generator_id: Uuid,
    pub hash_templates: HashMap<String, HashTemplate>,
    pub trigger_history: Vec<HashTrigger>,
    pub generation_statistics: GenerationStatistics,
}

/// Template for generating consistent hashes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashTemplate {
    pub template_id: String,
    pub input_fields: Vec<String>,
    pub normalization_rules: Vec<NormalizationRule>,
    pub salt: String,
    pub output_format: HashFormat,
}

/// Normalization rules for hash inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NormalizationRule {
    Lowercase,
    Uppercase,
    RemoveWhitespace,
    SortFields,
    RoundNumbers { precision: u32 },
    TimestampRounding { interval: u32 },
}

/// Hash output format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HashFormat {
    Hex,
    Base64,
    Base32,
    Base96, // CTAS native format
}

/// Statistics for hash generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStatistics {
    pub total_generated: u64,
    pub successful_triggers: u64,
    pub failed_triggers: u64,
    pub average_generation_time: f32,
    pub collision_count: u64,
}

/// Tool dispatcher - executes tools based on hash triggers
#[derive(Debug, Clone)]
pub struct ToolDispatcher {
    pub dispatcher_id: Uuid,
    pub active_dispatches: HashMap<String, DispatchRecord>,
    pub tool_registry: HashMap<String, ToolEntity>,
    pub execution_queue: Vec<ExecutionRequest>,
    pub dispatch_statistics: DispatchStatistics,
}

/// Record of tool dispatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchRecord {
    pub dispatch_id: Uuid,
    pub trigger_hash: String,
    pub tool_entity: Entity,
    pub start_time: f64,
    pub expected_completion: f64,
    pub status: DispatchStatus,
    pub results: Option<ExecutionResults>,
}

/// Status of tool dispatch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DispatchStatus {
    Queued,
    Initializing,
    Executing,
    Completing,
    Success,
    Failed,
    Cancelled,
    Timeout,
}

/// Tool entity for dispatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntity {
    pub entity_id: Entity,
    pub tool_name: String,
    pub capabilities: Vec<String>,
    pub resource_requirements: ResourceLimits,
    pub availability_status: ToolAvailabilityStatus,
}

/// Tool availability status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolAvailabilityStatus {
    Available,
    Busy,
    Maintenance,
    Offline,
    Restricted,
}

/// Execution request for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub request_id: Uuid,
    pub trigger_hash: String,
    pub tool_requirements: ToolRequirements,
    pub execution_parameters: HashMap<String, String>,
    pub priority: f32,
    pub deadline: f64,
}

/// Results from tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResults {
    pub success: bool,
    pub data_collected: Vec<CollectedData>,
    pub metrics: ExecutionMetrics,
    pub artifacts: Vec<String>,
    pub recommendations: Vec<String>,
    pub follow_up_hashes: Vec<String>, // New triggers generated from results
}

/// Collected data from tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedData {
    pub data_id: Uuid,
    pub data_type: String,
    pub source_tool: String,
    pub content: String,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

/// Execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub execution_time: f32,
    pub resource_usage: ResourceUsage,
    pub success_rate: f32,
    pub error_count: u32,
    pub performance_score: f32,
}

/// Resource usage during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time: f32,
    pub memory_peak: u64,
    pub network_bytes: u64,
    pub disk_io: u64,
    pub execution_threads: u32,
}

/// Dispatch statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchStatistics {
    pub total_dispatches: u64,
    pub successful_dispatches: u64,
    pub failed_dispatches: u64,
    pub average_dispatch_time: f32,
    pub queue_depth_average: f32,
}

/// Bevy systems for primitive sensing

/// System to update node sensing based on environmental inputs
pub fn primitive_sensing_update_system(
    mut sensing_query: Query<(Entity, &mut NodeSensing)>,
    time: Res<Time>,
) {
    for (entity, mut node_sensing) in sensing_query.iter_mut() {
        // Update sensing based on environmental inputs
        let current_time = time.elapsed_seconds_f64();
        
        // Process each sense type
        for sense in &node_sensing.primary_senses {
            match sense {
                PrimitiveSense::See { visual_range, clarity, spectrum } => {
                    // Process visual sensing - network topology, status indicators
                    let sensing_event = SensingEvent {
                        event_id: Uuid::new_v4(),
                        timestamp: current_time,
                        sense_type: sense.clone(),
                        stimulus_source: "network_topology".to_string(),
                        stimulus_strength: *clarity,
                        interpretation: StimulusInterpretation::Environmental {
                            condition_type: "network_visibility".to_string(),
                            impact_level: 0.7,
                            duration_estimate: 300.0,
                        },
                        hash_generated: None, // Will be generated if conditions met
                    };
                    
                    node_sensing.sensing_history.push(sensing_event);
                }
                
                PrimitiveSense::Hear { frequency_range, sensitivity, discrimination } => {
                    // Process auditory sensing - network traffic, protocols
                    // Implementation would analyze network traffic patterns
                }
                
                PrimitiveSense::Smell { chemical_sensitivity, signature_database, detection_threshold } => {
                    // Process olfactory sensing - anomaly detection, signatures
                    // Implementation would analyze for known threat signatures
                }
                
                PrimitiveSense::Touch { pressure_sensitivity, texture_analysis, response_time } => {
                    // Process tactile sensing - network probing, port scanning
                    // Implementation would trigger network reconnaissance
                }
                
                PrimitiveSense::Taste { flavor_profiles, palatability_threshold, toxicity_detection } => {
                    // Process gustatory sensing - data quality, payload analysis
                    // Implementation would analyze data quality and content
                }
            }
        }
    }
}

/// System to generate hash triggers from sensing events
pub fn hash_trigger_generation_system(
    mut sensing_query: Query<(Entity, &mut NodeSensing)>,
    mut trigger_events: EventWriter<HashTriggerEvent>,
    time: Res<Time>,
) {
    for (entity, mut node_sensing) in sensing_query.iter_mut() {
        // Analyze recent sensing events for trigger conditions
        let recent_events: Vec<&SensingEvent> = node_sensing.sensing_history
            .iter()
            .filter(|event| (time.elapsed_seconds_f64() - event.timestamp) < 60.0) // Last 60 seconds
            .collect();
        
        // Generate hash triggers based on patterns
        for event in &recent_events {
            if should_generate_trigger(event, &node_sensing.environmental_awareness) {
                let hash_trigger = generate_blake3_trigger(event, entity);
                node_sensing.hash_triggers.push(hash_trigger.clone());
                
                // Send trigger event to tool dispatcher
                trigger_events.send(HashTriggerEvent {
                    trigger: hash_trigger,
                    source_entity: entity,
                });
            }
        }
    }
}

/// System to dispatch tools based on hash triggers
pub fn tool_dispatch_system(
    mut trigger_events: EventReader<HashTriggerEvent>,
    tool_query: Query<(Entity, &OffSecToolComponent)>,
    mut commands: Commands,
) {
    for trigger_event in trigger_events.read() {
        let trigger = &trigger_event.trigger;
        
        // Find matching tools for the trigger
        for (tool_entity, tool_component) in tool_query.iter() {
            if tool_matches_requirements(tool_component, &trigger.tool_requirements) {
                // Dispatch tool execution
                commands.spawn(ToolExecutionTask {
                    task_id: Uuid::new_v4(),
                    trigger_hash: trigger.blake3_hash.clone(),
                    tool_entity,
                    execution_parameters: trigger.execution_parameters.clone(),
                    priority: trigger.priority.clone(),
                    deadline: time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                        .unwrap().as_secs_f64() + trigger.ttl as f64,
                });
                
                break; // Use first matching tool
            }
        }
    }
}

// Event types
#[derive(Event)]
pub struct HashTriggerEvent {
    pub trigger: HashTrigger,
    pub source_entity: Entity,
}

// Component for tool execution tasks
#[derive(Component)]
pub struct ToolExecutionTask {
    pub task_id: Uuid,
    pub trigger_hash: String,
    pub tool_entity: Entity,
    pub execution_parameters: HashMap<String, String>,
    pub priority: HD4Phase,
    pub deadline: f64,
}

// Helper functions
fn should_generate_trigger(event: &SensingEvent, awareness: &EnvironmentalAwareness) -> bool {
    // Logic to determine if sensing event should generate a trigger
    match &event.interpretation {
        StimulusInterpretation::Threat { severity, confidence, .. } => {
            *confidence > 0.7 && matches!(severity, ThreatSeverity::High | ThreatSeverity::Critical | ThreatSeverity::Emergency)
        }
        StimulusInterpretation::Opportunity { potential, .. } => {
            *potential > 0.8
        }
        StimulusInterpretation::Unknown { anomaly_score, investigation_priority, .. } => {
            *anomaly_score > 0.6 || *investigation_priority > 0.8
        }
        _ => false,
    }
}

fn generate_blake3_trigger(event: &SensingEvent, entity: Entity) -> HashTrigger {
    // Generate Blake3 hash from event data
    let hash_input = format!("{:?}_{}_{}_{}", 
        event.sense_type, 
        event.stimulus_source, 
        event.timestamp, 
        entity.index()
    );
    
    let hash = HashEngine::new().generate_trivariate_hash(hash_input.as_bytes());
    let blake3_hash = hex::encode(hash.as_bytes());
    
    HashTrigger {
        trigger_id: Uuid::new_v4(),
        blake3_hash,
        trigger_condition: TriggerCondition::ThresholdExceeded {
            metric: "stimulus_strength".to_string(),
            threshold: 0.5,
            current_value: event.stimulus_strength,
        },
        tool_requirements: ToolRequirements {
            required_tools: vec!["nmap".to_string()], // Default to network scanning
            preferred_tools: vec!["masscan".to_string()],
            tool_sequence: None,
            execution_constraints: ExecutionConstraints {
                max_execution_time: 300.0,
                resource_limits: ResourceLimits {
                    max_cpu_usage: 80.0,
                    max_memory_mb: 1024,
                    max_network_bandwidth_mbps: 100,
                    max_storage_mb: 512,
                    max_concurrent_operations: 5,
                },
                environmental_constraints: vec![],
                stealth_requirements: StealthRequirements {
                    detection_avoidance_level: 0.7,
                    traffic_obfuscation: true,
                    timing_randomization: true,
                    source_spoofing: false,
                    artifact_cleanup: true,
                },
                safety_parameters: SafetyParameters {
                    damage_prevention_level: 0.9,
                    reversibility_required: true,
                    backup_requirements: vec![],
                    rollback_procedures: vec![],
                    emergency_stop_conditions: vec!["user_abort".to_string()],
                },
            },
            success_criteria: vec!["network_scan_complete".to_string()],
        },
        execution_parameters: HashMap::new(),
        priority: HD4Phase::Hunt, // Default to Hunt phase
        urgency: event.stimulus_strength,
        ttl: 1800.0, // 30 minutes
    }
}

fn tool_matches_requirements(tool: &OffSecToolComponent, requirements: &ToolRequirements) -> bool {
    // Check if tool matches the trigger requirements
    requirements.required_tools.contains(&tool.tool_name) ||
    requirements.preferred_tools.contains(&tool.tool_name) ||
    tool.capabilities.iter().any(|cap| 
        requirements.required_tools.iter().any(|req| req.contains(&cap.to_string()))
    )
}

impl Default for EnvironmentalAwareness {
    fn default() -> Self {
        Self {
            metoc_sensitive: true,
            traffic_aware: true,
            illumination_dependent: false,
            resource_constrained: true,
            time_sensitive: true,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_usage: 50.0,
            max_memory_mb: 512,
            max_network_bandwidth_mbps: 10,
            max_storage_mb: 256,
            max_concurrent_operations: 3,
        }
    }
}