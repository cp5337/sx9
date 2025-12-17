# RFC-9024 — H2 Semantic Convergence Service Contract

**Version:** 1.0  
**Status:** Canonical Interface Definition  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9021, RFC-9023, RFC-9004  

---

## 1. Purpose and Scope

This specification defines the canonical REST/gRPC service interface used by the **ATLAS Daemon (Zone B)** to query pre-computed **H2 Semantic Convergence Scores** from the **GLAF Matroid Core (Zone C)**.

### 1.1 Dual Neurotransmitter Principle

The interface adheres to the **Dual Neurotransmitter Systems** principle:

| System | Analogy | Zone | Latency | Purpose |
|--------|---------|------|---------|---------|
| **H1** | Dopamine (fast) | Zone B | <1ms | Operational signal |
| **H2** | Serotonin (slow) | Zone C | 1-100ms | Semantic context |

The H2 score is generated asynchronously and persistently, providing the **Slow Modulatory context** needed to stabilize the fast H1 Operational signal.

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    H2 CONVERGENCE ARCHITECTURE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE A (Reflexive)                          │   │
│  │                         Latency: <1ms                               │   │
│  │  ┌─────────────┐                                                    │   │
│  │  │ Thalamic    │  Fast gate decision                               │   │
│  │  │ Filter      │  (RFC-9021 Layer 1)                               │   │
│  │  └──────┬──────┘                                                    │   │
│  └─────────┼───────────────────────────────────────────────────────────┘   │
│            │                                                                │
│            ▼                                                                │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE B (Operational)                        │   │
│  │                         Latency: 1-10ms                             │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │                    ATLAS DAEMON                              │   │   │
│  │  │                    Port: 18200                               │   │   │
│  │  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │   │   │
│  │  │  │ H1 Score    │    │ Cognitive   │    │ HD4 Phase   │     │   │   │
│  │  │  │ (fast)      │───▶│ Tick Loop   │───▶│ Router      │     │   │   │
│  │  │  └─────────────┘    └──────┬──────┘    └─────────────┘     │   │   │
│  │  │                            │                                │   │   │
│  │  │                     ASYNC  │  H2 Query                      │   │   │
│  │  │                            ▼                                │   │   │
│  │  └────────────────────────────┼────────────────────────────────┘   │   │
│  └───────────────────────────────┼────────────────────────────────────┘   │
│                                  │                                         │
│                    GET /api/v7/convergence/semantic/{sch_prefix}          │
│                                  │                                         │
│                                  ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ZONE C (Analytical)                         │   │
│  │                         Latency: 1-100ms                            │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │                 GLAF MATROID CORE                            │   │   │
│  │  │                 Port: 18300                                  │   │   │
│  │  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │   │   │
│  │  │  │ Matroid     │    │ Combinator- │    │ HMM Phase   │     │   │   │
│  │  │  │ Rank Calc   │───▶│ ial Optim.  │───▶│ Estimator   │     │   │   │
│  │  │  └─────────────┘    └─────────────┘    └──────┬──────┘     │   │   │
│  │  │                                               │            │   │   │
│  │  │                                               ▼            │   │   │
│  │  │                                      H2ConvergencePayload  │   │   │
│  │  └───────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. H2 Convergence Data Model

### 3.1 Payload Structure

```rust
/// Canonical Data Model for Semantic Convergence Score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H2ConvergencePayload {
    /// The core metric derived from the Combinatorial Optimizer (RFC-9021 §3.1)
    /// Range: 0.0 - 1.0
    pub semantic_score: f64,

    /// Matroid Independence Metrics (RFC-9023)
    /// Used to track redundancy and information value
    pub matroid_rank_current: usize,
    pub matroid_rank_delta: i32,  // Change since last check

    /// Hidden Markov Model (HMM) Input
    /// Estimated phase of the adversary based on sequence of PTCC primitives
    pub estimated_adversary_phase: AdversaryPhase,

    /// Secondary Trivariate SCH-S (Semantic Context Hash)
    /// For supersession tracking
    pub secondary_hash_sch: String,

    /// Timestamp when this score was generated (for decay calculation)
    pub generated_at_utc: DateTime<Utc>,

    /// Recommended action based on pattern match
    pub recommended_hd4_phase: Hd4Phase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdversaryPhase {
    Recon,
    Staging,
    Execution,
    Exfil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Hd4Phase {
    Hunt,
    Detect,
    Disable,
    Disrupt,
    Dominate,
}
```

### 3.2 Field Semantics

| Field | Source | Update Frequency | Purpose |
|-------|--------|------------------|---------|
| `semantic_score` | CombinatorialOptimizer | Per fragment batch | Primary H2 metric |
| `matroid_rank_current` | LatentMatroid::calculate_rank | Real-time | Information independence |
| `matroid_rank_delta` | Diff vs. previous | Per query | Trend detection |
| `estimated_adversary_phase` | HMM on PTCC sequence | Sliding window | Kill chain position |
| `secondary_hash_sch` | RFC-9001 SCH-S | Per context change | Supersession tracking |
| `recommended_hd4_phase` | Pattern matcher | Per query | Tactical guidance |

---

## 4. Service Endpoint Specification

### 4.1 Primary Endpoint

| Property | Value |
|----------|-------|
| **Method** | `GET` |
| **Path** | `/api/v7/convergence/semantic/{triv_sch_t_prefix}` |
| **Path Parameter** | `triv_sch_t_prefix`: Top 16 Base96 characters of Primary Trivariate Hash |
| **Zone** | Zone C (Analytical) |
| **Latency** | 1ms - 100ms |
| **Port** | 18300 |
| **Response** | `H2ConvergencePayload` (JSON) |

### 4.2 Request Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         H2 QUERY SEQUENCE                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ATLAS Daemon                         GLAF Matroid Core                     │
│  (Zone B)                             (Zone C)                              │
│      │                                      │                               │
│      │  GET /api/v7/convergence/semantic/   │                               │
│      │      {sch_prefix}                    │                               │
│      │─────────────────────────────────────▶│                               │
│      │                                      │                               │
│      │                         ┌────────────┴────────────┐                  │
│      │                         │ 1. Lookup context tree  │                  │
│      │                         │ 2. Calculate matroid    │                  │
│      │                         │    rank if stale        │                  │
│      │                         │ 3. Run HMM inference    │                  │
│      │                         │ 4. Generate payload     │                  │
│      │                         └────────────┬────────────┘                  │
│      │                                      │                               │
│      │  200 OK                              │                               │
│      │  H2ConvergencePayload (JSON)         │                               │
│      │◀─────────────────────────────────────│                               │
│      │                                      │                               │
│      │  (ASYNC - does NOT block 1ms tick)   │                               │
│      │                                      │                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.3 Error Responses

| Status | Meaning | Action |
|--------|---------|--------|
| 200 | Success | Use payload |
| 404 | SCH prefix not found | Use stale H2 or default |
| 408 | Timeout (>100ms) | Use stale H2 |
| 500 | Internal error | Log, use stale H2 |

---

## 5. ATLAS Daemon Integration

### 5.1 Client Implementation

```rust
// ctas7-atlas-daemon/src/glaf_client.rs

use ctas7_foundation_core::PrimaryTrivariate;
use reqwest::Client;
use std::time::Duration;
use anyhow::Result;

pub struct GlafClient {
    client: Client,
    endpoint: String,  // http://glaf-matroid-core:18300
}

impl GlafClient {
    pub fn new(endpoint: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(150))  // Hard cap above Zone C limit
            .build()
            .expect("Failed to build HTTP client");
            
        Self {
            client,
            endpoint: endpoint.to_string(),
        }
    }

    /// Fetches the H2 Semantic Score (Slow Modulatory Input)
    /// This runs asynchronously and does NOT block the 1ms cognitive tick.
    pub async fn get_h2_score(
        &self, 
        primary_hash: &PrimaryTrivariate
    ) -> Result<H2ConvergencePayload> {
        // Extract SCH prefix (upper 64 bits as hex)
        let sch_prefix = format!("{:016x}", primary_hash.sch_t >> 64);
        
        let url = format!(
            "{}/api/v7/convergence/semantic/{}", 
            self.endpoint, 
            sch_prefix
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<H2ConvergencePayload>()
            .await?;

        Ok(response)
    }
}
```

### 5.2 Async Integration Pattern

```rust
// ctas7-atlas-daemon/src/cognitive_loop.rs

use tokio::sync::watch;
use std::sync::Arc;

pub struct CognitiveLoop {
    h2_cache: Arc<RwLock<Option<H2ConvergencePayload>>>,
    glaf_client: GlafClient,
}

impl CognitiveLoop {
    /// Main 1ms tick loop - NEVER blocks on H2
    pub async fn run_tick(&self, h1_score: f64) -> TickResult {
        // Use cached H2 (may be stale, that's OK per dual-neurotransmitter model)
        let h2 = self.h2_cache.read().await.clone();
        
        // Fast path: combine H1 + cached H2
        let combined = self.compute_combined_score(h1_score, h2.as_ref());
        
        TickResult { combined, h2_age_ms: h2.map(|h| h.age_ms()).unwrap_or(u64::MAX) }
    }
    
    /// Background H2 refresh - runs independently of tick loop
    pub async fn refresh_h2(&self, primary_hash: &PrimaryTrivariate) {
        match self.glaf_client.get_h2_score(primary_hash).await {
            Ok(payload) => {
                *self.h2_cache.write().await = Some(payload);
            }
            Err(e) => {
                tracing::warn!("H2 refresh failed: {}, using stale value", e);
                // Don't clear cache - stale H2 is better than no H2
            }
        }
    }
}
```

---

## 6. Time-of-Value Decay

### 6.1 Decay Function

H2 scores decay over time to reflect information staleness:

```rust
impl H2ConvergencePayload {
    /// Calculate effective H2 score with time decay
    pub fn effective_score(&self) -> f64 {
        let age_secs = (Utc::now() - self.generated_at_utc).num_seconds() as f64;
        
        // Exponential decay with 5-minute half-life
        let decay_factor = (-age_secs / 300.0).exp();
        
        self.semantic_score * decay_factor
    }
    
    /// Check if H2 is still valid (< 10 minute age)
    pub fn is_valid(&self) -> bool {
        let age = Utc::now() - self.generated_at_utc;
        age < ChronoDuration::minutes(10)
    }
}
```

### 6.2 Decay Curve

```
Effective Score
    1.0 ┤
        │ ████
    0.8 ┤     ████
        │         ████
    0.6 ┤             ████
        │                 ████
    0.4 ┤                     ████
        │                         ████
    0.2 ┤                             ████
        │                                 ████
    0.0 ┼────────────────────────────────────────▶ Time
        0    5min   10min  15min  20min  25min
        
        Half-life: 5 minutes
        Valid threshold: 10 minutes
```

---

## 7. Port Allocation

Per RFC-9004 Bernoulli Zone assignments:

| Service | Port | Zone | Latency |
|---------|------|------|---------|
| ATLAS Daemon | 18200 | B | 1-10ms |
| GLAF Matroid Core | 18300 | C | 1-100ms |
| Leptose Inference | 18114 | C | 10-500ms |
| Neural Mux | 18400 | B | <1ms |

---

## 8. Testing Specification

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_h2_decay() {
        let payload = H2ConvergencePayload {
            semantic_score: 1.0,
            generated_at_utc: Utc::now() - ChronoDuration::minutes(5),
            ..Default::default()
        };
        
        // At 5 minutes (half-life), score should be ~0.5
        let effective = payload.effective_score();
        assert!((effective - 0.5).abs() < 0.1);
    }
    
    #[test]
    fn test_h2_validity() {
        let fresh = H2ConvergencePayload {
            generated_at_utc: Utc::now(),
            ..Default::default()
        };
        assert!(fresh.is_valid());
        
        let stale = H2ConvergencePayload {
            generated_at_utc: Utc::now() - ChronoDuration::minutes(15),
            ..Default::default()
        };
        assert!(!stale.is_valid());
    }
}
```

### 8.2 Integration Tests

```rust
#[tokio::test]
async fn test_atlas_glaf_integration() {
    // Start mock GLAF server
    let mock = MockGlafServer::start().await;
    
    let client = GlafClient::new(&mock.url());
    let hash = PrimaryTrivariate::test_hash();
    
    let result = client.get_h2_score(&hash).await;
    assert!(result.is_ok());
    
    let payload = result.unwrap();
    assert!(payload.semantic_score >= 0.0 && payload.semantic_score <= 1.0);
}
```

---

## 9. References

- RFC-9004: Deterministic Routing (Bernoulli Zones)
- RFC-9021: Cognitive Inference Engine  
- RFC-9023: GLAF Matroid Convergence Mathematics
- RFC-9001: Trivariate Hashing

---

**End of RFC-9024**
