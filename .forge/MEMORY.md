# SX9 Session Memory

**Last Updated:** 2025-12-26T23:30:00Z
**Session:** forge-tauri-integration

---

## Neo4j Graph Database (OrbStack)

**Container:** `ctas7-neo4j`
**Runtime:** OrbStack (k8s deprecated 1+ year)
**Auth:** `neo4j` / `Protected1`
**Ports:** 7474 (HTTP), 7687 (Bolt)

### Node Counts (VERIFIED)
| Label | Count |
|-------|-------|
| Tool | 2,365 |
| Term | 2,225 |
| PTCC | 2,000 |
| Technique | 1,073 |
| KaliTool | 347 |
| CTASTask | 164 |
| ElitePersona | 12 |
| HD4Phase | 5 |

**Access:**
```bash
docker exec -it ctas7-neo4j cypher-shell -u neo4j -p Protected1
MATCH (n) RETURN labels(n)[0] as label, count(*) as count ORDER BY count DESC;
```

---

## Three Operational Verticals

| Vertical | App | Port | Domain |
|----------|-----|------|--------|
| **OPS** | sx9-ops-main | 5173 | Threat/Security/CTAS |
| **DEV** | sx9-forge | Tauri | Development/Prompts/Agents |
| **ORBITAL** | sx9-orbital-simulator | - | Space/Ground Stations/FSO |

**Full Plan:** `01-rfc/IMPLEMENTATION-PLAN-2025-12-26.md`

---

## OPS Vertical (Threat Operations)

### Tool Run System
| Component | Location | Count |
|-----------|----------|-------|
| Kali Tools | `tools/abe/iac/kali_tools_test_10.json` | 10 (test file; header claims 335 but truncated) |
| CTAS Tasks | `apps/sx9-ops-main/ctas_tasks/` | 164 |
| Tool-Task Matches | `archive/glaf-migration/ctas-glaf/import/tools_tasks_matching.json` | 5,065 |
| PTCCs | same file | 2,000 |
| Docker Task Server | `apps/sx9-ops-main/docker-builds/kali-tools/` | Port 15178 |
| OSINT Incidents | `06-shuttle-folder/osint_map.csv` | 6,474 records |
| USIMs | `06-shuttle-folder/usims_all.json` | 32,327 records |

**Key Scripts:**
- `tools/abe/iac/match_tools_to_ctas_tasks.py` - Gemini-powered matching
- `apps/sx9-ops-main/docker-builds/kali-tools/task-server.py` - REST API

### HD4 + Tool Escalation
```
HUNT(T0:Script) → DETECT(T1:MicroK) → DISRUPT(T2:Kernel) → DISABLE(T3:Crate) → DOMINATE(T4:Multi-Crate)
```

### Hourglass Execution (RFC-9026)
```
IDEATION (Wide/LLM) → BERNOULLI (<50μs/NO LLM) → MANAGEMENT (Wide/LLM)
Hunt Phase            Detect/Disrupt/Disable      Dominate Phase
```

### Unicode Zones (Private Use Area)
| Zone | Range | Purpose | Latency |
|------|-------|---------|---------|
| A | U+E000-E3FF | SCH Hash Components | <50μs |
| B | U+E400-EBFF | CUID Slots (0-15) | 50μs-1ms |
| C | U+EC00-EDFF | SDT State / Crystal Family | 1-100ms |
| D | U+EE00-EFFF | Tool Triggers / Responses | >100ms |

### Elite Personas (12) + CLSGS Agents (10) + PTCCs (2000)
- Nonagon compute: Standard 4vCPU/16GB, Burst HPC 64vCPU/256GB/GPU
- Configuration: `antigravity/clsgs_elite.toml`

---

## DEV Vertical (Development Tooling)

### Forge UI (RFC-9150)
Three-Rail Layout: Intent Rail (240px) | Assembly Canvas (flex) | Context Rail (320px)
**Current:** PromptForgeScreen.tsx (1785 lines) - inverted IA, dead code, no state management

### Linear Agent (sx9-linear-agent)
**Status:** Compiles, ready to deploy

| Component | File | Status |
|-----------|------|--------|
| Linear GraphQL | `src/linear/client.rs` | ✅ Complete |
| Agent Loop | `src/agent/agent_loop.rs` | ✅ Complete |
| Slack MCP | `src/mcp/slack.rs` | ✅ Complete |
| Serena MCP | `src/mcp/serena.rs` | ✅ Complete |
| QA Gates | `agent_loop.rs:228` | ⚠️ Placeholder → wire to sx9-harness |
| PR Creation | `agent_loop.rs:342` | ⚠️ Placeholder → wire to git |

**To deploy:**
```bash
cd sx9-linear-agent && cargo build --release
export LINEAR_API_KEY="lin_api_..."
export SLACK_BOT_TOKEN="xoxb-..."
export SERENA_ENDPOINT="http://localhost:8080"
./target/release/sx9-linear-agent
```

### QA Harness (sx9-harness)
- Dual Heartbeat: Sync (real-time linting) + Async (background QA)
- Gates: Static → Arch → Pattern → Semantic
- Drift Vectors: RD, CD, CpD, AD, PD (RFC-9142)

### Disconnected UI Components
| Component | Location | Status |
|-----------|----------|--------|
| IACDashboard controls | `apps/sx9-ops-main/src/components/glaf/IACDashboard.tsx` | Play/Stop buttons not wired |
| CrateLeaderboard UI | Backend in `crate_leaderboard.rs` | No frontend cards |
| Dual Heartbeat UI | `sx9-harness/gates/heartbeat_gate.rs` | No dashboard integration |

---

## ORBITAL Vertical (Space Domain)

### Ground Station Network (257 LaserLight FSO)
| Component | Location | Status |
|-----------|----------|--------|
| Cable Landing Scraper | `tools/abe/iac/cable_landing_scraper.py` | Ready |
| Ground Stations Schema | `crates/sx9-cdn-data-fabric/src/ground_stations.rs` | Built |
| Weather Engine (3 providers) | `crates/sx9-foundation-core/src/weather_val.rs` | Built |
| Collision Avoidance | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | Built |

### Weather APIs
- Open-Meteo (free, global)
- WeatherAPI.com (key: `WEATHER_API_KEY` env)
- NOAA (free, US only)

---

## Crate Status Audit

### COMPILES (Core)
- sx9-foundation-core
- sx9-foundation-data
- sx9-harness
- sx9-claude-sdk
- sx9-gateway-primary
- sx9-glaf-core
- sx9-orbital-simulator
- sx9-cdn-data-fabric
- sx9-atlas-bus
- sx9-plasma-defender
- 25+ more crates

### CONVERGE CRATES (sx9-converge/)
| Crate | Location | Status |
|-------|----------|--------|
| sx9-converge | `sx9-converge/` | Compiles |
| sx9-converge-geometry | `sx9-converge/geometry/` | Compiles |
| sx9-converge-selection | `sx9-converge/selection/` | Compiles |

### DISCONNECTED (Built but not wired)
| Component | RFC | Status |
|-----------|-----|--------|
| Memory System | RFC-9060 | `.forge/` created, needs session hook |
| sx9-claude-sdk | RFC-9145 | Built, not wired to Claude sessions |
| sx9-harness agents | - | Registry exists, not invoked |
| NATS subjects | RFC-9400 | Defined, daemon not connected |

---

## RFC Implementation Status

### IMPLEMENTED
- RFC-9001 Trivariate Hashing (sx9-hashing-engine)
- RFC-9020 HD4 Framework (CTAS tasks)
- RFC-9026 Hourglass-Bernoulli (atlas-bus, kali-plasma)
- RFC-9050 QA Two-Heartbeat (sx9-harness gates)
- RFC-9109 Plasma Defender (sx9-plasma-defender)
- RFC-9120 Prompt Forge v4 (sx9-forge Tauri)

### SPEC ONLY (No implementation)
- RFC-9060 Agent Memory Architecture
- RFC-9130 Unified Forge Pipeline
- RFC-9150 Prompt Forge UI Specification
- RFC-9400 Gateway NATS Architecture
- RFC-9876 Layer-Two Unicode Orchestration

---

## Complete RFC Registry (64 RFCs)

### 9000-CORE (11 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9000 | JSON Metadata Schema | Spec |
| 9001 | Trivariate Hash | Implemented |
| 9002 | Entity Types | Spec |
| 9003 | Graph Entity Schema | Spec |
| 9005 | Unified Schema Spec | Spec |
| 9010 | Contextual Security | Spec |
| 9011 | Compound Unique Hash | Spec |
| 9015 | Prime Hash Evolution | Spec |
| 9016 | Short Code Spec | Spec |
| 9020 | HD4 Framework | Implemented |
| 9116 | Bootstrap Sequence | Spec |

### 9000-FOUNDATION (14 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9010 | Universal Rail System | Spec |
| 9010-Figma | Figma Spec | Design |
| 9010-Layout | Layout Diagram | Design |
| 9023 | SDT Threat Taxonomy | Spec |
| 9024 | GLAF Core | Spec |
| 9025 | Smart Crate Orchestrator | Spec |
| 9026 | Hourglass-Bernoulli | Implemented |
| 9050 | Two-Heartbeat QA | Implemented |
| 9060 | Agent Memory Arch | Spec |
| 9070 | Agent Routing | Spec |
| 9200 | ATLAS Mesh Arch | Spec |
| 9300 | Orbital Ground Stations | Spec |
| 9400 | Gateway NATS | Spec |
| 9500 | Orbital Subsystems | Spec |

### 9100-INTEGRATION (14 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9030 | Intent-Based Build | Spec |
| 9040 | GLAF Analysis | Spec |
| 9050 | QA Harness | Implemented |
| 9060 | Agent Memory | Spec |
| 9070 | Agent Routing | Spec |
| 9080 | Agent Skills | Spec |
| 9109 | Plasma Defender | Implemented |
| 9120 | Prompt Forge v4 | Implemented |
| 9130 | Unified Pipeline | Spec |
| 9131 | Intent Compiler | Spec |
| 9141 | BNE Workflow | Spec |
| 9142 | Drift Vectors | Spec |
| 9143 | QA Gate Types | Spec |
| 9145 | Agent SDK Manual | Spec |

### 9110-FORGE (10 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9120 | Prompt Forge | Implemented |
| 9130 | Unified Pipeline | Spec |
| 9131 | Intent Compiler | Spec |
| 9140 | Dev Sprint Kernel | Spec |
| 9141 | BNE Workflow | Spec |
| 9142 | Drift Vectors | Spec |
| 9143 | QA Gate Types | Spec |
| 9145 | Agent SDK Manual | Spec |
| 9150 | Forge UI Spec | Design |
| 9150-Critical | Critical Analysis | Analysis |

### 9300-COGNITIVE (3 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9023 | SDT Taxonomy | Spec |
| 9024 | GLAF Core | Spec |
| 9025 | Smart Crate Orchestrator | Spec |

### 9400-APPLICATION (3 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9150 | Forge UI | Design |
| 9300 | Orbital Stations | Spec |
| 9304B | Cognitive Atoms | Spec |

### 9500-PLATFORM (1 RFC)
| RFC | Title | Status |
|-----|-------|--------|
| 9200 | ATLAS Mesh | Spec |

### 9800-OPERATIONAL (3 RFCs)
| RFC | Title | Status |
|-----|-------|--------|
| 9130 | Pipeline | Spec |
| 9131 | Intent Compiler | Spec |
| 9876 | Unicode Orchestration | Spec |

---

## Repository Structures

### sx9 (This Repo)
```
sx9/
├── 01-rfc/              # 64 RFCs across 8 directories
├── sx9-converge/        # Geometry/selection crates (COMPILE)
├── 06-shuttle-folder/   # IGNORED (secrets purged)
├── apps/
│   └── sx9-ops-main/    # OPS vertical (threat dashboard)
├── crates/              # 33 Rust crates
├── packages/            # JS workspace
├── sx9-forge/           # Tauri desktop app
├── sx9-linear-agent/    # Autonomous agent
├── tools/
│   ├── abe/             # IAC tools
│   ├── forge-backend/   # Port 18350
│   └── kali-plasma/     # Kali integration
└── antigravity/         # Elite Personas + CLSGS
```

### sx9-development-center (Separate Repo)
```
sx9-development-center/
├── src/
│   ├── components/      # 67 React components
│   │   ├── SmartCrateControl.tsx  # 950 lines - crate dashboard
│   │   ├── TacticalHUD.tsx        # Tactical display
│   │   ├── AgentDesignStudio.tsx  # 30.7KB - agent builder
│   │   └── ...
│   ├── services/        # 27 services
│   └── hooks/           # 13 custom hooks
├── crates/
│   ├── sx9-phd-analyzer/
│   ├── sx9-smart-crate-orchestrator/
│   └── sx9-forge/
└── docs/                # 148+ documentation files
```

---

## Known Data Losses

| Item | Impact |
|------|--------|
| ctas7-command-center | ~15 Rust crates with Cesium integration |
| GEE needle extractor | Python script for cable landing coords |

---

## App / Task Domain Architecture

| App | Repo | Tasks | Port |
|-----|------|-------|------|
| **Forge** | `sx9/sx9-forge` | Dev tasks | Tauri |
| **ops-main** | `sx9/apps/sx9-ops-main` | CTAS tasks (threat/security) | 5173 |
| **orbital** | `sx9/crates/sx9-orbital-simulator` | Orbital tasks (space domain) | - |
| **development-center** | `sx9-development-center` (SEPARATE REPO) | Legacy crate management | 5174 |

### Key Components in development-center (DO NOT LOSE)
- `SmartCrateControl.tsx` - 950-line dashboard with crate cards, Docker status, health checks
- `TacticalHUD.tsx` - tactical display
- `CTASCrateManagement.tsx` - crate management

### Gateway Endpoints (port 18600)
- `/health` - basic connection status
- `/qa/heartbeat` - dual heartbeat from sx9-harness
- `/ws` - WebSocket for real-time

---

## Port Architecture

| Service | Port | Vertical |
|---------|------|----------|
| ops-main | 5173 | OPS |
| development-center | 5174 | DEV (legacy) |
| forge-backend | 18350 | DEV |
| Gateway | 18600 | Cross |
| Kali Task Server | 15178 | OPS |
| Hashing Engine | 8002 | Cross |

---

## Today's Accomplishments (Dec 26, 2025)

### Code Changes
- Added Nonagon compute nodes to all 12 Elite Personas
- Added Unicode short codes to `plasma.rs` (SDT: U+EC00) and `crystal.rs` (Crystal: U+ED00)
- Created RFC-9026 Hourglass-Bernoulli Cognitive Architecture
- Aligned Unicode allocations across RFC-9026, kali-plasma, atlas-bus
- Purged secrets from git history (shuttle folder with embedded tokens)
- Added `06-shuttle-folder/` to .gitignore permanently

### Data Alignment (VERIFIED)
- 10 Kali tools in test file (`kali_tools_test_10.json` - header claims 335 but truncated)
- 347 KaliTool nodes in Neo4j (full set)
- 164 CTAS tasks ready
- 5,065 tool-task matches ready
- 2,000 PTCCs ready
- Unicode zones A-D fully specified

---

## Invariants (Do Not Break)

1. Never use Blake3 - only Murmur3 trivariate
2. Foundation crates are canonical dependency source
3. NATS subjects must mirror Redux action types
4. All hashes are 48-char trivariate (H1:H2:H3)
5. GLAF is the graph layer, Three.js/Cesium just renders
6. Bernoulli Zone is <50μs, no LLMs allowed
7. Unicode Private Use Area allocations are fixed

---

## Next Actions

**OPS:**
- [ ] Fire stock Kali tools at Plasma-Defender (baseline)
- [ ] Tune Crystals and SDT based on results
- [ ] Create micro kernel tool variants (T1)
- [ ] Build HD4 phase scripts
- [ ] Wire IACDashboard Play/Stop buttons

**DEV:**
- [ ] Create Zustand store for Forge
- [ ] Extract IntentRail, AssemblyCanvas, ContextRail components
- [ ] Deploy sx9-linear-agent
- [ ] Wire QA gates to agent loop
- [ ] Connect Memory System session hooks (RFC-9060)

**ORBITAL:**
- [ ] Run cable landing scraper to populate 257 stations
- [ ] Integrate weather engine with orbital simulator
- [ ] Build FSO availability scoring algorithm
