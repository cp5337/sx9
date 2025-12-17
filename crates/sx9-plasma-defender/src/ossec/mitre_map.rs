//! MITRE ATT&CK Mapping for Plasma Defender
//!
//! RFC-9001 compliant mapping between:
//! - MITRE ATT&CK tactics → 32 Primitives
//! - MITRE ATT&CK tactics → HD4 phases
//! - Alert severity → HD4 phases
//! - Unicode triggers for deterministic routing

use crate::ecs::components::Hd4Phase;
use std::collections::HashMap;

// =============================================================================
// 32 PRIMITIVES (RFC-9001)
// =============================================================================

/// 32 Primitives from RFC-9001
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Primitive {
    Read = 0,
    Write = 1,
    Filter = 2,
    Transform = 3,
    Execute = 4,
    Authenticate = 5,
    Authorize = 6,
    Encrypt = 7,
    Decrypt = 8,
    Validate = 9,
    Route = 10,
    Buffer = 11,
    Queue = 12,
    Cache = 13,
    Replicate = 14,
    Synchronize = 15,
    Observe = 16,
    Measure = 17,
    Alert = 18,
    Log = 19,
    Notify = 20,
    Escalate = 21,
    Contain = 22,
    Isolate = 23,
    Remediate = 24,
    Recover = 25,
    Reconnaissance = 26,
    Weaponize = 27,
    Deliver = 28,
    Exploit = 29,
    Install = 30,
    CommandControl = 31,
}

impl Primitive {
    /// Get primitive from index
    pub fn from_index(idx: u8) -> Option<Self> {
        if idx < 32 {
            Some(unsafe { std::mem::transmute(idx) })
        } else {
            None
        }
    }

    /// Get as bitmask
    pub fn as_bitmask(&self) -> u32 {
        1 << (*self as u8)
    }

    /// Get Unicode trigger (U+E400-E41F range for OSSEC)
    pub fn unicode_trigger(&self) -> u32 {
        0xE400 + (*self as u32)
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Read => "READ",
            Self::Write => "WRITE",
            Self::Filter => "FILTER",
            Self::Transform => "TRANSFORM",
            Self::Execute => "EXECUTE",
            Self::Authenticate => "AUTHENTICATE",
            Self::Authorize => "AUTHORIZE",
            Self::Encrypt => "ENCRYPT",
            Self::Decrypt => "DECRYPT",
            Self::Validate => "VALIDATE",
            Self::Route => "ROUTE",
            Self::Buffer => "BUFFER",
            Self::Queue => "QUEUE",
            Self::Cache => "CACHE",
            Self::Replicate => "REPLICATE",
            Self::Synchronize => "SYNCHRONIZE",
            Self::Observe => "OBSERVE",
            Self::Measure => "MEASURE",
            Self::Alert => "ALERT",
            Self::Log => "LOG",
            Self::Notify => "NOTIFY",
            Self::Escalate => "ESCALATE",
            Self::Contain => "CONTAIN",
            Self::Isolate => "ISOLATE",
            Self::Remediate => "REMEDIATE",
            Self::Recover => "RECOVER",
            Self::Reconnaissance => "RECONNAISSANCE",
            Self::Weaponize => "WEAPONIZE",
            Self::Deliver => "DELIVER",
            Self::Exploit => "EXPLOIT",
            Self::Install => "INSTALL",
            Self::CommandControl => "COMMAND_CONTROL",
        }
    }
}

// =============================================================================
// TACTIC TO PRIMITIVE MAPPING
// =============================================================================

/// MITRE ATT&CK tactic to primitive mapping
#[derive(Debug, Clone)]
pub struct TacticPrimitive {
    pub tactic: &'static str,
    pub primitive: Primitive,
    pub hd4_affinity: Hd4Phase,
}

/// Get primitive for MITRE tactic
pub fn tactic_to_primitive(tactic: &str) -> Primitive {
    match tactic.to_lowercase().as_str() {
        "reconnaissance" => Primitive::Reconnaissance,
        "resource-development" => Primitive::Weaponize,
        "initial-access" => Primitive::Deliver,
        "execution" => Primitive::Execute,
        "persistence" => Primitive::Install,
        "privilege-escalation" => Primitive::Escalate,
        "defense-evasion" => Primitive::Filter,
        "credential-access" => Primitive::Authenticate,
        "discovery" => Primitive::Observe,
        "lateral-movement" => Primitive::Route,
        "collection" => Primitive::Read,
        "command-and-control" => Primitive::CommandControl,
        "exfiltration" => Primitive::Write,
        "impact" => Primitive::Transform,
        _ => Primitive::Observe, // Default to observe
    }
}

/// Get HD4 phase affinity for MITRE tactic
pub fn tactic_to_hd4(tactic: &str) -> Hd4Phase {
    match tactic.to_lowercase().as_str() {
        // Hunt phase - early reconnaissance
        "reconnaissance" | "resource-development" => Hd4Phase::Hunt,
        // Detect phase - initial compromise
        "initial-access" | "execution" | "persistence" => Hd4Phase::Detect,
        // Disrupt phase - active threats
        "privilege-escalation" | "defense-evasion" | "credential-access" => Hd4Phase::Disrupt,
        // Disable phase - lateral movement and collection
        "discovery" | "lateral-movement" | "collection" => Hd4Phase::Disable,
        // Dominate phase - command and control, exfil, impact
        "command-and-control" | "exfiltration" | "impact" => Hd4Phase::Dominate,
        _ => Hd4Phase::Detect,
    }
}

// =============================================================================
// HD4 LEVEL MAPPING
// =============================================================================

/// HD4 severity levels based on OSSEC alert levels (0-15)
#[derive(Debug, Clone, Copy)]
pub struct Hd4Level {
    pub min_level: u8,
    pub max_level: u8,
    pub phase: Hd4Phase,
}

/// Map OSSEC alert level (0-15) to HD4 phase
pub fn level_to_hd4(level: u8) -> Hd4Phase {
    match level {
        0..=3 => Hd4Phase::Hunt,      // Low severity - reconnaissance
        4..=7 => Hd4Phase::Detect,    // Medium severity - detection
        8..=10 => Hd4Phase::Disrupt,  // High severity - active response
        11..=13 => Hd4Phase::Disable, // Critical severity - neutralization
        _ => Hd4Phase::Dominate,      // Maximum severity - full control
    }
}

/// Get severity description for HD4 phase
pub fn hd4_to_severity(phase: Hd4Phase) -> &'static str {
    match phase {
        Hd4Phase::Hunt => "LOW",
        Hd4Phase::Detect => "MEDIUM",
        Hd4Phase::Disrupt => "HIGH",
        Hd4Phase::Disable => "CRITICAL",
        Hd4Phase::Dominate => "MAXIMUM",
    }
}

// =============================================================================
// MITRE MAPPING SERVICE
// =============================================================================

/// MITRE ATT&CK mapping service
#[derive(Debug, Clone)]
pub struct MitreMapping {
    /// Technique ID to name mapping
    techniques: HashMap<String, TechniqueInfo>,
    /// Tactic to techniques mapping
    tactic_techniques: HashMap<String, Vec<String>>,
}

/// Information about a MITRE technique
#[derive(Debug, Clone)]
pub struct TechniqueInfo {
    pub id: String,
    pub name: String,
    pub tactics: Vec<String>,
    pub platforms: Vec<String>,
    pub primitive: Primitive,
    pub hd4_phase: Hd4Phase,
}

impl MitreMapping {
    /// Create new MITRE mapping (will be populated from data files)
    pub fn new() -> Self {
        let mut mapping = Self {
            techniques: HashMap::new(),
            tactic_techniques: HashMap::new(),
        };

        // Add commonly used techniques
        mapping.add_common_techniques();

        mapping
    }

    /// Add commonly detected techniques
    fn add_common_techniques(&mut self) {
        // Execution techniques
        self.add_technique("T1059", "Command and Scripting Interpreter", &["execution"]);
        self.add_technique("T1059.001", "PowerShell", &["execution"]);
        self.add_technique("T1059.003", "Windows Command Shell", &["execution"]);
        self.add_technique("T1059.004", "Unix Shell", &["execution"]);

        // Persistence techniques
        self.add_technique("T1053", "Scheduled Task/Job", &["execution", "persistence"]);
        self.add_technique(
            "T1547",
            "Boot or Logon Autostart Execution",
            &["persistence"],
        );
        self.add_technique("T1543", "Create or Modify System Process", &["persistence"]);

        // Privilege Escalation
        self.add_technique(
            "T1548",
            "Abuse Elevation Control Mechanism",
            &["privilege-escalation"],
        );
        self.add_technique(
            "T1068",
            "Exploitation for Privilege Escalation",
            &["privilege-escalation"],
        );

        // Defense Evasion
        self.add_technique("T1070", "Indicator Removal", &["defense-evasion"]);
        self.add_technique("T1036", "Masquerading", &["defense-evasion"]);
        self.add_technique(
            "T1027",
            "Obfuscated Files or Information",
            &["defense-evasion"],
        );

        // Credential Access
        self.add_technique("T1003", "OS Credential Dumping", &["credential-access"]);
        self.add_technique("T1110", "Brute Force", &["credential-access"]);
        self.add_technique("T1552", "Unsecured Credentials", &["credential-access"]);

        // Discovery
        self.add_technique("T1087", "Account Discovery", &["discovery"]);
        self.add_technique("T1082", "System Information Discovery", &["discovery"]);
        self.add_technique("T1083", "File and Directory Discovery", &["discovery"]);

        // Lateral Movement
        self.add_technique("T1021", "Remote Services", &["lateral-movement"]);
        self.add_technique("T1570", "Lateral Tool Transfer", &["lateral-movement"]);

        // Collection
        self.add_technique("T1005", "Data from Local System", &["collection"]);
        self.add_technique("T1039", "Data from Network Shared Drive", &["collection"]);

        // Command and Control
        self.add_technique(
            "T1071",
            "Application Layer Protocol",
            &["command-and-control"],
        );
        self.add_technique(
            "T1095",
            "Non-Application Layer Protocol",
            &["command-and-control"],
        );
        self.add_technique("T1572", "Protocol Tunneling", &["command-and-control"]);

        // Exfiltration
        self.add_technique("T1041", "Exfiltration Over C2 Channel", &["exfiltration"]);
        self.add_technique(
            "T1048",
            "Exfiltration Over Alternative Protocol",
            &["exfiltration"],
        );

        // Impact
        self.add_technique("T1486", "Data Encrypted for Impact", &["impact"]);
        self.add_technique("T1490", "Inhibit System Recovery", &["impact"]);
        self.add_technique("T1489", "Service Stop", &["impact"]);
    }

    /// Add a technique to the mapping
    fn add_technique(&mut self, id: &str, name: &str, tactics: &[&str]) {
        let tactics_vec: Vec<String> = tactics.iter().map(|s| s.to_string()).collect();

        // Determine primitive and HD4 phase from primary tactic
        let primary_tactic = tactics.first().unwrap_or(&"discovery");
        let primitive = tactic_to_primitive(primary_tactic);
        let hd4_phase = tactic_to_hd4(primary_tactic);

        let info = TechniqueInfo {
            id: id.to_string(),
            name: name.to_string(),
            tactics: tactics_vec.clone(),
            platforms: vec![
                "Windows".to_string(),
                "Linux".to_string(),
                "macOS".to_string(),
            ],
            primitive,
            hd4_phase,
        };

        self.techniques.insert(id.to_string(), info);

        // Add to tactic mapping
        for tactic in tactics_vec {
            self.tactic_techniques
                .entry(tactic)
                .or_insert_with(Vec::new)
                .push(id.to_string());
        }
    }

    /// Get technique info by ID
    pub fn get_technique(&self, id: &str) -> Option<&TechniqueInfo> {
        self.techniques.get(id)
    }

    /// Get techniques for tactic
    pub fn get_techniques_for_tactic(&self, tactic: &str) -> Vec<&TechniqueInfo> {
        self.tactic_techniques
            .get(tactic)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.techniques.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get primitive for technique
    pub fn get_primitive(&self, technique_id: &str) -> Primitive {
        self.techniques
            .get(technique_id)
            .map(|t| t.primitive)
            .unwrap_or(Primitive::Observe)
    }

    /// Get HD4 phase for technique
    pub fn get_hd4_phase(&self, technique_id: &str) -> Hd4Phase {
        self.techniques
            .get(technique_id)
            .map(|t| t.hd4_phase)
            .unwrap_or(Hd4Phase::Detect)
    }

    /// Parse technique ID from string (handles "T1059.001" format)
    pub fn parse_technique_id(s: &str) -> Option<String> {
        let s = s.trim().to_uppercase();
        if s.starts_with('T') && s.len() >= 5 {
            // Validate it looks like a technique ID
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() <= 2 {
                return Some(s);
            }
        }
        None
    }

    /// Get all known technique IDs
    pub fn all_technique_ids(&self) -> Vec<&String> {
        self.techniques.keys().collect()
    }
}

impl Default for MitreMapping {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tactic_to_primitive() {
        assert_eq!(tactic_to_primitive("execution"), Primitive::Execute);
        assert_eq!(
            tactic_to_primitive("reconnaissance"),
            Primitive::Reconnaissance
        );
        assert_eq!(tactic_to_primitive("EXECUTION"), Primitive::Execute);
    }

    #[test]
    fn test_level_to_hd4() {
        assert_eq!(level_to_hd4(2), Hd4Phase::Hunt);
        assert_eq!(level_to_hd4(5), Hd4Phase::Detect);
        assert_eq!(level_to_hd4(9), Hd4Phase::Disrupt);
        assert_eq!(level_to_hd4(12), Hd4Phase::Disable);
        assert_eq!(level_to_hd4(15), Hd4Phase::Dominate);
    }

    #[test]
    fn test_mitre_mapping() {
        let mapping = MitreMapping::new();

        let technique = mapping.get_technique("T1059.001").unwrap();
        assert_eq!(technique.name, "PowerShell");
        assert!(technique.tactics.contains(&"execution".to_string()));
    }
}
