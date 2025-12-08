# Kali Plasma - Missing Components

**Date:** 2025-01-XX  
**Status:** ⚠️ **GAPS IDENTIFIED**

---

## Critical Missing Components

### 1. ❌ Plasma-Defender ANN Tool Result Handler

**Problem:** Plasma-Defender doesn't subscribe to `sx9.tool.result.ann` messages from Kali Plasma.

**Missing Code:**
```rust
// In sx9-plasma-defender/src/lib.rs or new module src/tool_handler.rs

/// Subscribe to tool results from Kali Plasma
pub async fn subscribe_tool_results(plasma_bus: Arc<PlasmaBus>) -> anyhow::Result<()> {
    let mut subscriber = plasma_bus.nats.subscribe("sx9.tool.result.ann").await?;
    
    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            // Parse tool result
            let tool_result: ToolResultMessage = serde_json::from_slice(&msg.payload)?;
            
            // Evaluate through crystal & SDT
            let threat_result = evaluate_threat(&tool_result.result_bytes).await?;
            
            // Feed to ANN daemon
            let ann_obs = AnnObservation {
                hash_entropy: threat_result.ring_strength,
                routing_latency_ns: 0, // Not applicable
                sdt_state: Some(threat_result.sdt_state as u8),
                crystal_resonance: Some(threat_result.ring_strength),
                timestamp: Utc::now(),
            };
            
            ann_daemon.observe(ann_obs).await?;
            
            // Get ANN advisory
            let ctx = AnnContext {
                entropy: threat_result.ring_strength,
                latency_score: 0.0,
            };
            
            if let Some(advisory) = ann_daemon.get_advisory(&ctx).await? {
                // Publish advisory
                plasma_bus.nats.publish(
                    "sx9.plasma.ann.advisory",
                    serde_json::to_vec(&advisory)?.into()
                ).await?;
            }
        }
    });
    
    Ok(())
}
```

**Location:** `crates/sx9-plasma-defender/src/tool_handler.rs` (NEW FILE)

---

### 2. ❌ ANN Daemon Module Missing

**Problem:** The current `sx9-plasma-defender` crate doesn't have the ANN daemon module.

**Missing Files:**
- `crates/sx9-plasma-defender/src/ann_daemon.rs`
- `crates/sx9-plasma-defender/src/advisory.rs`

**Source:** These exist in `ctas-7-shipyard-staging/06 Document and Code Drop/extracted/sx9-plasma-defender/` but need to be copied to the workspace crate.

**Action:** Copy from extracted crate to workspace crate.

---

### 3. ❌ Base64 Module Missing

**Problem:** Kali Plasma agent uses `base64::encode` but no base64 dependency.

**Missing:**
```toml
# In tools/kali-plasma/agent/Cargo.toml
[dependencies]
base64 = "0.21"  # Add this
```

**Also:** The `tunnel.rs` has a custom base64 implementation that should be replaced with the crate.

---

### 4. ❌ Missing Imports in plasma-agent

**Problem:** `futures_util::StreamExt` needed for NATS subscriber.

**Fix:**
```rust
// In tools/kali-plasma/agent/src/main.rs
use futures_util::StreamExt;  // Add this
```

**Status:** Already in Cargo.toml, just needs import.

---

### 5. ❌ Configuration Files Missing

**Problem:** No actual TOML config files for plasma-agent.

**Missing:**
- `/etc/plasma/plasma-agent.toml` (or `config/plasma-agent.toml` in repo)
- `/etc/plasma/crystals.toml`
- `/etc/plasma/gates.toml`
- `/etc/plasma/tools.toml`

**Action:** Create example configs in `tools/kali-plasma/config/`.

---

### 6. ❌ Plasma-Defender Main Entry Point

**Problem:** No `main.rs` for Plasma-Defender to start the service and subscribe to NATS.

**Missing:**
```rust
// crates/sx9-plasma-defender/src/main.rs

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DefenderConfig::load()?;
    let defender = PlasmaDefender::new(config).await?;
    
    // Subscribe to tool results
    subscribe_tool_results(defender.plasma_bus.clone()).await?;
    
    // Start server
    defender.start().await?;
    
    Ok(())
}
```

---

### 7. ❌ Error Handling in plasma-agent

**Problem:** Missing error handling for base64 decode, serde deserialize.

**Missing:**
```rust
// In tools/kali-plasma/agent/src/main.rs
use base64::{Engine as _, engine::general_purpose};

// Replace custom base64 with:
let payload = general_purpose::STANDARD.decode(&wire.payload)?;
```

---

### 8. ❌ ANN Advisory Struct Definition

**Problem:** `AnnAdvisory` struct not defined in plasma-agent.

**Missing:**
```rust
// In tools/kali-plasma/agent/src/main.rs
#[derive(Debug, Clone, serde::Deserialize)]
struct AnnAdvisory {
    confidence: f32,
    recommendation: String,
    reason_trace: Vec<String>,
}
```

**Status:** Already added in recent changes, but needs verification.

---

### 9. ❌ Plasma-Defender Config Missing ANN Settings

**Problem:** `DefenderConfig` doesn't have ANN daemon configuration.

**Missing:**
```rust
// In crates/sx9-plasma-defender/src/config.rs
pub struct DefenderConfig {
    // ... existing fields ...
    
    // ANN configuration
    pub ann_enabled: bool,
    pub ann_config: Option<AnnConfig>,
}
```

---

### 10. ❌ Tool Result Message Struct

**Problem:** No struct definition for `ToolResultMessage` that Plasma-Defender expects.

**Missing:**
```rust
// In crates/sx9-plasma-defender/src/tool_handler.rs
#[derive(Debug, Clone, serde::Deserialize)]
struct ToolResultMessage {
    operator_id: String,
    tool: String,
    result: String,  // base64 encoded
    success: bool,
    timestamp: u64,
}
```

---

## Summary Checklist

- [ ] **Copy ANN daemon modules** from extracted crate to workspace
- [ ] **Add base64 dependency** to plasma-agent Cargo.toml
- [ ] **Replace custom base64** with base64 crate
- [ ] **Add futures_util import** to plasma-agent main.rs
- [ ] **Create tool_handler.rs** in Plasma-Defender
- [ ] **Add subscribe_tool_results()** function
- [ ] **Create main.rs** for Plasma-Defender
- [ ] **Add ANN config** to DefenderConfig
- [ ] **Create config files** for plasma-agent
- [ ] **Add error handling** for base64/serde
- [ ] **Test NATS message flow** (Kali Plasma → Plasma-Defender → Advisory)

---

## Priority Order

1. **HIGH:** Copy ANN daemon modules (blocks ANN functionality)
2. **HIGH:** Create tool_handler.rs (blocks tool result processing)
3. **MEDIUM:** Add base64 dependency (compilation error)
4. **MEDIUM:** Create main.rs for Plasma-Defender (service won't start)
5. **LOW:** Configuration files (can use defaults for now)

---

## Next Steps

1. Copy `ann_daemon.rs` and `advisory.rs` from extracted crate
2. Create `tool_handler.rs` with NATS subscription
3. Add base64 dependency
4. Create Plasma-Defender main.rs
5. Test end-to-end: Kali Plasma → Plasma-Defender → ANN → Advisory


