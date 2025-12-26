//! Hover Provider
//!
//! Provides hover documentation for:
//! - Agent names
//! - Skill IDs
//! - RFC references
//! - N-V-N-N annotations

use ropey::Rope;
use tower_lsp::lsp_types::*;

use sx9_harness::SkillRegistry;

use crate::rfc::RfcIndex;

/// Get hover info at position
pub fn hover_info(
    document: &Rope,
    position: Position,
    skill_registry: &SkillRegistry,
    rfc_index: &RfcIndex,
) -> Option<Hover> {
    let line_idx = position.line as usize;
    let char_idx = position.character as usize;

    let line = document.get_line(line_idx)?;
    let line_str = line.to_string();

    // Extract word at position
    let word = extract_word_at(&line_str, char_idx);

    if word.is_empty() {
        return None;
    }

    // Check for agent mention
    if word.starts_with('@') || line_str.contains(&format!("@{}", word)) {
        return agent_hover(&word.trim_start_matches('@'));
    }

    // Check for skill ID
    if let Some(skill) = skill_registry.get(&word) {
        return Some(skill_hover(skill));
    }

    // Check for RFC reference
    if word.to_uppercase().starts_with("RFC-") {
        return rfc_hover(&word, rfc_index);
    }

    // Check for N-V-N-N annotation
    if line_str.contains("//") && word.chars().all(|c| c.is_uppercase() || c == '_') {
        return nvnn_hover(&line_str);
    }

    None
}

/// Extract word at character position
fn extract_word_at(line: &str, char_idx: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if char_idx >= len {
        return String::new();
    }

    let mut start = char_idx;
    let mut end = char_idx;

    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }

    while end < len && is_word_char(chars[end]) {
        end += 1;
    }

    chars[start..end].iter().collect()
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '@'
}

/// Hover for agent names
fn agent_hover(name: &str) -> Option<Hover> {
    let info = match name.to_lowercase().as_str() {
        "forge" => Some((
            "Forge",
            "Code Generation Agent",
            "Primary agent for implementing code from specifications.\n\n\
             **Capabilities:**\n\
             - CodeGeneration\n\
             - CodeReview\n\
             - Architecture\n\n\
             **N-V-N-N:** FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE\n\n\
             **Provider:** Claude 3.5 Sonnet",
        )),
        "axiom" => Some((
            "Axiom",
            "Mathematical Analysis Agent",
            "Specialized in mathematical optimization and analysis.\n\n\
             **Capabilities:**\n\
             - Analysis\n\
             - Research\n\
             - Architecture\n\n\
             **N-V-N-N:** ANALYZER_COMPUTE_ALGORITHM_OPTIMIZATION\n\n\
             **Provider:** Claude 3.5 Sonnet",
        )),
        "vector" => Some((
            "Vector",
            "Strategic Planning Agent",
            "Handles architectural decisions and strategic planning.\n\n\
             **Capabilities:**\n\
             - Planning\n\
             - Architecture\n\
             - Research\n\n\
             **N-V-N-N:** ORCHESTRATOR_PLAN_SYSTEM_ARCHITECTURE\n\n\
             **Provider:** Claude 3.5 Sonnet",
        )),
        "sentinel" => Some((
            "Sentinel",
            "Security Analysis Agent",
            "Security-focused agent for threat assessment and auditing.\n\n\
             **Capabilities:**\n\
             - Security\n\
             - CodeReview\n\
             - Analysis\n\n\
             **N-V-N-N:** ANALYZER_REVIEW_SECURITY_VULNERABILITY\n\n\
             **Provider:** Claude 3.5 Sonnet",
        )),
        "guardian" => Some((
            "Guardian",
            "Quality Assurance Agent",
            "Ensures code quality and testing coverage.\n\n\
             **Capabilities:**\n\
             - Testing\n\
             - CodeReview\n\
             - Infrastructure\n\n\
             **N-V-N-N:** VALIDATOR_TEST_QUALITY_COVERAGE\n\n\
             **Provider:** Claude 3.5 Sonnet",
        )),
        _ => None,
    };

    info.map(|(name, title, desc)| Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("## {} - {}\n\n{}", name, title, desc),
        }),
        range: None,
    })
}

/// Hover for skill IDs
fn skill_hover(skill: &sx9_harness::Skill) -> Hover {
    let input_schema = serde_json::to_string_pretty(&skill.input_schema)
        .unwrap_or_else(|_| "{}".to_string());
    let output_schema = serde_json::to_string_pretty(&skill.output_schema)
        .unwrap_or_else(|_| "{}".to_string());

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!(
                "## {} (`{}`)\n\n\
                 {}\n\n\
                 **Category:** {:?}\n\n\
                 **Version:** {}\n\n\
                 ### SLO\n\
                 - Target: {}ms\n\
                 - Max: {}ms\n\
                 - Success Rate: {:.0}%\n\n\
                 ### Input Schema\n\
                 ```json\n{}\n```\n\n\
                 ### Output Schema\n\
                 ```json\n{}\n```",
                skill.name,
                skill.id,
                skill.description,
                skill.category,
                skill.version,
                skill.slo.target_latency_ms,
                skill.slo.max_latency_ms,
                skill.slo.target_success_rate * 100.0,
                input_schema,
                output_schema
            ),
        }),
        range: None,
    }
}

/// Hover for RFC references
fn rfc_hover(rfc_id: &str, rfc_index: &RfcIndex) -> Option<Hover> {
    // Try to get from index
    if let Some(info) = rfc_index.get_info(rfc_id) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info,
            }),
            range: None,
        });
    }

    // Fallback to known RFCs
    let info = match rfc_id.to_uppercase().as_str() {
        "RFC-9141" => Some(
            "## RFC-9141: FORGE Assembly Line & QA Doctrine\n\n\
             Defines the dual-heartbeat QA system and assembly line prompt methodology.\n\n\
             **Key Concepts:**\n\
             - Prompts are assembled, not authored\n\
             - Variable selection precedes generation\n\
             - Four QA gates: Static, Arch, Pattern, Semantic",
        ),
        "RFC-9142" => Some(
            "## RFC-9142: Semantic Drift Scoring & Gates\n\n\
             Defines drift vectors and governance gates for semantic quality.\n\n\
             **Drift Vectors:**\n\
             - Role, Constraint, Coupling, Authority, Pattern\n\n\
             **Gates:**\n\
             - Observe, Warn, Gate, Escalate",
        ),
        "RFC-9030" => Some(
            "## RFC-9030: Linear Gateway Service\n\n\
             Defines the integration between SX9 agents and Linear project management.\n\n\
             **Features:**\n\
             - Agent team member registration\n\
             - Webhook handling\n\
             - Issue-to-task routing",
        ),
        _ => None,
    };

    info.map(|i| Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: i.to_string(),
        }),
        range: None,
    })
}

/// Hover for N-V-N-N annotations
fn nvnn_hover(line: &str) -> Option<Hover> {
    let regex = regex::Regex::new(r"//\s*([A-Z_]+)_([A-Z_]+)_([A-Z_]+)_([A-Z_]+)").unwrap();

    regex.captures(line).map(|caps| {
        let role = caps.get(1).map(|m| m.as_str()).unwrap_or("?");
        let action = caps.get(2).map(|m| m.as_str()).unwrap_or("?");
        let constraint = caps.get(3).map(|m| m.as_str()).unwrap_or("?");
        let object = caps.get(4).map(|m| m.as_str()).unwrap_or("?");

        Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "## N-V-N-N Behavioral Scope\n\n\
                     *Per CLSGS Annex A.2*\n\n\
                     | Part | Value | Description |\n\
                     |------|-------|-------------|\n\
                     | **N** (Role) | `{}` | The role this component fulfills |\n\
                     | **V** (Action) | `{}` | The bounded action scope |\n\
                     | **N** (Constraint) | `{}` | The constraint ownership |\n\
                     | **N** (Object) | `{}` | The object domain |\n\n\
                     This annotation declares the semantic boundary for drift detection.",
                    role, action, constraint, object
                ),
            }),
            range: None,
        }
    })
}
