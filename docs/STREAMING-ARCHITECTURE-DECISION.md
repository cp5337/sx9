# Streaming Architecture Decision
## Unified Streaming Infrastructure - Intel vs Non-Intel Separation

**Date:** December 2025  
**Status:** Architecture Decision Document  
**Goal:** Eliminate dual streaming infrastructure, separate intel from non-intel, extract bloated foundation-core modules

---

## Problem Statement

**Current Issues:**
1. **Dual Streaming Infrastructure** - Multiple streaming systems (TAPS, NATS, WebSocket, SSE)
2. **Foundation-Core Bloat** - Too many modules "popping out like rabbits" (EEI, streaming, multimedia, etc.)
3. **No Clear Separation** - Intel streaming mixed with operational streaming
4. **EEI Crate Missing** - EEI was its own crate but got merged into foundation-core

---

## 1. Streaming Infrastructure Decision

### 1.1 Single Unified Streaming Backbone: NATS JetStream

**Decision:** Use **NATS JetStream** as the single streaming backbone for ALL streaming (intel and non-intel).

**Rationale:**
- Already integrated in `sx9-atlas-bus` (RFC-9130)
- Supports time-of-value decay via message TTL
- Persistent streams for audit trails
- High performance (<1ms latency)
- Single infrastructure = easier maintenance

**Eliminate:**
- ❌ TAPS (Tokio Async Pub/Sub) - redundant with NATS
- ❌ Custom WebSocket servers - use NATS WebSocket
- ❌ SSE (Server-Sent Events) - use NATS JetStream
- ❌ Multiple pub/sub systems

---

## 2. Intel vs Non-Intel Separation

### 2.1 Subject Hierarchy

**Intel Streaming** (Time-of-Value Decay):
```
sx9.stream.intel.{type}.{tier}
├── sigint.{tier}    (48hr half-life)
├── humint.{tier}    (7day half-life)
├── geoint.{tier}    (30day half-life)
├── osint.{tier}     (24hr half-life)
├── techint.{tier}   (12hr half-life)
└── finint.{tier}    (7day half-life)
```

**Non-Intel Streaming** (Operational/System):
```
sx9.stream.ops.{category}
├── system.{event}      (System events, health, metrics)
├── workflow.{event}    (Workflow execution, n8n, Forge)
├── deployment.{event}  (Smart Crate, container orchestration)
├── linear.{event}      (Linear tasks, PRs, issues)
├── voice.{event}       (Voice commands, STT/TTS)
├── plasma.{event}      (Plasma state, SDT gates)
└── atlas.{event}       (Atlas daemon, cognitive ticks)
```

**Key Difference:**
- **Intel**: Has time-of-value decay, sliding window, TTL-based expiration
- **Non-Intel**: No decay, permanent or configurable retention

---

## 3. Foundation-Core Extraction Plan

### 3.1 Modules to Extract

**Extract to `ctas7-eei-system`:**
- `src/eei.rs`
- `src/eei_types.rs`
- `src/eei_processor.rs`
- `src/eei_decision_engine.rs`
- `src/distributed_eei.rs`
- `src/persistent_eei.rs`
- `src/node_interview/eei_engine.rs`
- `src/node_crate_eei_correlator.rs`

**Extract to `ctas7-streaming-core`:**
- `src/streaming.rs`
- `src/stream_hasher.rs`
- Time-of-value decay logic
- Sliding window implementation

**Extract to `ctas7-multimedia-core`:**
- `src/multimedia_content_analysis.rs`
- `src/multimedia_streams.rs`
- `src/platform_native_multimedia.rs`
- `src/platform_native_multimedia_nvnn_enhanced.rs`
- `src/media_processor.rs`

**Extract to `ctas7-voice-core`:**
- Voice-related modules (if any in foundation-core)
- Should be in `ctas7-foundation-voice` already

**Keep in Foundation-Core:**
- ✅ Hash functions (trivariate, murmur3)
- ✅ PTCC primitives
- ✅ Unicode assembly
- ✅ Core types and traits
- ✅ Basic networking (HTTP client, etc.)

---

## 4. Unified Streaming Architecture

### 4.1 Single NATS JetStream Backbone

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    UNIFIED STREAMING ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │              NATS JetStream (Single Backbone)                    │  │
│  │  Port: 18020 (NATS), 18021 (WebSocket), 18022 (JetStream)       │  │
│  └──────────────────┬───────────────────────────────────────────────┘  │
│                     │                                                    │
│         ┌───────────┴───────────┐                                        │
│         │                       │                                        │
│         ▼                       ▼                                        │
│  ┌──────────────┐      ┌──────────────┐                                 │
│  │ INTEL        │      │ NON-INTEL    │                                 │
│  │ STREAMS      │      │ STREAMS      │                                 │
│  │              │      │              │                                 │
│  │ • Time-of-   │      │ • No decay  │                                 │
│  │   Value      │      │ • Permanent │                                 │
│  │ • Decay      │      │   or TTL    │                                 │
│  │ • Sliding    │      │ • System    │                                 │
│  │   Window     │      │   events    │                                 │
│  └──────────────┘      └──────────────┘                                 │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Intel Streaming (Time-of-Value)

**Subject Pattern:** `sx9.stream.intel.{type}.{tier}`

**Features:**
- ✅ Time-of-value decay (RFC-9026)
- ✅ Sliding window theory
- ✅ Automatic TTL expiration
- ✅ Half-life-based retention
- ✅ Zero-value threshold archiving

**Streams:**
```rust
// Intel streams with decay
sx9.stream.intel.sigint.realtime    // 48hr half-life
sx9.stream.intel.humint.tactical    // 7day half-life
sx9.stream.intel.geoint.operational // 30day half-life
sx9.stream.intel.osint.realtime    // 24hr half-life
sx9.stream.intel.techint.tactical  // 12hr half-life
```

### 4.3 Non-Intel Streaming (Operational)

**Subject Pattern:** `sx9.stream.ops.{category}.{event}`

**Features:**
- ✅ No time-of-value decay
- ✅ Configurable retention (per stream)
- ✅ System/operational events
- ✅ Workflow execution
- ✅ Deployment status

**Streams:**
```rust
// Operational streams (no decay)
sx9.stream.ops.system.health        // Permanent retention
sx9.stream.ops.workflow.execution   // 30 day retention
sx9.stream.ops.deployment.status    // 90 day retention
sx9.stream.ops.linear.events        // Permanent retention
sx9.stream.ops.voice.commands       // 7 day retention
sx9.stream.ops.plasma.state         // 5 min retention (ephemeral)
sx9.stream.ops.atlas.tick           // 1 hour retention
```

---

## 5. EEI System Extraction

### 5.1 New Crate: `ctas7-eei-system`

**Extract from `ctas7-foundation-core`:**
- All EEI modules
- Distributed EEI node system
- Persistent EEI storage
- EEI decision engine
- Node interview EEI engine

**Dependencies:**
- `ctas7-foundation-core` (for trivariate hash, PTCC primitives)
- `ctas7-streaming-core` (for NATS streaming)
- `async-nats` (NATS client)
- `tokio` (async runtime)

**Integration:**
- Streams EEI fulfillment to `sx9.stream.intel.eei.{tier}`
- Uses time-of-value decay for EEI expiration
- Integrates with `ctas7-cognitive-intelligence-system`

---

## 6. Streaming Core Extraction

### 6.1 New Crate: `ctas7-streaming-core`

**Extract from `ctas7-foundation-core`:**
- `src/streaming.rs` - Streaming event ingestion
- `src/stream_hasher.rs` - Stream hashing
- Time-of-value decay trait and implementations
- Sliding window implementation
- NATS JetStream integration

**Responsibilities:**
- Unified streaming interface
- Time-of-value decay calculations
- Stream routing (intel vs non-intel)
- TTL management
- Message expiration

---

## 7. Implementation Plan

### Phase 1: Extract EEI System (Week 1)
1. Create `ctas7-eei-system` crate
2. Move EEI modules from foundation-core
3. Update dependencies
4. Test EEI functionality

### Phase 2: Extract Streaming Core (Week 2)
1. Create `ctas7-streaming-core` crate
2. Move streaming modules from foundation-core
3. Implement unified NATS interface
4. Separate intel vs non-intel routing

### Phase 3: Consolidate Streaming (Week 3)
1. Migrate TAPS → NATS JetStream
2. Migrate WebSocket servers → NATS WebSocket
3. Migrate SSE → NATS JetStream
4. Remove redundant streaming code

### Phase 4: Time-of-Value Integration (Week 4)
1. Implement time-of-value decay for intel streams
2. Implement sliding window for intel streams
3. Configure TTL per stream type
4. Test decay and expiration

---

## 8. Stream Configuration Matrix

| Stream Type | Subject Pattern | Decay? | Half-Life | Retention | TTL |
|-------------|----------------|--------|-----------|-----------|-----|
| **SIGINT** | `sx9.stream.intel.sigint.{tier}` | ✅ Yes | 48hr | 7 days | Auto |
| **HUMINT** | `sx9.stream.intel.humint.{tier}` | ✅ Yes | 7day | 30 days | Auto |
| **GEOINT** | `sx9.stream.intel.geoint.{tier}` | ✅ Yes | 30day | 90 days | Auto |
| **OSINT** | `sx9.stream.intel.osint.{tier}` | ✅ Yes | 24hr | 7 days | Auto |
| **TECHINT** | `sx9.stream.intel.techint.{tier}` | ✅ Yes | 12hr | 3 days | Auto |
| **System Health** | `sx9.stream.ops.system.health` | ❌ No | N/A | Permanent | Manual |
| **Workflow** | `sx9.stream.ops.workflow.execution` | ❌ No | N/A | 30 days | Manual |
| **Deployment** | `sx9.stream.ops.deployment.status` | ❌ No | N/A | 90 days | Manual |
| **Linear** | `sx9.stream.ops.linear.events` | ❌ No | N/A | Permanent | Manual |
| **Voice** | `sx9.stream.ops.voice.commands` | ❌ No | N/A | 7 days | Manual |
| **Plasma** | `sx9.stream.ops.plasma.state` | ❌ No | N/A | 5 min | Ephemeral |
| **Atlas** | `sx9.stream.ops.atlas.tick` | ❌ No | N/A | 1 hour | Ephemeral |

---

## 9. Migration Checklist

### From Foundation-Core
- [ ] Extract EEI modules → `ctas7-eei-system`
- [ ] Extract streaming modules → `ctas7-streaming-core`
- [ ] Extract multimedia modules → `ctas7-multimedia-core` (if needed)
- [ ] Update all dependencies
- [ ] Test extracted crates

### From TAPS/Other Streaming
- [ ] Migrate TAPS → NATS JetStream
- [ ] Migrate WebSocket servers → NATS WebSocket
- [ ] Migrate SSE → NATS JetStream
- [ ] Remove redundant streaming code
- [ ] Update all clients

### Intel Streaming
- [ ] Implement time-of-value decay
- [ ] Implement sliding window
- [ ] Configure TTL per intel type
- [ ] Test decay and expiration
- [ ] Integrate with GLAF

### Non-Intel Streaming
- [ ] Configure retention per stream
- [ ] No decay logic (keep simple)
- [ ] Test operational streams
- [ ] Integrate with ops-main-platform

---

## 10. Benefits

**Single Infrastructure:**
- ✅ One streaming system (NATS JetStream)
- ✅ Easier maintenance
- ✅ Consistent API
- ✅ Better performance

**Clear Separation:**
- ✅ Intel streams (with decay)
- ✅ Non-intel streams (no decay)
- ✅ Clear subject hierarchy
- ✅ Easy to route and filter

**Foundation-Core Cleanup:**
- ✅ Smaller, focused crate
- ✅ EEI in its own crate
- ✅ Streaming in its own crate
- ✅ Better modularity

---

**Status:** Ready for implementation

**Next Steps:**
1. Create `ctas7-eei-system` crate
2. Create `ctas7-streaming-core` crate
3. Migrate modules from foundation-core
4. Consolidate all streaming to NATS JetStream



