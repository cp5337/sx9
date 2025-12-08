# SX9 Cognitive Tools Inventory

> A complete catalog of cognitive engines, algorithms, and systems in the SYNAPTIX9 ecosystem.
> Each tool is defined with its purpose, integration points, and how it connects to the cognitive stack.

---

## Overview: The Cognitive Stack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SX9 COGNITIVE STACK                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   LAYER 8: PRIMITIVES (Foundation Types)                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ PTCC Primitives │ Cognigraph Primitives │ Cognitive Atoms (PTESR-Φ) │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ defines                                       │
│   LAYER 7: SEMANTIC                                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ Trivariate Hash │ Thalmic Filter │ HD4 Phases │ N-V-N-N Structure   │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ encodes                                       │
│   LAYER 6: ANALYTICAL                                                       │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ GLAF Correlation │ Matroid Rank │ Hawkes Process │ TETH Entropy     │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ analyzes                                      │
│   LAYER 5: LEARNING                                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ L-Star Learning │ GNN Patterns │ Behavioral Analysis │ AXON         │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ learns from                                   │
│   LAYER 4: EXECUTION                                                        │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ ATLAS Daemon │ apecs World │ Forge Workflows │ Slot Allocation      │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ executes via                                  │
│   LAYER 3: RESONANCE                                                        │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ Polycrystal │ Crystal Families │ Delta Class │ Ring Strength        │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ authenticates                                 │
│   LAYER 2: CONTROL                                                          │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ SDT Gate │ PlasmaState │ sx9-atlas-bus │ NATS Bridge                │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                              ▲ controls                                      │
│   LAYER 1: HARDWARE                                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │ eBPF/XDP │ Van Allen Entropy │ Biometric Binding │ SIMD Batch       │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## LAYER 8: PRIMITIVES (Foundation Types)

### 1. PTCC - 32 Enhanced Primitives (Universal Validation)

PTCC (Primitive Type Classification Categories) defines the **32 fundamental operations** that can be composed to describe any cyber, kinetic, cognitive, or temporal action. Validated through 1,000,000+ Monte Carlo simulations against modernized DHS 2024-2025 threat scenarios, these primitives form the atomic vocabulary of the SX9 system. Every workflow, every attack chain, every defensive response can be decomposed into sequences of these 32 primitives.

#### Core CRUD (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **CREATE** | Instantiate new entity, resource, or state | Spawn agent, allocate slot, create file |
| **READ** | Retrieve existing data without modification | Query database, observe state, fetch config |
| **UPDATE** | Modify existing entity while preserving identity | Patch config, update hash, evolve state |
| **DELETE** | Remove entity, resource, or state from system | Kill process, deallocate, purge cache |

#### Communication (2 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **SEND** | Transmit data to external destination | Publish NATS, HTTP POST, exfiltrate |
| **RECEIVE** | Accept data from external source | Subscribe NATS, webhook, ingest telemetry |

#### Data Processing (2 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **TRANSFORM** | Convert data format, structure, or encoding | ETL, hash generation, compression |
| **VALIDATE** | Verify data integrity, format, or constraints | Schema check, signature verify, bounds check |

#### Control Flow (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **BRANCH** | Conditional execution path selection | If/else, switch, crystal resonance decision |
| **LOOP** | Repeated execution until condition met | Scan iteration, retry logic, polling |
| **RETURN** | Exit current context with result | Function return, workflow complete, yield |
| **CALL** | Invoke external function or service | RPC, tool execution, agent dispatch |

#### Network Operations (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **CONNECT** | Establish communication channel | TCP handshake, WebSocket, NATS connect |
| **DISCONNECT** | Terminate communication channel | Close socket, logout, session end |
| **ROUTE** | Direct traffic to destination | Load balance, SDT gate, AXON path |
| **FILTER** | Selectively pass or block traffic | Firewall rule, Thalmic suppression, ACL |

#### Security (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **AUTHENTICATE** | Verify identity claims | Login, certificate check, biometric |
| **AUTHORIZE** | Grant or deny access rights | RBAC check, capability verify, ACL lookup |
| **ENCRYPT** | Transform plaintext to ciphertext | TLS, AES, trivariate hash |
| **DECRYPT** | Transform ciphertext to plaintext | Key unwrap, session decrypt, hash decode |

#### Resource Management (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **ALLOCATE** | Reserve resources for use | Slot assignment, memory alloc, port bind |
| **DEALLOCATE** | Release reserved resources | Slot free, memory release, port unbind |
| **LOCK** | Acquire exclusive access | Mutex, database row lock, file lock |
| **UNLOCK** | Release exclusive access | Mutex release, transaction commit |

#### State Management (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **SAVE** | Persist current state to storage | Checkpoint, snapshot, serialize |
| **RESTORE** | Load previously saved state | Rollback, deserialize, recover |
| **CHECKPOINT** | Mark state for potential rollback | Transaction savepoint, version tag |
| **ROLLBACK** | Revert to previous checkpoint | Undo, transaction abort, restore point |

#### Coordination (4 primitives)
| Primitive | Description | Domain Example |
|-----------|-------------|----------------|
| **COORDINATE** | Orchestrate multiple actors/resources | Workflow dispatch, multi-agent sync |
| **SYNCHRONIZE** | Align timing across components | Clock sync, barrier wait, consensus |
| **SIGNAL** | Emit event notification | Interrupt, broadcast, alert trigger |
| **WAIT** | Block until condition or signal | Semaphore wait, event listen, poll |

**Stack Integration**: PTCC primitives are the verbs in Layer 7's N-V-N-N structure (Noun-Verb-Noun-Noun). They map to Layer 4 Forge workflow nodes. Layer 6 GLAF attack chains are sequences of primitives. Layer 3 crystals detect anomalous primitive sequences.

**Validation**: Each primitive is validated against 11 modernized DHS scenarios (Nuclear, Biological, Chemical, Radiological, Cyber, etc.) plus APT campaigns (Volt Typhoon, Salt Typhoon, Chimera) using HMM pattern discovery and latent matroid constraint analysis.

---

### 2. PrimitiveType (Entity Classification)

PrimitiveType is the ontological classification that answers "what kind of thing is this?" Every node in GLAF, every task in a workflow, every entity in the system carries a PrimitiveType. This is distinct from PTCC (which classifies operations/verbs) — PrimitiveType classifies entities/nouns.

| Type | Symbol | Description | Examples |
|------|--------|-------------|----------|
| **Concept** | 𝒞 | Abstract ideas, strategies, methodologies | Attack patterns, defense doctrines, threat models |
| **Actor** | 𝒜 | Human or autonomous entities that act | Operators, agents, threat actors, AI personas |
| **Object** | 𝒪 | Physical or digital artifacts | Tools, files, devices, network assets |
| **Event** | ℰ | Discrete occurrences that change state | Alerts, attacks, logins, transactions |
| **Attribute** | 𝒫 | Properties and qualities | Priority, risk score, confidence, timestamp |
| **Unclassified** | ∅ | Pending classification or hybrid | New data, ambiguous inputs |

**Stack Integration**: PrimitiveType populates the Noun slots in Layer 7's N-V-N-N structure. Layer 6 GLAF nodes are labeled by PrimitiveType. Layer 4 slot allocation prioritizes Actors.

---

### 3. Cognigraph Primitives (Edge Types)

Cognigraph primitives define how entities connect and interact within the cognitive graph. They are the "verbs" of the system—describing flows of information, control, causation, and influence. Every edge in GLAF carries a cognigraph primitive type.

| Primitive | Symbol | Description | Use Case |
|-----------|--------|-------------|----------|
| **Cognitive Flow** | ⟿ | Information and decision pathways where understanding propagates | Analyst reasoning chains, threat correlation paths |
| **Neural Pathway** | ⥤ | Learning and adaptation connections that strengthen with use | Behavioral baselines, pattern reinforcement |
| **Data Stream** | ⇢ | High-volume, low-latency data transfers between components | Telemetry feeds, log aggregation, real-time sync |
| **Force Vector** | → | Physical or logical force relationships (cause → effect) | Attack chains, dependency graphs, kill chains |
| **Interference Pattern** | ⥊ | Conflict, competition, or mutual exclusion relationships | Resource contention, priority conflicts, blocking |
| **Dependency** | ⊸ | Required preconditions (A must exist for B to function) | Tool chains, service dependencies, build order |
| **Convergence** | ⋈ | Multiple inputs combining to produce unified output | Sensor fusion, multi-source intelligence, voting |
| **Resonance** | ∿ | Sympathetic activation (when A fires, B tends to fire) | Crystal families, correlated alerts, cascade triggers |

**Stack Integration**: Cognigraph primitives are encoded in Layer 7's SCH via the N-V-N-N (Noun-Verb-Noun-Noun) structure where the Verb slot maps to the cognigraph type. Layer 6's GLAF uses these as edge types for traversal algorithms. Layer 3's crystal resonance detects ∿ patterns across the graph.

---

### 4. Cognitive Atoms (PTESR-Φ)

Cognitive Atoms are the six-dimensional property framework that gives every node in the system physical grounding. Borrowed from physics simulation, they enable realistic modeling of resource constraints, timing, spatial relationships, and economic costs.

| Dimension | Symbol | Properties | Description |
|-----------|--------|------------|-------------|
| **Physical** | P | mass, resourceCost, energyFootprint | The tangible weight of a component—how much it costs to exist, move, or maintain. A heavy node (high mass) resists change; a light node adapts quickly. |
| **Temporal** | T | activationTime, duration, decayRate | When the component becomes active, how long it persists, and how quickly its effects fade. Critical for scheduling, TTL, and temporal correlation. |
| **Energetic** | E | consumption, generation, threshold | Energy economics—does this node consume resources, generate them, or only activate above a threshold? Enables power-aware orchestration. |
| **Spatial** | S | interactionRadius, exclusionRadius, volume | How far the node's influence extends, what it blocks others from, and how much space it occupies. Essential for geographic and network topology. |
| **Relational** | R | connectivity, dependencies, interactionMatrix | The node's connection profile—how many edges it can support, what it requires, and how it interacts with neighbors. |
| **Economic** | Φ | setupCost, maintenanceCost, opportunityCost | Financial and strategic costs—what it takes to deploy, keep running, and what you give up by choosing this over alternatives. |

**Stack Integration**: Cognitive Atoms populate node properties in Layer 6's GLAF, inform Layer 4's slot allocation (high-P nodes get priority slots), and drive Layer 5's AXON routing (minimize Φ while meeting T constraints).

---

### 4. Universal Node Types

Universal Node Types are the functional roles that components play in workflows. They abstract away domain-specific details to enable cross-industry workflow design.

| Type | Symbol | Description |
|------|--------|-------------|
| **Source** | ⊕ | Emits resources, data, or energy into the system. The origin point of flows. Sensors, APIs, generators. |
| **Sink** | ⊖ | Absorbs waste, output, or terminal state. The end point of flows. Logs, archives, disposal. |
| **Transformer** | ⊗ | Converts inputs to outputs, changing form or meaning. ETL, parsers, encoders. |
| **Router** | ⊛ | Controls directional flow based on conditions. Load balancers, switches, decision points. |
| **Buffer** | ⊞ | Temporarily holds state or resources. Queues, caches, staging areas. |
| **Gate** | ⊠ | Conditional access control. Firewalls, auth checks, SDT thyristors. |
| **Monitor** | ⊙ | Observes system behavior without altering it. SIEM, metrics collectors, probes. |
| **Catalyst** | ⊚ | Accelerates interactions without being consumed. Optimizers, caches, indexes. |
| **Inhibitor** | ⊘ | Blocks or throttles activity. Rate limiters, circuit breakers, kill switches. |
| **Relay** | ⊡ | Extends interaction range. Proxies, repeaters, CDN edges. |

**Stack Integration**: Node types map to Layer 4's Forge workflow nodes, Layer 6's GLAF node labels, and Layer 2's SDT gate logic (Gates use ⊠ behavior).

---

## LAYER 7: SEMANTIC SYSTEMS

### 5. Trivariate Hash (64-bit Base96)

The Trivariate Hash is the DNA of every entity in SX9. Each component is **64 bits extracted from a 128-bit source**, encoded in **Base96** for compact representation. Unlike traditional UUIDs that are random, the trivariate hash is meaningful: you can decode it to understand what an entity is, where it came from, and how it's currently behaving.

| Component | Source | Extracted | Encoding | Description |
|-----------|--------|-----------|----------|-------------|
| **SCH** | 128-bit semantic | **64-bit** | Base96 | Domain + HD4 Phase + N-V-N-N + Delta Angle |
| **CUID** | 128-bit cognitive | **64-bit** | Base96 | Agent + Task + Δθ + Entropy (critical slots) |
| **SX9-UUID** | 128-bit lineage | **64-bit** | Base96 | Origin + Birth TS + Generation (immutable) |

**Total**: 3 × 64-bit = **192 bits** (extracted from 3 × 128-bit = 384-bit sources)

**Base96 Encoding**: Uses 96-character alphabet for ~20% better density than Base64. Each 64-bit component encodes to ~11 characters.

**Canonical Format**: `triv:[SCH]_[CUID]_[SX9-UUID]` (~35 chars total)

**Stack Integration**: Generated at Layer 7, the trivariate hash flows down through every layer. Layer 6 uses it for correlation. Layer 3 crystals resonate against it. Layer 2 SDT frames carry it. Layer 1 eBPF uses the 64-bit components directly as map keys.

---

### 6. SCH - Semantic Content Hash (64-bit from 128-bit)

The SCH compresses meaning into 64 bits **extracted from a 128-bit source** using four 16-bit fields. It answers: "What domain? What phase? What structure? What state?" This enables semantic routing—messages can be filtered by meaning without parsing content. Encoded in **Base96** for compact transmission.

```
┌────────────┬────────────┬────────────┬────────────────────────┐
│  Domain    │ Execution  │   N-V-N-N  │    Delta Angle         │
│  (16 bits) │ (16 bits)  │  (16 bits) │    (16 bits)           │
└────────────┴────────────┴────────────┴────────────────────────┘
```

**Domains**: Cyber (0x10), Geo (0x20), Space (0x30), Maritime (0x40), Fusion (0x50)

**HD4 Phases**: Hunt (0x10), Detect (0x20), Disrupt (0x30), Disable (0x40), Dominate (0x50)

**Stack Integration**: SCH is the primary key for Layer 3 crystal resonance—crystals are tuned to specific domain/phase combinations.

---

### 7. Delta Angle (Δθ)

The Delta Angle measures cognitive drift—how far the current state has deviated from the expected baseline. Expressed as 0-360° (encoded as 0-65535), it's the "temperature" of an entity's behavior. Low delta = predictable. High delta = anomalous or evolving.

| Range | Class | Meaning | System Response |
|-------|-------|---------|-----------------|
| 0-2° | None | Perfect alignment with baseline | Fast-path, no regeneration needed |
| 2-10° | Micro | Minor drift, within tolerance | Tweak CUID slots 10-11 |
| 10-45° | Soft | Noticeable deviation | Regenerate SCH + CUID |
| 45-90° | Hard | Significant behavioral change | Full trivariate regeneration |
| >90° | Critical | Complete divergence from baseline | Supersede lineage, create new entity |

**Stack Integration**: Delta Angle is computed at Layer 6 (behavioral analysis), stored in Layer 7 (SCH and CUID), and drives Layer 3 crystal decisions and Layer 2 SDT gate state.

---

### 8. Thalmic Annotation

Named after the thalamus—the brain's relay station that filters sensory input before it reaches the cortex—Thalmic Annotation provides semantic filtering metadata. It decides what deserves attention and what should be suppressed.

| Field | Range | Purpose |
|-------|-------|---------|
| **Priority** | 0-127 | Processing urgency. Higher = process first. |
| **Confidence** | 0-127 | Source reliability. Higher = more trustworthy. |
| **Suppression** | 0-255 | Filter codes: 0x00=pass, 0x01=duplicate, 0x02=stale, 0xFF=force suppress |
| **Agent Route** | 0-255 | Target agent ID for directed delivery |

**Stack Integration**: Thalmic annotations are applied at Layer 7, evaluated at Layer 3 (crystals check confidence), and enforced at Layer 2 (SDT gates respect suppression).

---

### 9. HD4 Phases (Hunt-Detect-Disrupt-Disable-Dominate)

The HD4 framework is the operational tempo classifier. It answers "what are we trying to do right now?" and aligns all system components to that mission phase. Each phase has distinct tool profiles, risk tolerances, and success metrics.

| Phase | Posture | Description |
|-------|---------|-------------|
| **Hunt** | Proactive | Intelligence gathering, reconnaissance, target development. Quiet, patient, observational. |
| **Detect** | Reactive | Passive monitoring, alert generation, anomaly identification. Watchful, automated, continuous. |
| **Disrupt** | Active | Interference operations, degradation, confusion injection. Noisy, targeted, temporary. |
| **Disable** | Aggressive | Neutralization, takedown, capability destruction. Decisive, irreversible, high-risk. |
| **Dominate** | Total | Full control, persistent access, complete ownership. Sustained, comprehensive, strategic. |

**Stack Integration**: HD4 phase is encoded in Layer 7 SCH, drives Layer 6 tool selection (GLAF scenarios), and configures Layer 4 workflow templates (Forge).

---

## LAYER 6: ANALYTICAL ENGINES

### 10. GLAF - Graph-Lattice Allocation Framework

GLAF is the central nervous system for threat correlation and pattern discovery. It maintains a real-time graph of all entities (nodes) and their relationships (edges), enabling complex queries like "show me all paths from this compromised host to crown jewel assets." Built on SurrealDB with a Rust math engine, GLAF handles 10k+ node graphs with sub-second query times.

| Component | Port | Function |
|-----------|------|----------|
| GLAF Core | 18019 | SurrealDB graph storage |
| GLAF Graph Server | 18050 | Rust math endpoints (matroid, Hawkes, convergence) |
| GLAF Visualizer | 18018 | D3 force simulation UI |

**Stack Integration**: GLAF consumes Layer 8 primitives (PTCC for node types, Cognigraph for edge types), stores Layer 7 hashes, feeds Layer 5 learning, and receives commands via Layer 2 sx9-atlas-bus.

---

### 11. Matroid Rank

Matroid Rank measures the independence of a node set—how much unique capability does this group provide versus redundant coverage? Borrowed from combinatorics, it generalizes linear independence to arbitrary sets. A high matroid rank means each node contributes something the others don't.

**Use Cases**: Agent allocation (avoid sending two experts in the same skill), tool selection (maximize capability diversity), path optimization (minimize redundant hops).

**Stack Integration**: Computed by Layer 6 GLAF Graph Server, used by Layer 4 slot allocation, and informs Layer 5 AXON routing decisions.

---

### 12. Hawkes Process

The Hawkes Process is a self-exciting point process for event prediction. Unlike simple rate models, it captures clustering—events make future events more likely. APT campaigns cluster (one breach leads to lateral movement). Hawkes predicts when the next attack is likely based on the pattern of previous attacks.

**Formula**: λ(t) = μ + Σ α·exp(-β(t - tᵢ)) where μ is baseline intensity, α is excitation, β is decay.

**Stack Integration**: Fed by Layer 7 event timestamps, computed at Layer 6, drives Layer 4 resource pre-positioning and Layer 5 alert prioritization.

---

### 13. TETH - Topological Entropy Threat Heuristic

TETH quantifies threat level through graph topology entropy. A highly ordered graph (predictable structure) has low entropy—normal operations. A chaotic graph (random connections, unusual patterns) has high entropy—potential compromise. TETH combines node, edge, path, and temporal entropy into a single threat score.

**Stack Integration**: Computed continuously at Layer 6, stored as Layer 7 attributes, triggers Layer 3 crystal excitation when thresholds exceeded.

---

## LAYER 5: LEARNING SYSTEMS

### 14. L-Star Learning

L-Star (L*) is an automata learning algorithm that constructs minimal deterministic finite automata (DFAs) from observed behavior. Given a black-box system, L* asks "does this sequence belong to the language?" and "is my hypothesis correct?" until it converges on the simplest model that explains all observations.

**Use Cases**: Protocol reverse engineering (learn the state machine of unknown protocols), behavioral baseline extraction (what's "normal" for this user?), anomaly model construction (define the boundary of acceptable behavior).

**Stack Integration**: L* models are trained on Layer 6 GLAF event sequences, stored as Layer 7 baseline references, and used by Layer 3 crystals to detect deviations.

---

### 15. AXON - Adaptive Execution & Orchestration Network

AXON is the intelligent routing layer that decides how workflows execute. Given a task, AXON considers current system load, agent capabilities, network conditions, and economic costs to select the optimal execution path. It learns from outcomes, improving routing decisions over time.

| Component | Function |
|-----------|----------|
| Route Selector | Choose execution path based on constraints |
| Load Balancer | Distribute work across available agents |
| Failover Handler | Graceful degradation when components fail |
| Learning Loop | Reinforce successful routes, penalize failures |

**Stack Integration**: AXON sits between Layer 6 (receives task analysis) and Layer 4 (dispatches to execution engines), using Layer 8 Cognitive Atoms (PTESR-Φ) to evaluate costs.

---

### 16. GNN Pattern Discovery

Graph Neural Networks (GNNs) learn patterns directly from graph structure. Unlike traditional ML that requires feature engineering, GNNs discover features automatically by aggregating information from node neighborhoods. They excel at finding subtle patterns humans miss.

| Model | Use Case |
|-------|----------|
| GCN (Graph Convolutional Network) | Node classification—what type of threat actor is this? |
| GAT (Graph Attention Network) | Relationship weighting—which connections matter most? |
| GraphSAGE | Inductive learning—classify nodes never seen during training |
| Temporal GNN | Time-aware patterns—detect sequences, not just structure |

**Stack Integration**: GNNs train on Layer 6 GLAF graphs, produce Layer 7 classifications and embeddings, and inform Layer 5 AXON routing.

---

## LAYER 4: EXECUTION ENGINES

### 17. ATLAS Daemon

ATLAS is the high-performance cognitive compute engine—the "GPU" of the SX9 system. Built on Legion ECS (Entity-Component-System) with SIMD acceleration, it handles the hot paths that require sub-microsecond latency: tick synchronization, batch hashing, graph traversal, and matroid calculations.

| Function | Latency |
|----------|---------|
| Tick sync | 250ns |
| Matroid rank | <1μs |
| Graph traversal (10k nodes) | <10μs |
| Batch hashing (1k items) | <100μs |

**Stack Integration**: ATLAS receives commands via Layer 2 sx9-atlas-bus, executes Layer 6 algorithms at hardware speed, and returns results to Layer 5 for learning.

---

### 18. apecs World

apecs is the async I/O companion to ATLAS. While ATLAS handles CPU-bound hot paths, apecs manages I/O-bound operations: database queries, WebSocket connections, file operations, and network calls. Together, they form a hybrid ECS where ATLAS computes and apecs communicates.

**Responsibilities**: Database queries (Supabase, SurrealDB, Sled), WebSocket management, change tracking, event distribution.

**Stack Integration**: apecs communicates with ATLAS via Layer 2 ring buffer, serves Layer 6 GLAF queries, and feeds Layer 5 learning with fresh data.

---

### 19. Forge Workflow Engine

Forge is the Rust-based workflow orchestration engine—think n8n but 10x faster with zero Node.js overhead. Workflows are defined as node graphs (triggers, transforms, outputs) and execute via sx9-atlas-bus with sub-millisecond per-node latency.

| Category | Nodes |
|----------|-------|
| Triggers | Webhook, Schedule, NATS Sub, SDT Gate |
| Databases | Supabase, SurrealDB, Sled, Sledis |
| Transforms | Filter, Map, Hash, Code (WASM) |
| AI/ML | LLM Prompt, Embedding, Classify |
| Outputs | NATS Publish, HTTP, Alert |

**Stack Integration**: Forge workflows are defined using Layer 8 primitives, execute via Layer 2 bus, and produce Layer 7 hashed outputs.

---

### 20. Slot Allocation

Slot Allocation is the matroid-based scheduler that assigns agents to execution slots. Given a set of agents with different capabilities (Layer 8 Cognitive Atoms) and a set of tasks with different requirements, it finds the optimal assignment that maximizes coverage while minimizing redundancy.

**Stack Integration**: Uses Layer 6 matroid rank, respects Layer 8 PTESR-Φ constraints, and feeds Layer 4 ATLAS for execution.

---

## LAYER 3: RESONANCE SYSTEMS

### 21. Polycrystal

Polycrystal is the multi-crystal voting system for physics-based authentication. Instead of binary yes/no decisions, multiple "software crystals" evaluate each payload and vote. Different crystal families are tuned to different threat profiles, and configurable voting policies (Any, All, Majority, Weighted, Quorum) determine the final decision.

| Family | Behavior | Use Case |
|--------|----------|----------|
| AptSilent | Detects APT low-and-slow patterns | Long-term monitoring |
| StableCorporate | Normal business traffic patterns | Enterprise health |
| JitterScripty | Script kiddie detection | Perimeter defense |
| ChattyHoneypot | High-interaction decoy patterns | Threat intel |
| VanAllenCoil | Orbital entropy patterns | Satellite ops |
| OpticalJitter | Laser comm variations | Secure channels |
| MuonSpike | Cosmic ray correlation | Hardware auth |

**Stack Integration**: Crystals evaluate Layer 7 trivariate hashes, produce Layer 3 ring strength, and drive Layer 2 SDT gate decisions.

---

### 22. Ring Strength

Ring Strength is the 0.0-1.0 score produced by crystal resonance. It quantifies "how well does this payload match expected patterns?" High ring strength (≥0.98) means perfect alignment—fast-path processing. Low ring strength (<0.50) means critical deviation—supersede the lineage.

**Stack Integration**: Computed at Layer 3 from Layer 7 hashes, drives Layer 2 SDT state transitions.

---

### 23. Delta Class

Delta Class categorizes the delta angle into actionable buckets. Rather than dealing with continuous 0-360° values, systems respond to discrete classes: None, Micro, Soft, Hard, Critical. Each class triggers specific regeneration and routing behaviors.

**Stack Integration**: Derived from Layer 7 delta angle, used by Layer 3 crystals and Layer 2 SDT gates.

---

## LAYER 2: CONTROL SYSTEMS

### 24. SDT - Software-Defined Thyristor

SDT is a Layer 2 control primitive modeled on the silicon-controlled rectifier (thyristor). Like its hardware counterpart, SDT has four states: Off (blocking), Primed (ready to fire), Conducting (active), and Latched (locked on). Crystal resonance triggers state transitions; only an "anode drop" (lineage supersession) can unlatch.

| State | Description |
|-------|-------------|
| Off | Blocking—no messages pass |
| Primed | Forward-biased, waiting for gate pulse |
| Conducting | Fired—messages flow |
| Latched | Holding current met—stays on until anode drops |

**EtherType**: 0xSD77 (pending IEEE registration)

**Stack Integration**: SDT gates are controlled by Layer 3 ring strength, carry Layer 7 hashes in frames, and execute at Layer 1 via eBPF.

---

### 25. PlasmaState

PlasmaState is the unified cognitive state container—the "consciousness" of each daemon. It holds the current delta angle, entropy level, excitation status, SDT gate state, and polycrystal configuration. Every command that flows through sx9-atlas-bus carries PlasmaState in its DNA.

```rust
pub struct PlasmaState {
    pub delta_angle: u16,      // 0-65535 → 0-360°
    pub entropy: u32,          // Monte Carlo entropy
    pub excited: bool,         // Did crystal ring?
    pub sdt_state: SdtGate,    // Off/Primed/Conducting/Latched
    pub last_ring: Instant,    // Last resonance time
    pub polycrystal: Polycrystal, // Crystal array
}
```

**Stack Integration**: PlasmaState integrates Layer 7 (delta angle), Layer 3 (polycrystal), and Layer 2 (SDT gate) into a single coherent structure.

---

### 26. sx9-atlas-bus

sx9-atlas-bus is the zero-allocation, lock-free ring buffer that connects all daemons. With sub-10ns latency, 64-byte cache-line alignment, and priority lanes (Critical, Urgent, Normal), it's the spinal cord of the cognitive system—carrying commands, results, and state between components.

| Feature | Spec |
|---------|------|
| Latency | <10ns |
| Allocation | Zero (no heap) |
| Cache alignment | 64-byte |
| Priority lanes | Critical, Urgent, Normal |
| Backpressure | Automatic |

**Stack Integration**: sx9-atlas-bus connects all layers, carrying Layer 7 hashes, Layer 3 resonance results, and Layer 4 commands.

---

### 27. NATS Bridge

NATS Bridge extends sx9-atlas-bus to distributed systems. While the ring buffer handles local IPC, NATS JetStream provides reliable, priority-routed messaging across machines. SDT frames are serialized, transmitted, and reassembled with guaranteed ordering.

**Stack Integration**: NATS bridges multiple Layer 2 buses, enabling distributed Layer 4 execution and Layer 6 correlation.

---

## LAYER 1: HARDWARE INTEGRATION

### 28. eBPF/XDP

eBPF (extended Berkeley Packet Filter) with XDP (eXpress Data Path) enables kernel-level packet processing at 5-12ns latency. SDT frames are inspected, trivariate hashes are extracted, and tool triggers are fired—all before packets leave the NIC. This is the invisible layer that makes SX9 operations undetectable.

**Capabilities**: Zero-copy packet inspection, BPF map lookups (8-byte keys from Layer 7 CUID64), tool triggering (nmap, masscan), invisible operation.

**Stack Integration**: eBPF executes Layer 2 SDT logic at wire speed, using Layer 7 hash extractions as map keys.

---

### 29. Van Allen Entropy

Van Allen Entropy harvests randomness from orbital radiation—particle flux from the Van Allen radiation belts detected by satellite sensors. This provides post-quantum entropy that's physically impossible to predict or replicate, enabling hardware-grade authentication.

**Payload Type**: 0x09 (ENTROPY_DRIP)

**Stack Integration**: Van Allen entropy feeds Layer 7 CUID entropy slots and Layer 3 crystal excitation.

---

### 30. Biometric Binding

Biometric Binding ties operations to physical operator identity. Fingerprint, face, voice, keystroke dynamics, and gait analysis create a multi-factor biometric profile that's continuously verified. Operations are cryptographically bound to the operator who initiated them.

| Method | Sensor |
|--------|--------|
| Fingerprint | TouchID, USB reader |
| Face | FaceID, camera |
| Voice | Microphone + voice print |
| Keystroke | Keyboard timing patterns |
| Gait | Accelerometer (mobile) |

**Stack Integration**: Biometric binding produces Layer 7 operator hashes and Layer 1 hardware attestation.

---

## LAYER 9+: AI PERSONAS

### 31. AI Personas

AI Personas are specialized agents with distinct personalities, expertise, and operational styles. They're not just prompts—they're complete cognitive profiles that influence how the system interprets commands, generates responses, and prioritizes actions.

| Persona | Specialty | Style |
|---------|-----------|-------|
| **Volkov** | Tactical operations | Aggressive, direct, red team |
| **Echo** | Analysis & reporting | Methodical, thorough, intelligence |
| **Cipher** | Cryptography | Precise, paranoid, security-first |
| **Sentinel** | Monitoring | Watchful, patient, blue team |
| **Echo_DevOps** | Infrastructure | Practical, automated, reliable |
| **Volkov_Tactical** | Field operations | Mobile, adaptive, situational |

**Stack Integration**: Personas configure Layer 7 Thalmic annotations, Layer 5 AXON routing preferences, and Layer 4 Forge workflow templates.

---

## CROSS-CUTTING: FUSION SYSTEM

### 32. Fusion Nodes (Nonagon)

Fusion Nodes are the unique 9-sided (nonagon) entities representing cross-database correlation. When the same real-world entity exists in Supabase, SurrealDB, and Sled, a Fusion Node links them with a shared SX9-UUID. The nonagon shape (9 sides for SYNAPTIX9) visually distinguishes fused entities in the graph.

**Detection Methods**:
1. **Hash Match**: Trivariate hash comparison across databases
2. **Semantic Match**: LLM analysis of names/descriptions
3. **Graph Structure**: Similar relationship patterns
4. **Temporal Correlation**: Same-time events across sources
5. **Manual Override**: Operator-forced fusion/split

**Fusion Score**: 0.0-1.0 confidence in the correlation.

**Stack Integration**: Fusion uses Layer 6 GLAF for detection, Layer 7 hashes for matching, and creates new Layer 8 entities with Cognigraph ⋈ (Convergence) edges.

---

## Quick Reference: Stack Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         DATA FLOW THROUGH THE STACK                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   INPUT: "Scan network 192.168.1.0/24 for open ports"                       │
│                                                                              │
│   L8: Classify → Actor(Operator) + Object(Network) + Event(Scan)            │
│       Cognigraph: Operator ─[Force Vector]→ Network                         │
│       Atoms: T.duration=300s, E.consumption=high, S.radius=254              │
│                                                                              │
│   L7: Hash → SCH: Cyber|Hunt|scan-network-ports|Δθ=0°                       │
│              CUID: [agent:op1][task:scan][seq:1][ts:now][Δθ:0][H:med]       │
│              SX9-UUID: [origin:cyber][birth:now][parent:0][gen:0]           │
│       Thalmic: Priority=64, Confidence=127, Suppress=0x00                   │
│                                                                              │
│   L6: Analyze → GLAF: Create scan event node, link to network object        │
│                Hawkes: Check if this fits attack pattern                    │
│                TETH: Calculate entropy impact                               │
│                                                                              │
│   L5: Learn → AXON: Route to best available scanner agent                   │
│              L*: Update behavioral baseline for this operator               │
│                                                                              │
│   L4: Execute → Forge: Trigger nmap workflow                                │
│                ATLAS: Batch process target IPs                              │
│                Slot: Allocate execution slot                                │
│                                                                              │
│   L3: Resonate → Polycrystal: StableCorporate + JitterScripty vote          │
│                 Ring Strength: 0.94 (normal scan pattern)                   │
│                 Delta Class: Micro                                          │
│                                                                              │
│   L2: Control → SDT: Primed → Conducting (crystal fired)                    │
│                PlasmaState: Update delta_angle, set excited=true            │
│                Bus: Dispatch to nmap executor                               │
│                                                                              │
│   L1: Execute → eBPF: Insert hash into BPF map                              │
│                nmap: Run scan at kernel level                               │
│                                                                              │
│   OUTPUT: Scan results flow back up, creating new L6 nodes                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Reference Table

| Tool | Layer | Latency | Purpose |
|------|-------|---------|---------|
| PTCC Primitives | 8 | - | Entity classification |
| Cognigraph Primitives | 8 | - | Relationship types |
| Cognitive Atoms | 8 | - | Six-dimensional properties |
| Trivariate Hash | 7 | ~100ns | 3×64-bit Base96 identity |
| Thalmic Filter | 7 | ~50ns | Semantic filtering |
| HD4 Phases | 7 | - | Operational tempo |
| GLAF Correlation | 6 | ~1ms | Threat patterns |
| Matroid Rank | 6 | <1μs | Independence measure |
| Hawkes Process | 6 | ~10μs | Event prediction |
| TETH Entropy | 6 | ~100μs | Threat level |
| L-Star Learning | 5 | varies | Behavior modeling |
| AXON | 5 | ~1ms | Intelligent routing |
| GNN Patterns | 5 | ~10ms | Graph learning |
| ATLAS Daemon | 4 | 250ns | Compute engine |
| apecs World | 4 | ~1ms | Async I/O |
| Forge Workflows | 4 | <1ms/node | Orchestration |
| Polycrystal | 3 | ~10ns | Multi-crystal auth |
| Ring Strength | 3 | ~5ns | Resonance score |
| SDT Gate | 2 | ~2ns | Flow control |
| PlasmaState | 2 | - | Cognitive state |
| sx9-atlas-bus | 2 | <10ns | IPC |
| NATS Bridge | 2 | ~1ms | Distributed IPC |
| eBPF/XDP | 1 | 5-12ns | Kernel processing |
| Van Allen Entropy | 1 | - | Hardware RNG |
| Biometric Binding | 1 | ~100ms | Operator auth |

---

## Document References

| Document | Content |
|----------|---------|
| `SX9-UNIFIED-HASH-SPEC.md` | Hash system, encoding, SDT, crystals |
| `SDT-PROTOCOL-SPEC.md` | Layer 2 protocol details |
| `GLAF-ANALYSIS.md` | Graph analysis system |
| `RFC-9200-DATA-ANALYTICS-WORKBENCH.md` | DAW specification |
| `SX9-MASTER-PLAN.md` | Overall architecture |
| `KALI-PLASMA-ISO-SPEC.md` | Operator platform |
| `V0-SYNAPTIX9-WORKBENCH-SPEC.md` | UI specification |

---

**END OF INVENTORY**
