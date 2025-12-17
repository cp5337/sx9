# GLAF: Graph Learning & Analytics Fabric

**Central Documentation Hub**

This directory contains the canonical architecture and implementation documentation for GLAF within the `sx9` ecosystem.

## Key Documents

- **[GLAF Architecture](./GLAF-INTEL-ARCHITECTURE.md)**
  - High-level system design, layers (SlotGraph, GLAF-INTEL, Plasma-ECS), and data flow.
- **[Graph Engine Patterns](./graph-engine-patterns.md)**
  - Detailed design patterns for the Graph Engine, including the "Dual-Layer Node Model" and React Flow integration strategies.
- **[Backend Blueprint](./glaf_backend.md)**
  - Specification for the `glaf-intel-engine` Backend, including persistence layers and API contracts.

## Implementation Map

The GLAF system is implemented across several crates in this workspace:

| Component           | Path                            | Description                                                                    |
| ------------------- | ------------------------------- | ------------------------------------------------------------------------------ |
| **Core Engine**     | `crates/sx9-glaf-core`          | Defines `GLAFCore`, `Node`, `Edge`, and basic graph operations.                |
| **Backend Service** | `crates/sx9-foundation-daemon`  | Hosts the `BackendMCPServer` which wraps GLAFCore and handles persistence/API. |
| **Orbital Bridge**  | `crates/sx9-foundation-orbital` | Feeds satellite position data into GLAF via `BackendMCPServer`.                |
| **Frontend**        | `apps/sx9-ops-main`             | React/Cytoscape visualization of the graph.                                    |

## Persistence

GLAF graph snapshots are persisted using **Sled** in `sx9-foundation-data`.

- **Key Prefix**: `glaf_snapshot_{namespace}`
- **Format**: JSON Serialization of `nodes` and `edges`.
