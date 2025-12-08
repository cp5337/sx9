# RFC-9025 — Cognitive Convergence Mathematics

**Version:** 1.0  
**Status:** Core Algorithm Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9021, RFC-9023, RFC-9024

---

## 1. Abstract

This RFC specifies the three mathematical models that compute the **H1 (Operational)** and **H2 (Semantic)** convergence vectors driving HD4 phase transitions:

1. **Hidden Markov Models (HMM)** — Adversary phase detection
2. **Matroids** — Information independence measurement
3. **Hawkes Process** — Temporal self-excitation detection

These algorithms execute in the **OODA Orient phase** to determine when tactical situations cross the **75% Convergence Line**.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    COGNITIVE CONVERGENCE ENGINE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  RAW OBSERVATIONS (Industrial Events / Telemetry)                           │
│                         │                                                   │
│           ┌─────────────┴─────────────┐                                    │
│           │                           │                                    │
│           ▼                           ▼                                    │
│  ┌─────────────────┐         ┌─────────────────┐                          │
│  │ H1: OPERATIONAL │         │ H2: SEMANTIC    │                          │
│  │ (Fast/NOW)      │         │ (Slow/MEANING)  │                          │
│  │ ───────────────  │         │ ─────────────── │                          │
│  │                  │         │                 │                          │
│  │ ┌──────────────┐│         │┌───────────────┐│                          │
│  │ │Hawkes Process││         ││ Matroid Rank  ││                          │
│  │ │λ(t) intensity││         ││ r(S) independ.││                          │
│  │ └──────────────┘│         │└───────────────┘│                          │
│  │                  │         │                 │                          │
│  │ ┌──────────────┐│         │┌───────────────┐│                          │
│  │ │Δ-Angle Deriv.││         ││ HMM Phase     ││                          │
│  │ │(context drift)││         ││ Detection     ││                          │
│  │ └──────────────┘│         │└───────────────┘│                          │
│  │                  │         │                 │                          │
│  └────────┬─────────┘         └────────┬────────┘                          │
│           │                            │                                    │
│           └────────────┬───────────────┘                                   │
│                        ▼                                                    │
│           ┌─────────────────────────┐                                      │
│           │ ORIENT DECISION         │                                      │
│           │ ────────────────        │                                      │
│           │                         │                                      │
│           │ if H1 ≥ 0.75 AND        │                                      │
│           │    H2 ≥ 0.75            │                                      │
│           │ then ABOVE LINE         │                                      │
│           │ → HD4 Phase Transition  │                                      │
│           │                         │                                      │
│           └───────────┬─────────────┘                                      │
│                       │                                                     │
│                       ▼                                                     │
│           ┌─────────────────────────┐                                      │
│           │ HD4 PHASE OUTPUT        │                                      │
│           │ Hunt/Detect/Disable/    │                                      │
│           │ Disrupt/Dominate        │                                      │
│           └─────────────────────────┘                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Hidden Markov Models (HMM)

### 3.1 Purpose

Detect the **adversary's hidden state** (intent) from observable events (emissions). In IVCS context:

```
[Staging] → [Execution] = [Setting up remote access] → [Sending override command]
```

### 3.2 Hidden States

| State | Description | Observable Emissions |
|-------|-------------|---------------------|
| **Recon** | Information gathering | Port scans, queries, enumeration |
| **Staging** | Capability preparation | File drops, credential access |
| **Execution** | Active attack | Process injection, command execution |
| **Exfil** | Data extraction | Large transfers, encoding |
| **Cleanup** | Evidence removal | Log deletion, timestomping |

### 3.3 Implementation

```rust
// ctas7-foundation-math/src/hmm.rs

use ndarray::{Array1, Array2};

/// Hidden Markov Model for Adversary Phase Detection
pub struct HmmPhaseDetector {
    /// State names
    hidden_states: Vec<String>,
    
    /// Transition probability matrix A[i,j] = P(state_j | state_i)
    transition_probs: Array2<f64>,
    
    /// Emission probability matrix B[i,k] = P(observation_k | state_i)
    emission_probs: Array2<f64>,
    
    /// Initial state distribution π[i] = P(state_i at t=0)
    initial_probs: Array1<f64>,
}

impl HmmPhaseDetector {
    /// Viterbi algorithm: Find most likely state sequence
    pub fn viterbi(&self, observations: &[usize]) -> Vec<usize> {
        let n_states = self.hidden_states.len();
        let n_obs = observations.len();
        
        // δ[t,i] = max probability of reaching state i at time t
        let mut delta = Array2::<f64>::zeros((n_obs, n_states));
        // ψ[t,i] = argmax predecessor state
        let mut psi = Array2::<usize>::zeros((n_obs, n_states));
        
        // Initialization
        for i in 0..n_states {
            delta[[0, i]] = self.initial_probs[i] 
                * self.emission_probs[[i, observations[0]]];
        }
        
        // Recursion
        for t in 1..n_obs {
            for j in 0..n_states {
                let mut max_val = 0.0;
                let mut max_state = 0;
                
                for i in 0..n_states {
                    let val = delta[[t-1, i]] * self.transition_probs[[i, j]];
                    if val > max_val {
                        max_val = val;
                        max_state = i;
                    }
                }
                
                delta[[t, j]] = max_val * self.emission_probs[[j, observations[t]]];
                psi[[t, j]] = max_state;
            }
        }
        
        // Backtrack
        let mut path = vec![0; n_obs];
        path[n_obs - 1] = delta.row(n_obs - 1)
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap();
            
        for t in (0..n_obs-1).rev() {
            path[t] = psi[[t+1, path[t+1]]];
        }
        
        path
    }
    
    /// Forward algorithm: Compute observation likelihood
    pub fn forward(&self, observations: &[usize]) -> f64 {
        let n_states = self.hidden_states.len();
        let n_obs = observations.len();
        
        let mut alpha = Array2::<f64>::zeros((n_obs, n_states));
        
        // Initialization
        for i in 0..n_states {
            alpha[[0, i]] = self.initial_probs[i] 
                * self.emission_probs[[i, observations[0]]];
        }
        
        // Recursion
        for t in 1..n_obs {
            for j in 0..n_states {
                let sum: f64 = (0..n_states)
                    .map(|i| alpha[[t-1, i]] * self.transition_probs[[i, j]])
                    .sum();
                alpha[[t, j]] = sum * self.emission_probs[[j, observations[t]]];
            }
        }
        
        // Termination
        alpha.row(n_obs - 1).sum()
    }
    
    /// Detect current adversary phase from observation sequence
    pub fn detect_phase(&self, observations: &[usize]) -> (String, f64) {
        let state_sequence = self.viterbi(observations);
        let current_state = *state_sequence.last().unwrap_or(&0);
        let current_phase = self.hidden_states[current_state].clone();
        
        let likelihood = self.forward(observations);
        
        (current_phase, likelihood)
    }
}
```

### 3.4 H2 Contribution

HMM output contributes to H2 (Semantic) score:

```rust
pub fn hmm_h2_contribution(phase: &str, likelihood: f64) -> f64 {
    let phase_weight = match phase {
        "Recon" => 0.2,
        "Staging" => 0.4,
        "Execution" => 0.8,
        "Exfil" => 0.9,
        "Cleanup" => 0.6,
        _ => 0.1,
    };
    
    // Combine phase severity with detection confidence
    phase_weight * likelihood.min(1.0)
}
```

---

## 4. Matroids (Information Independence)

### 4.1 Purpose

Measure whether new intelligence fragments provide **independent information** or are redundant.

| Condition | Meaning |
|-----------|---------|
| High rank delta | New independent information arrived |
| Low rank delta | Redundant/already known data |
| Rank saturation | Fully informed state reached |

### 4.2 Implementation

```rust
// ctas7-glaf-matroid-core/src/matroid.rs

use nalgebra::{DMatrix, Vector3};

/// Intelligence fragment with embedding vector
#[derive(Debug, Clone)]
pub struct Fragment {
    pub id: u64,
    pub vector: Vector3<f64>,  // 3D demo (768D in production)
    pub confidence: f64,
}

/// Latent Matroid for measuring information independence
pub struct LatentMatroid {
    pub ground_set: Vec<Fragment>,
}

impl LatentMatroid {
    /// Calculate rank of subset (number of linearly independent vectors)
    pub fn calculate_rank(&self, subset_indices: &[usize]) -> usize {
        if subset_indices.is_empty() {
            return 0;
        }
        
        let vectors: Vec<Vector3<f64>> = subset_indices
            .iter()
            .filter_map(|&i| self.ground_set.get(i).map(|f| f.vector))
            .collect();
        
        let num_cols = vectors.len();
        if num_cols == 0 {
            return 0;
        }
        
        let matrix_data: Vec<f64> = vectors
            .into_iter()
            .flat_map(|v| v.iter().copied())
            .collect();
        
        let matrix = DMatrix::from_column_slice(3, num_cols, &matrix_data);
        matrix.rank(1e-6)
    }
    
    /// Measure rank change when adding new fragment
    pub fn rank_delta(&self, existing: &[usize], new_idx: usize) -> usize {
        let old_rank = self.calculate_rank(existing);
        
        let mut new_set = existing.to_vec();
        if !new_set.contains(&new_idx) {
            new_set.push(new_idx);
        }
        
        let new_rank = self.calculate_rank(&new_set);
        new_rank.saturating_sub(old_rank)
    }
}
```

### 4.3 H2 Score Calculation

```rust
/// Calculate H2 score from matroid analysis
pub fn matroid_h2_score(matroid: &LatentMatroid, indices: &[usize]) -> f64 {
    let rank = matroid.calculate_rank(indices);
    let count = indices.len();
    
    if count == 0 {
        return 0.0;
    }
    
    // H2 = rank / count (independence ratio)
    // 1.0 = all fragments independent (maximum information)
    // 0.5 = half redundant
    rank as f64 / count as f64
}
```

---

## 5. Hawkes Process (Temporal Self-Excitation)

### 5.1 Purpose

Detect **self-exciting patterns** where one event increases the probability of future events (like aftershocks or accelerating attack chains).

### 5.2 Mathematical Definition

Conditional intensity function:

```
λ(t) = μ + Σ α × e^(-β(t - τᵢ))
```

Where:
- `μ` = baseline rate
- `α` = excitation strength
- `β` = decay rate
- `τᵢ` = arrival times of past events

### 5.3 Implementation

```rust
// ctas7-foundation-math/src/hawkes.rs

/// Hawkes Process for detecting self-exciting event patterns
pub struct HawkesProcess {
    /// Baseline event rate (μ)
    pub mu: f64,
    
    /// Excitation strength (α) - how much each event excites future events
    pub alpha: f64,
    
    /// Decay rate (β) - how quickly excitation fades
    pub beta: f64,
    
    /// Arrival times of past events (τᵢ)
    pub arrival_times: Vec<f64>,
}

impl HawkesProcess {
    pub fn new(mu: f64, alpha: f64, beta: f64) -> Self {
        Self {
            mu,
            alpha,
            beta,
            arrival_times: Vec::new(),
        }
    }
    
    /// Record a new event
    pub fn record_event(&mut self, time: f64) {
        self.arrival_times.push(time);
    }
    
    /// Calculate conditional intensity λ*(t) at time t
    pub fn conditional_intensity(&self, t: f64) -> f64 {
        let excitation_sum: f64 = self.arrival_times
            .iter()
            .filter(|&&tau_i| tau_i < t)
            .map(|&tau_i| self.alpha * (-self.beta * (t - tau_i)).exp())
            .sum();
        
        self.mu + excitation_sum
    }
    
    /// Calculate H1 contribution (normalized intensity)
    pub fn h1_score(&self, t: f64) -> f64 {
        let intensity = self.conditional_intensity(t);
        
        // Normalize using sigmoid-like function
        // High intensity (>5) → score approaching 1.0
        (intensity / 5.0).min(1.0)
    }
    
    /// Detect if currently in self-exciting burst
    pub fn is_bursting(&self, t: f64, threshold: f64) -> bool {
        self.conditional_intensity(t) > threshold
    }
}
```

### 5.4 Intensity Visualization

```
λ(t)
  │
5 ┤                    ╱╲
  │                   ╱  ╲
4 ┤         ╱╲       ╱    ╲
  │        ╱  ╲     ╱      ╲
3 ┤   ╱╲  ╱    ╲   ╱        ╲
  │  ╱  ╲╱      ╲_╱          ╲
2 ┤ ╱                          ╲___
  │╱                                ╲___
1 ┼─────────────────────────────────────── μ (baseline)
  │
0 ┼────┬────┬────┬────┬────┬────┬────┬───▶ t
       │    │    │    │
      τ₁   τ₂   τ₃   τ₄  (event arrivals)
```

---

## 6. OODA Orient Decision

### 6.1 Convergence Line

The **75% Convergence Line** is the decision boundary:

```
H2 (Semantic)
  │
1.0┤          ┌─────────────────┐
   │          │  ABOVE LINE     │
   │          │  → Phase Trans. │
0.75├──────────┼─────────────────┤
   │          │                 │
   │  BELOW   │                 │
   │  → Hunt  │                 │
0.0┼──────────┴─────────────────┴──▶ H1 (Operational)
  0.0        0.75              1.0
```

### 6.2 Implementation

```rust
// ctas7-atlas-daemon/src/orient.rs

/// OODA Orient phase decision (RFC-9022 §5.3)
pub fn ooda_orient_decision(
    h1_score: f64,
    h2_score: f64,
    current_level: VerticalLevel,
) -> OrientResult {
    // 75% is the canonical convergence line (RFC-9021 §2.1)
    const THRESHOLD: f64 = 0.75;
    
    let above_line = h1_score >= THRESHOLD && h2_score >= THRESHOLD;
    
    let hd4_recommendation = if above_line {
        // Use H2 to determine how far to escalate
        match h2_score {
            s if s > 0.90 => Hd4Phase::Dominate,
            s if s > 0.85 => Hd4Phase::Disrupt,
            s if s > 0.80 => Hd4Phase::Disable,
            _ => Hd4Phase::Detect,
        }
    } else {
        Hd4Phase::Hunt  // Below line, cycle back to Observe
    };
    
    let should_escalate = check_escalation_criteria(
        current_level,
        h1_score + h2_score,
    );
    
    OrientResult {
        hd4_recommendation,
        should_escalate,
        above_line,
        h1_score,
        h2_score,
    }
}

/// Check vertical escalation criteria (RFC-9022 §3.1)
fn check_escalation_criteria(
    current_level: VerticalLevel,
    combined_score: f64,
) -> bool {
    match current_level {
        VerticalLevel::Tactical => {
            // Escalate to Operational if:
            // - Combined score > 1.5 (high confidence)
            // - Lateral movement implied
            // - Crown jewel touched
            combined_score > 1.5
        }
        VerticalLevel::Operational => {
            // Escalate to Strategic if:
            // - Nation-state indicators
            // - Multi-site impact
            combined_score > 1.8
        }
        VerticalLevel::Strategic => false,
    }
}

#[derive(Debug, Clone)]
pub struct OrientResult {
    pub hd4_recommendation: Hd4Phase,
    pub should_escalate: bool,
    pub above_line: bool,
    pub h1_score: f64,
    pub h2_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalLevel {
    Tactical,    // PLC/RTU/Device
    Operational, // SCADA/DCS/Site
    Strategic,   // Enterprise/Nation
}
```

---

## 7. Integration with ATLAS Daemon

### 7.1 1ms Cognitive Tick

```rust
// ctas7-atlas-daemon/src/cognitive_loop.rs

pub struct AtlasDaemon {
    hawkes: HawkesProcess,
    matroid: LatentMatroid,
    hmm: HmmPhaseDetector,
    h2_cache: Option<H2ConvergencePayload>,
}

impl AtlasDaemon {
    /// Main cognitive tick (1ms rate, Zone B)
    pub async fn tick(&mut self, event: Option<IndustrialEvent>) -> OrientResult {
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;
        
        // Record event if present
        if let Some(e) = event {
            self.hawkes.record_event(now);
        }
        
        // H1: Fast operational score (Hawkes intensity)
        let h1_score = self.hawkes.h1_score(now);
        
        // H2: Use cached semantic score (async refresh)
        let h2_score = self.h2_cache
            .as_ref()
            .map(|c| c.effective_score())
            .unwrap_or(0.0);
        
        // Orient decision
        ooda_orient_decision(h1_score, h2_score, VerticalLevel::Tactical)
    }
    
    /// Background H2 refresh (Zone C, async)
    pub async fn refresh_h2(&mut self, fragments: &[usize]) {
        // Matroid rank
        let matroid_score = matroid_h2_score(&self.matroid, fragments);
        
        // HMM phase (would need observation sequence)
        // let (phase, likelihood) = self.hmm.detect_phase(&observations);
        
        // Update cache
        self.h2_cache = Some(H2ConvergencePayload {
            semantic_score: matroid_score,
            matroid_rank_current: self.matroid.calculate_rank(fragments),
            matroid_rank_delta: 0,
            estimated_adversary_phase: AdversaryPhase::Recon,
            secondary_hash_sch: String::new(),
            generated_at_utc: Utc::now(),
            recommended_hd4_phase: Hd4Phase::Hunt,
        });
    }
}
```

---

## 8. Summary

| Model | Purpose | Output | Contributes To |
|-------|---------|--------|----------------|
| **Hawkes Process** | Detect self-exciting bursts | λ(t) intensity | H1 (Operational) |
| **Δ-Angle Derivative** | Measure context drift | Angular change | H1 (Operational) |
| **Matroid Rank** | Measure information independence | r(S) rank | H2 (Semantic) |
| **HMM Viterbi** | Detect adversary phase | Hidden state | H2 (Semantic) |

**Decision Rule:**
```
IF H1 ≥ 0.75 AND H2 ≥ 0.75 THEN
    Above Convergence Line → HD4 Phase Transition
ELSE
    Below Line → Continue Hunt (OODA Observe)
```

---

## 9. References

- RFC-9021: Graph Convergence Theory
- RFC-9023: GLAF Matroid Convergence Mathematics
- RFC-9024: H2 Convergence Service Contract
- RFC-9022: OODA Vertical Escalation

---

**End of RFC-9025**
