//! ECS Systems for Plasma
//!
//! Defines systems that operate on Plasma entities

use crate::components::*;
use anyhow::Result;

/// Plasma state update system
pub struct PlasmaUpdateSystem;

impl PlasmaUpdateSystem {
    pub async fn run(components: &mut [PlasmaComponent]) -> Result<()> {
        // Update all plasma components
        for component in components {
            // Apply state transitions, decay, etc.
            // This is where Plasma state logic would go
        }
        Ok(())
    }
}

/// Crystal resonance calculation system
pub struct CrystalResonanceSystem;

impl CrystalResonanceSystem {
    pub async fn run(
        plasma_components: &[PlasmaComponent],
        resonance_components: &mut [CrystalResonanceComponent],
    ) -> Result<()> {
        // Calculate crystal resonance for each entity
        for (plasma, resonance) in plasma_components
            .iter()
            .zip(resonance_components.iter_mut())
        {
            // Calculate resonance based on plasma state
            resonance.ring_strength = (plasma.delta_angle as f32 / 65535.0) * 2.0 - 1.0;
        }
        Ok(())
    }
}

/// SDT gate evaluation system
pub struct SdtGateSystem;

impl SdtGateSystem {
    pub async fn run(
        plasma_components: &[PlasmaComponent],
        gate_components: &mut [SdtGateComponent],
    ) -> Result<()> {
        // Evaluate SDT gates
        for (plasma, gate) in plasma_components.iter().zip(gate_components.iter_mut()) {
            gate.state = plasma.sdt_state;
            gate.current_value = (plasma.entropy as f32 / 1000.0).min(1.0);
        }
        Ok(())
    }
}

// ANN Observer Systems (RFC-9114 Rev 1.1)
// Systems for neural network observation and prediction

/// ANN observer system - watches entity patterns
pub struct AnnObserverSystem;

impl AnnObserverSystem {
    /// Observer captures plasma state patterns into ANN input buffer
    pub async fn observe(
        plasma_components: &[PlasmaComponent],
        observers: &mut [AnnObserverComponent],
    ) -> Result<()> {
        for (plasma, observer) in plasma_components.iter().zip(observers.iter_mut()) {
            // Only observe in active or learning mode
            if observer.mode == AnnObserverMode::Passive {
                continue;
            }

            // Observer extracts features from plasma state
            observer.pattern_buffer.clear();
            observer
                .pattern_buffer
                .push(plasma.delta_angle as f32 / 65535.0);
            observer
                .pattern_buffer
                .push(plasma.entropy as f32 / u32::MAX as f32);
            observer
                .pattern_buffer
                .push(plasma.sdt_state as u8 as f32 / 3.0);
            observer
                .pattern_buffer
                .push(plasma.crystal_family as u8 as f32 / 6.0);

            // Observer updates observation hash via murmur3
            let hash_input = observer
                .pattern_buffer
                .iter()
                .map(|f| f.to_bits())
                .fold(0u64, |acc, bits| {
                    acc.wrapping_mul(31).wrapping_add(bits as u64)
                });
            observer.last_observation_hash = hash_input;
            observer.observation_count += 1;
        }
        Ok(())
    }

    /// Set observer mode for all observers
    pub fn set_mode(observers: &mut [AnnObserverComponent], mode: AnnObserverMode) {
        for observer in observers {
            observer.mode = mode;
        }
    }
}

/// ANN forward propagation system
pub struct AnnForwardSystem;

impl AnnForwardSystem {
    /// Forward pass computes neuron activations from input layer
    pub async fn forward(
        input: &[f32],
        neurons: &mut [AnnNeuronComponent],
        activation_fn: ActivationFunction,
    ) -> Result<Vec<f32>> {
        let mut output = Vec::with_capacity(neurons.len());

        for neuron in neurons.iter_mut() {
            // Neuron computes weighted sum of inputs plus bias
            let weighted_sum: f32 = neuron
                .weights
                .iter()
                .zip(input.iter())
                .map(|(w, x)| w * x)
                .sum::<f32>()
                + neuron.bias;

            // System applies activation function to weighted sum
            neuron.activation = match activation_fn {
                ActivationFunction::ReLU => weighted_sum.max(0.0),
                ActivationFunction::Sigmoid => 1.0 / (1.0 + (-weighted_sum).exp()),
                ActivationFunction::Tanh => weighted_sum.tanh(),
                ActivationFunction::Softmax => weighted_sum, // Applied after all neurons
                ActivationFunction::Linear => weighted_sum,
            };

            output.push(neuron.activation);
        }

        // Softmax normalization applied across output layer
        if activation_fn == ActivationFunction::Softmax {
            let max_val = output.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp_sum: f32 = output.iter().map(|x| (x - max_val).exp()).sum();
            for val in &mut output {
                *val = (*val - max_val).exp() / exp_sum;
            }
        }

        Ok(output)
    }
}

/// ANN prediction system
pub struct AnnPredictionSystem;

impl AnnPredictionSystem {
    /// Prediction system generates output from observer patterns
    pub async fn predict(
        observers: &[AnnObserverComponent],
        predictions: &mut [AnnPredictionComponent],
    ) -> Result<()> {
        for (observer, prediction) in observers.iter().zip(predictions.iter_mut()) {
            // Only predict from active observers with sufficient data
            if observer.mode != AnnObserverMode::Predictive || observer.observation_count < 10 {
                continue;
            }

            // Prediction extracts confidence from pattern buffer
            prediction.output_vector = observer.pattern_buffer.clone();
            prediction.confidence = observer.confidence_score;
            prediction.prediction_id = observer.last_observation_hash;
            prediction.timestamp_ns = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
        }
        Ok(())
    }
}

/// ANN training system (backpropagation placeholder)
pub struct AnnTrainingSystem;

impl AnnTrainingSystem {
    /// Training adjusts weights based on error gradient
    pub async fn train(
        neurons: &mut [AnnNeuronComponent],
        target: &[f32],
        learning_rate: f32,
    ) -> Result<f32> {
        let mut total_error = 0.0f32;

        for (neuron, &target_val) in neurons.iter_mut().zip(target.iter()) {
            // Trainer computes error between activation and target
            let error = target_val - neuron.activation;
            total_error += error * error;

            // Trainer adjusts bias using gradient descent
            neuron.bias += learning_rate * error;

            // Trainer updates weights via delta rule
            for weight in &mut neuron.weights {
                *weight += learning_rate * error * neuron.activation;
            }
        }

        Ok(total_error / neurons.len() as f32)
    }
}
