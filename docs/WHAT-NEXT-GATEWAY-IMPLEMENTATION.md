# What's Next: Gateway Implementation
## Immediate Actionable Steps

**Date:** December 2025  
**Status:** Implementation Roadmap  
**Priority:** High

---

## Current State

✅ **Completed:**
- Gateway architecture defined (RFC-9114 Rev 1.1)
- Smart crate manifest created
- GLAF integration plan (three-tier architecture)
- Gateway crate structure exists (`sx9-gateway/`)
- Basic handlers and protocol defined

⏳ **In Progress:**
- GLAF integration (neural + analytics)
- Foundation crate integrations

❌ **Not Started:**
- sx9-glaf-core crate (neural operations)
- GLAFClient implementation
- Foundation crate integrations (USIM, EEI, etc.)
- Gateway testing

---

## Immediate Next Steps (Priority Order)

### Phase 1: Create All Missing Neural Retrofit Crates

#### 1. Create sx9-ann-engine (ANN Observer Mode)
**Priority:** HIGH  
**Time:** 2-3 hours  
**Dependencies:** None

**Tasks:**
- [ ] Create `crates/sx9-ann-engine/` structure
- [ ] Implement observer mode (no training)
- [ ] Implement routing entropy observation
- [ ] Add weight map storage (`/data/ann/weights-v1.0.json`)
- [ ] Add smart-crate.toml
- [ ] Integrate with gateway state

**Why First:** Required for RFC-9114 Rev 1.1 neural retrofit.

---

#### 2. Create sx9-glaf-core (Neural Operations)
**Priority:** HIGH  
**Time:** 2-3 hours  
**Dependencies:** None

**Tasks:**
- [ ] Create `crates/sx9-glaf-core/` structure
- [ ] Implement ANN/GNN integration module
- [ ] Implement topology mirroring (SlotGraph observation)
- [ ] Add routing feedback mechanism
- [ ] Integrate with `ctas7-slotgraph-engine`
- [ ] Add smart-crate.toml
- [ ] Add to gateway state

**Why Second:** Neural operations component for RFC-9114 Rev 1.1.

---

#### 3. Create GLAFClient (Data Analytics)
**Priority:** HIGH  
**Time:** 1-2 hours  
**Dependencies:** None (uses existing GLAF server)

**Tasks:**
- [ ] Create `sx9-gateway/src/glaf_client.rs`
- [ ] Implement HTTP client to `ctas7-glaf-graph-server` (port 18050)
- [ ] Add graph query methods
- [ ] Add GLAF math operations (Matroid, Hawkes, Convergence)
- [ ] Add error handling and fallback
- [ ] Integrate with gateway state

**Why Third:** Needed for all graph operations and future analytic workbench.

---

#### 4. Create sx9-dsl-engine (DSL Symbolic Control)
**Priority:** MEDIUM  
**Time:** 2-3 hours  
**Dependencies:** None

**Tasks:**
- [ ] Create `crates/sx9-dsl-engine/` structure
- [ ] Implement DSL runtime
- [ ] Add WASM execution support
- [ ] Add playbook execution
- [ ] Add smart-crate.toml
- [ ] Integrate with gateway state

**Why Fourth:** DSL engine for symbolic control.

---

#### 5. Create sx9-plasma-defender (PLASMA Health Monitoring)
**Priority:** MEDIUM  
**Time:** 1-2 hours  
**Dependencies:** None

**Tasks:**
- [ ] Create `crates/sx9-plasma-defender/` structure
- [ ] Implement health endpoint (`/health`)
- [ ] Implement metrics endpoint (`/metrics`)
- [ ] Add latency enforcement
- [ ] Add PLASMA state monitoring
- [ ] Add smart-crate.toml
- [ ] Integrate with gateway state

**Why Fifth:** PLASMA health monitoring.

---

#### 6. Create/Integrate sx9-atlas-daemon (ATLAS Cognitive Tick)
**Priority:** MEDIUM  
**Time:** 1-2 hours  
**Dependencies:** `sx9-atlas-bus`

**Tasks:**
- [ ] Option A: Create `crates/sx9-atlas-daemon/` (separate crate)
- [ ] Option B: Integrate into `sx9-atlas-bus` (recommended)
- [ ] Implement 1ms tick interval
- [ ] Implement OODA loop (observe, orient, decide, act)
- [ ] Add phase sequence execution
- [ ] Add smart-crate.toml (if separate)
- [ ] Integrate with gateway state

**Why Sixth:** ATLAS cognitive operations.

---

### Phase 2: PLASMA-ECS Architecture

#### 7. Create sx9-plasma-ecs (Unified ECS)
**Priority:** HIGH  
**Time:** 4-6 hours  
**Dependencies:** None

**Tasks:**
- [ ] Create `crates/sx9-plasma-ecs/` structure
- [ ] Integrate Legion (Layer 2 - deterministic batch)
- [ ] Integrate apecs (Layer 1 - async I/O)
- [ ] Integrate ATLAS (Layer 3 - cognitive)
- [ ] Unified PlasmaState
- [ ] Preserve Slot Graph integration (hash/unicode routing)
- [ ] Add smart-crate.toml
- [ ] Test Task > Skill > Tool Chain > Tool hierarchy

**Why Critical:** Fixes ECS fragmentation, unifies architecture.

---

### Phase 3: Gateway Integration

#### 8. Update Gateway State & Handlers
**Priority:** HIGH  
**Time:** 2-3 hours  
**Dependencies:** All neural retrofit crates (#1-6), PLASMA-ECS (#7)

**Tasks:**
- [ ] Update gateway state with all neural retrofit components
- [ ] Update `handle_get_graph()` to use GLAF client
- [ ] Update `handle_get_fusion_nodes()` to use GLAF client
- [ ] Update `handle_expand_node()` to use GLAF client
- [ ] Update `handle_run_correlation()` to use GLAF client
- [ ] Add PLASMA-ECS queries
- [ ] Add fallback to SurrealDB if GLAF unavailable
- [ ] Test graph operations end-to-end

**Why Critical:** Makes gateway functional with all components.

---

#### 9. Integrate Foundation Crates
**Priority:** MEDIUM  
**Time:** 3-4 hours  
**Dependencies:** Gateway handlers (#8)

**Tasks:**
- [ ] USIM integration (`ctas7-usim` or create `sx9-usim`)
- [ ] EEI integration (`ctas7-eei` or create `sx9-eei`)
- [ ] Foundation Manifold integration
- [ ] Foundation Math integration
- [ ] Government Data Manifold integration
- [ ] Add integration handlers to gateway

**Why Important:** Foundation crates provide core capabilities.

---

#### 10. Gateway Testing & Validation
**Priority:** HIGH  
**Time:** 2-3 hours  
**Dependencies:** All above

**Tasks:**
- [ ] Unit tests for handlers
- [ ] Integration tests with GLAF server
- [ ] Test foundation crate integrations
- [ ] Test smart crate features
- [ ] Test PLASMA-ECS integration
- [ ] Performance testing (latency, throughput)
- [ ] RFC-9114 compliance validation

**Why Critical:** Ensures everything works together.

---

## Quick Wins (Can Do Now)

### Option A: Start with sx9-glaf-core
**Why:** Neural operations are needed for RFC-9114 Rev 1.1  
**Time:** 2-3 hours  
**Impact:** High

### Option B: Start with GLAFClient
**Why:** Enables graph operations immediately  
**Time:** 1-2 hours  
**Impact:** High

### Option C: Update Gateway Handlers First
**Why:** Makes existing gateway more functional  
**Time:** 1-2 hours  
**Impact:** Medium (needs GLAF client first)

---

## Recommended Order

**Phase 1 (Week 1): Create All Missing Crates**
1. Create `sx9-ann-engine` (ANN observer mode)
2. Create `sx9-glaf-core` (neural operations)
3. Create `GLAFClient` (data analytics)
4. Create `sx9-dsl-engine` (DSL symbolic control)
5. Create `sx9-plasma-defender` (PLASMA health monitoring)
6. Create/integrate `sx9-atlas-daemon` (ATLAS cognitive tick)
7. Create `sx9-plasma-ecs` (PLASMA-ECS architecture)

**Phase 2 (Week 2): Gateway Integration**
8. Update gateway state & handlers
9. Integrate foundation crates (USIM, EEI, etc.)
10. Gateway testing & validation

**Phase 3 (Week 3): Advanced Features**
11. Ops-Main-Platform integration
12. Layer 2 execution (Kali ISO)
13. Streaming intelligence (NATS JetStream)

---

## Dependencies Map

```
sx9-glaf-core
    │
    ├─→ Gateway State
    │
GLAFClient
    │
    ├─→ Gateway Handlers
    │       │
    │       └─→ Foundation Crates
    │               │
    │               └─→ Smart Crate Features
    │                       │
    │                       └─→ Testing
```

---

## Questions to Answer

1. **Start with sx9-glaf-core or GLAFClient?**
   - Recommendation: Start with **GLAFClient** (faster, enables graph ops immediately)

2. **Create new crates or use existing?**
   - Recommendation: Create `sx9-*` crates for gateway-specific components
   - Use existing `ctas7-*` crates for foundation

3. **Test as we go or build then test?**
   - Recommendation: Test as we go (incremental validation)

---

## Next Action

**Recommended:** Start with **GLAFClient** implementation

**Command:**
```bash
cd /Users/cp5337/Developer/synaptix9-workflow-system
# Create GLAFClient module
```

**Why:** 
- Fastest to implement (1-2 hours)
- Enables graph operations immediately
- No dependencies on other new crates
- Uses existing GLAF server (port 18050)

---

**Status:** Ready to implement. Choose starting point and proceed.

