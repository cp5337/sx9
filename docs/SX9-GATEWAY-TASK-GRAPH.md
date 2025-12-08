# SX9 Gateway Task Graph

**Version:** 1.0  
**Status:** Planning  
**Date:** December 2025  
**Domain:** Gateway Infrastructure  
**HD4 Phase:** Hunt → Detect → Disrupt → Disable → Dominate  

---

## Overview

This document defines the SX9 Gateway as a **task graph** — each task is a node with:
- **Predecessors/Successors** — Graph relationships
- **HD4 Phase** — Operational phase classification
- **PTCC Primitive** — Universal validation primitive
- **PTH Metrics** — Probability, Time, Hazard factors
- **Trivariate Hash** — SCH + CUID + SX9-UUID identity
- **Bernoulli Zone** — Latency classification
- **Port Manager Integration** — Uses `ctas7-real-port-manager` (port 18104) as port of record

---

## Port Manager Integration

**Canonical Port Manager:** `ctas7-real-port-manager` (version 7.2.0)  
**Port:** 18104 (canonical)  
**Status:** ✅ **Port of Record** — All gateway port allocations must go through this service

**Architecture:** Rust equivalent of nginx for port management — dynamic port blocks, mirrored ports, deception settings

**Integration Points:**
- Dynamic port allocation for CDN services
- Service priority-based port assignment
- Security context-aware port management
- Port conflict resolution
- Port usage tracking and reporting
- **Mirror port allocation** (load balancing, failover, deception)
- **Deception settings** (stealth mode, fake ports, decoy services)

**Port Blocks (Dynamic, Not Traditional Numbers):**
- **Orbital Blocks:** 18120-18139 (critical services, gateway core, neural mux)
- **CDN Blocks:** 18140-18159 (high-bandwidth streams, CDN services)
- **Neural Mesh Blocks:** 18160-18179 (adaptive, low-latency communication)

**Mirror Port System:**
- Each primary port can have multiple mirror ports
- Mirror types: LoadBalancing, Failover, Deception, Stealth
- Automatic mirror allocation on port assignment

**Note:** The current implementation still uses traditional port numbers. Future enhancement: **Hash-based port allocation** using trivariate hashes to eliminate traditional port numbers entirely.

---

## Task Graph Structure

### Graph Node Format

```typescript
interface GatewayTask {
  // Core Identification
  hash_id: string;              // Trivariate hash identifier
  task_name: string;             // Human-readable name
  description: string;           // Detailed description
  
  // Classification
  category: string;              // Task category/domain
  hd4_phase: HD4Phase;           // Hunt/Detect/Disrupt/Disable/Dominate
  primitive_type: PTCCPrimitive; // CREATE/READ/TRANSFORM/etc.
  
  // Graph Relationships
  predecessors: string[];        // Array of predecessor hash_ids
  successors: string[];         // Array of successor hash_ids
  
  // PTH Metrics
  p_probability: number;         // 0.0-1.0 (success probability)
  t_time: number;               // 0.0-1.0 (time factor)
  h_hazard: number;             // 0.0-1.0 (risk factor)
  
  // Operational Context
  bernoulli_zone: BernoulliZone; // Tactical/Operational/Analytical/Infrastructure
  port_manager_required: boolean; // Whether port allocation needed
  port_allocation: number | null; // Allocated port (if applicable)
  
  // Trivariate Hash
  sch_hash: string;             // Semantic Content Hash (64-bit Base96)
  cuid_hash: string;            // Cognitive Unique Identifier (64-bit Base96)
  sx9_uuid: string;            // Immutable Lineage Anchor (64-bit Base96)
  
  // Sequencing
  task_seq: number;            // Sequential ordering
}
```

---

## Task Graph Nodes

### Layer 0: Foundation (Root Nodes)

#### TASK-001: Initialize Gateway State
- **hash_id:** `gateway-001-init-state`
- **task_name:** Initialize Gateway State
- **description:** Create GatewayState struct, initialize connection statuses, CDN configs, Neural Mux state
- **category:** Foundation
- **hd4_phase:** Hunt
- **primitive_type:** CREATE
- **predecessors:** []
- **successors:** [`gateway-002-connect-db`, `gateway-003-connect-nats`, `gateway-004-init-port-manager`]
- **p_probability:** 0.99
- **t_time:** 0.10
- **h_hazard:** 0.01
- **bernoulli_zone:** Infrastructure
- **port_manager_required:** false
- **task_seq:** 1

#### TASK-002: Connect to SurrealDB
- **hash_id:** `gateway-002-connect-db`
- **task_name:** Connect to SurrealDB
- **description:** Establish WebSocket connection to SurrealDB (port 18010), authenticate, select namespace/database
- **category:** Database
- **hd4_phase:** Hunt
- **primitive_type:** CONNECT
- **predecessors:** [`gateway-001-init-state`]
- **successors:** [`gateway-010-query-handler`, `gateway-011-workflow-handler`]
- **p_probability:** 0.95
- **t_time:** 0.30
- **h_hazard:** 0.05
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **port_allocation:** 18010
- **task_seq:** 2

#### TASK-003: Connect to NATS
- **hash_id:** `gateway-003-connect-nats`
- **task_name:** Connect to NATS
- **description:** Establish connection to NATS server (port 18020), subscribe to health check subjects
- **category:** Messaging
- **hd4_phase:** Hunt
- **primitive_type:** CONNECT
- **predecessors:** [`gateway-001-init-state`]
- **successors:** [`gateway-012-health-subscriber`, `gateway-013-plasma-subscriber`]
- **p_probability:** 0.95
- **t_time:** 0.30
- **h_hazard:** 0.05
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **port_allocation:** 18020
- **task_seq:** 3

#### TASK-004: Initialize Port Manager Client (Crystal-Gated)
- **hash_id:** `gateway-004-init-port-manager`
- **task_name:** Initialize Port Manager Client with Crystal/Thyristor Integration
- **description:** Connect to `ctas7-real-port-manager` at port 18104, initialize `CrystalGatedPortManager` with `PlasmaState`, `Polycrystal`, and `ThyristorConfig`. Port allocations are now gated by crystal resonance and SDT state.
- **category:** Infrastructure
- **hd4_phase:** Hunt
- **primitive_type:** CONNECT
- **predecessors:** [`gateway-001-init-state`]
- **successors:** [`gateway-005-allocate-cdn-ports`, `gateway-006-reserve-gateway-port`]
- **p_probability:** 0.98
- **t_time:** 0.25
- **h_hazard:** 0.02
- **bernoulli_zone:** Infrastructure
- **port_manager_required:** true
- **port_allocation:** 18104
- **crystal_integration:** ✅ **Required** — Port allocations must pass crystal resonance check
- **task_seq:** 4

---

### Layer 1: Port Allocation

#### TASK-005: Allocate CDN Ports (Crystal-Gated)
- **hash_id:** `gateway-005-allocate-cdn-ports`
- **task_name:** Allocate CDN Ports with Crystal Resonance
- **description:** Request port allocations for all 8 CDNs using `allocate_port_gated()`. Each CDN service hash is evaluated by crystal resonance. Ring strength determines mirror count (0.98+ = 3 mirrors, 0.90+ = 2 mirrors, 0.75+ = 1 mirror). Only allocated if SDT gate is Conducting or Latched.
- **category:** Infrastructure
- **hd4_phase:** Detect
- **primitive_type:** ALLOCATE
- **predecessors:** [`gateway-004-init-port-manager`]
- **successors:** [`gateway-014-cdn-health-checker`]
- **p_probability:** 0.90
- **t_time:** 0.40
- **h_hazard:** 0.10
- **bernoulli_zone:** Infrastructure
- **port_manager_required:** true
- **port_allocation:** null (multiple ports, dynamic based on crystal)
- **crystal_integration:** ✅ **Required** — Each CDN port allocation requires crystal resonance check
- **task_seq:** 5

#### TASK-006: Reserve Gateway Port (Crystal-Gated)
- **hash_id:** `gateway-006-reserve-gateway-port`
- **task_name:** Reserve Gateway Port with Crystal Resonance
- **description:** Reserve gateway port using `allocate_port_gated()` with service hash. Crystal resonance determines if allocation is allowed. If ring strength ≥ 0.98, port is latched (permanent). Mirrors allocated based on ring strength.
- **category:** Infrastructure
- **hd4_phase:** Detect
- **primitive_type:** ALLOCATE
- **predecessors:** [`gateway-004-init-port-manager`]
- **successors:** [`gateway-007-start-websocket-server`]
- **p_probability:** 0.95
- **t_time:** 0.20
- **h_hazard:** 0.05
- **bernoulli_zone:** Infrastructure
- **port_manager_required:** true
- **port_allocation:** Dynamic (CDN block, determined by crystal)
- **crystal_integration:** ✅ **Required** — Gateway port must pass crystal resonance
- **task_seq:** 6

---

### Layer 2: Server Initialization

#### TASK-007: Start WebSocket Server
- **hash_id:** `gateway-007-start-websocket-server`
- **task_name:** Start WebSocket Server
- **description:** Initialize Axum WebSocket server on port 18600, set up connection handlers
- **category:** Server
- **hd4_phase:** Detect
- **primitive_type:** CREATE
- **predecessors:** [`gateway-006-reserve-gateway-port`]
- **successors:** [`gateway-008-register-handlers`, `gateway-009-setup-middleware`]
- **p_probability:** 0.98
- **t_time:** 0.30
- **h_hazard:** 0.02
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **port_allocation:** 18600
- **task_seq:** 7

#### TASK-008: Register Message Handlers
- **hash_id:** `gateway-008-register-handlers`
- **task_name:** Register Message Handlers
- **description:** Register handlers for all WsMessage types (Auth, Query, ExecuteWorkflow, GetPlasmaState, etc.)
- **category:** Server
- **hd4_phase:** Detect
- **primitive_type:** CREATE
- **predecessors:** [`gateway-007-start-websocket-server`]
- **successors:** [`gateway-010-query-handler`, `gateway-011-workflow-handler`, `gateway-015-routing-handler`]
- **p_probability:** 0.99
- **t_time:** 0.20
- **h_hazard:** 0.01
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 8

#### TASK-009: Setup Middleware
- **hash_id:** `gateway-009-setup-middleware`
- **task_name:** Setup Middleware
- **description:** Configure CORS, compression, logging, rate limiting middleware
- **category:** Server
- **hd4_phase:** Detect
- **primitive_type:** CREATE
- **predecessors:** [`gateway-007-start-websocket-server`]
- **successors:** [`gateway-016-auth-middleware`]
- **p_probability:** 0.95
- **t_time:** 0.25
- **h_hazard:** 0.05
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 9

---

### Layer 3: Core Handlers

#### TASK-010: Query Handler
- **hash_id:** `gateway-010-query-handler`
- **task_name:** Query Handler
- **description:** Handle SurrealQL queries, route via Neural Mux, execute on SurrealDB, return results
- **category:** Handler
- **hd4_phase:** Disrupt
- **primitive_type:** READ
- **predecessors:** [`gateway-002-connect-db`, `gateway-008-register-handlers`]
- **successors:** [`gateway-015-routing-handler`]
- **p_probability:** 0.90
- **t_time:** 0.50
- **h_hazard:** 0.10
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 10

#### TASK-011: Workflow Handler
- **hash_id:** `gateway-011-workflow-handler`
- **task_name:** Workflow Handler
- **description:** Handle workflow execution requests, route to Forge backend (port 18350), track execution state
- **category:** Handler
- **hd4_phase:** Disrupt
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-002-connect-db`, `gateway-008-register-handlers`]
- **successors:** [`gateway-015-routing-handler`]
- **p_probability:** 0.85
- **t_time:** 0.60
- **h_hazard:** 0.15
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **port_allocation:** 18350
- **task_seq:** 11

#### TASK-012: Health Subscriber
- **hash_id:** `gateway-012-health-subscriber`
- **task_name:** Health Subscriber
- **description:** Subscribe to NATS health check subjects, update connection statuses in GatewayState
- **category:** Monitoring
- **hd4_phase:** Detect
- **primitive_type:** RECEIVE
- **predecessors:** [`gateway-003-connect-nats`]
- **successors:** [`gateway-014-cdn-health-checker`]
- **p_probability:** 0.95
- **t_time:** 0.30
- **h_hazard:** 0.05
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 12

#### TASK-013: Plasma Subscriber
- **hash_id:** `gateway-013-plasma-subscriber`
- **task_name:** Plasma Subscriber
- **description:** Subscribe to sx9-atlas-bus plasma state updates, update GatewayState.plasma_snapshot
- **category:** Monitoring
- **hd4_phase:** Detect
- **primitive_type:** RECEIVE
- **predecessors:** [`gateway-003-connect-nats`]
- **successors:** [`gateway-017-plasma-handler`]
- **p_probability:** 0.90
- **t_time:** 0.35
- **h_hazard:** 0.10
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 13

---

### Layer 4: Routing & Neural Mux

#### TASK-014: CDN Health Checker
- **hash_id:** `gateway-014-cdn-health-checker`
- **task_name:** CDN Health Checker
- **description:** Periodic health checks for all 8 CDNs, update CDN statuses, trigger port manager reallocation on failure
- **category:** Monitoring
- **hd4_phase:** Detect
- **primitive_type:** MONITOR
- **predecessors:** [`gateway-005-allocate-cdn-ports`, `gateway-012-health-subscriber`]
- **successors:** [`gateway-015-routing-handler`]
- **p_probability:** 0.85
- **t_time:** 0.40
- **h_hazard:** 0.15
- **bernoulli_zone:** Operational
- **port_manager_required:** true
- **task_seq:** 14

#### TASK-015: Routing Handler
- **hash_id:** `gateway-015-routing-handler`
- **task_name:** Routing Handler
- **description:** Implement NeuralMuxRouter with Unicode-based routing (<250ns), trivariate hash prefix lookup, domain mask fallback, Bernoulli zone classification
- **category:** Routing
- **hd4_phase:** Disrupt
- **primitive_type:** ROUTE
- **predecessors:** [`gateway-010-query-handler`, `gateway-011-workflow-handler`, `gateway-014-cdn-health-checker`]
- **successors:** [`gateway-018-neural-mux-integration`]
- **p_probability:** 0.80
- **t_time:** 0.70
- **h_hazard:** 0.20
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 15

#### TASK-016: Auth Middleware
- **hash_id:** `gateway-016-auth-middleware`
- **task_name:** Auth Middleware
- **description:** Implement authentication middleware, validate trivariate hash tokens, enforce SDT gate checks
- **category:** Security
- **hd4_phase:** Disable
- **primitive_type:** AUTHENTICATE
- **predecessors:** [`gateway-009-setup-middleware`]
- **successors:** [`gateway-019-sdt-gate-check`]
- **p_probability:** 0.90
- **t_time:** 0.50
- **h_hazard:** 0.10
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 16

---

### Layer 5: Advanced Features

#### TASK-017: Plasma Handler
- **hash_id:** `gateway-017-plasma-handler`
- **task_name:** Plasma Handler
- **description:** Expose PlasmaState via WebSocket, stream SDT gate state, delta angle, entropy, crystal resonance
- **category:** Handler
- **hd4_phase:** Disrupt
- **primitive_type:** READ
- **predecessors:** [`gateway-013-plasma-subscriber`]
- **successors:** [`gateway-020-plasma-visualization`]
- **p_probability:** 0.85
- **t_time:** 0.45
- **h_hazard:** 0.15
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 17

#### TASK-018: Neural Mux Integration
- **hash_id:** `gateway-018-neural-mux-integration`
- **task_name:** Neural Mux Integration
- **description:** Integrate with ctas7-foundation-core NeuralMuxRouter, use Unicode ranges for operation routing, Bernoulli zone classification
- **category:** Integration
- **hd4_phase:** Disrupt
- **primitive_type:** COORDINATE
- **predecessors:** [`gateway-015-routing-handler`]
- **successors:** [`gateway-021-unicode-routing`]
- **p_probability:** 0.75
- **t_time:** 0.80
- **h_hazard:** 0.25
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 18

#### TASK-019: SDT Gate Check
- **hash_id:** `gateway-019-sdt-gate-check`
- **task_name:** SDT Gate Check
- **description:** Implement SDT gate validation, check crystal resonance, enforce thyristor state (Off/Primed/Conducting/Latched)
- **category:** Security
- **hd4_phase:** Disable
- **primitive_type:** AUTHENTICATE
- **predecessors:** [`gateway-016-auth-middleware`]
- **successors:** [`gateway-022-crystal-resonance`]
- **p_probability:** 0.80
- **t_time:** 0.60
- **h_hazard:** 0.20
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 19

---

### Layer 6: Optimization & Monitoring

#### TASK-020: Plasma Visualization
- **hash_id:** `gateway-020-plasma-visualization`
- **task_name:** Plasma Visualization
- **description:** Stream plasma state updates to UI, visualize SDT gate transitions, crystal resonance scores
- **category:** Visualization
- **hd4_phase:** Dominate
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-017-plasma-handler`]
- **successors:** []
- **p_probability:** 0.70
- **t_time:** 0.50
- **h_hazard:** 0.30
- **bernoulli_zone:** Analytical
- **port_manager_required:** false
- **task_seq:** 20

#### TASK-021: Unicode Routing
- **hash_id:** `gateway-021-unicode-routing`
- **task_name:** Unicode Routing
- **description:** Implement Unicode-based routing using U+E000-F8FF ranges, map trivariate hashes to Unicode runes for eBPF processing
- **category:** Routing
- **hd4_phase:** Dominate
- **primitive_type:** ROUTE
- **predecessors:** [`gateway-018-neural-mux-integration`]
- **successors:** []
- **p_probability:** 0.75
- **t_time:** 0.70
- **h_hazard:** 0.25
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 21

#### TASK-022: Crystal Resonance
- **hash_id:** `gateway-022-crystal-resonance`
- **task_name:** Crystal Resonance
- **description:** Integrate polycrystal voting, evaluate resonance scores, apply voting policies (Any/All/Majority/Weighted/Quorum)
- **category:** Security
- **hd4_phase:** Dominate
- **primitive_type:** VALIDATE
- **predecessors:** [`gateway-019-sdt-gate-check`]
- **successors:** []
- **p_probability:** 0.80
- **t_time:** 0.65
- **h_hazard:** 0.20
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 22

---

## Task Graph Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│                    GATEWAY TASK GRAPH                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  LAYER 0: Foundation (Root)                                     │
│  ┌─────────────┐                                                │
│  │ TASK-001    │───► Init State                                 │
│  └──────┬──────┘                                                │
│         │                                                        │
│         ├──► TASK-002: Connect SurrealDB                       │
│         ├──► TASK-003: Connect NATS                             │
│         └──► TASK-004: Init Port Manager                        │
│                  │                                               │
│                  ├──► TASK-005: Allocate CDN Ports             │
│                  └──► TASK-006: Reserve Gateway Port            │
│                           │                                      │
│                           └──► TASK-007: Start WebSocket Server │
│                                    │                             │
│                                    ├──► TASK-008: Register Handlers
│                                    └──► TASK-009: Setup Middleware
│                                             │                    │
│  LAYER 3: Core Handlers                                           │
│  ├──► TASK-010: Query Handler                                    │
│  ├──► TASK-011: Workflow Handler                                 │
│  ├──► TASK-012: Health Subscriber                                │
│  └──► TASK-013: Plasma Subscriber                                │
│                                                                  │
│  LAYER 4: Routing & Neural Mux                                   │
│  ├──► TASK-014: CDN Health Checker                              │
│  ├──► TASK-015: Routing Handler                                 │
│  └──► TASK-016: Auth Middleware                                 │
│                                                                  │
│  LAYER 5: Advanced Features                                      │
│  ├──► TASK-017: Plasma Handler                                  │
│  ├──► TASK-018: Neural Mux Integration                          │
│  └──► TASK-019: SDT Gate Check                                  │
│                                                                  │
│  LAYER 6: Optimization & Monitoring                             │
│  ├──► TASK-020: Plasma Visualization                            │
│  ├──► TASK-021: Unicode Routing                                │
│  └──► TASK-022: Crystal Resonance                               │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Capability & Capacity Analysis

### Capability Matrix

| Task | Capability | Dependencies | Port Manager | Bernoulli Zone |
|------|------------|--------------|--------------|----------------|
| TASK-001 | State Management | None | No | Infrastructure |
| TASK-002 | Database Connection | SurrealDB | No | Operational |
| TASK-003 | Messaging | NATS | No | Operational |
| TASK-004 | Port Allocation | Port Manager | **Yes** | Infrastructure |
| TASK-005 | CDN Port Allocation | Port Manager | **Yes** | Infrastructure |
| TASK-006 | Gateway Port Reservation | Port Manager | **Yes** | Infrastructure |
| TASK-007 | WebSocket Server | Axum | No | Operational |
| TASK-008 | Message Routing | Handlers | No | Operational |
| TASK-009 | Middleware Stack | Axum | No | Operational |
| TASK-010 | Query Execution | SurrealDB, Neural Mux | No | Operational |
| TASK-011 | Workflow Execution | Forge Backend | No | Operational |
| TASK-012 | Health Monitoring | NATS | No | Operational |
| TASK-013 | Plasma State | NATS, sx9-atlas-bus | No | Tactical |
| TASK-014 | CDN Health | Port Manager | **Yes** | Operational |
| TASK-015 | Deterministic Routing | Neural Mux | No | Tactical |
| TASK-016 | Authentication | SDT Gate | No | Tactical |
| TASK-017 | Plasma Streaming | Plasma State | No | Tactical |
| TASK-018 | Neural Mux | Foundation Core | No | Tactical |
| TASK-019 | SDT Validation | Crystal, Thyristor | No | Tactical |
| TASK-020 | Visualization | Plasma State | No | Analytical |
| TASK-021 | Unicode Routing | Neural Mux | No | Tactical |
| TASK-022 | Crystal Voting | Polycrystal | No | Tactical |
| TASK-023 | Ops-Platform Bridge | React Frontend | No | Operational |
| TASK-024 | Modal Inventory | Playwright | No | Operational |
| TASK-025 | Kali Layer 2 Execution | eBPF/XDP | **Yes** | Tactical |
| TASK-026 | UI State Sync | WebSocket | No | Operational |
| TASK-027 | Tool Result Handler | eBPF Results | No | Tactical |
| TASK-028 | Database Connections | SurrealDB/Postgres/Redis | No | Operational |
| TASK-029 | Threat Correlation | PTCC-TETH DB | No | Analytical |
| TASK-030 | ML Model Handler | DistilBERT/Phi3/GNN | No | Analytical |
| TASK-031 | Model Inference | ML Pipeline | No | Analytical |
| TASK-032 | Threat Reports | ML Outputs | No | Analytical |

### Capacity Requirements

| Bernoulli Zone | Task Count | Latency Target | Port Manager Calls |
|----------------|------------|----------------|-------------------|
| **Tactical** | 9 tasks | <50μs | 1 |
| **Operational** | 11 tasks | 50μs-1ms | 0 |
| **Analytical** | 5 tasks | 1ms-100ms | 0 |
| **Infrastructure** | 6 tasks | 100ms-60s | **4** |

**Total Tasks:** 32  
**Port Manager Dependencies:** 5 tasks (TASK-004, TASK-005, TASK-006, TASK-014, TASK-025)

---

## Critical Path Analysis

**Critical Path:** TASK-001 → TASK-004 → TASK-006 → TASK-007 → TASK-008 → TASK-015 → TASK-018 → TASK-023 → TASK-025 → TASK-030 → TASK-031 → TASK-032

**Bottlenecks:**
1. **Port Manager Integration** (TASK-004, TASK-005, TASK-006) — Infrastructure zone, 100ms-60s
2. **Neural Mux Integration** (TASK-018) — Tactical zone, <50μs target
3. **Routing Handler** (TASK-015) — Tactical zone, <250ns target

**Parallel Execution Opportunities:**
- TASK-002, TASK-003, TASK-004 can run in parallel after TASK-001
- TASK-010, TASK-011, TASK-012, TASK-013 can run in parallel after their predecessors
- TASK-020, TASK-021, TASK-022 can run in parallel (all leaf nodes)

---

## Port Manager Integration Details

### Required Port Allocations

| Service | Port Range | Allocation Method | Priority |
|---------|-----------|-------------------|----------|
| Gateway WebSocket | 18600 | Reserve (TASK-006) | Critical |
| CDN Static Assets | 19000 | Allocate (TASK-005) | High |
| CDN Rust Crates | 19001 | Allocate (TASK-005) | High |
| CDN Geospatial | 19002 | Allocate (TASK-005) | High |
| CDN ML Models | 19003 | Allocate (TASK-005) | High |
| CDN Conda | 19010 | Allocate (TASK-005) | Medium |
| CDN Tools | 19011 | Allocate (TASK-005) | Medium |
| CDN WASM | 19012 | Allocate (TASK-005) | Medium |
| CDN Plasma | 19013 | Allocate (TASK-005) | Medium |

### Port Manager Client Implementation

```rust
use ctas7_real_port_manager::{PortManager, ServiceType, PortAllocation};

pub struct GatewayPortManager {
    manager: PortManager,
    gateway_allocation: Option<PortAllocation>,
    cdn_allocations: Vec<PortAllocation>,
}

impl GatewayPortManager {
    pub async fn new() -> Result<Self> {
        let manager = PortManager::new();
        Ok(Self {
            manager,
            gateway_allocation: None,
            cdn_allocations: Vec::new(),
        })
    }
    
    /// Allocate gateway port from CDN block (dynamic, not traditional number)
    pub async fn allocate_gateway_port(&mut self) -> Result<PortAllocation> {
        let allocation = self.manager.allocate_cdn_port("sx9-gateway").await?;
        self.gateway_allocation = Some(allocation.clone());
        
        // Mirror ports are automatically allocated
        info!("Gateway port allocated: {} with mirrors: {:?}", 
              allocation.port, allocation.mirror_ports);
        
        Ok(allocation)
    }
    
    /// Allocate all CDN ports dynamically (no traditional port numbers)
    pub async fn allocate_cdn_ports(&mut self) -> Result<Vec<PortAllocation>> {
        let cdn_services = vec![
            "cdn-static",
            "cdn-crates",
            "cdn-geo",
            "cdn-models",
            "cdn-conda",
            "cdn-tools",
            "cdn-wasm",
            "cdn-plasma",
        ];
        
        let mut allocations = Vec::new();
        for service_name in cdn_services {
            match self.manager.allocate_cdn_port(service_name).await {
                Ok(allocation) => {
                    allocations.push(allocation.clone());
                    info!("CDN port allocated: {} for {} (mirrors: {:?})", 
                          allocation.port, service_name, allocation.mirror_ports);
                }
                Err(e) => {
                    warn!("Failed to allocate CDN port for {}: {:?}", service_name, e);
                }
            }
        }
        
        self.cdn_allocations = allocations.clone();
        Ok(allocations)
    }
    
    /// Get mirror ports for a service (load balancing, failover)
    pub fn get_mirror_ports(&self, port: u16) -> Vec<u16> {
        self.manager.get_mirror_ports(port)
    }
}
```

**Key Features:**
- **No Traditional Port Numbers:** Uses dynamic port blocks (18140-18159 for CDN)
- **Automatic Mirror Allocation:** Each port gets mirror ports for redundancy
- **Deception Support:** Fake ports and decoy services for stealth
- **Service Type Classification:** CDN, Orbital, Neural, Foundation, etc.

---

## Ops-Main-Platform Integration Tasks

### TASK-023: Ops-Main-Platform WebSocket Bridge
- **hash_id:** `gateway-023-ops-platform-bridge`
- **task_name:** Ops-Main-Platform WebSocket Bridge
- **description:** Bridge React/Vite frontend (port 15174) to sx9-gateway WebSocket server. Handle all UI component requests (HD4 phases, tasks, graph visualization, modals, database connections, Kali tools, etc.). Route through gateway handlers.
- **category:** Integration
- **hd4_phase:** Disrupt
- **primitive_type:** CONNECT
- **predecessors:** [`gateway-007-start-websocket-server`]
- **successors:** [`gateway-024-modal-inventory-handler`, `gateway-025-kali-layer2-execution`]
- **p_probability:** 0.90
- **t_time:** 0.60
- **h_hazard:** 0.10
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **port_allocation:** 15174 (frontend), 18600 (gateway)
- **task_seq:** 23

### TASK-024: Modal Inventory Handler
- **hash_id:** `gateway-024-modal-inventory-handler`
- **task_name:** Modal Inventory Handler
- **description:** Integrate Playwright modal inventory system. Catalog all UI modals, forms, interactions. Expose modal state via WebSocket for UI synchronization. Support modal triggers from gateway commands.
- **category:** Integration
- **hd4_phase:** Detect
- **primitive_type:** READ
- **predecessors:** [`gateway-023-ops-platform-bridge`]
- **successors:** [`gateway-026-ui-state-sync`]
- **p_probability:** 0.85
- **t_time:** 0.50
- **h_hazard:** 0.15
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 24

### TASK-025: Kali ISO Layer 2 Execution
- **hash_id:** `gateway-025-kali-layer2-execution`
- **task_name:** Kali ISO Layer 2 Execution
- **description:** Integrate Kali Plasma ISO for Layer 2 tool execution via eBPF/XDP. Route tool execution requests from ops-platform through gateway to Kali ISO. Tools execute at Layer 2 (5-12ns) using SDT protocol (EtherType 0xSD77). Support nmap, masscan, nuclei, sqlmap, hydra, metasploit, responder, impacket, bloodhound, crackmapexec.
- **category:** Execution
- **hd4_phase:** Disrupt
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-023-ops-platform-bridge`]
- **successors:** [`gateway-027-tool-result-handler`]
- **p_probability:** 0.80
- **t_time:** 0.70
- **h_hazard:** 0.20
- **bernoulli_zone:** Tactical
- **port_manager_required:** true
- **port_allocation:** Dynamic (Kali ISO port via port manager)
- **layer2_execution:** ✅ **Required** — All Kali tools execute via eBPF/XDP at Layer 2
- **task_seq:** 25

### TASK-026: UI State Synchronization
- **hash_id:** `gateway-026-ui-state-sync`
- **task_name:** UI State Synchronization
- **description:** Synchronize UI state between ops-platform frontend and gateway backend. Handle database connections status, CDN health, Plasma state, task status, graph updates. Use WebSocket bidirectional communication.
- **category:** Integration
- **hd4_phase:** Detect
- **primitive_type:** COORDINATE
- **predecessors:** [`gateway-024-modal-inventory-handler`]
- **successors:** [`gateway-028-database-connection-handler`]
- **p_probability:** 0.85
- **t_time:** 0.55
- **h_hazard:** 0.15
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 26

### TASK-027: Tool Result Handler
- **hash_id:** `gateway-027-tool-result-handler`
- **task_name:** Tool Result Handler
- **description:** Handle results from Layer 2 Kali tool execution. Parse eBPF/XDP tool responses, encode as Unicode runes, route back to ops-platform UI. Store results in SurrealDB with trivariate hash IDs.
- **category:** Handler
- **hd4_phase:** Disrupt
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-025-kali-layer2-execution`]
- **successors:** [`gateway-029-threat-intelligence-correlation`]
- **p_probability:** 0.85
- **t_time:** 0.50
- **h_hazard:** 0.15
- **bernoulli_zone:** Tactical
- **port_manager_required:** false
- **task_seq:** 27

### TASK-028: Database Connection Handler
- **hash_id:** `gateway-028-database-connection-handler`
- **task_name:** Database Connection Handler
- **description:** Manage database connections for ops-platform. Support SurrealDB, PostgreSQL, Redis, Sled, Sledis. Expose connection status via WebSocket. Handle connection pooling, health checks, reconnection logic.
- **category:** Handler
- **hd4_phase:** Detect
- **primitive_type:** CONNECT
- **predecessors:** [`gateway-026-ui-state-sync`]
- **successors:** [`gateway-030-ml-model-handler`]
- **p_probability:** 0.90
- **t_time:** 0.45
- **h_hazard:** 0.10
- **bernoulli_zone:** Operational
- **port_manager_required:** false
- **task_seq:** 28

### TASK-029: Threat Intelligence Correlation
- **hash_id:** `gateway-029-threat-intelligence-correlation`
- **task_name:** Threat Intelligence Correlation
- **description:** Correlate tool execution results with PTCC-TETH threat intelligence database. Use trivariate hash lookup for threat actor matching, attack scenario correlation, TETH entropy analysis. Feed results to ML models for prediction.
- **category:** Intelligence
- **hd4_phase:** Disrupt
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-027-tool-result-handler`]
- **successors:** [`gateway-030-ml-model-handler`]
- **p_probability:** 0.80
- **t_time:** 0.65
- **h_hazard:** 0.20
- **bernoulli_zone:** Analytical
- **port_manager_required:** false
- **task_seq:** 29

### TASK-030: ML Model Handler (DistilBERT, Phi3, GNN)
- **hash_id:** `gateway-030-ml-model-handler`
- **task_name:** ML Model Handler
- **description:** Integrate three ML models trained on threat intelligence database:
  - **DistilBERT**: Text classification and threat pattern recognition from tool outputs
  - **Phi3**: Small language model for threat intelligence summarization and reasoning
  - **GNN (Graph Neural Network)**: Threat actor relationship graph analysis, attack path prediction
- **category:** Intelligence
- **hd4_phase:** Dominate
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-028-database-connection-handler`, `gateway-029-threat-intelligence-correlation`]
- **successors:** [`gateway-031-model-inference-pipeline`]
- **p_probability:** 0.75
- **t_time:** 0.80
- **h_hazard:** 0.25
- **bernoulli_zone:** Analytical
- **port_manager_required:** false
- **ml_models:** ✅ **Required** — DistilBERT, Phi3, GNN models trained on threat database
- **task_seq:** 30

### TASK-031: Model Inference Pipeline
- **hash_id:** `gateway-031-model-inference-pipeline`
- **task_name:** Model Inference Pipeline
- **description:** Orchestrate ML model inference pipeline. Route tool results → DistilBERT (classification) → Phi3 (summarization) → GNN (graph analysis). Combine results into unified threat intelligence report. Cache results with trivariate hash keys.
- **category:** Intelligence
- **hd4_phase:** Dominate
- **primitive_type:** COORDINATE
- **predecessors:** [`gateway-030-ml-model-handler`]
- **successors:** [`gateway-032-threat-report-handler`]
- **p_probability:** 0.70
- **t_time:** 0.90
- **h_hazard:** 0.30
- **bernoulli_zone:** Analytical
- **port_manager_required:** false
- **task_seq:** 31

### TASK-032: Threat Report Handler
- **hash_id:** `gateway-032-threat-report-handler`
- **task_name:** Threat Report Handler
- **description:** Generate unified threat intelligence reports from ML model outputs. Format for ops-platform UI display (graph visualization, threat actor cards, attack scenario mapping). Stream reports via WebSocket to frontend.
- **category:** Handler
- **hd4_phase:** Dominate
- **primitive_type:** TRANSFORM
- **predecessors:** [`gateway-031-model-inference-pipeline`]
- **successors:** []
- **p_probability:** 0.80
- **t_time:** 0.60
- **h_hazard:** 0.20
- **bernoulli_zone:** Analytical
- **port_manager_required:** false
- **task_seq:** 32

---

## Next Steps

1. **Generate Trivariate Hashes** — Create SCH/CUID/SX9-UUID for each task
2. **Implement Port Manager Client** — Integrate `ctas7-real-port-manager` client
3. **Create Task Execution Engine** — Build task graph executor with dependency resolution
4. **Add Metrics Collection** — Track PTH metrics, Bernoulli zone compliance, port allocation success
5. **Visualize Graph** — Create graph visualization for task dependencies
6. **Integrate Ops-Main-Platform** — Bridge React frontend to gateway WebSocket
7. **Deploy Kali ISO** — Set up Layer 2 execution environment for Kali tools
8. **Load ML Models** — Integrate DistilBERT, Phi3, and GNN models trained on threat database

---

**Port Manager Status:** ✅ **Confirmed** — `ctas7-real-port-manager` (port 18104) is the port of record for all gateway port allocations.

**Ops-Main-Platform Status:** ✅ **Integration Required** — React/Vite frontend (port 15174) needs WebSocket bridge to gateway (port 18600).

**Kali ISO Status:** ✅ **Layer 2 Execution Required** — Kali Plasma ISO must execute tools via eBPF/XDP at Layer 2 (5-12ns latency).

**ML Models Status:** ✅ **Integration Required** — DistilBERT, Phi3, and GNN models trained on threat intelligence database must be integrated into gateway inference pipeline.

