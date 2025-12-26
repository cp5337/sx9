//! Completion Provider
//!
//! Provides completions for:
//! - Skill IDs (code.generate, security.scan, etc.)
//! - Agent names (@forge, @sentinel, etc.)
//! - RFC references (RFC-9141, RFC-9142, etc.)
//! - N-V-N-N patterns

use ropey::Rope;
use tower_lsp::lsp_types::*;

use sx9_harness::SkillRegistry;

use crate::document::DocumentIndex;
use crate::rfc::RfcIndex;

/// Provide completions at position
pub fn complete(
    document: &Rope,
    position: Position,
    skill_registry: &SkillRegistry,
    rfc_index: &RfcIndex,
    _document_index: &DocumentIndex,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get current line and character
    let line_idx = position.line as usize;
    let char_idx = position.character as usize;

    let Some(line) = document.get_line(line_idx) else {
        return items;
    };

    let line_str = line.to_string();
    let prefix = if char_idx > 0 {
        &line_str[..char_idx.min(line_str.len())]
    } else {
        ""
    };

    // Check for agent mention (@)
    if prefix.ends_with('@') || prefix.contains("@") && !prefix.ends_with(' ') {
        items.extend(agent_completions());
    }

    // Check for skill ID completion
    if prefix.contains("skill") || prefix.ends_with('.') {
        items.extend(skill_completions(skill_registry));
    }

    // Check for RFC reference
    if prefix.to_uppercase().contains("RFC") || prefix.contains("rfc") {
        items.extend(rfc_completions(rfc_index));
    }

    // Check for N-V-N-N annotation
    if prefix.contains("//") {
        items.extend(nvnn_completions());
    }

    items
}

/// Agent name completions
fn agent_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "@forge".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Code generation agent".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Forge** - Code generation and implementation\n\n\
                       Capabilities: CodeGeneration, CodeReview, Architecture"
                    .to_string(),
            })),
            insert_text: Some("forge".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@axiom".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Mathematical analysis agent".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Axiom** - Mathematical analysis and optimization\n\n\
                       Capabilities: Analysis, Research, Architecture"
                    .to_string(),
            })),
            insert_text: Some("axiom".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@vector".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Strategic planning agent".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Vector** - Strategic planning and architecture\n\n\
                       Capabilities: Planning, Architecture, Research"
                    .to_string(),
            })),
            insert_text: Some("vector".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@sentinel".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Security analysis agent".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Sentinel** - Security analysis and threat assessment\n\n\
                       Capabilities: Security, CodeReview, Analysis"
                    .to_string(),
            })),
            insert_text: Some("sentinel".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@guardian".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Quality assurance agent".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Guardian** - Quality assurance and testing\n\n\
                       Capabilities: Testing, CodeReview, Infrastructure"
                    .to_string(),
            })),
            insert_text: Some("guardian".to_string()),
            ..Default::default()
        },
    ]
}

/// Skill ID completions
fn skill_completions(registry: &SkillRegistry) -> Vec<CompletionItem> {
    registry
        .list_all()
        .iter()
        .map(|skill| CompletionItem {
            label: skill.id.clone(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(skill.name.clone()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "**{}**\n\n{}\n\n**Category:** {:?}\n\n**SLO:** {}ms target, {}ms max",
                    skill.name,
                    skill.description,
                    skill.category,
                    skill.slo.target_latency_ms,
                    skill.slo.max_latency_ms
                ),
            })),
            insert_text: Some(skill.id.clone()),
            ..Default::default()
        })
        .collect()
}

/// RFC reference completions
fn rfc_completions(rfc_index: &RfcIndex) -> Vec<CompletionItem> {
    let mut items: Vec<_> = rfc_index
        .list_all()
        .iter()
        .map(|(name, _)| CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("RFC Document".to_string()),
            insert_text: Some(name.clone()),
            ..Default::default()
        })
        .collect();

    // Add common RFC suggestions
    let common_rfcs = [
        ("RFC-9141", "FORGE Assembly Line & QA Doctrine"),
        ("RFC-9142", "Semantic Drift Scoring & Gates"),
        ("RFC-9030", "Linear Gateway Service"),
        ("RFC-9050", "Agent Heartbeat System"),
    ];

    for (id, title) in common_rfcs {
        if !items.iter().any(|i| i.label == id) {
            items.push(CompletionItem {
                label: id.to_string(),
                kind: Some(CompletionItemKind::REFERENCE),
                detail: Some(title.to_string()),
                insert_text: Some(id.to_string()),
                ..Default::default()
            });
        }
    }

    items
}

/// N-V-N-N pattern completions
fn nvnn_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("N-V-N-N: Code generator scope".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**CLSGS Annex A.2 Behavioral Scope**\n\n\
                       - Role: FACTORY\n\
                       - Action: GENERATE\n\
                       - Constraint: RUST_CRATE\n\
                       - Object: SOURCE_CODE"
                    .to_string(),
            })),
            insert_text: Some("FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "// ANALYZER_REVIEW_SECURITY_VULNERABILITY".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("N-V-N-N: Security reviewer scope".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**CLSGS Annex A.2 Behavioral Scope**\n\n\
                       - Role: ANALYZER\n\
                       - Action: REVIEW\n\
                       - Constraint: SECURITY\n\
                       - Object: VULNERABILITY"
                    .to_string(),
            })),
            insert_text: Some("ANALYZER_REVIEW_SECURITY_VULNERABILITY".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "// ORCHESTRATOR_DISPATCH_AGENT_TASK".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("N-V-N-N: Task dispatcher scope".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**CLSGS Annex A.2 Behavioral Scope**\n\n\
                       - Role: ORCHESTRATOR\n\
                       - Action: DISPATCH\n\
                       - Constraint: AGENT\n\
                       - Object: TASK"
                    .to_string(),
            })),
            insert_text: Some("ORCHESTRATOR_DISPATCH_AGENT_TASK".to_string()),
            ..Default::default()
        },
    ]
}
