//! PTCC Entropy-Based Persona-Tool Chain System
//!
//! Replaces keyword-based persona selection with mathematical entropy calculations
//! Based on TETH (Tool Entropy Testing Harness) foundation from Python models
//!
//! USIM Header: PTCC:ENTROPY:RUST:v1.0
//! SCH: blake2b("ptcc_entropy_system:2025")
//! CUID: ptcc:entropy:persona_tool_chain
//! UUID: generated per operation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 4-Tier Capability System from TETH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityTier {
    Novice { min_entropy: f64, max_entropy: f64 },    // 10-15
    Intermediate { min_entropy: f64, max_entropy: f64 }, // 15-25
    Advanced { min_entropy: f64, max_entropy: f64 },  // 25-35
    Elite { min_entropy: f64, max_entropy: f64 },     // 35-50
}

impl CapabilityTier {
    pub fn new_novice() -> Self {
        Self::Novice { min_entropy: 10.0, max_entropy: 15.0 }
    }

    pub fn new_intermediate() -> Self {
        Self::Intermediate { min_entropy: 15.0, max_entropy: 25.0 }
    }

    pub fn new_advanced() -> Self {
        Self::Advanced { min_entropy: 25.0, max_entropy: 35.0 }
    }

    pub fn new_elite() -> Self {
        Self::Elite { min_entropy: 35.0, max_entropy: 50.0 }
    }

    pub fn entropy_range(&self) -> (f64, f64) {
        match self {
            Self::Novice { min_entropy, max_entropy } => (*min_entropy, *max_entropy),
            Self::Intermediate { min_entropy, max_entropy } => (*min_entropy, *max_entropy),
            Self::Advanced { min_entropy, max_entropy } => (*min_entropy, *max_entropy),
            Self::Elite { min_entropy, max_entropy } => (*min_entropy, *max_entropy),
        }
    }
}

/// Tool properties for entropy calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProperties {
    pub branching_paths: u32,
    pub cognitive_load: f64,      // 0-10
    pub variability: f64,         // 0-10
    pub operational_risk: f64,    // 0-1
    pub feedback_clarity: f64,    // 0-1
    pub execution_time: f64,      // minutes
}

impl ToolProperties {
    pub fn calculate_entropy(&self) -> f64 {
        let mut entropy = 0.0;

        // Branching complexity
        entropy += (self.branching_paths as f64).log2();

        // Cognitive load factor
        entropy += self.cognitive_load * 2.0;

        // Variability increases entropy
        entropy += self.variability * 1.5;

        // Risk increases perceived complexity
        entropy += self.operational_risk * 5.0;

        // Poor feedback increases uncertainty
        entropy += (1.0 - self.feedback_clarity) * 3.0;

        entropy
    }
}

/// Enhanced persona with entropy-based capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyPersona {
    pub id: String,
    pub name: String,
    pub capability_tier: CapabilityTier,
    pub specializations: Vec<String>,
    pub fatigue_level: f64,           // 0-1, affects entropy tolerance
    pub success_history: Vec<f64>,    // Recent success rates
    pub preferred_entropy_range: (f64, f64),
}

impl EntropyPersona {
    pub fn new(id: String, name: String, tier: CapabilityTier) -> Self {
        let base_range = tier.entropy_range();
        Self {
            id,
            name,
            capability_tier: tier,
            specializations: Vec::new(),
            fatigue_level: 0.0,
            success_history: Vec::new(),
            preferred_entropy_range: base_range,
        }
    }

    pub fn current_entropy_tolerance(&self) -> (f64, f64) {
        let (min, max) = self.preferred_entropy_range;

        // Fatigue reduces tolerance
        let fatigue_penalty = self.fatigue_level * 5.0;

        (
            (min - fatigue_penalty).max(0.0),
            (max - fatigue_penalty).max(min)
        )
    }

    pub fn can_handle_entropy(&self, entropy: f64) -> (bool, f64) {
        let (min_tolerance, max_tolerance) = self.current_entropy_tolerance();

        if entropy < min_tolerance {
            // Too simple - boredom risk
            let confidence = 0.8 - (min_tolerance - entropy) / 10.0;
            (confidence > 0.5, confidence.max(0.0))
        } else if entropy > max_tolerance {
            // Too complex - failure risk
            let confidence = (1.0 - (entropy - max_tolerance) / 5.0).max(0.0);
            (confidence > 0.5, confidence)
        } else {
            // Within range - calculate confidence
            let optimal = (min_tolerance + max_tolerance) / 2.0;
            let distance = (entropy - optimal).abs();
            let confidence = 1.0 - distance / (max_tolerance - min_tolerance);
            (true, confidence.max(0.5))
        }
    }
}

/// Tool chain with entropy validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyToolChain {
    pub id: String,
    pub name: String,
    pub tools: Vec<ToolProperties>,
    pub assigned_personas: Vec<String>,
    pub total_entropy: f64,
    pub success_probability: f64,
    pub validated_scenarios: u32,
}

impl EntropyToolChain {
    pub fn new(id: String, name: String, tools: Vec<ToolProperties>) -> Self {
        let total_entropy = tools.iter().map(|t| t.calculate_entropy()).sum();

        Self {
            id,
            name,
            tools,
            assigned_personas: Vec::new(),
            total_entropy,
            success_probability: 0.0,
            validated_scenarios: 0,
        }
    }

    pub fn calculate_success_probability(&mut self, personas: &[EntropyPersona]) -> f64 {
        if self.tools.len() != personas.len() {
            return 0.0;
        }

        let mut individual_confidences = Vec::new();

        for (tool, persona) in self.tools.iter().zip(personas.iter()) {
            let tool_entropy = tool.calculate_entropy();
            let (_can_handle, confidence) = persona.can_handle_entropy(tool_entropy);
            individual_confidences.push(confidence);
        }

        // Chain success = product of individual successes
        let probability = individual_confidences.iter().product();
        self.success_probability = probability;
        probability
    }
}

/// Main PTCC entropy system
#[derive(Debug, Clone)]
pub struct PTCCEntropySystem {
    pub personas: Vec<EntropyPersona>,
    pub tool_chains: Vec<EntropyToolChain>,
    pub operational_context: OperationalContext,
}

/// Operational context affecting entropy calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalContext {
    pub time_pressure: String,      // low, medium, high, critical
    pub stakes: String,             // low, medium, high, critical
    pub environment_complexity: f64, // multiplier
    pub team_size: u32,
    pub communication_quality: f64,  // 0-1
    pub resource_availability: f64,  // 0-1
}

impl OperationalContext {
    pub fn default() -> Self {
        Self {
            time_pressure: "medium".to_string(),
            stakes: "medium".to_string(),
            environment_complexity: 1.0,
            team_size: 1,
            communication_quality: 1.0,
            resource_availability: 1.0,
        }
    }

    pub fn get_entropy_modifier(&self) -> f64 {
        let mut modifier = 1.0;

        // Time pressure increases effective entropy
        modifier *= match self.time_pressure.as_str() {
            "low" => 0.9,
            "medium" => 1.0,
            "high" => 1.2,
            "critical" => 1.5,
            _ => 1.0,
        };

        // High stakes increase cognitive load
        modifier *= match self.stakes.as_str() {
            "low" => 0.95,
            "medium" => 1.0,
            "high" => 1.1,
            "critical" => 1.3,
            _ => 1.0,
        };

        // Environmental factors
        modifier *= self.environment_complexity;

        // Team factors (larger teams can handle more complexity)
        if self.team_size > 1 {
            modifier *= 1.0 - 0.05 * (self.team_size - 1).min(5) as f64;
        }

        // Communication and resource constraints
        modifier *= (2.0 - self.communication_quality);
        modifier *= (2.0 - self.resource_availability);

        modifier
    }
}

impl PTCCEntropySystem {
    pub fn new() -> Self {
        Self {
            personas: Self::initialize_default_personas(),
            tool_chains: Vec::new(),
            operational_context: OperationalContext::default(),
        }
    }

    /// Initialize enhanced personas with entropy capabilities
    fn initialize_default_personas() -> Vec<EntropyPersona> {
        vec![
            EntropyPersona::new(
                "hayes".to_string(),
                "Commander Hayes".to_string(),
                CapabilityTier::new_advanced()
            ),
            EntropyPersona::new(
                "kozlov".to_string(),
                "Dmitri Kozlov".to_string(),
                CapabilityTier::new_elite()
            ),
            EntropyPersona::new(
                "sterling".to_string(),
                "James Sterling".to_string(),
                CapabilityTier::new_advanced()
            ),
            EntropyPersona::new(
                "al_rashid".to_string(),
                "Omar Al-Rashid".to_string(),
                CapabilityTier::new_intermediate()
            ),
            EntropyPersona::new(
                "volkov".to_string(),
                "Natasha Volkov".to_string(),
                CapabilityTier::new_elite()
            ),
            EntropyPersona::new(
                "chen".to_string(),
                "Emily Chen".to_string(),
                CapabilityTier::new_intermediate()
            ),
        ]
    }

    /// Select optimal persona for task entropy
    pub fn select_persona_for_entropy(&self, task_entropy: f64) -> Result<&EntropyPersona> {
        let adjusted_entropy = task_entropy * self.operational_context.get_entropy_modifier();

        let mut best_match: Option<(&EntropyPersona, f64)> = None;

        for persona in &self.personas {
            let (can_handle, confidence) = persona.can_handle_entropy(adjusted_entropy);

            if can_handle {
                match best_match {
                    None => best_match = Some((persona, confidence)),
                    Some((_, current_confidence)) => {
                        if confidence > current_confidence {
                            best_match = Some((persona, confidence));
                        }
                    }
                }
            }
        }

        best_match
            .map(|(persona, _)| persona)
            .ok_or_else(|| anyhow::anyhow!("No persona can handle entropy level {}", adjusted_entropy))
    }

    /// Optimize tool chain assignment
    pub fn optimize_tool_chain(&mut self, chain_id: &str) -> Result<f64> {
        let chain_index = self.tool_chains
            .iter()
            .position(|c| c.id == chain_id)
            .ok_or_else(|| anyhow::anyhow!("Tool chain not found: {}", chain_id))?;

        let chain = &mut self.tool_chains[chain_index];

        // Simple greedy assignment
        let mut assigned_personas = Vec::new();

        for tool in &chain.tools {
            let tool_entropy = tool.calculate_entropy();
            let adjusted_entropy = tool_entropy * self.operational_context.get_entropy_modifier();

            if let Ok(best_persona) = self.select_persona_for_entropy(adjusted_entropy) {
                assigned_personas.push(best_persona.id.clone());
            } else {
                return Err(anyhow::anyhow!("Cannot assign persona for tool entropy {}", adjusted_entropy));
            }
        }

        chain.assigned_personas = assigned_personas;
        let success_prob = chain.calculate_success_probability(&self.personas);

        Ok(success_prob)
    }

    /// Calculate content entropy from text (replacement for keyword matching)
    pub fn calculate_content_entropy(&self, content: &str) -> f64 {
        let content_lower = content.to_lowercase();
        let mut entropy = 15.0; // Base entropy

        // Technical complexity indicators
        let complexity_indicators = [
            ("algorithm", 5.0),
            ("architecture", 4.0),
            ("security", 8.0),
            ("threat", 7.0),
            ("financial", 6.0),
            ("blockchain", 8.0),
            ("cultural", 5.0),
            ("geopolitical", 7.0),
            ("ai", 9.0),
            ("neural", 8.0),
            ("infrastructure", 4.0),
            ("network", 3.0),
            ("cryptography", 10.0),
            ("vulnerability", 9.0),
        ];

        for (indicator, weight) in &complexity_indicators {
            if content_lower.contains(indicator) {
                entropy += weight;
            }
        }

        // Word count complexity
        let word_count = content.split_whitespace().count();
        entropy += (word_count as f64 / 100.0).min(10.0);

        // Adjust for operational context
        entropy * self.operational_context.get_entropy_modifier()
    }
}

/*
IMPLEMENTATION NOTES:

1. ENTROPY CALCULATION:
   - Branching paths: log2 scaling for decision complexity
   - Cognitive load: Direct 2x multiplier (most important factor)
   - Variability: 1.5x multiplier for unpredictability
   - Risk: 5x multiplier for high-stakes impact
   - Feedback: Inverse relationship, poor feedback = higher entropy

2. PERSONA MATCHING:
   - Four tiers with entropy tolerance ranges
   - Fatigue reduces tolerance (realistic operator degradation)
   - Confidence scoring based on distance from optimal range
   - Handles both under-stimulation (boredom) and over-stimulation (failure)

3. TOOL CHAIN OPTIMIZATION:
   - Greedy assignment algorithm (can be upgraded to Hungarian later)
   - Chain success = product of individual confidences
   - Considers operational context modifiers

4. CONTENT ENTROPY REPLACEMENT:
   - Replaces keyword counting with complexity analysis
   - Technical domain indicators with weighted scoring
   - Word count as complexity proxy
   - Operational context adjustments

5. INTEGRATION POINTS:
   - Can replace existing persona_manager.rs select_persona() method
   - Compatible with existing ElitePersona struct (conversion needed)
   - Ready for USIM header generation
   - Extensible for Monte Carlo validation

NEXT STEPS:
1. Test entropy calculations with sample content
2. Compare results against keyword-based selection
3. Add Monte Carlo validation system
4. Integration with agent interview EEI fields
5. Performance benchmarking and optimization

POTENTIAL ISSUES TO FIX:
- Need proper error handling for edge cases
- Persona assignment conflicts (same person on multiple tools)
- Context modifier bounds checking
- Success probability calibration with real data
- Memory/performance optimization for large tool chains
*/