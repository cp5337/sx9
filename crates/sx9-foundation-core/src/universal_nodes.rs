//! Universal Node Types - The Ten Universal Node Types (B₁ through B₁₀)
//! 
//! Implements the complete set of universal behavioral node classes from Universal Cognigraph.
//! Each node type provides specific atomic functions for operational intelligence.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Ten Universal Node Types (B₁ through B₁₀) from Universal Cognigraph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UniversalNodeType {
    Source,     // B₁: Emits resources, data, or energy  
    Sink,       // B₂: Absorbs waste, output, or terminal state
    Transformer,// B₃: Converts inputs to outputs
    Router,     // B₄: Controls directional flow
    Buffer,     // B₅: Temporarily holds state or resources
    Gate,       // B₆: Implements conditional access control
    Monitor,    // B₇: Observes system behavior
    Catalyst,   // B₈: Accelerates interactions
    Inhibitor,  // B₉: Blocks or throttles activity
    Relay,      // B₁₀: Extends interaction range
}

/// CTAS-specific node types (domain implementation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CTASNodeType {
    // Primary Domain Types
    People, Object, Location, Event, Document, Signal,
    Activity, Plan, Path, Behavior, Capability, Resource,
    Intent, Belief, Narrative, Relationship, Task, Asset,
    Agent, System, Intelligence, 
    
    // Security Domain Types
    Threat, Vulnerability, Countermeasure, Tool, Data, 
    Communication, Decision,
}

/// Universal node function executor
pub trait UniversalNodeFunction {
    fn execute_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32>;
}

/// Atomic function implementations for each universal node type
pub struct NodeFunctionExecutor {
    pub buffer_storage: HashMap<String, f32>,
    pub gate_threshold: f32,
    pub transformation_efficiency: f32,
}

impl Default for NodeFunctionExecutor {
    fn default() -> Self {
        Self {
            buffer_storage: HashMap::new(),
            gate_threshold: 0.5,
            transformation_efficiency: 0.8,
        }
    }
}

impl NodeFunctionExecutor {
    /// Execute function based on Universal Node Type
    pub fn execute_for_type(
        &mut self,
        node_type: &UniversalNodeType,
        inputs: &HashMap<String, f32>,
        energy_generation: f32,
        mass: f32,
    ) -> HashMap<String, f32> {
        match node_type {
            UniversalNodeType::Source => self.source_function(energy_generation, mass),
            UniversalNodeType::Sink => self.sink_function(inputs),
            UniversalNodeType::Transformer => self.transformer_function(inputs),
            UniversalNodeType::Router => self.router_function(inputs),
            UniversalNodeType::Buffer => self.buffer_function(inputs),
            UniversalNodeType::Gate => self.gate_function(inputs, energy_generation),
            UniversalNodeType::Monitor => self.monitor_function(inputs),
            UniversalNodeType::Catalyst => self.catalyst_function(inputs),
            UniversalNodeType::Inhibitor => self.inhibitor_function(inputs),
            UniversalNodeType::Relay => self.relay_function(inputs),
        }
    }
    
    // Universal Node Type Functions (implementing B₁ through B₁₀)
    
    /// B₁: Source Node - Emits resources, data, or energy
    fn source_function(&mut self, energy_generation: f32, mass: f32) -> HashMap<String, f32> {
        let mut outputs = HashMap::new();
        outputs.insert("energy".to_string(), energy_generation);
        outputs.insert("resource".to_string(), mass * 0.1);
        outputs.insert("data_stream".to_string(), 1.0);
        outputs
    }
    
    /// B₂: Sink Node - Absorbs waste, output, or terminal state
    fn sink_function(&mut self, _inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        // Absorb all inputs, return empty (terminal state)
        HashMap::new()
    }
    
    /// B₃: Transformer Node - Converts inputs to outputs
    fn transformer_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = HashMap::new();
        for (key, value) in inputs {
            // Transform inputs based on efficiency
            let transformed_value = value * self.transformation_efficiency;
            outputs.insert(format!("transformed_{}", key), transformed_value);
        }
        outputs
    }
    
    /// B₄: Router Node - Controls directional flow
    fn router_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = HashMap::new();
        
        // Route inputs based on priority/routing logic
        for (key, value) in inputs {
            if key.contains("priority") {
                outputs.insert(format!("high_priority_{}", key), *value);
            } else {
                outputs.insert(format!("standard_{}", key), *value);
            }
        }
        outputs
    }
    
    /// B₅: Buffer Node - Temporarily holds state or resources
    fn buffer_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        // Store inputs in buffer
        for (key, value) in inputs {
            *self.buffer_storage.entry(key.clone()).or_insert(0.0) += value;
        }
        
        // Release buffered content gradually
        let mut outputs = HashMap::new();
        let release_rate = 0.3; // Release 30% of buffer per cycle
        
        for (key, value) in self.buffer_storage.iter_mut() {
            let release_amount = *value * release_rate;
            outputs.insert(key.clone(), release_amount);
            *value -= release_amount;
        }
        
        // Clean up empty buffers
        self.buffer_storage.retain(|_, v| *v > 0.001);
        outputs
    }
    
    /// B₆: Gate Node - Implements conditional access control
    fn gate_function(&mut self, inputs: &HashMap<String, f32>, threshold_energy: f32) -> HashMap<String, f32> {
        // Conditional pass-through based on threshold
        if threshold_energy >= self.gate_threshold {
            inputs.clone() // Gate open - pass through
        } else {
            HashMap::new() // Gate closed - block all
        }
    }
    
    /// B₇: Monitor Node - Observes system behavior
    fn monitor_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = inputs.clone(); // Pass through unchanged
        
        // Add monitoring metadata
        let total_input = inputs.values().sum::<f32>();
        outputs.insert("monitor_total".to_string(), total_input);
        outputs.insert("monitor_count".to_string(), inputs.len() as f32);
        outputs.insert("monitor_timestamp".to_string(), 
                      std::time::SystemTime::now()
                          .duration_since(std::time::UNIX_EPOCH)
                          .unwrap_or_default()
                          .as_secs_f32());
        
        outputs
    }
    
    /// B₈: Catalyst Node - Accelerates interactions
    fn catalyst_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = HashMap::new();
        let acceleration_factor = 1.5;
        
        for (key, value) in inputs {
            // Accelerate interactions
            outputs.insert(key.clone(), value * acceleration_factor);
            outputs.insert(format!("catalyzed_{}", key), value * 0.2); // Side product
        }
        outputs
    }
    
    /// B₉: Inhibitor Node - Blocks or throttles activity
    fn inhibitor_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = HashMap::new();
        let inhibition_factor = 0.3; // Allow only 30% through
        
        for (key, value) in inputs {
            // Throttle interactions
            outputs.insert(key.clone(), value * inhibition_factor);
        }
        outputs
    }
    
    /// B₁₀: Relay Node - Extends interaction range
    fn relay_function(&mut self, inputs: &HashMap<String, f32>) -> HashMap<String, f32> {
        let mut outputs = inputs.clone(); // Pass through
        
        // Add range extension metadata
        outputs.insert("relay_boost".to_string(), 1.0);
        outputs.insert("extended_range".to_string(), 2.0); // 2x range multiplier
        
        outputs
    }
}

impl UniversalNodeType {
    /// Get the atomic number for this node type (1-10)
    pub fn atomic_number(&self) -> u8 {
        match self {
            UniversalNodeType::Source => 1,
            UniversalNodeType::Sink => 2,
            UniversalNodeType::Transformer => 3,
            UniversalNodeType::Router => 4,
            UniversalNodeType::Buffer => 5,
            UniversalNodeType::Gate => 6,
            UniversalNodeType::Monitor => 7,
            UniversalNodeType::Catalyst => 8,
            UniversalNodeType::Inhibitor => 9,
            UniversalNodeType::Relay => 10,
        }
    }
    
    /// Get human-readable description of the node function
    pub fn description(&self) -> &'static str {
        match self {
            UniversalNodeType::Source => "Emits resources, data, or energy",
            UniversalNodeType::Sink => "Absorbs waste, output, or terminal state",
            UniversalNodeType::Transformer => "Converts inputs to outputs",
            UniversalNodeType::Router => "Controls directional flow",
            UniversalNodeType::Buffer => "Temporarily holds state or resources",
            UniversalNodeType::Gate => "Implements conditional access control",
            UniversalNodeType::Monitor => "Observes system behavior",
            UniversalNodeType::Catalyst => "Accelerates interactions",
            UniversalNodeType::Inhibitor => "Blocks or throttles activity",
            UniversalNodeType::Relay => "Extends interaction range",
        }
    }
}
