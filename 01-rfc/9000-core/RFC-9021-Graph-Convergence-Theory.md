# RFC-9021: Graph Convergence Theory

**Status:** Canonical
**Author:** Charlie Payne
**Version:** 7.3.1
**Related:** RFC-9020 (HD4), RFC-9016 (Dual Trivariate), GLAF-CORE-SPEC

---

## Abstract

Graph Convergence Theory formalizes how the CTAS task graph functions as a **sensor** that detects when scattered intelligence coalesces toward predictive insight. The dual trivariate hash system (H1 Operational, H2 Semantic) provides two convergence vectors, enabling detection of threats in operational-semantic space. OODA loops serve as **escalatory triggers** for vertical context shifts (DDA).

---

## 1. The Graph as Sensor

The graph is not storage - **the graph IS a sensor.**

### 1.1 What the Graph Senses

| Signal | Detection Method |
|--------|------------------|
| **Operational Convergence** | H1 trivariate delta angles approaching zero |
| **Semantic Convergence** | H2 trivariate pattern matching against corpus |
| **Temporal Patterns** | Hawkes Process detecting event clustering |
| **Hidden States** | HMM phase detection across node sequences |
| **Structural Anomalies** | Matroid rank changes in dependency sets |

### 1.2 Two Sensors, Two Graphs

```
H1 (Operational):           H2 (Semantic):
┌─────────────────┐        ┌─────────────────┐
│  SCH + CUID +   │        │ Semantic +      │
│     UUID        │        │ BlockID +       │
│                 │        │ Belief + Auth   │
│  Fast (<50μs)   │        │  Deep (async)   │
│  "NOW" sensor   │        │ "MEANING" sensor│
└─────────────────┘        └─────────────────┘
         │                          │
         ▼                          ▼
   ████████░░ 78%           ███░░░░░░░ 32%
   "Something happening"    "Seen pieces before"
```

---

## 2. The Convergence Line

### 2.1 Informed vs Uninformed

The convergence line is a **decision boundary**:

```
100% ─────────────────────────────────────
     │  PREDICTIVE ZONE
     │  High confidence, ACT NOW window
     │
 75% ─ ─ ─ CONVERGENCE LINE ─ ─ ─ ─ ─ ─
     │
     │  UNCERTAINTY ZONE
     │  Need more collection
     │
  0% ─────────────────────────────────────
```

- **Above line** = Informed (enough signal to decide)
- **Below line** = Uninformed (need more collection)

### 2.2 The Four Quadrants

```
                    H2 Semantic HIGH
                          │
     "Pattern match,      │      "CONVERGENCE"
      quiet wire"         │       Both sensors hot
      (APT dormant?)      │       (ACT NOW)
                          │
 H1 Operational ──────────┼────────── H1 Operational
      LOW                 │               HIGH
                          │
     "Cold"               │      "Activity, no pattern"
      (Nothing or         │       (Novel attack?
       good OPSEC?)       │        False positive?)
                          │
                    H2 Semantic LOW
```

### 2.3 Diagnostic: Why Below the Line?

Delta angles reveal WHY you're below threshold:

```toml
[convergence_diagnostic]
h1_operational = 0.23
h2_semantic = 0.31

# Which nodes are NOT contributing?
h1_deltas = [
    { node = "network_recon", delta = 0.8, status = "no_signal" },
    { node = "credential_access", delta = 0.2, status = "weak_signal" },
]

# Which patterns are NOT matching?
h2_deltas = [
    { pattern = "APT29_playbook", delta = 0.7, match = "partial" },
    { pattern = "insider_threat", delta = 0.4, match = "possible" },
]

diagnosis = "Weak credential signal - expand IAM monitoring"
next_action = "Hunt phase: credential_access node"
```

---

## 3. Mathematical Foundations

### 3.1 Combinatorial Optimization

The convergence problem is fundamentally **combinatorial optimization**:

- Given: Set of intelligence fragments
- Find: Optimal assignment to task graph nodes
- Objective: Maximize convergence score
- Constraint: Time-of-value decay

```mathematica
max Σᵢ (confidence(nodeᵢ) × relevance(nodeᵢ))
s.t. Σⱼ allocation(fragmentⱼ) ≤ 1  ∀ fragments
     time_decay(fragmentⱼ) > threshold  ∀ assignments
```

GLAF APOC++ procedure: `combo.greedy_opt(objective)`

### 3.2 Hidden Markov Models (HMM)

Adversary behavior has **hidden states** (intent) and **observable emissions** (actions):

```
Hidden States:    [Recon] → [Staging] → [Execution] → [Exfil]
                     ↓          ↓           ↓          ↓
Observations:    DNS queries  Downloads   Lateral    Outbound
                                          Movement   Traffic
```

HMM detects:
- Current phase (which hidden state?)
- Phase transition probability (what's next?)
- Anomaly (observations don't fit any state sequence)

GLAF APOC++ procedure: `hmm.phase(sequence)`

### 3.3 Latent Matroids

A **matroid** captures independence structure in sets:

```mathematica
Matroid M = (E, I) where:
  E = ground set (all intelligence fragments)
  I = independent sets (non-redundant combinations)

Rank function: r(S) = max |X| where X ⊆ S and X ∈ I
```

**Latent matroid** = the matroid structure is hidden, must be learned from observations.

Application to convergence:
- **High rank change** = new independent information arrived
- **Low rank change** = redundant information (already knew this)
- **Rank saturation** = no more independent info possible (fully informed)

GLAF APOC++ procedure: `matroid.rank(set)`

### 3.4 Hawkes Process

Temporal event clustering for detecting **self-exciting patterns**:

```mathematica
λ(t) = μ + Σᵢ α × e^(-β(t-tᵢ))

where:
  λ(t) = intensity at time t
  μ = background rate
  α = excitation strength
  β = decay rate
  tᵢ = previous event times
```

Adversary activity is **self-exciting** - reconnaissance leads to staging leads to execution. Hawkes detects this acceleration.

GLAF APOC++ procedure: `timeseries.hawkes(events)`

### 3.5 TETH (Topological Entropy Threat Heuristic)

Graph entropy measures **information content** of the current state:

```mathematica
H(G) = -Σᵥ p(v) log p(v)

where p(v) = degree(v) / Σ degree(w)
```

**High entropy** = scattered, uncertain
**Low entropy** = concentrated, converging

GLAF APOC++ procedure: `teth.entropy(node)`

---

## 4. OODA as Escalatory Trigger

### 4.1 The Vertical Problem

HD4 phases operate at multiple levels (tactical → national). How do you know when to **escalate** up the vertical?

**Answer: OODA loop completion triggers vertical escalation.**

### 4.2 Nested OODA at Each Level

```
Tactical OODA:
  Observe → Orient → Decide → Act
       ↓
  If Act = "escalate" → Trigger Operational OODA

Operational OODA:
  Observe → Orient → Decide → Act
       ↓
  If Act = "escalate" → Trigger Strategic OODA

Strategic OODA:
  Observe → Orient → Decide → Act
       ↓
  If Act = "escalate" → Trigger National OODA
```

### 4.3 Escalation Criteria

| Trigger | From | To | Criteria |
|---------|------|-----|----------|
| Scope expansion | Tactical | Operational | Threat affects >1 device/node |
| Campaign detection | Operational | Strategic | Pattern matches known APT |
| Sector impact | Strategic | National | Critical infrastructure at risk |

### 4.4 OODA + Convergence

The OODA "Orient" phase IS the convergence calculation:

```
Observe: Collect intelligence fragments
Orient:  Calculate H1/H2 convergence scores  ← THIS IS THE MATH
Decide:  Above line? → Act. Below? → Collect more.
Act:     Execute HD4 phase OR escalate OR hunt more
```

---

## 5. The Oscillation

The graph **breathes** - convergence is not static:

```
Time →
     ████░░░░░░ 42%  Intel arrives
     ██████░░░░ 58%  Correlation found
     ████████░░ 78%  APPROACHING LINE
     █████████░ 89%  ABOVE LINE - ACT NOW
     ███████░░░ 68%  Intel aged out (decay)
     █████░░░░░ 52%  Below line again
     ███████░░░ 71%  New collection arrives
```

### 5.1 Time-of-Value Decay

Intelligence has a shelf life:

```rust
fn time_decay(fragment: &Intel, now: Timestamp) -> f64 {
    let age = now - fragment.collected_at;
    let half_life = fragment.intel_type.half_life();

    // Exponential decay
    0.5_f64.powf(age.as_secs_f64() / half_life.as_secs_f64())
}
```

### 5.2 Convergence Window

When both H1 and H2 exceed threshold simultaneously = **convergence window**

```rust
fn detect_convergence_window(h1: f64, h2: f64, threshold: f64) -> Option<ActionWindow> {
    if h1 >= threshold && h2 >= threshold {
        Some(ActionWindow {
            confidence: (h1 + h2) / 2.0,
            recommended_action: determine_hd4_phase(h1, h2),
            window_estimate: estimate_decay_time(h1, h2),
        })
    } else {
        None
    }
}
```

---

## 6. Implementation in GLAF

### 6.1 Cypher++ Convergence Queries

```cypher
// Calculate convergence for a threat cluster
MATCH (t:Threat)-[:INDICATES]->(a:Activity)
WHERE t.risk > 50
CALL teth.entropy(t) YIELD entropy
CALL hmm.phase(collect(a)) YIELD current_phase, transition_prob
CALL matroid.rank(collect(a)) YIELD rank, rank_delta
WITH t, entropy, current_phase, transition_prob, rank_delta

// Convergence score
WITH t,
     (1.0 - entropy/5.0) as h1_score,
     transition_prob as h2_score
WHERE h1_score > 0.75 AND h2_score > 0.75

RETURN t, h1_score, h2_score, current_phase
ORDER BY (h1_score + h2_score) DESC
```

### 6.2 APOC++ Procedures Used

| Procedure | Purpose in Convergence |
|-----------|------------------------|
| `teth.entropy(node)` | Measure operational scatter |
| `hmm.phase(sequence)` | Detect adversary phase |
| `matroid.rank(set)` | Measure information independence |
| `timeseries.hawkes(events)` | Detect activity acceleration |
| `combo.greedy_opt(objective)` | Optimize fragment assignment |
| `lstar.learn(pattern)` | Learn adversary behavior automaton |

### 6.3 Real-Time Convergence Monitoring

```rust
pub struct ConvergenceMonitor {
    h1_threshold: f64,
    h2_threshold: f64,
    alert_callback: Box<dyn Fn(ConvergenceEvent)>,
}

impl ConvergenceMonitor {
    pub async fn monitor(&self, graph: &GLAFCore) {
        loop {
            let h1 = self.calculate_operational_convergence(graph).await;
            let h2 = self.calculate_semantic_convergence(graph).await;

            if h1 >= self.h1_threshold && h2 >= self.h2_threshold {
                (self.alert_callback)(ConvergenceEvent {
                    h1_score: h1,
                    h2_score: h2,
                    timestamp: Utc::now(),
                    recommended_action: self.recommend_action(h1, h2),
                });
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
```

---

## 7. Scope and Context

### 7.1 Same Math, Different Scope

The convergence math works at any scope:

| Scope | Example | Uninformed | Informed | Predictive |
|-------|---------|------------|----------|------------|
| **Sandwich** | Turkey allocation | "Someone's cooking" | "Making turkey sandwich" | "Extra mayo, no tomato" |
| **Tactical** | Single endpoint | "Anomaly detected" | "Credential theft" | "Lateral move in 30s" |
| **Operational** | Network | "Campaign active" | "Targeting finance" | "Exfil in 2 hours" |
| **Strategic** | Sector | "APT active" | "Critical infra targeted" | "Attack in 2 weeks" |

### 7.2 Threshold Varies with Scope

| Scope | Convergence Threshold | Time to Act |
|-------|----------------------|-------------|
| Tactical | 90% | Seconds |
| Operational | 75% | Hours |
| Strategic | 60% | Days |
| National | 50% | Weeks |

Higher scope = lower threshold because you have more time to validate.

---

## 8. Integration with HD4

### 8.1 Convergence per HD4 Phase

```
Hunt:     ████░░░░░░ 42%  "Looking for signal"
Detect:   ██████░░░░ 61%  "Signal found, characterizing"
Disable:  BLOCKED         "Need Detect > 75%"
Disrupt:  BLOCKED         "Need Disable complete"
Dominate: BLOCKED         "Need Disrupt in progress"
```

### 8.2 Phase Transition Rules

```rust
fn can_transition(current: HD4Phase, convergence: f64) -> Option<HD4Phase> {
    match current {
        HD4Phase::Hunt => (convergence > 0.50).then_some(HD4Phase::Detect),
        HD4Phase::Detect => (convergence > 0.75).then_some(HD4Phase::Disable),
        HD4Phase::Disable => (convergence > 0.85).then_some(HD4Phase::Disrupt),
        HD4Phase::Disrupt => (convergence > 0.90).then_some(HD4Phase::Dominate),
        HD4Phase::Dominate => None, // Terminal phase
    }
}
```

---

## 9. References

- RFC-9020: HD4 Framework
- RFC-9016: Dual Trivariate PTCC Integration
- GLAF Core Specification (CTAS7-GLAF-ES-001)
- Boyd, J. "OODA Loop" (Patterns of Conflict, 1986)
- Rabiner, L. "A Tutorial on Hidden Markov Models"
- Hawkes, A. "Spectra of some self-exciting and mutually exciting point processes"
- Oxley, J. "Matroid Theory"

---

## 10. Related RFCs

- RFC-9020: HD4 Framework
- RFC-9022: OODA-HD4 Integration
- RFC-9016: Dual Trivariate PTCC Integration
- RFC-9000: SX9 Core Architecture
