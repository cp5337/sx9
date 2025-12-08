# RFC-9110: SX9 Lisp Interpreter

**Status:** Implemented
**Created:** 2025-12-01
**Crate:** `sx9-lisp`
**Dependencies:** None (zero external dependencies)
**Related:** RFC-9021 (Cognitive Inference), RFC-9101 (Smart Crate System), RFC-9108 (Thalmic Filter)

---

## 1. Abstract

This RFC specifies the SX9 Lisp Interpreter, a zero-dependency HFT-optimized Lisp implementation using Unicode Private Use Area (U+E000-E9FF) as bytecode encoding. The interpreter provides the cognitive evaluation layer for the SX9 PLASMA Defender platform, integrating with 7-tier escalation, OODA loop processing, and delta angle tracking for threat detection.

## 2. Motivation

The SX9 platform requires a cognitive processing engine that:

1. **Sub-microsecond Evaluation** - HFT-grade performance for real-time threat response
2. **Zero Dependencies** - Minimal attack surface, no supply chain risk
3. **Unicode Bytecode** - Compact wire format, human-readable, Unicode sequences ARE the bytecode
4. **Escalation Integration** - Native support for 7-tier WASM→Orb escalation ladder
5. **Delta Angle Tracking** - 6-decimal precision (0.000001 radians ≈ 0.11mm) for motion analysis

Previous approaches using external crates (Steel, Ketos, rust_lisp) were rejected due to:
- Excessive complexity and dependency chains
- Performance overhead incompatible with HFT requirements
- Lack of native escalation and fire event integration

## 3. Unicode Instruction Set Architecture

### 3.1 Memory Layout

```
U+E000-E4FF  Reserved (future expansion)
U+E500-E5FF  Lisp Primitives (256 opcodes)
U+E600-E6FF  Delta Angle Operations (256 opcodes)
U+E700-E7FF  State Machine Triggers (256 opcodes)
U+E800-E8FF  Escalation Tier Triggers (256 opcodes)
U+E900-E9FF  Extensions (256 opcodes)
```

### 3.2 Lisp Primitives (U+E500-E5FF)

| Range | Category | Opcodes |
|-------|----------|---------|
| U+E500-E50A | Arithmetic | ADD, SUB, MUL, DIV, MOD, NEG, ABS, SQRT, POW, LOG, EXP |
| U+E510-E515 | Comparison | EQ, LT, GT, LE, GE, NE |
| U+E520-E523 | Logic | AND, OR, NOT, XOR |
| U+E530-E539 | Lists | CONS, CAR, CDR, LIST, LEN, NTH, APPEND, MAP, FOLD, FILTER |
| U+E540-E547 | Control | IF, COND, PROGN, LET, LAMBDA, APPLY, EVAL, QUOTE |
| U+E550-E554 | Special | NIL, TRUE, FALSE, DEFINE, SET |
| U+E560-E566 | Trigonometry | SIN, COS, TAN, ASIN, ACOS, ATAN, ATAN2 |

### 3.3 Delta Angle Operations (U+E600-E6FF)

| Range | Category | Opcodes |
|-------|----------|---------|
| U+E600-E608 | Axis Control | X_SET, Y_SET, Z_SET, X_DELTA, Y_DELTA, Z_DELTA, X_READ, Y_READ, Z_READ |
| U+E610-E613 | Rate Calculation | RATE_X, RATE_Y, RATE_Z, RATE_MAG |
| U+E620-E624 | State Detection | STATE_STABLE, STATE_DRIFT, STATE_MANEUVER, STATE_ANOMALY, STATE_RATE_EXCEED |
| U+E630-E634 | EMA Signals | EMA_CONVERGING, EMA_DIVERGING, EMA_STABLE, EMA_FAST, EMA_SLOW |
| U+E640-E645 | Hawkes Process | HAWKES_LOW, HAWKES_MEDIUM, HAWKES_HIGH, HAWKES_SPIKE, HAWKES_DECAY, HAWKES_EXCITE |

### 3.4 State Machine Triggers (U+E700-E7FF)

| Range | Category | Opcodes |
|-------|----------|---------|
| U+E700-E703 | HD4 States | HD4_DISCOVER, HD4_DETECT, HD4_DISRUPT, HD4_DOMINATE |
| U+E710-E713 | OODA Phases | OODA_OBSERVE, OODA_ORIENT, OODA_DECIDE, OODA_ACT |
| U+E720-E723 | Kill Chain | KC_RECON, KC_STAGING, KC_EXECUTION, KC_EXFIL |
| U+E730-E732 | H2 Convergence | H2_COMPUTE, H2_THRESHOLD, H2_CRISIS |

### 3.5 Escalation Tier Triggers (U+E800-E8FF)

| Opcode | Tier | System | Max Response Time |
|--------|------|--------|-------------------|
| U+E800 | 1 | WASM Sandbox | <1ms |
| U+E801 | 2 | Microkernel | <10ms |
| U+E802 | 3 | Kernel Crate | <100ms |
| U+E803 | 4 | Multi-Crates | <1s |
| U+E804 | 5 | Containers | <10s |
| U+E805 | 6 | Firefly Swarm | <1min |
| U+E806 | 7 | Orb Orbital | <10min |
| U+E810 | - | ESCALATE | +1 tier |
| U+E811 | - | DEESCALATE | -1 tier |
| U+E812 | - | HOLD | maintain |
| U+E813 | - | RESET | tier 1 |

### 3.6 Mathematical Operators (Greek Unicode)

For readability, standard Greek letters are recognized as operators:

| Symbol | Name | Usage |
|--------|------|-------|
| λ | Lambda | Hawkes intensity function |
| Δ | Delta | Change/difference |
| Σ | Sigma | Summation/EMA |
| ∫ | Integral | Integration |
| ∂ | Partial | Partial derivative |
| ∇ | Nabla | Gradient |
| × | Cross | Cross product |
| · | Dot | Dot product |
| θ φ ψ | Theta/Phi/Psi | Angles (radians) |
| ω | Omega | Angular velocity |
| α β | Alpha/Beta | Hawkes parameters |
| ε | Epsilon | Error term |
| μ | Mu | Baseline intensity |

## 4. Value System

### 4.1 Value Types

```rust
pub enum Value {
    Nil,                           // Empty/false
    Bool(bool),                    // Boolean
    Num(f64),                      // 64-bit float (6 decimal precision)
    Sym(String),                   // Symbol (interned string)
    List(Vec<Value>),              // List of values
    Lambda { params, body },       // Function closure
    Fire(char, Option<f64>),       // Fire event trigger
    Opcode(char),                  // Unicode instruction
}
```

### 4.2 Precision

All numeric values use 6-decimal precision:

```rust
// 0.000001 radians ≈ 0.11mm at 1m distance
fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}
```

## 5. Fire Events

Fire events are the primary mechanism for state recognition and escalation. When the interpreter evaluates a state trigger opcode, it emits a `FireEvent`:

```rust
pub struct FireEvent {
    pub trigger: char,        // Unicode trigger character
    pub value: Option<f64>,   // Associated value (if any)
    pub timestamp_ns: u64,    // Nanosecond timestamp (HFT mode)
    pub tier: u8,             // Current escalation tier (1-7)
}
```

### 5.1 Fire Event Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  Expression Evaluation                                          │
│  ────────────────────                                           │
│  (Δ x 0.001)  →  DeltaState.set_x(prev + 0.001)               │
│                                                                 │
│  if rate_magnitude() > threshold:                               │
│      emit FireEvent(STATE_ANOMALY, rate_magnitude())           │
│      → collected in interpreter.fires                          │
│      → returned via take_fires()                               │
│      → processed by EscalationPolicy                           │
│      → tier change if critical                                 │
└─────────────────────────────────────────────────────────────────┘
```

## 6. OODA Loop Integration

### 6.1 Phase Mapping

| OODA Phase | HD4 State | Activity |
|------------|-----------|----------|
| Observe | Discover | Gather sensor data, angle updates |
| Orient | Detect | Analyze patterns, vector search |
| Decide | Disrupt | Choose response, rule evaluation |
| Act | Dominate | Execute escalation, fire events |

### 6.2 OodaLoop API

```rust
pub struct OodaLoop {
    interpreter: Interpreter,
    phase: OodaPhase,
    sequence: u64,
    handlers: [Vec<Value>; 4],  // Per-phase expressions
}

impl OodaLoop {
    fn tick(&mut self) -> LispResult<OodaTick>;
    fn cycle(&mut self) -> LispResult<Vec<OodaTick>>;  // 4 ticks
    fn on_phase(&mut self, phase: OodaPhase, expr: Value);
}
```

## 7. Escalation Policy

### 7.1 Automatic Escalation

The `EscalationPolicy` automatically escalates on:

- `STATE_ANOMALY` - Abnormal motion detected
- `STATE_RATE_EXCEED` - Rate threshold exceeded
- `HAWKES_SPIKE` - Self-exciting intensity spike
- `HAWKES_HIGH` - High intensity sustained
- `H2_CRISIS` - Semantic convergence crisis
- `ESCALATE` opcode - Explicit escalation request

### 7.2 Critical Event Minimum Tier

Critical events force minimum tier (default: Tier 3 Kernel Crate):

- `STATE_ANOMALY`
- `HAWKES_SPIKE`
- `HD4_DISRUPT`
- `HD4_DOMINATE`
- `H2_CRISIS`
- `KC_EXECUTION`
- `KC_EXFIL`

### 7.3 De-escalation Cooldown

De-escalation has a 60-second cooldown to prevent tier oscillation.

## 8. Example Expressions

### 8.1 Basic Arithmetic

```lisp
; Addition using opcode
(U+E500 1 2 3)  ; → 6.0

; Nested operations
(U+E502 (U+E500 1 2) (U+E501 10 3))  ; → (1+2) * (10-3) = 21.0
```

### 8.2 Delta Angle Tracking

```lisp
; Set X angle
(U+E600 1.234567)  ; X = 1.234567 radians

; Read delta
(U+E603)  ; → Δx since last update

; Check rate magnitude
(U+E613)  ; → sqrt(Δx² + Δy² + Δz²)
```

### 8.3 State Triggers

```lisp
; Fire anomaly event with score
(U+E623 0.95)  ; → FireEvent { trigger: STATE_ANOMALY, value: Some(0.95) }

; Escalate to tier 5
(U+E804)  ; → tier = 5, FireEvent { trigger: TIER_5_CONTAINER }
```

### 8.4 Lambda Definitions

```lisp
(define distance
  (λ (x y)
    (U+E507  ; sqrt
      (U+E500  ; +
        (U+E508 x 2)   ; x²
        (U+E508 y 2))))) ; y²

(distance 3 4)  ; → 5.0
```

## 9. Wire Format

Unicode sequences are transmitted directly as UTF-8. No compilation step is required - the Unicode IS the bytecode.

### 9.1 Compact Encoding

```
Expression: (+ 1 2 3)
Wire:       (U+E500 1 2 3)
UTF-8:      28 bytes
```

### 9.2 S-Expression Compatibility

Standard Lisp syntax is supported alongside opcodes:

```lisp
; Both equivalent:
(+ 1 2)
(U+E500 1 2)

; Mixed mode:
(define f (λ (x) (U+E502 x x)))  ; f(x) = x²
```

## 10. Performance Targets

| Operation | Target | Measured |
|-----------|--------|----------|
| Parse simple expr | <100ns | TBD |
| Eval arithmetic (5 nums) | <50ns | TBD |
| Delta set (3 axis) | <30ns | TBD |
| Fire event emit | <20ns | TBD |
| Full OODA tick | <1μs | TBD |

## 11. Security Considerations

### 11.1 Sandboxing

- No filesystem access
- No network access
- No external process execution
- Memory bounded by stack/heap limits

### 11.2 Escalation Guards

- Tier changes logged with timestamps
- De-escalation cooldown prevents rapid oscillation
- Maximum tier (7) requires explicit authorization

### 11.3 Fire Event Audit

All fire events include:
- Nanosecond timestamp
- Triggering opcode
- Current tier
- Optional payload value

## 12. Implementation Status

| Module | Status | Tests |
|--------|--------|-------|
| `opcodes.rs` | ✅ Complete | 3 |
| `value.rs` | ✅ Complete | 5 |
| `interpreter.rs` | ✅ Complete | 9 |
| `escalation.rs` | ✅ Complete | 6 |
| `ooda.rs` | ✅ Complete | 4 |
| **Total** | **✅ Complete** | **29 passing** |

## 13. Future Work

1. **Layer 2 Vector Search** - Integration with Qdrant for semantic similarity
2. **Layer 4 Generative** - Phi-3 integration for natural language analysis
3. **Neo4j GLAF Traversal** - Graph relationship navigation
4. **ATLAS Daemon Wiring** - Integration with OODA tick loop
5. **Benchmark Suite** - Criterion benchmarks for performance validation

## 14. References

- RFC-9021: 4-Layer Cognitive Inference Architecture
- RFC-9101: Smart Crate System
- RFC-9107: Unified Agent Infrastructure
- RFC-9108: Thalmic Filter Model Registry
- [CTAS-7.3.1 System Design Meta-Overview](../sx9-rfc/META/CTAS-7.3.1-SYSTEM-DESIGN-META-OVERVIEW.md)

---

**Appendix A: Full Opcode Table**

See `sx9-lisp/src/opcodes.rs` for the complete opcode definitions.

**Appendix B: Test Coverage**

```
test escalation::tests::test_critical_auto_escalation ... ok
test escalation::tests::test_policy_escalation ... ok
test escalation::tests::test_max_response_times ... ok
test escalation::tests::test_policy_explicit_tier ... ok
test escalation::tests::test_tier_ordering ... ok
test escalation::tests::test_tier_from_u8 ... ok
test interpreter::tests::test_arithmetic ... ok
test interpreter::tests::test_comparison ... ok
test interpreter::tests::test_delta_state ... ok
test interpreter::tests::test_fire_events ... ok
test interpreter::tests::test_if ... ok
test interpreter::tests::test_tier_escalation ... ok
test interpreter::tests::test_lambda ... ok
test interpreter::tests::test_trigonometry ... ok
test ooda::tests::test_hd4_to_ooda ... ok
test ooda::tests::test_ooda_loop_cycle ... ok
test ooda::tests::test_ooda_loop_tick ... ok
test ooda::tests::test_ooda_phase_cycle ... ok
test opcodes::tests::test_is_opcode ... ok
test opcodes::tests::test_opcode_category ... ok
test opcodes::tests::test_opcode_values ... ok
test tests::test_fire_event ... ok
test tests::test_fire_event_with_tier ... ok
test tests::test_tier_clamping ... ok
test value::tests::test_fire_event ... ok
test value::tests::test_list ... ok
test value::tests::test_num_precision ... ok
test value::tests::test_truthy ... ok
test value::tests::test_type_conversions ... ok
```
