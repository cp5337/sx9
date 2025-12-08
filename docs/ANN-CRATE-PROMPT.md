# Prompt for GPT/Gemini: Create sx9-ann-engine Crate

## Context

You are implementing the `sx9-ann-engine` crate for the Synaptix9 (SX9) Gateway Neural Retrofit architecture (RFC-9114 Rev 1.1). This crate provides an **Artificial Neural Network (ANN) engine in observer mode** - it observes routing entropy and system behavior but does NOT interfere with deterministic operations.

## Requirements

### 1. Core Architecture

- **Observer Mode Only**: The ANN engine must NEVER interfere with deterministic routing (<250 ns latency requirement)
- **Dormant by Default**: Compiles into binary but remains disabled at runtime unless explicitly enabled via feature flag
- **Non-Blocking**: All ANN operations must be async and non-blocking
- **Zero Impact on Zone A**: Must not affect the Bernoulli Zone A (<50µs) operations

### 2. Dependencies

The crate should use:
- `ctas7-foundation-core` (or `sx9-foundation-core` after cloning) for all common dependencies
- `sx9-atlas-bus` for inter-daemon communication
- Re-export common dependencies through foundation-core modules:
  - `ctas7_foundation_core::data::serde` (instead of direct `serde`)
  - `ctas7_foundation_core::diagnostics::anyhow` (instead of direct `anyhow`)
  - `ctas7_foundation_core::diagnostics::tracing` (instead of direct `tracing`)
  - `ctas7_foundation_core::async_runtime::tokio` (instead of direct `tokio`)

### 3. Observer Functionality

The ANN engine should observe:

#### 3.1 Routing Entropy
- Track routing decisions and their outcomes
- Measure routing latency patterns
- Identify routing anomalies or inefficiencies
- Store observations in a time-series format

#### 3.2 System Behavior
- Monitor gateway request patterns
- Track trivariate hash distributions
- Observe crystal resonance patterns (from sx9-atlas-bus)
- Monitor SDT gate state transitions

#### 3.3 Neural Network Architecture
- Use a lightweight ANN architecture suitable for observation
- Support feedforward networks for pattern recognition
- Implement online learning (if enabled) that does NOT affect routing
- Store learned patterns in a separate storage layer (not in routing path)

### 4. API Surface

```rust
// Main ANN Engine
pub struct ANNEngine {
    observer: Arc<ObserverState>,
    network: Option<Arc<NeuralNetwork>>,
    enabled: bool,
}

impl ANNEngine {
    /// Create new ANN engine in observer mode
    pub fn new(config: ANNConfig) -> Self;
    
    /// Observe a routing decision (non-blocking)
    pub async fn observe_routing(&self, decision: RoutingObservation) -> Result<()>;
    
    /// Observe system behavior (non-blocking)
    pub async fn observe_behavior(&self, behavior: BehaviorObservation) -> Result<()>;
    
    /// Get advisory recommendation (only if enabled and non-blocking)
    pub async fn get_advisory(&self, context: &str) -> Result<Option<Advisory>>;
    
    /// Check if ANN is enabled
    pub fn is_enabled(&self) -> bool;
    
    /// Enable/disable ANN (runtime flag)
    pub fn set_enabled(&mut self, enabled: bool);
}

// Observation types
pub struct RoutingObservation {
    pub hash: TrivariateHash,
    pub route: RouteEntry,
    pub latency_ns: u64,
    pub timestamp: DateTime<Utc>,
    pub outcome: RoutingOutcome,
}

pub struct BehaviorObservation {
    pub event_type: BehaviorEventType,
    pub crystal_resonance: Option<f32>,
    pub sdt_state: Option<SDTState>,
    pub plasma_delta_angle: Option<f32>,
    pub timestamp: DateTime<Utc>,
}

pub struct Advisory {
    pub confidence: f32,
    pub recommendation: String,
    pub reasoning: Vec<String>,
    pub timestamp: DateTime<Utc>,
}
```

### 5. Integration Points

#### 5.1 sx9-atlas-bus Integration
- Subscribe to `sx9-atlas-bus` events for crystal/SDT observations
- Use `PlasmaState` snapshots for delta angle tracking
- Non-blocking event subscription

#### 5.2 Gateway Integration
- Optional integration with `sx9-gateway-primary`
- Only used for advisory routing (fail-safe fallback, see RFC-9114 §6.3)
- Must not be in the critical path

### 6. Storage

- Use `sled` or similar embedded database for observation storage
- Separate storage from routing path
- Time-series data structure for pattern analysis
- Configurable retention policy

### 7. Neural Network Implementation

- Use a lightweight Rust neural network library (e.g., `candle`, `burn`, or custom)
- Support feedforward architecture
- Online learning capability (if enabled)
- Pattern recognition for routing optimization suggestions
- Must be completely optional and non-blocking

### 8. Configuration

```rust
pub struct ANNConfig {
    pub enabled: bool,
    pub observer_mode: bool,
    pub storage_path: PathBuf,
    pub network_architecture: Option<NetworkArchitecture>,
    pub learning_enabled: bool,
    pub advisory_threshold: f32,
}
```

### 9. Testing Requirements

- Unit tests for observer functionality
- Integration tests with sx9-atlas-bus
- Performance tests to verify zero impact on routing latency
- Tests to verify ANN can be disabled at runtime

### 10. Code Standards

- **NO BLAKE3 HASHES** - Use Murmur3-64 only (via foundation-core)
- **NO SHA-256** - Use trivariate hash system
- **NO FAKE CODE** - All code must be functional
- **NO STUBS** - Complete implementations only
- **NO HARD CODED DATA** - Use configuration
- **NO DEMOS** - Production-ready code only

### 11. File Structure

```
sx9-ann-engine/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API
│   ├── engine.rs           # Main ANNEngine implementation
│   ├── observer.rs         # Observer state and logic
│   ├── network.rs          # Neural network implementation
│   ├── storage.rs           # Observation storage
│   ├── advisory.rs         # Advisory generation
│   ├── config.rs           # Configuration types
│   └── integration/        # Integration with sx9-atlas-bus
│       ├── atlas_bus.rs
│       └── gateway.rs
└── tests/
    ├── observer_tests.rs
    ├── network_tests.rs
    └── integration_tests.rs
```

## Implementation Guidelines

1. **Start with Observer**: Implement observer functionality first, then add neural network if needed
2. **Feature Flags**: Use Rust feature flags to enable/disable ANN functionality
3. **Async Everything**: All operations must be async and non-blocking
4. **Error Handling**: Use `anyhow::Result` for error handling (via foundation-core)
5. **Logging**: Use `tracing` for logging (via foundation-core)
6. **Serialization**: Use `serde` for serialization (via foundation-core)

## References

- RFC-9114 Rev 1.1: SX9 Gateway Neural Retrofit Architecture
- RFC-9001: Trivariate Hash System
- RFC-9004: Foundation Manifold
- RFC-9026: Hourglass-Bernoulli Zone Compliance
- `sx9-atlas-bus` crate for inter-daemon communication
- `ctas7-foundation-core` for common dependencies

## Deliverable

Create a complete, production-ready Rust crate that:
1. Implements observer mode for routing and system behavior
2. Provides optional neural network for pattern recognition
3. Integrates with sx9-atlas-bus
4. Can be enabled/disabled at runtime
5. Has zero impact on deterministic routing performance
6. Follows all code standards (no stubs, no demos, no fake code)
7. Uses foundation-core for all common dependencies
8. Includes comprehensive tests
