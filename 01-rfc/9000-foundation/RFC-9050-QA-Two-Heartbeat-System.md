# RFC-9050: QA Two-Heartbeat System

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

Zero-Trust Dual Heartbeat architecture with independent Quality and Security verification planes.

### 1.1 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        ZERO-TRUST DUAL HEARTBEAT                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐    │
│  │   sx9-foundation-core       │    │   sx9-plasma-defender       │    │
│  │   HEARTBEAT 1: QUALITY      │    │   HEARTBEAT 2: SECURITY     │    │
│  ├─────────────────────────────┤    ├─────────────────────────────┤    │
│  │                             │    │                             │    │
│  │ health_network.rs (453)     │    │ health.rs (30 → ~100)       │    │
│  │ qa_validator.rs (~150) NEW  │    │ security_monitor.rs NEW     │    │
│  │                             │    │                             │    │
│  │ • Hash integrity            │    │ • SDT state                 │    │
│  │ • Grade enforcement         │    │ • Threat excitation         │    │
│  │ • Complexity thresholds     │    │ • SARIF violations          │    │
│  │ • Registry compliance       │    │ • Anomaly detection         │    │
│  │                             │    │                             │    │
│  │ UDP 239.1.7.53:17053        │    │ UDP 239.1.7.53:17054        │    │
│  │ (Quality Plane)             │    │ (Security Plane)            │    │
│  └──────────────┬──────────────┘    └──────────────┬──────────────┘    │
│                 │                                   │                   │
│                 └───────────────┬───────────────────┘                   │
│                                 │                                       │
│                                 ▼                                       │
│                    ┌─────────────────────────┐                         │
│                    │   SmartCrate Gateway    │                         │
│                    │   (Listens to both)     │                         │
│                    │                         │                         │
│                    │   BLOCK if:             │                         │
│                    │   • Quality: CRITICAL   │                         │
│                    │   • Security: ELEVATED  │                         │
│                    └─────────────────────────┘                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 2. The Split

| Heartbeat | Crate | Port | Watches | Blocks On |
|-----------|-------|------|---------|-----------|
| **Quality** | `sx9-foundation-core` | 17053 | Grade, complexity, LOC, hash | Grade < threshold, registry violation |
| **Security** | `sx9-plasma-defender` | 17054 | SARIF issues, threat level, SDT | Critical CVE, anomaly spike |

---

## 3. Heartbeat α (Quality) - Foundation

### 3.1 Location

```
sx9-foundation-core/src/
├── health_network.rs    # Existing (453 lines)
└── qa_validator.rs      # NEW (~150 lines)
```

### 3.2 Quality Metrics

```rust
#[derive(Debug, Clone)]
pub struct QualityHeartbeat {
    pub grade: Grade,
    pub complexity_score: f64,
    pub loc_count: u32,
    pub hash_valid: bool,
    pub registry_compliant: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grade {
    A = 5,
    B = 4,
    C = 3,
    D = 2,
    F = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum QualityHealth {
    Healthy,
    Degraded,
    Critical,
}
```

### 3.3 QA Validator

```rust
pub struct QaValidator {
    pub thresholds: QaThresholds,
    pub guardrails: QaGuardrails,
}

#[derive(Debug, Clone)]
pub struct QaThresholds {
    pub min_grade: Grade,
    pub max_complexity: f64,
    pub max_loc_per_file: u32,
}

impl QaValidator {
    /// Validate against TOML spec, return JSON metrics
    pub fn validate(&self, crate_metrics: &CrateMetrics) -> QualityHeartbeat {
        let grade = self.calculate_grade(crate_metrics);
        let complexity = crate_metrics.cyclomatic_complexity;
        
        QualityHeartbeat {
            grade,
            complexity_score: complexity,
            loc_count: crate_metrics.total_loc,
            hash_valid: self.verify_hash(crate_metrics),
            registry_compliant: self.check_registry(crate_metrics),
            timestamp: Utc::now(),
        }
    }
    
    pub fn health_level(&self, heartbeat: &QualityHeartbeat) -> QualityHealth {
        if heartbeat.grade < self.thresholds.min_grade {
            return QualityHealth::Critical;
        }
        if heartbeat.complexity_score > self.thresholds.max_complexity {
            return QualityHealth::Degraded;
        }
        QualityHealth::Healthy
    }
}
```

---

## 4. Heartbeat β (Security) - Plasma

### 4.1 Location

```
sx9-plasma-defender/src/
├── health.rs            # Existing (30 lines → expand)
└── security_monitor.rs  # NEW
```

### 4.2 Security Metrics

```rust
#[derive(Debug, Clone)]
pub struct SecurityHeartbeat {
    pub sdt_state: SdtState,
    pub threat_excitation: f64,
    pub sarif_critical: u32,
    pub sarif_high: u32,
    pub anomaly_score: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityHealth {
    Normal,
    Elevated,
    Critical,
}

impl SecurityMonitor {
    pub fn evaluate(&self, heartbeat: &SecurityHeartbeat) -> SecurityHealth {
        if heartbeat.sarif_critical > 0 {
            return SecurityHealth::Critical;
        }
        if heartbeat.threat_excitation > 0.7 || heartbeat.anomaly_score > 0.8 {
            return SecurityHealth::Elevated;
        }
        SecurityHealth::Normal
    }
}
```

### 4.3 SARIF Integration

Security heartbeat consumes SARIF (Static Analysis Results Interchange Format) output:

```rust
pub struct SarifConsumer {
    pub results_path: PathBuf,
}

impl SarifConsumer {
    pub fn parse_results(&self) -> SarifSummary {
        // Parse SARIF JSON, extract critical/high/medium/low counts
    }
}
```

---

## 5. SmartCrate Gateway

### 5.1 AND Gate Logic

SmartCrate listens to both heartbeats and enforces an AND gate:

```rust
pub struct SmartCrateGateway {
    pub quality_receiver: UdpSocket,   // :17053
    pub security_receiver: UdpSocket,  // :17054
}

impl SmartCrateGateway {
    pub fn should_allow(&self) -> bool {
        let quality = self.receive_quality();
        let security = self.receive_security();
        
        // Must pass BOTH gates
        quality.health_level() != QualityHealth::Critical &&
        security.health_level() != SecurityHealth::Critical
    }
}
```

### 5.2 SmartCrate Port Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SMARTCRATE PORT                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  INBOUND ──►  ┌─────────────────┐                                      │
│               │ C2 Beacon Eater │ ◄── Plasma Security Heartbeat        │
│               │ (Security)      │                                      │
│               └────────┬────────┘                                      │
│                        │ Clean traffic only                            │
│                        ▼                                               │
│               ┌─────────────────┐                                      │
│               │ QA Validator    │ ◄── Foundation Quality Heartbeat     │
│               │ (TOML→JSON)     │                                      │
│               └────────┬────────┘                                      │
│                        │ Passes both gates                             │
│                        ▼                                               │
│               ┌─────────────────┐                                      │
│               │ SmartCrate Core │                                      │
│               └─────────────────┘                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Dual Heartbeat Verification

### 6.1 Offset Timing

```rust
pub struct DualHeartbeat {
    pub interval: Duration,    // Default: 30s
    pub offset: Duration,      // Default: 15s (β offset from α)
    pub last_alpha: Option<HeartbeatRecord>,
    pub last_beta: Option<HeartbeatRecord>,
}
```

### 6.2 Divergence Detection

```rust
#[derive(Debug, Clone)]
pub struct HeartbeatDivergence {
    pub alpha: HeartbeatRecord,
    pub beta: HeartbeatRecord,
    pub diverged_on: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl DualHeartbeat {
    pub fn check_divergence(&self) -> Option<HeartbeatDivergence> {
        // If α and β report conflicting state, flag divergence
        // Triggers HALT + ROLLBACK to gold disc
    }
}
```

---

## 7. QA Guardrails TOML

```toml
# qa-guardrails.toml

[thresholds]
min_grade = "C"
max_complexity = 15.0
max_loc_per_file = 500

[layer.foundation]
# Strictest - core infrastructure
min_grade = "B"
max_complexity = 10.0
max_loc_per_file = 300

[layer.application]
# Standard
min_grade = "C"
max_complexity = 15.0
max_loc_per_file = 500

[layer.tool]
# Relaxed for scripts
min_grade = "D"
max_complexity = 20.0
max_loc_per_file = 800

[patterns]
forbidden = [
    "unwrap\\(\\)",      # Use expect() or ?
    "panic!",            # Use Result
    "unsafe",            # Requires review
]

[clippy]
deny = ["clippy::correctness"]
warn = ["clippy::suspicious", "clippy::complexity"]
```

---

## 8. Health Network Integration

Foundation's health_network.rs (453 lines) provides:

- UDP multicast for efficient network communication
- Global health state tracking across all orchestrators
- Hash integrity verification
- TOML export for reporting
- Critical path routing

```rust
// sx9-foundation-core/src/health_network.rs

pub struct HealthNetwork {
    pub multicast_addr: SocketAddr,  // 239.1.7.53:17053
    pub node_id: Uuid,
    pub health_state: Arc<RwLock<GlobalHealthState>>,
}
```

---

## Critical Constraints

- **Two independent heartbeats** - Quality AND Security
- **AND gate** - Must pass both to proceed
- **TOML for specs** - qa-guardrails.toml defines thresholds
- **JSON for metrics** - Runtime health data
- **UDP multicast** - Efficient broadcast
- **15s offset** - α and β staggered to detect drift

---

## References

- RFC-9024/9025: Dual H1/H2 Architecture
- RFC-9400: NATS Architecture
- RFC-9301: TCR Triad
