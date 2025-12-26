# SX9 Session Memory

**Last Updated:** 2025-12-26T18:00:00Z
**Session:** forge-tauri-integration

---

## CRITICAL: Two Major Work Bodies

### 1. Tool Run System (Kali + CTAS + MITRE)
**Status:** READY TO RUN - all data aligned

| Component | Location | Records |
|-----------|----------|---------|
| Kali Tools | `tools/abe/iac/kali_tools_test_10.json` | 335 tools |
| CTAS Tasks | `apps/sx9-ops-main/ctas_tasks/` | 164 tasks |
| Tools-Tasks Matching | `archive/glaf-migration/ctas-glaf/import/tools_tasks_matching.json` | 5,065 matches |
| PTCCs | same file | 2,000 |
| Docker Task Server | `apps/sx9-ops-main/docker-builds/kali-tools/` | Port 15178 |
| OSINT Incidents | `06-shuttle-folder/osint_map.csv` | 6,474 records |
| USIMs | `06-shuttle-folder/usims_all.json` | 32,327 records |

**Key Scripts:**
- `tools/abe/iac/match_tools_to_ctas_tasks.py` - Gemini-powered matching
- `apps/sx9-ops-main/docker-builds/kali-tools/task-server.py` - REST API

### 2. Ground Station Network (257 LaserLight FSO)
**Status:** SCRAPER READY - needs execution

| Component | Location | Status |
|-----------|----------|--------|
| Cable Landing Scraper | `tools/abe/iac/cable_landing_scraper.py` | Ready |
| Ground Stations Schema | `crates/sx9-cdn-data-fabric/src/ground_stations.rs` | Built |
| Weather Engine (3 providers) | `crates/sx9-foundation-core/src/weather_val.rs` | Built |
| Collision Avoidance | `crates/sx9-orbital-simulator/src/satellite_simulator.rs` | Built |

**Weather APIs:**
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
| Linear agent | RFC-9030 | Built, not running |
| IACDashboard controls | - | `apps/sx9-ops-main/src/components/glaf/IACDashboard.tsx` - Play/Stop buttons not wired |
| CrateLeaderboard UI | - | Backend in `crate_leaderboard.rs`, no frontend cards |
| Dual Heartbeat UI | RFC-9141 | `sx9-harness/gates/heartbeat_gate.rs` built, no dashboard integration |

---

## RFC Implementation Status

### IMPLEMENTED
- RFC-9001 Trivariate Hashing (sx9-hashing-engine)
- RFC-9020 HD4 Framework (CTAS tasks)
- RFC-9050 QA Two-Heartbeat (sx9-harness gates)
- RFC-9109 Plasma Defender (sx9-plasma-defender)
- RFC-9120 Prompt Forge v4 (sx9-forge Tauri)

### SPEC ONLY (No implementation)
- RFC-9060 Agent Memory Architecture
- RFC-9130 Unified Forge Pipeline
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
| **Forge** | `sx9/sx9-forge` | Dev tasks | - |
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

## Invariants (Do Not Break)

1. Never use Blake3 - only Murmur3 trivariate
2. Foundation crates are canonical dependency source
3. NATS subjects must mirror Redux action types
4. All hashes are 48-char trivariate (H1:H2:H3)
5. GLAF is the graph layer, Three.js/Cesium just renders
