# RFC GAP ANALYSIS — December 2025

**Purpose:** Comprehensive gap analysis for RFC alignment with current architecture  
**Date:** December 16, 2025  
**Scope:** Database changes, Intel system integration, Pending RFC updates  

---

## EXECUTIVE SUMMARY

### Critical Changes Required

| Change | Impact | Priority |
|--------|--------|----------|
| **Remove SurrealDB references** | 35+ files affected | HIGH |
| **Remove Neo4j references** | 35+ files affected | HIGH |
| **Update Supabase role** | Primary OLTP + expanded analytics | HIGH |
| **Define Neon role** | ACID SQL for transactional workloads | HIGH |
| **Intel System RFC** | No RFC exists for Leptose/EEI/OSINT | CRITICAL |
| **HashRef 16-byte update** | RFC-9001 needs major revision | HIGH |
| **32 Primitives at U+E500** | RFC-9002 needs major revision | HIGH |
| **Delta angle normalization** | RFC-9100 needs update | MEDIUM |

---

## 1. DATABASE ARCHITECTURE GAPS

### 1.1 Current RFC-9005 States (INCORRECT)

```
RFC-9005 v1.2 Current Architecture:
┌─────────────────────────────────────────────────────────────────┐
│  Primary: PostgreSQL (Supabase)                                 │
│  Secondary: Sledis (KV cache), SurrealDB (graph queries) ← WRONG│
│  Waterfall: KV → R2 → Supabase → Neon → Neo4j ← WRONG          │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Correct Architecture (TO BE)

```
CORRECT Architecture:
┌─────────────────────────────────────────────────────────────────┐
│  SUPABASE (Primary)                                             │
│  ├─ PostgreSQL: OLTP, entities, relationships                   │
│  ├─ Realtime: Subscriptions, presence                           │
│  ├─ Storage: Blobs, documents                                   │
│  └─ Edge Functions: Serverless compute                          │
│                                                                 │
│  NEON (ACID SQL)                                                │
│  ├─ Transactional workloads requiring strict ACID               │
│  ├─ Analytics queries (branching for dev/test)                  │
│  └─ Serverless scale-to-zero for cost efficiency                │
│                                                                 │
│  SLED/SLEDIS (Local Cache)                                      │
│  ├─ High-speed KV cache                                         │
│  ├─ Embedded graph operations (petgraph)                        │
│  └─ TTL-based ephemeral storage                                 │
│                                                                 │
│  CHROMADB (Vector Store)                                        │
│  ├─ Threat intelligence embeddings                              │
│  ├─ OSINT vectorization                                         │
│  └─ Semantic search for EEI resolution                          │
│                                                                 │
│  CLOUDFLARE R2 (CDN)                                            │
│  ├─ Zero-egress threat intel distribution                       │
│  ├─ KV edge cache                                               │
│  └─ Workers for processing                                      │
└─────────────────────────────────────────────────────────────────┘

Waterfall (CORRECTED):
  KV (Edge) → R2 (Bulk) → Supabase (Primary) → Neon (ACID)
  
  NO Neo4j, NO SurrealDB
```

### 1.3 Files Requiring Database Reference Updates

**SurrealDB References (REMOVE):**
```
01-rfc/9000-core/RFC-9005-Unified-Schema.md
01-rfc/9100-integration/RFC-9114-SX9-Gateway-Neural-Retrofit.md
01-rfc/9100-integration/RFC-9105-SPIRES-Extraction.md
01-rfc/shuttle_folder/tasks/03-frontend-integration/*
01-rfc/rfc_alignment_temp/*
01-rfc/_archive/etl-prep-docs/*
```

**Neo4j References (REMOVE):**
```
01-rfc/shuttle_folder/tasks/02-threat-intel-pipeline/sync_neon_to_neo4j.js
01-rfc/shuttle_folder/tasks/02-threat-intel-pipeline/quick_deploy.sh
01-rfc/9000-core/RFC-9304-GLAF-Graph-Engine-Specification.md (comparison table)
01-rfc/9100-integration/RFC-9109-Plasma-Defender.md
01-rfc/9300-cognitive/RFC-9025-Node-Interview-Schema.md
```

### 1.4 RFC-9005 Required Changes

```diff
## 2.1 Single Source of Truth

- **Secondary Systems (Derived/Cached):**
- - Sledis: High-speed key-value cache
- - SurrealDB: Graph query acceleration (read replicas)

+ **Secondary Systems:**
+ - Sled/Sledis: High-speed embedded KV cache
+ - ChromaDB: Vector embeddings for semantic search
+ - Neon: ACID SQL for transactional workloads

## 10.3 Waterfall Data Access Pattern

- 1. Cloudflare KV (Edge)        →  <10ms   (80%+ hit rate target)
- 2. Cloudflare R2 (Bulk)        →  <50ms   (15% of requests)
- 3. Supabase (Primary DB)       →  100-200ms (4% of requests)
- 4. Neon (PostgreSQL)           →  200-300ms (0.9% of requests)
- 5. Neo4j (Graph DB)            →  300-500ms (0.1% of requests)

+ 1. Cloudflare KV (Edge)        →  <10ms   (80%+ hit rate target)
+ 2. Cloudflare R2 (Bulk)        →  <50ms   (15% of requests)
+ 3. Supabase (Primary OLTP)     →  50-100ms (4% of requests)
+ 4. Neon (ACID Transactions)    →  100-200ms (1% of requests)
+ 
+ Graph queries handled by embedded GLAF (sled + petgraph), NOT external DB
```

---

## 2. RFC-9304 GLAF SPECIFICATION GAPS

### 2.1 Current Spec Issues

| Issue | Current | Should Be |
|-------|---------|-----------|
| Neo4j comparison | References Neo4j as alternative | Remove - GLAF is THE graph solution |
| SurrealDB mention | Listed as option | Remove entirely |
| Storage backend | sled only | sled + petgraph (already correct) |
| ChromaDB integration | Not mentioned | Add for vector search |

### 2.2 Required Changes to RFC-9304

```diff
## 11. Comparison Matrix

- | Feature            | Neo4j      | Dgraph   | TigerGraph | **GLAF**        |
- | ------------------ | ---------- | -------- | ---------- | --------------- |
- | Deployment         | Server     | Server   | Server     | **Embedded**    |
- ...

+ ## 11. Why GLAF (No External Graph DB)
+ 
+ GLAF eliminates the need for external graph databases:
+ 
+ | Requirement | Traditional (Neo4j/SurrealDB) | GLAF Solution |
+ |-------------|-------------------------------|---------------|
+ | Graph queries | External server, network hop | Embedded, <1ms |
+ | Vector search | Separate system | ChromaDB integration |
+ | ACID | Varies | Sled transactions |
+ | Offline | Requires connectivity | Fully embedded |
+ | Cold start | 10-30 seconds | 50ms |
```

---

## 3. INTEL SYSTEM INTEGRATION GAPS

### 3.1 Missing RFC: Intelligence Orchestration

**CRITICAL GAP:** No RFC exists for the unified intelligence system we just built:

- `sx9-leptose` (Knowledge Engine)
- `sx9-glaf-core` (Neural Graph Operations)
- ChromaDB integration
- EEI System
- OSINT Machine integration

**Recommendation:** Create **RFC-9118: Unified Intelligence Orchestration**

### 3.2 Proposed RFC-9118 Scope

```
RFC-9118: Unified Intelligence Orchestration
├── Leptose Knowledge Engine
│   ├── NATS Bridge (osint.intel, eei.query, eei.response)
│   ├── ChromaDB Client (Python subprocess integration)
│   └── Knowledge Graph (petgraph + GLAF)
├── EEI System
│   ├── Time-of-Value decay curves
│   ├── Sliding window management
│   └── Toolchain satisfaction matrix
├── OSINT Integration
│   ├── Master Orchestrator (12+ sources)
│   ├── GNN OSINT Model
│   └── Enhanced OSINT (VT, Shodan, OTX, etc.)
├── ChromaDB Vector Store
│   ├── Collections: tools, ctas_tasks, ptcc_configs, tool_chains
│   ├── Dual trivariate hash extraction
│   └── Unicode operation mapping
└── graph-db Analytical UI
    ├── Legion Hot Path (<1µs queries)
    ├── Workflow Builder (LangGraph/n8n-like)
    └── TTL-based retrieval (not storage)
```

### 3.3 Unicode Allocation for Intel (RFC-9002 Extension)

Current RFC-9005 allocates U+EC00-EFFF for "EEI & Intelligence Patterns" but this is not detailed in RFC-9002.

**Required Addition to RFC-9002:**

```
## Class I: Intelligence Operations (U+EC00-EFFF)

| Range | Purpose | Count |
|-------|---------|-------|
| U+EC00-EC7F | EEI Primitives | 128 |
| U+EC80-ECFF | OSINT Source Types | 128 |
| U+ED00-ED7F | Intelligence Confidence Levels | 128 |
| U+ED80-EDFF | Time-of-Value Markers | 128 |
| U+EE00-EEFF | Threat Classification | 256 |
| U+EF00-EFFF | Reserved | 256 |
```

---

## 4. PENDING RFC UPDATES (from shuttle_folder/tasks/04-rfc-updates)

### 4.1 RFC_ALIGNMENT_ANALYSIS.md Summary

| RFC | Status | Changes Required |
|-----|--------|------------------|
| RFC-9001 | NEEDS UPDATE | HashRef 16-byte, Lisp heredity operators |
| RFC-9002 | NEEDS MAJOR UPDATE | Class T (triggers), Class P (priority), Class S (system), Class H (heredity) |
| RFC-9003 | NEEDS UPDATE | Unicode mapping for 32 primitives |
| RFC-9004 | MINOR UPDATES | HashRef-based routing |

### 4.2 RFC_9016_DELTA_ANGLE_FIX.md Summary

| Change | Old | New |
|--------|-----|-----|
| Format | Degrees (0-180°) | Normalized (0.0-1.0) |
| Precision | f32 | f64 with 6 decimal places |
| Thresholds | <2°, 2-10°, 10-25°, 25-60°, >60° | <0.011111, 0.011-0.056, 0.056-0.139, 0.139-0.333, >0.333 |

### 4.3 32 Primitives Architecture (NOT YET IN RFCs)

```
CATEGORY 0: GRAPH (U+E500-E503)
  00: TRAVERSE, 01: SEARCH, 02: AGGREGATE, 03: TRANSFORM

CATEGORY 1: MATROID (U+E504-E507)
  04: RANK, 05: CLOSURE, 06: CIRCUIT, 07: SPAN

CATEGORY 2: CONVERGENCE (U+E508-E50B)
  08: DETECT, 09: MEASURE, 10: THRESHOLD, 11: STABILIZE

CATEGORY 3: HASH (U+E50C-E50F)
  12: COMPUTE, 13: VERIFY, 14: CHAIN, 15: DERIVE

CATEGORY 4: TICK (U+E510-E513)
  16: SYNC, 17: QUERY, 18: ADVANCE, 19: RESET

CATEGORY 5: SDT/GATE (U+E514-E517)
  20: TRIGGER, 21: RELEASE, 22: QUERY, 23: CONFIGURE

CATEGORY 6: PLASMA (U+E518-E51B)
  24: EXCITE, 25: DAMPEN, 26: QUERY, 27: COUPLE

CATEGORY 7: CONTROL (U+E51C-E51F)
  28: PING, 29: STATS, 30: SHUTDOWN, 31: EMERGENCY
```

---

## 5. EFFECTIVENESS TEST PLAN

### 5.1 Intelligence System Validation

```
┌─────────────────────────────────────────────────────────────────┐
│                  INTEL SYSTEM TEST MATRIX                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  TEST 1: OSINT → ChromaDB → EEI Resolution                      │
│  ├─ Inject OSINT message via NATS (osint.intel)                 │
│  ├─ Verify ChromaDB vectorization                               │
│  ├─ Query EEI (eei.query)                                       │
│  └─ Validate response relevance                                 │
│                                                                 │
│  TEST 2: PTCC → ATT&CK → EEI Generation                         │
│  ├─ Load PTCC primitive (e.g., CONNECT)                         │
│  ├─ Map to ATT&CK techniques                                    │
│  ├─ Auto-generate EEIs                                          │
│  └─ Verify TETH entropy thresholds                              │
│                                                                 │
│  TEST 3: Tool Execution → OSSEC → Correlation                   │
│  ├─ Execute Kali tool via L2 trigger                            │
│  ├─ Capture tool output (Class R block)                         │
│  ├─ Monitor OSSEC alerts                                        │
│  └─ Correlate with scenario expectations                        │
│                                                                 │
│  TEST 4: Knowledge Graph → TETH → Persona Assignment            │
│  ├─ Build knowledge graph from intel                            │
│  ├─ Calculate TETH entropy                                      │
│  ├─ Assign appropriate persona tier                             │
│  └─ Verify persona tool proficiencies                           │
│                                                                 │
│  TEST 5: End-to-End Scenario (APT29)                            │
│  ├─ Load APT29 scenario                                         │
│  ├─ Execute through HD4 phases                                  │
│  ├─ Monitor all subsystems                                      │
│  └─ Calculate detection rate, false positives                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Database Architecture Validation

```bash
# Test 1: Supabase Primary OLTP
curl -X POST "https://<project>.supabase.co/rest/v1/entities" \
  -H "Authorization: Bearer $SUPABASE_KEY" \
  -d '{"name": "test", "entity_type": "component"}'

# Test 2: Neon ACID Transaction
psql $NEON_URL -c "BEGIN; INSERT INTO entities...; COMMIT;"

# Test 3: ChromaDB Vector Query
python3 -c "
import chromadb
client = chromadb.Client()
collection = client.get_collection('tools')
results = collection.query(query_texts=['port scan'], n_results=5)
print(results)
"

# Test 4: GLAF Embedded Query (<1ms)
cargo test -p sx9-glaf-core -- --test glaf_query_latency

# Test 5: Waterfall Fallback
# Simulate KV miss → R2 miss → Supabase hit
```

### 5.3 Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| OSINT → EEI latency | <500ms | NATS request-reply timing |
| ChromaDB query | <100ms | Vector search benchmark |
| GLAF graph query | <1ms | Embedded benchmark |
| PTCC → ATT&CK mapping | 100% coverage | Technique count validation |
| TETH entropy accuracy | ±0.1 bits | Statistical validation |
| Detection rate (APT29) | >85% | Scenario execution |
| False positive rate | <15% | OSSEC correlation |

---

## 6. ACTION ITEMS

### 6.1 Immediate (This Session)

- [ ] Update RFC-9005 §2.1 - Remove SurrealDB, clarify Supabase/Neon roles
- [ ] Update RFC-9005 §10.3 - Remove Neo4j from waterfall
- [ ] Update RFC-9304 §11 - Remove Neo4j comparison table

### 6.2 Short-Term (Next Session)

- [ ] Apply RFC-9001 HashRef updates (16-byte, Lisp heredity)
- [ ] Apply RFC-9002 Unicode class updates (T, P, S, H)
- [ ] Apply RFC-9100 delta angle normalization
- [ ] Create RFC-9118 Intelligence Orchestration draft

### 6.3 Medium-Term (Testing Phase)

- [ ] Execute Test 1-5 from effectiveness matrix
- [ ] Document results in test report
- [ ] Iterate on TETH thresholds based on results
- [ ] Validate PTCC → ATT&CK mappings

---

## 7. REVISION HISTORY

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-16 | Initial gap analysis |

---

## 8. PLASMA-DEFENDER STATUS & TWEAKS NEEDED

### 8.1 Current Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| `PlasmaDefender` struct | ✅ Working | Core orchestrator functional |
| `PlasmaDefenderServer` | ✅ Working | Axum endpoints (/health, /metrics, /sdt/state, /crystal/resonance) |
| `PlasmaBus` | ✅ Working | NATS telemetry publishing |
| `CrystalIntegration` | ✅ Working | Ring strength evaluation |
| `SdtIntegration` | ✅ Working | Gate control, resonance |
| `AnnDaemon` | ✅ Working | ANN advisory generation |
| `ThreatMonitor` | ⚠️ Partial | Loop works, but agents are stubs |
| `ThreatAgent` | ❌ Stub | All agent methods return `Ok(None)` |
| `tool_handler` | ⚠️ Partial | Subscribes to NATS, but simplified processing |
| OSSEC Integration | ❌ Missing | Not implemented |
| SlotGraph Integration | ❌ Missing | Not connected to 164-task graph |
| EEI Correlation | ❌ Missing | Not connected to intel system |

### 8.2 Agent Implementation Gaps

The `agents.rs` file has 4 agent types that are all stubs:

```rust
// ALL RETURN Ok(None) - NEED IMPLEMENTATION
async fn monitor_network(&self) -> anyhow::Result<Option<ThreatEvent>>
async fn hunt_threats(&self) -> anyhow::Result<Option<ThreatEvent>>
async fn watch_canaries(&self) -> anyhow::Result<Option<ThreatEvent>>
async fn detect_anomalies(&self) -> anyhow::Result<Option<ThreatEvent>>
```

### 8.3 OSSEC Integration Requirements

Per RFC-9109, OSSEC should:
1. Parse minimal OSSEC alerts (not full Wazuh complexity)
2. Feed alerts into threat agents
3. Correlate with tool execution results
4. Generate EEI queries based on detected threats

**Proposed OSSEC Agent:**

```rust
pub struct OssecAgent {
    alert_path: PathBuf,       // /var/ossec/logs/alerts/alerts.json
    rule_path: PathBuf,        // /var/ossec/rules/
    last_position: u64,        // File position for incremental reads
    plasma: Arc<PlasmaState>,
}

impl OssecAgent {
    pub async fn poll_alerts(&mut self) -> anyhow::Result<Vec<OssecAlert>> {
        // Read new alerts since last_position
        // Parse JSON alerts
        // Map to ThreatEvent
    }
    
    pub fn map_to_mitre(&self, rule_id: u32) -> Option<String> {
        // Map OSSEC rule ID to MITRE technique
        // e.g., 5710 → T1110 (Brute Force)
    }
}
```

### 8.4 Tool Output → OSSEC → Correlation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    MISSING INTEGRATION                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Kali Tool Execution (L2 trigger)                               │
│       │                                                         │
│       ▼                                                         │
│  sx9.tool.result.ann (NATS) ──────────────────────────────────► │
│       │                                    ▲                    │
│       ▼                                    │                    │
│  tool_handler.rs (PARTIAL)                 │                    │
│       │                                    │                    │
│       ▼                                    │                    │
│  ANN Advisory ─────────────────────────────┘                    │
│                                                                 │
│  ════════════════════════════════════════════════════════════   │
│                     WHAT'S MISSING:                             │
│  ════════════════════════════════════════════════════════════   │
│                                                                 │
│  1. OSSEC alert parsing (OssecAgent)                            │
│  2. MITRE technique mapping                                     │
│  3. SlotGraph task correlation                                  │
│  4. EEI query generation                                        │
│  5. Leptose knowledge graph updates                             │
│  6. TETH entropy recalculation                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 8.5 Recommended Tweaks (Priority Order)

1. **Implement OssecAgent** - Parse /var/ossec/logs/alerts/alerts.json
2. **Add MITRE mapping** - Rule ID → Technique lookup table
3. **Connect to SlotGraph** - Map alerts to 164 tasks
4. **Implement agent methods** - Replace stubs with actual logic
5. **Add EEI bridge** - Publish to `eei.query` on threat detection
6. **Connect to Leptose** - Update knowledge graph with threat intel

### 8.6 Test Plan (Once Tweaks Complete)

**Pre-requisite:** Plasma-Defender agents implemented

| Test | Input | Expected Output |
|------|-------|-----------------|
| OSSEC Alert Parse | Sample alert JSON | `ThreatEvent` with MITRE mapping |
| Agent Detection | Simulated network anomaly | `ThreatEvent` severity classification |
| SDT Gate | High-severity threat | Gate BLOCKED state |
| EEI Generation | Detected threat | EEI query published to NATS |
| Correlation | Tool output + OSSEC alert | Matched scenario expectation |

---

**Status:** DRAFT - Awaiting review and implementation

