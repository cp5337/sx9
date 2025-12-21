# RFC-9100 — Dual-Trivariate PTCC Integration & Delta-Angle Expansion

**Version:** 1.0
**Status:** Draft
**Date:** November 26, 2025
**Applies To:** Synaptix9, CTAS-7.3.1, PLASMA, GLAF, SlotGraph, Legion ECS
**Author:** CTAS Core Engineering Group
**Dependencies:** RFC-9001, RFC-9002, RFC-9003, RFC-9005

---

## 1. Purpose

This RFC extends RFC-9001 to define the operational semantics of **Dual-Trivariate** hashing in the context of the validated **32 PTCC Universal Primitives**, expanding the **Delta-Angle (Δ-angle)** specification for cognitive system integration.

This RFC SHALL:

1. Define the Primary and Secondary Trivariate roles
2. Specify 32 primitive encoding in SCH
3. Expand Δ-angle semantics for cognitive operations
4. Define Lisp compression operators
5. Integrate GLAF processor with dual-trivariate architecture

---

## 2. The 32 Universal Primitives (PTCC 7.0 Validated)

Per PTCC 7.0 Stock Market Ultimate Universality Test, the following 32 primitives are domain-universal:

### 2.1 Primitive Categories

| Category                    | Primitives                                | Bit Range |
| --------------------------- | ----------------------------------------- | --------- |
| **Core CRUD** (4)           | CREATE, READ, UPDATE, DELETE              | 0x00-0x03 |
| **Communication** (2)       | SEND, RECEIVE                             | 0x04-0x05 |
| **Data Processing** (2)     | TRANSFORM, VALIDATE                       | 0x06-0x07 |
| **Control Flow** (4)        | BRANCH, LOOP, RETURN, CALL                | 0x08-0x0B |
| **Network Operations** (4)  | CONNECT, DISCONNECT, ROUTE, FILTER        | 0x0C-0x0F |
| **Security** (4)            | AUTHENTICATE, AUTHORIZE, ENCRYPT, DECRYPT | 0x10-0x13 |
| **Resource Management** (4) | ALLOCATE, DEALLOCATE, LOCK, UNLOCK        | 0x14-0x17 |
| **State Management** (4)    | SAVE, RESTORE, CHECKPOINT, ROLLBACK       | 0x18-0x1B |
| **Coordination** (4)        | COORDINATE, SYNCHRONIZE, SIGNAL, WAIT     | 0x1C-0x1F |

### 2.2 Domain Mappings (Stock Market Proof)

```
┌─────────────────┬────────────────────┬────────────────────┐
│ Primitive       │ Stock Market       │ Threat Intel       │
├─────────────────┼────────────────────┼────────────────────┤
│ CREATE          │ open_position      │ spawn_actor        │
│ READ            │ market_research    │ reconnaissance     │
│ UPDATE          │ adjust_position    │ modify_payload     │
│ DELETE          │ close_position     │ eliminate_target   │
│ SEND            │ place_order        │ exfiltrate_data    │
│ RECEIVE         │ get_market_data    │ receive_c2         │
│ TRANSFORM       │ calculate_indicators│ encode_payload     │
│ VALIDATE        │ verify_signals     │ verify_foothold    │
│ BRANCH          │ conditional_trading│ conditional_action │
│ LOOP            │ recurring_strategies│ persistence_loop  │
│ RETURN          │ exit_strategy      │ extract_results    │
│ CALL            │ execute_strategy   │ invoke_technique   │
│ CONNECT         │ connect_broker     │ establish_c2       │
│ DISCONNECT      │ logout_session     │ terminate_session  │
│ ROUTE           │ order_routing      │ lateral_movement   │
│ FILTER          │ screen_stocks      │ filter_targets     │
│ AUTHENTICATE    │ verify_identity    │ credential_access  │
│ AUTHORIZE       │ check_permissions  │ privilege_escalation│
│ ENCRYPT         │ secure_transactions│ encrypt_comms      │
│ DECRYPT         │ decode_signals     │ decrypt_payload    │
│ ALLOCATE        │ allocate_capital   │ allocate_resources │
│ DEALLOCATE      │ free_capital       │ release_resources  │
│ LOCK            │ reserve_funds      │ lock_target        │
│ UNLOCK          │ release_funds      │ unlock_access      │
│ SAVE            │ save_portfolio     │ persist_foothold   │
│ RESTORE         │ restore_positions  │ restore_access     │
│ CHECKPOINT      │ snapshot_state     │ checkpoint_progress│
│ ROLLBACK        │ undo_trades        │ rollback_changes   │
│ COORDINATE      │ coordinate_trades  │ coordinate_attack  │
│ SYNCHRONIZE     │ sync_portfolios    │ sync_c2_nodes      │
│ SIGNAL          │ send_alerts        │ trigger_action     │
│ WAIT            │ wait_for_signal    │ await_instruction  │
└─────────────────┴────────────────────┴────────────────────┘
```

---

## 3. Dual-Trivariate Specification

Per RFC-9001 §2.2, Dual-Trivariate consists of Primary and Secondary hash sets.

### 3.1 Primary Trivariate (Tactical/Execution)

**Purpose:** Real-time operational execution via SlotGraph/Legion ECS

**Format:** `triv:[SCH-T]_[CUID-T]_[UUID-T]`

**SCH-T Encoding (24 chars):**

- Bits 0-4: Primitive ID (32 values)
- Bits 5-7: HD4 Phase (Hunt/Detect/Disrupt/Disable/Dominate)
- Bits 8-11: Domain Mask
- Bits 12-15: Execution Mask
- Remaining: N-V-N-N Grammar Tokens

**CUID-T Slot Mapping (16 chars, per RFC-9001 §6.1):**
| Slots | Meaning | Value |
|-------|---------|-------|
| 1-4 | Timestamp shard | ContextFrame.timestamp |
| 5-7 | Execution Env | WASM/Container/Microkernel |
| 8-9 | Agent ID | Legion world + entity |
| **10-11** | **Δ-Angle Derivative** | 0x00-0xFF |
| 12 | State Flag | Cold/Warm/Hot/L2 |
| 13-14 | Lineage | Parent trivariate |
| 15-16 | Nonce | Random salt |

**Target Latency:** < 50μs (Bernoulli zone)

### 3.2 Secondary Trivariate (Semantic/Analysis)

**Purpose:** Deep semantic analysis via GLAF processor

**Format:** `triv:[SCH-S]_[CUID-S]_[UUID-S]`

**SCH-S Encoding (24 chars):**

- Bits 0-3: Entity Type (16 values)
- Bits 4-7: Domain (8 values)
- Bits 8-11: Analysis Algorithm
- Remaining: Semantic context tokens

**Entity Types:**
| ID | Type | Description |
|----|------|-------------|
| 0x0 | THREAT_ACTOR | PTCC configurations |
| 0x1 | TECHNIQUE | ATT&CK T-codes |
| 0x2 | TOOL | Software/exploits |
| 0x3 | SIGNATURE | Sigma/YARA/Nuclei |
| 0x4 | SCENARIO | Attack chains |
| 0x5 | INDICATOR | IOCs |
| 0x6 | INFRASTRUCTURE | C2/domains |
| 0x7 | MITIGATION | Defenses/playbooks |
| 0x8 | GROUND_STATION | OSINT sources |
| 0x9 | ASSET | Resources |
| 0xA | MARKET | Financial instruments |
| 0xB | IDENTITY | Personas/accounts |

**CUID-S Slot Mapping (16 chars):**
| Slots | Meaning | Value |
|-------|---------|-------|
| 1-4 | Analysis run ID | Timestamp |
| 5-8 | Graph node ID | Node hash |
| 9-11 | Algorithm ID | TETH/k-NN/Monte Carlo |
| 12-13 | Confidence | 0-99% |
| 14 | Version | Result version |
| 15-16 | Salt | Random |

**Target Latency:** Seconds-Hours (async acceptable)

### 3.3 Dual-Trivariate Generation Rules

Per RFC-9001 §2.2:

**Secondary MUST be auto-generated for:**

- Synaptix9 operations
- ATLAS cognitive operations
- PLASMA gate operations
- GLAF analysis operations
- OrbitalOS tier operations

**Secondary MAY be omitted for:**

- Low-tier toolchain operations (Tier 1-2)
- Local execution
- Administrative operations

---

## 4. Delta-Angle (Δ-Angle) Specification

### 4.1 Delta Measurement Components

Per `delta_operator.rs`, delta measurement consists of:

```rust
/// Six-decimal precision delta position (0.000000-1.000000)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeltaPosition {
    /// X-axis: Semantic (MITRE kill chain stage)
    pub x: f64,  // 0.000000 - 1.000000
    /// Y-axis: Operational (HD4 phase)
    pub y: f64,  // 0.000000 - 1.000000
    /// Z-axis: Temporal (time correlation)
    pub z: f64,  // 0.000000 - 1.000000
}

impl DeltaPosition {
    /// Round to exactly 6 decimal places
    #[inline]
    pub fn round6(v: f64) -> f64 {
        (v * 1_000_000.0).round() / 1_000_000.0
    }

    /// Create with automatic 6-decimal rounding and clamping
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Self::round6(x.clamp(0.0, 1.0)),
            y: Self::round6(y.clamp(0.0, 1.0)),
            z: Self::round6(z.clamp(0.0, 1.0)),
        }
    }

    /// Calculate angular difference (normalized 0.0-1.0)
    pub fn angular_diff(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        // Maximum possible distance in unit cube = sqrt(3)
        Self::round6((dist / 3.0_f64.sqrt()).min(1.0))
    }
}

/// Delta measurement with normalized values
#[derive(Debug, Clone, Copy)]
pub struct DeltaMeasurement {
    /// Current delta position (6-decimal precision)
    pub position: DeltaPosition,
    /// Entropy drift (0.000000-1.000000)
    pub entropy_drift: f64,
    /// Semantic drift (0.000000-1.000000)
    pub semantic_drift: f64,
    /// Combined noise score (0.000000-1.000000)
    pub noise_score: f64,
}
```

**Noise Score Formula:**

```rust
impl DeltaMeasurement {
    /// Calculate noise score (normalized 0.0-1.0)
    pub fn calculate_noise_score(
        angular_diff: f64,
        entropy_drift: f64,
        semantic_drift: f64,
    ) -> f64 {
        DeltaPosition::round6(
            (angular_diff * 0.4) + (entropy_drift * 0.3) + (semantic_drift * 0.3)
        )
    }
}
```

### 4.2 Supersession Thresholds (per RFC-9001 §7.1)

| Δ-Angle (Normalized) | Δ-Angle (Degrees) | Class    | Action                       | Trivariate Impact                      |
| -------------------- | ----------------- | -------- | ---------------------------- | -------------------------------------- |
| < 0.011111           | < 2°              | None     | No action                    | None                                   |
| 0.011111-0.055556    | 2-10°             | Micro    | Adjust CUID only             | CUID-T slots 10-11 updated             |
| 0.055556-0.138889    | 10-25°            | Soft     | Regenerate SCH + CUID        | Primary trivariate regenerated         |
| 0.138889-0.333333    | 25-60°            | Hard     | Full trivariate regeneration | Both Primary and Secondary regenerated |
| > 0.333333           | > 60°             | Critical | New lineage                  | New dual-trivariate, lineage broken    |

### 4.3 Δ-Angle Computation

```rust
/// Compute delta position between two contexts
pub fn compute_delta_position(&self, ctx1: &ContextFrame, ctx2: &ContextFrame) -> DeltaPosition {
    DeltaPosition::new(
        // X-axis: Semantic (MITRE stage progression)
        ctx2.semantic_position - ctx1.semantic_position,
        // Y-axis: Operational (HD4 phase)
        ctx2.operational_position - ctx1.operational_position,
        // Z-axis: Temporal (time correlation)
        ctx2.temporal_position - ctx1.temporal_position,
    )
}

/// Compute angular difference (normalized 0.0-1.0)
pub fn compute_angular_diff(&self, ctx1: &ContextFrame, ctx2: &ContextFrame) -> f64 {
    let pos1 = self.get_delta_position(ctx1);
    let pos2 = self.get_delta_position(ctx2);
    pos1.angular_diff(&pos2)
}
```

### 4.4 Δ-Angle in CUID Encoding

CUID slots 10-11 encode the Δ-angle derivative:

| Byte | Encoding                                                     | Range     |
| ---- | ------------------------------------------------------------ | --------- |
| 10   | Angle class (3 bits) + Sign (1 bit) + Magnitude MSB (4 bits) | 0x00-0xFF |
| 11   | Magnitude LSB (8 bits)                                       | 0x00-0xFF |

**Angle Class Encoding (bits 7-5 of slot 10):**

- 0b000: None (< 0.011111)
- 0b001: Micro (0.011111-0.055556)
- 0b010: Soft (0.055556-0.138889)
- 0b011: Hard (0.138889-0.333333)
- 0b100: Critical (> 0.333333)
- 0b101-0b111: Reserved

**Sign Bit (bit 4 of slot 10):**

- 0: Positive drift (increasing complexity)
- 1: Negative drift (decreasing complexity)

**Magnitude (12 bits across slots 10-11):**

- 0x000-0xFFF: 0.000000 to 1.000000 in 0.000244 increments (1/4095)

### 4.5 Δ-Angle Semantic Meaning

| Angle Range (Normalized) | Angle Range (Degrees) | Semantic Interpretation                | Cognitive Action                  |
| ------------------------ | --------------------- | -------------------------------------- | --------------------------------- |
| 0.000-0.011              | 0-2°                  | Same operation, minor variation        | Continue execution                |
| 0.011-0.056              | 2-10°                 | Same operation, context shift          | Update context, maintain identity |
| 0.056-0.139              | 10-25°                | Related operation, significant shift   | Regenerate semantic hash          |
| 0.139-0.333              | 25-60°                | Different operation, lineage preserved | Full regeneration, track lineage  |
| 0.333-0.500              | 60-90°                | Orthogonal operation                   | Break lineage, new chain          |
| 0.500-1.000              | 90-180°               | Opposing operation                     | Potential conflict, may cancel    |

---

## 5. Lisp Compression Operators

### 5.1 Tactical Operator (τ)

```lisp
;; Format: (τ primitive slot execution)
(τ COORDINATE 0x4F2A 0x9B3E)

;; Expanded meaning:
;; - Primitive: COORDINATE (0x1C)
;; - Slot: Legion slot 0x4F2A
;; - Execution: Context hash 0x9B3E
```

### 5.2 Semantic Operator (σ)

```lisp
;; Format: (σ entity algorithm result)
(σ THREAT_ACTOR TETH_ENTROPY 0xA2F1)

;; Expanded meaning:
;; - Entity: THREAT_ACTOR (0x0)
;; - Algorithm: TETH_ENTROPY (0x0)
;; - Result hash: 0xA2F1
```

### 5.3 Unified Operator (Ω)

```lisp
;; Format: (Ω tactical semantic)
(Ω (τ COORDINATE 0x4F2A 0x9B3E)
   (σ THREAT_ACTOR TETH_ENTROPY 0xA2F1))

;; Meaning: Execute COORDINATE primitive with TETH entropy analysis context
```

### 5.4 Chain Operators

```lisp
;; Sequential execution
(Ω* (τ AUTHENTICATE slot1 ctx1)
    (τ CONNECT slot2 ctx2)
    (τ SEND slot3 ctx3))

;; Parallel execution
(Ω|| (τ ENCRYPT slot1 ctx1)
     (τ LOCK slot2 ctx2))

;; Analysis-driven execution
(Ω→ (σ THREAT_ACTOR TETH_ENTROPY result)
    (τ DISRUPT slot target))

;; Execution-fed analysis
(Ω← (τ COORDINATE slot ctx)
    (σ SCENARIO CHAIN_PREDICTION result))
```

### 5.5 Delta-Aware Operators

```lisp
;; Delta-gated execution (only if Δ < threshold)
(Ωδ< 25 (τ UPDATE slot ctx))  ; Execute only if Δ-angle < 25°

;; Delta-triggered supersession
(Ωδ> 60 (supersede (τ old) (τ new)))  ; Supersede if Δ-angle > 60°

;; Delta-adaptive execution
(Ωδ (τ primary) (τ fallback)
    :micro (adjust-cuid)
    :soft (regen-sch)
    :hard (full-regen)
    :critical (new-lineage))
```

---

## 6. GLAF Processor Integration

### 6.1 GLAF Algorithm Library

GLAF (Graph Logic Analysis Framework) provides semantic analysis for Secondary Trivariate generation:

**Graph Algorithms:**

- k-NN Similarity (768-dim embeddings)
- GraphSAGE (neighborhood aggregation)
- PageRank (importance scoring)
- Louvain (community detection)
- Spectral Clustering
- Matroid Independence

**Entropy Algorithms:**

- TETH-Topological (Shannon entropy)
- TETH-Heuristic (multi-factor scoring)
- TETH-Behavioral (pattern recognition)
- TETH-Predictive (30-day horizon)

**Probabilistic Algorithms:**

- Monte Carlo (1M+ iterations)
- Las Vegas (randomized verification)
- HMM (Hidden Markov patterns)
- Electric Football detection

### 6.2 GLAF Operations

```lisp
;; Analyze entity with algorithm
(glaf:analyze THREAT_ACTOR TETH_ENTROPY {:name "BlueDusk"})
;; Returns: Secondary Trivariate

;; Check coverage gaps
(glaf:gap entity signatures threshold)
;; Returns: Gap report + Secondary Trivariate

;; Predict chain progression
(glaf:predict entity CHAIN_PREDICTION horizon)
;; Returns: Prediction + Secondary Trivariate

;; Cluster entities
(glaf:cluster entities k-MEANS k)
;; Returns: Cluster assignments + Secondary Trivariates
```

### 6.3 GLAF → Dual-Trivariate Flow

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Tactical Input  │     │  GLAF Processor  │     │ Dual-Trivariate  │
│  (τ primitive)   │────▶│  (σ analysis)    │────▶│ (Ω unified)      │
│                  │     │                  │     │                  │
│  Primary Triv    │     │  Secondary Triv  │     │  Both Trivs      │
│  [SCH-T]_[CUID-T]│     │  [SCH-S]_[CUID-S]│     │  + Δ-angle       │
└──────────────────┘     └──────────────────┘     └──────────────────┘
```

---

## 7. Unicode Routing Integration (per RFC-9002)

### 7.1 Unicode Class Assignment

| Range       | Class    | Purpose                                   | Trivariate |
| ----------- | -------- | ----------------------------------------- | ---------- |
| U+E400-E4FF | D1       | Tactical Primitives (32 ops × 8 variants) | Primary    |
| U+E500-E5FF | D2       | GLAF Analysis Operations                  | Secondary  |
| U+E600-E6FF | D3       | Meta Orchestration (Ω operators)          | Unified    |
| U+E700-E7FF | Reserved | Delta-Aware Operations                    | Both       |

### 7.2 Primitive → Unicode Mapping

```rust
impl Primitive {
    pub fn to_unicode(&self) -> char {
        // Base offset U+E400 for tactical primitives
        let base = 0xE400u32;
        let offset = self.id() as u32;
        char::from_u32(base + offset).unwrap()
    }
}
```

---

## 8. Supersession Tracking (per RFC-9005)

### 8.1 Required Schema Fields

Per RFC-9005 compliance, entities table MUST include:

```sql
-- Supersession tracking
superseded_by: option<record<entities>>,
supersedes: option<record<entities>>,
supersession_reason: option<string>,
supersession_timestamp: option<datetime>,

-- Delta angle tracking
delta_angle: float,
delta_class: string,
noise_score: float,

-- Dual trivariate
primary_trivariate: string,
secondary_trivariate: option<string>,
```

### 8.2 Supersession Event

```sql
-- Event when supersession occurs
DEFINE EVENT entity_supersession ON TABLE entities WHEN $before.superseded_by != $after.superseded_by THEN {
    CREATE supersession_log SET
        old_entity = $before.id,
        new_entity = $after.superseded_by,
        delta_class = $after.delta_class,
        delta_angle = $after.delta_angle,
        timestamp = time::now()
};
```

---

## 9. Implementation Requirements

### 9.1 MUST Requirements

1. Every operational artifact MUST have a Primary Trivariate
2. SX9/ATLAS/PLASMA/GLAF operations MUST generate Secondary Trivariate
3. Δ-angle MUST be computed for every context transition
4. Supersession MUST occur per §4.2 thresholds
5. CUID slots 10-11 MUST encode Δ-angle per §4.4

### 9.2 SHALL Requirements

1. Lisp compression SHALL be available for all trivariates
2. GLAF processor SHALL support all algorithms in §6.1
3. Unicode routing SHALL follow RFC-9002 allocation
4. Schema SHALL include all fields per §8.1

### 9.3 MAY Requirements

1. Secondary Trivariate MAY be omitted for Tier 1-2 operations
2. Δ-angle MAY use hardware acceleration for computation
3. GLAF MAY cache results for repeated queries

---

## 10. Conformance

Systems claiming RFC-9100 conformance MUST:

1. Implement dual-trivariate generation
2. Compute Δ-angle for all context transitions
3. Apply supersession logic per thresholds
4. Support Lisp compression operators
5. Integrate with GLAF processor
6. Follow Unicode routing per RFC-9002

---

## 11. References

- RFC-9001: Synaptix9 Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9003: Operation Classifier & Escalation Logic
- RFC-9005: Unified Schema Specification
- PTCC 7.0: Stock Market Ultimate Universality Test
- delta_operator.rs: PLASMA Delta Operator implementation

---

## Appendix A: Complete Primitive Encoding Table

| ID   | Primitive    | Category      | Unicode | HD4 Affinity |
| ---- | ------------ | ------------- | ------- | ------------ |
| 0x00 | CREATE       | CRUD          | U+E400  | Hunt         |
| 0x01 | READ         | CRUD          | U+E401  | Detect       |
| 0x02 | UPDATE       | CRUD          | U+E402  | Disrupt      |
| 0x03 | DELETE       | CRUD          | U+E403  | Disable      |
| 0x04 | SEND         | Communication | U+E404  | Dominate     |
| 0x05 | RECEIVE      | Communication | U+E405  | Detect       |
| 0x06 | TRANSFORM    | Data          | U+E406  | Disrupt      |
| 0x07 | VALIDATE     | Data          | U+E407  | Detect       |
| 0x08 | BRANCH       | Control       | U+E408  | Hunt         |
| 0x09 | LOOP         | Control       | U+E409  | Hunt         |
| 0x0A | RETURN       | Control       | U+E40A  | Dominate     |
| 0x0B | CALL         | Control       | U+E40B  | Hunt         |
| 0x0C | CONNECT      | Network       | U+E40C  | Hunt         |
| 0x0D | DISCONNECT   | Network       | U+E40D  | Disable      |
| 0x0E | ROUTE        | Network       | U+E40E  | Disrupt      |
| 0x0F | FILTER       | Network       | U+E40F  | Detect       |
| 0x10 | AUTHENTICATE | Security      | U+E410  | Detect       |
| 0x11 | AUTHORIZE    | Security      | U+E411  | Detect       |
| 0x12 | ENCRYPT      | Security      | U+E412  | Disrupt      |
| 0x13 | DECRYPT      | Security      | U+E413  | Disrupt      |
| 0x14 | ALLOCATE     | Resource      | U+E414  | Hunt         |
| 0x15 | DEALLOCATE   | Resource      | U+E415  | Disable      |
| 0x16 | LOCK         | Resource      | U+E416  | Disrupt      |
| 0x17 | UNLOCK       | Resource      | U+E417  | Dominate     |
| 0x18 | SAVE         | State         | U+E418  | Dominate     |
| 0x19 | RESTORE      | State         | U+E419  | Hunt         |
| 0x1A | CHECKPOINT   | State         | U+E41A  | Detect       |
| 0x1B | ROLLBACK     | State         | U+E41B  | Disrupt      |
| 0x1C | COORDINATE   | Coordination  | U+E41C  | Dominate     |
| 0x1D | SYNCHRONIZE  | Coordination  | U+E41D  | Dominate     |
| 0x1E | SIGNAL       | Coordination  | U+E41E  | Hunt         |
| 0x1F | WAIT         | Coordination  | U+E41F  | Detect       |

---

## Appendix B: Δ-Angle Class Quick Reference

```
┌────────────────────────────────────────────────────────────────────┐
│                     Δ-ANGLE SUPERSESSION DIAGRAM                   │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  0°        2°       10°       25°       60°       90°      180°   │
│  ├─────────┼─────────┼─────────┼─────────┼─────────┼────────┤     │
│  │  NONE   │  MICRO  │  SOFT   │  HARD   │ CRITICAL│ OPPOSE │     │
│  │         │         │         │         │         │        │     │
│  │ No      │ Adjust  │ Regen   │ Full    │ New     │ Might  │     │
│  │ action  │ CUID    │ SCH+    │ regen   │ lineage │ cancel │     │
│  │         │ only    │ CUID    │ both    │         │        │     │
│  └─────────┴─────────┴─────────┴─────────┴─────────┴────────┘     │
│                                                                    │
│  Noise Score = 0.4×(Δ/180) + 0.3×(entropy) + 0.3×(semantic)       │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

**End of RFC-9100**
