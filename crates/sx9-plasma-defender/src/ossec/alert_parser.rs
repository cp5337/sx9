//! OSSEC Alert Parser for Plasma Defender
//!
//! Parses OSSEC/Wazuh alerts from JSON format and converts
//! them to ECS-compatible components.

use crate::ecs::components::OssecAlertComponent;
use crate::ossec::mitre_map::{level_to_hd4, tactic_to_primitive, MitreMapping, Primitive};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// OSSEC ALERT STRUCTURES
// =============================================================================

/// Raw OSSEC alert from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssecRawAlert {
    /// Rule ID
    pub rule_id: Option<u32>,
    /// Alert level (0-15)
    pub level: Option<u8>,
    /// Rule description
    pub description: Option<String>,
    /// Rule groups
    pub groups: Option<Vec<String>>,
    /// MITRE ATT&CK info
    pub mitre: Option<MitreInfo>,
    /// Agent info
    pub agent: Option<AgentInfo>,
    /// Source info (syscheck, etc.)
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Full log message
    pub full_log: Option<String>,
}

/// MITRE info in OSSEC alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreInfo {
    /// MITRE techniques (e.g., ["T1059", "T1059.001"])
    pub technique: Option<Vec<String>>,
    /// MITRE tactics (e.g., ["execution"])
    pub tactic: Option<Vec<String>>,
    /// MITRE IDs (alternative format)
    pub id: Option<Vec<String>>,
}

/// Agent info in OSSEC alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Agent ID
    pub id: Option<String>,
    /// Agent name
    pub name: Option<String>,
    /// Agent IP
    pub ip: Option<String>,
}

/// Parsed alert with enriched data
#[derive(Debug, Clone)]
pub struct ParsedAlert {
    /// Original alert component
    pub component: OssecAlertComponent,
    /// Primitive for routing
    pub primitive: Primitive,
    /// Unicode trigger
    pub unicode_trigger: u32,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Is this a high-priority alert?
    pub high_priority: bool,
}

// =============================================================================
// OSSEC ALERT PARSER
// =============================================================================

/// OSSEC Alert Parser with MITRE enrichment
pub struct OssecAlertParser {
    mitre_mapping: MitreMapping,
}

impl OssecAlertParser {
    /// Create new parser
    pub fn new() -> Self {
        Self {
            mitre_mapping: MitreMapping::new(),
        }
    }

    /// Create parser with custom MITRE mapping
    pub fn with_mapping(mapping: MitreMapping) -> Self {
        Self {
            mitre_mapping: mapping,
        }
    }

    /// Parse raw JSON bytes into ParsedAlert
    pub fn parse_bytes(&self, data: &[u8]) -> Result<ParsedAlert> {
        let raw: OssecRawAlert = serde_json::from_slice(data)?;
        self.parse_raw(raw)
    }

    /// Parse JSON string into ParsedAlert
    pub fn parse_json(&self, json: &str) -> Result<ParsedAlert> {
        let raw: OssecRawAlert = serde_json::from_str(json)?;
        self.parse_raw(raw)
    }

    /// Parse raw alert struct into ParsedAlert
    pub fn parse_raw(&self, raw: OssecRawAlert) -> Result<ParsedAlert> {
        let rule_id = raw.rule_id.unwrap_or(0);
        let level = raw.level.unwrap_or(0);
        let description = raw.description.clone().unwrap_or_default();

        // Extract MITRE info
        let (mitre_technique, mitre_tactic) = self.extract_mitre(&raw);

        // Determine primitive
        let primitive = if let Some(ref tactic) = mitre_tactic {
            tactic_to_primitive(tactic)
        } else if let Some(ref technique) = mitre_technique {
            self.mitre_mapping.get_primitive(technique)
        } else {
            // Infer from rule groups
            self.infer_primitive_from_groups(&raw.groups)
        };

        // Extract IPs from data
        let (src_ip, dst_ip) = self.extract_ips(&raw.data);

        // Calculate timestamp
        let timestamp = self.parse_timestamp(&raw.timestamp);

        // Build component
        let component = OssecAlertComponent {
            rule_id,
            level,
            mitre_technique,
            mitre_tactic,
            description,
            src_ip,
            dst_ip,
            timestamp,
            raw_data: serde_json::to_string(&raw).ok(),
        };

        // Calculate confidence from level
        let confidence = (level as f32) / 15.0;

        // High priority if level >= 10 or certain techniques
        let high_priority =
            level >= 10 || self.is_high_priority_technique(&component.mitre_technique);

        Ok(ParsedAlert {
            component,
            primitive,
            unicode_trigger: primitive.unicode_trigger(),
            confidence,
            high_priority,
        })
    }

    /// Extract MITRE technique and tactic from raw alert
    fn extract_mitre(&self, raw: &OssecRawAlert) -> (Option<String>, Option<String>) {
        if let Some(ref mitre) = raw.mitre {
            let technique = mitre
                .technique
                .as_ref()
                .and_then(|t| t.first().cloned())
                .or_else(|| mitre.id.as_ref().and_then(|t| t.first().cloned()));

            let tactic = mitre.tactic.as_ref().and_then(|t| t.first().cloned());

            return (technique, tactic);
        }

        // Try to extract from groups
        if let Some(ref groups) = raw.groups {
            for group in groups {
                // Check for technique patterns
                if group.starts_with('T') && group.len() >= 5 {
                    if let Some(technique) = MitreMapping::parse_technique_id(group) {
                        return (Some(technique), None);
                    }
                }
                // Check for tactic names
                if self.is_tactic(group) {
                    return (None, Some(group.clone()));
                }
            }
        }

        (None, None)
    }

    /// Check if string is a MITRE tactic
    fn is_tactic(&self, s: &str) -> bool {
        matches!(
            s.to_lowercase().as_str(),
            "reconnaissance"
                | "resource-development"
                | "initial-access"
                | "execution"
                | "persistence"
                | "privilege-escalation"
                | "defense-evasion"
                | "credential-access"
                | "discovery"
                | "lateral-movement"
                | "collection"
                | "command-and-control"
                | "exfiltration"
                | "impact"
        )
    }

    /// Infer primitive from rule groups
    fn infer_primitive_from_groups(&self, groups: &Option<Vec<String>>) -> Primitive {
        if let Some(groups) = groups {
            for group in groups {
                let g = group.to_lowercase();
                if g.contains("authentication") || g.contains("login") || g.contains("auth") {
                    return Primitive::Authenticate;
                }
                if g.contains("syscheck") || g.contains("file") {
                    return Primitive::Read;
                }
                if g.contains("rootcheck") || g.contains("rootkit") {
                    return Primitive::Install;
                }
                if g.contains("sshd") || g.contains("ssh") {
                    return Primitive::Route;
                }
                if g.contains("web") || g.contains("apache") || g.contains("nginx") {
                    return Primitive::Deliver;
                }
                if g.contains("firewall") || g.contains("iptables") {
                    return Primitive::Filter;
                }
                if g.contains("sudo") || g.contains("privilege") {
                    return Primitive::Escalate;
                }
            }
        }
        Primitive::Observe // Default
    }

    /// Extract source and destination IPs from data
    fn extract_ips(
        &self,
        data: &Option<HashMap<String, serde_json::Value>>,
    ) -> (Option<String>, Option<String>) {
        if let Some(data) = data {
            let src_ip = data
                .get("srcip")
                .or_else(|| data.get("src_ip"))
                .or_else(|| data.get("srcuser"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let dst_ip = data
                .get("dstip")
                .or_else(|| data.get("dst_ip"))
                .or_else(|| data.get("dstuser"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            return (src_ip, dst_ip);
        }
        (None, None)
    }

    /// Parse timestamp string to nanoseconds
    fn parse_timestamp(&self, ts: &Option<String>) -> u64 {
        if let Some(ts) = ts {
            // Try to parse ISO 8601 format
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
                return dt.timestamp_nanos_opt().unwrap_or(0) as u64;
            }
            // Try other formats
            if let Ok(dt) = chrono::DateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
                return dt.timestamp_nanos_opt().unwrap_or(0) as u64;
            }
        }
        // Default to now
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    /// Check if technique is high priority
    fn is_high_priority_technique(&self, technique: &Option<String>) -> bool {
        if let Some(t) = technique {
            // High priority techniques
            matches!(
                t.as_str(),
                "T1003" |     // Credential Dumping
                "T1486" |     // Ransomware
                "T1489" |     // Service Stop
                "T1490" |     // Inhibit Recovery
                "T1068" |     // Privilege Escalation Exploit
                "T1548" |     // Abuse Elevation Control
                "T1059.001" | // PowerShell
                "T1021" |     // Remote Services
                "T1041" |     // Exfil over C2
                "T1572" // Protocol Tunneling
            )
        } else {
            false
        }
    }

    /// Get MITRE mapping reference
    pub fn mitre_mapping(&self) -> &MitreMapping {
        &self.mitre_mapping
    }
}

impl Default for OssecAlertParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_alert() {
        let parser = OssecAlertParser::new();

        let json = r#"{
            "rule_id": 5501,
            "level": 5,
            "description": "Login session opened.",
            "groups": ["pam", "syslog", "authentication_success"]
        }"#;

        let result = parser.parse_json(json).unwrap();
        assert_eq!(result.component.rule_id, 5501);
        assert_eq!(result.component.level, 5);
        assert_eq!(result.primitive, Primitive::Authenticate);
    }

    #[test]
    fn test_parse_mitre_alert() {
        let parser = OssecAlertParser::new();

        let json = r#"{
            "rule_id": 60001,
            "level": 12,
            "description": "PowerShell execution detected",
            "mitre": {
                "technique": ["T1059.001"],
                "tactic": ["execution"]
            }
        }"#;

        let result = parser.parse_json(json).unwrap();
        assert_eq!(
            result.component.mitre_technique,
            Some("T1059.001".to_string())
        );
        assert_eq!(result.component.mitre_tactic, Some("execution".to_string()));
        assert_eq!(result.primitive, Primitive::Execute);
        assert!(result.high_priority);
    }
}
