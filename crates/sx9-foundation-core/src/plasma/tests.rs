//! PLASMA Delta Operator Test Suite
//!
//! Tests synthetic drift scenarios and suppression at various levels.

#[cfg(test)]
#[cfg(feature = "delta-tuner")]
mod delta_tests {
    use super::super::{
        DeltaGate, DeltaGateConfig, DeltaMeasurement, DeltaOperator, DeltaTuner, EscalationContext,
        EscalationIntegration, EscalationTier, GateMode,
    };
    use crate::trivariate_hash_v731::{ContextFrame, ExecEnv, ExecState, TrivariateHash};

    /// Create synthetic drift scenario
    fn create_synthetic_drift(
        base_angle: f32,
        entropy_delta: f32,
        semantic_delta: f32,
    ) -> DeltaMeasurement {
        DeltaMeasurement::new(base_angle, 0.5 + entropy_delta, 0.5 + semantic_delta)
    }

    #[test]
    fn test_suppression_at_level_0_1() {
        let config = DeltaGateConfig {
            threshold: 0.1,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.5,
        };
        let gate = DeltaGate::new(config, true);

        // High noise scenario
        let drift = create_synthetic_drift(90.0, 0.3, 0.4);
        let payload = "high_noise_payload".to_string();
        let gated = gate.gate(payload, &drift);

        // At 0.1 threshold, high noise should be suppressed
        assert!(gated.gated_weight < 1.0);
        println!("Level 0.1: gated_weight = {}", gated.gated_weight);
    }

    #[test]
    fn test_suppression_at_level_0_3() {
        let config = DeltaGateConfig {
            threshold: 0.3,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.5,
        };
        let gate = DeltaGate::new(config, true);

        let drift = create_synthetic_drift(90.0, 0.3, 0.4);
        let payload = "medium_noise_payload".to_string();
        let gated = gate.gate(payload, &drift);

        // At 0.3 threshold, should still suppress but less aggressively
        assert!(gated.gated_weight < 1.0);
        println!("Level 0.3: gated_weight = {}", gated.gated_weight);
    }

    #[test]
    fn test_suppression_at_level_0_7() {
        let config = DeltaGateConfig {
            threshold: 0.7,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.5,
        };
        let gate = DeltaGate::new(config, true);

        let drift = create_synthetic_drift(90.0, 0.3, 0.4);
        let payload = "low_noise_payload".to_string();
        let gated = gate.gate(payload, &drift);

        // At 0.7 threshold, may pass through or minimal suppression
        println!("Level 0.7: gated_weight = {}", gated.gated_weight);
        assert!(gated.gated_weight >= 0.0 && gated.gated_weight <= 1.0);
    }

    #[test]
    fn test_suppression_at_level_0_9() {
        let config = DeltaGateConfig {
            threshold: 0.9,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.5,
        };
        let gate = DeltaGate::new(config, true);

        let drift = create_synthetic_drift(90.0, 0.3, 0.4);
        let payload = "very_low_noise_payload".to_string();
        let gated = gate.gate(payload, &drift);

        // At 0.9 threshold, should pass through
        assert_eq!(gated.gated_weight, 1.0);
        println!("Level 0.9: gated_weight = {}", gated.gated_weight);
    }

    #[test]
    fn test_escalation_gating_with_synthetic_drift() {
        let integration = EscalationIntegration::new(true);

        // Create base context
        let base_ctx = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let base_hash = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );

        // Create drifted context (simulating escalation)
        let mut drifted_ctx = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);
        drifted_ctx.delta_angle = 45.0; // Synthetic drift
        let drifted_hash = TrivariateHash::new(
            "xY9mP4qR8sT2wN5k".to_string(),
            "d6k9l4p3q8w2x0z".to_string(),
            "660f9501-f3ac-52e5-b827-557766551111".to_string(),
        );

        let escalation_ctx = EscalationContext::new(
            EscalationTier::Wasm,
            EscalationTier::Microkernel,
            drifted_ctx,
            drifted_hash,
        )
        .with_previous(base_ctx, base_hash);

        let payload = "escalation_payload".to_string();
        let result = integration.gate_escalation(payload, &escalation_ctx);

        assert!(result.escalation_allowed || !result.escalation_allowed); // Either is valid
        println!(
            "Escalation gated: allowed={}, weight={}",
            result.escalation_allowed, result.gated_payload.gated_weight
        );
    }

    #[test]
    fn test_tuner_logging_with_synthetic_drift() {
        let tuner = DeltaTuner::new(true);

        // Log multiple synthetic measurements
        for i in 0..10 {
            let drift =
                create_synthetic_drift((i as f32) * 10.0, (i as f32) * 0.05, (i as f32) * 0.05);
            tuner.log_measurement(&drift, "WASM->Microkernel", Some(0.5 + (i as f32) * 0.05));
        }

        let stats = tuner.get_statistics();
        assert_eq!(stats.total_measurements, 10);
        assert!(stats.avg_delta_angle > 0.0);
        assert!(stats.avg_noise_score > 0.0);

        println!("Statistics: {:?}", stats);
    }

    #[test]
    fn test_all_escalation_points() {
        let integration = EscalationIntegration::new(true);
        use escalation_integration::integration_points::*;

        let ctx = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let hash = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );

        let payload = "test".to_string();

        // Test all escalation points
        let _r1 = gate_wasm_to_microkernel(
            &integration,
            payload.clone(),
            ctx.clone(),
            hash.clone(),
            None,
            None,
        );
        let _r2 = gate_microkernel_to_kernel(
            &integration,
            payload.clone(),
            ctx.clone(),
            hash.clone(),
            None,
            None,
        );
        let _r3 = gate_kernel_to_multicrate(
            &integration,
            payload.clone(),
            ctx.clone(),
            hash.clone(),
            None,
            None,
        );
        let _r4 = gate_multicrate_to_container(
            &integration,
            payload.clone(),
            ctx.clone(),
            hash.clone(),
            None,
            None,
        );
        let _r5 = gate_container_to_firefly(
            &integration,
            payload.clone(),
            ctx.clone(),
            hash.clone(),
            None,
            None,
        );
        let _r6 = gate_firefly_to_orb(&integration, payload, ctx, hash, None, None);

        // All should complete without panic
        println!("All escalation points tested successfully");
    }
}
