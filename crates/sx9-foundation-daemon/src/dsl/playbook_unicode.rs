//! Unicode Playbook Format
//!
//! Defines the Unicode s-expression format for compressed playbook execution.
//! Supports 7-tier escalation: WASM → Microkernel → Kernel Crate → Multi-Crates → Containers → Firefly → Orb

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Escalation tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationTier {
    /// Tier 1: WASM microkernels
    Wasm = 1,
    /// Tier 2: Microkernel
    Microkernel = 2,
    /// Tier 3: Kernel crate (single crate)
    KernelCrate = 3,
    /// Tier 4: Multi-crates (orchestration)
    MultiCrates = 4,
    /// Tier 5: Containers (Docker/OrbStack)
    Containers = 5,
    /// Tier 6: Firefly
    Firefly = 6,
    /// Tier 7: Orb
    Orb = 7,
}

/// Unicode playbook step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodePlaybookStep {
    /// Step name
    pub name: String,
    /// Escalation tier
    pub tier: EscalationTier,
    /// Unicode operation trigger
    pub unicode_op: char,
    /// Tool or crate name
    pub tool: Option<String>,
    /// Target or parameters
    pub target: Option<String>,
    /// Dependencies (step names)
    pub depends_on: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Unicode playbook structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodePlaybook {
    /// Playbook name
    pub name: String,
    /// Version
    pub version: String,
    /// Description
    pub description: Option<String>,
    /// Trivariate hash (SCH-CUID-UUID)
    pub trivariate_hash: Option<TrivariateHash>,
    /// Escalation configuration
    pub escalation: EscalationConfig,
    /// Unicode assembly triggers
    pub unicode_assembly: UnicodeAssembly,
    /// Playbook steps
    pub steps: Vec<UnicodePlaybookStep>,
}

/// Trivariate hash components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrivariateHash {
    /// SCH (positions 1-16): Component identity
    pub sch: String,
    /// CUID (positions 17-32): Context
    pub cuid: String,
    /// UUID (positions 33-48): Persistence
    pub uuid: String,
}

/// Escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationConfig {
    /// Tier 1: WASM
    pub tier_1_wasm: Option<String>,
    /// Tier 2: Microkernel
    pub tier_2_microkernel: Option<String>,
    /// Tier 3: Kernel crate
    pub tier_3_kernel_crate: Option<String>,
    /// Tier 4: Multi-crates
    pub tier_4_multi_crates: Option<Vec<String>>,
    /// Tier 5: Containers
    pub tier_5_containers: Option<String>,
    /// Tier 6: Firefly
    pub tier_6_firefly: Option<String>,
    /// Tier 7: Orb
    pub tier_7_orb: Option<String>,
}

/// Unicode assembly triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeAssembly {
    /// Primary trigger
    pub primary_trigger: String,
    /// Escalation triggers
    pub escalation_triggers: Vec<String>,
}

impl UnicodePlaybook {
    /// Create new Unicode playbook
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            trivariate_hash: None,
            escalation: EscalationConfig {
                tier_1_wasm: None,
                tier_2_microkernel: None,
                tier_3_kernel_crate: None,
                tier_4_multi_crates: None,
                tier_5_containers: None,
                tier_6_firefly: None,
                tier_7_orb: None,
            },
            unicode_assembly: UnicodeAssembly {
                primary_trigger: String::new(),
                escalation_triggers: Vec::new(),
            },
            steps: Vec::new(),
        }
    }
    
    /// Add step to playbook
    pub fn add_step(&mut self, step: UnicodePlaybookStep) {
        self.steps.push(step);
    }
    
    /// Get steps for a specific tier
    pub fn steps_for_tier(&self, tier: EscalationTier) -> Vec<&UnicodePlaybookStep> {
        self.steps
            .iter()
            .filter(|step| step.tier == tier)
            .collect()
    }
    
    /// Validate playbook structure
    pub fn validate(&self) -> Result<(), String> {
        // Check that steps have valid dependencies
        let step_names: std::collections::HashSet<&str> = self.steps.iter().map(|s| s.name.as_str()).collect();
        
        for step in &self.steps {
            for dep in &step.depends_on {
                if !step_names.contains(dep.as_str()) {
                    return Err(format!("Step '{}' depends on unknown step '{}'", step.name, dep));
                }
            }
        }
        
        Ok(())
    }
}

/// Convert Unicode playbook to s-expression format
pub fn to_s_expression(playbook: &UnicodePlaybook) -> String {
    let mut expr = String::new();
    expr.push_str("(playbook\n");
    expr.push_str(&format!("  (name \"{}\")\n", playbook.name));
    expr.push_str(&format!("  (version \"{}\")\n", playbook.version));
    
    if let Some(ref hash) = playbook.trivariate_hash {
        expr.push_str("  (trivariate_hash\n");
        expr.push_str(&format!("    (sch \"{}\")\n", hash.sch));
        expr.push_str(&format!("    (cuid \"{}\")\n", hash.cuid));
        expr.push_str(&format!("    (uuid \"{}\")\n", hash.uuid));
        expr.push_str("  )\n");
    }
    
    expr.push_str("  (steps\n");
    for step in &playbook.steps {
        expr.push_str(&format!("    (step\n"));
        expr.push_str(&format!("      (name \"{}\")\n", step.name));
        expr.push_str(&format!("      (tier {})\n", step.tier as u8));
        expr.push_str(&format!("      (unicode_op \"\\u{{{:04X}}}\")\n", step.unicode_op as u32));
        
        if let Some(ref tool) = step.tool {
            expr.push_str(&format!("      (tool \"{}\")\n", tool));
        }
        
        if let Some(ref target) = step.target {
            expr.push_str(&format!("      (target \"{}\")\n", target));
        }
        
        if !step.depends_on.is_empty() {
            expr.push_str("      (depends_on");
            for dep in &step.depends_on {
                expr.push_str(&format!(" \"{}\"", dep));
            }
            expr.push_str(")\n");
        }
        
        expr.push_str("    )\n");
    }
    expr.push_str("  )\n");
    expr.push_str(")\n");
    
    expr
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unicode_playbook_creation() {
        let mut playbook = UnicodePlaybook::new("test-playbook".to_string(), "1.0".to_string());
        
        let step = UnicodePlaybookStep {
            name: "test-step".to_string(),
            tier: EscalationTier::Wasm,
            unicode_op: '\u{E900}',
            tool: Some("test-tool".to_string()),
            target: None,
            depends_on: Vec::new(),
            metadata: HashMap::new(),
        };
        
        playbook.add_step(step);
        assert_eq!(playbook.steps.len(), 1);
    }
    
    #[test]
    fn test_validate_playbook() {
        let mut playbook = UnicodePlaybook::new("test".to_string(), "1.0".to_string());
        
        let step1 = UnicodePlaybookStep {
            name: "step1".to_string(),
            tier: EscalationTier::Wasm,
            unicode_op: '\u{E900}',
            tool: None,
            target: None,
            depends_on: Vec::new(),
            metadata: HashMap::new(),
        };
        
        let step2 = UnicodePlaybookStep {
            name: "step2".to_string(),
            tier: EscalationTier::Microkernel,
            unicode_op: '\u{E901}',
            tool: None,
            target: None,
            depends_on: vec!["step1".to_string()],
            metadata: HashMap::new(),
        };
        
        playbook.add_step(step1);
        playbook.add_step(step2);
        
        assert!(playbook.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_dependency() {
        let mut playbook = UnicodePlaybook::new("test".to_string(), "1.0".to_string());
        
        let step = UnicodePlaybookStep {
            name: "step1".to_string(),
            tier: EscalationTier::Wasm,
            unicode_op: '\u{E900}',
            tool: None,
            target: None,
            depends_on: vec!["nonexistent".to_string()],
            metadata: HashMap::new(),
        };
        
        playbook.add_step(step);
        assert!(playbook.validate().is_err());
    }
}
