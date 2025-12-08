# Plasma Dual-Layer Architecture

**Version:** 1.0  
**Status:** Design Specification  
**Date:** December 2025  

---

## The Problem

**Current PlasmaState** tracks:
- Delta Angle (cognitive state)
- Entropy (randomness)
- Crystal resonance (physics-based)
- SDT gate (thyristor control)

**But it's missing:**
- **Semantic understanding** (SCH alignment)
- **Cognitive context** (CUID alignment)
- **LISP rule evaluation** (semantic processing)
- **Meaning-based gating** (not just physics)

**Trivariate Hash has:**
- **SCH** (Semantic Content Hash): Domain, HD4 phase, N-V-N-N structure
- **CUID** (Cognitive Unique Identifier): Agent, Task, Sequence, Delta Angle, Entropy

**Plasma needs a semantic counterpart that aligns with these hashes.**

---

## The Solution: Dual-Layer Plasma

```
┌─────────────────────────────────────────────────────────────────┐
│                    DUAL-LAYER PLASMA                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │         PLASMA PHYSICS (PlasmaState)                      │  │
│   │                                                            │  │
│   │   Aligns with: CUID (Cognitive Unique Identifier)         │  │
│   │   • Delta Angle (Δθ):  Cognitive state                   │  │
│   │   • Entropy (H):        Randomness/chaos                  │  │
│   │   • Crystal Resonance:  Physics-based evaluation          │  │
│   │   • SDT Gate:           Thyristor control                 │  │
│   │                                                            │  │
│   │   Purpose: Physics-based gate control                     │  │
│   │   Latency: <250ns                                         │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │         PLASMA SEMANTIC (PlasmaSemantic)                  │  │
│   │                                                            │  │
│   │   Aligns with: SCH (Semantic Content Hash)               │  │
│   │   • Domain:              Cyber/Geo/Space/Maritime        │  │
│   │   • HD4 Phase:           Hunt/Detect/Disrupt/Disable/     │  │
│   │                          Dominate                          │  │
│   │   • N-V-N-N Structure:   Noun-Verb-Noun-Noun semantic    │  │
│   │   • LISP Rules:          Semantic rule evaluation          │  │
│   │   • Semantic Score:     Meaning-based evaluation (0.0-1.0)│  │
│   │                                                            │  │
│   │   Purpose: Semantic understanding and rule evaluation     │  │
│   │   Latency: <1ms (LISP evaluation)                        │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │              COMBINED GATE DECISION                        │  │
│   │                                                            │  │
│   │   Physics Score (0.0-1.0) + Semantic Score (0.0-1.0)     │  │
│   │   → Combined Score (0.0-1.0)                              │  │
│   │   → SDT Gate Decision                                     │  │
│   │                                                            │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Architecture

### Layer 1: PlasmaPhysics (PlasmaState)

**Aligns with CUID (Cognitive Unique Identifier)**

```rust
pub struct PlasmaState {
    // CUID-aligned fields
    delta_angle: AtomicU16,      // CUID slots 10-11 (CRITICAL)
    entropy: AtomicU32,           // CUID slots 12-13
    excited: AtomicBool,          // Crystal ringing?
    sdt_state: AtomicU8,         // Thyristor gate state
    
    // Metrics
    last_trigger_tick: AtomicU64,
    trigger_count: AtomicU32,
    last_ring_strength: AtomicU32, // Physics-based (0.0-1.0)
    supersession_count: AtomicU32,
}
```

**Purpose:** Physics-based gate control (crystal resonance, thyristor)

**Input:** Payload hash, entropy, delta angle

**Output:** Ring strength (0.0-1.0), SDT gate state

**Latency:** <250ns

---

### Layer 2: PlasmaSemantic (NEW)

**Aligns with SCH (Semantic Content Hash)**

```rust
pub struct PlasmaSemantic {
    // SCH-aligned fields
    domain: AtomicU8,            // SCH bits 0-15 (Domain)
    hd4_phase: AtomicU8,          // SCH bits 16-31 (HD4 Phase)
    nvnn_hash: AtomicU32,         // SCH bits 32-47 (N-V-N-N structure)
    semantic_delta: AtomicU16,    // SCH bits 48-63 (Delta Angle)
    
    // Semantic evaluation
    lisp_rules: Arc<LispRuleEngine>, // LISP rule evaluator
    semantic_score: AtomicU32,    // Meaning-based score (f32 as bits)
    rule_matches: AtomicU32,      // How many rules matched
    semantic_excited: AtomicBool,  // Semantic resonance?
    
    // Thalmic annotation
    priority: AtomicU8,           // 0-127
    confidence: AtomicU8,         // 0-127
    suppression: AtomicU8,        // Suppression code
    agent_route: AtomicU8,         // Agent routing (0-255)
    
    // Voice & Prompt Tuning
    voice_tuning: Arc<VoiceTuning>,      // Voice synthesis optimization
    prompt_tuning: Arc<PromptTuning>,   // Prompt generation optimization
}
```

**Purpose:** Semantic understanding and rule evaluation

**Input:** Domain, HD4 phase, N-V-N-N structure, LISP rules

**Output:** Semantic score (0.0-1.0), rule matches, suppression decision

**Latency:** <1ms (LISP evaluation)

---

## Integration

### Combined Evaluation

```rust
pub struct DualPlasma {
    physics: PlasmaState,
    semantic: PlasmaSemantic,
    combined_config: CombinedConfig,
}

impl DualPlasma {
    /// Evaluate command through both physics and semantic layers
    pub fn evaluate(
        &self,
        payload: &[u8],
        sch: &SchHash,      // Semantic Content Hash
        cuid: &CuidHash,    // Cognitive Unique Identifier
        tick: u64,
    ) -> CombinedResult {
        // 1. Physics evaluation (CUID-aligned)
        let physics_result = self.physics.resonate_poly(
            &self.physics.polycrystal,
            payload,
            tick,
            &self.physics.thyristor_config,
        );
        
        // 2. Semantic evaluation (SCH-aligned)
        let semantic_result = self.semantic.evaluate_semantic(
            sch,
            &self.semantic.lisp_rules,
            tick,
        );
        
        // 3. Combined scoring
        let combined_score = self.combine_scores(
            physics_result.final_strength,
            semantic_result.semantic_score,
        );
        
        // 4. Gate decision
        let gate_allows = self.compute_gate_decision(
            combined_score,
            physics_result.sdt_state,
            semantic_result.suppression,
        );
        
        CombinedResult {
            physics_score: physics_result.final_strength,
            semantic_score: semantic_result.semantic_score,
            combined_score,
            gate_allows,
            physics_sdt: physics_result.sdt_state,
            semantic_suppression: semantic_result.suppression,
        }
    }
    
    fn combine_scores(&self, physics: f32, semantic: f32) -> f32 {
        // Weighted combination
        let physics_weight = self.combined_config.physics_weight;  // 0.6
        let semantic_weight = self.combined_config.semantic_weight; // 0.4
        
        (physics * physics_weight) + (semantic * semantic_weight)
    }
}
```

---

## SCH Alignment (PlasmaSemantic)

### SCH Components → PlasmaSemantic Fields

| SCH Component | Bits | PlasmaSemantic Field | Purpose |
|--------------|------|---------------------|---------|
| Domain | 0-15 | `domain: AtomicU8` | Cyber/Geo/Space/Maritime |
| HD4 Phase | 16-31 | `hd4_phase: AtomicU8` | Hunt/Detect/Disrupt/Disable/Dominate |
| N-V-N-N | 32-47 | `nvnn_hash: AtomicU32` | Semantic structure hash |
| Delta Angle | 48-63 | `semantic_delta: AtomicU16` | Semantic state delta |

### Semantic Evaluation

```rust
impl PlasmaSemantic {
    /// Evaluate semantic content using SCH and LISP rules
    pub fn evaluate_semantic(
        &self,
        sch: &SchHash,
        lisp_rules: &LispRuleEngine,
        tick: u64,
    ) -> SemanticResult {
        // 1. Extract SCH components
        let domain = sch.domain();
        let hd4_phase = sch.hd4_phase();
        let nvnn = sch.nvnn_structure();
        let delta = sch.delta_angle();
        
        // 2. Update semantic state
        self.domain.store(domain as u8, Ordering::Release);
        self.hd4_phase.store(hd4_phase as u8, Ordering::Release);
        self.nvnn_hash.store(nvnn, Ordering::Release);
        self.semantic_delta.store(delta, Ordering::Release);
        
        // 3. Evaluate LISP rules
        let rule_result = lisp_rules.evaluate(
            domain,
            hd4_phase,
            nvnn,
            delta,
        );
        
        // 4. Compute semantic score
        let semantic_score = self.compute_semantic_score(
            domain,
            hd4_phase,
            nvnn,
            rule_result.matches,
        );
        
        // 5. Check suppression
        let suppression = self.check_suppression(
            rule_result.matches,
            semantic_score,
        );
        
        SemanticResult {
            semantic_score,
            rule_matches: rule_result.matches,
            suppression,
            domain,
            hd4_phase,
        }
    }
    
    fn compute_semantic_score(
        &self,
        domain: Domain,
        hd4_phase: HD4Phase,
        nvnn: u32,
        rule_matches: u32,
    ) -> f32 {
        // Domain coherence
        let domain_coherence = self.domain_coherence(domain);
        
        // HD4 phase alignment
        let phase_alignment = self.phase_alignment(hd4_phase);
        
        // N-V-N-N structure coherence
        let nvnn_coherence = self.nvnn_coherence(nvnn);
        
        // Rule match strength
        let rule_strength = (rule_matches as f32) / 10.0; // Normalize
        
        // Weighted combination
        (domain_coherence * 0.3)
            + (phase_alignment * 0.3)
            + (nvnn_coherence * 0.2)
            + (rule_strength * 0.2)
    }
}
```

---

## CUID Alignment (PlasmaState)

### CUID Components → PlasmaState Fields

| CUID Component | Slots | PlasmaState Field | Purpose |
|----------------|-------|-------------------|---------|
| Agent ID | 0-1 | (not tracked) | Agent context |
| Task ID | 2-3 | (not tracked) | Task context |
| Sequence | 4-5 | (not tracked) | Sequence context |
| Timestamp | 6-9 | (not tracked) | Temporal context |
| **Delta Angle** | **10-11** | **`delta_angle: AtomicU16`** | **CRITICAL - Cognitive state** |
| **Entropy** | **12-13** | **`entropy: AtomicU32`** | **CRITICAL - Randomness** |
| Checksum | 14-15 | (not tracked) | Integrity |

**Current PlasmaState already aligns with CUID slots 10-13 (the critical ones).**

---

## LISP Rule Integration

### LISP Rule Engine

```rust
pub struct LispRuleEngine {
    rules: Vec<LispRule>,
    domain_rules: HashMap<Domain, Vec<LispRule>>,
    phase_rules: HashMap<HD4Phase, Vec<LispRule>>,
}

pub struct LispRule {
    id: String,
    domain: Option<Domain>,
    hd4_phase: Option<HD4Phase>,
    nvnn_pattern: Option<NvnnPattern>,
    condition: LispExpression,
    action: LispAction,
    weight: f32,
}

impl LispRuleEngine {
    /// Evaluate rules against semantic content
    pub fn evaluate(
        &self,
        domain: Domain,
        hd4_phase: HD4Phase,
        nvnn: u32,
        delta: u16,
    ) -> RuleResult {
        let mut matches = 0;
        let mut total_weight = 0.0;
        
        // Get relevant rules
        let domain_rules = self.domain_rules.get(&domain).unwrap_or(&vec![]);
        let phase_rules = self.phase_rules.get(&hd4_phase).unwrap_or(&vec![]);
        
        // Evaluate domain rules
        for rule in domain_rules {
            if self.matches(rule, domain, hd4_phase, nvnn, delta) {
                matches += 1;
                total_weight += rule.weight;
            }
        }
        
        // Evaluate phase rules
        for rule in phase_rules {
            if self.matches(rule, domain, hd4_phase, nvnn, delta) {
                matches += 1;
                total_weight += rule.weight;
            }
        }
        
        RuleResult {
            matches,
            total_weight,
            strength: if matches > 0 { total_weight / matches as f32 } else { 0.0 },
        }
    }
}
```

---

## Combined Gate Decision

```rust
impl DualPlasma {
    fn compute_gate_decision(
        &self,
        combined_score: f32,
        physics_sdt: SdtState,
        semantic_suppression: SuppressionCode,
    ) -> bool {
        // Suppression overrides everything
        if semantic_suppression != SuppressionCode::None {
            return false; // Suppressed
        }
        
        // Combined score threshold
        if combined_score < self.combined_config.min_score {
            return false; // Below threshold
        }
        
        // Physics gate must be open
        match physics_sdt {
            SdtState::Conducting | SdtState::Latched => true,
            SdtState::Off | SdtState::Primed => false,
        }
    }
}
```

---

## Benefits

1. **Dual Alignment**: Physics aligns with CUID, Semantic aligns with SCH
2. **Meaning + Physics**: Both semantic understanding and physics-based control
3. **LISP Integration**: Rule-based semantic evaluation
4. **Suppression Support**: Semantic layer can suppress based on rules
5. **Unified Gate**: Combined decision from both layers

---

## Implementation Plan

### Phase 1: PlasmaSemantic Structure
- [ ] Create `PlasmaSemantic` struct with SCH-aligned fields
- [ ] Add LISP rule engine integration
- [ ] Implement semantic score computation

### Phase 2: DualPlasma Integration
- [ ] Create `DualPlasma` wrapper
- [ ] Implement combined scoring
- [ ] Implement unified gate decision

### Phase 3: LISP Rule Engine
- [ ] Integrate `sx9-lisp` for rule evaluation
- [ ] Add domain/phase rule matching
- [ ] Add suppression logic

### Phase 4: Voice & Prompt Tuning
- [ ] Implement voice tuning (ElevenLabs, Azure Speech)
- [ ] Implement prompt optimization (PromptScript v3)
- [ ] Add domain/phase-based adjustments
- [ ] Add LISP rule evaluation for tuning

### Phase 5: Integration Points
- [ ] Update `sx9-atlas-bus` to use `DualPlasma`
- [ ] Update port manager to use semantic layer
- [ ] Update Plasma Defender to use semantic layer
- [ ] Integrate voice synthesis with semantic tuning
- [ ] Integrate prompt generation with semantic optimization

---

**The crystal is the quartz. The thyristor is the switch it triggers. Plasma is the field they operate in. PlasmaSemantic is the meaning layer that understands what flows through it.**

