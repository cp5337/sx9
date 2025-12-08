# SX9 Forge - Dynamic Tool Generation Engine

**Version:** 7.3.1
**Ring Bus Node:** 9
**Port:** 18350

## Overview

SX9 Forge is the dynamic tool generation engine for the Synaptix9 workflow system. It provides:

- **GLAF Graph Storage** - Neo4j-like embedded graph using sled + petgraph
- **RFC-9302 Nonagon Analytic Nodes** - 9-vertex workflow cells with TETH entropy
- **Mission Load Sets** - In-app purchase tool chain packages
- **Dynamic Tool Creation** - Ring Bus Layer 2 execution via Kali ISO

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     SX9 FORGE ENGINE                        │
│                      Port 18350                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   GLAF      │  │  Mission    │  │    Tool     │        │
│  │   Graph     │  │   Load      │  │  Generator  │        │
│  │  (sled)     │  │  Catalog    │  │             │        │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘        │
│         │                │                │                │
│         └────────────────┼────────────────┘                │
│                          │                                  │
│                   ┌──────▼──────┐                          │
│                   │  Nonagon    │                          │
│                   │   Cell      │                          │
│                   │ (9-vertex)  │                          │
│                   └──────┬──────┘                          │
│                          │                                  │
└──────────────────────────┼──────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │  Ring Bus   │
                    │   Layer 2   │
                    │  Execution  │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  Kali ISO   │
                    │   :18200    │
                    └─────────────┘
```

## RFC-9302 Nonagon Analytic Node

Each workflow cell is a 9-vertex nonagon with three trivariates:

| Trivariate | Dimensions | Purpose |
|------------|------------|---------|
| **Alpha (α)** | context, meaning, intent | Semantic analysis |
| **Beta (β)** | phase, intensity, duration | Operational state (HD4 mapped) |
| **Gamma (γ)** | historical, current, predictive | Temporal context |

### TETH Entropy Calculation

```rust
// Shannon entropy over 9-vertex probability distribution
// H = -Σ p(x) * log2(p(x))

let vertices = [
    alpha.context, alpha.meaning, alpha.intent,
    beta.phase, beta.intensity, beta.duration,
    gamma.historical, gamma.current, gamma.predictive,
];

// Validated entropy: 3.9232 bits
// Minimum threshold: 2.5 bits
```

### HD4 Kill Chain Mapping

The beta.phase value maps to HD4 operational phases:

| Phase | Beta.x | Description |
|-------|--------|-------------|
| HUNT | 0.2 | Threat hunting, OSINT |
| DETECT | 0.4 | Detection, signature matching |
| DISRUPT | 0.6 | Active disruption |
| DISABLE | 0.8 | System disable operations |
| DOMINATE | 1.0 | Full spectrum dominance |

## Mission Load Sets (In-App Purchases)

Mission Loads are curated tool chain packages providing force multiplication:

### Tiers

| Tier | Clearance | Example Loads |
|------|-----------|---------------|
| **Free** | Public | Hunt Basic, Detect Basic |
| **Commercial** | Commercial | Hunt Premium, Detect Premium, Disrupt Pro |
| **Enterprise** | Restricted | Disable Enterprise, Dominate Restricted |

### Primitives

Each Mission Load includes specific primitive operations:

```rust
pub enum Primitive {
    // Data Operations
    Read, Write, Transform, Filter,

    // Security Operations
    Encrypt, Decrypt, Authenticate, Authorize, Validate,

    // Network Operations
    Route, Buffer, Queue, Synchronize, Replicate,

    // Analysis Operations
    Observe, Cache, Execute,

    // Restricted Operations
    Reconnaissance, CommandControl, Install,
}
```

## OSSEC TOML Rules Integration

Forge loads 700 OSSEC TOML rules with embedded nonagon analytics:

```toml
[rule]
id = 60000
level = 5
description = "EWM injection detection"
primitive = "TRANSFORM"
unicode_trigger = "U+E403"
sch_id = "SCH68322468482b549e"

[1nf.indicators.plasma]
regex = ".*(Extra|Window|Memory).*"
countermeasures = ["ossec-active-response:log-alert", "plasma-notify:transform"]

[nine_sided]
alpha_x_context = 0.5
alpha_y_meaning = 0.6
alpha_z_intent = 0.5
beta_x_phase = 0.57
beta_y_intensity = 0.02
beta_z_duration = 0.419498
gamma_x_historical = 0.53
gamma_y_current = 0.51
gamma_z_predictive = 0.543291
center = 0.465865
confidence = 0.888889
```

### Rule Statistics (700 Rules)

| Primitive | Count |
|-----------|-------|
| AUTHENTICATE | 65 |
| EXECUTE | 59 |
| ENCRYPT | 48 |
| WRITE | 46 |
| READ | 42 |
| ROUTE | 42 |
| DECRYPT | 38 |
| VALIDATE | 37 |
| SYNCHRONIZE | 37 |
| CACHE | 37 |
| QUEUE | 36 |
| BUFFER | 36 |
| REPLICATE | 36 |
| AUTHORIZE | 36 |
| OBSERVE | 33 |
| FILTER | 30 |
| TRANSFORM | 28 |
| COMMAND_CONTROL | 8 |
| RECONNAISSANCE | 5 |
| INSTALL | 1 |

## API Endpoints

### Health & Status

```
GET  /health              # Service health check
GET  /smart-crate/status  # Smart crate status
GET  /metrics             # Prometheus metrics
```

### Graph Operations

```
GET  /graph               # Graph statistics
GET  /graph/nodes         # List all nodes
GET  /graph/nodes/:id     # Get specific node
```

### Nonagon Operations

```
POST /nonagon             # Create nonagon cell
GET  /nonagon/:id         # Get nonagon by ID
GET  /nonagon/:id/entropy # Get TETH entropy
```

### Mission Load Operations

```
GET  /mission-loads       # List all mission loads
GET  /mission-loads/:id   # Get specific load
POST /mission-loads/:id/tool  # Create tool from load
```

### Tool & Chain Operations

```
GET  /tools               # List generated tools
POST /tools/:id/execute   # Execute tool via L2
POST /chains              # Create tool chain
POST /chains/:id/execute  # Execute chain via L2
```

## Configuration

### smart-crate.toml

```toml
[smart-crate]
name = "sx9-forge"
version = "7.3.1"
classification = "forge"
tesla_grade = true

[ports]
main = 18350
graph_api = 18351
tool_gen = 18352

[ring_bus]
node_id = 9
ring_direction = "bidirectional"
token_arbitration = true

[ring_bus.layer2]
unicode_triggers_enabled = true
kali_iso_endpoint = "http://localhost:18200"
trigger_latency_target_us = 10

[nonagon]
vertices = 9
trivariates = ["alpha", "beta", "gamma"]
delta_precision = 0.000001
min_entropy = 2.5
validated = true
```

## Usage

### Creating a Tool from Mission Load

```rust
use sx9_forge::{ForgeEngine, ForgeConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ForgeConfig::default();
    let engine = ForgeEngine::new(config).await?;

    // Create tool from Hunt Premium mission load
    let tool = engine.create_tool_from_load("hunt_premium").await?;

    println!("Created tool: {} (entropy: {})",
        tool.name, tool.nonagon.teth_entropy);

    Ok(())
}
```

### Loading OSSEC Rules

```rust
use sx9_forge::OssecLoader;

let mut loader = OssecLoader::new("/opt/sx9/forge/ossec_toml_rules");
let count = loader.load_all().await?;

// Create chains by primitive type
let chains = loader.create_chains_by_primitive();

// Create chains by severity
let severity_chains = loader.create_chains_by_severity();
```

### Executing Tool Chain

```rust
use sx9_forge::{ToolChain, ExecutionMode};

let mut chain = ToolChain::new("Threat Hunt Chain");
chain.add_tool("ossec-60000");
chain.add_tool("ossec-60100");
chain.execution_mode = ExecutionMode::Sequential;

let result = engine.execute_tool_chain(&chain).await?;
println!("Chain executed: {} tools, {}ms",
    result.tool_results.len(),
    result.total_duration_ms);
```

## Integration with GLAF

Forge integrates with the GLAF Graph Visualization system:

- **Geographical Networks** - Location-based threat mapping
- **Social Networks** - Actor relationships, APT analysis
- **Technical Networks** - Infrastructure dependencies
- **Temporal Networks** - Timeline analysis, campaign evolution
- **APT Emulation Networks** - Real-time TTP execution chains

## Dependencies

- `sx9-foundation-core` - Foundation crate
- `sx9-glaf-core` - GLAF neural operations
- `sx9-atlas-bus` - Ring bus communication
- `sled` - Embedded key-value store
- `petgraph` - Graph data structure
- `axum` - HTTP server framework
- `tokio` - Async runtime

## License

Part of the SX9 Platform - Enterprise AI-First Engineering System
