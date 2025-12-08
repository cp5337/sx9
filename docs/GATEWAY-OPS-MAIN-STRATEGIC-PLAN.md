# Gateway & Ops-Main Strategic Plan
## Essential Crates, Decluttering, and Operational Readiness

**Date:** December 2025  
**Status:** Strategic Assessment  
**Goal:** Get ops-main online, Smart Crate operational, QA running hard, Kali ISO Layer 2 ready, containerize canonical

---

## 🎯 Executive Summary

**Current State:**
- 30 canonical crates defined (down from 296)
- Smart Crate System exists but has Blake3 contamination + missing integrations
- Lightning QA exists but disconnected from Statistical CDN
- Ops-main-platform (React/Vite) ready but needs gateway bridge
- Kali ISO exists but needs Layer 2 execution integration
- CDN Terraform spec ready for three-tier deployment

**Architecture Principle:** System runs **minimal startup** and **escalates through NATS + DSL playbooks**. Not all components run simultaneously - they activate on demand.

**Critical Path (Minimal Startup):**
1. **Gateway Core** → sx9-gateway, sx9-atlas-bus, ctas7-neural-mux, ctas7-real-port-manager
2. **Database Layer** → SurrealDB, ctas7-cdn-data-fabric, ctas7-nats-fabric
3. **Frontend** → ops-main-platform connected via gateway
4. **Smart Crate Fix** → Remove Blake3, basic integrations
5. **Kali ISO Layer 2** → eBPF/XDP execution via SDT protocol

**Parallel Track (Escalation Components):**
- **Agents** → Agentic infrastructure (4 crates) - activate via NATS/DSL
- **Cognition** → Cognitive inference, GLAF, matroids, math (4 crates) - activate on demand
- **Intel** → Intel systems - activate via playbooks
- **Tools** → Tool execution - activate via gateway requests
- **Defense** → Plasma Defender - activate via threat detection

**Key Principle:** This is a **cognitive computing system** with **minimal startup + escalation architecture**. Components activate through NATS messages and DSL playbooks, not all at once.

---

## 1. Essential Crates for Gateway/Ops-Main

### 1.1 Core Foundation (5 Crates) - **KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-foundation-core` | Trivariate hash, PTCC primitives | **CRITICAL** - All entity IDs, routing |
| `ctas7-foundation-manifold` | Deterministic routing | **CRITICAL** - Neural Mux routing |
| `ctas7-real-port-manager` | Dynamic port allocation | **CRITICAL** - Port management |
| `sx9-atlas-bus` | PlasmaState, crystal, SDT gate | **CRITICAL** - Command gating |
| `sx9-gateway` | WebSocket API gateway | **CRITICAL** - Frontend bridge |

### 1.2 Gateway Infrastructure (3 Crates) - **KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-neural-mux` | <250ns routing | **CRITICAL** - Request routing |
| `ctas7-cdn-data-fabric` | Multi-DB aggregation | **REQUIRED** - Database queries |
| `ctas7-nats-fabric` | NATS messaging | **REQUIRED** - Service communication |

### 1.3 Ops-Main Integration (2 Crates) - **KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-wasm-primitives` | 32 Universal Primitives | **REQUIRED** - Browser execution |
| `ctas7-cognitive-execution-tool` | L* Learning, iTunes protocol | **REQUIRED** - Tool execution |

### 1.4 Kali ISO Layer 2 (1 Crate) - **KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `tools/kali-plasma/agent` | Kali ISO agent | **REQUIRED** - Layer 2 tool execution |

### 1.5 Smart Crate System (1 Crate) - **FIX & KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-orchestrator` | Smart Crate Orchestrator | **REQUIRED** - Container orchestration |

### 1.6 QA System (1 Crate) - **FIX & KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-cdn-isolated-monitoring` | Lightning QA integration | **REQUIRED** - QA metrics |

### 1.7 Cognitive Computing (4 Crates) - **CRITICAL - KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-cognitive-inference` | Full cognitive pipeline | **CRITICAL** - Core cognitive computing |
| `ctas7-glaf-matroid-core` | Information independence, H2 score | **CRITICAL** - Cognitive graph analysis |
| `ctas7-gnn-fabric` | GNN embeddings, training | **CRITICAL** - Predictive threat analysis |
| `ctas7-atlas-daemon` | 1ms cognitive tick, H1/H2 convergence | **CRITICAL** - Cognitive orchestration |

### 1.8 Agentic Infrastructure (4 Crates) - **CRITICAL - KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-agentic-core` | Base agent types, traits | **CRITICAL** - Agent foundation |
| `ctas7-persona-core` | Persona definitions, lifecycle | **CRITICAL** - Agent personas |
| `ctas7-agent-registry` | gRPC mesh registry | **CRITICAL** - Agent coordination |
| `ctas7-agent-dispatch` | QA-to-Agent routing, ABE integration | **CRITICAL** - Agent dispatch |

### 1.9 Defense Component (1 Crate) - **CRITICAL - KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-plasma-defender` | Security monitoring, OSSEC, Thalmic | **CRITICAL** - Defense operations |

### 1.10 ML Models (3 Components) - **INTEGRATE**

| Component | Purpose | Gateway/Ops-Main Role |
|-----------|---------|----------------------|
| DistilBERT | Text classification | **REQUIRED** - Threat pattern recognition |
| Phi3 | Threat summarization | **REQUIRED** - Intelligence reports |
| GNN | Graph analysis | **REQUIRED** - Threat actor relationships |

### 1.11 Supporting Infrastructure (3 Crates) - **KEEP**

| Crate | Purpose | Gateway/Ops-Main Role |
|-------|---------|----------------------|
| `ctas7-foundation-math` | Matroid, HMM, Hawkes Process | **REQUIRED** - Cognitive math foundation |
| `ctas7-foundation-data` | Unified schema, storage abstraction | **REQUIRED** - Data layer |
| `ctas7-thalmic-filter` | DistilBERT gating (<10ms) | **REQUIRED** - Cognitive filtering |

**Total Essential: 26 crates/components** (refined from 30, keeping cognitive core)

### 1.12 Parallel Track - Escalation Components (14 crates)

**These activate via NATS + DSL playbooks (not minimal startup):**

**Agentic Infrastructure (4 crates) - Parallel Track:**
- `ctas7-agentic-core` - Base agent types (activate via agent dispatch)
- `ctas7-persona-core` - Persona definitions (activate via agent registry)
- `ctas7-agent-registry` - gRPC mesh registry (activate via NATS)
- `ctas7-agent-dispatch` - QA-to-Agent routing (activate via playbook)

**Cognitive Computing (4 crates) - Parallel Track:**
- `ctas7-cognitive-inference` - Full cognitive pipeline (activate via cognitive requests)
- `ctas7-glaf-matroid-core` - GLAF graph analysis (activate via graph queries)
- `ctas7-gnn-fabric` - GNN embeddings (activate via threat analysis)
- `ctas7-foundation-math` - Matroid, HMM, Hawkes (activate via math operations)

**Intel Systems (2 crates) - Parallel Track:**
- `ctas7-ingestion-pipeline` - Canonical ingestion (activate via intel playbook)
- `ctas7-yaml-dsl-converter` - DSL conversion (activate via playbook execution)

**Defense (1 crate) - Parallel Track:**
- `ctas7-plasma-defender` - Security monitoring (activate via threat detection)

**Supporting (3 crates) - Parallel Track:**
- `ctas7-foundation-voice` - Voice integration (activate via voice requests)
- `ctas7-interview-engine` - Node/Crate interviews (activate via interview playbook)
- `ctas7-thalmic-filter` - DistilBERT gating (activate via cognitive filtering)

**Total System: 12 minimal startup + 14 escalation = 26 crates**

---

## 1.13 System Component Architecture

**Complete Component Inventory:**

| Category | Components | Activation Method |
|----------|------------|-------------------|
| **Front Ends** | ops-main-platform (React/Vite), Command Center | Direct HTTP/WebSocket |
| **Gateway** | sx9-gateway, sx9-atlas-bus | Minimal startup |
| **Databases** | SurrealDB, PostgreSQL, Redis, Sled, Sledis | Minimal startup (core), escalate (others) |
| **Agents** | ctas7-agentic-core, ctas7-persona-core, ctas7-agent-registry, ctas7-agent-dispatch | **NATS + DSL playbooks** |
| **Intel** | ctas7-ingestion-pipeline, ctas7-yaml-dsl-converter, PTCC-TETH database | **NATS + DSL playbooks** |
| **Cognition** | ctas7-cognitive-inference, ctas7-glaf-matroid-core, ctas7-gnn-fabric, ctas7-foundation-math | **NATS + DSL playbooks** |
| **Tools** | ctas7-cognitive-execution-tool, Kali ISO, WASM primitives | Gateway requests + NATS |
| **Orchestration** | ctas7-orchestrator (Smart Crate), ctas7-neural-mux | Minimal startup (mux), escalate (orchestrator) |
| **State Management** | sx9-atlas-bus (PlasmaState), ATLAS Daemon | Minimal startup |
| **Security/Defense** | ctas7-plasma-defender, ctas7-thalmic-filter | **NATS + threat detection** |
| **Routing** | ctas7-neural-mux, ctas7-foundation-manifold | Minimal startup |
| **Port Management** | ctas7-real-port-manager | Minimal startup |
| **CDN/Distribution** | ctas7-cdn-data-fabric, ctas7-cdn-isolated-monitoring | Minimal startup (data-fabric), escalate (monitoring) |
| **QA/Validation** | Lightning QA, Statistical CDN | **NATS + QA playbooks** |

**Missing Components (to add):**
- **Consolidated Agent Registry** - Where should this live? (Recommendation: `ctas7-agent-registry` or new `ctas7-unified-agent-registry`)
- **Consolidated Intel Gateway** - Where should this live? (Recommendation: New `ctas7-intel-gateway` crate)
- **DSL Playbook Engine** - Playbook execution engine (Recommendation: Part of `ctas7-orchestrator` or new `ctas7-dsl-engine`)
- **NATS Escalation Router** - Routes escalation requests (Recommendation: Part of `ctas7-neural-mux` or new `ctas7-escalation-router`)

---

## 2. Smart Crate System - Fix & Restore

### 2.1 Current Issues

**Blake3 Contamination:**
- 47 files using Blake3 instead of Murmur3-128
- Semantic lock files non-compliant
- Container verification using wrong hash

**Missing Integrations:**
- Health Dashboard not connected
- Lightning QA (port 18109) not connected to Statistical CDN
- ATLAS Daemon missing 1ms cognitive tick
- PLASMA Monitoring not connected
- Neural Mux incomplete <250ns routing

### 2.2 Fix Plan

**Phase 1: Blake3 Removal (Week 1)**
1. Audit all 47 files with Blake3 references
2. Replace with Murmur3-128 trivariate hash
3. Update semantic lock files
4. Fix container verification
5. Update CDN routing hash schemes

**Phase 2: Integration (Week 2)**
1. Connect Health Dashboard to Smart Crate monitoring
2. Bridge Lightning QA (18109) → Statistical CDN
3. Integrate ATLAS Daemon 1ms cognitive tick
4. Connect PLASMA Monitoring threat pipeline
5. Complete Neural Mux <250ns routing

**Phase 3: Testing (Week 3)**
1. Run Lightning QA on all Smart Crates
2. Verify RFC-9001 through RFC-9005 compliance
3. Test Health Dashboard integration
4. Validate ATLAS cognitive tick
5. Test PLASMA threat detection

---

## 3. QA System - Get Running Hard

### 3.1 Lightning QA Architecture

**Current State:**
- Lightning QA Engine exists (port 18109)
- GPU-accelerated script-only analysis
- Non-invasive, read-only
- Automated PR generation

**Missing:**
- Connection to Statistical CDN
- Integration with Smart Crate System
- Health Dashboard metrics
- Gateway/ops-main validation

### 3.2 QA Hardening Plan

**Phase 1: Connect Lightning QA (Week 1)**
1. Bridge Lightning QA → Statistical CDN
2. Integrate with Smart Crate System
3. Add Health Dashboard metrics
4. Create QA dashboard in ops-main

**Phase 2: Gateway/Ops-Main Validation (Week 2)**
1. Run Lightning QA on sx9-gateway
2. Validate ops-main-platform components
3. Test WebSocket bridge
4. Verify database connections
5. Test ML model integration

**Phase 3: Continuous QA (Week 3)**
1. Set up automated QA runs
2. Integrate with CI/CD pipeline
3. Create QA reports in ops-main
4. Alert on QA failures
5. Track QA metrics over time

---

## 4. Ops-Main-Platform - Get Online

### 4.1 Current State

**Frontend:**
- React/Vite (port 15174)
- Radix UI components
- Playwright modal inventory
- HD4 phase pages
- Database connections
- Graph visualization

**Missing:**
- WebSocket bridge to sx9-gateway
- Real backend integration
- Database connection status
- CDN health monitoring
- Plasma state streaming

### 4.2 Ops-Main Integration Plan

**Phase 1: Gateway Bridge (Week 1)**
1. Implement WebSocket bridge (TASK-023)
2. Connect React frontend to sx9-gateway
3. Route all UI requests through gateway
4. Handle WebSocket message protocol

**Phase 2: Backend Integration (Week 2)**
1. Connect to SurrealDB via gateway
2. Integrate NATS for real-time updates
3. Connect to all 8 CDNs
4. Integrate Neural Mux routing
5. Add Plasma state streaming

**Phase 3: UI Enhancements (Week 3)**
1. Add database connection status indicators
2. Show CDN health in UI
3. Display Plasma state in real-time
4. Add modal inventory integration
5. Create threat intelligence dashboard

---

## 5. Kali ISO Layer 2 Execution

### 5.1 Current State

**Kali ISO:**
- Kali Plasma ISO exists
- eBPF tooling scaffolded
- NATS tunnel implemented
- Operator build defined

**Missing:**
- Layer 2 execution via eBPF/XDP
- SDT protocol integration (EtherType 0xSD77)
- Gateway routing for tool execution
- Tool result handling

### 5.2 Kali ISO Integration Plan

**Phase 1: Layer 2 Execution (Week 1)**
1. Complete eBPF/XDP tool execution
2. Implement SDT protocol (EtherType 0xSD77)
3. Route tool requests from gateway
4. Handle tool results via Unicode runes

**Phase 2: Gateway Integration (Week 2)**
1. Implement TASK-025 (Kali Layer 2 Execution)
2. Route tool execution through gateway
3. Handle tool results (TASK-027)
4. Store results in SurrealDB
5. Stream results to ops-main UI

**Phase 3: Tool Coverage (Week 3)**
1. Support all 10 Kali tools
2. Test nmap, masscan, nuclei, sqlmap
3. Test hydra, metasploit, responder
4. Test impacket, bloodhound, crackmapexec
5. Validate 5-12ns execution latency

---

## 6. Containerization - Canonical Crates

### 6.1 Containerization Strategy

**Three-Tier CDN (from Terraform spec):**
1. **Cloudflare R2** - Public/semi-public assets ($0 egress)
2. **GCP Cloud CDN** - IAM-gated assets ($0 idle LB)
3. **Internal** - Classified/air-gapped (existing infra)

### 6.2 Containerization Plan

**Phase 1: Essential Crates (Week 1)**
1. Containerize sx9-gateway
2. Containerize sx9-atlas-bus
3. Containerize ctas7-neural-mux
4. Containerize ctas7-real-port-manager
5. Containerize ctas7-cdn-data-fabric

**Phase 2: Ops-Main Services (Week 2)**
1. Containerize ops-main-platform (React build)
2. Containerize Kali ISO agent
3. Containerize ML model services (DistilBERT, Phi3, GNN)
4. Containerize Lightning QA
5. Containerize Smart Crate Orchestrator

**Phase 3: CDN Deployment (Week 3)**
1. Deploy to Cloudflare R2 (public assets)
2. Deploy to GCP Cloud CDN (IAM-gated)
3. Set up internal deployment (air-gapped)
4. Configure Terraform for all tiers
5. Test CDN routing and caching

---

## 7. Prioritized Execution Plan

### Week 1: Minimal Startup + Parallel Tracks
**Days 1-2: Minimal Startup Configuration**
- [ ] Define minimal startup crates (12 crates: gateway, databases, routing, port manager)
- [ ] Configure NATS escalation subjects
- [ ] Create DSL playbook templates for escalation
- [ ] Set up escalation routing in neural-mux

**Days 3-4: Gateway Core (Minimal Startup)**
- [ ] sx9-gateway operational
- [ ] sx9-atlas-bus connected
- [ ] ctas7-neural-mux routing active
- [ ] ctas7-real-port-manager allocating ports
- [ ] Database connections (SurrealDB, NATS)

**Days 5-7: Parallel Track Setup**
- [ ] **Track A: Agents** - Set up agent registry, dispatch, personas (activate via NATS)
- [ ] **Track B: Cognition** - Set up cognitive inference, GLAF, GNN (activate via NATS)
- [ ] **Track C: Intel** - Set up ingestion pipeline, DSL converter (activate via playbooks)
- [ ] **Track D: Defense** - Set up Plasma Defender (activate via threat detection)

**Days 3-4: Smart Crate Fix**
- [ ] Remove Blake3 contamination (47 files)
- [ ] Replace with Murmur3-128
- [ ] Update semantic lock files
- [ ] Fix container verification

**Days 5-7: Smart Crate Integration**
- [ ] Connect Health Dashboard
- [ ] Bridge Lightning QA → Statistical CDN
- [ ] Integrate ATLAS Daemon
- [ ] Connect PLASMA Monitoring
- [ ] Complete Neural Mux routing

### Week 2: Gateway & Ops-Main + Escalation Testing
**Days 1-3: Gateway Bridge (Minimal Startup)**
- [ ] Implement TASK-023 (WebSocket bridge)
- [ ] Connect React frontend to gateway
- [ ] Route UI requests through gateway
- [ ] Handle WebSocket protocol

**Days 4-5: Backend Integration (Minimal Startup)**
- [ ] Connect SurrealDB via gateway
- [ ] Integrate NATS for real-time
- [ ] Connect core CDNs (data-fabric)
- [ ] Add Plasma state streaming

**Days 6-7: Escalation Testing (Parallel Tracks)**
- [ ] Test agent escalation via NATS (Track A)
- [ ] Test cognitive escalation via NATS (Track B)
- [ ] Test intel escalation via DSL playbooks (Track C)
- [ ] Test defense escalation via threat detection (Track D)
- [ ] Verify escalation routing works correctly

### Week 3: Kali ISO & Containerization
**Days 1-3: Kali ISO Layer 2**
- [ ] Complete eBPF/XDP execution
- [ ] Implement SDT protocol
- [ ] Route through gateway (TASK-025)
- [ ] Handle tool results (TASK-027)

**Days 4-5: ML Model Integration**
- [ ] Integrate DistilBERT
- [ ] Integrate Phi3
- [ ] Integrate GNN
- [ ] Create inference pipeline (TASK-031)

**Days 6-7: Containerization**
- [ ] Containerize essential crates
- [ ] Deploy to Cloudflare R2
- [ ] Deploy to GCP Cloud CDN
- [ ] Test CDN routing

---

## 8. Success Criteria

### Week 1 Success
- ✅ 26 essential crates identified (cognitive core preserved)
- ✅ Smart Crate Blake3 removed
- ✅ Smart Crate integrations connected
- ✅ Lightning QA → Statistical CDN bridge

### Week 2 Success
- ✅ Ops-main frontend connected to gateway
- ✅ Database connections working
- ✅ CDN health monitoring active
- ✅ Plasma state streaming to UI
- ✅ QA running on gateway/ops-main

### Week 3 Success
- ✅ Kali ISO Layer 2 execution working
- ✅ ML models integrated and inferencing
- ✅ All essential crates containerized
- ✅ CDN deployment successful
- ✅ Full ops-main platform operational

---

## 9. Risk Mitigation

### High Risk Items
1. **Blake3 Removal** - 47 files to update, risk of breaking changes
   - **Mitigation:** Comprehensive testing after each file update
   
2. **Gateway Bridge** - Complex WebSocket protocol integration
   - **Mitigation:** Incremental implementation, test each handler

3. **Kali ISO Layer 2** - eBPF/XDP complexity, kernel-level execution
   - **Mitigation:** Staged rollout, extensive testing in sandbox

4. **ML Model Integration** - Model loading, inference latency
   - **Mitigation:** Lazy loading, caching, async inference

### Medium Risk Items
1. **Smart Crate Integration** - Multiple systems to connect
   - **Mitigation:** One integration at a time, test thoroughly

2. **Containerization** - Deployment complexity
   - **Mitigation:** Use existing Terraform spec, incremental deployment

---

## 10. Key Observations

### What's Working Well
- ✅ 30 canonical crates defined (good reduction from 296)
- ✅ Smart Crate System architecture solid
- ✅ Lightning QA engine exists and functional
- ✅ Ops-main-platform frontend complete
- ✅ Kali ISO scaffolded and ready
- ✅ CDN Terraform spec ready for deployment

### What Needs Immediate Attention
- 🔴 Blake3 contamination in Smart Crate System
- 🔴 Missing Smart Crate integrations (Health Dashboard, Lightning QA, ATLAS, PLASMA)
- 🔴 Ops-main frontend disconnected from backend
- 🔴 Kali ISO Layer 2 execution incomplete
- 🔴 ML models not integrated into gateway

### Strategic Recommendations
1. **Minimal Startup Architecture** - Start with 12 core crates, escalate through NATS + DSL playbooks
2. **Parallel Tracks** - Work on 14 escalation components in parallel while bringing up gateway
3. **Consolidated Services** - Create `ctas7-unified-agent-registry` and `ctas7-intel-gateway` for consolidation
4. **DSL Playbook Engine** - Integrate into `ctas7-orchestrator` or create `ctas7-dsl-engine`
5. **NATS Escalation Router** - Enhance `ctas7-neural-mux` with escalation routing or create `ctas7-escalation-router`
6. **Gateway Bridge is Critical Path** - Ops-main can't work without it
7. **Fix Smart Crate First** - It's the foundation for everything else
8. **QA Hardening Enables Confidence** - Run QA hard from day 1
9. **Containerization Last** - Get everything working first, then containerize

---

**Next Steps:** Begin Week 1 execution plan, starting with essential crates identification and Smart Crate Blake3 removal.

