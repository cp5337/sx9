//! PLASMA Delta Gate Integration Points
//!
//! Inserts Delta gates between escalation tiers:
//! WASM → Microkernel → Kernel → MultiCrate → Container → Firefly → Orb

#[cfg(feature = "delta-tuner")]
use super::{DeltaOperator, DeltaGate, DeltaTuner, DeltaMeasurement, GatedPayload};
use crate::trivariate_hash_v731::{ContextFrame, TrivariateHash, ExecEnv};

/// Escalation tier identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscalationTier {
    Wasm,
    Microkernel,
    Kernel,
    MultiCrate,
    Container,
    Firefly,
    Orb,
}

impl EscalationTier {
    /// Get tier name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            EscalationTier::Wasm => "WASM",
            EscalationTier::Microkernel => "Microkernel",
            EscalationTier::Kernel => "Kernel",
            EscalationTier::MultiCrate => "MultiCrate",
            EscalationTier::Container => "Container",
            EscalationTier::Firefly => "Firefly",
            EscalationTier::Orb => "Orb",
        }
    }

    /// Get escalation path string
    pub fn escalation_path(from: Self, to: Self) -> String {
        format!("{}->{}", from.as_str(), to.as_str())
    }
}

/// Escalation context with delta gating
#[cfg(feature = "delta-tuner")]
pub struct EscalationContext {
    pub current_tier: EscalationTier,
    pub target_tier: EscalationTier,
    pub context: ContextFrame,
    pub hash: TrivariateHash,
    pub previous_context: Option<ContextFrame>,
    pub previous_hash: Option<TrivariateHash>,
}

#[cfg(feature = "delta-tuner")]
impl EscalationContext {
    /// Create new escalation context
    pub fn new(
        current_tier: EscalationTier,
        target_tier: EscalationTier,
        context: ContextFrame,
        hash: TrivariateHash,
    ) -> Self {
        Self {
            current_tier,
            target_tier,
            context,
            hash,
            previous_context: None,
            previous_hash: None,
        }
    }

    /// Set previous context/hash for delta computation
    pub fn with_previous(
        mut self,
        previous_context: ContextFrame,
        previous_hash: TrivariateHash,
    ) -> Self {
        self.previous_context = Some(previous_context);
        self.previous_hash = Some(previous_hash);
        self
    }
}

/// Escalation gate result
#[cfg(feature = "delta-tuner")]
pub struct EscalationGateResult<T> {
    pub gated_payload: GatedPayload<T>,
    pub escalation_allowed: bool,
    pub escalation_tier: String,
}

/// PLASMA Escalation Integration
#[cfg(feature = "delta-tuner")]
pub struct EscalationIntegration {
    operator: DeltaOperator,
    gate: DeltaGate,
    tuner: DeltaTuner,
}

#[cfg(feature = "delta-tuner")]
impl EscalationIntegration {
    /// Create new escalation integration
    pub fn new(enabled: bool) -> Self {
        Self {
            operator: DeltaOperator::new(enabled),
            gate: DeltaGate::with_default_config(enabled),
            tuner: DeltaTuner::new(enabled),
        }
    }

    /// Gate escalation between tiers
    pub fn gate_escalation<T: Clone>(
        &self,
        payload: T,
        escalation_ctx: &EscalationContext,
    ) -> EscalationGateResult<T> {
        let escalation_path = EscalationTier::escalation_path(
            escalation_ctx.current_tier,
            escalation_ctx.target_tier,
        );

        // Compute delta if previous context exists
        let delta = if let (Some(prev_ctx), Some(prev_hash)) = (
            &escalation_ctx.previous_context,
            &escalation_ctx.previous_hash,
        ) {
            let measurement = self.operator.measure_delta(
                prev_ctx,
                &escalation_ctx.context,
                prev_hash,
                &escalation_ctx.hash,
            );

            // Log measurement
            self.tuner.log_measurement(&measurement, &escalation_path, None);

            measurement
        } else {
            // No previous context: create zero delta
            DeltaMeasurement::new(0.0, 0.0, 0.0)
        };

        // Gate the payload
        let gated = self.gate.gate(payload, &delta);

        // Log gated weight
        self.tuner.log_measurement(&delta, &escalation_path, Some(gated.gated_weight));

        EscalationGateResult {
            gated_payload: gated.clone(),
            escalation_allowed: gated.passed && gated.gated_weight > 0.0,
            escalation_tier: escalation_path,
        }
    }

    /// Get tuner for diagnostics
    pub fn tuner(&self) -> &DeltaTuner {
        &self.tuner
    }

    /// Get gate for configuration
    pub fn gate_mut(&mut self) -> &mut DeltaGate {
        &mut self.gate
    }
}

#[cfg(feature = "delta-tuner")]
impl Default for EscalationIntegration {
    fn default() -> Self {
        Self::new(true)
    }
}

/// Integration helper functions for each escalation point
#[cfg(feature = "delta-tuner")]
pub mod integration_points {
    use super::*;

    /// Gate WASM → Microkernel escalation
    pub fn gate_wasm_to_microkernel<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::Wasm,
            EscalationTier::Microkernel,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }

    /// Gate Microkernel → Kernel escalation
    pub fn gate_microkernel_to_kernel<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::Microkernel,
            EscalationTier::Kernel,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }

    /// Gate Kernel → MultiCrate escalation
    pub fn gate_kernel_to_multicrate<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::Kernel,
            EscalationTier::MultiCrate,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }

    /// Gate MultiCrate → Container escalation
    pub fn gate_multicrate_to_container<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::MultiCrate,
            EscalationTier::Container,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }

    /// Gate Container → Firefly escalation
    pub fn gate_container_to_firefly<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::Container,
            EscalationTier::Firefly,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }

    /// Gate Firefly → Orb escalation
    pub fn gate_firefly_to_orb<T: Clone>(
        integration: &EscalationIntegration,
        payload: T,
        ctx: ContextFrame,
        hash: TrivariateHash,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> EscalationGateResult<T> {
        let escalation_ctx = EscalationContext::new(
            EscalationTier::Firefly,
            EscalationTier::Orb,
            ctx,
            hash,
        )
        .with_previous_optional(prev_ctx, prev_hash);

        integration.gate_escalation(payload, &escalation_ctx)
    }
}

#[cfg(feature = "delta-tuner")]
impl EscalationContext {
    fn with_previous_optional(
        mut self,
        prev_ctx: Option<ContextFrame>,
        prev_hash: Option<TrivariateHash>,
    ) -> Self {
        if let (Some(ctx), Some(hash)) = (prev_ctx, prev_hash) {
            self.previous_context = Some(ctx);
            self.previous_hash = Some(hash);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trivariate_hash_v731::{ExecEnv, ExecState};

    #[cfg(feature = "delta-tuner")]
    #[test]
    fn test_escalation_gating() {
        let integration = EscalationIntegration::new(true);

        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let ctx2 = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);

        let hash1 = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );

        let hash2 = TrivariateHash::new(
            "xY9mP4qR8sT2wN5k".to_string(),
            "d6k9l4p3q8w2x0z".to_string(),
            "660f9501-f3ac-52e5-b827-557766551111".to_string(),
        );

        let escalation_ctx = EscalationContext::new(
            EscalationTier::Wasm,
            EscalationTier::Microkernel,
            ctx2,
            hash2,
        )
        .with_previous(ctx1, hash1);

        let payload = "test_payload".to_string();
        let result = integration.gate_escalation(payload, &escalation_ctx);

        assert!(result.escalation_tier.contains("WASM"));
        assert!(result.escalation_tier.contains("Microkernel"));
    }
}


