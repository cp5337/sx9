# CTAS-7 Escalation Architecture
## Minimal Startup + NATS + DSL Playbooks

**Principle:** System runs minimal startup and escalates components through NATS messages and DSL playbooks.

---

## Minimal Startup (12 Crates)

**Always Running:**
1. `sx9-gateway` - WebSocket API gateway
2. `sx9-atlas-bus` - PlasmaState, crystal, SDT gate
3. `ctas7-neural-mux` - <250ns routing
4. `ctas7-real-port-manager` - Dynamic port allocation
5. `ctas7-cdn-data-fabric` - Multi-DB aggregation
6. `ctas7-nats-fabric` - NATS messaging
7. `ctas7-foundation-core` - Trivariate hash, PTCC primitives
8. `ctas7-foundation-manifold` - Deterministic routing
9. `ctas7-wasm-primitives` - 32 Universal Primitives
10. `ctas7-cognitive-execution-tool` - Tool execution
11. `tools/kali-plasma/agent` - Kali ISO agent
12. `ctas7-orchestrator` - Smart Crate Orchestrator (basic)

**Databases (Minimal):**
- SurrealDB (core)
- NATS (messaging)
- Sled (local state)

---

## Escalation Components (14 Crates)

**Activate via NATS + DSL Playbooks:**

### Agentic Infrastructure (4 crates)
- `ctas7-agentic-core` → NATS subject: `sx9.escalate.agent`
- `ctas7-persona-core` → NATS subject: `sx9.escalate.persona`
- `ctas7-agent-registry` → NATS subject: `sx9.escalate.registry`
- `ctas7-agent-dispatch` → DSL playbook: `agent-dispatch.yml`

### Cognitive Computing (4 crates)
- `ctas7-cognitive-inference` → NATS subject: `sx9.escalate.cognitive`
- `ctas7-glaf-matroid-core` → NATS subject: `sx9.escalate.glaf`
- `ctas7-gnn-fabric` → NATS subject: `sx9.escalate.gnn`
- `ctas7-foundation-math` → NATS subject: `sx9.escalate.math`

### Intel Systems (2 crates)
- `ctas7-ingestion-pipeline` → DSL playbook: `intel-ingestion.yml`
- `ctas7-yaml-dsl-converter` → DSL playbook: `dsl-convert.yml`

### Defense (1 crate)
- `ctas7-plasma-defender` → NATS subject: `sx9.escalate.threat` (auto-activate on threat)

### Supporting (3 crates)
- `ctas7-foundation-voice` → NATS subject: `sx9.escalate.voice`
- `ctas7-interview-engine` → DSL playbook: `node-interview.yml`
- `ctas7-thalmic-filter` → NATS subject: `sx9.escalate.thalmic`

---

## NATS Escalation Subjects

```
sx9.escalate.agent          → Agent activation
sx9.escalate.persona        → Persona activation
sx9.escalate.registry       → Agent registry activation
sx9.escalate.cognitive      → Cognitive inference activation
sx9.escalate.glaf           → GLAF graph analysis activation
sx9.escalate.gnn            → GNN activation
sx9.escalate.math           → Math operations activation
sx9.escalate.voice          → Voice integration activation
sx9.escalate.thalmic        → Thalmic filter activation
sx9.escalate.threat         → Plasma Defender activation (auto)
```

---

## DSL Playbook Escalation

**Playbook Format:**
```yaml
name: agent-dispatch
trigger: gateway.request.agent
escalate:
  - component: ctas7-agent-dispatch
    condition: request.type == "agent"
    activate: true
  - component: ctas7-agent-registry
    condition: request.type == "agent"
    activate: true
```

**Playbook Locations:**
- `playbooks/agent-dispatch.yml`
- `playbooks/intel-ingestion.yml`
- `playbooks/dsl-convert.yml`
- `playbooks/node-interview.yml`

---

## Escalation Router

**Location:** `ctas7-neural-mux` or new `ctas7-escalation-router`

**Function:**
- Listen for escalation requests
- Route to appropriate NATS subject or DSL playbook
- Activate components on demand
- Track component state (active/inactive)

---

## Consolidated Services

### Unified Agent Registry
**Location:** `ctas7-unified-agent-registry` (new crate or enhance `ctas7-agent-registry`)

**Function:**
- Consolidate all agent implementations
- Single registry for all agents
- Bridge RFC-9107 spec with implementations
- Integrate with PLASMA-ECS

### Intel Gateway
**Location:** `ctas7-intel-gateway` (new crate)

**Function:**
- Route intel requests to appropriate system
- Consolidate Python tools (conda environment)
- API-only access to Python intel generators
- Integrate PTCC-TETH database intelligence

---

## Component Activation Flow

```
Gateway Request
    │
    ▼
Neural Mux Router
    │
    ├──► Minimal Component? → Execute directly
    │
    └──► Escalation Needed?
            │
            ├──► NATS Subject → Publish to subject → Component activates
            │
            └──► DSL Playbook → Execute playbook → Components activate
```

---

## Benefits

1. **Fast Startup** - Only 12 crates start initially
2. **On-Demand Activation** - Components activate when needed
3. **Resource Efficiency** - Don't run everything all the time
4. **Scalability** - Add components without changing core
5. **Flexibility** - Easy to add new escalation paths



