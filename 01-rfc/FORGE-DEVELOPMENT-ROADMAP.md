# SX9 Forge Development Roadmap

**Date:** 2025-12-24
**Status:** Active Development
**Priority:** Forge System (sx9-development-center)

---

## Executive Summary

This roadmap synthesizes requirements from RFC-9000 through RFC-9130 to reconstitute the Forge system - an advanced deterministic prompt system designed for IDE bootloader integration with long-running agent harness support.

---

## Current Implementation Status

### Frontend (sx9-forge)
| Component | Status | Location |
|-----------|--------|----------|
| React Native + Tauri + Vite | ✅ Complete | `sx9-forge/` |
| PromptForgeScreen UI | ✅ Complete | `src/screens/PromptForgeScreen.tsx` |
| Zustand Store | ✅ Complete | `src/store/forgeStore.ts` |
| Redux Intelligence Store | ✅ Complete | `src/store/intelligence/` |
| UI Components (GlyphRail, Tabs, Editor) | ✅ Complete | `src/components/` |
| Linear/Slack Integration UI | ✅ Complete | PromptForgeScreen |
| YAML Prompt Generation (v4.0) | ✅ Complete | PromptForgeScreen:149-201 |

### Backend (Tauri Commands)
| Command | Status | Location |
|---------|--------|----------|
| `save_prompt` | ✅ Complete | `src-tauri/src/commands/forge.rs:47` |
| `create_linear_issue_forge` | ✅ Complete | `src-tauri/src/commands/forge.rs:101` |
| `notify_slack` | ✅ Complete | `src-tauri/src/commands/forge.rs:212` |
| `copy_to_clipboard` | ⚠️ Stub | `src-tauri/src/commands/forge.rs:266` |
| `check_leptose` | ⚠️ Stub | `src-tauri/src/commands/forge.rs:291` |
| `check_chroma` | ⚠️ Stub | `src-tauri/src/commands/forge.rs:299` |

### Harness Crate (sx9-harness)
| Module | Status | Notes |
|--------|--------|-------|
| `types.rs` | ✅ Complete | HarnessMode, Persona, Mission, QaReport, Action |
| `executor.rs` | ⚠️ Stub | TODO: NATS publishing, result collection |
| `gates/mod.rs` | ✅ Structure | StaticGate, ArchGate, PatternGate |
| `gates/static_gate.rs` | 🔧 Needs impl | Structure/complexity analysis |
| `gates/arch_gate.rs` | 🔧 Needs impl | ECS compliance checks |
| `gates/pattern_gate.rs` | 🔧 Needs impl | Canonical pattern matching |
| `validators.rs` | ✅ Exists | |
| `reducer.rs` | ✅ Exists | Redux-style state |
| `selectors.rs` | ✅ Exists | |

---

## Gap Analysis (RFC Requirements vs. Implementation)

### Critical Gaps (RFC-9112, RFC-9120, RFC-9121)

| Component | RFC Source | Priority | Status |
|-----------|------------|----------|--------|
| **Thalmic Filter** | RFC-9112, RFC-9120 | P0 | ❌ Missing |
| **PromptScript v3 Parser** | RFC-9112 | P0 | ❌ Missing |
| **Lightning QA Integration** | RFC-9121 | P0 | ⚠️ Partial (gates exist) |
| **Pattern Resolver (Sledis)** | RFC-9120 | P1 | ❌ Missing |
| **NATS Message Fabric** | RFC-9112 | P1 | ❌ Missing |
| **Hermetic Tool Chain** | RFC-9112 | P1 | ❌ Missing |
| **Interview Auto-Population** | RFC-9120, RFC-9025 | P2 | ❌ Missing |
| **Birth Certificate Generation** | RFC-9120 | P2 | ❌ Missing |
| **IDE Bootloader Integration** | RFC-9130 | P2 | ❌ Missing |

---

## Development Phases

### Phase 1: Core Pipeline (P0)

**Objective:** Complete the four-stage Unified Forge Pipeline per RFC-9130

#### 1.1 Thalmic Filter Implementation
- **RFC:** RFC-9112 Section 5, RFC-9120 Stage 1
- **Purpose:** Semantic suppression gate with clarity threshold ≥ 0.7
- **Components:**
  - Intent classifier (Create/Modify/Query/Execute/Analyze)
  - Semantic clarity scorer
  - Domain extractor (Cyber, Finance, Intel, IoT, GIS)
- **Location:** `crates/sx9-harness/src/thalmic/`

```rust
// Target API
pub struct ThalmicFilter {
    clarity_threshold: f32, // Default: 0.7
}

impl ThalmicFilter {
    pub fn process(&self, input: &str) -> Result<ThalmicOutput, ThalmicError> {
        // Returns: intent, domain, clarity_score, filtered_output
    }
}
```

#### 1.2 PromptScript v3 Parser
- **RFC:** RFC-9112 Section 3
- **Purpose:** N-V-N-N grammar enforcement with Lisp compression
- **Grammar:** `(Ω <hash> (τ <primitive> <domain> <phase>) (σ <lisp-ops>))`
- **Location:** `crates/sx9-harness/src/promptscript/`

```rust
// Target API
pub struct PromptScriptParser;

impl PromptScriptParser {
    pub fn parse(&self, input: &str) -> Result<PromptAst, ParseError>;
    pub fn validate_nvnn(&self, ast: &PromptAst) -> ValidationResult;
    pub fn compile_unicode(&self, ast: &PromptAst) -> Vec<u8>;
}
```

#### 1.3 Lightning QA Integration
- **RFC:** RFC-9121
- **Purpose:** 4-dimension grading with anti-pattern detection
- **Dimensions:**
  - Structure (25%): File organization, module boundaries
  - Complexity (25%): Cyclomatic complexity, nesting depth
  - Pattern (25%): Canonical pattern usage, TETH anti-pattern flags
  - Architecture (25%): ECS compliance, dependency direction

**TETH Anti-Patterns (auto-fail):**
- `use bevy::` → -50 points (Legion only)
- Local `Rune` type → -30 points (use canonical)
- `unwrap()` chains → -10 points

**Grade Scale:**
| Grade | Score | Meaning |
|-------|-------|---------|
| A | 85-100 | Production ready |
| B | 70-84 | Minor issues |
| C | 55-69 | Refactor needed |
| D | 40-54 | Significant issues |
| F | 0-39 | Reject |

---

### Phase 2: Integration Layer (P1)

#### 2.1 NATS Message Fabric
- **RFC:** RFC-9112 Section 6
- **Purpose:** Distributed task routing for hermetic execution
- **Topics:**
  - `sx9.forge.task.{id}` - Task submission
  - `sx9.forge.result.{id}` - Execution results
  - `sx9.qa.grade.{crate}` - QA reports
- **Location:** `crates/sx9-harness/src/nats/`

#### 2.2 Pattern Resolver (Sledis/Redis)
- **RFC:** RFC-9120 Stage 2
- **Purpose:** Prior art lookup, semantic pattern matching
- **Data:**
  - Pattern embeddings (ChromaDB)
  - Pattern metadata (Redis/Sledis)
  - Usage statistics
- **Location:** `crates/sx9-harness/src/patterns/`

#### 2.3 Hermetic Tool Chain
- **RFC:** RFC-9112 Section 7
- **Purpose:** Sandboxed tool execution with NATS-only I/O
- **Constraints:**
  - No shell access
  - No direct file I/O
  - No logging
  - Rust FFI wrappers for all tools

```rust
#[async_trait]
pub trait HermeticTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn trigger_rune(&self) -> char; // U+E800-U+E9FF
    async fn execute(&self, params: UnicodeParams, nats: &NatsClient) -> Result<UnicodeResponse>;
    fn binary(&self) -> &'static [u8];
}
```

---

### Phase 3: Advanced Features (P2)

#### 3.1 Interview Schema Auto-Population
- **RFC:** RFC-9120 Stage 3, RFC-9025
- **Purpose:** Dynamic question generation based on context
- **Components:**
  - Question bank (RFC-9025 schema)
  - Context analyzer
  - Smart defaults from codebase analysis

#### 3.2 Birth Certificate Generation
- **RFC:** RFC-9120 Stage 4
- **Purpose:** Crate metadata and RFC lineage tracking
- **Fields:**
  - Crate name, version
  - RFC dependencies
  - SCH hash (RFC-9001)
  - CUID allocation
  - Creation timestamp

#### 3.3 IDE Bootloader Integration
- **RFC:** RFC-9130
- **Target IDEs:**
  - Claude Code (primary)
  - VSCode Extension
  - Cursor
- **Features:**
  - Long-running agent sessions
  - Harness mode switching
  - Real-time QA feedback
  - Linear/Slack workflow automation

---

## Architecture Alignment

### Key Constraints (RFC-9000)

1. **ECS Runtime:** Legion only (Bevy FORBIDDEN)
2. **Graph Backend:** SlotGraph + GLAF (SurrealDB deprecated)
3. **Cache:** Sledis (Redis-compatible Sled)
4. **Message Fabric:** NATS
5. **Vector DB:** ChromaDB
6. **SQL:** PostgreSQL (Supabase/Neon)

### Port Allocation

| Service | Port | Purpose |
|---------|------|---------|
| Prompt Forge | 3001 | UI Server |
| GLAF | 18050 | Graph Logic |
| Port Manager | 18103 | Service Registry |
| ATLAS Daemon | 18106 | Monitoring |
| Gateway | 18120 | WebSocket |
| DB API | 18889 | Unified Access |

### PTCC 32 Primitives (RFC-9100)

The system uses exactly 32 universal primitives:
- CRUD: CREATE, READ, UPDATE, DELETE
- Communication: SEND, RECEIVE, ROUTE, FILTER
- Transform: TRANSFORM, VALIDATE
- Control: BRANCH, LOOP, RETURN, CALL
- Connection: CONNECT, DISCONNECT
- Security: AUTHENTICATE, AUTHORIZE, ENCRYPT, DECRYPT
- Memory: ALLOCATE, DEALLOCATE, LOCK, UNLOCK
- State: SAVE, RESTORE, CHECKPOINT, ROLLBACK
- Coordination: COORDINATE, SYNCHRONIZE, SIGNAL, WAIT

---

## Immediate Next Steps

### Sprint 1: Foundation
1. Complete `executor.rs` with basic task execution
2. Implement `StaticGate` with complexity scoring
3. Wire Tauri commands to harness crate

### Sprint 2: Thalmic Filter
1. Create `thalmic/` module structure
2. Implement intent classification
3. Add clarity scoring
4. Integrate with PromptForgeScreen

### Sprint 3: QA Pipeline
1. Implement `ArchGate` (ECS compliance)
2. Implement `PatternGate` (TETH detection)
3. Create SARIF output format
4. Wire QA results to UI

### Sprint 4: NATS Integration
1. Add nats.rs dependency
2. Create message types
3. Implement publish/subscribe
4. Connect executor to NATS

---

## File Structure Target

```
crates/sx9-harness/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types.rs          ✅ Complete
│   ├── executor.rs       🔧 In Progress
│   ├── actions.rs        ✅ Exists
│   ├── middleware.rs     ✅ Exists
│   ├── validators.rs     ✅ Exists
│   ├── reducer.rs        ✅ Exists
│   ├── selectors.rs      ✅ Exists
│   ├── gates/
│   │   ├── mod.rs        ✅ Complete
│   │   ├── static_gate.rs    🔧 Needs impl
│   │   ├── arch_gate.rs      🔧 Needs impl
│   │   └── pattern_gate.rs   🔧 Needs impl
│   ├── thalmic/          ❌ Missing
│   │   ├── mod.rs
│   │   ├── filter.rs
│   │   ├── intent.rs
│   │   └── clarity.rs
│   ├── promptscript/     ❌ Missing
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   └── compiler.rs
│   ├── nats/             ❌ Missing
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── messages.rs
│   └── patterns/         ❌ Missing
│       ├── mod.rs
│       ├── resolver.rs
│       └── embeddings.rs
```

---

**Document Status:** ACTIVE
**Last Updated:** 2025-12-24
**Owner:** SX9 Development Team
