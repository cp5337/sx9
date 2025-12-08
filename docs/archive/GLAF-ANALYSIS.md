# GLAF System Analysis

## What GLAF Actually Is

**GLAF = Graph-Lattice Allocation Framework**

It's NOT just a graph viewer - it's a complete **threat correlation and analysis system** with:

1. **Matroid-based slot allocation** (RFC-9023)
2. **Hawkes process intensity** for event prediction
3. **Trivariate hash correlation** (SCH + CUID + UUID)
4. **Unicode operation encoding** (U+E000-U+F8FF)
5. **GNN-powered pattern discovery**
6. **41 pre-built attack scenarios** (APT campaigns, DHS scenarios)

---

## Existing Components (Already Built!)

### 1. Rust Backend: `ctas7-glaf-graph-server`
**Port: 18050**

```rust
// Key endpoints
GET  /health
GET  /api/graph/nodes?limit=100&hd4_phase=Hunt&label=Agent
GET  /api/graph/relationships
POST /api/query                    // SurrealQL
POST /api/glaf/matroid-rank        // Matroid independence
POST /api/glaf/hawkes-intensity    // Event prediction
POST /api/glaf/convergence         // H1/H2 convergence
WS   /ws/stream                    // Real-time updates
```

**Key structs:**
```rust
struct GlafNode {
    id: String,
    labels: Vec<String>,
    properties: Value,
    _glaf: GlafNodeMeta {
        triv_hash: Option<String>,
        hd4_phase: Option<String>,
        teth_entropy: Option<f64>,
        matroid_rank: Option<f64>,
    }
}
```

### 2. TypeScript Client: `glafClient.ts`
**Connects to: localhost:18019 (SurrealDB)**

```typescript
class GlafClient {
  async connect(): Promise<void>
  async query(surql: string): Promise<GlafQueryResult>
  async queryForViz(surql: string): Promise<any>  // Neo4j format
  async getThreatActors(limit: number): Promise<GlafQueryResult>
  async getOsintTools(category?: string): Promise<GlafQueryResult>
  async getScenarios(hd4Phase?: string): Promise<GlafQueryResult>
  async traverseWithEntropy(startNode: string, maxDepth: number): Promise<GlafQueryResult>
  async calculateMatroidRank(nodeIds: string[]): Promise<number>
  async getHawkesIntensity(eventType: string, windowHours: number): Promise<number>
}
```

### 3. React Viewer: `GLAFGraphViewer.tsx`
**1234 lines of working code!**

Features:
- SVG-based graph rendering
- Node types: `hash`, `agent`, `service`, `data`, `voice`, `infrastructure`, `user`
- Edge types: `data`, `control`, `dependency`, `communication`, `hash`, `voice`
- Layer system (Infrastructure, Processing, Interface, Data)
- Real-time performance monitoring
- Zoom/pan controls
- Voice feedback integration
- Export to JSON

### 4. Threat Correlation Engine: `glaf_correlation.rs`

```rust
pub struct GLAFCorrelationEngine {
    glaf_client: GLAFClient,
    hash_correlator: HashCorrelator,
    unicode_correlator: UnicodeCorrelator,
    pattern_discovery: PatternDiscoveryEngine,
    interdiction_analyzer: InterdictionPointAnalyzer,
}

impl GLAFCorrelationEngine {
    // Correlate threats using hash and Unicode
    async fn correlate_threats(&self, threats: &[RecognizedThreat]) -> Result<CorrelationResult>
    
    // Find interdiction points (earlier = better)
    async fn find_interdiction_points(&self, technique: &ATTACKTechnique) -> Result<Vec<InterdictionPoint>>
}
```

### 5. CDN Data Fabric: `ctas7-cdn-data-fabric`
**Port: 18100**

Central hub that:
- Registers all databases
- Routes queries to appropriate DB
- Transforms formats (graph ↔ table ↔ geojson ↔ cypher ↔ sql)
- Pushes live updates

---

## Attack Scenario Database

**41 pre-built GLAF configs** including:

| Category | Scenarios |
|----------|-----------|
| **APT Campaigns** | Volt Typhoon, Salt Typhoon, APT29, Chimera |
| **Ransomware** | WannaCry, NotPetya |
| **Terrorism** | 9/11, Mumbai 2008, Paris 2015, London 7/7 |
| **DHS Scenarios** | Nuclear, Biological, Chemical, Radiological |
| **Cyber Attacks** | OPM Breach, Sony Pictures, Bangladesh Bank |

Each scenario has:
- Technique mappings (MITRE ATT&CK)
- HD4 phase recommendations
- TETH entropy calculations
- Trivariate hash assignments

---

## Port Allocation (GLAF-specific)

| Port | Service | Status |
|------|---------|--------|
| 18018 | GLAF UI | Planned |
| 18019 | GLAF Core (SurrealDB) | ✅ Ready |
| 18025 | GLAF Analytics (SurrealDB) | ✅ Ready |
| 18050 | GLAF Graph Server (Rust) | ✅ Built |
| 18051 | GLAF MCP Server | Planned |
| 18052 | GLAF Visualizer API | Planned |
| 18100 | CDN Data Fabric | ✅ Built |

---

## What's Missing for Bolt Integration

### 1. Visual Query Builder
The existing `GLAFGraphViewer.tsx` has graph rendering but no query builder.

**Need to add:**
```typescript
interface VisualQuery {
  find: 'nodes' | 'relationships' | 'paths';
  startConditions: Condition[];
  hops: Hop[];
  options: { limit: number; orderBy?: string; };
}

// Generate SurrealQL from visual query
function generateSurrealQL(query: VisualQuery): string
```

### 2. Multi-Database Switcher
Current client only connects to one SurrealDB instance.

**Need:**
```typescript
interface DatabaseConnection {
  id: string;
  type: 'surrealdb' | 'supabase' | 'sled' | 'sledis';
  host: string;
  port: number;
}

// Switch between databases
function switchDatabase(connectionId: string): Promise<void>
```

### 3. Scenario Loader
41 YAML configs exist but no UI to load them.

**Need:**
```typescript
// Load scenario into graph
async function loadScenario(scenarioId: string): Promise<void>

// Get all available scenarios
async function listScenarios(): Promise<ScenarioInfo[]>
```

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           GLAF SYSTEM ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    FRONTEND (React + D3/SVG)                         │   │
│  │                                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │ Visual Query │  │ Graph Canvas │  │   Scenario   │              │   │
│  │  │   Builder    │  │ (D3 Force)   │  │    Loader    │              │   │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │   │
│  │         │                 │                 │                       │   │
│  │  ┌──────┴─────────────────┴─────────────────┴──────┐               │   │
│  │  │              glafClient.ts (TypeScript)          │               │   │
│  │  └──────────────────────┬───────────────────────────┘               │   │
│  └─────────────────────────┼───────────────────────────────────────────┘   │
│                            │                                                │
│  ┌─────────────────────────┼───────────────────────────────────────────┐   │
│  │                         ▼                                            │   │
│  │  ┌──────────────────────────────────────────────────────────────┐  │   │
│  │  │                 CDN Data Fabric (:18100)                      │  │   │
│  │  │                                                               │  │   │
│  │  │  • Database Registry    • Query Router                       │  │   │
│  │  │  • Schema Cache         • Format Transformer                 │  │   │
│  │  │  • Live Subscriptions   • Health Monitoring                  │  │   │
│  │  └───────────────────────────┬───────────────────────────────────┘  │   │
│  │                              │                                       │   │
│  │       ┌──────────────────────┼──────────────────────┐              │   │
│  │       │                      │                      │              │   │
│  │       ▼                      ▼                      ▼              │   │
│  │  ┌─────────────┐      ┌─────────────┐      ┌─────────────┐        │   │
│  │  │ GLAF Core   │      │ GLAF Graph  │      │ Foundation  │        │   │
│  │  │ (SurrealDB) │      │   Server    │      │   Daemon    │        │   │
│  │  │   :18019    │      │   :18050    │      │   (Rust)    │        │   │
│  │  │             │      │             │      │             │        │   │
│  │  │ • Scenarios │      │ • Matroid   │      │ • Threat    │        │   │
│  │  │ • Actors    │      │ • Hawkes    │      │   Reaction  │        │   │
│  │  │ • Techniques│      │ • Converge  │      │ • Pattern   │        │   │
│  │  └─────────────┘      └─────────────┘      │   Discovery │        │   │
│  │                                            └─────────────┘        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         OTHER DATABASES                              │   │
│  │                                                                      │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ Supabase │  │   Sled   │  │  Sledis  │  │  Redis   │            │   │
│  │  │  :18000  │  │  :18400  │  │  :18401  │  │  :18030  │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Bolt Integration Strategy

### Option A: Enhance Existing GLAFGraphViewer
- Add visual query builder panel
- Add database switcher
- Add scenario loader
- Connect to existing Rust backend

### Option B: Build New with Existing Backend
- Use D3.js for graph (like existing)
- Add React Flow for workflow canvas
- Connect to `glaf-graph-server:18050` for math
- Connect to `cdn-data-fabric:18100` for DB routing

### Option C: Fork and Extend
- Fork the existing `GLAFGraphViewer.tsx` (1234 lines)
- Add missing features
- Keep the proven graph rendering

---

## Key Files to Reference

| File | Lines | Purpose |
|------|-------|---------|
| `ctas7-glaf-graph-server/src/main.rs` | 370 | Rust backend with math endpoints |
| `ctas7-glaf-browser/src/shared/services/glaf/glafClient.ts` | 145 | TypeScript client |
| `ctas7-ui-components-tactical/src/components/GLAFGraphViewer.tsx` | 1234 | React viewer |
| `ctas7-foundation-daemon/src/threat_reaction/glaf_correlation.rs` | 283 | Threat correlation |
| `ctas7-glaf-graph-server/GLAF_UNIVERSAL_VISUALIZER.md` | 368 | Architecture doc |
| `ctas7-scenarios-database/glaf_configs/glaf_master_config.yaml` | 53 | 41 scenarios |

---

## Recommended Next Steps

1. **Start the existing GLAF stack:**
   ```bash
   docker-compose -f docker-compose.glaf-visualizer.yml up
   ```

2. **Test the Rust backend:**
   ```bash
   curl http://localhost:18050/health
   curl http://localhost:18050/api/graph/nodes?limit=10
   ```

3. **Load a scenario:**
   ```bash
   curl -X POST http://localhost:18050/api/query \
     -H "Content-Type: application/json" \
     -d '{"surql": "SELECT * FROM attack_scenarios WHERE id = \"volt_typhoon_apt\""}'
   ```

4. **Give Bolt the existing code as reference:**
   - Copy `GLAFGraphViewer.tsx` structure
   - Add visual query builder
   - Connect to existing backends

---

## Summary

**You already have 80% of GLAF built.** The Rust backend, TypeScript client, React viewer, and 41 attack scenarios are all there. What's missing:

| Component | Status | Effort |
|-----------|--------|--------|
| Graph rendering | ✅ Built | - |
| Rust math engine | ✅ Built | - |
| SurrealDB integration | ✅ Built | - |
| 41 scenarios | ✅ Built | - |
| Visual query builder | ❌ Missing | Medium |
| Multi-DB switcher | ❌ Missing | Low |
| Scenario loader UI | ❌ Missing | Low |
| Forge workflow canvas | ❌ Missing | Medium |

**Total effort to complete: ~2-3 days of focused work.**




