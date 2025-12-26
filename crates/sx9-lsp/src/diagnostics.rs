//! Diagnostics Provider
//!
//! Validates:
//! - N-V-N-N annotation format
//! - Skill schema compliance
//! - RFC reference validity

use regex::Regex;
use tower_lsp::lsp_types::*;

use sx9_harness::SkillRegistry;

/// Generate diagnostics for a document
pub fn diagnose(content: &str, skill_registry: &SkillRegistry) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Check N-V-N-N annotations
    diagnostics.extend(check_nvnn_annotations(content));

    // Check skill references
    diagnostics.extend(check_skill_references(content, skill_registry));

    // Check RFC references
    diagnostics.extend(check_rfc_references(content));

    diagnostics
}

/// Validate N-V-N-N annotation format
fn check_nvnn_annotations(content: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Pattern for N-V-N-N annotations (should be 4 parts)
    let nvnn_regex = Regex::new(r"//\s*([A-Z_]+)_([A-Z_]+)_([A-Z_]+)_([A-Z_]+)(?:_([A-Z_]+))?")
        .unwrap();

    for (line_num, line) in content.lines().enumerate() {
        // Check for partial N-V-N-N patterns
        if line.contains("//") && line.to_uppercase().contains("_") {
            let upper_line = line.to_uppercase();

            // Check for exactly 4 parts
            if let Some(caps) = nvnn_regex.captures(&upper_line) {
                // If there's a 5th capture group, it's malformed (too many parts)
                if caps.get(5).is_some() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: 0,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(NumberOrString::String("NVNN001".to_string())),
                        source: Some("sx9".to_string()),
                        message: "N-V-N-N annotation should have exactly 4 parts: ROLE_ACTION_CONSTRAINT_OBJECT".to_string(),
                        related_information: None,
                        tags: None,
                        code_description: None,
                        data: None,
                    });
                }
            }

            // Check for common role values
            let valid_roles = ["FACTORY", "ANALYZER", "ORCHESTRATOR", "MONITOR", "VALIDATOR"];
            if let Some(caps) = nvnn_regex.captures(&upper_line) {
                if let Some(role) = caps.get(1) {
                    let role_str = role.as_str();
                    if !valid_roles.contains(&role_str) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::HINT),
                            code: Some(NumberOrString::String("NVNN002".to_string())),
                            source: Some("sx9".to_string()),
                            message: format!(
                                "Unknown role '{}'. Consider: {:?}",
                                role_str, valid_roles
                            ),
                            related_information: None,
                            tags: None,
                            code_description: None,
                            data: None,
                        });
                    }
                }
            }
        }
    }

    diagnostics
}

/// Check skill references in code
fn check_skill_references(content: &str, skill_registry: &SkillRegistry) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Look for skill ID patterns
    let skill_pattern = Regex::new(r#"["']([a-z]+\.[a-z_]+)["']"#).unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for caps in skill_pattern.captures_iter(line) {
            if let Some(skill_id) = caps.get(1) {
                let id = skill_id.as_str();

                // Check if skill exists
                if skill_registry.get(id).is_none() {
                    // Only warn if it looks like a skill ID
                    if id.contains('.') {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: skill_id.start() as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: skill_id.end() as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            code: Some(NumberOrString::String("SKL001".to_string())),
                            source: Some("sx9".to_string()),
                            message: format!("Unknown skill ID: '{}'", id),
                            related_information: None,
                            tags: None,
                            code_description: None,
                            data: None,
                        });
                    }
                }
            }
        }
    }

    diagnostics
}

/// Check RFC references
fn check_rfc_references(content: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // RFC pattern
    let rfc_pattern = Regex::new(r"RFC-(\d+)").unwrap();

    // Known RFC ranges
    let valid_ranges = [
        (9000..9100, "Core RFCs"),
        (9100..9200, "Forge RFCs"),
        (9200..9300, "Security RFCs"),
    ];

    for (line_num, line) in content.lines().enumerate() {
        for caps in rfc_pattern.captures_iter(line) {
            if let Some(num_match) = caps.get(1) {
                if let Ok(num) = num_match.as_str().parse::<u32>() {
                    // Check if RFC number is in a known range
                    let in_range = valid_ranges.iter().any(|(range, _)| range.contains(&num));

                    if !in_range && num >= 9000 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: caps.get(0).unwrap().start() as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: caps.get(0).unwrap().end() as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::HINT),
                            code: Some(NumberOrString::String("RFC001".to_string())),
                            source: Some("sx9".to_string()),
                            message: format!(
                                "RFC-{} is not in a recognized range. Known ranges: 9000-9099 (Core), 9100-9199 (Forge), 9200-9299 (Security)",
                                num
                            ),
                            related_information: None,
                            tags: None,
                            code_description: None,
                            data: None,
                        });
                    }
                }
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvnn_validation() {
        let content = "// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE";
        let diagnostics = check_nvnn_annotations(content);
        // Valid annotation should not produce errors
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_nvnn_extra_parts() {
        let content = "// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE_EXTRA";
        let diagnostics = check_nvnn_annotations(content);
        // Should warn about extra parts
        assert!(!diagnostics.is_empty());
    }
}
