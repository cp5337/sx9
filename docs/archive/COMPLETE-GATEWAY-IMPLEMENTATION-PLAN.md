# Complete Gateway Implementation Plan
## All Missing Crates, ECS Fixes, and Renaming

**Date:** December 2025  
**Status:** Complete Implementation Plan  
**Priority:** HIGH

---

## Executive Summary

**Missing Components:**
1. ✅ `sx9-glaf-core` - Neural operations (ANN/GNN + topology mirroring)
2. ❌ `sx9-ann-engine` - ANN observer mode (RFC-9114 Rev 1.1)
3. ❌ `sx9-dsl-engine` - DSL symbolic control
4. ❌ `sx9-plasma-defender` - PLASMA health monitoring
5. ❌ `sx9-atlas-daemon` - ATLAS cognitive tick (may integrate with `sx9-atlas-bus`)
6. ❌ `GLAFClient` - Data analytics client (HTTP to GLAF server)

**Also Needed:**
- ❌ **PLASMA-ECS** - Unified ECS architecture (Legion + apecs + ATLAS)
- ❌ **Crate Renaming** - Clone/rename `ctas7-*` to `sx9-*` where appropriate

---

## 1. Missing Neural Retrofit Crates

### 1.1 sx9-ann-engine (ANN Observer Mode)

**Purpose:** ANN observer mode for neural retrofit (RFC-9114 Rev 1.1)

**Location:** `crates/sx9-ann-engine/`

**Features:**
- Observer mode (no training, just observation)
- Routing entropy observation
- Weight map storage (`/data/ann/weights-v1.0.json`)
- Entropy seed: `murmur3-64`
- Sync interval: 1000ms

**Dependencies:**
- `ctas7-foundation-core`
- `sx9-atlas-bus`
- `serde`, `serde_json`

**Status:** ❌ Not created

---

### 1.2 sx9-dsl-engine (DSL Symbolic Control)

**Purpose:** DSL engine for symbolic control

**Location:** `crates/sx9-dsl-engine/`

**Features:**
- Symbolic language runtime
- WASM execution
- Reload on change
- DSL playbook execution

**Dependencies:**
- `ctas7-foundation-core`
- `wasmtime` or `wasmer`
- `serde`, `serde_json`

**Status:** ❌ Not created

---

### 1.3 sx9-plasma-defender (PLASMA Health Monitoring)

**Purpose:** PLASMA defender for health monitoring

**Location:** `crates/sx9-plasma-defender/`

**Features:**
- Health endpoint (`/health`)
- Metrics endpoint (`/metrics`)
- Latency enforcement
- PLASMA state monitoring

**Dependencies:**
- `ctas7-foundation-core`
- `sx9-atlas-bus` (for PlasmaState)
- `axum` (for endpoints)

**Status:** ❌ Not created

---

### 1.4 sx9-atlas-daemon (ATLAS Cognitive Tick)

**Purpose:** ATLAS daemon for cognitive operations

**Location:** `crates/sx9-atlas-daemon/`

**Features:**
- 1ms tick interval
- OODA loop (observe, orient, decide, act)
- Phase sequence execution
- Integration with `sx9-atlas-bus`

**Dependencies:**
- `sx9-atlas-bus`
- `ctas7-foundation-core`
- `tokio`

**Status:** ❌ Not created (may integrate with `sx9-atlas-bus`)

**Note:** May be integrated into `sx9-atlas-bus` instead of separate crate.

---

### 1.5 sx9-glaf-core (Neural Operations)

**Purpose:** GLAF neural operations (ANN/GNN + topology mirroring)

**Location:** `crates/sx9-glaf-core/`

**Features:**
- ANN/GNN model inference
- Topology mirroring (SlotGraph observation)
- Routing feedback
- Integration with `ctas7-slotgraph-engine`

**Dependencies:**
- `ctas7-slotgraph-engine`
- `ctas7-foundation-core`
- `sx9-atlas-bus`

**Status:** ❌ Not created

---

### 1.6 GLAFClient (Data Analytics)

**Purpose:** HTTP client to GLAF server for data analytics

**Location:** `sx9-gateway/src/glaf_client.rs`

**Features:**
- HTTP client to `ctas7-glaf-graph-server` (port 18050)
- Graph queries
- GLAF math operations (Matroid, Hawkes, Convergence)
- Error handling and fallback

**Dependencies:**
- `reqwest`
- `serde`, `serde_json`

**Status:** ❌ Not created

---

## 2. PLASMA-ECS Architecture

### 2.1 Current ECS Fragmentation

**Problem:**
- `ctas7-world-ecs` uses **Legion** (deterministic batch processing)
- `ctas7-network-world` uses **hecs + bevy_ecs** (async I/O)
- No unified architecture
- Worlds can't easily share entities

**Solution:** PLASMA-ECS (Three-Layer Architecture)

### 2.2 PLASMA-ECS Structure

**Crate:** `ctas7-plasma-ecs` (or `sx9-plasma-ecs` for gateway?)

**Three Layers:**
1. **Layer 1: apecs** - Async I/O operations
2. **Layer 2: Legion** - Deterministic batch processing + Slot Graph routing
3. **Layer 3: ATLAS** - Cognitive operations (sx9-atlas-bus)

**Integration:**
- Unified PlasmaState
- Shared entity IDs (trivariate hash)
- Cross-layer entity relationships

**Status:** ❌ Not created

---

## 3. Crate Renaming/Cloning

### 3.1 Crates to Create (New sx9-* Crates)

| Crate | Source | Action |
|-------|--------|--------|
| `sx9-ann-engine` | New | Create from scratch |
| `sx9-glaf-core` | New | Create wrapper for GLAF |
| `sx9-dsl-engine` | New | Create from scratch |
| `sx9-plasma-defender` | New | Create from scratch |
| `sx9-atlas-daemon` | `sx9-atlas-bus` | May integrate into existing |
| `sx9-plasma-ecs` | New | Create PLASMA-ECS architecture |

### 3.2 Foundation Crates (Keep CTAS7)

**Keep as `ctas7-*`:**
- `ctas7-foundation-core` (gold disk)
- `ctas7-real-port-manager`
- `ctas7-hashing-engine`
- `ctas7-neural-mux`
- `ctas7-slotgraph-engine`
- All other foundation crates

**Rationale:** Foundation crates are the gold disk - don't rename.

---

## 4. Implementation Order

### Phase 1: Core Neural Retrofit Crates (Week 1)

**Priority:** HIGH

1. **Create sx9-ann-engine**
   - Observer mode implementation
   - Routing entropy observation
   - Weight map storage
   - Time: 2-3 hours

2. **Create sx9-glaf-core**
   - ANN/GNN integration
   - Topology mirroring
   - Routing feedback
   - Time: 2-3 hours

3. **Create GLAFClient**
   - HTTP client to GLAF server
   - Graph queries
   - Math operations
   - Time: 1-2 hours

4. **Create sx9-dsl-engine**
   - DSL runtime
   - WASM execution
   - Playbook execution
   - Time: 2-3 hours

5. **Create sx9-plasma-defender**
   - Health monitoring
   - Metrics endpoint
   - Latency enforcement
   - Time: 1-2 hours

6. **Create/Integrate sx9-atlas-daemon**
   - Option A: Create separate crate
   - Option B: Integrate into `sx9-atlas-bus`
   - Time: 1-2 hours

**Total Phase 1:** ~10-15 hours

---

### Phase 2: PLASMA-ECS Architecture (Week 2)

**Priority:** HIGH

1. **Create PLASMA-ECS crate**
   - Integrate Legion (Layer 2)
   - Integrate apecs (Layer 1)
   - Integrate ATLAS (Layer 3)
   - Unified PlasmaState
   - Time: 4-6 hours

2. **Preserve Slot Graph Integration**
   - Verify hash/unicode routing works
   - Test Task > Skill > Tool Chain > Tool hierarchy
   - Ensure <1ms routing performance
   - Time: 2-3 hours

3. **Network World Migration (Optional)**
   - Migrate to apecs (Layer 1)
   - Bridge to PLASMA-ECS
   - Time: 3-4 hours

**Total Phase 2:** ~9-13 hours

---

### Phase 3: Gateway Integration (Week 2-3)

**Priority:** HIGH

1. **Update Gateway State**
   - Add all neural retrofit components
   - Add GLAF client
   - Add PLASMA-ECS
   - Time: 2-3 hours

2. **Update Gateway Handlers**
   - Use GLAF client for graph operations
   - Integrate neural retrofit components
   - Add PLASMA-ECS queries
   - Time: 2-3 hours

3. **Test Integration**
   - End-to-end testing
   - Performance validation
   - RFC-9114 compliance
   - Time: 2-3 hours

**Total Phase 3:** ~6-9 hours

---

## 5. Complete Checklist

### Neural Retrofit Crates
- [ ] Create `sx9-ann-engine` crate
- [ ] Create `sx9-glaf-core` crate
- [ ] Create `sx9-dsl-engine` crate
- [ ] Create `sx9-plasma-defender` crate
- [ ] Create/integrate `sx9-atlas-daemon`
- [ ] Create `GLAFClient` in gateway

### PLASMA-ECS
- [ ] Create `sx9-plasma-ecs` crate (or `ctas7-plasma-ecs`?)
- [ ] Integrate Legion (Layer 2)
- [ ] Integrate apecs (Layer 1)
- [ ] Integrate ATLAS (Layer 3)
- [ ] Unified PlasmaState
- [ ] Preserve Slot Graph integration
- [ ] Test hash/unicode routing

### Gateway Integration
- [ ] Update gateway state with all components
- [ ] Update handlers to use GLAF client
- [ ] Integrate neural retrofit components
- [ ] Add PLASMA-ECS queries
- [ ] Test end-to-end
- [ ] Performance validation

### Smart Crate Manifests
- [ ] Add smart-crate.toml to `sx9-ann-engine`
- [ ] Add smart-crate.toml to `sx9-glaf-core`
- [ ] Add smart-crate.toml to `sx9-dsl-engine`
- [ ] Add smart-crate.toml to `sx9-plasma-defender`
- [ ] Add smart-crate.toml to `sx9-atlas-daemon` (if separate)
- [ ] Add smart-crate.toml to `sx9-plasma-ecs`

### Documentation
- [ ] Update RFC-9114 Rev 1.1 with crate locations
- [ ] Document PLASMA-ECS architecture
- [ ] Document renaming strategy
- [ ] Create migration guide

---

## 6. Quick Start Commands

### Create All Neural Retrofit Crates

```bash
cd /Users/cp5337/Developer/synaptix9-workflow-system/crates

# Create crates
cargo new --lib sx9-ann-engine
cargo new --lib sx9-glaf-core
cargo new --lib sx9-dsl-engine
cargo new --lib sx9-plasma-defender
cargo new --bin sx9-atlas-daemon
cargo new --lib sx9-plasma-ecs

# Create GLAFClient module
touch ../crates/sx9-gateway/src/glaf_client.rs
```

### Add Smart Crate Manifests

```bash
# Copy template and customize for each
cp sx9-gateway/smart-crate.toml sx9-ann-engine/smart-crate.toml
cp sx9-gateway/smart-crate.toml sx9-glaf-core/smart-crate.toml
cp sx9-gateway/smart-crate.toml sx9-dsl-engine/smart-crate.toml
cp sx9-gateway/smart-crate.toml sx9-plasma-defender/smart-crate.toml
cp sx9-gateway/smart-crate.toml sx9-plasma-ecs/smart-crate.toml
```

---

## 7. Summary

**Missing Crates:**
1. `sx9-ann-engine` - ANN observer mode
2. `sx9-glaf-core` - Neural operations
3. `sx9-dsl-engine` - DSL symbolic control
4. `sx9-plasma-defender` - PLASMA health monitoring
5. `sx9-atlas-daemon` - ATLAS cognitive tick
6. `GLAFClient` - Data analytics client
7. `sx9-plasma-ecs` - PLASMA-ECS architecture

**Actions Needed:**
- Create all missing crates
- Implement PLASMA-ECS architecture
- Integrate into gateway
- Test end-to-end

**Estimated Time:** ~25-37 hours total

---

**Status:** Complete plan ready. Ready to start implementation.



