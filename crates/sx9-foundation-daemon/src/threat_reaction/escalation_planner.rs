//! Escalation Planner
//!
//! Plans escalation path through 7 tiers:
//! WASM → Microkernel → Kernel → MultiCrate → Container → Firefly → Orb

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::dsl::playbook_unicode::{EscalationTier, UnicodePlaybook, UnicodePlaybookStep};
use crate::threat_reaction::recognize::ThreatSeverity;

/// Escalation plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPlan {
    pub steps: Vec<EscalationStep>,
    pub tier_path: Vec<EscalationTier>,
}

impl EscalationPlan {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            tier_path: vec![],
        }
    }

    pub fn add_step(&mut self, step: UnicodePlaybookStep, tier: EscalationTier) {
        self.steps.push(EscalationStep {
            step,
            tier,
            escalation_trigger: None,
            delta_gate: None,
        });
        if !self.tier_path.contains(&tier) {
            self.tier_path.push(tier);
        }
    }

    pub fn add_escalation(&mut self, escalation: EscalationStep) {
        self.steps.push(escalation);
    }

    pub fn tiers(&self) -> &[EscalationTier] {
        &self.tier_path
    }

    pub fn path(&self) -> Vec<EscalationTier> {
        self.tier_path.clone()
    }
}

impl Default for EscalationPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// Escalation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationStep {
    pub step: UnicodePlaybookStep,
    pub tier: EscalationTier,
    pub escalation_trigger: Option<EscalationTrigger>,
    pub delta_gate: Option<DeltaGate>,
}

/// Escalation trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationTrigger {
    ResourceConstraint,
    Timeout,
    Complexity,
    Dependency,
}

/// Delta gate (for noise filtering)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaGate {
    pub noise_score: f64,
    pub threshold: f64,
    pub passed: bool,
}

/// Escalation tier selector
pub struct EscalationTierSelector;

impl Default for EscalationTierSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl EscalationTierSelector {
    pub fn new() -> Self {
        Self
    }

    pub async fn select_tier(
        &self,
        step: &UnicodePlaybookStep,
        severity: &ThreatSeverity,
    ) -> Result<EscalationTier> {
        // Start at WASM for simple operations
        // Escalate based on complexity and severity
        match severity {
            ThreatSeverity::Critical => {
                // Critical threats may need higher tiers
                if step.depends_on.len() > 3 {
                    Ok(EscalationTier::Containers)
                } else if step.depends_on.len() > 1 {
                    Ok(EscalationTier::MultiCrates)
                } else {
                    Ok(EscalationTier::KernelCrate)
                }
            }
            ThreatSeverity::High => {
                if step.depends_on.len() > 2 {
                    Ok(EscalationTier::MultiCrates)
                } else {
                    Ok(EscalationTier::KernelCrate)
                }
            }
            ThreatSeverity::Medium => {
                if step.depends_on.len() > 1 {
                    Ok(EscalationTier::KernelCrate)
                } else {
                    Ok(EscalationTier::Microkernel)
                }
            }
            ThreatSeverity::Low => Ok(EscalationTier::Wasm),
        }
    }
}

/// Resource analyzer
pub struct ResourceAnalyzer;

impl Default for ResourceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

/// Escalation Planner
pub struct EscalationPlanner {
    tier_selector: EscalationTierSelector,
    _resource_analyzer: ResourceAnalyzer,
}

impl Default for EscalationPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl EscalationPlanner {
    pub fn new() -> Self {
        Self {
            tier_selector: EscalationTierSelector::new(),
            _resource_analyzer: ResourceAnalyzer::new(),
        }
    }

    /// Plan escalation path through 7 tiers
    pub async fn plan(
        &self,
        playbook: &UnicodePlaybook,
        severity: &ThreatSeverity,
    ) -> Result<EscalationPlan> {
        info!(
            "Planning escalation for playbook with {} steps",
            playbook.steps.len()
        );

        let mut plan = EscalationPlan::new();

        // Start at Tier 1 (WASM) for simple operations
        for step in &playbook.steps {
            let tier = self.tier_selector.select_tier(step, severity).await?;

            // Check if escalation needed (compare by tier value)
            let tier_value = tier as u8;
            if tier_value > EscalationTier::Wasm as u8 {
                plan.add_escalation(EscalationStep {
                    step: step.clone(),
                    tier,
                    escalation_trigger: Some(EscalationTrigger::Complexity),
                    delta_gate: Some(self.evaluate_delta_gate(step).await),
                });
            } else {
                plan.add_step(step.clone(), tier);
            }
        }

        debug!("Planned escalation through tiers: {:?}", plan.tier_path);
        Ok(plan)
    }

    /// Evaluate delta gate for step
    async fn evaluate_delta_gate(&self, _step: &UnicodePlaybookStep) -> DeltaGate {
        // TODO: Implement actual delta gate evaluation
        // For now, return a passed gate
        DeltaGate {
            noise_score: 0.3,
            threshold: 0.5,
            passed: true,
        }
    }
}
