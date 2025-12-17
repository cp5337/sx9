//! ECS Tool Components - Bevy ECS Tool Integration
//!
//! Transforms offensive tools into proper Bevy ECS components for seamless
//! SlotGraph task node integration and real-time execution management.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::slotgraph_task_tool_mapper::HD4Phase;

/// Tool ECS Component - Core tool representation
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct OffSecToolComponent {
    pub tool_id: String,
    pub tool_name: String,
    pub tool_type: ToolType,
    pub hd4_phases: Vec<HD4Phase>,
    pub status: ToolStatus,
    pub capabilities: Vec<ToolCapability>,
    pub resource_requirements: ToolResourceRequirements,
    pub execution_metadata: ToolExecutionMetadata,
}

/// Metasploit-specific ECS Component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MetasploitComponent {
    pub module_path: String,
    pub module_type: MetasploitModuleType,
    pub target_platforms: Vec<String>,
    pub payload_types: Vec<String>,
    pub difficulty_level: DifficultyLevel,
    pub reliability: f32, // 0.0 to 1.0
    pub cve_mappings: Vec<String>,
    pub required_options: HashMap<String, OptionType>,
}

/// Nmap-specific ECS Component  
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NmapComponent {
    pub scan_types: Vec<NmapScanType>,
    pub script_categories: Vec<NmapScriptCategory>,
    pub timing_template: TimingTemplate,
    pub output_formats: Vec<OutputFormat>,
    pub stealth_options: Vec<StealthOption>,
    pub performance_options: PerformanceOptions,
}

/// Tool execution state component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionState {
    pub execution_id: Uuid,
    pub current_phase: ExecutionPhase,
    pub progress: f32, // 0.0 to 1.0
    pub start_time: f64,
    pub estimated_completion: f64,
    pub intermediate_results: Vec<IntermediateResult>,
    pub resource_usage: CurrentResourceUsage,
    pub error_count: u32,
    pub retry_count: u32,
}

/// Tool target assignment component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolTarget {
    pub target_entities: Vec<Entity>,
    pub target_ips: Vec<String>,
    pub target_ports: Vec<u16>,
    pub target_services: HashMap<String, String>,
    pub target_vulnerabilities: Vec<String>,
    pub geographic_constraints: Option<GeographicConstraint>,
}

/// Tool configuration component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfiguration {
    pub config_id: Uuid,
    pub parameters: HashMap<String, ConfigValue>,
    pub environment_variables: HashMap<String, String>,
    pub input_files: Vec<String>,
    pub output_files: Vec<String>,
    pub logging_level: LoggingLevel,
    pub safety_constraints: Vec<SafetyConstraint>,
}

/// Tool results component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolResults {
    pub results_id: Uuid,
    pub success: bool,
    pub execution_time: f64,
    pub data_collected: Vec<CollectedData>,
    pub vulnerabilities_discovered: Vec<DiscoveredVulnerability>,
    pub network_map: Option<NetworkMap>,
    pub access_credentials: Vec<Credential>,
    pub system_access: Vec<SystemAccess>,
    pub intelligence_value: IntelligenceAssessment,
}

/// Tool orchestration component - manages tool chains
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolOrchestration {
    pub orchestration_id: Uuid,
    pub tool_chain: Vec<Entity>, // Ordered sequence of tool entities
    pub current_tool_index: usize,
    pub chain_status: ChainStatus,
    pub dependency_graph: HashMap<Entity, Vec<Entity>>,
    pub failure_policy: FailurePolicy,
    pub rollback_plan: Vec<RollbackAction>,
}

/// Tool availability component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ToolAvailability {
    pub is_available: bool,
    pub current_load: f32, // 0.0 to 1.0
    pub max_concurrent_executions: u32,
    pub current_executions: u32,
    pub maintenance_window: Option<MaintenanceWindow>,
    pub licensing_status: LicensingStatus,
}

// Enums and supporting types

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    NetworkScanner,
    VulnerabilityScanner,
    ExploitFramework,
    PayloadGenerator,
    PostExploitation,
    SocialEngineering,
    WebApplication,
    Wireless,
    Forensics,
    Persistence,
    Lateral,
    Exfiltration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolStatus {
    Ready,
    Busy,
    Executing,
    Failed,
    Maintenance,
    Disabled,
    Updating,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolCapability {
    NetworkReconnaissance,
    ServiceEnumeration,
    VulnerabilityAssessment,
    ExploitExecution,
    PayloadDelivery,
    PrivilegeEscalation,
    LateralMovement,
    DataExfiltration,
    PersistenceEstablishment,
    AntiForensics,
    TrafficAnalysis,
    CredentialHarvesting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResourceRequirements {
    pub cpu_cores: f32,
    pub memory_mb: u64,
    pub storage_mb: u64,
    pub network_bandwidth_mbps: u32,
    pub gpu_required: bool,
    pub external_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionMetadata {
    pub version: String,
    pub installation_path: String,
    pub last_updated: f64,
    pub execution_count: u64,
    pub success_rate: f32,
    pub average_execution_time: f32,
    pub supported_platforms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetasploitModuleType {
    Exploit,
    Payload,
    Auxiliary,
    Post,
    Encoder,
    Nop,
    Evasion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Trivial,
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptionType {
    String,
    Integer,
    Boolean,
    IPAddress,
    Port,
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NmapScanType {
    TcpSyn,
    TcpConnect,
    UdpScan,
    AckScan,
    WindowScan,
    MaimonScan,
    IdleScan,
    FtpBounceScan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NmapScriptCategory {
    Auth,
    Broadcast,
    Brute,
    Default,
    Discovery,
    Dos,
    Exploit,
    External,
    Fuzzer,
    Intrusive,
    Malware,
    Safe,
    Version,
    Vuln,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimingTemplate {
    Paranoid,   // T0
    Sneaky,     // T1
    Polite,     // T2
    Normal,     // T3
    Aggressive, // T4
    Insane,     // T5
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutputFormat {
    Normal,
    XML,
    Grepable,
    ScriptKiddie,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StealthOption {
    DecoyHosts,
    SourcePortSpoofing,
    DataLength,
    IpOptions,
    Ttl,
    SpoofMac,
    BadSum,
    Adler32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptions {
    pub min_hostgroup: u32,
    pub max_hostgroup: u32,
    pub min_parallelism: u32,
    pub max_parallelism: u32,
    pub max_rtt_timeout: u32,
    pub max_retries: u32,
    pub host_timeout: u32,
    pub scan_delay: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionPhase {
    Initializing,
    ConfigurationValidation,
    TargetPreparation,
    Execution,
    ResultProcessing,
    Cleanup,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediateResult {
    pub timestamp: f64,
    pub phase: ExecutionPhase,
    pub data_type: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentResourceUsage {
    pub cpu_percent: f32,
    pub memory_mb: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub disk_reads: u64,
    pub disk_writes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicConstraint {
    pub allowed_regions: Vec<String>,
    pub blocked_countries: Vec<String>,
    pub latitude_range: Option<(f64, f64)>,
    pub longitude_range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoggingLevel {
    Silent,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraint {
    pub constraint_type: String,
    pub constraint_value: ConfigValue,
    pub enforcement_level: EnforcementLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Advisory,
    Warning,
    Blocking,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedData {
    pub data_id: Uuid,
    pub data_type: String,
    pub source_tool: String,
    pub classification: String,
    pub size_bytes: u64,
    pub hash_sha256: String,
    pub collection_timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredVulnerability {
    pub vulnerability_id: Uuid,
    pub cve_id: Option<String>,
    pub cvss_score: f32,
    pub severity: VulnerabilitySeverity,
    pub description: String,
    pub affected_systems: Vec<String>,
    pub exploit_available: bool,
    pub metasploit_modules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMap {
    pub hosts: Vec<DiscoveredHost>,
    pub services: Vec<DiscoveredService>,
    pub network_topology: Vec<NetworkConnection>,
    pub subnets: Vec<Subnet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredHost {
    pub ip_address: String,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub os_fingerprint: Option<String>,
    pub status: HostStatus,
    pub ports: Vec<DiscoveredPort>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HostStatus {
    Up,
    Down,
    Unknown,
    Filtered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPort {
    pub port: u16,
    pub protocol: String,
    pub state: PortState,
    pub service_name: Option<String>,
    pub service_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    Unfiltered,
    OpenFiltered,
    ClosedFiltered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    pub service_name: String,
    pub version: Option<String>,
    pub port: u16,
    pub protocol: String,
    pub banner: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub source_ip: String,
    pub destination_ip: String,
    pub connection_type: ConnectionType,
    pub strength: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Direct,
    Routed,
    Tunneled,
    VPN,
    Wireless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub host_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub credential_id: Uuid,
    pub credential_type: CredentialType,
    pub username: String,
    pub secret: String, // Should be encrypted in real implementation
    pub domain: Option<String>,
    pub source_system: String,
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CredentialType {
    Password,
    Hash,
    Token,
    Certificate,
    PrivateKey,
    ApiKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationStatus {
    Unvalidated,
    Valid,
    Invalid,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAccess {
    pub access_id: Uuid,
    pub system_identifier: String,
    pub access_level: AccessLevel,
    pub access_method: AccessMethod,
    pub persistence_established: bool,
    pub access_timestamp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessLevel {
    User,
    Administrator,
    System,
    Root,
    Domain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessMethod {
    Exploit,
    CredentialReuse,
    BruteForce,
    SocialEngineering,
    PhysicalAccess,
    Supply Chain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceAssessment {
    pub relevance_score: f32, // 0.0 to 1.0
    pub reliability_score: f32, // 0.0 to 1.0
    pub timeliness_score: f32, // 0.0 to 1.0
    pub completeness_score: f32, // 0.0 to 1.0
    pub actionability_score: f32, // 0.0 to 1.0
    pub overall_value: f32, // Calculated from above scores
    pub intelligence_types: Vec<crate::slotgraph_integration::IntelligenceType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChainStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Aborted,
    Retrying,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FailurePolicy {
    StopOnFailure,
    ContinueOnFailure,
    RetryOnFailure,
    RollbackOnFailure,
    SkipFailedTool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackAction {
    pub action_type: String,
    pub target_entity: Option<Entity>,
    pub parameters: HashMap<String, ConfigValue>,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub start_time: f64,
    pub end_time: f64,
    pub maintenance_type: MaintenanceType,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaintenanceType {
    Scheduled,
    Emergency,
    Update,
    Repair,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LicensingStatus {
    Licensed,
    Trial,
    Expired,
    Invalid,
    CommunityEdition,
}

// ECS Bundle for complete tool entity creation
#[derive(Bundle)]
pub struct OffSecToolBundle {
    pub tool: OffSecToolComponent,
    pub execution_state: ToolExecutionState,
    pub target: ToolTarget,
    pub config: ToolConfiguration,
    pub availability: ToolAvailability,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Bundle)]
pub struct MetasploitBundle {
    pub tool: OffSecToolComponent,
    pub metasploit: MetasploitComponent,
    pub execution_state: ToolExecutionState,
    pub target: ToolTarget,
    pub config: ToolConfiguration,
    pub availability: ToolAvailability,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Bundle)]
pub struct NmapBundle {
    pub tool: OffSecToolComponent,
    pub nmap: NmapComponent,
    pub execution_state: ToolExecutionState,
    pub target: ToolTarget,
    pub config: ToolConfiguration,
    pub availability: ToolAvailability,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for PerformanceOptions {
    fn default() -> Self {
        Self {
            min_hostgroup: 30,
            max_hostgroup: 1024,
            min_parallelism: 1,
            max_parallelism: 100,
            max_rtt_timeout: 10000,
            max_retries: 10,
            host_timeout: 900000,
            scan_delay: 0,
        }
    }
}

impl Default for ToolResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_cores: 1.0,
            memory_mb: 512,
            storage_mb: 100,
            network_bandwidth_mbps: 10,
            gpu_required: false,
            external_dependencies: Vec::new(),
        }
    }
}

impl Default for ToolExecutionMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            installation_path: "/usr/bin".to_string(),
            last_updated: 0.0,
            execution_count: 0,
            success_rate: 0.0,
            average_execution_time: 0.0,
            supported_platforms: vec!["linux".to_string(), "macos".to_string()],
        }
    }
}

impl Default for CurrentResourceUsage {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            disk_reads: 0,
            disk_writes: 0,
        }
    }
}