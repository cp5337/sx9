//! CTAS-7 PowerShell Beacon Dissector
//!
//! Advanced PowerShell payload analysis and deobfuscation system
//! specifically designed for C2 beacon analysis with comprehensive
//! detection capabilities for Cobalt Strike, Havoc, and other frameworks.

use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// PowerShell beacon dissector with comprehensive analysis capabilities
#[derive(Debug)]
pub struct PowerShellBeaconDissector {
    deobfuscation_engines: Vec<DeobfuscationEngine>,
    malware_signatures: HashMap<String, MalwareSignature>,
    c2_patterns: HashMap<String, C2Pattern>,
    evasion_detectors: Vec<EvasionDetector>,
}

/// Comprehensive PowerShell analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerShellAnalysisResult {
    pub analysis_id: Uuid,
    pub original_script_hash: String,
    pub obfuscation_analysis: ObfuscationAnalysis,
    pub deobfuscation_results: Vec<DeobfuscationResult>,
    pub c2_framework_detection: Option<C2FrameworkDetection>,
    pub beacon_configuration: Option<BeaconConfiguration>,
    pub malicious_functions: Vec<MaliciousFunction>,
    pub network_indicators: Vec<NetworkIndicator>,
    pub file_operations: Vec<FileOperation>,
    pub registry_operations: Vec<RegistryOperation>,
    pub process_operations: Vec<ProcessOperation>,
    pub privilege_escalation: Vec<PrivilegeEscalation>,
    pub persistence_mechanisms: Vec<PersistenceMechanism>,
    pub evasion_techniques: Vec<EvasionTechnique>,
    pub iocs: Vec<IndicatorOfCompromise>,
    pub mitre_mappings: Vec<MitreMapping>,
    pub risk_assessment: RiskAssessment,
    pub analysis_metadata: AnalysisMetadata,
}

/// Obfuscation analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObfuscationAnalysis {
    pub obfuscation_detected: bool,
    pub obfuscation_confidence: f64,
    pub techniques_detected: Vec<ObfuscationTechnique>,
    pub complexity_score: f64,
    pub entropy_analysis: EntropyAnalysis,
    pub string_analysis: StringAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObfuscationTechnique {
    Base64Encoding,
    XorEncryption,
    StringConcatenation,
    CharacterSubstitution,
    VariableRenaming,
    CommandSplitting,
    InvokeExpression,
    CompressedPayload,
    ReflectiveLoading,
    DynamicInvocation,
    ScriptBlockLogging_Bypass,
    AMSI_Bypass,
    Custom(String),
}

/// Entropy analysis for detecting obfuscation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAnalysis {
    pub overall_entropy: f64,
    pub string_entropy: f64,
    pub variable_entropy: f64,
    pub function_entropy: f64,
    pub entropy_threshold_exceeded: bool,
}

/// String analysis for pattern detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringAnalysis {
    pub total_strings: usize,
    pub encoded_strings: usize,
    pub suspicious_strings: Vec<SuspiciousString>,
    pub base64_candidates: Vec<String>,
    pub hex_candidates: Vec<String>,
    pub url_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousString {
    pub value: String,
    pub suspicious_type: SuspiciousStringType,
    pub confidence: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspiciousStringType {
    EncodedCommand,
    C2_URL,
    CryptoFunction,
    SystemCommand,
    RegistryPath,
    FilePath,
    ProcessName,
    NetworkAddress,
}

/// Deobfuscation result from different engines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeobfuscationResult {
    pub engine_name: String,
    pub technique_used: ObfuscationTechnique,
    pub success: bool,
    pub deobfuscated_code: Option<String>,
    pub confidence: f64,
    pub layers_removed: usize,
    pub artifacts_found: Vec<String>,
}

/// C2 framework detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2FrameworkDetection {
    pub framework: C2Framework,
    pub confidence: f64,
    pub version: Option<String>,
    pub indicators: Vec<String>,
    pub signature_matches: Vec<SignatureMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum C2Framework {
    CobaltStrike,
    Havoc,
    Empire,
    Covenant,
    Metasploit,
    PoshC2,
    Sliver,
    Mythic,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMatch {
    pub signature_name: String,
    pub pattern_matched: String,
    pub confidence: f64,
    pub location: String,
}

/// Beacon configuration extracted from payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconConfiguration {
    pub c2_servers: Vec<String>,
    pub beacon_interval: Option<u64>,
    pub jitter_percentage: Option<f64>,
    pub user_agent: Option<String>,
    pub communication_protocol: Option<String>,
    pub encryption_key: Option<String>,
    pub persistence_method: Option<String>,
    pub injection_technique: Option<String>,
    pub malleable_profile: Option<String>,
}

/// Malicious function detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaliciousFunction {
    pub function_name: String,
    pub function_type: MaliciousFunctionType,
    pub parameters: HashMap<String, String>,
    pub risk_level: RiskLevel,
    pub description: String,
    pub mitre_techniques: Vec<String>,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaliciousFunctionType {
    NetworkConnection,
    FileDownload,
    ProcessInjection,
    MemoryManipulation,
    RegistryModification,
    ServiceManipulation,
    CredentialAccess,
    Persistence,
    DefenseEvasion,
    PrivilegeEscalation,
    Reconnaissance,
    LateralMovement,
}

/// Network indicator from PowerShell analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIndicator {
    pub indicator_type: NetworkIndicatorType,
    pub value: String,
    pub context: String,
    pub confidence: f64,
    pub first_seen: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkIndicatorType {
    IPAddress,
    Domain,
    URL,
    UserAgent,
    Port,
    Protocol,
}

/// File operation detected in script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub operation_type: FileOperationType,
    pub file_path: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub mitre_technique: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperationType {
    Create,
    Read,
    Write,
    Delete,
    Execute,
    Download,
    Upload,
    Copy,
    Move,
    Modify,
}

/// Registry operation detected in script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryOperation {
    pub operation_type: RegistryOperationType,
    pub registry_path: String,
    pub value_name: Option<String>,
    pub value_data: Option<String>,
    pub description: String,
    pub risk_level: RiskLevel,
    pub mitre_technique: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryOperationType {
    CreateKey,
    DeleteKey,
    SetValue,
    GetValue,
    DeleteValue,
    EnumerateKeys,
    EnumerateValues,
}

/// Process operation detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOperation {
    pub operation_type: ProcessOperationType,
    pub process_name: Option<String>,
    pub command_line: Option<String>,
    pub description: String,
    pub risk_level: RiskLevel,
    pub mitre_technique: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessOperationType {
    Create,
    Inject,
    Hollow,
    Kill,
    Suspend,
    Resume,
    ListProcesses,
    GetProcessInfo,
}

/// Privilege escalation technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeEscalation {
    pub technique: String,
    pub method: String,
    pub confidence: f64,
    pub description: String,
    pub mitre_technique: String,
}

/// Persistence mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceMechanism {
    pub mechanism: String,
    pub location: String,
    pub method: String,
    pub confidence: f64,
    pub description: String,
    pub mitre_technique: String,
}

/// Evasion technique detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvasionTechnique {
    pub technique: String,
    pub method: String,
    pub confidence: f64,
    pub description: String,
    pub mitre_technique: String,
}

/// Indicator of compromise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorOfCompromise {
    pub ioc_type: IocType,
    pub value: String,
    pub confidence: f64,
    pub context: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IocType {
    IPAddress,
    Domain,
    URL,
    FileHash,
    FilePath,
    RegistryKey,
    ProcessName,
    Mutex,
    UserAgent,
    EmailAddress,
}

/// MITRE ATT&CK mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMapping {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk: RiskLevel,
    pub risk_score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor: String,
    pub weight: f64,
    pub contribution: f64,
    pub description: String,
}

/// Analysis metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub analyzer_version: String,
    pub analysis_timestamp: SystemTime,
    pub analysis_duration_ms: u64,
    pub engines_used: Vec<String>,
    pub success_rate: f64,
}

/// Deobfuscation engine
#[derive(Debug, Clone)]
pub struct DeobfuscationEngine {
    pub name: String,
    pub techniques: Vec<ObfuscationTechnique>,
    pub patterns: HashMap<String, Regex>,
    pub success_rate: f64,
}

/// Malware signature for detection
#[derive(Debug, Clone)]
pub struct MalwareSignature {
    pub name: String,
    pub framework: C2Framework,
    pub patterns: Vec<String>,
    pub confidence_threshold: f64,
    pub description: String,
}

/// C2 pattern for framework identification
#[derive(Debug, Clone)]
pub struct C2Pattern {
    pub framework: C2Framework,
    pub pattern_type: PatternType,
    pub pattern: String,
    pub confidence_weight: f64,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    FunctionName,
    VariableName,
    StringLiteral,
    RegexPattern,
    APICall,
    NetworkPattern,
}

/// Evasion detector
#[derive(Debug, Clone)]
pub struct EvasionDetector {
    pub name: String,
    pub techniques: Vec<EvasionTechnique>,
    pub patterns: HashMap<String, Regex>,
}

impl PowerShellBeaconDissector {
    /// Create new PowerShell beacon dissector with default engines and signatures
    pub fn new() -> Self {
        let mut dissector = Self {
            deobfuscation_engines: Vec::new(),
            malware_signatures: HashMap::new(),
            c2_patterns: HashMap::new(),
            evasion_detectors: Vec::new(),
        };

        dissector.initialize_engines();
        dissector.load_signatures();
        dissector.load_c2_patterns();
        dissector.load_evasion_detectors();

        dissector
    }

    /// Initialize deobfuscation engines
    fn initialize_engines(&mut self) {
        // Base64 deobfuscation engine
        let mut base64_patterns = HashMap::new();
        base64_patterns.insert(
            "base64_pattern".to_string(),
            Regex::new(r"(?i)[A-Za-z0-9+/]{20,}={0,2}").unwrap(),
        );

        self.deobfuscation_engines.push(DeobfuscationEngine {
            name: "Base64Decoder".to_string(),
            techniques: vec![ObfuscationTechnique::Base64Encoding],
            patterns: base64_patterns,
            success_rate: 0.95,
        });

        // XOR deobfuscation engine
        let mut xor_patterns = HashMap::new();
        xor_patterns.insert(
            "xor_pattern".to_string(),
            Regex::new(r"(?i)-bxor\s+0x[0-9a-f]+").unwrap(),
        );

        self.deobfuscation_engines.push(DeobfuscationEngine {
            name: "XorDecryptor".to_string(),
            techniques: vec![ObfuscationTechnique::XorEncryption],
            patterns: xor_patterns,
            success_rate: 0.80,
        });

        // String concatenation deobfuscator
        let mut concat_patterns = HashMap::new();
        concat_patterns.insert(
            "concat_pattern".to_string(),
            Regex::new(r#"(?i)["'][^"']*["']\s*\+\s*["'][^"']*["']"#).unwrap(),
        );

        self.deobfuscation_engines.push(DeobfuscationEngine {
            name: "StringConcatenationResolver".to_string(),
            techniques: vec![ObfuscationTechnique::StringConcatenation],
            patterns: concat_patterns,
            success_rate: 0.90,
        });
    }

    /// Load malware signatures
    fn load_signatures(&mut self) {
        // Cobalt Strike signatures
        self.malware_signatures.insert(
            "CobaltStrike_Beacon".to_string(),
            MalwareSignature {
                name: "Cobalt Strike Beacon".to_string(),
                framework: C2Framework::CobaltStrike,
                patterns: vec![
                    r"(?i)beacon\.dll".to_string(),
                    r"(?i)rundll32\.exe.*beacon".to_string(),
                    r"(?i)powershell.*-enc.*beacon".to_string(),
                    r"(?i)IEX.*beacon".to_string(),
                ],
                confidence_threshold: 0.7,
                description: "Cobalt Strike beacon detection".to_string(),
            },
        );

        // Havoc framework signatures
        self.malware_signatures.insert(
            "Havoc_Agent".to_string(),
            MalwareSignature {
                name: "Havoc C2 Agent".to_string(),
                framework: C2Framework::Havoc,
                patterns: vec![
                    r"(?i)havoc".to_string(),
                    r"(?i)demon\.exe".to_string(),
                    r"(?i)HavocAgent".to_string(),
                ],
                confidence_threshold: 0.8,
                description: "Havoc C2 framework detection".to_string(),
            },
        );

        // Empire signatures
        self.malware_signatures.insert(
            "Empire_Agent".to_string(),
            MalwareSignature {
                name: "Empire Agent".to_string(),
                framework: C2Framework::Empire,
                patterns: vec![
                    r"(?i)empire".to_string(),
                    r"(?i)powershell.*empire".to_string(),
                    r"(?i)Get-Empire".to_string(),
                ],
                confidence_threshold: 0.75,
                description: "PowerShell Empire detection".to_string(),
            },
        );
    }

    /// Load C2 framework patterns
    fn load_c2_patterns(&mut self) {
        // Cobalt Strike patterns
        self.c2_patterns.insert(
            "cs_function_pattern".to_string(),
            C2Pattern {
                framework: C2Framework::CobaltStrike,
                pattern_type: PatternType::FunctionName,
                pattern: r"(?i)(beacon_\w+|cs_\w+)".to_string(),
                confidence_weight: 0.8,
            },
        );

        self.c2_patterns.insert(
            "cs_api_pattern".to_string(),
            C2Pattern {
                framework: C2Framework::CobaltStrike,
                pattern_type: PatternType::APICall,
                pattern: r"(?i)(VirtualAlloc|WriteProcessMemory|CreateRemoteThread)".to_string(),
                confidence_weight: 0.6,
            },
        );

        // Havoc patterns
        self.c2_patterns.insert(
            "havoc_function_pattern".to_string(),
            C2Pattern {
                framework: C2Framework::Havoc,
                pattern_type: PatternType::FunctionName,
                pattern: r"(?i)(havoc_\w+|demon_\w+)".to_string(),
                confidence_weight: 0.9,
            },
        );
    }

    /// Load evasion detectors
    fn load_evasion_detectors(&mut self) {
        let mut amsi_patterns = HashMap::new();
        amsi_patterns.insert(
            "amsi_bypass".to_string(),
            Regex::new(r"(?i)(amsi|antimalware|scan|interface)").unwrap(),
        );

        self.evasion_detectors.push(EvasionDetector {
            name: "AMSI_Bypass_Detector".to_string(),
            techniques: vec![],
            patterns: amsi_patterns,
        });
    }

    /// Analyze PowerShell script for beacon indicators
    pub async fn analyze_script(&self, script: &[u8]) -> Result<PowerShellAnalysisResult> {
        let analysis_id = Uuid::new_v4();
        let start_time = SystemTime::now();

        info!("🔍 Starting PowerShell beacon analysis: {}", analysis_id);

        // Convert to string
        let script_content = String::from_utf8_lossy(script);
        let script_hash = format!("{:x}", md5::compute(script));

        // Stage 1: Obfuscation analysis
        debug!("📊 Analyzing obfuscation patterns");
        let obfuscation_analysis = self.analyze_obfuscation(&script_content)?;

        // Stage 2: Deobfuscation
        debug!("🔓 Attempting deobfuscation");
        let deobfuscation_results = self.perform_deobfuscation(&script_content).await?;

        // Stage 3: C2 framework detection
        debug!("🎯 Detecting C2 framework");
        let c2_detection = self.detect_c2_framework(&script_content, &deobfuscation_results)?;

        // Stage 4: Extract beacon configuration
        debug!("⚙️ Extracting beacon configuration");
        let beacon_config =
            self.extract_beacon_configuration(&script_content, &deobfuscation_results)?;

        // Stage 5: Analyze malicious functions
        debug!("⚠️ Analyzing malicious functions");
        let malicious_functions = self.analyze_malicious_functions(&script_content)?;

        // Stage 6: Extract network indicators
        debug!("🌐 Extracting network indicators");
        let network_indicators = self.extract_network_indicators(&script_content)?;

        // Stage 7: Analyze file operations
        debug!("📁 Analyzing file operations");
        let file_operations = self.analyze_file_operations(&script_content)?;

        // Stage 8: Analyze registry operations
        debug!("🗃️ Analyzing registry operations");
        let registry_operations = self.analyze_registry_operations(&script_content)?;

        // Stage 9: Analyze process operations
        debug!("🔄 Analyzing process operations");
        let process_operations = self.analyze_process_operations(&script_content)?;

        // Stage 10: Detect evasion techniques
        debug!("🕵️ Detecting evasion techniques");
        let evasion_techniques = self.detect_evasion_techniques(&script_content)?;

        // Stage 11: Extract IoCs
        debug!("🚨 Extracting indicators of compromise");
        let iocs = self.extract_iocs(&script_content, &network_indicators, &file_operations)?;

        // Stage 12: Generate MITRE mappings
        debug!("📋 Generating MITRE ATT&CK mappings");
        let mitre_mappings =
            self.generate_mitre_mappings(&malicious_functions, &evasion_techniques)?;

        // Stage 13: Risk assessment
        debug!("⚖️ Performing risk assessment");
        let risk_assessment =
            self.perform_risk_assessment(&c2_detection, &malicious_functions, &evasion_techniques)?;

        let analysis_duration = start_time.elapsed().unwrap_or_default();

        let result = PowerShellAnalysisResult {
            analysis_id,
            original_script_hash: script_hash,
            obfuscation_analysis,
            deobfuscation_results,
            c2_framework_detection: c2_detection,
            beacon_configuration: beacon_config,
            malicious_functions,
            network_indicators,
            file_operations,
            registry_operations,
            process_operations,
            privilege_escalation: Vec::new(),   // TODO: Implement
            persistence_mechanisms: Vec::new(), // TODO: Implement
            evasion_techniques,
            iocs,
            mitre_mappings,
            risk_assessment,
            analysis_metadata: AnalysisMetadata {
                analyzer_version: "CTAS-7-PowerShell-v1.0".to_string(),
                analysis_timestamp: SystemTime::now(),
                analysis_duration_ms: analysis_duration.as_millis() as u64,
                engines_used: self
                    .deobfuscation_engines
                    .iter()
                    .map(|e| e.name.clone())
                    .collect(),
                success_rate: 0.95, // TODO: Calculate based on actual results
            },
        };

        info!(
            "✅ PowerShell analysis completed: {} ({}ms)",
            analysis_id,
            analysis_duration.as_millis()
        );

        Ok(result)
    }

    /// Analyze script for obfuscation patterns
    fn analyze_obfuscation(&self, script: &str) -> Result<ObfuscationAnalysis> {
        let mut techniques_detected = Vec::new();
        let mut confidence_scores = Vec::new();

        // Check for Base64 encoding
        if Regex::new(r"(?i)[A-Za-z0-9+/]{50,}={0,2}")?.is_match(script) {
            techniques_detected.push(ObfuscationTechnique::Base64Encoding);
            confidence_scores.push(0.9);
        }

        // Check for XOR operations
        if Regex::new(r"(?i)-bxor")?.is_match(script) {
            techniques_detected.push(ObfuscationTechnique::XorEncryption);
            confidence_scores.push(0.8);
        }

        // Check for string concatenation
        if Regex::new(r#"["'][^"']*["']\s*\+\s*["'][^"']*["']"#)?.is_match(script) {
            techniques_detected.push(ObfuscationTechnique::StringConcatenation);
            confidence_scores.push(0.7);
        }

        // Check for Invoke-Expression
        if Regex::new(r"(?i)(IEX|Invoke-Expression)")?.is_match(script) {
            techniques_detected.push(ObfuscationTechnique::InvokeExpression);
            confidence_scores.push(0.85);
        }

        // Calculate entropy
        let entropy = self.calculate_entropy(script);
        let entropy_analysis = EntropyAnalysis {
            overall_entropy: entropy,
            string_entropy: self.calculate_string_entropy(script),
            variable_entropy: self.calculate_variable_entropy(script),
            function_entropy: self.calculate_function_entropy(script),
            entropy_threshold_exceeded: entropy > 4.5,
        };

        // String analysis
        let string_analysis = self.analyze_strings(script)?;

        let obfuscation_detected = !techniques_detected.is_empty() || entropy > 4.5;
        let obfuscation_confidence = if obfuscation_detected {
            confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64
        } else {
            0.0
        };

        let complexity_score = self.calculate_complexity_score(script, &techniques_detected);

        Ok(ObfuscationAnalysis {
            obfuscation_detected,
            obfuscation_confidence,
            techniques_detected,
            complexity_score,
            entropy_analysis,
            string_analysis,
        })
    }

    /// Perform deobfuscation using available engines
    async fn perform_deobfuscation(&self, script: &str) -> Result<Vec<DeobfuscationResult>> {
        let mut results = Vec::new();

        for engine in &self.deobfuscation_engines {
            let result = self.run_deobfuscation_engine(engine, script).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Run a specific deobfuscation engine
    async fn run_deobfuscation_engine(
        &self,
        engine: &DeobfuscationEngine,
        script: &str,
    ) -> Result<DeobfuscationResult> {
        match engine.name.as_str() {
            "Base64Decoder" => self.decode_base64(script).await,
            "XorDecryptor" => self.decrypt_xor(script).await,
            "StringConcatenationResolver" => self.resolve_string_concatenation(script).await,
            _ => Ok(DeobfuscationResult {
                engine_name: engine.name.clone(),
                technique_used: ObfuscationTechnique::Custom("Unknown".to_string()),
                success: false,
                deobfuscated_code: None,
                confidence: 0.0,
                layers_removed: 0,
                artifacts_found: Vec::new(),
            }),
        }
    }

    /// Decode Base64 encoded content
    async fn decode_base64(&self, script: &str) -> Result<DeobfuscationResult> {
        let base64_regex = Regex::new(r"(?i)[A-Za-z0-9+/]{20,}={0,2}")?;
        let mut decoded_parts = Vec::new();
        let mut artifacts = Vec::new();

        for capture in base64_regex.find_iter(script) {
            let encoded = capture.as_str();
            if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(encoded) {
                if let Ok(decoded_string) = String::from_utf8(decoded_bytes) {
                    // Check if decoded content looks like PowerShell
                    if decoded_string.contains("powershell")
                        || decoded_string.contains("IEX")
                        || decoded_string.contains("Invoke")
                    {
                        decoded_parts.push(decoded_string.clone());
                        artifacts.push(format!("Decoded Base64: {}", encoded[..20].to_string()));
                    }
                }
            }
        }

        let success = !decoded_parts.is_empty();
        let deobfuscated_code = if success {
            Some(decoded_parts.join("\n"))
        } else {
            None
        };

        Ok(DeobfuscationResult {
            engine_name: "Base64Decoder".to_string(),
            technique_used: ObfuscationTechnique::Base64Encoding,
            success,
            deobfuscated_code,
            confidence: if success { 0.9 } else { 0.0 },
            layers_removed: decoded_parts.len(),
            artifacts_found: artifacts,
        })
    }

    /// Decrypt XOR encoded content
    async fn decrypt_xor(&self, script: &str) -> Result<DeobfuscationResult> {
        // Simplified XOR decryption - in practice would be more sophisticated
        let xor_regex = Regex::new(r"(?i)-bxor\s+(0x[0-9a-f]+|\d+)")?;
        let mut artifacts = Vec::new();

        for capture in xor_regex.captures_iter(script) {
            if let Some(key_match) = capture.get(1) {
                artifacts.push(format!("XOR key found: {}", key_match.as_str()));
            }
        }

        Ok(DeobfuscationResult {
            engine_name: "XorDecryptor".to_string(),
            technique_used: ObfuscationTechnique::XorEncryption,
            success: !artifacts.is_empty(),
            deobfuscated_code: None, // Would implement actual decryption
            confidence: if !artifacts.is_empty() { 0.7 } else { 0.0 },
            layers_removed: artifacts.len(),
            artifacts_found: artifacts,
        })
    }

    /// Resolve string concatenation
    async fn resolve_string_concatenation(&self, script: &str) -> Result<DeobfuscationResult> {
        let concat_regex = Regex::new(r#"["']([^"']*)["']\s*\+\s*["']([^"']*)["']"#)?;
        let mut resolved_strings = Vec::new();
        let mut artifacts = Vec::new();

        for capture in concat_regex.captures_iter(script) {
            if let (Some(part1), Some(part2)) = (capture.get(1), capture.get(2)) {
                let resolved = format!("{}{}", part1.as_str(), part2.as_str());
                resolved_strings.push(resolved.clone());
                artifacts.push(format!("Concatenated: {}", resolved));
            }
        }

        let success = !resolved_strings.is_empty();

        Ok(DeobfuscationResult {
            engine_name: "StringConcatenationResolver".to_string(),
            technique_used: ObfuscationTechnique::StringConcatenation,
            success,
            deobfuscated_code: if success {
                Some(resolved_strings.join("\n"))
            } else {
                None
            },
            confidence: if success { 0.8 } else { 0.0 },
            layers_removed: resolved_strings.len(),
            artifacts_found: artifacts,
        })
    }

    /// Detect C2 framework based on patterns
    fn detect_c2_framework(
        &self,
        script: &str,
        deob_results: &[DeobfuscationResult],
    ) -> Result<Option<C2FrameworkDetection>> {
        let mut framework_scores: HashMap<C2Framework, f64> = HashMap::new();
        let mut all_indicators = Vec::new();
        let mut signature_matches = Vec::new();

        // Check original script
        self.check_framework_patterns(script, &mut framework_scores, &mut all_indicators);

        // Check deobfuscated content
        for result in deob_results {
            if let Some(deob_code) = &result.deobfuscated_code {
                self.check_framework_patterns(
                    deob_code,
                    &mut framework_scores,
                    &mut all_indicators,
                );
            }
        }

        // Check against signatures
        for (sig_name, signature) in &self.malware_signatures {
            let mut match_count = 0;
            for pattern in &signature.patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(script) {
                        match_count += 1;
                        signature_matches.push(SignatureMatch {
                            signature_name: sig_name.clone(),
                            pattern_matched: pattern.clone(),
                            confidence: 0.8,
                            location: "main_script".to_string(),
                        });
                    }
                }
            }

            if match_count > 0 {
                let confidence = match_count as f64 / signature.patterns.len() as f64;
                if confidence >= signature.confidence_threshold {
                    *framework_scores
                        .entry(signature.framework.clone())
                        .or_insert(0.0) += confidence;
                }
            }
        }

        // Determine best match
        if let Some((framework, &confidence)) = framework_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        {
            if confidence > 0.5 {
                return Ok(Some(C2FrameworkDetection {
                    framework: framework.clone(),
                    confidence,
                    version: None, // Could be enhanced to detect versions
                    indicators: all_indicators,
                    signature_matches,
                }));
            }
        }

        Ok(None)
    }

    /// Check script against C2 framework patterns
    fn check_framework_patterns(
        &self,
        script: &str,
        scores: &mut HashMap<C2Framework, f64>,
        indicators: &mut Vec<String>,
    ) {
        for (_, pattern) in &self.c2_patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                if let Some(m) = regex.find(script) {
                    *scores.entry(pattern.framework.clone()).or_insert(0.0) +=
                        pattern.confidence_weight;
                    indicators.push(m.as_str().to_string());
                }
            }
        }
    }

    /// Extract beacon configuration from script
    fn extract_beacon_configuration(
        &self,
        script: &str,
        _deob_results: &[DeobfuscationResult],
    ) -> Result<Option<BeaconConfiguration>> {
        let mut config = BeaconConfiguration {
            c2_servers: Vec::new(),
            beacon_interval: None,
            jitter_percentage: None,
            user_agent: None,
            communication_protocol: None,
            encryption_key: None,
            persistence_method: None,
            injection_technique: None,
            malleable_profile: None,
        };

        // Extract C2 servers (IP addresses and domains)
        let ip_regex = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b")?;
        for ip_match in ip_regex.find_iter(script) {
            config.c2_servers.push(ip_match.as_str().to_string());
        }

        let domain_regex = Regex::new(r"(?i)https?://([a-zA-Z0-9.-]+)")?;
        for domain_match in domain_regex.captures_iter(script) {
            if let Some(domain) = domain_match.get(1) {
                config.c2_servers.push(domain.as_str().to_string());
            }
        }

        // Extract beacon interval
        let interval_regex = Regex::new(r"(?i)sleep\s+(\d+)")?;
        if let Some(interval_match) = interval_regex.captures(script) {
            if let Some(interval) = interval_match.get(1) {
                if let Ok(seconds) = interval.as_str().parse::<u64>() {
                    config.beacon_interval = Some(seconds);
                }
            }
        }

        // Extract user agent
        let ua_regex = Regex::new(r#"(?i)user-agent["'\s]*:?["'\s]*([^"'\n\r]+)"#)?;
        if let Some(ua_match) = ua_regex.captures(script) {
            if let Some(ua) = ua_match.get(1) {
                config.user_agent = Some(ua.as_str().trim().to_string());
            }
        }

        if !config.c2_servers.is_empty() || config.beacon_interval.is_some() {
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    /// Analyze malicious functions in the script
    fn analyze_malicious_functions(&self, script: &str) -> Result<Vec<MaliciousFunction>> {
        let mut functions = Vec::new();

        // Network connection functions
        let network_patterns = [
            (
                r"(?i)New-Object.*Net\.WebClient",
                "WebClient Creation",
                MaliciousFunctionType::NetworkConnection,
            ),
            (
                r"(?i)Invoke-WebRequest",
                "Web Request",
                MaliciousFunctionType::NetworkConnection,
            ),
            (
                r"(?i)wget",
                "File Download",
                MaliciousFunctionType::FileDownload,
            ),
            (
                r"(?i)curl",
                "File Download",
                MaliciousFunctionType::FileDownload,
            ),
        ];

        for (pattern, desc, func_type) in &network_patterns {
            let regex = Regex::new(pattern)?;
            for m in regex.find_iter(script) {
                functions.push(MaliciousFunction {
                    function_name: m.as_str().to_string(),
                    function_type: func_type.clone(),
                    parameters: HashMap::new(),
                    risk_level: RiskLevel::High,
                    description: desc.to_string(),
                    mitre_techniques: vec!["T1071.001".to_string()], // Web Protocols
                    line_number: None,
                });
            }
        }

        // Process injection functions
        let injection_patterns = [
            (
                r"(?i)VirtualAlloc",
                "Memory Allocation",
                MaliciousFunctionType::ProcessInjection,
            ),
            (
                r"(?i)WriteProcessMemory",
                "Process Memory Write",
                MaliciousFunctionType::ProcessInjection,
            ),
            (
                r"(?i)CreateRemoteThread",
                "Remote Thread Creation",
                MaliciousFunctionType::ProcessInjection,
            ),
        ];

        for (pattern, desc, func_type) in &injection_patterns {
            let regex = Regex::new(pattern)?;
            for m in regex.find_iter(script) {
                functions.push(MaliciousFunction {
                    function_name: m.as_str().to_string(),
                    function_type: func_type.clone(),
                    parameters: HashMap::new(),
                    risk_level: RiskLevel::Critical,
                    description: desc.to_string(),
                    mitre_techniques: vec!["T1055".to_string()], // Process Injection
                    line_number: None,
                });
            }
        }

        Ok(functions)
    }

    /// Extract network indicators from script
    fn extract_network_indicators(&self, script: &str) -> Result<Vec<NetworkIndicator>> {
        let mut indicators = Vec::new();

        // IP addresses
        let ip_regex = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b")?;
        for ip_match in ip_regex.find_iter(script) {
            indicators.push(NetworkIndicator {
                indicator_type: NetworkIndicatorType::IPAddress,
                value: ip_match.as_str().to_string(),
                context: "Found in script".to_string(),
                confidence: 0.9,
                first_seen: SystemTime::now(),
            });
        }

        // Domains and URLs
        let url_regex = Regex::new(r"(?i)https?://([a-zA-Z0-9.-]+)")?;
        for url_match in url_regex.captures_iter(script) {
            if let Some(domain) = url_match.get(1) {
                indicators.push(NetworkIndicator {
                    indicator_type: NetworkIndicatorType::Domain,
                    value: domain.as_str().to_string(),
                    context: "Found in URL".to_string(),
                    confidence: 0.95,
                    first_seen: SystemTime::now(),
                });
            }
        }

        Ok(indicators)
    }

    /// Analyze file operations in script
    fn analyze_file_operations(&self, script: &str) -> Result<Vec<FileOperation>> {
        let mut operations = Vec::new();

        let file_patterns = [
            (
                r"(?i)Get-Content\s+([^\s]+)",
                "File Read",
                FileOperationType::Read,
            ),
            (
                r"(?i)Set-Content\s+([^\s]+)",
                "File Write",
                FileOperationType::Write,
            ),
            (
                r"(?i)Remove-Item\s+([^\s]+)",
                "File Delete",
                FileOperationType::Delete,
            ),
            (
                r"(?i)Copy-Item\s+([^\s]+)",
                "File Copy",
                FileOperationType::Copy,
            ),
        ];

        for (pattern, desc, op_type) in &file_patterns {
            let regex = Regex::new(pattern)?;
            for capture in regex.captures_iter(script) {
                if let Some(path) = capture.get(1) {
                    operations.push(FileOperation {
                        operation_type: op_type.clone(),
                        file_path: path.as_str().to_string(),
                        description: desc.to_string(),
                        risk_level: RiskLevel::Medium,
                        mitre_technique: Some("T1005".to_string()), // Data from Local System
                    });
                }
            }
        }

        Ok(operations)
    }

    /// Analyze registry operations in script
    fn analyze_registry_operations(&self, script: &str) -> Result<Vec<RegistryOperation>> {
        let mut operations = Vec::new();

        let reg_patterns = [
            (
                r"(?i)Get-ItemProperty\s+([^\s]+)",
                "Registry Read",
                RegistryOperationType::GetValue,
            ),
            (
                r"(?i)Set-ItemProperty\s+([^\s]+)",
                "Registry Write",
                RegistryOperationType::SetValue,
            ),
            (
                r"(?i)New-ItemProperty\s+([^\s]+)",
                "Registry Create",
                RegistryOperationType::SetValue,
            ),
            (
                r"(?i)Remove-ItemProperty\s+([^\s]+)",
                "Registry Delete",
                RegistryOperationType::DeleteValue,
            ),
        ];

        for (pattern, desc, op_type) in &reg_patterns {
            let regex = Regex::new(pattern)?;
            for capture in regex.captures_iter(script) {
                if let Some(path) = capture.get(1) {
                    operations.push(RegistryOperation {
                        operation_type: op_type.clone(),
                        registry_path: path.as_str().to_string(),
                        value_name: None,
                        value_data: None,
                        description: desc.to_string(),
                        risk_level: RiskLevel::Medium,
                        mitre_technique: Some("T1112".to_string()), // Modify Registry
                    });
                }
            }
        }

        Ok(operations)
    }

    /// Analyze process operations in script
    fn analyze_process_operations(&self, script: &str) -> Result<Vec<ProcessOperation>> {
        let mut operations = Vec::new();

        let proc_patterns = [
            (
                r"(?i)Start-Process\s+([^\s]+)",
                "Process Start",
                ProcessOperationType::Create,
            ),
            (
                r"(?i)Stop-Process\s+([^\s]+)",
                "Process Stop",
                ProcessOperationType::Kill,
            ),
            (
                r"(?i)Get-Process",
                "Process List",
                ProcessOperationType::ListProcesses,
            ),
        ];

        for (pattern, desc, op_type) in &proc_patterns {
            let regex = Regex::new(pattern)?;
            for capture in regex.captures_iter(script) {
                let process_name = capture.get(1).map(|m| m.as_str().to_string());

                operations.push(ProcessOperation {
                    operation_type: op_type.clone(),
                    process_name,
                    command_line: None,
                    description: desc.to_string(),
                    risk_level: RiskLevel::Medium,
                    mitre_technique: Some("T1059.001".to_string()), // PowerShell
                });
            }
        }

        Ok(operations)
    }

    /// Detect evasion techniques in script
    fn detect_evasion_techniques(&self, script: &str) -> Result<Vec<EvasionTechnique>> {
        let mut techniques = Vec::new();

        // AMSI bypass detection
        if Regex::new(r"(?i)(amsi|antimalware|scan|interface)")?.is_match(script) {
            techniques.push(EvasionTechnique {
                technique: "AMSI Bypass".to_string(),
                method: "AMSI interface manipulation".to_string(),
                confidence: 0.8,
                description: "Attempts to bypass Windows Antimalware Scan Interface".to_string(),
                mitre_technique: "T1562.001".to_string(), // Disable or Modify Tools
            });
        }

        // Execution policy bypass
        if Regex::new(r"(?i)-ExecutionPolicy\s+Bypass")?.is_match(script) {
            techniques.push(EvasionTechnique {
                technique: "Execution Policy Bypass".to_string(),
                method: "PowerShell execution policy modification".to_string(),
                confidence: 0.9,
                description: "Bypasses PowerShell execution policy restrictions".to_string(),
                mitre_technique: "T1562.001".to_string(),
            });
        }

        Ok(techniques)
    }

    /// Extract IoCs from analysis results
    fn extract_iocs(
        &self,
        _script: &str,
        network_indicators: &[NetworkIndicator],
        file_operations: &[FileOperation],
    ) -> Result<Vec<IndicatorOfCompromise>> {
        let mut iocs = Vec::new();

        // Convert network indicators to IoCs
        for indicator in network_indicators {
            let ioc_type = match indicator.indicator_type {
                NetworkIndicatorType::IPAddress => IocType::IPAddress,
                NetworkIndicatorType::Domain => IocType::Domain,
                NetworkIndicatorType::URL => IocType::URL,
                _ => continue,
            };

            iocs.push(IndicatorOfCompromise {
                ioc_type,
                value: indicator.value.clone(),
                confidence: indicator.confidence,
                context: indicator.context.clone(),
                tags: vec!["powershell".to_string(), "beacon".to_string()],
            });
        }

        // Convert file operations to IoCs
        for operation in file_operations {
            iocs.push(IndicatorOfCompromise {
                ioc_type: IocType::FilePath,
                value: operation.file_path.clone(),
                confidence: 0.7,
                context: format!("File operation: {:?}", operation.operation_type),
                tags: vec!["powershell".to_string(), "file".to_string()],
            });
        }

        Ok(iocs)
    }

    /// Generate MITRE ATT&CK mappings
    fn generate_mitre_mappings(
        &self,
        malicious_functions: &[MaliciousFunction],
        evasion_techniques: &[EvasionTechnique],
    ) -> Result<Vec<MitreMapping>> {
        let mut mappings = HashMap::new();

        // From malicious functions
        for func in malicious_functions {
            for technique in &func.mitre_techniques {
                let mapping = mappings
                    .entry(technique.clone())
                    .or_insert_with(|| MitreMapping {
                        technique_id: technique.clone(),
                        technique_name: self.get_mitre_technique_name(technique),
                        tactic: self.get_mitre_tactic(technique),
                        confidence: 0.0,
                        evidence: Vec::new(),
                    });
                mapping.confidence = (mapping.confidence + 0.8).min(1.0);
                mapping.evidence.push(func.function_name.clone());
            }
        }

        // From evasion techniques
        for evasion in evasion_techniques {
            let mapping = mappings
                .entry(evasion.mitre_technique.clone())
                .or_insert_with(|| MitreMapping {
                    technique_id: evasion.mitre_technique.clone(),
                    technique_name: self.get_mitre_technique_name(&evasion.mitre_technique),
                    tactic: self.get_mitre_tactic(&evasion.mitre_technique),
                    confidence: 0.0,
                    evidence: Vec::new(),
                });
            mapping.confidence = (mapping.confidence + evasion.confidence).min(1.0);
            mapping.evidence.push(evasion.technique.clone());
        }

        Ok(mappings.into_values().collect())
    }

    /// Perform risk assessment
    fn perform_risk_assessment(
        &self,
        c2_detection: &Option<C2FrameworkDetection>,
        malicious_functions: &[MaliciousFunction],
        evasion_techniques: &[EvasionTechnique],
    ) -> Result<RiskAssessment> {
        let mut risk_factors = Vec::new();
        let mut total_score = 0.0;

        // C2 framework detection factor
        if let Some(detection) = c2_detection {
            let weight = 0.4;
            let score = detection.confidence * 100.0;
            risk_factors.push(RiskFactor {
                factor: "C2 Framework Detection".to_string(),
                weight,
                contribution: score * weight,
                description: format!(
                    "Detected {} framework",
                    format!("{:?}", detection.framework)
                ),
            });
            total_score += score * weight;
        }

        // Malicious functions factor
        if !malicious_functions.is_empty() {
            let weight = 0.3;
            let critical_count = malicious_functions
                .iter()
                .filter(|f| matches!(f.risk_level, RiskLevel::Critical))
                .count() as f64;
            let high_count = malicious_functions
                .iter()
                .filter(|f| matches!(f.risk_level, RiskLevel::High))
                .count() as f64;

            let score = ((critical_count * 10.0) + (high_count * 7.0)).min(100.0);
            risk_factors.push(RiskFactor {
                factor: "Malicious Functions".to_string(),
                weight,
                contribution: score * weight,
                description: format!("Found {} malicious functions", malicious_functions.len()),
            });
            total_score += score * weight;
        }

        // Evasion techniques factor
        if !evasion_techniques.is_empty() {
            let weight = 0.2;
            let avg_confidence = evasion_techniques.iter().map(|e| e.confidence).sum::<f64>()
                / evasion_techniques.len() as f64;
            let score = avg_confidence * 100.0;

            risk_factors.push(RiskFactor {
                factor: "Evasion Techniques".to_string(),
                weight,
                contribution: score * weight,
                description: format!("Found {} evasion techniques", evasion_techniques.len()),
            });
            total_score += score * weight;
        }

        let overall_risk = match total_score {
            s if s >= 80.0 => RiskLevel::Critical,
            s if s >= 60.0 => RiskLevel::High,
            s if s >= 40.0 => RiskLevel::Medium,
            s if s >= 20.0 => RiskLevel::Low,
            _ => RiskLevel::Info,
        };

        let recommendations = self.generate_recommendations(&overall_risk, &risk_factors);

        Ok(RiskAssessment {
            overall_risk,
            risk_score: total_score,
            risk_factors,
            recommendations,
        })
    }

    /// Generate security recommendations
    fn generate_recommendations(
        &self,
        risk_level: &RiskLevel,
        factors: &[RiskFactor],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        match risk_level {
            RiskLevel::Critical => {
                recommendations
                    .push("IMMEDIATE ACTION REQUIRED: Isolate affected systems".to_string());
                recommendations
                    .push("Block all network communications to identified C2 servers".to_string());
                recommendations.push("Initiate incident response procedures".to_string());
            }
            RiskLevel::High => {
                recommendations.push("High priority investigation required".to_string());
                recommendations
                    .push("Monitor network traffic for beacon communications".to_string());
                recommendations.push("Review system logs for additional indicators".to_string());
            }
            RiskLevel::Medium => {
                recommendations.push("Schedule detailed analysis of the payload".to_string());
                recommendations
                    .push("Implement additional monitoring for suspicious activities".to_string());
            }
            _ => {
                recommendations.push("Continue monitoring for evolving threats".to_string());
            }
        }

        // Factor-specific recommendations
        for factor in factors {
            match factor.factor.as_str() {
                "C2 Framework Detection" => {
                    recommendations.push("Deploy C2 traffic detection rules".to_string());
                }
                "Malicious Functions" => {
                    recommendations.push("Review process execution logs".to_string());
                }
                "Evasion Techniques" => {
                    recommendations
                        .push("Update security tools to detect evasion techniques".to_string());
                }
                _ => {}
            }
        }

        recommendations
    }

    // Helper methods for entropy and complexity calculations
    fn calculate_entropy(&self, data: &str) -> f64 {
        let mut frequency = HashMap::new();
        for byte in data.bytes() {
            *frequency.entry(byte).or_insert(0) += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in frequency.values() {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }

        entropy
    }

    fn calculate_string_entropy(&self, script: &str) -> f64 {
        // Extract strings and calculate their entropy
        let string_regex = Regex::new(r#"["']([^"']*)["']"#).unwrap();
        let strings: Vec<&str> = string_regex
            .captures_iter(script)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();

        if strings.is_empty() {
            return 0.0;
        }

        let combined = strings.join("");
        self.calculate_entropy(&combined)
    }

    fn calculate_variable_entropy(&self, script: &str) -> f64 {
        // Extract variable names and calculate entropy
        let var_regex = Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
        let variables: Vec<&str> = var_regex
            .captures_iter(script)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();

        if variables.is_empty() {
            return 0.0;
        }

        let combined = variables.join("");
        self.calculate_entropy(&combined)
    }

    fn calculate_function_entropy(&self, script: &str) -> f64 {
        // Extract function names and calculate entropy
        let func_regex = Regex::new(r"(?i)function\s+([a-zA-Z_][a-zA-Z0-9_-]*)").unwrap();
        let functions: Vec<&str> = func_regex
            .captures_iter(script)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();

        if functions.is_empty() {
            return 0.0;
        }

        let combined = functions.join("");
        self.calculate_entropy(&combined)
    }

    fn calculate_complexity_score(&self, script: &str, techniques: &[ObfuscationTechnique]) -> f64 {
        let mut score = 0.0;

        // Base score from script length and structure
        score += (script.len() as f64).log10() * 10.0;

        // Add score for each obfuscation technique
        for technique in techniques {
            score += match technique {
                ObfuscationTechnique::Base64Encoding => 15.0,
                ObfuscationTechnique::XorEncryption => 25.0,
                ObfuscationTechnique::StringConcatenation => 10.0,
                ObfuscationTechnique::InvokeExpression => 20.0,
                _ => 5.0,
            };
        }

        // Normalize to 0-100 scale
        (score / 10.0).min(100.0)
    }

    fn analyze_strings(&self, script: &str) -> Result<StringAnalysis> {
        let string_regex = Regex::new(r#"["']([^"']*)["']"#)?;
        let mut all_strings = Vec::new();
        let mut suspicious_strings = Vec::new();
        let mut base64_candidates = Vec::new();
        let mut hex_candidates = Vec::new();
        let mut url_patterns = Vec::new();

        for capture in string_regex.captures_iter(script) {
            if let Some(string_match) = capture.get(1) {
                let string_value = string_match.as_str();
                all_strings.push(string_value);

                // Check for Base64
                if string_value.len() > 20
                    && Regex::new(r"^[A-Za-z0-9+/]+=*$")
                        .unwrap()
                        .is_match(string_value)
                {
                    base64_candidates.push(string_value.to_string());
                }

                // Check for hex
                if Regex::new(r"^[0-9a-fA-F]+$")
                    .unwrap()
                    .is_match(string_value)
                    && string_value.len() % 2 == 0
                    && string_value.len() > 10
                {
                    hex_candidates.push(string_value.to_string());
                }

                // Check for URLs
                if Regex::new(r"^https?://").unwrap().is_match(string_value) {
                    url_patterns.push(string_value.to_string());
                }

                // Check for suspicious content
                if self.is_suspicious_string(string_value) {
                    suspicious_strings.push(SuspiciousString {
                        value: string_value.to_string(),
                        suspicious_type: self.classify_suspicious_string(string_value),
                        confidence: 0.8,
                        context: "Found in string literal".to_string(),
                    });
                }
            }
        }

        let encoded_count = base64_candidates.len() + hex_candidates.len();

        Ok(StringAnalysis {
            total_strings: all_strings.len(),
            encoded_strings: encoded_count,
            suspicious_strings,
            base64_candidates,
            hex_candidates,
            url_patterns,
        })
    }

    fn is_suspicious_string(&self, string_value: &str) -> bool {
        let suspicious_patterns = [
            r"(?i)powershell",
            r"(?i)invoke-expression",
            r"(?i)iex",
            r"(?i)downloadstring",
            r"(?i)webclient",
            r"(?i)base64",
            r"(?i)bypass",
            r"(?i)hidden",
            r"(?i)noprofile",
        ];

        for pattern in &suspicious_patterns {
            if Regex::new(pattern).unwrap().is_match(string_value) {
                return true;
            }
        }

        false
    }

    fn classify_suspicious_string(&self, string_value: &str) -> SuspiciousStringType {
        if Regex::new(r"(?i)https?://").unwrap().is_match(string_value) {
            SuspiciousStringType::C2_URL
        } else if Regex::new(r"(?i)(powershell|cmd|rundll32)")
            .unwrap()
            .is_match(string_value)
        {
            SuspiciousStringType::SystemCommand
        } else if Regex::new(r"(?i)(HKEY_|HKLM|HKCU)")
            .unwrap()
            .is_match(string_value)
        {
            SuspiciousStringType::RegistryPath
        } else if Regex::new(r"(?i)[A-Za-z0-9+/]{20,}={0,2}")
            .unwrap()
            .is_match(string_value)
        {
            SuspiciousStringType::EncodedCommand
        } else {
            SuspiciousStringType::NetworkAddress
        }
    }

    fn get_mitre_technique_name(&self, technique_id: &str) -> String {
        match technique_id {
            "T1071.001" => "Application Layer Protocol: Web Protocols".to_string(),
            "T1055" => "Process Injection".to_string(),
            "T1562.001" => "Disable or Modify Tools".to_string(),
            "T1112" => "Modify Registry".to_string(),
            "T1059.001" => "Command and Scripting Interpreter: PowerShell".to_string(),
            "T1005" => "Data from Local System".to_string(),
            _ => format!("Unknown Technique ({})", technique_id),
        }
    }

    fn get_mitre_tactic(&self, technique_id: &str) -> String {
        match technique_id {
            "T1071.001" => "Command and Control".to_string(),
            "T1055" => "Defense Evasion".to_string(),
            "T1562.001" => "Defense Evasion".to_string(),
            "T1112" => "Defense Evasion".to_string(),
            "T1059.001" => "Execution".to_string(),
            "T1005" => "Collection".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl Default for PowerShellBeaconDissector {
    fn default() -> Self {
        Self::new()
    }
}
