# SX9 Foundation NATS Bundle

**Purpose:** Progressive NATS integration via smart-crate.toml  
**Date:** December 14, 2025  
**Status:** Ready for drop-in

---

## Bundle Contents

```
nats-bundle/
├── foundation-core-patch/
│   ├── Cargo.toml.patch      # Dependency additions
│   └── src/nats/
│       ├── mod.rs
│       ├── client.rs
│       ├── subjects.rs
│       ├── streams.rs
│       └── messages.rs
│
├── smart-crate-nats.toml     # NATS section for any smart-crate.toml
│
└── examples/
    ├── publisher.rs
    └── consumer.rs
```

---

## Step 1: Add to Cargo.toml (foundation-core)

Add under `[dependencies]`:
```toml
# NATS messaging - JetStream for durable messaging
async-nats = { version = "0.35", optional = true }
```

Add under `[features]`:
```toml
messaging = ["async-nats"]
```

Add `"messaging"` to the `full` feature list.

---

## Step 2: Add nats/ module to src/

See individual files below.

---

## Step 3: Add to lib.rs

```rust
#[cfg(feature = "messaging")]
pub mod nats;
```

---

## Step 4: smart-crate.toml Integration

Each crate that uses NATS adds to their smart-crate.toml:

```toml
[nats]
enabled = true
url = "nats://localhost:4222"
# Subjects this crate publishes to
publishes = ["sx9.kali.result.>", "sx9.kali.telemetry.>"]
# Subjects this crate subscribes to
subscribes = ["sx9.kali.exec.>", "sx9.kali.chain.>"]
# JetStream streams this crate uses
streams = ["KALI"]
# Consumer name (for durable consumers)
consumer = "kali-executor"
```

---

## Progressive Adoption Path

```
Phase 1: foundation-core gets nats/ module (feature-gated)
         ↓
Phase 2: sx9-atlas-daemon enables "messaging" feature
         ↓
Phase 3: sx9-kali-daemon enables "messaging" feature
         ↓
Phase 4: CDNs enable as needed
         ↓
Phase 5: Full workspace on NATS
```

No crate breaks until it explicitly opts in.
