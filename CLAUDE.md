# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**SYNAPTIX9 (SX9)** - Universal Workflow System. A polyglot monorepo combining Rust (Cargo workspace) and JavaScript/TypeScript (pnpm workspace). The system implements a "Neural Mux" universal connectivity platform with workflow design, cognitive atoms framework, and threat detection capabilities.

## Build Commands

### Rust (Cargo workspace - 33 crates)
```bash
cargo build --release                    # Build entire workspace
cargo build -p sx9-foundation-core       # Build specific crate
cargo test --all                         # Run all tests
cargo test -p sx9-foundation-core        # Test specific crate
cargo fmt --all                          # Format all code
cargo clippy --all --all-targets         # Lint with Clippy
```

### JavaScript/TypeScript (pnpm workspace)
```bash
pnpm install                             # Install dependencies
pnpm build                               # Build all packages
pnpm test                                # Run all tests
pnpm lint                                # Lint all packages
pnpm start:forge                         # Start workflow backend (Port 18350)
pnpm start:web                           # Start web interface
pnpm forge:status                        # Check if Forge backend is running
```

### Tauri Desktop (sx9-forge)
```bash
cd sx9-forge
npm run tauri:dev                        # Development with hot reload
npm run tauri:build                      # Production build
```

## Architecture

### Directory Layout
- **crates/** - Rust workspace with 33 crates
  - `sx9-foundation-*` - Core foundation libraries (core, daemon, data, interface, math, tactical, voice, orbital, manifold)
  - `sx9-cdn-*` - Threat intelligence and monitoring systems
  - `sx9-plasma-*` - ECS (Legion-based) and security systems
  - `sx9-glaf-core` - Graph Learning & Analytics Fabric
  - `sx9-hashing-engine` - Trivariate hash microservice (Port 8002)
  - `sx9-atlas-*` - Service mesh and message bus
  - `sx9-harness` - QA gates and testing harness
- **packages/** - JavaScript workspace (core engine, UI components, forge-integration, chromadb-client)
- **apps/** - End-user applications (sx9-ops-main, sx9-graph-viewer)
- **tools/** - Backend services (forge-backend on Port 18350, abe, kali-plasma, vault)
- **sx9-forge/** - Vite + React + Tauri desktop application
- **sx9-linear-agent/** - Autonomous AI agent integrating Linear, Serena MCP, and Slack

### Three-Layer ECS Architecture
1. **Plasma ECS** - Core entity management (Legion-based)
2. **ATLAS Bus** - Event/message routing (NATS/JetStream)
3. **Defender** - Security and threat response layer

### Technology Stack
- **Rust:** Tokio, Axum, Serde, SurrealDB, Legion ECS, Wasmtime
- **JavaScript:** React 18, Vite, Tauri 2.2, Zustand, Monaco Editor, Tailwind CSS
- **Database:** SurrealDB (primary), PostgreSQL, Sled
- **Messaging:** NATS/JetStream (async-nats)

### Key Ports
- **18350** - forge-backend (workflow execution)
- **8002** - sx9-hashing-engine (trivariate hashing)
- **4222** - NATS/JetStream

## Crate Metadata System

Each Rust crate includes Smart Crate System v7.3.1 metadata in `Cargo.toml`:
```toml
[package.metadata.ctas7]
crate_type = "foundation"
mission = "DependencyUnification"
security_level = "Production"
orchestrator_compatible = true
```

## Cognitive Atoms Framework

Workflows are built from universal node types with six-dimensional properties:
- **Node types:** source, sink, transformer, router, buffer, gate, monitor, catalyst, inhibitor, relay
- **Dimensions:** Physical (P), Temporal (T), Energetic (E), Spatial (S), Relational (R), Economic (Phi)

## Git Workflow

- **Main branch:** `main`
- **Current branch:** `forge-tauri-integration`
- **Automation:** `scripts/hourly-wip.sh` runs hourly WIP commits for recovery safety
- **Hooks:** `scripts/install_hooks.sh` sets up pre-commit hooks

## Key Integration Points

- **@synaptix9/core** and **@synaptix9/ui** - Core workflow engine and React components
- **sx9-glaf-core** - In-memory graph with Cypher++ queries for workflow analysis
- **sx9-linear-agent** - Integrates Linear project management, Serena MCP code generation, and Slack notifications

## Code Quality

The project uses an AI-first coding approach with static analysis harness (see `code-quality.md`). Quality gates include:
- Clippy with strict rules
- ESLint + Prettier for JavaScript
- SARIF output format for CI/CD integration
- Cross-crate dependency analysis

## Documentation

- **docs/** - Architecture documentation including GLAF and intelligence systems
- **01-rfc/** - RFC documents (aligned with Smart Crate v7.3.1)
- **math-documents/** - Mathematical foundations
- **0X-sx9-ontology-master/** - Ontology and conceptual models
