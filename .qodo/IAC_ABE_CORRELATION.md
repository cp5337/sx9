# IAC-ABE Correlation: Orchestrator ↔ ABE System Integration

**Version:** 7.3.1  
**Date:** November 28, 2025  
**Status:** Integration Specification

---

## Executive Summary

This document correlates the **CTAS-7 Orchestrator IAC triggers** with the **ABE (Automated Business Environment) system** to enable end-to-end infrastructure spawning from HD4 phase transitions.

---

## Unicode Trigger Mapping

### Orchestrator IAC Manifolds → ABE Resources

| Orchestrator IAC Type | Unicode | ABE Manifold | Terraform Path | Purpose |
|------------------------|---------|--------------|----------------|---------|
| `StrategicPlanning` | `0xEA01` | ✅ **ABE Customer Environment** | `04-abe-iac/abe-qa-system/iac-manifolds/abe-customer-env/` | Strategic planning, ABE workspace |
| `ValidationCluster` | `0xEA11` | ⚠️ **Not yet mapped** | *TBD* | Complex data validation, QA processing |
| `SmartCrateOverflow` | `0xEA20` | ⚠️ **Not yet mapped** | *TBD* | Dynamic crate overflow, execution capacity |
| `PortExpansion` | `0xEA21` | ⚠️ **Not yet mapped** | *TBD* | Port exhaustion relief |
| `InfrastructureError` | `0xEAFF` | ⚠️ **Not yet mapped** | *TBD* | Emergency infrastructure |

### ABE IAC Manifolds (from Linear ATLAS Node)

| ABE Manifold | Unicode | Terraform Path | Compute Tier | Cost/min |
|--------------|---------|----------------|--------------|----------|
| `abe-customer-env-burst` | `\u{EA01}` | `./manifolds/abe-customer-env` | GPU (Tesla V100) | $0.50 |
| `cuda-parallel-cluster` | `\u{EA02}` | `./manifolds/cuda-parallel-cluster` | 8x GPU instances | $12.00 |
| `conda-scientific-env` | `\u{EA03}` | `./manifolds/conda-scientific` | Scientific packages | $2.00 |

---

## Correlation Status

### ✅ **CORRELATED**

**StrategicPlanning (0xEA01) ↔ ABE Customer Environment**
- **Orchestrator:** Triggers on `HD4Phase::Dominate` or `VerticalLevel::Strategic` escalation
- **ABE:** Spawns ABE customer environment with GPU support
- **Terraform:** `04-abe-iac/abe-qa-system/iac-manifolds/abe-customer-env/main.tf`
- **Resources:** GKE cluster, Lightning QA Engine, GPU nodes, Pub/Sub, Storage

### ⚠️ **NEEDS MAPPING**

1. **ValidationCluster (0xEA11)**
   - **Use Case:** HD4 Disable phase approval workflow
   - **ABE Resource:** Could use ABE customer environment with QA compute tier
   - **Action:** Map to `abe-customer-env` with `qa_compute_tier = "expert_45s_multi_gpu"`

2. **SmartCrateOverflow (0xEA20)**
   - **Use Case:** Tactical escalation, execution capacity overflow
   - **ABE Resource:** Could use CUDA parallel cluster for burst compute
   - **Action:** Map to `cuda-parallel-cluster` manifold

3. **PortExpansion (0xEA21)**
   - **Use Case:** HD4 Disrupt phase, traffic rerouting
   - **ABE Resource:** Could spawn additional ABE services
   - **Action:** Map to ABE service scaling

4. **InfrastructureError (0xEAFF)**
   - **Use Case:** Emergency state, critical failures
   - **ABE Resource:** Full ABE environment with all services
   - **Action:** Map to complete ABE stack

---

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  CTAS-7 Orchestrator (Port 18111)                               │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ IAC Controller                                            │  │
│  │ • issue_unicode_trigger()                                 │  │
│  │ • Unicode: 0xEA01, 0xEA11, 0xEA20, 0xEA21, 0xEAFF        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ NATS/Kafka Subject: "iac.triggers.abe"                    │  │
│  │ Message: { unicode: "\\u{EA01}", level: "Strategic" }    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ ABE Linear ATLAS Cognitive Node                           │  │
│  │ • Listens on "iac.triggers.abe"                           │  │
│  │ • Maps Unicode to IACManifold                             │  │
│  │ • Calls spawn_iac_manifold()                              │  │
│  └───────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Terraform Execution                                        │  │
│  │ • terraform apply -auto-approve                            │  │
│  │ • Path: 04-abe-iac/abe-qa-system/iac-manifolds/...       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ ABE Infrastructure Spawned                                 │  │
│  │ • GKE Cluster                                              │  │
│  │ • Lightning QA Engine (Port 18109)                         │  │
│  │ • GPU Nodes                                                │  │
│  │ • Pub/Sub Topics                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
```

---

## HD4 Phase → ABE Resource Mapping

| HD4 Phase | Orchestrator IAC Trigger | ABE Manifold | Terraform Module | Compute Tier |
|-----------|--------------------------|--------------|------------------|--------------|
| **Hunt** | None | N/A | N/A | N/A |
| **Detect** | None | N/A | N/A | N/A |
| **Disable** | `ValidationCluster` (0xEA11) | `abe-customer-env` | `abe-customer-env/` | `expert_45s_multi_gpu` |
| **Disrupt** | `PortExpansion` (0xEA21) | `abe-customer-env` | `abe-customer-env/` | `lightning_2s_gpu` |
| **Dominate** | `StrategicPlanning` (0xEA01) | ✅ `abe-customer-env` | `abe-customer-env/` | `burst_unlimited` |

---

## Vertical Escalation → ABE Resource Mapping

| Vertical Level | Orchestrator IAC Trigger | ABE Manifold | Purpose |
|----------------|--------------------------|--------------|---------|
| **Tactical** | `SmartCrateOverflow` (0xEA20) | `cuda-parallel-cluster` | More execution capacity |
| **Operational** | `ValidationCluster` (0xEA11) | `abe-customer-env` | More analytical power (QA) |
| **Strategic** | `StrategicPlanning` (0xEA01) | ✅ `abe-customer-env` | ABE planning environment |
| **National** | `InfrastructureError` (0xEAFF) | All ABE services | Emergency state |

---

## Implementation Requirements

### 1. Orchestrator → ABE Communication

**Current State:**
- Orchestrator logs IAC triggers (simulated)
- ABE Linear ATLAS node has `spawn_iac_manifold()` function
- **Gap:** No connection between them

**Required:**
- Replace simulated triggers with NATS/Kafka publish
- ABE node subscribes to `iac.triggers.abe` subject
- Unicode trigger → ABE manifold mapping

### 2. Unicode Code Alignment

**Current State:**
- ✅ `0xEA01` matches: Orchestrator `StrategicPlanning` = ABE `abe-customer-env`
- ⚠️ Other codes need mapping or new ABE manifolds

**Required:**
- Map `0xEA11` (ValidationCluster) → ABE QA environment
- Map `0xEA20` (SmartCrateOverflow) → CUDA cluster
- Map `0xEA21` (PortExpansion) → ABE service scaling
- Map `0xEAFF` (InfrastructureError) → Full ABE stack

### 3. Terraform Module Paths

**ABE Terraform Locations:**
- `04-abe-iac/abe-qa-system/iac-manifolds/abe-customer-env/`
- `04-abe-iac/cognetix-abe/` (main ABE infrastructure)

**Required:**
- Orchestrator IAC controller needs Terraform path mapping
- Or: Orchestrator publishes to NATS, ABE node executes Terraform

---

## Integration Options

### Option 1: Direct Terraform Execution (Recommended)

**Orchestrator IAC Controller** directly executes Terraform:

```rust
pub async fn issue_unicode_trigger(&self, manifold_type: IACManifoldType, level: VerticalLevel) -> Result<()> {
    // Map to ABE Terraform path
    let terraform_path = match manifold_type {
        IACManifoldType::StrategicPlanning => "04-abe-iac/abe-qa-system/iac-manifolds/abe-customer-env",
        IACManifoldType::ValidationCluster => "04-abe-iac/abe-qa-system/iac-manifolds/abe-customer-env",
        // ...
    };
    
    // Execute Terraform
    self.execute_terraform(terraform_path, manifold_type, level).await
}
```

**Pros:**
- Direct control
- No additional service dependency
- Faster execution

**Cons:**
- Orchestrator needs Terraform binary
- Duplicates ABE node functionality

### Option 2: NATS/Kafka Message Bus (Recommended for Production)

**Orchestrator** publishes Unicode trigger to NATS:

```rust
pub async fn issue_unicode_trigger(&self, manifold_type: IACManifoldType, level: VerticalLevel) -> Result<()> {
    // Publish to NATS
    nats_client.publish("iac.triggers.abe", json!({
        "unicode": manifold_type.to_unicode(),
        "level": level,
        "manifold_type": manifold_type.name(),
    })).await?;
}
```

**ABE Linear ATLAS Node** subscribes and spawns:

```python
async def handle_iac_trigger(message):
    unicode_char = message.data["unicode"]
    manifold = find_manifold_by_unicode(unicode_char)
    await spawn_iac_manifold(manifold)
```

**Pros:**
- Decoupled architecture
- ABE node handles all Terraform execution
- Scalable

**Cons:**
- Requires NATS/Kafka infrastructure
- Additional network hop

### Option 3: HTTP API (Hybrid)

**Orchestrator** calls ABE node HTTP API:

```rust
pub async fn issue_unicode_trigger(&self, manifold_type: IACManifoldType, level: VerticalLevel) -> Result<()> {
    let response = reqwest::Client::new()
        .post("http://localhost:18180/api/manifolds/spawn")
        .json(&json!({
            "unicode": manifold_type.to_unicode(),
            "level": level,
        }))
        .send()
        .await?;
}
```

**Pros:**
- Simple HTTP integration
- ABE node handles Terraform
- Easy to test

**Cons:**
- Requires ABE node to be running
- HTTP overhead

---

## Recommended Implementation

**Phase 1: Immediate (Option 2 - NATS)**
1. Add NATS client to orchestrator
2. Publish Unicode triggers to `iac.triggers.abe`
3. ABE node subscribes and spawns manifolds

**Phase 2: Enhanced (Option 1 - Direct)**
1. Add Terraform execution to orchestrator
2. Map IAC types to Terraform paths
3. Execute Terraform directly for faster spawning

---

## Unicode Code Registry

### Orchestrator IAC Codes (RFC-9004/9101)

| Code | Name | Purpose |
|------|------|---------|
| `0xEA01` | StrategicPlanning | ABE planning environment |
| `0xEA11` | ValidationCluster | QA/validation processing |
| `0xEA20` | SmartCrateOverflow | Execution capacity |
| `0xEA21` | PortExpansion | Port relief |
| `0xEAFF` | InfrastructureError | Emergency state |

### ABE Manifold Codes (from Linear ATLAS Node)

| Code | Name | Terraform Path |
|------|------|----------------|
| `0xEA01` | abe-customer-env | `./manifolds/abe-customer-env` |
| `0xEA02` | cuda-parallel-cluster | `./manifolds/cuda-parallel-cluster` |
| `0xEA03` | conda-scientific-env | `./manifolds/conda-scientific` |

### Alignment

✅ **Perfect Match:** `0xEA01` = StrategicPlanning = ABE Customer Environment

⚠️ **Needs Mapping:**
- `0xEA11` → Could use `abe-customer-env` with validation tier
- `0xEA20` → Could use `cuda-parallel-cluster`
- `0xEA21` → Could use `abe-customer-env` with port scaling
- `0xEAFF` → Could use all ABE services

---

## Next Steps

1. **Enhance IAC Controller** to publish to NATS or call ABE API
2. **Map remaining Unicode codes** to ABE manifolds
3. **Add Terraform path resolution** in orchestrator
4. **Test end-to-end flow:** HD4 Disable → ValidationCluster → ABE QA Environment
5. **Document Terraform variables** for each IAC trigger

---

## Files to Modify

1. `ctas7-orchestrator/src/iac_interface.rs` - Add NATS/HTTP client
2. `04-abe-iac/abe-qa-system/linear-integration/linear_atlas_cognitive_node.py` - Add NATS subscriber
3. Create mapping configuration file for Unicode → Terraform paths

---

**Status:** ✅ Correlation identified, implementation pending




