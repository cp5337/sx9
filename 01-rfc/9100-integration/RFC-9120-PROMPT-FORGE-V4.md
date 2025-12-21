# RFC-9120: Prompt Forge v4 — Plain Language Crate Manufacturing

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Depends On:** RFC-9001 (Trivariate), RFC-9005 (Schema), RFC-9025 (Interview), RFC-9101 (SmartCrate), RFC-9112 (PromptScript), RFC-9116 (Dev Forge)

---

## Abstract

Prompt Forge v4 eliminates manual pattern selection, constraint specification, and boilerplate generation. Users speak plain language intent; the system derives patterns, populates interviews, generates birth certificates, and produces deterministic canonical prompts that factory agents execute without supervision.

**Core Insight:** The Prompt is the Point. If the canonical prompt is perfect, the factory runs autonomously.

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PROMPT FORGE v4 PIPELINE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │  PLAIN       │    │   THALMIC    │    │   SLEDIS     │                  │
│  │  LANGUAGE    │───▶│   FILTER     │───▶│   PATTERN    │                  │
│  │  INPUT       │    │   (Clarity)  │    │   RESOLVER   │                  │
│  └──────────────┘    └──────────────┘    └──────────────┘                  │
│         │                   │                   │                           │
│         │            [Clarity Score]     [Pattern + Constraints]            │
│         │                   │                   │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                  RFC-9025 INTERVIEW POPULATOR               │           │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │           │
│  │  │Identity │ │ Voice   │ │Capabil. │ │Tactical │           │           │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘           │           │
│  └─────────────────────────────────────────────────────────────┘           │
│         │                                                                   │
│         ▼                                                                   │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │   BIRTH      │    │  CANONICAL   │    │    NATS      │                  │
│  │ CERTIFICATE  │───▶│   PROMPT     │───▶│   HEALTH     │                  │
│  │  GENERATOR   │    │  ASSEMBLER   │    │  CONTRACT    │                  │
│  └──────────────┘    └──────────────┘    └──────────────┘                  │
│         │                   │                   │                           │
│         │            [Gold Master Prompt]       │                           │
│         │                   │                   │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                    FACTORY AGENT EXECUTOR                    │           │
│  │  Build → Self-Interview → QA Filter → Git Push → Registry   │           │
│  └─────────────────────────────────────────────────────────────┘           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Plain Language Input

### 2.1 User Intent Examples

| User Says | System Derives |
|-----------|----------------|
| "Route NATS messages through trust verification in real-time" | Reactor pattern, async-nats, zero-trust factors |
| "Parse config files and validate against schema" | Strategy pattern, serde, toml, validator |
| "Cache graph queries with LRU eviction" | Decorator pattern, lru, petgraph |
| "Expose REST endpoints for tool registry" | Facade pattern, axum, tower |
| "Transform threat intel formats to unified schema" | Adapter pattern, serde, RFC-9005 |

### 2.2 Input Capture

```typescript
interface ForgeInput {
  // Primary: spoken/typed intent
  intent: string;
  
  // Optional: explicit overrides
  overrides?: {
    pattern?: string;        // Force specific pattern
    foundations?: string[];  // Force specific crates
    constraints?: Record<string, any>;
  };
  
  // Context: auto-populated from environment
  context: {
    project?: string;        // From Linear or cwd
    rfc_refs?: string[];     // Referenced RFCs
    existing_crates?: string[]; // Dependency candidates
  };
}
```

---

## 3. Thalmic Filter — Clarity Scoring

The Thalmic Filter scores input for deterministic interpretability. Not security redaction—clarity measurement.

### 3.1 Scoring Dimensions

| Dimension | Weight | Measures |
|-----------|--------|----------|
| **Specificity** | 0.3 | Concrete nouns vs vague references |
| **Actionability** | 0.3 | Clear verbs with defined outcomes |
| **Boundedness** | 0.2 | Explicit constraints and limits |
| **Unambiguity** | 0.2 | Single interpretation possible |

### 3.2 Score Thresholds

```
GREEN  (0.8 - 1.0): Deterministic. Proceed to pattern resolution.
YELLOW (0.5 - 0.8): Caution. Show confirmation with suggested clarifications.
RED    (0.0 - 0.5): Ambiguous. Request clarification before proceeding.
```

### 3.3 Clarity Indicators

```toml
[thalmic.indicators]

# Positive signals (increase score)
specific_nouns = ["NATS", "gRPC", "PostgreSQL", "trivariate"]
action_verbs = ["route", "validate", "transform", "cache", "parse"]
constraint_words = ["max", "limit", "timeout", "threshold", "only"]
quality_modifiers = ["real-time", "async", "fault-tolerant", "idempotent"]

# Negative signals (decrease score)
vague_nouns = ["thing", "stuff", "data", "it", "that"]
weak_verbs = ["handle", "process", "do", "make", "work"]
ambiguous_phrases = ["kind of", "maybe", "something like", "etc"]
missing_bounds = true  # No limits specified
```

### 3.4 Feature Extraction Output

```rust
pub struct ThalMicResult {
    pub score: f32,                    // 0.0 - 1.0
    pub status: ThalMicStatus,         // Green | Yellow | Red
    pub features: Vec<SemanticFeature>, // Extracted for pattern resolution
    pub suggestions: Vec<String>,       // Clarification prompts if Yellow/Red
}

pub struct SemanticFeature {
    pub category: FeatureCategory,     // Action | Subject | Quality | Constraint
    pub value: String,                 // Extracted term
    pub confidence: f32,               // Extraction confidence
}

pub enum FeatureCategory {
    Action,      // Verbs: route, parse, validate, cache
    Subject,     // Nouns: messages, config, queries, requests  
    Quality,     // Modifiers: real-time, async, fault-tolerant
    Constraint,  // Bounds: max, timeout, threshold
    Domain,      // Context: security, graph, messaging
}
```

---

## 4. Sledis Pattern Resolution

Pattern resolution is **lookup, not computation**. The Thalmic filter extracts features; sledis maps features to patterns.

### 4.1 Sledis Key Schema

```
# Pattern definitions
pattern:registry:{name}           → Full pattern definition (TOML)
pattern:constraints:{name}        → Birth certificate constraints (TOML)
pattern:health:{name}             → NATS health check spec (TOML)
pattern:foundations:{name}        → Required/recommended crates (JSON array)

# Feature → Pattern resolution
pattern:resolve:{sorted_features} → Pattern name (string)

# Anti-patterns (TETH-derived)
antipattern:registry:{name}       → Anti-pattern definition
antipattern:detect:{indicators}   → Detection rules

# Cache metadata
pattern:version                   → Current registry version (semver)
pattern:updated                   → Last update timestamp
```

### 4.2 Feature Resolution Algorithm

```rust
pub fn resolve_pattern(features: &[SemanticFeature]) -> Option<String> {
    // 1. Extract high-confidence features
    let key_features: Vec<&str> = features
        .iter()
        .filter(|f| f.confidence > 0.7)
        .map(|f| f.value.as_str())
        .collect();
    
    // 2. Sort for deterministic key
    let mut sorted = key_features.clone();
    sorted.sort();
    
    // 3. Build lookup key
    let key = format!("pattern:resolve:{}", sorted.join(":"));
    
    // 4. Sledis lookup
    sledis.get(&key)
}
```

### 4.3 Pattern Registry Structure

```toml
# /config/pattern-registry.toml

[patterns.reactor]
description = "Event-driven message processing with non-blocking I/O"
category = "behavioral"

[patterns.reactor.triggers]
# Feature combinations that resolve to this pattern
features = [
    ["route", "messages", "real-time"],
    ["event", "stream", "async"],
    ["subscribe", "publish", "nats"],
    ["process", "events", "non-blocking"],
]

[patterns.reactor.foundations]
required = ["async-nats", "tokio"]
recommended = ["tracing", "thiserror"]

[patterns.reactor.birth.required]
tick_subscription = { subject = "atlas.tick", description = "Dead man's switch" }
presence_check = { fn = "is_system_healthy", threshold_ms = 100 }
grammar_parser = { type = "RouterParser", validates = "trivariate" }
bounded_channels = { capacity = "configurable", default = 1024 }
graceful_shutdown = { signal = "SIGTERM", timeout_ms = 5000 }

[patterns.reactor.birth.forbidden]
blocking_io_in_loop = { reason = "Violates non-blocking reactor contract" }
unbounded_channels = { reason = "Memory exhaustion risk" }
sync_mutex_in_async = { reason = "Deadlock risk in async context" }
panic_in_handler = { reason = "Must propagate errors, not panic" }

[patterns.reactor.birth.limits]
max_lines_per_file = 300
max_dependencies = 15
max_public_functions = 20
cyclomatic_complexity_max = 10

[patterns.reactor.health]
structure_subject = "sx9.health.{crate}.structure"
contract_subject = "sx9.health.{crate}.contract"
check_interval_ms = 30000

[patterns.reactor.health.structure_checks]
file_line_counts = true
dependency_count = true
required_functions = ["is_system_healthy", "handle_message", "shutdown"]
forbidden_imports = ["std::thread::sleep", "std::sync::Mutex"]

[patterns.reactor.health.contract_checks]
entry_signature = "async fn run(&self) -> Result<(), Error>"
factor_presence = ["presence", "grammar"]
nvnn_flow = true


[patterns.facade]
description = "Unified interface to subsystem complexity"
category = "structural"
# ... similar structure

[patterns.strategy]
description = "Interchangeable algorithm family"
category = "behavioral"
# ... similar structure

[patterns.adapter]
description = "Interface translation between incompatible types"
category = "structural"
# ... similar structure

[patterns.builder]
description = "Step-by-step complex object construction"
category = "creational"
# ... similar structure

[patterns.observer]
description = "Publish-subscribe notification system"
category = "behavioral"
# ... similar structure
```

### 4.4 Anti-Pattern Registry (TETH-Derived)

```toml
# /config/antipattern-registry.toml

[antipatterns.god_object]
description = "Single struct with too many responsibilities"
severity = "high"

[antipatterns.god_object.detection]
struct_field_count_max = 15
impl_method_count_max = 25
file_line_count_max = 500

[antipatterns.god_object.remediation]
action = "split"
guidance = "Extract cohesive field groups into separate structs"


[antipatterns.spaghetti_deps]
description = "Circular or excessive dependencies"
severity = "high"

[antipatterns.spaghetti_deps.detection]
circular_deps = true
transitive_depth_max = 5
direct_deps_max = 20

[antipatterns.spaghetti_deps.remediation]
action = "refactor"
guidance = "Introduce abstraction layers, apply dependency inversion"


[antipatterns.async_mutex_abuse]
description = "std::sync::Mutex used in async context"
severity = "critical"

[antipatterns.async_mutex_abuse.detection]
patterns = [
    "std::sync::Mutex",
    "std::sync::RwLock",
]
context = "async fn"

[antipatterns.async_mutex_abuse.remediation]
action = "replace"
guidance = "Use tokio::sync::Mutex or tokio::sync::RwLock"


[antipatterns.unbounded_growth]
description = "Collections without capacity limits"
severity = "high"

[antipatterns.unbounded_growth.detection]
patterns = [
    "Vec::new()",
    "HashMap::new()",
    "channel()",  # mpsc::channel without bound
]
without_capacity = true

[antipatterns.unbounded_growth.remediation]
action = "bound"
guidance = "Use with_capacity() or bounded channels"
```

### 4.5 Sledis Loader

```rust
// Load pattern registry into sledis on startup
pub async fn load_pattern_registry(sledis: &Sledis, config_path: &Path) -> Result<()> {
    let registry: PatternRegistry = toml::from_str(
        &fs::read_to_string(config_path)?
    )?;
    
    for (name, pattern) in registry.patterns {
        // Store full definition
        sledis.set(
            format!("pattern:registry:{}", name),
            toml::to_string(&pattern)?
        ).await?;
        
        // Store constraints separately for fast lookup
        sledis.set(
            format!("pattern:constraints:{}", name),
            toml::to_string(&pattern.birth)?
        ).await?;
        
        // Store health spec
        sledis.set(
            format!("pattern:health:{}", name),
            toml::to_string(&pattern.health)?
        ).await?;
        
        // Store foundations
        sledis.set(
            format!("pattern:foundations:{}", name),
            serde_json::to_string(&pattern.foundations)?
        ).await?;
        
        // Build feature resolution keys
        for feature_set in &pattern.triggers.features {
            let mut sorted = feature_set.clone();
            sorted.sort();
            let key = format!("pattern:resolve:{}", sorted.join(":"));
            sledis.set(key, name.clone()).await?;
        }
    }
    
    // Broadcast update
    nats.publish("sx9.config.pattern.updated", registry.version).await?;
    
    Ok(())
}
```

---

## 5. RFC-9025 Interview Auto-Population

Once pattern is resolved, the interview schema populates automatically from extracted features and pattern constraints.

### 5.1 Interview Schema (RFC-9025 Subset for Crates)

```json
{
  "$schema": "https://sx9.dev/schemas/rfc-9025-crate-interview.json",
  "version": "1.0.0",
  
  "metadata": {
    "interview_id": "uuid-v4",
    "created_at": "iso-8601",
    "forge_version": "4.0.0",
    "thalmic_score": 0.85,
    "pattern_resolved": "reactor"
  },
  
  "identity": {
    "name": "sx9-nats-router",
    "type": "crate",
    "category": "infrastructure",
    "description": "Routes NATS messages through zero-trust verification"
  },
  
  "voice": {
    "narrative": "I am the message router. I receive events from atlas.tick, verify operator presence and message grammar, then dispatch to appropriate handlers.",
    "purpose": "Ensure all NATS traffic passes through two-factor zero-trust gates",
    "ownership": "sx9-core",
    "vector": "NATS → Verify → Route → Respond"
  },
  
  "capabilities": {
    "primary": ["message_routing", "trust_verification", "pattern_matching"],
    "operational": ["graceful_shutdown", "health_reporting", "metric_emission"],
    "integration": ["nats", "sledis", "atlas"]
  },
  
  "constraints": {
    "required_structures": {
      "tick_subscription": { "subject": "atlas.tick" },
      "presence_check": { "fn": "is_system_healthy", "threshold_ms": 100 },
      "grammar_parser": { "type": "RouterParser" }
    },
    "forbidden_structures": {
      "blocking_io_in_loop": true,
      "unbounded_channels": true,
      "sync_mutex_in_async": true
    },
    "limits": {
      "max_lines_per_file": 300,
      "max_dependencies": 15,
      "cyclomatic_complexity_max": 10
    }
  },
  
  "foundations": {
    "required": ["async-nats", "tokio"],
    "recommended": ["tracing", "thiserror", "murmur3"]
  },
  
  "health_contract": {
    "structure_subject": "sx9.health.sx9-nats-router.structure",
    "contract_subject": "sx9.health.sx9-nats-router.contract",
    "check_interval_ms": 30000
  },
  
  "lineage": {
    "parent_rfc": ["RFC-9112", "RFC-9101"],
    "pattern": "reactor",
    "forge_session": "uuid-of-forge-session"
  }
}
```

### 5.2 Auto-Population Logic

```rust
pub fn populate_interview(
    input: &ForgeInput,
    thalmic: &ThalMicResult,
    pattern: &PatternDefinition,
) -> CrateInterview {
    CrateInterview {
        metadata: InterviewMetadata {
            interview_id: Uuid::new_v4(),
            created_at: Utc::now(),
            forge_version: env!("CARGO_PKG_VERSION").to_string(),
            thalmic_score: thalmic.score,
            pattern_resolved: pattern.name.clone(),
        },
        
        identity: Identity {
            name: derive_crate_name(&input.intent, &thalmic.features),
            type_: "crate".to_string(),
            category: pattern.category.clone(),
            description: input.intent.clone(),
        },
        
        voice: Voice {
            narrative: generate_first_person_narrative(&input.intent, pattern),
            purpose: extract_purpose(&thalmic.features),
            ownership: input.context.project.clone().unwrap_or("sx9-core".to_string()),
            vector: derive_nvnn_vector(&thalmic.features),
        },
        
        capabilities: Capabilities {
            primary: extract_capabilities(&thalmic.features, CapabilityType::Primary),
            operational: pattern.birth.required.keys().cloned().collect(),
            integration: extract_integrations(&thalmic.features),
        },
        
        constraints: pattern.birth.clone(),
        
        foundations: pattern.foundations.clone(),
        
        health_contract: HealthContract {
            structure_subject: format!("sx9.health.{}.structure", derive_crate_name(&input.intent, &thalmic.features)),
            contract_subject: format!("sx9.health.{}.contract", derive_crate_name(&input.intent, &thalmic.features)),
            check_interval_ms: pattern.health.check_interval_ms,
        },
        
        lineage: Lineage {
            parent_rfc: pattern.parent_rfcs.clone(),
            pattern: pattern.name.clone(),
            forge_session: Uuid::new_v4(),
        },
    }
}
```

---

## 6. Birth Certificate Generation

Two outputs: `crate_interview.json` (full interview) and `smartcrate.toml` (runtime manifest).

### 6.1 crate_interview.json

The complete RFC-9025 interview, stored at crate root. This is the **source of truth** for what the crate was born to do.

```
my-crate/
├── Cargo.toml
├── crate_interview.json    ← Birth certificate (full)
├── smartcrate.toml         ← Runtime manifest (operational)
└── src/
    └── lib.rs
```

### 6.2 smartcrate.toml (RFC-9101 Appendix A)

```toml
# smartcrate.toml - Runtime manifest for sx9-nats-router

[crate]
name = "sx9-nats-router"
version = "0.1.0"
classification = "router"
pattern = "reactor"

[semantic_lock]
algorithm = "murmur3-128"
interview_hash = "a1b2c3d4e5f6..."  # Hash of crate_interview.json
code_hash = "f6e5d4c3b2a1..."       # Hash of src/**/*.rs

[neural_mux]
enabled = true
primary_channel = "sx9.router.commands"
response_channel = "sx9.router.responses"

[zero_trust]
factor_1 = { type = "presence", source = "atlas.tick", threshold_ms = 100 }
factor_2 = { type = "grammar", parser = "RouterParser", validates = "trivariate" }

[health]
structure_subject = "sx9.health.sx9-nats-router.structure"
contract_subject = "sx9.health.sx9-nats-router.contract"
interval_ms = 30000

[constraints]
max_lines_per_file = 300
max_dependencies = 15
forbidden_imports = ["std::thread::sleep", "std::sync::Mutex"]

[lineage]
interview_id = "uuid"
forge_session = "uuid"
created_at = "2025-12-20T00:00:00Z"
parent_rfcs = ["RFC-9112", "RFC-9101"]
```

---

## 7. Canonical Prompt Assembly

The Gold Master Prompt is deterministic: same input always produces same prompt.

### 7.1 Prompt Template Structure

```yaml
# Canonical Prompt Template

header:
  forge_version: "4.0.0"
  interview_id: "{interview.metadata.interview_id}"
  pattern: "{interview.lineage.pattern}"
  timestamp: "{now}"

mission:
  objective: |
    Build a Rust crate named `{interview.identity.name}` that:
    {interview.identity.description}
  
  voice: |
    The crate speaks in first person:
    "{interview.voice.narrative}"
  
  vector: "{interview.voice.vector}"

constraints:
  required_structures:
    {for each in interview.constraints.required_structures}
    - name: {each.key}
      spec: {each.value | yaml}
    {end for}
  
  forbidden_structures:
    {for each in interview.constraints.forbidden_structures}
    - {each.key}: {each.reason}
    {end for}
  
  limits:
    {interview.constraints.limits | yaml}

foundations:
  required:
    {for each in interview.foundations.required}
    - {each}
    {end for}
  recommended:
    {for each in interview.foundations.recommended}
    - {each}
    {end for}

health_contract:
  on_build_complete:
    - Generate `crate_interview.json` by inspecting built code
    - Calculate code_hash via murmur3-128 of src/**/*.rs
    - Validate all required_structures exist
    - Verify no forbidden_structures present
    - Check all limits are satisfied
  
  registration:
    - Publish to {interview.health_contract.structure_subject}
    - Publish to {interview.health_contract.contract_subject}

execution:
  steps:
    1. Create Cargo.toml with foundations
    2. Implement core logic per pattern "{interview.lineage.pattern}"
    3. Implement all required_structures
    4. Add health check endpoints
    5. Generate crate_interview.json
    6. Generate smartcrate.toml
    7. Run `cargo check` and `cargo clippy`
    8. Self-validate against birth certificate
  
  on_failure:
    max_retries: 3
    on_max_retries: "HALT and report violation"

output:
  files:
    - Cargo.toml
    - src/lib.rs (max {interview.constraints.limits.max_lines_per_file} lines)
    - src/*.rs (split as needed to respect line limits)
    - crate_interview.json
    - smartcrate.toml
    - tests/integration_test.rs
```

### 7.2 Assembled Prompt Example

```markdown
# FORGE SESSION: 7f8a9b0c-1d2e-3f4a-5b6c-7d8e9f0a1b2c

## Mission

Build a Rust crate named `sx9-nats-router` that:
Routes NATS messages through zero-trust verification in real-time

### Voice

The crate speaks in first person:
"I am the message router. I receive events from atlas.tick, verify operator presence 
and message grammar, then dispatch to appropriate handlers."

### Data Flow Vector
NATS → Verify → Route → Respond

---

## Constraints

### Required Structures

| Structure | Specification |
|-----------|---------------|
| tick_subscription | subject: "atlas.tick" |
| presence_check | fn: "is_system_healthy", threshold_ms: 100 |
| grammar_parser | type: "RouterParser", validates: "trivariate" |
| bounded_channels | capacity: configurable, default: 1024 |
| graceful_shutdown | signal: SIGTERM, timeout_ms: 5000 |

### Forbidden Structures

| Structure | Reason |
|-----------|--------|
| blocking_io_in_loop | Violates non-blocking reactor contract |
| unbounded_channels | Memory exhaustion risk |
| sync_mutex_in_async | Deadlock risk in async context |
| panic_in_handler | Must propagate errors, not panic |

### Limits

- max_lines_per_file: 300
- max_dependencies: 15
- max_public_functions: 20
- cyclomatic_complexity_max: 10

---

## Foundations

### Required
- async-nats
- tokio

### Recommended
- tracing
- thiserror
- murmur3

---

## Health Contract

On build complete:
1. Generate `crate_interview.json` by inspecting built code
2. Calculate code_hash via murmur3-128 of src/**/*.rs
3. Validate all required_structures exist
4. Verify no forbidden_structures present
5. Check all limits are satisfied

Register at:
- sx9.health.sx9-nats-router.structure
- sx9.health.sx9-nats-router.contract

---

## Execution

1. Create Cargo.toml with required foundations
2. Implement core logic per Reactor pattern
3. Implement all required_structures
4. Add health check endpoints
5. Generate crate_interview.json
6. Generate smartcrate.toml
7. Run `cargo check` and `cargo clippy`
8. Self-validate against birth certificate

**On Failure:** Max 3 retries, then HALT and report violation.

---

## Output Files

- Cargo.toml
- src/lib.rs (max 300 lines)
- src/*.rs (split as needed)
- crate_interview.json
- smartcrate.toml
- tests/integration_test.rs
```

---

## 8. NATS Health Contract Registration

Every crate registers two health subjects on first run.

### 8.1 Subject Hierarchy

```
sx9.health.{crate}.structure    # AST/structure validation
sx9.health.{crate}.contract     # Runtime behavior validation
sx9.health.{crate}.heartbeat    # Liveness (if running service)
```

### 8.2 Structure Check Payload

```json
{
  "crate": "sx9-nats-router",
  "check_type": "structure",
  "timestamp": "2025-12-20T00:00:00Z",
  "results": {
    "file_line_counts": {
      "src/lib.rs": 287,
      "src/router.rs": 156,
      "src/parser.rs": 89
    },
    "max_exceeded": false,
    "dependency_count": 12,
    "required_functions": {
      "is_system_healthy": true,
      "handle_message": true,
      "shutdown": true
    },
    "forbidden_imports": {
      "std::thread::sleep": false,
      "std::sync::Mutex": false
    },
    "passed": true
  }
}
```

### 8.3 Contract Check Payload

```json
{
  "crate": "sx9-nats-router",
  "check_type": "contract",
  "timestamp": "2025-12-20T00:00:00Z",
  "results": {
    "entry_signature_valid": true,
    "factor_1_presence": {
      "type": "presence",
      "source": "atlas.tick",
      "last_tick_ms": 45,
      "threshold_ms": 100,
      "passed": true
    },
    "factor_2_grammar": {
      "type": "grammar",
      "parser": "RouterParser",
      "validates": "trivariate",
      "passed": true
    },
    "nvnn_flow_valid": true,
    "passed": true
  }
}
```

### 8.4 Health Check Daemon

```rust
// Subscribes to sx9.health.*.structure and sx9.health.*.contract
// Runs on every commit via git hook or CI trigger

pub async fn health_daemon(nats: &async_nats::Client) -> Result<()> {
    let mut structure_sub = nats.subscribe("sx9.health.*.structure").await?;
    let mut contract_sub = nats.subscribe("sx9.health.*.contract").await?;
    
    loop {
        tokio::select! {
            Some(msg) = structure_sub.next() => {
                let check: StructureCheck = serde_json::from_slice(&msg.payload)?;
                if !check.results.passed {
                    emit_drift_alert(&check).await?;
                    block_merge(&check.crate_name).await?;
                }
            }
            Some(msg) = contract_sub.next() => {
                let check: ContractCheck = serde_json::from_slice(&msg.payload)?;
                if !check.results.passed {
                    emit_contract_violation(&check).await?;
                    block_merge(&check.crate_name).await?;
                }
            }
        }
    }
}
```

---

## 9. Factory Agent Execution Loop

The factory agent executes the canonical prompt and self-validates.

### 9.1 Execution Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         FACTORY AGENT LOOP                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. RECEIVE ──────────────────────────────────────────────────────────────▶│
│     │  Canonical prompt from Forge                                         │
│     │                                                                       │
│  2. BUILD ────────────────────────────────────────────────────────────────▶│
│     │  Execute prompt: create files, implement logic                       │
│     │                                                                       │
│  3. SELF-INTERVIEW ───────────────────────────────────────────────────────▶│
│     │  Inspect built code                                                  │
│     │  Generate crate_interview.json                                       │
│     │  Calculate hashes                                                    │
│     │                                                                       │
│  4. VALIDATE ─────────────────────────────────────────────────────────────▶│
│     │  Compare self-interview against birth certificate                   │
│     │  Check required_structures present                                   │
│     │  Check forbidden_structures absent                                   │
│     │  Check limits satisfied                                              │
│     │                                                                       │
│  5. DECISION ─────────────────────────────────────────────────────────────▶│
│     │                                                                       │
│     ├─── PASS ──▶ 6. PUBLISH ──▶ 7. GIT PUSH ──▶ 8. REGISTRY ──▶ DONE     │
│     │                                                                       │
│     └─── FAIL ──▶ 6. REFACTOR (retry 1-3) ──▶ 5. DECISION                  │
│                         │                                                   │
│                         └──▶ MAX RETRIES ──▶ HALT + REPORT                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2 Self-Interview Generation

```rust
pub fn generate_self_interview(crate_path: &Path) -> Result<CrateInterview> {
    // Parse all Rust files
    let files = glob::glob(&format!("{}/**/*.rs", crate_path.display()))?;
    let mut syntax_trees = Vec::new();
    
    for file in files {
        let content = fs::read_to_string(&file?)?;
        let ast = syn::parse_file(&content)?;
        syntax_trees.push((file, ast));
    }
    
    // Extract capabilities from impl blocks and public functions
    let capabilities = extract_capabilities(&syntax_trees);
    
    // Verify required structures
    let required_structures = verify_required_structures(&syntax_trees, &birth_certificate)?;
    
    // Check for forbidden structures
    let forbidden_violations = detect_forbidden_structures(&syntax_trees, &birth_certificate)?;
    
    // Calculate hashes
    let code_hash = calculate_murmur3_hash(&crate_path)?;
    
    // Build interview
    Ok(CrateInterview {
        // ... populate from extracted data
    })
}
```

### 9.3 Validation Logic

```rust
pub fn validate_against_birth_certificate(
    self_interview: &CrateInterview,
    birth_certificate: &CrateInterview,
) -> ValidationResult {
    let mut violations = Vec::new();
    
    // Check required structures
    for (name, spec) in &birth_certificate.constraints.required_structures {
        if !self_interview.constraints.required_structures.contains_key(name) {
            violations.push(Violation::MissingRequired(name.clone()));
        }
    }
    
    // Check forbidden structures
    for (name, _) in &self_interview.constraints.forbidden_structures {
        if birth_certificate.constraints.forbidden_structures.contains_key(name) {
            violations.push(Violation::ForbiddenPresent(name.clone()));
        }
    }
    
    // Check limits
    for (limit_name, limit_value) in &birth_certificate.constraints.limits {
        if let Some(actual) = self_interview.constraints.limits.get(limit_name) {
            if actual > limit_value {
                violations.push(Violation::LimitExceeded {
                    name: limit_name.clone(),
                    limit: *limit_value,
                    actual: *actual,
                });
            }
        }
    }
    
    if violations.is_empty() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(violations)
    }
}
```

---

## 10. Continuous EA — Structure + Contract Enforcement

Every commit triggers validation. This is **Enterprise Architecture at code level**.

### 10.1 Git Hook Integration

```bash
#!/bin/bash
# .git/hooks/pre-push

# Find all crates with birth certificates
for interview in $(find . -name "crate_interview.json"); do
    crate_dir=$(dirname "$interview")
    crate_name=$(jq -r '.identity.name' "$interview")
    
    echo "Validating $crate_name..."
    
    # Run structure check
    sx9-health-check structure "$crate_dir" | \
        nats pub "sx9.health.$crate_name.structure"
    
    # Run contract check
    sx9-health-check contract "$crate_dir" | \
        nats pub "sx9.health.$crate_name.contract"
    
    # Check results
    if [ $? -ne 0 ]; then
        echo "BLOCKED: $crate_name failed health check"
        exit 1
    fi
done

echo "All crates passed health checks"
exit 0
```

### 10.2 CI Pipeline Integration

```yaml
# .github/workflows/ea-enforcement.yml

name: EA Enforcement

on:
  pull_request:
    branches: [main, develop]

jobs:
  validate-birth-certificates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Find crates with birth certificates
        id: find-crates
        run: |
          echo "crates=$(find . -name 'crate_interview.json' -printf '%h\n' | jq -R -s -c 'split("\n")[:-1]')" >> $GITHUB_OUTPUT
      
      - name: Validate each crate
        run: |
          for crate in $(echo '${{ steps.find-crates.outputs.crates }}' | jq -r '.[]'); do
            echo "::group::Validating $crate"
            sx9-health-check structure "$crate"
            sx9-health-check contract "$crate"
            echo "::endgroup::"
          done
      
      - name: Block on violations
        if: failure()
        run: |
          echo "::error::Birth certificate violations detected. See logs above."
          exit 1
```

### 10.3 Drift Detection Example

```
DRIFT_DETECTED: sx9-nats-router

Violation: src/lib.rs = 487 lines (birth certificate: max 300)
Action: BLOCKED
Remediation: Split per original Reactor pattern constraints

Violation: std::sync::Mutex found in src/router.rs:45
Action: BLOCKED  
Remediation: Replace with tokio::sync::Mutex per birth certificate

Commit: abc123
Author: dev@sx9.dev
Branch: feature/add-caching

This PR cannot be merged until violations are resolved.
```

---

## 11. Forge UI Integration

The Forge UI (RFC-9116 Dev Forge) presents this pipeline visually.

### 11.1 UI Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PROMPT FORGE v4                                              [Clarity: 🟢] │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  What do you want to build?                                         │   │
│  │                                                                      │   │
│  │  > Route NATS messages through trust verification in real-time_     │   │
│  │                                                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  DETECTED                                                           │   │
│  │  ────────────────────────────────────────────────────────────────── │   │
│  │  Pattern: Reactor                                                   │   │
│  │  Foundations: async-nats, tokio                                     │   │
│  │  Constraints: 7 required, 4 forbidden, 4 limits                     │   │
│  │                                                                      │   │
│  │  [Adjust]                                            [Confirm ✓]    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  INTERVIEW PREVIEW                                                  │   │
│  │  ────────────────────────────────────────────────────────────────── │   │
│  │  Name: sx9-nats-router                                              │   │
│  │  Voice: "I am the message router..."                                │   │
│  │  Vector: NATS → Verify → Route → Respond                            │   │
│  │                                                                      │   │
│  │  [Edit Interview]                            [Generate Prompt ▶]    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 11.2 TSX Component Structure

```typescript
// PromptForgeV4.tsx

export function PromptForgeV4() {
  const [input, setInput] = useState<ForgeInput>({ intent: '', context: {} });
  const [thalmic, setThalmic] = useState<ThalMicResult | null>(null);
  const [pattern, setPattern] = useState<PatternDefinition | null>(null);
  const [interview, setInterview] = useState<CrateInterview | null>(null);
  const [prompt, setPrompt] = useState<string | null>(null);

  // Real-time clarity scoring as user types
  useEffect(() => {
    const debounced = debounce(async () => {
      if (input.intent.length > 10) {
        const result = await thalMicScore(input.intent);
        setThalmic(result);
        
        if (result.status === 'green') {
          const resolved = await resolvePattern(result.features);
          setPattern(resolved);
        }
      }
    }, 300);
    debounced();
  }, [input.intent]);

  // Auto-populate interview when pattern confirmed
  const confirmPattern = async () => {
    if (pattern && thalmic) {
      const populated = await populateInterview(input, thalmic, pattern);
      setInterview(populated);
    }
  };

  // Generate canonical prompt
  const generatePrompt = async () => {
    if (interview) {
      const canonical = await assemblePrompt(interview);
      setPrompt(canonical);
    }
  };

  return (
    <div className="forge-container">
      <IntentInput value={input} onChange={setInput} />
      <ClarityIndicator result={thalmic} />
      {pattern && <PatternConfirmation pattern={pattern} onConfirm={confirmPattern} />}
      {interview && <InterviewPreview interview={interview} />}
      {interview && <GenerateButton onClick={generatePrompt} />}
      {prompt && <PromptOutput prompt={prompt} />}
    </div>
  );
}
```

---

## 12. Storage Architecture

Four-tier storage per RFC-9400.

### 12.1 Layer Responsibilities

| Layer | Technology | Purpose | Latency |
|-------|------------|---------|---------|
| L1 | Sledis | Hot cache, session state, pattern lookup | <1µs |
| L2 | Sled | Warm cache, tool definitions, configs | <10µs |
| L3 | SurrealDB | Graph queries, pattern discovery | <10ms |
| L4 | Supabase | Audit trail, RFC canonical, multi-user | <50ms |

### 12.2 Sledis Keys for Prompt Forge

```
# Session state
forge:session:{id}              → Current forge session state
forge:session:{id}:input        → User input
forge:session:{id}:thalmic      → Clarity result
forge:session:{id}:pattern      → Resolved pattern
forge:session:{id}:interview    → Populated interview

# Pattern cache (from Section 4)
pattern:registry:{name}         → Full pattern definition
pattern:resolve:{features}      → Resolved pattern name
pattern:constraints:{name}      → Birth certificate constraints
pattern:health:{name}           → Health check spec

# Prompt cache
prompt:cache:{hash}             → Generated prompt (24h TTL)

# Health state
health:crate:{name}:structure   → Last structure check result
health:crate:{name}:contract    → Last contract check result
```

---

## 13. Implementation Checklist

### Phase 1: Core Pipeline (Week 1)

- [ ] Thalmic Filter: feature extraction from plain language
- [ ] Sledis loader: pattern registry → sledis cache
- [ ] Pattern resolver: features → pattern lookup
- [ ] Interview populator: pattern + features → RFC-9025 interview

### Phase 2: Birth Certificates (Week 2)

- [ ] crate_interview.json generator
- [ ] smartcrate.toml generator
- [ ] Canonical prompt assembler
- [ ] Prompt template engine

### Phase 3: Health Contracts (Week 3)

- [ ] AST analyzer for structure checks
- [ ] Contract validator for runtime checks
- [ ] NATS health daemon
- [ ] Git hook integration

### Phase 4: Factory Agent (Week 4)

- [ ] Self-interview generator (code → interview)
- [ ] Validation logic (self vs birth certificate)
- [ ] Retry/halt logic
- [ ] Registry gatekeeper

### Phase 5: UI Integration (Week 5)

- [ ] Real-time clarity indicator
- [ ] Pattern confirmation flow
- [ ] Interview editor
- [ ] Prompt preview and export

---

## 14. References

- RFC-9001: Trivariate Hashing
- RFC-9005: Unified Schema
- RFC-9025: Interview Schema
- RFC-9101: SmartCrate Manifest
- RFC-9112: PromptScript v3
- RFC-9116: Dev Forge Architecture
- RFC-9400: Graph Database (SurrealDB)

---

## Appendix A: Two-Factor Zero Trust Reference

From reactor.rs implementation:

```rust
/// Factor 1: Presence (Ops Gate)
/// Dead man's switch via atlas.tick subscription
fn is_system_healthy(&self) -> bool {
    let last_tick = self.last_tick.load(Ordering::SeqCst);
    let now = Instant::now();
    now.duration_since(last_tick).as_millis() < 100
}

/// Factor 2: Grammar (Dev Gate)  
/// RouterParser validates trivariate hash
fn validate_message(&self, msg: &Message) -> Result<ValidatedMessage, GrammarError> {
    let parsed = RouterParser::parse(&msg.payload)?;
    if !parsed.trivariate_valid() {
        return Err(GrammarError::InvalidTrivariate);
    }
    Ok(parsed.into())
}

/// Both factors must pass before processing
async fn handle(&self, msg: Message) -> Result<()> {
    // Factor 1: Presence
    if !self.is_system_healthy() {
        return Err(Error::SystemLock("Presence check failed"));
    }
    
    // Factor 2: Grammar
    let validated = self.validate_message(&msg)?;
    
    // Only now do we process
    self.process(validated).await
}
```

---

## Appendix B: Common Language Vocabulary

Feature space for pattern derivation:

```toml
[vocabulary.actions]
# Verbs that indicate data movement/transformation
route = ["reactor", "facade"]
parse = ["strategy", "adapter"]
validate = ["strategy", "decorator"]
cache = ["decorator", "proxy"]
transform = ["adapter", "strategy"]
subscribe = ["observer", "reactor"]
publish = ["observer", "reactor"]
build = ["builder", "factory"]
create = ["factory", "builder"]

[vocabulary.subjects]
# Nouns that indicate domain
messages = ["reactor", "observer"]
config = ["strategy", "builder"]
requests = ["facade", "proxy"]
events = ["observer", "reactor"]
queries = ["decorator", "proxy"]

[vocabulary.qualities]
# Modifiers that add constraints
real_time = { patterns = ["reactor"], constraints = ["no_blocking"] }
async = { patterns = ["reactor"], foundations = ["tokio"] }
fault_tolerant = { patterns = ["reactor"], constraints = ["graceful_shutdown"] }
idempotent = { patterns = ["strategy"], constraints = ["no_side_effects"] }
cached = { patterns = ["decorator", "proxy"], foundations = ["lru"] }

[vocabulary.domains]
security = { patterns = ["reactor", "strategy"], constraints = ["zero_trust"] }
graph = { foundations = ["petgraph"], patterns = ["visitor"] }
messaging = { foundations = ["async-nats"], patterns = ["reactor", "observer"] }
api = { foundations = ["axum"], patterns = ["facade", "adapter"] }
```

---

*End of RFC-9120*
