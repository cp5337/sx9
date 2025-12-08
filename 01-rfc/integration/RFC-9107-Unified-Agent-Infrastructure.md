# RFC-9107: Unified Agent Infrastructure

**Version:** 1.0.0
**Status:** Draft
**Date:** December 1, 2025
**Author:** CTAS Core Engineering Group
**Dependencies:** RFC-9001, RFC-9004, RFC-9101, RFC-9030

---

## 1. Abstract

This RFC specifies the unified agent infrastructure for the SX9 platform, consolidating the gRPC agent mesh, voice integration (ElevenLabs), IDE unification, and ABE QA system integration. Agents operate in dual roles: **Operations (Ops)** for tactical mission execution and **Development (Dev)** for engineering workflow automation.

---

## 2. Problem Statement

The CTAS-7 agent system became fragmented during development:

1. **Voice system split**: `ctas7-foundation-voice` contains hollow stubs while actual ElevenLabs implementation resides in `ctas7-voice-bridge`
2. **Broken dependencies**: `ctas7-repoagent` references non-existent path `../Cognitive Tactics Engine/cte-backend/ctas7-playbook-mux`
3. **IDE fragmentation**: No unified MCP configuration across Cursor, VSCode (Antigravity), and Custom GPTs
4. **Agent localization incomplete**: GNN embeddings for geopolitical specialization not fully implemented
5. **QA integration disconnected**: ABE Lightning QA (port 18109) not wired to agent dispatch

---

## 3. Agent Architecture

### 3.1 Elite Agent Mesh (gRPC)

| Port  | Agent   | LLM                | Primary Role (Ops)      | Secondary Role (Dev)     |
|-------|---------|--------------------|-----------------------|--------------------------|
| 50051 | Grok    | Grok-1.5           | Space Engineering     | Starlink Integration     |
| 50052 | Natasha | Claude-3.5-Sonnet  | Red Team Operations   | Voice Command Interface  |
| 50053 | Cove    | GPT-4-Turbo        | DevOps Orchestration  | QA5/XSD Validation       |
| 50054 | Altair  | Claude-GPT-Hybrid  | Space Domain Awareness| SDA Analytics            |
| 50055 | Claude  | Claude-3.5-Sonnet  | Meta-Agent Orchestration | IDE Integration       |
| 50056 | Zoe     | Gemini-1.5-Pro     | Orbital Operations    | CesiumJS Visualization   |
| 50057 | GPT     | GPT-4-Turbo        | Tactical Operations   | Code Generation          |
| 50058 | Elena   | GPT-4-Turbo        | LATAM Intelligence    | Cartel Threat Analysis   |

### 3.2 Dual Role Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     AGENT DUAL-ROLE SYSTEM                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │   OPS MODE  │     │  DISPATCH   │     │  DEV MODE   │       │
│  │             │     │   ROUTER    │     │             │       │
│  │ • Red Team  │◄────┤             ├────►│ • CI/CD     │       │
│  │ • Intel     │     │ RFC-9004    │     │ • Testing   │       │
│  │ • Tactical  │     │ Neural Mux  │     │ • Reviews   │       │
│  └─────────────┘     └──────┬──────┘     └─────────────┘       │
│                             │                                    │
│                    ┌────────▼────────┐                          │
│                    │   ABE QA SYSTEM  │                          │
│                    │   Port 18109     │                          │
│                    │                  │                          │
│                    │ • Lightning QA   │                          │
│                    │ • Code Analysis  │                          │
│                    │ • Threat Scoring │                          │
│                    └──────────────────┘                          │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 Agent Mode Selection

Agents determine operational mode based on:

1. **Request Context**: Trivariate hash prefix indicates Ops (0xC7A5_0000) or Dev (0xC7A5_0100)
2. **Linear Issue Type**: `type:bug` or `type:security` → Ops mode; `type:feature` → Dev mode
3. **Voice Command Prefix**: "Natasha, execute..." → Ops; "Natasha, implement..." → Dev
4. **QA Trigger**: Critical findings → Ops escalation; PR review → Dev mode

---

## 4. ABE QA Integration

### 4.1 QA System Hierarchy (from IAC)

```
Layer 4: Lightning QA Engine (Port 18109) ─────► Linear Issue Creation
    │
    ├── Layer 3: Statistical CDN (Port 18108) ─► Agent Dispatch
    │
    ├── Layer 2: Threat Scoring ─────────────────► Ops Mode Trigger
    │
    └── Layer 1: Code Analysis ─────────────────► Dev Mode Trigger
```

### 4.2 QA-to-Agent Routing

| QA Finding Level | Linear Priority | Agent Mode | Assigned Agent |
|------------------|-----------------|------------|----------------|
| Critical         | P0              | Ops        | Natasha (Red Team) |
| High             | P1              | Ops        | Altair (SDA) |
| Medium           | P2              | Dev        | Cove (DevOps) |
| Low              | P3              | Dev        | GPT (Tactical) |

### 4.3 Integration Points

```rust
// RFC-9107 §4.3: QA-Agent Integration
pub struct QAAgentDispatch {
    pub qa_finding_id: String,
    pub severity: QASeverity,
    pub agent_mode: AgentMode,
    pub assigned_agent: AgentId,
    pub linear_issue_id: Option<String>,
    pub trivariate_hash: TrivariateHash,  // RFC-9001
}

pub enum AgentMode {
    Ops,  // Tactical, Red Team, Intel
    Dev,  // CI/CD, Testing, Reviews
}
```

---

## 5. Voice System (ElevenLabs)

### 5.1 Architecture

```
┌──────────────────────────────────────────────────────────┐
│                  VOICE INTEGRATION                        │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  ctas7-foundation-voice (facade)                         │
│       │                                                   │
│       ├──► ctas7-voice-bridge (implementation)           │
│       │         │                                         │
│       │         ├── ElevenLabsTTS (Port 18260)           │
│       │         ├── AudioCapture                          │
│       │         ├── VoiceActivityDetector                │
│       │         └── VoiceSession                          │
│       │                                                   │
│       └──► Cloudflare Workers (Edge CDN)                 │
│                  │                                        │
│                  └── voice.sx9.io/synthesize             │
│                                                           │
└──────────────────────────────────────────────────────────┘
```

### 5.2 Voice-to-Agent Mapping

| Agent   | Voice ID (ElevenLabs)      | Language   | Domain              |
|---------|----------------------------|------------|---------------------|
| Natasha | EXAVITQu4vr4xnSDxMaL       | Russian    | Geopolitical Intel  |
| Elena   | oWAxZDx7w5VEj9dCyTzz       | Spanish    | Cartel Operations   |
| Zoe     | 21m00Tcm4TlvDq8ikWAM       | English    | Orbital Control     |

### 5.3 Cargo.toml Integration

```toml
# ctas7-foundation-voice/Cargo.toml
[dependencies]
ctas7-voice-bridge = { path = "../ctas7-voice-bridge", optional = true }

[features]
default = ["elevenlabs"]
elevenlabs = ["dep:ctas7-voice-bridge"]
```

---

## 6. IDE Unification

### 6.1 Unified MCP Configuration

```json
{
  "mcpServers": {
    "sx9-agent-mesh": {
      "command": "cargo",
      "args": ["run", "--bin", "ctas7-mcp-connector"],
      "env": {
        "AGENT_MESH_URL": "grpc://localhost:50055",
        "QA_ENDPOINT": "http://localhost:18109"
      }
    },
    "sx9-linear": {
      "command": "node",
      "args": ["ctas7-linear-mcp/dist/index.js"]
    },
    "sx9-voice": {
      "command": "cargo",
      "args": ["run", "-p", "ctas7-voice-bridge", "--bin", "voice-mcp"]
    },
    "sx9-qa": {
      "command": "python",
      "args": ["04-abe-iac/abe-qa-system/lightning-qa-engine/src/lightning_qa_mcp.py"]
    }
  }
}
```

### 6.2 IDE-Specific Configuration

| IDE                | Configuration Location           | Agent Access |
|--------------------|----------------------------------|--------------|
| Cursor             | `.mcp.json` (project root)       | All agents   |
| VSCode (Antigravity)| `.vscode/settings.json`         | All agents   |
| GPT Custom GPT     | OpenAPI Schema                   | Natasha, Zoe |

### 6.3 Custom GPT OpenAPI Specs

| Agent   | Spec File                                               | Primary Endpoint |
|---------|--------------------------------------------------------|------------------|
| Natasha | `ctas7-repoagent/CUSTOM_GPT_NATASHA_ENHANCED_OPENAPI_V7.yaml` | `natasha.sx9.io` |
| Zoe     | `ctas7-repoagent/CUSTOM_GPT_ZOE_ORBITAL_OPENAPI_V7.yaml` | `zoe.sx9.io` |

---

## 7. Agent Localization (GNN)

### 7.1 Embedding Collections (ChromaDB)

| Agent   | Collection                  | Domain                    |
|---------|----------------------------|---------------------------|
| Natasha | `threat_actors_ru`         | Russian APT, GRU, FSB     |
| Elena   | `threat_actors_latam`      | Cartel, MS-13, CJNG       |
| Zoe     | `satellites_constellation` | Orbital mechanics, TLE    |

### 7.2 Persona Registry

File: `ctas7-agent-registry/personas.json`

```json
{
  "natasha": {
    "port": 50052,
    "mode": ["ops", "dev"],
    "domain": "russian_geopolitical",
    "embedding_collection": "threat_actors_ru",
    "voice_id": "EXAVITQu4vr4xnSDxMaL",
    "qa_severity_threshold": "high"
  },
  "elena": {
    "port": 50058,
    "mode": ["ops"],
    "domain": "cartel_operations",
    "embedding_collection": "threat_actors_latam",
    "voice_id": "oWAxZDx7w5VEj9dCyTzz",
    "qa_severity_threshold": "critical"
  },
  "zoe": {
    "port": 50056,
    "mode": ["ops", "dev"],
    "domain": "orbital_operations",
    "embedding_collection": "satellites_constellation",
    "voice_id": "21m00Tcm4TlvDq8ikWAM",
    "orbital_port": 18405,
    "laserlight_port": 18406,
    "qa_severity_threshold": "medium"
  }
}
```

---

## 8. Port Architecture

### 8.1 Complete Port Allocation (RFC-9004 Compliant)

| Port Range    | Purpose                      | RFC Reference |
|---------------|------------------------------|---------------|
| 15180         | Natasha Gateway HTTP         | RFC-9107      |
| 18100         | Neural Mux CDN               | RFC-9004      |
| 18103         | Real Port Manager            | RFC-9004      |
| 18108         | Statistical CDN              | RFC-9004      |
| 18109         | Lightning QA (ABE)           | RFC-9107      |
| 18120         | Linear Gateway               | RFC-9030      |
| 18260         | Voice Bridge (ElevenLabs)    | RFC-9107      |
| 18405         | Orbital Mechanics            | RFC-9107      |
| 18406         | LaserLight MCP               | RFC-9107      |
| 50051-50058   | Agent Mesh (gRPC)            | RFC-9107      |
| 4222          | NATS Event Bus               | RFC-9004      |

---

## 9. Dependency Recovery

### 9.1 Deprecated Dependencies

| Crate                   | Original Path                                           | Status     | Action           |
|------------------------|--------------------------------------------------------|------------|------------------|
| `ctas7-playbook-mux`   | `../Cognitive Tactics Engine/cte-backend/...`          | DEPRECATED | Comment out      |
| `ctas7-atlas-daemon`   | `../ctas7-atlas-daemon/`                               | EXISTS     | Verify path      |
| `ctas7-neural-mux`     | `../ctas7-neural-mux/`                                 | EXISTS     | Verify path      |

### 9.2 Recovery Actions

```toml
# ctas7-repoagent/Cargo.toml - DEPRECATED
# ctas7-playbook-mux = { path = "..." }  # RFC-9107 §9.1: Path no longer exists

# ctas7-foundation-manifold/Cargo.toml - VERIFIED
ctas7-atlas-daemon = { path = "../ctas7-atlas-daemon", optional = true }  # RFC-9107 §9.2
ctas7-neural-mux = { path = "../ctas7-neural-mux", optional = true }      # RFC-9107 §9.2
```

---

## 10. Environment Variables

```bash
# ElevenLabs (RFC-9107 §5)
ELEVEN_KEY=your_key
ELEVEN_VOICE_MODEL=eleven_multilingual_v2

# Linear Integration (RFC-9030)
LINEAR_API_KEY=lin_api_xxx
LINEAR_TEAM_ID=sx9_team

# ABE QA System (RFC-9107 §4)
QA_ENDPOINT=http://localhost:18109
QA_SEVERITY_THRESHOLD=medium

# Cloudflare Edge (RFC-9107 §5.1)
CF_ACCOUNT_ID=xxx
CF_API_TOKEN=xxx
```

---

## 11. Success Criteria

1. ✅ All agent crates compile without errors (Phase 1)
2. ✅ Voice works via ElevenLabs through `foundation-voice` facade (Phase 2)
3. ✅ All IDEs (Cursor, VSCode, GPT) connect to unified agent mesh (Phase 3)
4. ✅ Agents operate in dual Ops/Dev modes based on context (Phase 4)
5. ✅ ABE Lightning QA creates Linear issues and triggers agent dispatch (Phase 5)
6. ✅ GNN-localized embeddings active for Natasha, Elena, Zoe (Phase 6)
7. ✅ Voice synthesis works via Cloudflare edge CDN (Phase 7)

---

## 12. References

- **RFC-9001**: Trivariate Hashing Standard
- **RFC-9004**: Deterministic Routing Architecture
- **RFC-9030**: Unified Linear Agent Infrastructure
- **RFC-9101**: Smart Crate System v7.3.1+

---

## Appendix A: Migration Checklist

- [ ] Comment out `ctas7-playbook-mux` in `ctas7-repoagent/Cargo.toml`
- [ ] Verify `ctas7-atlas-daemon` and `ctas7-neural-mux` paths in `ctas7-foundation-manifold/Cargo.toml`
- [ ] Add `ctas7-voice-bridge` dependency to `ctas7-foundation-voice/Cargo.toml`
- [ ] Create unified `.mcp.json` in project root
- [ ] Save Zoe OpenAPI spec to `ctas7-repoagent/`
- [ ] Create `ctas7-agent-registry/personas.json`
- [ ] Wire QA findings to agent dispatch router

---

**End of RFC-9107**
