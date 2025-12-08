# SX9 MASTER PLAN

> Generated: December 4, 2025
> Status: Bar Napkin → Engineering Spec
> Context: Complete session documentation

---

## RELATED DOCUMENTS

| Document | Purpose |
|----------|---------|
| `BOLT-SPEC-FINAL.md` | **Bolt prompt for Data Analytics Workbench** |
| `RFC-9200-DATA-ANALYTICS-WORKBENCH.md` | **RFC for DAW v0.1** |
| `PORTS-CDN-CONDA-SPEC.md` | Port allocation, CDN, Conda integration |
| `BOLT-FORGE-WORKBENCH-PROMPT.md` | Extended Bolt prompt with all tabs |
| `CDN-TERRAFORM-SPEC.md` | Terraform for CDN infrastructure |
| `SDT-PROTOCOL-SPEC.md` | Layer 2 SDT protocol |
| `DOMAIN-INVENTORY.md` | Domain allocation |

---

## TABLE OF CONTENTS

1. [Domain Inventory](#1-domain-inventory)
2. [Repository Inventory](#2-repository-inventory)
3. [Architecture Overview](#3-architecture-overview)
4. [Data Analytics Workbench](#4-data-analytics-workbench)
5. [Cesium/GIS System](#5-cesiumgis-system)
6. [CDN Strategy](#6-cdn-strategy)
7. [Kali Plasma](#7-kali-plasma)
8. [Software-Defined Thyristor (SDT)](#8-software-defined-thyristor-sdt)
9. [Ephemeral Systems](#9-ephemeral-systems)
10. [Biometric + Quantum Build](#10-biometric--quantum-build)
11. [iOS Native Strategy](#11-ios-native-strategy)
12. [ECS Strategy (ATLAS + apecs)](#12-ecs-strategy-atlas--apecs)
13. [Daemon Upgrade Plan](#13-daemon-upgrade-plan) ← NEW
14. [Immediate Action Items](#14-immediate-action-items)
15. [Component Locations](#15-component-locations)

---

## 1. DOMAIN INVENTORY

### Production Domains (Hostinger)
| Domain | Purpose | Status |
|--------|---------|--------|
| `sx9.io` | Primary SX9 brand | ✓ Owned |
| `synaptix9.com` | Enterprise/Partner facing | ✓ Owned |
| `synaptix9.io` | Alternate | ✓ Owned (verify) |

### Development Domains
| Domain | Purpose | Status |
|--------|---------|--------|
| `devstackone.dev` | Dev/staging CDN | ✓ Owned |

### Obfuscated Domains (Ops)
| Domain | Registrar | Purpose |
|--------|-----------|---------|
| `dityauto.???` | GoDaddy | Dark ops cover |
| `autorepair-tech.net` | GoDaddy | Dark ops cover |
| `autorepair-tech.info` | GoDaddy | Dark ops cover (verify) |

### Recommended Domain Allocation
```
PRODUCTION:
  sx9.io                    → Main portal
  cdn.sx9.io                → Public CDN (Cloudflare R2)
  api.sx9.io                → API gateway
  
PARTNER:
  synaptix9.com             → Partner CTAS Main portal
  crates.synaptix9.com      → Foundation crate registry
  
DEV/STAGING:
  devstackone.dev           → Development CDN
  staging.devstackone.dev   → Staging environment
  
OPS (Obfuscated):
  dityauto.???              → Kali Plasma updates
  autorepair-tech.net       → Dark ops tunnel endpoint
```

---

## 2. REPOSITORY INVENTORY

### Canonical (DO NOT TOUCH)
| Repo | Status | Port | Notes |
|------|--------|------|-------|
| `ctas7-command-center-canonical` | ✅ Working | 25175 | THE canonical UI |

### Development (Safe to Experiment)
| Repo | Status | Notes |
|------|--------|-------|
| `sx9-development-center` | Copy of canonical | Experiment here |

### Shipyard Staging (174 directories!)
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `UniversalTopologyDesigner.tsx` | `ctas-7-shipyard-staging/ctas7-command-center/src/components/SYNAPTIX9/` | 2,372 | Has nonagon + Forge |
| `ForgeIntegrationService.ts` | `ctas-7-shipyard-staging/ctas7-command-center/src/services/` | 251 | Forge backend client |
| `ctas7-foundation-daemon` | `ctas-7-shipyard-staging/` | Large | DSL, GLAF, threat reaction |
| `ctas7-glaf-graph-server` | `ctas-7-shipyard-staging/` | Rust | GLAF server |
| `ctas7-glaf-clients` | `ctas-7-shipyard-staging/` | Rust | GLAF clients |
| `ctas7-exploit-arsenal` | `ctas-7-shipyard-staging/` | 32K | Offensive toolkit |
| `sx9-lisp` | `ctas-7-shipyard-staging/` | 1,276 | Lisp interpreter |
| `GLAFGraphViewer.tsx` | `ctas-7-shipyard-staging/ctas7-ui-components-tactical/` | - | Neo4j-style browser |

### Recycle (Recoverable)
| Component | Location | Notes |
|-----------|----------|-------|
| `EphemeralAssetOrchestrator.ts` | `ctas7-recycle/rcx-ctas7-broken-deprecated/src/services/` | KASM-style orchestration |
| `EphemeralToolchainManager.rs` | `ctas7-recycle/rcx-ctas7-broken-deprecated/crates/ctas-core-integration/` | Rust lifecycle mgmt |
| `ctas7-port-manager` | `ctas7-recycle/rcx-ctas7-broken-deprecated/crates/` | Port management |

### GIS/Cesium Projects
| Repo | Status | Notes |
|------|--------|-------|
| `ctas7-gis-cesium` | Has components | SpaceWorldDemo, KPIDrawers |
| `ctas-7-shipyard-staging/ctas7-cesium-beam-ui` | Has components | BeamPatternDashboard |
| `ctas-7-shipyard-staging/ctas7-cesium-geolocation` | Has components | RadiationBeltInfo |

### Other Key Repos
| Repo | Purpose |
|------|---------|
| `synaptix9-workflow-system` | This project - Forge backend works |
| `ctas7-retrograde` | Rust crates collection |
| `v0-satellite-financial-dashboard` | Financial + satellite UI |

---

## 3. ARCHITECTURE OVERVIEW

```
┌─────────────────────────────────────────────────────────────────────┐
│                         SX9 UNIFIED SYSTEM                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐     │
│  │  KALI PLASMA    │  │  CTAS MAIN      │  │  iPAD PLASMA    │     │
│  │  (Operator)     │  │  (Partner)      │  │  (Field)        │     │
│  │                 │  │                 │  │                 │     │
│  │ • eBPF/XDP      │  │ • Full HD4      │  │ • Native iOS    │     │
│  │ • L2 SDT        │  │ • 4 Worlds      │  │ • MapKit        │     │
│  │ • Invisible     │  │ • Adjudication  │  │ • Ground Truth  │     │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘     │
│           │                    │                    │               │
│           └────────────────────┼────────────────────┘               │
│                                │                                    │
│                    ┌───────────▼───────────┐                       │
│                    │      ATLAS DAEMON     │ ◀── NEW!              │
│                    │  (Legion ECS + SIMD)  │                       │
│                    │  • Tick sync (250ns)  │                       │
│                    │  • Matroid rank       │                       │
│                    │  • Graph traversal    │                       │
│                    │  • Batch hashing      │                       │
│                    └───────────┬───────────┘                       │
│                                │                                    │
│                    ┌───────────▼───────────┐                       │
│                    │     apecs World       │ ◀── Async I/O         │
│                    │  • DB queries         │                       │
│                    │  • WebSocket          │                       │
│                    │  • Change tracking    │                       │
│                    └───────────┬───────────┘                       │
│                                │                                    │
│                    ┌───────────▼───────────┐                       │
│                    │     TUNNEL / CDN      │                       │
│                    │  (BYOCDN Interface)   │                       │
│                    └───────────┬───────────┘                       │
│                                │                                    │
│         ┌──────────────────────┼──────────────────────┐            │
│         ▼                      ▼                      ▼            │
│  ┌─────────────┐       ┌─────────────┐       ┌─────────────┐      │
│  │ CLOUDFLARE  │       │    GCP      │       │  INTERNAL   │      │
│  │  R2 + CDN   │       │  Cloud CDN  │       │    CDN      │      │
│  │             │       │             │       │             │      │
│  │ • Crates    │       │ • IAM-gated │       │ • Classified│      │
│  │ • Geo tiles │       │ • Partners  │       │ • Dark ops  │      │
│  │ • $0 egress │       │ • $0 idle   │       │ • Air-gap   │      │
│  └─────────────┘       └─────────────┘       └─────────────┘      │
│                                │                                    │
│                    ┌───────────▼───────────┐                       │
│                    │    PORT FILTER        │                       │
│                    │  (C2 Blocking)        │                       │
│                    │  • Cobalt Strike      │                       │
│                    │  • Metasploit         │                       │
│                    │  • Sliver/Brute Ratel │                       │
│                    └───────────┬───────────┘                       │
│                                │                                    │
│                    ┌───────────▼───────────┐                       │
│                    │    PORT MANAGER       │                       │
│                    │    + DSL ROUTER       │                       │
│                    └───────────────────────┘                       │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. DATA ANALYTICS WORKBENCH (NEW!)

> **RFC-9200** defines the complete specification
> **BOLT-SPEC-FINAL.md** contains the prompt for Bolt

### Overview

The DAW is a unified platform combining:
- **GLAF Graph Browser** - Better than Neo4j (visual query builder!)
- **Forge Canvas** - n8n-style workflow automation
- **Database Studio** - Supabase-style views for ALL databases
- **Model Viewer** - GNN/ANN architecture visualization
- **Vector Search** - Embedding search with t-SNE viz

### Key Differentiator: Visual Query Builder

Instead of memorizing Cypher:
```
MATCH (n:Agent {status: 'ACTIVE'})-[r:ALLOCATED_TO]->(s:Slot)
WHERE n.priority > 2
RETURN n, r, s
```

Users build visually:
```
┌─ Query Builder ─────────────────────────────────────────┐
│ FIND [Nodes ▼] WHERE                                    │
│   [label    ▼] [equals      ▼] [Agent    ]             │
│   [status   ▼] [equals      ▼] [ACTIVE   ]             │
│   [priority ▼] [greater than▼] [2        ]             │
│                                                         │
│ CONNECTED BY [ALLOCATED_TO ▼] TO                       │
│   [label ▼] [equals ▼] [Slot ]                         │
│                                          [▶ Run Query] │
└─────────────────────────────────────────────────────────┘
```

### Database Support

| Database | Port | Status | Features |
|----------|------|--------|----------|
| Supabase | 18000 | ✅ Ready | Tables, RLS, Realtime |
| SurrealDB | 18010 | ✅ Ready | Graph links, Live queries |
| Sled | 18400 | ✅ Ready | KV, Prefix scan |
| **Sledis** | 18401 | ✅ Ready | Redis-over-Sled (RESP) |
| Redis | 18030 | ✅ Ready | Cache, Pub/sub |

### Conda Integration

Full Python/scientific computing via API:
- NumPy, SciPy, Pandas (18810)
- PyTorch, TensorFlow (18820)
- GeoPandas, Shapely (18840)
- Astropy, Skyfield, SGP4 (18841)
- PyTorch Geometric, DGL (18842)

### Future Integrations (v0.2+)

| Integration | Version | Notes |
|-------------|---------|-------|
| OSINT Tools | v0.2 | Shodan, VirusTotal |
| Google Cloud | v0.3 | BigQuery, Vertex AI |
| Office 365 | v0.3 | Graph API |
| Figma MCP | v0.3 | Design import |

### Build Order

1. ✅ Get Bolt prompt (`BOLT-SPEC-FINAL.md`)
2. Build in Bolt with visual query builder
3. Connect to existing Supabase
4. Add SurrealDB adapter
5. Add Sled/Sledis adapters
6. Integrate Conda bridge
7. Push to git for collaborative dev

---

## 5. CESIUM/GIS SYSTEM

### Current Problems
1. **3 separate Cesium Viewer instances** (bad for performance)
   - `CesiumWorldView.tsx:117`
   - `LaserLightMultiView.tsx:116`
   - `SpaceWorldDemo.tsx:254`

2. **Only 6 ground stations** (need 257)
3. **Only ~1 satellite** in Supabase (need 12 birds)
4. **Beams not firing** (no satellites = no beams)
5. **Cognigraph force fields** = garbage math (replace later with real delta angles)

### Target Architecture
```
ONE Cesium Viewer Instance
├── Space DataSource (12 birds, Van Allen, lasers)
├── Maritime DataSource (vessels, ports)
├── Land DataSource (ground stations)
└── Fusion DataSource (combined view)

Toggle domains by visibility, NOT separate viewers
```

### 12 Birds (Walker Delta Constellation)
From `ctas7-command-center/src/data/satelliteData.ts`:
- **Alpha, Beta, Gamma, Delta** (Plane 0)
- **Epsilon, Zeta, Eta, Theta** (Plane 1)
- **Iota, Kappa, Lambda, Mu** (Plane 2)
- 55° inclination, 15,000 km altitude (Van Allen belt)

### 257 Ground Stations
Schema exists in: `ctas-7-shipyard-staging/ctas7_ground_station_graph_schema.surql`
Data needs to be seeded (currently only 6 mock stations)

### Multiworld Structure (Already Built!)
```
src/fusion/
├── core/
├── domains/
│   └── MaritimeDomain.ts (13,793 lines)
├── integration/
├── ontology/
└── worlds/
    └── WorldManager.ts (12,414 lines)
```

### GIS Split
| Engine | Use Case | Platform |
|--------|----------|----------|
| **Cesium** | 3D globe, orbital, space, lasers | Desktop/Server |
| **Mapbox** | 2D ops-main, tactical | Desktop + iOS |
| **MapKit** | iOS native | iOS only |

---

## 5. CDN STRATEGY

### Tier 1: Cloudflare R2 (Public/Semi-public)
```
Cost: ~$5-20/month
Egress: FREE

Contents:
├── Foundation crates (obfuscated, signed)
├── Cesium tiles (terrain, imagery)
├── WASM modules
├── UI assets (JS, CSS, fonts)
└── Documentation / schemas

Domain: cdn.sx9.io
```

### Tier 2: GCP Cloud CDN (IAM-gated)
```
Cost: ~$10-50/month (LB free at idle)

Contents:
├── Operator-bound crates (encrypted)
├── QKD key material cache
├── Biometric verification artifacts
├── Mission-specific configs
└── Partner CTAS distributions

Domain: crates.synaptix9.com
```

### Tier 3: Internal CDN (Air-gapped)
```
Cost: Existing infrastructure

Contents:
├── Classified crates
├── Dark ops tooling
├── Kali Plasma ISOs
└── Biometric templates

Access: Internal only / tunnel
```

### Terraform Required
```
terraform/
├── cloudflare/
│   ├── r2_buckets.tf
│   ├── workers.tf
│   └── dns.tf
├── gcp/
│   ├── cloud_cdn.tf
│   ├── load_balancer.tf
│   ├── gcs_buckets.tf
│   └── iam.tf
└── variables.tf
```

---

## 6. KALI PLASMA

### Overview
Biometric-bound Kali ISO with invisible eBPF tooling

### Stack
```
┌─────────────────────────────────────────┐
│         USERSPACE (Clean)               │
│  • Vanilla Kali appearance              │
│  • No suspicious binaries               │
│  • No C2 signatures                     │
└─────────────────────────────────────────┘
                    │
════════════════════════════════════════════
                KERNEL BOUNDARY
════════════════════════════════════════════
                    │
┌─────────────────────────────────────────┐
│      KERNEL (eBPF/XDP) - Invisible      │
│                                         │
│  • Rust eBPF (Aya framework)            │
│  • SDT packet processor                 │
│  • L2 frame intercept                   │
│  • Plasma state evaluation              │
│  • No userspace tools needed            │
└─────────────────────────────────────────┘
```

### Build Pipeline
1. Base Kali ISO (clean)
2. Strip unnecessary tools
3. Inject Rust eBPF modules (compiled, no source)
4. Minimal loader (loads eBPF, then exits)
5. NATS bridge (optional, for distributed)
6. Output: `kali-plasma-{operator_hash}.iso`

### Components Needed
- `KaliISOFactory.tsx` (exists in canonical)
- Rust eBPF modules (Aya framework)
- Biometric verification integration
- CDN tunnel for updates

---

## 7. SOFTWARE-DEFINED THYRISTOR (SDT)

### Concept
Software transistor: Base (trigger) controls flow from Emitter (input) to Collector (output)

### States
```
OFF → PRIMED → CONDUCTING → LATCHED
 ▲                              │
 └────────── RESET ─────────────┘
```

### L2 Frame Format
```
┌───────┬───────┬──────┬─────┬─────┬─────┬─────┬─────┬────┐
│ DST   │ SRC   │ TYPE │ VER │STATE│ Δθ  │  H  │HASH │ PL │
│ MAC   │ MAC   │0xSD77│     │     │     │     │     │    │
└───────┴───────┴──────┴─────┴─────┴─────┴─────┴─────┴────┘
  6       6       2      2     2     4     4     4    var

TYPE: 0xSD77 (custom EtherType)
Δθ: Delta angle (fixed point)
H: Entropy
HASH: Truncated Murmur3
PL: Payload (trigger data, action, etc)
```

### Payload Types
```
PING    0x00   [seq:2][timestamp:8]
TRIG    0x01   [gate_id:4][reason:2][data:N]
STATE   0x02   [sdt_id:4][new_state:2][plasma:12]
LATCH   0x03   [sdt_id:4][lock_hash:4]
RESET   0x04   [sdt_id:4][auth_sig:32]
PLASMA  0x05   [field_id:4][Δθ:4][H:4][excited:1]
ROUTE   0x06   [src:4][dst:4][path:N]
CANARY  0x07   [trip_type:2][evidence_hash:4]
SWARM   0x08   [swarm_id:4][cmd:2][params:N]
```

### NATS Bridge
```
L2 SDT Frame              NATS Subject
─────────────             ────────────
[TRIG gate_id=42]    ◀──▶  sx9.sdt.42.trigger
[STATE id=7]         ◀──▶  sx9.sdt.7.state
[PLASMA field=3]     ◀──▶  sx9.plasma.3.update
[CANARY trip]        ◀──▶  sx9.canary.trip
```

### Applications
- CDN ingress filtering (C2 blocking)
- PLC/SCADA safety interlocks
- Robotics motion gating
- Dark ops canary triggers
- Neurological microkernel (prompt = program)

---

## 8. EPHEMERAL SYSTEMS

### Standard Ephemeral (Ground Truth Collection)
```
Session starts
├── Add locations, amend, drag
├── Collect ground truth (native GPS, LiDAR, camera)
├── Annotate
└── Session ends
    ├── DISCARD (gone)
    ├── WORK FILE → Adjudication queue
    └── COMMIT (trusted user)
```

### Dark Ops Ephemeral
```
Tunnel Stack:
├── Layer 1: Tor / I2P (Anonymity)
├── Layer 2: WireGuard (Encryption)
├── Layer 3: Steganographic Channel (Covert)
└── Layer 4: Decoy Traffic (Obfuscation)

Container:
├── RAM-only filesystem
├── No logs
├── Randomized fingerprint
├── Time-bombed (auto-destruct)
├── Canary triggers
└── Dead man's switch

Exfil:
├── Strip metadata
├── Remove timestamps
├── Hash-only references
├── Delayed transmission
└── Dead drop (onion/stealth S3/physical)
```

### Stealth Levels
| Level | Use |
|-------|-----|
| OVERT | Regular ephemeral (ground truth) |
| COVERT | Tunneled, no attribution |
| INVISIBLE | Full dark ops, canary protected |

---

## 9. BIOMETRIC + QUANTUM BUILD

### Current: Biometric Binding
```
Operator biometric
    │
    ▼
H(bio) → 256-bit
    │
    ▼
K_build = KDF(H(bio), salt, iterations)
    │
    ├── Decrypt source (encrypted at rest)
    ├── Compile with embedded key check
    └── Sign artifact
    │
    ▼
Bound artifact (won't run without operator)
```

### Phase 1: Hybrid Keys (Next SmartCrate)
```
Classical Half          Quantum Half
H(bio) → 128-bit       QKD → 128-bit
        │                    │
        └────────┬───────────┘
                 ▼
         K = H(bio) ⊕ QKD
         256-bit hybrid
```

### Phase 2: Quantum Obfuscation (Foundation Crates)
- Control flow flattening (quantum seed)
- Opaque predicates (quantum-derived)
- String encryption (QKD)
- Dead code injection (quantum-random)
- Instruction substitution (quantum coin)

Result: Every build unique, defeats signatures, reverse engineering → quantum problem

### Phase 3: Full Quantum (Future)
- QKD from 12-bird constellation
- Bio-quantum binding
- Quantum-signed builds
- Post-quantum crypto throughout

---

## 10. iOS NATIVE STRATEGY

### Why Native?
- Cesium doesn't work on iOS (WebGL issues)
- Need native sensors (GPS, LiDAR, ARKit)
- Touch/gesture optimization
- App Store distribution option

### Architecture
```
iPad Plasma App (Swift UI)
├── Native UI
│   ├── MapKit (2D maps)
│   ├── Graph Viewer (native)
│   └── Workflow controls
├── Toolchain Launcher (iTunes-style)
│   └── HD4 phase icons (Hunt, Detect, Disrupt, Disable, Dominate)
├── Native Sensors
│   ├── CoreLocation (GPS)
│   ├── ARKit (LiDAR, spatial)
│   ├── Camera (geotagged)
│   └── Compass/Barometer
├── Tunnel to CDN
│   ├── Cloudflare WARP
│   └── WireGuard
└── Cesium Stream (server-rendered, displayed on device)
```

### Ground Truth Collection
```swift
struct GroundTruthCollection {
    timestamp: DateTime
    location: {
        lat, lon, alt
        accuracy_m
        source: 'gps' | 'lidar' | 'manual'
    }
    attachments: {
        photos: GeotaggedPhoto[]
        lidar_scan: LiDARData
        notes: String
    }
    validates: EntityID  // What this ground truth corrects
}
```

---

## 11. DATA FIXES REQUIRED

### Priority 1: 12 Birds
**Source:** `ctas7-command-center/src/data/satelliteData.ts`
**Target:** Supabase `satellites` table
**Action:** Seed Walker Delta constellation

### Priority 2: 257 Ground Stations
**Source:** Need to find actual coordinates (GEE scrape? WASM sims?)
**Schema:** `ctas-7-shipyard-staging/ctas7_ground_station_graph_schema.surql`
**Target:** Supabase `ground_nodes` + SurrealDB
**Issue:** Lat/Long may be inverted in scraped data

### Priority 3: Fix Lat/Long Inversion
**Problem:** Ground stations appearing in ocean
**Cause:** Rogue blake hash or data scraping error
**Solution:** Ground truth collection via iOS app to correct

### Priority 4: Cognigraph Math
**Problem:** `generateForceFields()` is LLM-hallucinated garbage
**Solution:** Replace with real delta angles from hash engine
**When:** During orbital system work (not now)

---

## 12. ECS STRATEGY (ATLAS + apecs)

### Why Dual-ECS?

| Need | ATLAS (Legion) | apecs |
|------|----------------|-------|
| Graph traversal 100K nodes | ✅ SIMD | ❌ Slow |
| Matroid rank calculation | ✅ Parallel | ❌ Sequential |
| Tick sync (250ns) | ✅ Precise | ❌ Async jitter |
| Database queries | ❌ Blocks | ✅ Native async |
| WebSocket streams | ❌ Awkward | ✅ Tokio |
| Change tracking | ❌ Needs plugin | ✅ Built-in |
| WASM deployment | ⚠️ Partial | ✅ Full |

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      ATLAS DAEMON (Rust)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ Legion Core  │  │ SIMD Batch   │  │ Tick Synchronizer    │  │
│  │ (Hot Path)   │  │ Processor    │  │ (250ns resolution)   │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘  │
│         └────────────┬────┴──────────────────────┘              │
│                      ▼                                          │
│              ┌───────────────┐                                  │
│              │  Ring Buffer  │  ← Zero-copy shared memory       │
│              │  (SPSC/MPMC)  │                                  │
│              └───────┬───────┘                                  │
└──────────────────────┼──────────────────────────────────────────┘
                       │ NATS / IPC
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    apecs World (Async)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ I/O Systems  │  │ State Sync   │  │ Change Tracking      │  │
│  │ (DB, HTTP)   │  │ (from ATLAS) │  │ (to ATLAS)           │  │
│  └──────────────┘  └──────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Hot Path (ATLAS Daemon)
- Dijkstra/BFS on 100K+ node graphs
- Matroid independence checking
- Trivariate hash batch generation
- Convergence detection
- Tick-aligned CUID slot encoding

### Cold Path (apecs World)
- Supabase/SurrealDB queries
- WebSocket message handling
- NATS pub/sub
- Change event propagation
- UI state synchronization

### SlotGraph Bridge

```typescript
class SlotGraphQueryEngine {
  // Hot path → ATLAS via ring buffer
  async findOptimalRoute(src: string, dst: string): Promise<Route> {
    return this.atlas.dispatch('dijkstra', { src, dst });
  }
  
  // Cold path → stays async
  async queryNetwork(query: SlotGraphQuery): Promise<SlotGraphResult> {
    return this.querySurreal(query);
  }
}
```

### Performance Targets

| Operation | apecs alone | apecs + ATLAS |
|-----------|-------------|---------------|
| 10K node Dijkstra | ~50ms | ~3ms |
| Matroid rank (1K) | ~20ms | ~0.5ms |
| Batch hash (10K) | ~100ms | ~5ms |
| Convergence check | ~10ms | ~0.2ms |

### sx9-atlas-bus (IMPLEMENTED ✅)

Custom zero-allocation ring buffer with built-in delta state tracking:

```
crates/sx9-atlas-bus/
├── src/
│   ├── lib.rs       # Module exports
│   ├── ring.rs      # Lock-free SPSC ring buffer
│   ├── command.rs   # Command types (Dijkstra, Matroid, SDT, Plasma)
│   ├── result.rs    # Result types
│   ├── plasma.rs    # Plasma state + SDT gate control
│   └── bus.rs       # Priority-routed bus (critical/urgent/normal)
└── benches/
    └── throughput.rs
```

**Key Features:**
- **Delta Angle Tracking**: Built into `PlasmaState` (0-360° → u16)
- **Delta Class Thresholds**: None (<2°) / Micro (2-10°) / Soft (10-25°) / Hard (25-60°) / Critical (≥60°)
- **SDT Gate Control**: Off → Primed → Conducting → Latched
- **Priority Lanes**: Critical (256) > Urgent (1024) > Normal (4096)
- **Backpressure Signaling**: Pressure ratio triggers delta class warnings
- **~5-10ns latency** per push/pop
- **NATS Bridge** (feature flag): JetStream for distributed daemons

**NATS Subjects (with `nats` feature):**
```
sx9.atlas.cmd.critical    - Critical commands
sx9.atlas.cmd.urgent      - Urgent commands
sx9.atlas.cmd.normal      - Normal commands
sx9.atlas.result          - Results back to apecs
sx9.atlas.plasma          - Plasma state broadcasts
sx9.atlas.tick            - Tick synchronization
sx9.sdt.{gate_id}.trigger - SDT gate triggers
sx9.sdt.{gate_id}.state   - SDT state changes
```

**JetStream Streams:**
- `SX9_ATLAS_COMMANDS` - Persisted commands (1 hour retention)
- `SX9_ATLAS_RESULTS` - Persisted results (1 hour retention)
- `SX9_PLASMA_STATE` - Plasma snapshots (5 min retention)
- `SX9_SDT_EVENTS` - SDT events (24 hour retention)

---

## 13. DAEMON UPGRADE PLAN

All daemons must migrate to `sx9-atlas-bus` for unified delta state tracking.

### Current Daemon Inventory

| Daemon | Location | Current IPC | Status |
|--------|----------|-------------|--------|
| `ctas7-atlas-daemon` | shipyard-staging | `tokio::broadcast` | 🔴 Upgrade |
| `ctas7-foundation-daemon` | shipyard-staging | HTTP/TCP | 🔴 Upgrade |
| `ctas7-osint-machine` | shipyard-staging | Raw TCP | 🔴 Upgrade |
| `ctas7-cdn-threat-reaction` | shipyard-staging | Unknown | 🔴 Upgrade |
| `sx9-atlas-daemon` | TBD | `sx9-atlas-bus` | 🟢 Reference |

### Upgrade Benefits

| Before | After |
|--------|-------|
| No delta tracking | Built-in Δθ in every message |
| No supersession logic | Delta class triggers regeneration |
| No SDT gates | Full SDT lifecycle (canary, plasma) |
| Async overhead (~50-100ns) | Lock-free (~5-10ns) |
| No backpressure | Pressure → delta class warnings |
| Scattered state | Unified `PlasmaState` snapshot |

### Delta State Flow

```
Command dispatched
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  sx9-atlas-bus                                      │
│  ┌─────────────────────────────────────────────┐   │
│  │ PlasmaState                                  │   │
│  │  • delta_angle: u16 (0-65535 → 0-360°)      │   │
│  │  • entropy: u32                              │   │
│  │  • excited: bool                             │   │
│  │  • sdt_state: Off/Primed/Conducting/Latched │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
       │
       ▼
Delta Class Evaluation
       │
       ├── <2°  → None: No action
       ├── 2-10° → Micro: Adjust CUID slots 10-11
       ├── 10-25° → Soft: Regenerate SCH + CUID
       ├── 25-60° → Hard: Full trivariate regeneration
       └── ≥60° → Critical: Supersede with new lineage
```

### Upgrade Order

1. **sx9-atlas-daemon** (new) - Reference implementation
2. **ctas7-atlas-daemon** - Core cognitive engine (1ms tick)
3. **ctas7-foundation-daemon** - Orchestrator
4. **ctas7-osint-machine** - Tool execution
5. **ctas7-cdn-threat-reaction** - Canary/plasma integration

---

## 14. IMMEDIATE ACTION ITEMS

### Phase 1: Stabilize (This Week)
- [ ] Verify `sx9-development-center` is safe copy of canonical
- [ ] Add mock data fallback to `useGroundNodes()` and `useSatellites()`
- [ ] Seed Supabase with 12 birds from `satelliteData.ts`
- [ ] Test LaserLight with actual satellites

### Phase 2: CDN Setup (Next Week)
- [ ] Create Cloudflare R2 bucket for foundation crates
- [ ] Set up GCP Cloud CDN with IAM
- [ ] Configure DNS (cdn.sx9.io, crates.synaptix9.com)
- [ ] Write Terraform for infrastructure

### Phase 3: Consolidate Cesium (Week 3)
- [ ] Refactor to single Viewer instance
- [ ] Implement DataSource per domain
- [ ] Wire up domain visibility toggles
- [ ] Remove duplicate viewer components

### Phase 4: Foundation Crate Registry (Week 4)
- [ ] Build `sx9-registry` publish tool
- [ ] Implement signature verification
- [ ] Set up cargo config for private registry
- [ ] Publish first foundation crate

### Phase 5: ATLAS Daemon (Month 2) ← NEW
- [ ] Create `sx9-atlas-daemon` Rust crate
- [ ] Integrate Legion ECS for hot path
- [ ] Integrate apecs for async I/O
- [ ] Implement ring buffer (crossbeam/custom)
- [ ] Port SlotGraph Dijkstra to Legion
- [ ] Add matroid rank SIMD implementation
- [ ] NATS bridge for IPC
- [ ] Benchmark: target 10x improvement

### Phase 6: Kali Plasma MVP (Month 2-3)
- [ ] Rust eBPF modules (Aya)
- [ ] SDT frame processor
- [ ] Biometric build integration
- [ ] ISO build pipeline

### Phase 7: iOS App (Month 3)
- [ ] Swift UI scaffold
- [ ] MapKit integration
- [ ] Ground truth collection
- [ ] Tunnel to CDN

---

## 15. COMPONENT LOCATIONS

### Forge Workflow System
| Component | Location |
|-----------|----------|
| Backend | `synaptix9-workflow-system/tools/forge-backend/src/index.ts` |
| UI | `ctas-7-shipyard-staging/ctas7-command-center/src/components/SYNAPTIX9/UniversalTopologyDesigner.tsx` |
| Service | `ctas-7-shipyard-staging/ctas7-command-center/src/services/ForgeIntegrationService.ts` |

### Cesium/LaserLight
| Component | Location |
|-----------|----------|
| Main Viewer | `ctas7-command-center-canonical/src/components/LaserLightMultiView.tsx` |
| Satellite Panel | `ctas7-command-center-canonical/src/components/SatelliteControlPanel.tsx` |
| World Manager | `ctas7-command-center-canonical/src/fusion/worlds/WorldManager.ts` |
| Maritime Domain | `ctas7-command-center-canonical/src/fusion/domains/MaritimeDomain.ts` |
| Cesium Manager | `ctas7-command-center-canonical/src/services/cesiumWorldManager.ts` |

### Ephemeral Systems
| Component | Location |
|-----------|----------|
| Asset Orchestrator | `ctas7-recycle/rcx-ctas7-broken-deprecated/src/services/EphemeralAssetOrchestrator.ts` |
| Toolchain Manager | `ctas7-recycle/rcx-ctas7-broken-deprecated/crates/ctas-core-integration/src/ephemeral_toolchain_manager.rs` |

### GLAF
| Component | Location |
|-----------|----------|
| Graph Server | `ctas-7-shipyard-staging/ctas7-glaf-graph-server/` |
| Clients | `ctas-7-shipyard-staging/ctas7-glaf-clients/` |
| Visualizer | `ctas-7-shipyard-staging/mcp-glaf-visualizer/` |
| UI Viewer | `ctas-7-shipyard-staging/ctas7-ui-components-tactical/src/components/GLAFGraphViewer.tsx` |

### ATLAS Daemon (NEW)
| Component | Location | Status |
|-----------|----------|--------|
| ATLAS Core | `TBD: sx9-atlas-daemon/` | 🔴 To Build |
| Legion ECS | External crate | ✅ Available |
| apecs | External crate | ✅ Available |
| Ring Buffer | `crossbeam-channel` / custom | ✅ Available |
| SlotGraph Bridge | `packages/core/src/services/SlotGraphQueryEngine.ts` | 🟡 Needs integration |

### Port Manager
| Component | Location |
|-----------|----------|
| Rust Crate | `ctas7-recycle/rcx-ctas7-broken-deprecated/crates/ctas7-port-manager-saving/` |

### Tunnel
| Component | Location |
|-----------|----------|
| Docker Compose | `ctas7-command-center-canonical/deployment-modes/docker-compose.tunnel.yml` |

### Satellite Data
| Component | Location |
|-----------|----------|
| Walker Delta | `ctas7-command-center/src/data/satelliteData.ts` |
| Ground Station Schema | `ctas-7-shipyard-staging/ctas7_ground_station_graph_schema.surql` |
| Starlink Gateways | `ctas-7-shipyard-staging/ctas7_starlink_gateway_schema.surql` |

### SurrealDB Data
| File | Contents |
|------|----------|
| `ctas7_ground_station_graph_schema.surql` | 257 station schema + graph |
| `ctas7_starlink_gateway_schema.surql` | Starlink gateways |
| `sample_ground_stations.sql` | 16 sample stations |
| `surrealdb_seed.surql` | OV/SV namespace seed |

---

## APPENDIX A: Port Allocations

```
18xxx - Internal services
  18005 - Hashing Engine
  18050 - GLAF Graph Server
  18100 - Neural Mux / Cannon
  18108 - Stats
  18350 - Forge Backend
  18400 - Sled KV Store
  18401 - Sledis (Redis-over-Sled)
  18500 - ATLAS Daemon (NEW!)
  
18xxx - Databases (continued)
  18000 - Supabase local
  18010 - SurrealDB (graph)
  18019 - SurrealDB (GLAF namespace)
  18020 - NATS
  18030 - Redis
  
18xxx - Conda Bridge
  18800 - Conda API Gateway
  18810 - NumPy/SciPy/Pandas
  18820 - PyTorch/TensorFlow
  18840 - GeoPandas/Shapely
  18841 - Astropy/Skyfield/SGP4
  18842 - PyTorch Geometric/DGL
  
25xxx - UI/Frontend
  25175 - Canonical Command Center
  25176 - DAW (Data Analytics Workbench)
  
28xxx - Voice/WebSocket
  18765 - Voice pipeline
  28765 - Voice pipeline (alt)
  
8xxx  - External Databases
  8000  - SurrealDB (legacy)
```

---

## APPENDIX B: Key Files Summary

### Must Not Touch
- `ctas7-command-center-canonical/` (entire directory)

### Safe to Experiment
- `sx9-development-center/`

### Need to Migrate to Canonical
- `UniversalTopologyDesigner.tsx` (from staging)
- `ForgeIntegrationService.ts` (from staging)
- `satelliteData.ts` (from non-canonical)

### Need to Recover from Recycle
- `EphemeralAssetOrchestrator.ts`
- `EphemeralToolchainManager.rs`
- `ctas7-port-manager`

---

## APPENDIX C: Database Strategy

```
┌─────────────┐  ┌──────────────┐  ┌─────────────┐
│  Supabase   │  │  SurrealDB   │  │    Sled     │
│   (ACID)    │  │   (Graph)    │  │   (RFC)     │
│             │  │              │  │             │
│ • 12 birds  │  │ • Relations  │  │ • KV sync   │
│ • 257 GS    │  │ • Routes     │  │ • Local     │
│ • Telemetry │  │ • Analysis   │  │ • Fast      │
└──────┬──────┘  └──────┬───────┘  └──────┬──────┘
       │                │                 │
       └────────────────┼─────────────────┘
                        ▼
              ┌──────────────────┐
              │  DatabaseMux     │
              │  (Keeps sync)    │
              └──────────────────┘
                        │
                        ▼
              ┌──────────────────┐
              │  Cesium picks    │
              │  ONE for viz     │
              └──────────────────┘
```

---

*End of Master Plan*
*Review and update as implementation progresses*

