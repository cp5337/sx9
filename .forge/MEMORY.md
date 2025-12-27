# SX9 Session Memory

**Last Updated:** 2025-12-26T22:00:00Z
**Session:** forge-tauri-integration

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
| Kali Tools | `tools/abe/iac/kali_tools_test_10.json` | 335 |
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

### BROKEN (2 crates)
| Crate | Issue |
|-------|-------|
| converge-geometry | Missing modules: earth, enu, intercept |
| converge-selection | Missing modules: partition, laminar, greedy |

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

### Data Alignment
- 335 Kali tools ready
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
