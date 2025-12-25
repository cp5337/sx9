# RFC-9024/9025: Dual H1/H2 Architecture

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

The Dual H1/H2 Architecture separates operational (fast-path) and semantic (slow-path) processing into two complementary layers.

### 1.1 Design Principle

**Dual Neurotransmitter Analogy:**
- **H1 (Dopamine):** Fast, immediate response, action-oriented
- **H2 (Serotonin):** Slow, modulatory, meaning-oriented

Together they provide stable decision-making: fast H1 + slow H2 = balanced cognition.

---

## 2. H1 (Operational Layer) - Fast Path

### 2.1 Characteristics

| Property | Value |
|----------|-------|
| **Format** | JSON (SPIRES format) |
| **Target Latency** | <50µs (Bernoulli zone) |
| **Purpose** | Real-time execution via Legion ECS |
| **Zone** | Zone B (ATLAS Daemon) |
| **Trivariate** | Primary (Tactical/Execution) |

### 2.2 H1 Encodes

- 32 PTCC primitives
- HD4 phase
- Domain mask
- Execution tier
- Immediate action commands

### 2.3 H1 Payload Structure

```rust
#[derive(Serialize, Deserialize)]
pub struct H1Payload {
    pub primitive_id: u8,         // PTCC primitive (0-31)
    pub hd4_phase: Hd4Phase,      // Hunt/Detect/Disrupt/Disable/Dominate
    pub domain_mask: u16,         // Bit mask for active realms
    pub execution_tier: u8,       // 0=immediate, 1=queued, 2=deferred
    pub delta_angle: DeltaPositionFixed,
    pub timestamp_us: u64,
    pub correlation_id: Uuid,
}

impl H1Payload {
    /// Serialize to JSON (SPIRES format)
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
```

---

## 3. H2 (Semantic Layer) - Slow Path

### 3.1 Characteristics

| Property | Value |
|----------|-------|
| **Format** | TOML (semantic definitions) |
| **Target Latency** | Seconds to Hours (async acceptable) |
| **Purpose** | Deep analysis via GLAF processor |
| **Zone** | Zone C (GLAF Matroid Core) |
| **Trivariate** | Secondary (Semantic/Analysis) |

### 3.2 H2 Encodes

- Entity type
- Algorithm selection
- Confidence scores
- Semantic embeddings
- Contextual meaning

### 3.3 H2 Convergence Payload

```rust
#[derive(Serialize, Deserialize)]
pub struct H2ConvergencePayload {
    pub semantic_score: f64,                    // H2 Score (0.0 - 1.0)
    pub matroid_rank_current: u32,
    pub matroid_rank_delta: i32,
    pub estimated_adversary_phase: AdversaryPhase,
    pub secondary_hash_sch: String,             // SCH of semantic analysis
    pub generated_at_utc: DateTime<Utc>,
    pub recommended_hd4_phase: Hd4Phase,
}

#[derive(Serialize, Deserialize)]
pub enum AdversaryPhase {
    Recon,
    Staging,
    Execution,
    Exfil,
}
```

### 3.4 H2 TOML Definition Format

```toml
[entity]
type = "threat_actor"
id = "triv:ABC123_DEF456_01234567-89ab-cdef-0123-456789abcdef"

[semantic]
confidence = 0.847000
embedding_model = "text-embedding-3-large"
embedding_dim = 3072

[analysis]
algorithm = "matroid_rank"
iterations = 1000
convergence_threshold = 0.001000

[context]
domain = "CYBER"
temporal_window_hours = 24
related_entities = [
    "triv:XYZ789_...",
    "triv:QRS012_...",
]
```

---

## 4. Dual Trivariate Operation

```
Primary (Operational):   triv:[SCH-T]_[CUID-T]_[UUID-T]
Secondary (Semantic):    triv:[SCH-S]_[CUID-S]_[UUID-S]
```

### 4.1 Relationship

| Aspect | Primary (H1) | Secondary (H2) |
|--------|--------------|----------------|
| Hash Source | Tactical state | Semantic context |
| Update Frequency | Per tick (1ms) | Per analysis window |
| Lifetime | Transient | Persistent |
| Storage | Legion ECS | Supabase/SurrealDB |

---

## 5. Service Endpoints

### 5.1 H2 Convergence Query

```
Path: /api/v7/convergence/semantic/{primary_hash}
Method: GET
Zone: Zone C (Analytical)
Latency: 1ms - 100ms
Purpose: Asynchronously provides semantic context for OODA Orient
```

### 5.2 Integration Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    ATLAS Daemon (Zone B)                     │
│                                                              │
│  ┌──────────┐     1ms tick     ┌──────────────────┐         │
│  │  Legion  │ ───────────────► │  H1 Processing   │         │
│  │   ECS    │                  │  (JSON/SPIRES)   │         │
│  └──────────┘                  └────────┬─────────┘         │
│                                         │                    │
│                    Once per operational window               │
│                                         │                    │
│                                         ▼                    │
│                            ┌────────────────────┐            │
│                            │  Query H2 Score    │            │
│                            │  GET /convergence  │            │
│                            └─────────┬──────────┘            │
└──────────────────────────────────────┼───────────────────────┘
                                       │
                                       ▼
┌──────────────────────────────────────────────────────────────┐
│                    GLAF Matroid Core (Zone C)                │
│                                                              │
│  ┌──────────────────┐        ┌──────────────────┐           │
│  │  H2 Processing   │ ◄───── │  Semantic Store  │           │
│  │  (TOML/Analysis) │        │  (SurrealDB)     │           │
│  └────────┬─────────┘        └──────────────────┘           │
│           │                                                  │
│           ▼                                                  │
│  ┌──────────────────┐                                       │
│  │ H2Convergence    │                                       │
│  │ Payload Response │                                       │
│  └──────────────────┘                                       │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Format Rules

### 6.1 Critical Constraints

| Layer | Format | Rationale |
|-------|--------|-----------|
| H1 | JSON | Fast parsing, operational |
| H2 | TOML | Semantic definitions, human-readable |
| DSL | TOML | ATLAS daemon configuration |
| Messages | JSON | NATS/JetStream transport |

### 6.2 TOML for Semantic, JSON for Operational

```
TOML → Semantic definitions, configuration, analysis specs
JSON → Operational messages, real-time state, API responses
```

This maintains **full determinism** - TOML defines what, JSON executes when.

---

## 7. Convergence Score Calculation

```rust
pub struct ConvergenceScore {
    pub h1_score: f64,    // Operational confidence (0.0 - 1.0)
    pub h2_score: f64,    // Semantic confidence (0.0 - 1.0)
    pub combined: f64,    // Weighted combination
}

impl ConvergenceScore {
    pub fn calculate(h1: &H1Payload, h2: &H2ConvergencePayload) -> Self {
        let h1_score = Self::evaluate_h1(h1);
        let h2_score = h2.semantic_score;
        
        // Weighted combination: 60% operational, 40% semantic
        let combined = 0.6 * h1_score + 0.4 * h2_score;
        
        Self { h1_score, h2_score, combined }
    }
    
    fn evaluate_h1(h1: &H1Payload) -> f64 {
        // Evaluate based on delta angle magnitude, tier, phase
        let delta_mag = h1.delta_angle.magnitude();
        let tier_factor = match h1.execution_tier {
            0 => 1.0,
            1 => 0.8,
            _ => 0.6,
        };
        (1.0 - delta_mag) * tier_factor
    }
}
```

---

## 8. Zone Architecture

| Zone | Name | Layer | Latency |
|------|------|-------|---------|
| A | Edge/CDN | Pre-processing | <1ms |
| B | ATLAS Daemon | H1 Operational | <50µs |
| C | GLAF Matroid | H2 Semantic | 1ms-100ms |
| D | Deep Analysis | Batch | Seconds-Hours |

---

## Critical Constraints

- **H1 = JSON** - Always SPIRES format
- **H2 = TOML** - Always semantic definitions
- **H2 queries are async** - Never block H1 hot path
- **Convergence once per window** - Not per tick
- **Legion ECS for H1** - NOT alternative ECS frameworks

---

## References

- RFC-9020: Trivariate Hashing
- RFC-9301: TCR Triad
- RFC-9400: NATS Architecture
