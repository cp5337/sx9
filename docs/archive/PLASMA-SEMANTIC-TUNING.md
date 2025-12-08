# PlasmaSemantic Tuning for Voice & Prompts

**Version:** 1.0  
**Status:** Design Specification  
**Date:** December 2025  

---

## Overview

**PlasmaSemantic** can tune voice synthesis and prompt generation based on semantic understanding (SCH alignment). The semantic layer understands **meaning**, not just physics, enabling intelligent optimization.

---

## Voice Tuning Architecture

### Current Voice System

**ElevenLabs Voice Settings:**
```rust
voice_settings: {
    stability: 0.5,           // Fixed
    similarity_boost: 0.75,   // Fixed
    style: 0.5,               // Fixed
    use_speaker_boost: true   // Fixed
}
```

**Problem:** Static settings don't adapt to semantic content.

### PlasmaSemantic Voice Tuning

```rust
pub struct VoiceTuning {
    /// Base voice settings
    base_settings: VoiceSettings,
    
    /// Semantic tuning rules
    semantic_rules: Vec<VoiceTuningRule>,
    
    /// Domain-specific voice profiles
    domain_profiles: HashMap<Domain, VoiceProfile>,
    
    /// HD4 phase voice adjustments
    phase_adjustments: HashMap<HD4Phase, VoiceAdjustment>,
}

pub struct VoiceTuningRule {
    /// LISP condition
    condition: LispExpression,
    
    /// Voice parameter adjustments
    adjustments: VoiceAdjustment,
    
    /// Weight (for multiple matching rules)
    weight: f32,
}

pub struct VoiceAdjustment {
    stability_delta: f32,      // -0.3 to +0.3
    similarity_delta: f32,     // -0.2 to +0.2
    style_delta: f32,         // -0.4 to +0.4
    speaker_boost: Option<bool>,
    voice_id_override: Option<String>,
}
```

### Semantic-Based Voice Tuning

```rust
impl PlasmaSemantic {
    /// Tune voice settings based on semantic content
    pub fn tune_voice(
        &self,
        sch: &SchHash,
        text: &str,
        base_settings: VoiceSettings,
    ) -> VoiceSettings {
        // 1. Extract semantic components
        let domain = sch.domain();
        let hd4_phase = sch.hd4_phase();
        let nvnn = sch.nvnn_structure();
        
        // 2. Get domain profile
        let domain_profile = self.domain_profiles.get(&domain)
            .unwrap_or(&VoiceProfile::default());
        
        // 3. Get phase adjustment
        let phase_adj = self.phase_adjustments.get(&hd4_phase)
            .unwrap_or(&VoiceAdjustment::none());
        
        // 4. Evaluate LISP rules
        let rule_adjustments = self.evaluate_voice_rules(
            domain,
            hd4_phase,
            nvnn,
            text,
        );
        
        // 5. Combine adjustments
        let mut tuned = base_settings;
        
        // Domain profile
        tuned.stability += domain_profile.stability_delta;
        tuned.similarity_boost += domain_profile.similarity_delta;
        tuned.style += domain_profile.style_delta;
        
        // Phase adjustment
        tuned.stability += phase_adj.stability_delta;
        tuned.similarity_boost += phase_adj.similarity_delta;
        tuned.style += phase_adj.style_delta;
        
        // Rule-based adjustments (weighted average)
        for (adj, weight) in rule_adjustments {
            tuned.stability += adj.stability_delta * weight;
            tuned.similarity_boost += adj.similarity_delta * weight;
            tuned.style += adj.style_delta * weight;
        }
        
        // Clamp values
        tuned.stability = tuned.stability.clamp(0.0, 1.0);
        tuned.similarity_boost = tuned.similarity_boost.clamp(0.0, 1.0);
        tuned.style = tuned.style.clamp(0.0, 1.0);
        
        tuned
    }
    
    fn evaluate_voice_rules(
        &self,
        domain: Domain,
        hd4_phase: HD4Phase,
        nvnn: u32,
        text: &str,
    ) -> Vec<(VoiceAdjustment, f32)> {
        let mut adjustments = Vec::new();
        
        for rule in &self.voice_tuning.semantic_rules {
            // Evaluate LISP condition
            if self.lisp_rules.evaluate_condition(
                &rule.condition,
                domain,
                hd4_phase,
                nvnn,
                text,
            ) {
                adjustments.push((rule.adjustments.clone(), rule.weight));
            }
        }
        
        adjustments
    }
}
```

---

## Voice Tuning Rules

### Domain-Based Tuning

```rust
// Domain voice profiles
let domain_profiles = HashMap::from([
    (Domain::Cyber, VoiceProfile {
        stability_delta: -0.1,      // More dynamic (cyber ops are fast)
        similarity_delta: 0.0,
        style_delta: 0.2,          // More expressive
        voice_id_override: Some("cyber-operator".to_string()),
    }),
    (Domain::Orbital, VoiceProfile {
        stability_delta: 0.2,      // More stable (precision required)
        similarity_delta: 0.1,
        style_delta: -0.1,          // Less expressive (professional)
        voice_id_override: Some("ground-station".to_string()),
    }),
    (Domain::Maritime, VoiceProfile {
        stability_delta: 0.0,
        similarity_delta: 0.0,
        style_delta: 0.0,
        voice_id_override: None,
    }),
]);
```

### HD4 Phase Tuning

```rust
// Phase-based adjustments
let phase_adjustments = HashMap::from([
    (HD4Phase::Hunt, VoiceAdjustment {
        stability_delta: -0.15,     // More dynamic (exploratory)
        similarity_delta: 0.0,
        style_delta: 0.1,
        speaker_boost: Some(true),
    }),
    (HD4Phase::Detect, VoiceAdjustment {
        stability_delta: 0.0,
        similarity_delta: 0.0,
        style_delta: 0.0,
        speaker_boost: None,
    }),
    (HD4Phase::Disrupt, VoiceAdjustment {
        stability_delta: -0.2,      // Very dynamic (action phase)
        similarity_delta: 0.0,
        style_delta: 0.3,           // More expressive
        speaker_boost: Some(true),
    }),
    (HD4Phase::Disable, VoiceAdjustment {
        stability_delta: 0.1,       // More stable (precision)
        similarity_delta: 0.0,
        style_delta: -0.1,
        speaker_boost: None,
    }),
    (HD4Phase::Dominate, VoiceAdjustment {
        stability_delta: 0.2,       // Very stable (authoritative)
        similarity_delta: 0.1,
        style_delta: 0.0,
        speaker_boost: Some(false),
    }),
]);
```

### LISP Rule Examples

```lisp
;; Rule: High-priority alerts get more expressive voice
(voice-tuning-rule
  :condition (and (>= priority 80) (eq domain "cyber"))
  :adjustments {
    :stability-delta -0.2
    :style-delta 0.3
    :speaker-boost true
  }
  :weight 1.0)

;; Rule: Long technical explanations get more stable voice
(voice-tuning-rule
  :condition (and (> text-length 500) (contains? text "technical"))
  :adjustments {
    :stability-delta 0.15
    :style-delta -0.1
  }
  :weight 0.8)

;; Rule: Urgent commands get dynamic voice
(voice-tuning-rule
  :condition (or (eq hd4-phase "disrupt") (eq hd4-phase "disable"))
  :adjustments {
    :stability-delta -0.1
    :style-delta 0.2
  }
  :weight 0.9)
```

---

## Prompt Tuning Architecture

### Current Prompt Generation

**PromptScript v3 Pipeline:**
```
SemanticVector → GrammarEnforcer → TypeChecker → HashSeeder → PromptScript
```

**Problem:** No semantic optimization based on context.

### PlasmaSemantic Prompt Tuning

```rust
pub struct PromptTuning {
    /// Base prompt generator
    base_generator: PromptGenerator,
    
    /// Semantic optimization rules
    optimization_rules: Vec<PromptOptimizationRule>,
    
    /// Domain-specific prompt templates
    domain_templates: HashMap<Domain, PromptTemplate>,
    
    /// HD4 phase prompt adjustments
    phase_prompts: HashMap<HD4Phase, PromptAdjustment>,
}

pub struct PromptOptimizationRule {
    /// LISP condition
    condition: LispExpression,
    
    /// Prompt adjustments
    adjustments: PromptAdjustment,
    
    /// Weight
    weight: f32,
}

pub struct PromptAdjustment {
    /// Add context keywords
    add_context: Vec<String>,
    
    /// Remove noise keywords
    remove_noise: Vec<String>,
    
    /// Adjust verbosity (0.0-1.0)
    verbosity_delta: f32,
    
    /// Adjust formality (0.0-1.0)
    formality_delta: f32,
    
    /// Add domain-specific instructions
    domain_instructions: Option<String>,
    
    /// Override ANN synthesis mode
    ann_mode_override: Option<AnnMode>,
}
```

### Semantic-Based Prompt Optimization

```rust
impl PlasmaSemantic {
    /// Optimize prompt based on semantic content
    pub fn optimize_prompt(
        &self,
        sch: &SchHash,
        base_prompt: &str,
        intent: &str,
    ) -> String {
        // 1. Extract semantic components
        let domain = sch.domain();
        let hd4_phase = sch.hd4_phase();
        let nvnn = sch.nvnn_structure();
        
        // 2. Get domain template
        let template = self.domain_templates.get(&domain)
            .unwrap_or(&PromptTemplate::default());
        
        // 3. Get phase adjustment
        let phase_adj = self.phase_prompts.get(&hd4_phase)
            .unwrap_or(&PromptAdjustment::none());
        
        // 4. Evaluate optimization rules
        let rule_adjustments = self.evaluate_prompt_rules(
            domain,
            hd4_phase,
            nvnn,
            base_prompt,
            intent,
        );
        
        // 5. Build optimized prompt
        let mut optimized = template.apply(base_prompt);
        
        // Apply phase adjustments
        optimized = self.apply_adjustment(&optimized, phase_adj);
        
        // Apply rule-based adjustments
        for (adj, weight) in rule_adjustments {
            optimized = self.apply_adjustment_weighted(&optimized, &adj, weight);
        }
        
        // 6. Post-process (remove noise, add context)
        optimized = self.post_process_prompt(&optimized);
        
        optimized
    }
    
    fn apply_adjustment(
        &self,
        prompt: &str,
        adj: &PromptAdjustment,
    ) -> String {
        let mut result = prompt.to_string();
        
        // Add context
        if !adj.add_context.is_empty() {
            let context = adj.add_context.join(", ");
            result = format!("Context: {}. {}", context, result);
        }
        
        // Remove noise
        for noise in &adj.remove_noise {
            result = result.replace(noise, "");
        }
        
        // Adjust verbosity
        if adj.verbosity_delta > 0.0 {
            result = self.increase_verbosity(&result, adj.verbosity_delta);
        } else if adj.verbosity_delta < 0.0 {
            result = self.decrease_verbosity(&result, -adj.verbosity_delta);
        }
        
        // Add domain instructions
        if let Some(instructions) = &adj.domain_instructions {
            result = format!("{}\n\nDomain Instructions: {}", result, instructions);
        }
        
        result
    }
}
```

---

## Prompt Tuning Rules

### Domain-Based Templates

```rust
let domain_templates = HashMap::from([
    (Domain::Cyber, PromptTemplate {
        prefix: "You are a cyber security expert. ",
        suffix: " Provide technical details and MITRE ATT&CK mappings.",
        verbosity: 0.7,
        formality: 0.8,
    }),
    (Domain::Orbital, PromptTemplate {
        prefix: "You are an orbital mechanics specialist. ",
        suffix: " Include TLE data and propagation calculations.",
        verbosity: 0.9,
        formality: 0.9,
    }),
    (Domain::Maritime, PromptTemplate {
        prefix: "You are a maritime intelligence analyst. ",
        suffix: " Include AIS data and vessel tracking information.",
        verbosity: 0.6,
        formality: 0.7,
    }),
]);
```

### HD4 Phase Prompt Adjustments

```rust
let phase_prompts = HashMap::from([
    (HD4Phase::Hunt, PromptAdjustment {
        add_context: vec!["exploratory", "reconnaissance", "information gathering"],
        remove_noise: vec!["definitive", "conclusive"],
        verbosity_delta: 0.2,      // More verbose (exploratory)
        formality_delta: -0.1,     // Less formal
        domain_instructions: Some("Focus on discovery and pattern recognition.".to_string()),
        ann_mode_override: Some(AnnMode::GNN), // Graph-based for exploration
    }),
    (HD4Phase::Detect, PromptAdjustment {
        add_context: vec!["analysis", "pattern matching", "anomaly detection"],
        remove_noise: vec![],
        verbosity_delta: 0.0,
        formality_delta: 0.0,
        domain_instructions: Some("Focus on detection and classification.".to_string()),
        ann_mode_override: Some(AnnMode::DistilBERT), // Classification
    }),
    (HD4Phase::Disrupt, PromptAdjustment {
        add_context: vec!["action", "execution", "tactical"],
        remove_noise: vec!["theoretical", "speculative"],
        verbosity_delta: -0.2,     // Less verbose (action-oriented)
        formality_delta: 0.1,      // More formal (precise)
        domain_instructions: Some("Focus on execution and tactical response.".to_string()),
        ann_mode_override: Some(AnnMode::Symbolic), // Rule-based for speed
    }),
    (HD4Phase::Disable, PromptAdjustment {
        add_context: vec!["precision", "targeted", "surgical"],
        remove_noise: vec!["broad", "general"],
        verbosity_delta: -0.3,     // Concise (precision)
        formality_delta: 0.2,      // Very formal
        domain_instructions: Some("Focus on precise, targeted actions.".to_string()),
        ann_mode_override: Some(AnnMode::Symbolic), // Rule-based
    }),
    (HD4Phase::Dominate, PromptAdjustment {
        add_context: vec!["authoritative", "comprehensive", "complete"],
        remove_noise: vec![],
        verbosity_delta: 0.1,      // Slightly verbose (comprehensive)
        formality_delta: 0.3,      // Very formal (authoritative)
        domain_instructions: Some("Focus on comprehensive control and dominance.".to_string()),
        ann_mode_override: Some(AnnMode::Hybrid), // Combined approach
    }),
]);
```

### LISP Rule Examples

```lisp
;; Rule: Technical domains get more detailed prompts
(prompt-optimization-rule
  :condition (or (eq domain "cyber") (eq domain "orbital"))
  :adjustments {
    :verbosity-delta 0.2
    :formality-delta 0.1
    :add-context ["technical", "precise", "detailed"]
  }
  :weight 1.0)

;; Rule: Urgent intents get concise prompts
(prompt-optimization-rule
  :condition (or (contains? intent "urgent") (contains? intent "critical"))
  :adjustments {
    :verbosity-delta -0.3
    :formality-delta 0.2
    :remove-noise ["background", "context", "history"]
  }
  :weight 0.9)

;; Rule: Complex N-V-N-N structures get more context
(prompt-optimization-rule
  :condition (> nvnn-complexity 0.7)
  :adjustments {
    :verbosity-delta 0.15
    :add-context ["complex", "multi-step", "hierarchical"]
  }
  :weight 0.8)
```

---

## Integration Points

### 1. Voice Synthesis Integration

```rust
// In voice synthesis service
pub async fn synthesize_with_semantic_tuning(
    text: &str,
    sch: &SchHash,
    base_voice_id: &str,
) -> Result<Vec<u8>> {
    // Get base settings
    let base_settings = VoiceSettings::default();
    
    // Tune via PlasmaSemantic
    let tuned_settings = plasma_semantic.tune_voice(sch, text, base_settings);
    
    // Synthesize with tuned settings
    elevenlabs_client.text_to_speech(
        text,
        base_voice_id,
        tuned_settings,
    ).await
}
```

### 2. Prompt Generation Integration

```rust
// In prompt generator
pub fn generate_prompt_with_semantic_optimization(
    intent: &str,
    sch: &SchHash,
    base_template: &str,
) -> String {
    // Generate base prompt
    let base_prompt = prompt_generator.generate(intent, base_template);
    
    // Optimize via PlasmaSemantic
    plasma_semantic.optimize_prompt(sch, &base_prompt, intent)
}
```

---

## Tuning Matrix

| Domain | HD4 Phase | Voice Stability | Voice Style | Prompt Verbosity | Prompt Formality |
|--------|-----------|----------------|-------------|------------------|------------------|
| Cyber | Hunt | -0.15 | +0.1 | +0.2 | -0.1 |
| Cyber | Disrupt | -0.2 | +0.3 | -0.2 | +0.1 |
| Cyber | Dominate | +0.2 | 0.0 | +0.1 | +0.3 |
| Orbital | Hunt | -0.1 | 0.0 | +0.2 | 0.0 |
| Orbital | Detect | 0.0 | 0.0 | 0.0 | 0.0 |
| Orbital | Disable | +0.1 | -0.1 | -0.3 | +0.2 |
| Maritime | Hunt | -0.1 | +0.1 | +0.15 | -0.1 |
| Maritime | Disrupt | -0.15 | +0.2 | -0.15 | +0.1 |

---

## Benefits

1. **Adaptive Voice**: Voice settings adapt to semantic content (domain, phase, urgency)
2. **Optimized Prompts**: Prompts are optimized for context (verbosity, formality, domain-specific)
3. **LISP-Driven**: Rules can be updated without code changes
4. **Unified Control**: Same semantic layer controls both voice and prompts
5. **Performance**: Semantic tuning adds <1ms latency

---

## Implementation Plan

### Phase 1: Voice Tuning
- [ ] Create `VoiceTuning` struct
- [ ] Implement domain/phase profiles
- [ ] Add LISP rule evaluation
- [ ] Integrate with ElevenLabs client

### Phase 2: Prompt Tuning
- [ ] Create `PromptTuning` struct
- [ ] Implement domain templates
- [ ] Add phase adjustments
- [ ] Integrate with PromptGenerator

### Phase 3: Unified Integration
- [ ] Create `PlasmaSemanticTuner` wrapper
- [ ] Add combined tuning API
- [ ] Add metrics/observability
- [ ] Add dev toggle (bypass tuning)

---

**PlasmaSemantic understands meaning. Voice and prompts adapt to that meaning.**



