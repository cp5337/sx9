# SX9 Implementation Plan - December 26, 2025

**Session:** forge-tauri-integration
**Last Updated:** 2025-12-26T22:00:00Z

---

## Three Operational Verticals

| Vertical | App | Port | Domain |
|----------|-----|------|--------|
| **OPS** | sx9-ops-main | 5173 | Threat/Security/CTAS |
| **DEV** | sx9-forge | Tauri | Development/Prompts/Agents |
| **ORBITAL** | sx9-orbital-simulator | - | Space/Ground Stations/FSO |

---

## VERTICAL 1: OPS (Threat Operations)

**App:** `apps/sx9-ops-main` (Port 5173)
**Core Systems:** Plasma-Defender, Kali-Plasma, CTAS Tasks

### 1.1 Tool Run System

| Component | Location | Count |
|-----------|----------|-------|
| Kali Tools | `tools/abe/iac/kali_tools_test_10.json` | 10 (test file; header claims 335 but truncated) |
| CTAS Tasks | `apps/sx9-ops-main/ctas_tasks/` | 164 |
| Tool-Task Matches | `archive/glaf-migration/ctas-glaf/import/tools_tasks_matching.json` | 5,065 |
| PTCCs | same file | 2,000 |
| Docker Task Server | `apps/sx9-ops-main/docker-builds/kali-tools/` | Port 15178 |

### 1.2 HD4 Framework (Hunt-Detect-Disrupt-Disable-Dominate)

**Tool Escalation Pattern:**
```
HUNT         DETECT        DISRUPT       DISABLE       DOMINATE
 │             │              │             │              │
 ▼             ▼              ▼             ▼              ▼
T0:Script    T1:MicroK     T2:Kernel    T3:Crate      T4:Multi-Crate
```

### 1.3 Hourglass Execution (RFC-9026)

```
IDEATION (Wide)    →    BERNOULLI (<50μs)    →    MANAGEMENT (Wide)
LLM Allowed              NO LLMs                   LLM Allowed
Hunt Phase               Detect/Disrupt/Disable    Dominate Phase
```

**Unicode Zones (Private Use Area):**
| Zone | Range | Purpose | Latency |
|------|-------|---------|---------|
| A | U+E000-E3FF | SCH Hash Components | <50μs |
| B | U+E400-EBFF | CUID Slots (0-15) | 50μs-1ms |
| C | U+EC00-EDFF | SDT State / Crystal Family | 1-100ms |
| D | U+EE00-EFFF | Tool Triggers / Responses | >100ms |

### 1.4 Elite Persona System

**Three-Tier Hierarchy:**
```
12 Elite Personas (H2 Semantic)  ─→ Cultural, Strategic, Predictive
        ↓ augments
10 CLSGS Agents (H1 Operational) ─→ Coder, Analyst, SecOps, etc.
        ↓ orchestrates
2,000 PTCCs (Tool Layer)         ─→ Deterministic tool chains
```

**Nonagon Compute (per Elite Persona):**
- Standard: 4 vCPU, 16GB RAM
- Burst HPC: 64 vCPU, 256GB RAM, optional GPU (H100/A100)
- Triggers: 50+ PTCCs, 100k+ graph nodes, <10ms latency

### 1.5 OPS Next Actions

- [ ] Fire stock Kali tools at Plasma-Defender (baseline)
- [ ] Tune Crystals and SDT based on detection results
- [ ] Create micro kernel tool variants (T1 tier)
- [ ] Build HD4 phase scripts
- [ ] Wire IACDashboard Play/Stop buttons

---

## VERTICAL 2: DEV (Development Tooling)

**App:** `sx9-forge` (Tauri Desktop)
**Core Systems:** Prompt Forge, Linear Agent, QA Harness

### 2.1 Forge UI Refactor (RFC-9150)

**Current Problem:** PromptForgeScreen.tsx (1785 lines) - inverted IA, dead code, no state management

**Target: Three-Rail Layout:**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                            HEADER BAR                                    │
│  [Forge Logo]  Mission: ____________  Agent: [Forge ▼]  [Run] [Export]  │
├─────────────┬───────────────────────────────────────┬───────────────────┤
│   INTENT    │           ASSEMBLY CANVAS             │    CONTEXT        │
│   RAIL      │                                       │    RAIL           │
│   (240px)   │           (flex-grow)                 │    (320px)        │
│             │                                       │                   │
│  Mission    │     Variable Binding Zone             │  Drift Score      │
│  Context    │     Template Editor (Monaco)          │  QA Gates         │
│  Variables  │     Preview / Output                  │  Agent Caps       │
│  Templates  │                                       │  RFC Refs         │
│  History    │                                       │                   │
├─────────────┴───────────────────────────────────────┴───────────────────┤
│  [Static ✓] [Arch ✓] [Pattern ⏳] [Semantic ○]  Tokens: 1,234           │
└─────────────────────────────────────────────────────────────────────────┘
```

**Implementation Phases:**
1. State Extraction → Zustand store
2. Component Extraction → IntentRail, AssemblyCanvas, ContextRail
3. API Integration → forge-backend (Port 18350), WebSocket
4. Polish → Accessibility, keyboard nav, responsive

### 2.2 Linear Agent (sx9-linear-agent)

**Status:** Compiles, ready to deploy

| Component | File | Status |
|-----------|------|--------|
| Linear GraphQL | `src/linear/client.rs` | ✅ Complete |
| Agent Loop | `src/agent/agent_loop.rs` | ✅ Complete |
| Slack MCP | `src/mcp/slack.rs` | ✅ Complete |
| Serena MCP | `src/mcp/serena.rs` | ✅ Complete |
| QA Gates | `agent_loop.rs:228` | ⚠️ Wire to sx9-harness |
| PR Creation | `agent_loop.rs:342` | ⚠️ Wire to git |

```bash
cd sx9-linear-agent && cargo build --release
export LINEAR_API_KEY="lin_api_..."
./target/release/sx9-linear-agent
```

### 2.3 QA Harness (sx9-harness)

**Dual Heartbeat System (RFC-9050):**
- Sync Heartbeat: Real-time linting during assembly
- Async Heartbeat: Background QA gate execution

**Gates:** Static → Arch → Pattern → Semantic

**Drift Vectors (RFC-9142):**
- Role Drift (RD)
- Constraint Drift (CD)
- Coupling Drift (CpD)
- Authority Drift (AD)
- Pattern Drift (PD)

### 2.4 DEV Next Actions

- [ ] Create Zustand store for Forge
- [ ] Extract IntentRail, AssemblyCanvas, ContextRail components
- [ ] Deploy sx9-linear-agent
- [ ] Wire QA gates to agent loop
- [ ] Connect Memory System session hooks (RFC-9060)

---

## VERTICAL 3: ORBITAL (Space Domain)

**Crates:** sx9-orbital-simulator, sx9-cdn-data-fabric
**Core Systems:** Ground Stations, Weather Engine, Satellite Simulation

### 3.1 Ground Station Network (257 LaserLight FSO)

| Component | Location | Status |
|-----------|----------|--------|
| Cable Landing Scraper | `tools/abe/iac/cable_landing_scraper.py` | Ready |
| Ground Stations Schema | `crates/sx9-cdn-data-fabric/src/ground_stations.rs` | Built |
| Weather Engine | `crates/sx9-foundation-core/src/weather_val.rs` | Built |
| Collision Avoidance | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | Built |

### 3.2 Weather API Providers

| Provider | Coverage | Cost | Status |
|----------|----------|------|--------|
| Open-Meteo | Global | Free | Active |
| WeatherAPI.com | Global | Freemium | Needs key |
| NOAA | US Only | Free | Active |

### 3.3 FSO Scoring

Free-Space Optical link availability depends on:
- Cloud cover percentage
- Visibility (km)
- Precipitation
- Atmospheric turbulence

### 3.4 ORBITAL Next Actions

- [ ] Run cable landing scraper to populate 257 stations
- [ ] Integrate weather engine with orbital simulator
- [ ] Build FSO availability scoring algorithm
- [ ] Create ground station ranking system

---

## Cross-Vertical Infrastructure

### Crate Status

**Compiles (Core):**
sx9-foundation-core, sx9-foundation-data, sx9-harness, sx9-claude-sdk,
sx9-gateway-primary, sx9-glaf-core, sx9-orbital-simulator, sx9-cdn-data-fabric,
sx9-atlas-bus, sx9-plasma-defender, 25+ more

**Converge Crates (in 04-sx9-converge/):**
| Crate | Location | Status |
|-------|----------|--------|
| sx9-converge | `04-sx9-converge/sx9-converge` | Compiles |
| sx9-converge-geometry | `04-sx9-converge/sx9-converge-geometry` | Compiles |
| sx9-converge-selection | `04-sx9-converge/sx9-converge-selection` | Compiles |

**Disconnected (Built but not wired):**
| Component | RFC | Status |
|-----------|-----|--------|
| Memory System | RFC-9060 | `.forge/` created, needs session hook |
| sx9-claude-sdk | RFC-9145 | Built, not wired |
| sx9-harness agents | - | Registry exists, not invoked |
| NATS subjects | RFC-9400 | Defined, daemon not connected |

### Port Architecture

| Service | Port | Vertical |
|---------|------|----------|
| ops-main | 5173 | OPS |
| development-center | 5174 | DEV (legacy) |
| forge-backend | 18350 | DEV |
| Gateway | 18600 | Cross |
| Kali Task Server | 15178 | OPS |
| Hashing Engine | 8002 | Cross |

### Secrets Management

**Gitignored:**
- `tools/vault/*.json` - API vault
- `06-shuttle-folder/` - Staging data with embedded tokens

**Actions Completed Today:**
- Purged secrets from git history (filter-branch)
- Added shuttle folder to .gitignore permanently
- Force pushed clean history

---

## Today's Accomplishments (Dec 26, 2025)

### Code Changes
- Added Nonagon compute nodes to all 12 Elite Personas
- Added Unicode short codes to `plasma.rs` (SDT: U+EC00) and `crystal.rs` (Crystal: U+ED00)
- Created RFC-9026 Hourglass-Bernoulli Cognitive Architecture
- Aligned Unicode allocations across RFC-9026, kali-plasma, atlas-bus

### Data Alignment
- 10 Kali tools in test file (full 335-tool scrape needed)
- 164 CTAS tasks ready
- 5,065 tool-task matches ready
- 2,000 PTCCs ready
- Unicode zones A-D fully specified

### Infrastructure
- Removed shuttle folder from git history (103K+ lines purged)
- Cleaned secrets from all commits

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

**Document Version:** 2.0
**Author:** Claude Code + cp5337
**Next Review:** 2025-12-27
