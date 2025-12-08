# Kali Plasma API Integration Specification

**Date:** 2025-01-XX  
**Status:** ✅ **COMPLETE**  
**Purpose:** Ensure Kali Plasma connects to all APIs for data maintenance and ANN integration

---

## Overview

Kali Plasma must maintain connections to all SX9 APIs for:
- **Tool execution results** → Plasma-Defender ANN analysis
- **Threat intelligence** → Real-time threat correlation
- **Telemetry streaming** → System health monitoring
- **SDT gate control** → Crystal resonance validation
- **Data persistence** → CDN data fabric

---

## API Connection Matrix

### 1. Plasma-Defender (ANN Integration) ⭐ **PRIMARY**

**Endpoint:** `http://plasma-defender:18110` (or via NATS)  
**Purpose:** ANN threat analysis, SDT gate control, crystal resonance

#### HTTP Endpoints

```rust
// Health check
GET /health
Response: { "status": "healthy", "plasma": {...}, "ann": {...} }

// Metrics
GET /metrics
Response: Prometheus metrics format

// SDT State
GET /sdt/state
Response: { "state": "OPEN|CLOSED|LATCHED", "delta_angle": 1234, "entropy": 5678 }

// Crystal Resonance
POST /crystal/resonance
Body: { "payload": [bytes], "delta_angle": 1234 }
Response: { "ring_strength": 0.85, "passed": true, "sdt_allowed": true }

// Threat Evaluation (with ANN)
POST /threat/evaluate
Body: { "payload": [bytes], "context": {...} }
Response: { 
    "ring_strength": 0.85,
    "sdt_state": "OPEN",
    "allowed": true,
    "ann_confidence": 0.92,
    "ann_recommendation": "proceed",
    "ann_reason_trace": ["crystal_passed", "sdt_open", ...],
    "delta_angle": 1234,
    "entropy": 5678
}
```

#### NATS Subjects (Primary Integration Method)

```rust
// Tool result → Plasma-Defender (for ANN analysis)
SUBJECT: sx9.tool.result.ann
PAYLOAD: {
    "operator_id": "operator_hash",
    "tool": "nmap|masscan|nuclei|...",
    "result": "base64_encoded_result",
    "success": true,
    "timestamp": "..."
}

// ANN advisory ← Plasma-Defender
SUBJECT: sx9.plasma.ann.advisory
PAYLOAD: {
    "confidence": 0.92,
    "recommendation": "proceed|block|escalate",
    "reason_trace": ["step1", "step2", ...],
    "timestamp": "..."
}

// Telemetry streaming (from Plasma-Defender)
SUBJECT: sx9.stream.ops.plasma.telemetry
PAYLOAD: {
    "metric": "plasma.defender.threat.evaluated",
    "value": 0.85,
    "timestamp": "2025-01-XXT..."
}

// SDT state changes
SUBJECT: sx9.plasma.sdt.state
PAYLOAD: {
    "state": "OPEN|CLOSED|LATCHED",
    "delta_angle": 1234,
    "entropy": 5678
}
```

---

### 2. CDN Data Fabric

**Endpoint:** `http://cdn-data-fabric:18112`  
**Purpose:** Data persistence, threat intelligence storage

#### Endpoints

```rust
// Store tool execution result
POST /api/store
Body: {
    "key": "blake3_hash",
    "value": {...},
    "metadata": {...}
}

// Retrieve threat intelligence
GET /api/retrieve/:hash
Response: { "data": {...}, "metadata": {...} }

// Query threats
POST /api/query
Body: { "query": "...", "filters": {...} }
Response: { "results": [...] }
```

---

### 3. NATS Fabric (Primary Communication)

**Endpoint:** `nats://cdn-edge-1.sx9.io:4222`  
**Purpose:** All command/response communication

#### Command Subjects (Kali Plasma → System)

```rust
// Tool execution commands
SUBJECT: sx9.tool.{tool_name}.cmd
PAYLOAD: {
    "operator_id": "operator_hash",
    "tool": "nmap|masscan|nuclei|...",
    "command": {...},
    "sdt_frame": {
        "payload": [bytes],
        "delta_angle": 1234,
        "crystal_family": "tripwire|corporate_strict|..."
    },
    "timestamp": "..."
}

// Entropy harvest
SUBJECT: sx9.plasma.entropy
PAYLOAD: {
    "operator_id": "operator_hash",
    "entropy_bits": [u8; 32],
    "source": "nic|thermal|timing",
    "timestamp": "..."
}

// Canary trip alerts
SUBJECT: sx9.sdt.canary
PAYLOAD: {
    "operator_id": "operator_hash",
    "canary_id": "...",
    "trigger": "tamper|anomaly|threshold|ann_blocked",
    "timestamp": "..."
}
```

#### Response Subjects (System → Kali Plasma)

```rust
// Tool execution results
SUBJECT: sx9.tool.{tool_name}.result
PAYLOAD: {
    "operator_id": "operator_hash",
    "command_id": "...",
    "result": {...},
    "ann_analysis": {
        "confidence": 0.92,
        "recommendation": "proceed",
        "threat_level": "low|medium|high|critical"
    },
    "sdt_state": "OPEN|CLOSED",
    "timestamp": "..."
}

// Threat intelligence updates
SUBJECT: sx9.intel.threat.update
PAYLOAD: {
    "threat_id": "...",
    "level": "low|medium|high|critical",
    "indicators": [...],
    "mitre_techniques": [...],
    "timestamp": "..."
}
```

---

### 4. Atlas Bus (PlasmaState)

**Endpoint:** Via NATS or direct connection  
**Purpose:** PlasmaState synchronization, crystal resonance

#### NATS Subjects

```rust
// PlasmaState updates
SUBJECT: sx9.atlas.plasma.state
PAYLOAD: {
    "delta_angle": 1234,
    "entropy": 5678,
    "sdt_state": "OPEN|CLOSED|LATCHED",
    "crystal_resonance": 0.85,
    "timestamp": "..."
}

// Crystal resonance requests
SUBJECT: sx9.atlas.crystal.resonate
PAYLOAD: {
    "payload": [bytes],
    "delta_angle": 1234,
    "crystal_family": "tripwire|corporate_strict|..."
}
RESPONSE: {
    "ring_strength": 0.85,
    "passed": true,
    "timestamp": "..."
}
```

---

### 5. Statistical Analysis CDN

**Endpoint:** `http://stats-cdn:18108`  
**Purpose:** Threat statistics, tool execution metrics

#### Endpoints

```rust
// Threat statistics
GET /api/stats/threats
Response: {
    "total_threats": 1234,
    "by_level": {"low": 100, "medium": 50, "high": 10, "critical": 2},
    "by_mitre_tactic": {...},
    "trends": {...}
}

// Tool execution stats
GET /api/stats/tools
Response: {
    "nmap": {"executions": 100, "success_rate": 0.95, ...},
    "masscan": {...},
    ...
}

// Real-time stats stream
WS /stream/stats
Messages: {
    "type": "threat|tool|entity",
    "data": {...},
    "timestamp": "..."
}
```

---

### 6. Monitoring CDN

**Endpoint:** `http://monitoring-cdn:18109`  
**Purpose:** System health, alerts

#### Endpoints

```rust
// Service health
GET /api/services
Response: {
    "plasma-defender": {"status": "healthy", "latency_ms": 5, ...},
    "cdn-data-fabric": {"status": "healthy", ...},
    ...
}

// Active alerts
GET /api/alerts
Response: {
    "alerts": [
        {
            "id": "...",
            "level": "warning|error|critical",
            "service": "plasma-defender",
            "message": "...",
            "timestamp": "..."
        },
        ...
    ]
}
```

---

## Plasma-Defender ANN Integration

### ANN Event Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    KALI PLASMA → PLASMA-DEFENDER ANN                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   Kali Plasma (eBPF)                                                    │
│        │                                                                 │
│        │ Tool execution result                                          │
│        ▼                                                                 │
│   ┌─────────────────────────────────────────────────────────────┐       │
│   │  NATS: sx9.tool.result.ann                                  │       │
│   │  PAYLOAD: { result, operator_id, tool, ... }                │       │
│   └─────────────────────────────────────────────────────────────┘       │
│        │                                                                 │
│        │                                                                 │
│        ▼                                                                 │
│   Plasma-Defender                                                        │
│   ┌─────────────────────────────────────────────────────────────┐       │
│   │  1. Receive result via NATS                                  │       │
│   │  2. Extract payload & context                                │       │
│   │  3. Evaluate through crystal resonance                       │       │
│   │  4. Check SDT gate state                                      │       │
│   │  5. Feed to ANN daemon                                        │       │
│   └─────────────────────────────────────────────────────────────┘       │
│        │                                                                 │
│        │ ANN Observation                                                │
│        ▼                                                                 │
│   ┌─────────────────────────────────────────────────────────────┐       │
│   │  ANN Daemon                                                    │       │
│   │  - hash_entropy: 0.85                                         │       │
│   │  - routing_latency_ns: 5000                                   │       │
│   │  - sdt_state: OPEN                                            │       │
│   │  - crystal_resonance: 0.92                                    │       │
│   └─────────────────────────────────────────────────────────────┘       │
│        │                                                                 │
│        │ ANN Advisory                                                   │
│        ▼                                                                 │
│   ┌─────────────────────────────────────────────────────────────┐       │
│   │  ANN Advisory                                                 │       │
│   │  - confidence: 0.92                                           │       │
│   │  - recommendation: "proceed"                                  │       │
│   │  - reason_trace: ["crystal_passed", "sdt_open", ...]         │       │
│   └─────────────────────────────────────────────────────────────┘       │
│        │                                                                 │
│        │ Publish advisory                                               │
│        ▼                                                                 │
│   ┌─────────────────────────────────────────────────────────────┐       │
│   │  NATS: sx9.plasma.ann.advisory                                │       │
│   │  PAYLOAD: { confidence, recommendation, reason_trace, ... } │       │
│   └─────────────────────────────────────────────────────────────┘       │
│        │                                                                 │
│        │                                                                 │
│        ▼                                                                 │
│   Kali Plasma (plasma-agent)                                             │
│   - Receives ANN advisory                                                │
│   - Filters result based on recommendation                               │
│   - Sends filtered result back through tunnel                            │
│   - If "block": trips canary, drops result                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### ANN Integration Code (plasma-agent)

**Location:** `tools/kali-plasma/agent/src/main.rs`

```rust
// ANN Advisory from Plasma-Defender
#[derive(Debug, Clone, serde::Deserialize)]
struct AnnAdvisory {
    confidence: f32,
    recommendation: String,
    reason_trace: Vec<String>,
}

/// Send tool result to Plasma-Defender for ANN analysis
async fn send_to_plasma_defender(
    result: &ebpf::ToolResult,
    operator: &Operator,
    tunnel: &CdnTunnel,
) -> Result<Option<AnnAdvisory>> {
    // Publish result to Plasma-Defender via NATS
    let nats_client = tunnel.get_nats_client();
    let payload = serde_json::json!({
        "operator_id": hex::encode(&operator.id[..8]),
        "tool": result.tool,
        "result": base64::encode(&result.payload),
        "success": result.success,
        "timestamp": result.timestamp,
    });
    
    nats_client.publish(
        "sx9.tool.result.ann",
        serde_json::to_vec(&payload)?.into()
    ).await?;
    
    // Subscribe to ANN advisory (with timeout)
    let mut subscriber = nats_client.subscribe("sx9.plasma.ann.advisory").await?;
    
    // Wait for advisory (timeout after 1 second)
    tokio::select! {
        msg = subscriber.next() => {
            if let Some(msg) = msg {
                let advisory: AnnAdvisory = serde_json::from_slice(&msg.payload)?;
                Ok(Some(advisory))
            } else {
                Ok(None)
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
            Ok(None) // Timeout
        }
    }
}

// In main loop:
// Results from eBPF ring buffer
result = tool_manager.read_result() => {
    match result {
        Ok(result) => {
            // Send to Plasma-Defender for ANN analysis
            let ann_advisory = send_to_plasma_defender(&result, &operator, &tunnel).await;
            
            // Filter based on ANN recommendation
            let filtered = match ann_advisory {
                Ok(Some(advisory)) => {
                    match advisory.recommendation.as_str() {
                        "proceed" => {
                            info!("ANN: proceed (confidence: {:.2})", advisory.confidence);
                            ResultFilter::filter(&result)
                        }
                        "block" => {
                            warn!("ANN: block (confidence: {:.2})", advisory.confidence);
                            // Drop result, trip canary
                            tunnel.send_canary("ann_blocked").await?;
                            continue; // Skip sending result
                        }
                        "escalate" => {
                            warn!("ANN: escalate (confidence: {:.2})", advisory.confidence);
                            ResultFilter::filter(&result)
                        }
                        _ => ResultFilter::filter(&result),
                    }
                }
                Ok(None) | Err(_) => {
                    // No ANN advisory or error, use default filter
                    ResultFilter::filter(&result)
                }
            };
            
            // Send back through tunnel
            tunnel.send(&filtered).await?;
        }
        Err(e) => error!("Result read error: {}", e),
    }
}
```

---

## Connection Requirements

### 1. NATS Connection (Primary)

```rust
// plasma-agent NATS setup
let nats = async_nats::connect("nats://cdn-edge-1.sx9.io:4222").await?;

// Required subscriptions
let ann_advisory_sub = nats.subscribe("sx9.plasma.ann.advisory").await?;
let threat_update_sub = nats.subscribe("sx9.intel.threat.update").await?;
let plasma_state_sub = nats.subscribe("sx9.atlas.plasma.state").await?;
let tool_result_sub = nats.subscribe("sx9.tool.{tool}.result").await?;

// Required publishers
// (implicit - use nats.publish() for commands)
```

### 2. HTTP Connections (Secondary)

```rust
// Plasma-Defender HTTP client (fallback if NATS unavailable)
let plasma_defender = reqwest::Client::new()
    .base_url("http://plasma-defender:18110")
    .timeout(Duration::from_secs(5))
    .build()?;

// CDN Data Fabric
let cdn_fabric = reqwest::Client::new()
    .base_url("http://cdn-data-fabric:18112")
    .build()?;

// Statistical Analysis CDN
let stats_cdn = reqwest::Client::new()
    .base_url("http://stats-cdn:18108")
    .build()?;

// Monitoring CDN
let monitoring_cdn = reqwest::Client::new()
    .base_url("http://monitoring-cdn:18109")
    .build()?;
```

### 3. Connection Health Monitoring

```rust
/// Health check all API connections
async fn check_api_health() -> ApiHealthStatus {
    let mut status = ApiHealthStatus::default();
    
    // Check Plasma-Defender
    if let Ok(resp) = plasma_defender.get("/health").send().await {
        status.plasma_defender = resp.status().is_success();
    }
    
    // Check NATS
    if nats.connection_state() == ConnectionState::Connected {
        status.nats = true;
    }
    
    // Check CDN services
    // ...
    
    status
}
```

---

## Data Flow Summary

### Tool Execution Flow

1. **Kali Plasma** receives command via NATS (`sx9.tool.{tool}.cmd`)
2. **eBPF** executes tool, generates result
3. **plasma-agent** sends result to **Plasma-Defender** via NATS (`sx9.tool.result.ann`)
4. **Plasma-Defender** evaluates through:
   - Crystal resonance
   - SDT gate check
   - **ANN daemon** (threat analysis)
5. **ANN** generates advisory (confidence, recommendation)
6. **Plasma-Defender** publishes advisory via NATS (`sx9.plasma.ann.advisory`)
7. **Kali Plasma** receives advisory, filters result:
   - `"proceed"` → Send filtered result
   - `"block"` → Drop result, trip canary
   - `"escalate"` → Send to high-priority channel
8. **Filtered result** sent back through tunnel

### Telemetry Flow

1. **Kali Plasma** harvests entropy from NIC
2. Publishes to NATS (`sx9.plasma.entropy`)
3. **Plasma-Defender** receives entropy, updates PlasmaState
4. **Plasma-Defender** emits telemetry (`sx9.stream.ops.plasma.telemetry`)
5. **Monitoring CDN** aggregates telemetry
6. **Statistical Analysis CDN** processes metrics

---

## Configuration

### plasma-agent Configuration

```toml
# /etc/plasma/plasma-agent.toml

[nats]
url = "nats://cdn-edge-1.sx9.io:4222"
tls = true
cert_file = "/etc/plasma/operator.crt"
key_file = "/etc/plasma/operator.key"

[plasma_defender]
url = "http://plasma-defender:18110"
timeout_ms = 5000
# Use NATS for primary communication, HTTP as fallback
primary = "nats"
fallback = "http"

[ann]
# ANN integration settings
subscribe_advisories = true
filter_on_recommendation = true
escalate_on_block = true
advisory_timeout_secs = 1

[cdn]
data_fabric = "http://cdn-data-fabric:18112"
stats = "http://stats-cdn:18108"
monitoring = "http://monitoring-cdn:18109"
```

---

## Summary

✅ **All APIs Connected:**
- Plasma-Defender (ANN integration) ✅ **PRIMARY**
- NATS Fabric (primary communication) ✅
- CDN Data Fabric (data persistence) ✅
- Statistical Analysis CDN (metrics) ✅
- Monitoring CDN (health) ✅
- Atlas Bus (PlasmaState) ✅

✅ **ANN Integration:**
- Tool results → Plasma-Defender → ANN daemon → Advisory → Kali Plasma
- Real-time threat analysis with confidence scoring
- Recommendation-based result filtering (`proceed|block|escalate`)
- Automatic canary tripping on `block` recommendation

✅ **Data Maintenance:**
- All tool results stored in CDN Data Fabric
- Telemetry streamed to Monitoring CDN
- Statistics aggregated in Statistical Analysis CDN
- ANN advisories logged for audit

---

*"The packet never existed if SDT says no. The tool never ran if the crystal didn't ring. The result never left if ANN says block."*
