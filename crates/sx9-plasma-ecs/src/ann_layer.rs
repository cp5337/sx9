//! ANN Layer - Neural Network Observer Integration for PLASMA-ECS
//!
//! Provides high-level ANN observer management for the ECS world.
//! Per RFC-9114 Rev 1.1 - ANN observer mode for neural retrofit.

use crate::components::*;
use crate::systems::*;
use anyhow::Result;
use std::collections::HashMap;

/// ANN observer world - manages neural network entities
pub struct AnnObserverWorld {
    observers: Vec<AnnObserverComponent>,
    neurons: HashMap<u32, Vec<AnnNeuronComponent>>,
    layers: Vec<AnnLayerComponent>,
    predictions: Vec<AnnPredictionComponent>,
    config: AnnConfig,
}

/// ANN configuration for observer world
#[derive(Debug, Clone)]
pub struct AnnConfig {
    pub default_mode: AnnObserverMode,
    pub learning_rate: f32,
    pub pattern_buffer_size: usize,
    pub confidence_threshold: f32,
}

impl Default for AnnConfig {
    fn default() -> Self {
        Self {
            default_mode: AnnObserverMode::Passive,
            learning_rate: 0.01,
            pattern_buffer_size: 128,
            confidence_threshold: 0.7,
        }
    }
}

impl AnnObserverWorld {
    /// Create new ANN observer world with config
    pub fn new(config: AnnConfig) -> Self {
        Self {
            observers: Vec::new(),
            neurons: HashMap::new(),
            layers: Vec::new(),
            predictions: Vec::new(),
            config,
        }
    }

    /// Create observer world with default configuration
    pub fn with_defaults() -> Self {
        Self::new(AnnConfig::default())
    }

    /// World adds new observer entity with unique ID
    pub fn add_observer(&mut self, observer_id: String) -> usize {
        let observer = AnnObserverComponent {
            observer_id,
            mode: self.config.default_mode,
            observation_count: 0,
            last_observation_hash: 0,
            confidence_score: 0.0,
            pattern_buffer: Vec::with_capacity(self.config.pattern_buffer_size),
        };
        self.observers.push(observer);
        self.predictions.push(AnnPredictionComponent::default());
        self.observers.len() - 1
    }

    /// World creates neural layer with specified dimensions
    pub fn add_layer(
        &mut self,
        layer_type: AnnLayerType,
        input_size: usize,
        output_size: usize,
        activation_fn: ActivationFunction,
    ) -> u32 {
        let layer_id = self.layers.len() as u32;

        // Layer configuration tracks topology
        let layer = AnnLayerComponent {
            layer_id,
            layer_type,
            input_size,
            output_size,
            activation_fn,
        };
        self.layers.push(layer);

        // Layer spawns neurons with random initialization
        let mut layer_neurons = Vec::with_capacity(output_size);
        for neuron_id in 0..output_size {
            let neuron = AnnNeuronComponent {
                layer_id,
                neuron_id: neuron_id as u32,
                activation: 0.0,
                bias: 0.0,
                weights: vec![0.0; input_size],
            };
            layer_neurons.push(neuron);
        }
        self.neurons.insert(layer_id, layer_neurons);

        layer_id
    }

    /// World initializes weights with random values
    pub fn initialize_weights(&mut self, seed: u64) {
        let mut rng_state = seed;
        for neurons in self.neurons.values_mut() {
            for neuron in neurons.iter_mut() {
                // Simple LCG for weight initialization
                for weight in &mut neuron.weights {
                    rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    *weight = ((rng_state >> 33) as f32 / u32::MAX as f32) * 2.0 - 1.0;
                }
                rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
                neuron.bias = ((rng_state >> 33) as f32 / u32::MAX as f32) * 0.1;
            }
        }
    }

    /// World sets observer mode for all entities
    pub fn set_observer_mode(&mut self, mode: AnnObserverMode) {
        AnnObserverSystem::set_mode(&mut self.observers, mode);
    }

    /// World runs observation pass on plasma components
    pub async fn observe(&mut self, plasma_components: &[PlasmaComponent]) -> Result<()> {
        AnnObserverSystem::observe(plasma_components, &mut self.observers).await
    }

    /// World executes forward propagation through network
    pub async fn forward(&mut self, input: &[f32]) -> Result<Vec<f32>> {
        let mut current_input = input.to_vec();

        // Forward pass propagates through each layer sequentially
        for layer in &self.layers {
            if let Some(neurons) = self.neurons.get_mut(&layer.layer_id) {
                current_input =
                    AnnForwardSystem::forward(&current_input, neurons, layer.activation_fn).await?;
            }
        }

        Ok(current_input)
    }

    /// World generates predictions from observer patterns
    pub async fn predict(&mut self) -> Result<()> {
        AnnPredictionSystem::predict(&self.observers, &mut self.predictions).await
    }

    /// World trains network on target output
    pub async fn train(&mut self, target: &[f32]) -> Result<f32> {
        let mut total_error = 0.0;

        // Training proceeds backward through layers
        for layer in self.layers.iter().rev() {
            if let Some(neurons) = self.neurons.get_mut(&layer.layer_id) {
                let error =
                    AnnTrainingSystem::train(neurons, target, self.config.learning_rate).await?;
                total_error += error;
            }
        }

        Ok(total_error / self.layers.len().max(1) as f32)
    }

    /// World returns observer count
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// World returns layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// World retrieves observer by index
    pub fn get_observer(&self, index: usize) -> Option<&AnnObserverComponent> {
        self.observers.get(index)
    }

    /// World retrieves prediction by index
    pub fn get_prediction(&self, index: usize) -> Option<&AnnPredictionComponent> {
        self.predictions.get(index)
    }

    /// World returns all observers with confidence above threshold
    pub fn confident_observers(&self) -> Vec<&AnnObserverComponent> {
        self.observers
            .iter()
            .filter(|o| o.confidence_score >= self.config.confidence_threshold)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ann_world_creation() {
        let world = AnnObserverWorld::with_defaults();
        assert_eq!(world.observer_count(), 0);
        assert_eq!(world.layer_count(), 0);
    }

    #[test]
    fn test_add_observer() {
        let mut world = AnnObserverWorld::with_defaults();
        let idx = world.add_observer("test-observer".to_string());
        assert_eq!(idx, 0);
        assert_eq!(world.observer_count(), 1);

        let observer = world.get_observer(0).unwrap();
        assert_eq!(observer.observer_id, "test-observer");
        assert_eq!(observer.mode, AnnObserverMode::Passive);
    }

    #[test]
    fn test_add_layer() {
        let mut world = AnnObserverWorld::with_defaults();
        let layer_id = world.add_layer(AnnLayerType::Dense, 4, 8, ActivationFunction::ReLU);
        assert_eq!(layer_id, 0);
        assert_eq!(world.layer_count(), 1);
    }

    #[test]
    fn test_weight_initialization() {
        let mut world = AnnObserverWorld::with_defaults();
        world.add_layer(AnnLayerType::Dense, 4, 8, ActivationFunction::ReLU);
        world.initialize_weights(42);

        // Verify weights are initialized (not all zeros)
        let neurons = world.neurons.get(&0).unwrap();
        let has_nonzero = neurons.iter().any(|n| n.weights.iter().any(|&w| w != 0.0));
        assert!(has_nonzero);
    }
}
