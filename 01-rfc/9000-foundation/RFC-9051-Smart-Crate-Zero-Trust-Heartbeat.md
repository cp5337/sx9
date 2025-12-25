# RFC-9051: Smart Crate Zero-Trust Heartbeat System

**Status:** CANONICAL
**Version:** 7.3.1
**Date:** 2025-12-25
**Depends On:** RFC-9050, RFC-9400
**Implements:** sx9-foundation-core/src/heartbeat.rs, sx9-harness/src/nats/heartbeat_emitter.rs

---

## 1. Abstract

RFC-9051 defines a distributed, non-blocking heartbeat system that enforces zero-trust dependency validation across all smart crates. Unlike RFC-9050's Quality/Security dual heartbeat (UDP multicast), this system uses NATS pub/sub for compile-time and runtime verification that all crates depend on `sx9-foundation-core`.

**Core Invariant:** Any crate running without foundation-core is UNAUTHORIZED and triggers immediate alerts.

---

## 2. Problem Statement

In a distributed smart crate ecosystem, how do we ensure:
1. All crates depend on `sx9-foundation-core` (zero-trust requirement)
2. Detection is instant (compile-time) not just runtime
3. No central bottleneck or single point of failure
4. Alerts propagate immediately for unauthorized crates

**Solution:** Dual-layer validation with compile-time token + NATS pub/sub.

---

## 3. Architecture

### 3.1 System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SX9 ZERO-TRUST HEARTBEAT SYSTEM                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                              COMPILE TIME                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                                                                      │   │
│   │    Crate A         Crate B         Crate C         Crate D          │   │
│   │    ┌─────┐         ┌─────┐         ┌─────┐         ┌─────┐          │   │
│   │    │ ✓ f │         │ ✓ f │         │ ✗   │         │ ✓ f │          │   │
│   │    │ o u │         │ o u │         │     │         │ o u │          │   │
│   │    │ n n │         │ n n │         │ NO  │         │ n n │          │   │
│   │    │ d d │         │ d d │         │TOKEN│         │ d d │          │   │
│   │    └──┬──┘         └──┬──┘         └──┬──┘         └──┬──┘          │   │
│   │       │               │               │               │              │   │
│   │       │ COMPILES      │ COMPILES      │ FAILS!        │ COMPILES    │   │
│   │       ▼               ▼               ▼               ▼              │   │
│   │                                                                      │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│                               RUNTIME                                        │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                                                                      │   │
│   │    Crate A              Crate B              Crate D                 │   │
│   │    ┌─────┐              ┌─────┐              ┌─────┐                 │   │
│   │    │     │──publish──▶  │     │──publish──▶  │     │──publish──▶    │   │
│   │    └─────┘              └─────┘              └─────┘                 │   │
│   │        │                    │                    │                   │   │
│   │        └────────────────────┼────────────────────┘                   │   │
│   │                             │                                        │   │
│   │                             ▼                                        │   │
│   │              ╔══════════════════════════════╗                        │   │
│   │              ║        NATS SERVER           ║                        │   │
│   │              ║   sx9.heartbeat.crate.*      ║                        │   │
│   │              ╚══════════════════════════════╝                        │   │
│   │                             │                                        │   │
│   │                             ▼                                        │   │
│   │              ┌──────────────────────────────┐                        │   │
│   │              │    HEARTBEAT ORCHESTRATOR    │                        │   │
│   │              │                              │                        │   │
│   │              │  • Validates tokens          │                        │   │
│   │              │  • Detects missing beats     │                        │   │
│   │              │  • Publishes global state    │                        │   │
│   │              │  • Emits alerts              │                        │   │
│   │              └──────────────────────────────┘                        │   │
│   │                                                                      │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘

Legend: f = foundation-core dependency, ✓ = has token, ✗ = missing
```

### 3.2 Two-Layer Non-Blocking Design

```
┌─────────────────────────────────────────────────────────────────────────┐
│                   SMART CRATE ZERO-TRUST HEARTBEAT                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LAYER 1: LOCAL VALIDATION (Compile-Time, 0µs)                          │
│  ════════════════════════════════════════════════════════════════════   │
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  FOUNDATION_CORE_TOKEN = "sx9-foundation-core-7.3.1"            │   │
│  │                                                                  │   │
│  │  • Compile-time constant proves dependency exists                │   │
│  │  • assert_has_foundation_core!() macro for enforcement           │   │
│  │  • Zero network, zero locks, zero latency                        │   │
│  │  • QA gate checks this FIRST before any other gates              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  LAYER 2: GLOBAL AGGREGATION (Async Pub/Sub, ~50µs)                     │
│  ════════════════════════════════════════════════════════════════════   │
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  NATS Subject Hierarchy (Core NATS, not JetStream)              │   │
│  │                                                                  │   │
│  │  sx9.heartbeat.local          - Local broadcast                 │   │
│  │  sx9.heartbeat.global         - Orchestrator publishes state    │   │
│  │  sx9.heartbeat.crate.{name}   - Per-crate heartbeat            │   │
│  │  sx9.heartbeat.crate.*        - Orchestrator wildcard sub       │   │
│  │  sx9.heartbeat.alert.unauthorized - CRITICAL alert              │   │
│  │  sx9.heartbeat.alert.missing  - WARNING alert                   │   │
│  │                                                                  │   │
│  │  • Fire-and-forget pattern (no acks required)                   │   │
│  │  • Eventual consistency for global state                        │   │
│  │  • No central registry lock                                     │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.3 Message Flow

```
                    HEARTBEAT MESSAGE FLOW
                    ══════════════════════

    Smart Crate                  NATS                    Orchestrator
    ───────────                  ────                    ────────────
         │                         │                          │
         │  LocalHeartbeat         │                          │
         │ ┌───────────────┐       │                          │
         │ │ config        │       │                          │
         │ │ token ───────────────────────────────────────────┼─▶ Validate
         │ │ state_hash    │       │                          │
         │ │ health        │       │                          │
         │ │ timestamp     │       │                          │
         │ │ sequence      │       │                          │
         │ └───────────────┘       │                          │
         │                         │                          │
         ├─────publish────────────▶│                          │
         │  sx9.heartbeat.crate.X  │                          │
         │                         ├─────────deliver─────────▶│
         │                         │                          │
         │                         │                          │ record_heartbeat()
         │                         │                          │
         │                         │    GlobalHeartbeatState  │
         │                         │◀───────publish───────────┤
         │                         │  sx9.heartbeat.global    │
         │                         │                          │
         │                         │                          │ If unauthorized:
         │                         │       Alert              │
         │                         │◀───────publish───────────┤
         │                         │  sx9.heartbeat.alert.*   │
         │                         │                          │
```

### 3.4 No Bottlenecks

| Operation | Latency | Blocking | Description |
|-----------|---------|----------|-------------|
| Token check | 0µs | No | Compile-time constant |
| NATS publish | ~50µs | No | Fire-and-forget |
| Global validation | ~5s | No | Background task |
| Alert emission | ~50µs | No | Fire-and-forget |

### 3.5 QA Gate Pipeline Integration

```
                         QA GATE PIPELINE
    ═══════════════════════════════════════════════════════════════════

    ┌─────────────────────────────────────────────────────────────────┐
    │                        GATE 0: HEARTBEAT                         │
    │                    (MUST PASS FIRST - RFC-9051)                  │
    ├─────────────────────────────────────────────────────────────────┤
    │                                                                  │
    │   ┌───────────────┐     ┌───────────────┐                       │
    │   │  Local Check  │     │ Global Check  │                       │
    │   │               │     │               │                       │
    │   │ • Token valid │     │ • NATS state  │                       │
    │   │ • Compile-time│     │ • No unauth   │                       │
    │   │               │     │ • Hash OK     │                       │
    │   └───────┬───────┘     └───────┬───────┘                       │
    │           │                     │                                │
    │           └─────────┬───────────┘                                │
    │                     ▼                                            │
    │           ┌─────────────────┐                                   │
    │           │  Zero-Trust     │                                   │
    │           │  Score (0-100)  │                                   │
    │           └────────┬────────┘                                   │
    │                    │                                            │
    │         Score > 0? │                                            │
    │           ┌────────┴────────┐                                   │
    │          YES               NO ──▶ PIPELINE STOPS                │
    │           │                                                     │
    └───────────┼─────────────────────────────────────────────────────┘
                │
                ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                       GATE 1: STATIC (RFC-9141 Cold Truth)       │
    │   • AST extraction • Complexity • Rule enforcement • Compile    │
    └───────────────────────────────────────────────────────────────────┘
                │
                ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                     GATE 2: SEMANTIC (RFC-9141 Warm Annotation)  │
    │   • Intent alignment • Drift detection • Pattern matching       │
    └───────────────────────────────────────────────────────────────────┘
                │
                ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                     GATE 3: PATTERN (Canonical N-V-N-N)          │
    │   • Header validation • Behavioral scope • Constraint check     │
    └───────────────────────────────────────────────────────────────────┘
                │
                ▼
              DEPLOY
```

---

## 4. Implementation

### 4.1 Foundation Token

```rust
// sx9-foundation-core/src/heartbeat.rs

/// Compile-time proof of foundation-core dependency
pub const FOUNDATION_CORE_TOKEN: &str = "sx9-foundation-core-7.3.1";

/// Macro for compile-time enforcement
#[macro_export]
macro_rules! assert_has_foundation_core {
    () => {
        const _: () = {
            let _ = $crate::heartbeat::FOUNDATION_CORE_TOKEN;
        };
    };
}
```

### 4.2 Heartbeat Types

```rust
/// Local heartbeat payload (emitted by each crate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHeartbeat {
    pub config: HeartbeatConfig,
    pub foundation_token: String,    // Proves dependency
    pub state_hash: String,          // Trivariate hash
    pub health: HealthStatus,
    pub timestamp_ms: u64,
    pub sequence: u64,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Starting,
    Stopping,
}

/// Validation result
pub enum HeartbeatValidation {
    Valid,
    Unauthorized { crate_name: String, reason: String },
    Late { crate_name: String, latency_ms: u64 },
    HashMismatch { crate_name: String, expected: String, actual: String },
}
```

### 4.3 NATS Emitter

```rust
// sx9-harness/src/nats/heartbeat_emitter.rs

pub struct HeartbeatEmitter {
    client: async_nats::Client,
    inner: SmartCrateHeartbeat,
}

impl HeartbeatEmitter {
    /// Start background emission (non-blocking)
    pub async fn start_background_emission(
        self: Arc<Self>,
        interval: Duration,
        state_hash_fn: impl Fn() -> String + Send + Sync + 'static,
    ) {
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;
                let hash = state_hash_fn();
                let _ = self.emit(hash).await; // Fire-and-forget
            }
        });
    }
}
```

### 4.4 Orchestrator

```rust
pub struct HeartbeatOrchestrator {
    client: async_nats::Client,
    registry: HeartbeatRegistry,
}

impl HeartbeatOrchestrator {
    /// Subscribe to sx9.heartbeat.crate.* and aggregate state
    pub async fn run(self: Arc<Self>) -> Result<(), String> {
        // Subscribe to all crate heartbeats
        let mut subscriber = self.client
            .subscribe("sx9.heartbeat.crate.*")
            .await?;

        // Background validation loop (every 5s)
        tokio::spawn(async move {
            loop {
                let state = self.registry.validate_all();

                // Publish global state
                self.client.publish("sx9.heartbeat.global", state).await;

                // Alert on unauthorized crates
                for crate_name in &state.unauthorized_crates {
                    self.client.publish(
                        "sx9.heartbeat.alert.unauthorized",
                        alert_payload(crate_name)
                    ).await;
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        // Process incoming heartbeats
        while let Some(msg) = subscriber.next().await {
            let heartbeat: LocalHeartbeat = serde_json::from_slice(&msg.payload)?;
            self.registry.record_heartbeat(heartbeat);
        }
    }
}
```

---

## 5. QA Gate Integration

### 5.1 Zero-Trust Score

The heartbeat gate calculates a zero-trust score (0-100):

| Component | Points | Description |
|-----------|--------|-------------|
| Local service health | 40 | HTTP health check ratio |
| Global heartbeat | 30 | Base points for passing |
| Hash integrity | 20 | Trivariate hash verified |
| Node health ratio | 10 | Healthy/active nodes |
| **Unauthorized penalty** | **-50 each** | **CRITICAL violation** |

### 5.2 Gate Enforcement

```rust
// sx9-harness/src/gates/heartbeat_gate.rs

// Compile-time assertion
sx9_foundation_core::assert_has_foundation_core!();

impl HeartbeatGate {
    fn calculate_zero_trust_score(&self, local: &LocalHeartbeat, global: &GlobalHeartbeat) -> u8 {
        let mut score: i16 = 0;

        // CRITICAL: -50 per unauthorized crate
        let penalty = (global.unauthorized_crates.len() as i16) * 50;
        if penalty > 0 {
            for crate_name in &global.unauthorized_crates {
                eprintln!("🚨 ZERO-TRUST VIOLATION: {} running without foundation-core", crate_name);
            }
        }

        // ... calculate base score ...

        score.saturating_sub(penalty).clamp(0, 100) as u8
    }
}
```

---

## 6. NATS Subject Hierarchy

```rust
// sx9-harness/src/nats/subjects.rs

pub mod heartbeat {
    pub const PREFIX: &str = "sx9.heartbeat";
    pub const LOCAL: &str = "sx9.heartbeat.local";
    pub const GLOBAL: &str = "sx9.heartbeat.global";
    pub const ALERT_UNAUTHORIZED: &str = "sx9.heartbeat.alert.unauthorized";
    pub const ALERT_MISSING: &str = "sx9.heartbeat.alert.missing";
    pub const CRATE_WILDCARD: &str = "sx9.heartbeat.crate.*";

    pub fn for_crate(crate_name: &str) -> String {
        format!("sx9.heartbeat.crate.{}", crate_name)
    }
}
```

---

## 7. Usage

### 7.1 For Any Smart Crate

```rust
// In lib.rs or main.rs
sx9_foundation_core::assert_has_foundation_core!();

// For NATS heartbeat emission
use sx9_harness::nats::{HeartbeatEmitter, HealthStatus};

let emitter = HeartbeatEmitter::foundation(
    "nats://localhost:4222",
    "sx9-my-crate",
    "1.0.0",
    "data-foundation",
    18400
).await?;

emitter.set_health(HealthStatus::Healthy);
Arc::new(emitter).start_background_emission(
    Duration::from_secs(1),
    || compute_state_hash()
).await;
```

### 7.2 Orchestrator Startup

```rust
let orchestrator = HeartbeatOrchestrator::connect("nats://localhost:4222").await?;
Arc::new(orchestrator).run().await?;
```

---

## 8. Relationship to Other Heartbeats

| RFC | System | Transport | Focus |
|-----|--------|-----------|-------|
| RFC-9050 | Quality/Security | UDP Multicast | Grade + SARIF |
| RFC-9141 | Static/Semantic | In-process | Cold truth vs warm annotation |
| **RFC-9051** | **Zero-Trust** | **NATS pub/sub** | **Dependency enforcement** |

These systems are complementary:
- RFC-9051 ensures all crates have foundation-core (prerequisite)
- RFC-9050 monitors quality and security metrics
- RFC-9141 enforces QA doctrine in the assembly line

---

## 9. Invariants

1. `FOUNDATION_CORE_TOKEN` is the single source of truth for version
2. Missing foundation-core = UNAUTHORIZED = pipeline stops
3. NATS heartbeats are non-blocking (fire-and-forget)
4. Global state is eventually consistent (5s validation loop)
5. Zero-trust score of 0 = gate failure

---

## 10. References

- RFC-9050: QA Two-Heartbeat System (Quality/Security)
- RFC-9141: FORGE Assembly Line & QA Doctrine
- RFC-9400: Gateway & NATS Architecture
- sx9-foundation-core/src/heartbeat.rs
- sx9-harness/src/gates/heartbeat_gate.rs
- sx9-harness/src/nats/heartbeat_emitter.rs
