//! TOML → Unicode Compiler
//!
//! Compiles human-readable TOML playbooks to Unicode s-expressions for execution.
//! Generates trivariate hashes and maps escalation tiers to Unicode ranges.

use crate::dsl::playbook_unicode::*;
use crate::dsl::{DSLError, DSLResult};
use std::collections::HashMap;
use toml::Value;

/// TOML playbook compiler
pub struct TOMLUnicodeCompiler;

impl TOMLUnicodeCompiler {
    /// Compile TOML playbook to Unicode playbook
    pub fn compile(toml_content: &str) -> DSLResult<UnicodePlaybook> {
        let value: Value = toml::from_str(toml_content)
            .map_err(|e| DSLError::InvalidParameters(format!("TOML parse error: {}", e)))?;

        let table = value
            .as_table()
            .ok_or_else(|| DSLError::InvalidParameters("Expected TOML table".to_string()))?;

        let playbook_table = table
            .get("playbook")
            .and_then(|v| v.as_table())
            .ok_or_else(|| DSLError::InvalidParameters("Missing [playbook] section".to_string()))?;

        // Extract basic playbook info
        let name = playbook_table
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DSLError::InvalidParameters("Missing playbook.name".to_string()))?
            .to_string();

        let version = playbook_table
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0")
            .to_string();

        let description = playbook_table
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut playbook = UnicodePlaybook::new(name, version);
        playbook.description = description;

        // Extract trivariate hash
        if let Some(hash_table) = playbook_table
            .get("trivariate_hash")
            .and_then(|v| v.as_table())
        {
            let trivariate_hash = TrivariateHash {
                sch: hash_table
                    .get("sch")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                cuid: hash_table
                    .get("cuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                uuid: hash_table
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            };
            playbook.trivariate_hash = Some(trivariate_hash);
        }

        // Extract escalation configuration
        if let Some(escalation_table) = playbook_table.get("escalation").and_then(|v| v.as_table())
        {
            playbook.escalation = EscalationConfig {
                tier_1_wasm: escalation_table
                    .get("tier_1_wasm")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tier_2_microkernel: escalation_table
                    .get("tier_2_microkernel")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tier_3_kernel_crate: escalation_table
                    .get("tier_3_kernel_crate")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tier_4_multi_crates: escalation_table
                    .get("tier_4_multi_crates")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    }),
                tier_5_containers: escalation_table
                    .get("tier_5_containers")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tier_6_firefly: escalation_table
                    .get("tier_6_firefly")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tier_7_orb: escalation_table
                    .get("tier_7_orb")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            };
        }

        // Extract Unicode assembly triggers
        if let Some(unicode_table) = playbook_table
            .get("unicode_assembly")
            .and_then(|v| v.as_table())
        {
            playbook.unicode_assembly = UnicodeAssembly {
                primary_trigger: unicode_table
                    .get("primary_trigger")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                escalation_triggers: unicode_table
                    .get("escalation_triggers")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
            };
        }

        // Extract steps
        if let Some(steps_table) = playbook_table.get("steps").and_then(|v| v.as_table()) {
            for (step_key, step_value) in steps_table {
                if let Some(step_table) = step_value.as_table() {
                    let step = Self::parse_step(step_key, step_table)?;
                    playbook.add_step(step);
                }
            }
        }

        // Validate playbook
        playbook.validate()?;

        Ok(playbook)
    }

    /// Parse a single step from TOML
    fn parse_step(
        step_key: &str,
        step_table: &toml::map::Map<String, Value>,
    ) -> DSLResult<UnicodePlaybookStep> {
        let name = step_table
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(step_key)
            .to_string();

        let tier_num = step_table
            .get("tier")
            .and_then(|v| v.as_integer())
            .unwrap_or(1) as u8;

        let tier = match tier_num {
            1 => EscalationTier::Wasm,
            2 => EscalationTier::Microkernel,
            3 => EscalationTier::KernelCrate,
            4 => EscalationTier::MultiCrates,
            5 => EscalationTier::Containers,
            6 => EscalationTier::Firefly,
            7 => EscalationTier::Orb,
            _ => {
                return Err(DSLError::InvalidParameters(format!(
                    "Invalid tier: {}",
                    tier_num
                )))
            }
        };

        // Parse Unicode operation
        let unicode_op_str = step_table
            .get("unicode_op")
            .and_then(|v| v.as_str())
            .unwrap_or("\\u{E800}");

        let unicode_op = Self::parse_unicode_char(unicode_op_str)?;

        let tool = step_table
            .get("tool")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let target = step_table
            .get("target")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let depends_on = step_table
            .get("depends_on")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut metadata = HashMap::new();
        if let Some(metadata_table) = step_table.get("metadata").and_then(|v| v.as_table()) {
            for (key, value) in metadata_table {
                if let Some(str_val) = value.as_str() {
                    metadata.insert(key.clone(), str_val.to_string());
                }
            }
        }

        Ok(UnicodePlaybookStep {
            name,
            tier,
            unicode_op,
            tool,
            target,
            depends_on,
            metadata,
        })
    }

    /// Parse Unicode character from string (e.g., "\\u{E800}" or "U+E800")
    fn parse_unicode_char(s: &str) -> DSLResult<char> {
        // Handle \u{XXXX} format
        if let Some(hex_start) = s.find("\\u{") {
            let hex_end = s[hex_start + 3..].find('}').ok_or_else(|| {
                DSLError::InvalidParameters(format!("Invalid Unicode format: {}", s))
            })?;
            let hex_str = &s[hex_start + 3..hex_start + 3 + hex_end];
            let code_point = u32::from_str_radix(hex_str, 16)
                .map_err(|e| DSLError::InvalidParameters(format!("Invalid hex: {}", e)))?;
            return char::from_u32(code_point).ok_or_else(|| {
                DSLError::InvalidParameters(format!("Invalid Unicode code point: {}", code_point))
            });
        }

        // Handle U+XXXX format
        if let Some(hex_start) = s.find("U+") {
            let hex_str = &s[hex_start + 2..];
            let code_point = u32::from_str_radix(hex_str, 16)
                .map_err(|e| DSLError::InvalidParameters(format!("Invalid hex: {}", e)))?;
            return char::from_u32(code_point).ok_or_else(|| {
                DSLError::InvalidParameters(format!("Invalid Unicode code point: {}", code_point))
            });
        }

        Err(DSLError::InvalidParameters(format!(
            "Unrecognized Unicode format: {}",
            s
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_toml() {
        let toml_content = r#"
[playbook]
name = "test-playbook"
version = "1.0"

[playbook.steps.1]
name = "test-step"
tier = 1
unicode_op = "\\u{E900}"
tool = "nmap"
"#;

        let playbook = TOMLUnicodeCompiler::compile(toml_content).unwrap();
        assert_eq!(playbook.name, "test-playbook");
        assert_eq!(playbook.steps.len(), 1);
        assert_eq!(playbook.steps[0].name, "test-step");
    }

    #[test]
    fn test_parse_unicode_char() {
        assert_eq!(
            TOMLUnicodeCompiler::parse_unicode_char("\\u{E800}").unwrap(),
            '\u{E800}'
        );
        assert_eq!(
            TOMLUnicodeCompiler::parse_unicode_char("U+E800").unwrap(),
            '\u{E800}'
        );
    }
}
