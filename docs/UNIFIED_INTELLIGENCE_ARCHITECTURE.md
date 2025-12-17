# SX9 Unified Intelligence Architecture

## Overview

This document maps the complete intelligence infrastructure across the SX9 ecosystem:
- **Python Pipelines** - OSINT collection, ChromaDB vectorization
- **Rust Orchestration** - Leptose engine, EEI system, GLAF core
- **TypeScript Analytics** - graph-db UI, workflow builder, Legion hot path

## Component Map

```
═══════════════════════════════════════════════════════════════════════════════
                    SX9 UNIFIED INTELLIGENCE ARCHITECTURE
═══════════════════════════════════════════════════════════════════════════════

┌─────────────────────────────────────────────────────────────────────────────┐
│                         DATA COLLECTION LAYER                                │
│                           (Python Pipelines)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  OSINT MACHINE (ctas7-command-center/ctas7-intelligence-generator)  │   │
│  │                                                                     │   │
│  │  master_intelligence_orchestrator.py                                │   │
│  │  ├── LegalCaseHarvester      ├── VirtualLOCBranch                  │   │
│  │  ├── GAOIntelligence         ├── CongressionalIntel                │   │
│  │  ├── DataGovHarvester        ├── DOJRealTimeIntel                  │   │
│  │  ├── LocalMediaIntel         ├── SocialMediaIntel                  │   │
│  │  ├── CybersecurityUSIM       ├── MISPThreatIntel                   │   │
│  │  └── PlasmaDisplay           └── VirusTotal/OTX/Shodan             │   │
│  │                                                                     │   │
│  │  gnn_osint_intelligence.py                                          │   │
│  │  ├── GNNOSINTModel (GCN + GAT + Transformer)                       │   │
│  │  ├── OSINTNode/OSINTEdge/OSINTGraph                                │   │
│  │  └── LOCAL_CPU/LOCAL_GPU/HIGH_GPU_REMOTE modes                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  THREAT VECTOR PIPELINE (sx9/tools/abe/iac/node-interview-generator)│   │
│  │                                                                     │   │
│  │  threat_vector_pipeline.py                                          │   │
│  │  Sources (15+):                                                     │   │
│  │  ├── MITRE ATT&CK (Enterprise, ICS, Mobile)                        │   │
│  │  ├── MITRE D3FEND (countermeasures)                                │   │
│  │  ├── Atomic Red Team, Sigma, Nuclei                                │   │
│  │  ├── LOLBAS, GTFOBins, YARA rules                                  │   │
│  │  ├── Nmap scripts, Wazuh rules, MITRE CAR                          │   │
│  │  ├── Caldera, LOLDrivers, HijackLibs, WADComs                      │   │
│  │  ├── OSINT Framework, Sherlock, ExploitDB                          │   │
│  │  └── ATL-Physical (training only, invisible operationally)         │   │
│  │                                                                     │   │
│  │  Outputs:                                                           │   │
│  │  ├── ChromaDB vectors (threat_content)                             │   │
│  │  ├── Phi-3 LoRA training data                                      │   │
│  │  ├── DistilBERT training data                                      │   │
│  │  └── Neo4j Cypher queries                                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  CHROMADB UNICODE LOADER (sx9/tools/abe/iac/)                       │   │
│  │                                                                     │   │
│  │  add_to_chromadb_with_unicode.py                                    │   │
│  │  Collections:                                                       │   │
│  │  ├── tools         - Threat intel tools with Unicode ops           │   │
│  │  ├── ctas_tasks    - CTAS tasks (uuid- format)                     │   │
│  │  ├── ptcc_configs  - PTCC configurations                           │   │
│  │  └── tool_chains   - Tool chains with Unicode ops                  │   │
│  │                                                                     │   │
│  │  Features:                                                          │   │
│  │  ├── Dual trivariate hash extraction (SCH, CUID, UUID)             │   │
│  │  ├── Unicode operation mapping (U+E000-E9FF)                       │   │
│  │  └── Sentence transformer embeddings (all-MiniLM-L6-v2, 384-dim)   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ NATS (osint.intel, eei.query)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ORCHESTRATION LAYER                                    │
│                         (Rust - sx9-leptose)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  LEPTOSE KNOWLEDGE ENGINE (sx9/crates/sx9-leptose)                  │   │
│  │                                                                     │   │
│  │  LeptoseEngine                                                      │   │
│  │  ├── NatsBridge (osint.intel, eei.query, eei.answer, leptose.*)    │   │
│  │  ├── KnowledgeGraph (petgraph + GLAF integration)                  │   │
│  │  ├── ChromaDbClient (query existing Python vectors)                │   │
│  │  └── EEI satisfaction routing                                      │   │
│  │                                                                     │   │
│  │  Responsibilities:                                                  │   │
│  │  ├── Ingest OSINT from Python pipelines via NATS                   │   │
│  │  ├── Build runtime knowledge graph                                 │   │
│  │  ├── Query ChromaDB for semantic search                            │   │
│  │  └── Route EEI queries to satisfiers                               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  GLAF CORE (sx9/crates/sx9-glaf-core)                               │   │
│  │                                                                     │   │
│  │  ├── GLAFCore (graph storage + query)                              │   │
│  │  ├── TethAnalyzer (entropy calculation)                            │   │
│  │  ├── HawkesIntensity (temporal analysis)                           │   │
│  │  ├── HmmPhaseDetector (phase detection)                            │   │
│  │  ├── MatroidRank (rank calculation)                                │   │
│  │  └── ConvergenceMonitor (h1/h2 convergence)                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Feeds
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         EEI SYSTEM LAYER                                     │
│                  (Rust - sx9-foundation-core)                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  eei.rs                    - Sliding windows, buffer_until, handoff        │
│  eei_processor.rs          - Toolchain satisfaction matrix                 │
│  eei_decision_engine.rs    - Multi-domain intel, LLM orchestra, Monte Carlo│
│  distributed_eei.rs        - Node specialization, TTL/LTOV propagation     │
│  persistent_eei.rs         - Time-of-Value decay curves                    │
│  intel_retrieval.rs        - Threat feeds (MalwareBazaar, VT, ThreatFox)   │
│  node_crate_eei_correlator - EEI ↔ capabilities mapping                    │
│                                                                             │
│  PTCC → ATT&CK → EEI Correlation:                                          │
│  ├── 32 PTCC Primitives (U+E500-E51F)                                      │
│  ├── 14 ATT&CK Tactics → Primitive mappings                                │
│  └── Auto-generated EEIs from techniques                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Drives
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    ANALYTICAL SENSOR LAYER                                   │
│              (TypeScript - graph-db) - NOT STORAGE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  LEGION HOT PATH (src/lib/glaf/legionHotPath.ts)                    │   │
│  │  <1µs query resolution through:                                     │   │
│  │  ├── Unicode → Memory (10ns arithmetic)                             │   │
│  │  ├── Nonagon routing (9 analytical nodes by domain)                 │   │
│  │  │   • Cyber: 0-1, Kinetic: 2-3, Cognitive: 4-5                    │   │
│  │  │   • Social: 6-7, Temporal: 8                                     │   │
│  │  ├── Crystal tuning (precision, speed, depth, noise)                │   │
│  │  └── Lock-free ring buffer (30ns writes)                            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  WORKFLOW BUILDER (src/lib/workflow/ + components/Workflow*.tsx)    │   │
│  │  LangGraph/n8n-style tool chain orchestration:                      │   │
│  │  ├── WorkflowExecutor - DAG validation, cycle detection             │   │
│  │  ├── WorkflowBuilder.tsx - Visual node editor                       │   │
│  │  ├── WorkflowCanvas.tsx - Drag-drop canvas                          │   │
│  │  ├── WorkflowNodeLibrary.tsx - Tool palette                         │   │
│  │  └── WorkflowExecutionHistory.tsx - Run history                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  GLAF ANALYTICS (src/lib/glaf/)                                     │   │
│  │  ├── apecsLayer.ts      - APECS temporal layer                      │   │
│  │  ├── plasma.ts          - Plasma-Defender components                │   │
│  │  ├── threat_intel.ts    - Threat knowledge base                     │   │
│  │  ├── ringBuffer.ts      - SPSC lock-free ring buffer                │   │
│  │  ├── synaptixBridge.ts  - Synaptix integration                      │   │
│  │  └── orbital.ts         - Orbital operations                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  UI COMPONENTS (src/components/)                                    │   │
│  │  ├── GLAFDashboard.tsx       - Main GLAF dashboard                  │   │
│  │  ├── GLAFIntelWorksurface.tsx- Intel worksurface                    │   │
│  │  ├── GraphVisualization.tsx  - Cytoscape graph rendering            │   │
│  │  ├── NonagonPanel.tsx        - 9-node analytical display            │   │
│  │  ├── ConvergenceDashboard.tsx- h1/h2 convergence metrics            │   │
│  │  ├── ATLASMonitor.tsx        - ATLAS daemon monitoring              │   │
│  │  ├── IACDashboard.tsx        - IAC operations                       │   │
│  │  └── MonitoringDashboard.tsx - System monitoring                    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  src/lib/trivariate.ts     - Trivariate hash generation                    │
│  src/lib/thalamicFilter.ts - Clarity scoring                               │
│  src/lib/gateway.ts        - Backend gateway client                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Storage Locations

| Component | Path | Format |
|-----------|------|--------|
| ChromaDB (tools, tasks, ptcc) | `sx9/tools/abe/iac/04-abe-iac/node-interview-generator/output/vectors/chromadb` | Persistent |
| Threat Vectors | `sx9/tools/abe/iac/node-interview-generator/output/vectors/` | JSON |
| Training Data | `sx9/tools/abe/iac/node-interview-generator/output/training_data/` | JSONL |
| Cypher Queries | `sx9/tools/abe/iac/node-interview-generator/output/cypher/` | Cypher |
| Tool Corpus | `sx9/data/tool-corpus/` | TOML/JSON |
| OSSEC Rules | `sx9/crates/sx9-plasma-defender/rules/` | TOML |

## NATS Subjects

| Subject | Direction | Purpose |
|---------|-----------|---------|
| `osint.intel` | Python → Rust | OSINT intelligence ingest |
| `eei.query` | Any → Leptose | EEI query requests |
| `eei.answer` | Leptose → Any | EEI answer responses |
| `leptose.status` | Leptose → Any | Engine status updates |
| `leptose.graph.*` | Any → Leptose | Graph operations |
| `tool.output.*` | Tool Executor → Any | Tool execution results |

## Data Flow

```
1. OSINT Collection (Python)
   ├── master_intelligence_orchestrator.py (12+ sources)
   ├── threat_vector_pipeline.py (15+ sources)
   └── add_to_chromadb_with_unicode.py (vectorization)
                    │
                    ▼
2. Vector Storage (ChromaDB)
   ├── tools collection
   ├── ctas_tasks collection
   ├── ptcc_configs collection
   ├── tool_chains collection
   └── threat_content collection
                    │
                    ▼
3. Orchestration (Rust - sx9-leptose)
   ├── NATS ingest (osint.intel)
   ├── Knowledge graph building
   ├── ChromaDB semantic search
   └── EEI satisfaction routing
                    │
                    ▼
4. EEI System (Rust - sx9-foundation-core)
   ├── Time-of-Value decay
   ├── Sliding windows
   ├── PTCC → ATT&CK → EEI correlation
   └── Toolchain satisfaction
                    │
                    ▼
5. Scenario Execution (Rust - sx9-threat-simulator)
   ├── PTCC personas
   ├── Tool executor
   ├── OSSEC monitoring
   └── Correlation engine
```

## Integration Points

### Python → Rust (NATS)

```python
# Python: Publish OSINT intel
import asyncio
import nats
import json

async def publish_intel(intel):
    nc = await nats.connect("nats://localhost:4222")
    await nc.publish("osint.intel", json.dumps(intel).encode())
    await nc.close()
```

```rust
// Rust: Subscribe to OSINT intel
let mut subscriber = nats.subscribe("osint.intel").await?;
while let Some(msg) = subscriber.next().await {
    let intel: OsintIntel = serde_json::from_slice(&msg.payload)?;
    // Process intel...
}
```

### Rust → ChromaDB (Python subprocess)

```rust
// Rust: Query ChromaDB via Python
let results = chromadb.query_tools("nmap reconnaissance", 10).await?;
```

### EEI Satisfaction

```rust
// Query: Find tools that can answer an EEI
let eei_question = "What network scanning tools can detect open ports?";
let satisfiers = chromadb.find_eei_satisfiers(eei_question).await?;
// Returns: tools, tasks, threat_intel that match
```

## Key Files

| File | Purpose |
|------|---------|
| `sx9/crates/sx9-leptose/src/engine.rs` | Main orchestrator |
| `sx9/crates/sx9-leptose/src/chromadb_client.rs` | ChromaDB queries |
| `sx9/crates/sx9-leptose/src/nats_bridge.rs` | NATS messaging |
| `sx9/crates/sx9-glaf-core/src/teth.rs` | TETH entropy |
| `sx9/crates/sx9-foundation-core/src/eei_decision_engine.rs` | EEI decisions |
| `ctas7-command-center/ctas7-intelligence-generator/master_intelligence_orchestrator.py` | OSINT master |
| `sx9/tools/abe/iac/add_to_chromadb_with_unicode.py` | ChromaDB loader |
| `sx9/tools/abe/iac/node-interview-generator/threat_vector_pipeline.py` | Vectorization |

## Next Steps

1. **Wire OSINT to NATS** - Add NATS publishing to Python OSINT scripts
2. **Run ChromaDB population** - Execute `add_to_chromadb_with_unicode.py`
3. **Start Leptose engine** - `cargo run -p sx9-leptose`
4. **Test EEI queries** - Send EEI requests via NATS
5. **Connect to scenarios** - Wire scenario engine to Leptose for intel

