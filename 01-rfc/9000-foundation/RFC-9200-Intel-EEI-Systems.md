# RFC-9200: Intel & EEI Systems

**Status:** RECOVERED  
**Version:** 7.3.1  
**Date:** 2025-12-24  

---

## 1. Overview

Essential Elements of Information (EEI) framework for systematic intelligence gap identification and collection.

---

## 2. EEI (Essential Elements of Information)

### 2.1 Definition

Intelligence gaps each node needs filled to execute properly. Extracted systematically from task flow analysis.

### 2.2 Extraction Process

1. Examine parent nodes first
2. Identify child node requirements
3. Map to collection capabilities
4. Assign to appropriate INT discipline

---

## 3. Ground Station Infrastructure

### 3.1 LaserLight FSO Network

- **257 stations** worldwide (updated from 247)
- Free Space Optical (FSO) communication
- WASM microkernel deployment at each station

### 3.2 Ground Station Roles

| Role | Description |
|------|-------------|
| OSINT Collection Node | WASM microkernel deployment |
| FSO Communication Hub | Satellite uplink/downlink |
| Threat Intelligence Sensor | Network traffic analysis |
| Legion ECS Entity | Part of geospatial world via SlotGraph |

### 3.3 WASM Deployment

```bash
# Compile to WASM
cargo build --target wasm32-wasi --release

# Deploy to ground station
scp target/wasm32-wasi/release/ctas7-intel.wasm \
    station@gs-001:/opt/ctas7/wasm/

# Run in WASM runtime
wasmtime run --dir=/data ctas7-intel.wasm
```

---

## 4. Intelligence Disciplines (9 INTs)

| Index | INT | Description | Collection Method |
|-------|-----|-------------|-------------------|
| 0 | CYBINT | Cyber Intelligence | Network sensors, honeypots |
| 1 | SIGINT | Signals Intelligence | RF monitoring, ELINT |
| 2 | HUMINT | Human Intelligence | Interviews, contacts |
| 3 | IMINT | Imagery Intelligence | Satellite, drone, GEE |
| 4 | MASINT | Measurement/Signature | Sensors, forensics |
| 5 | OSINT | Open Source | Web scraping, social media |
| 6 | GEOINT | Geospatial Intelligence | Mapping, terrain analysis |
| 7 | FININT | Financial Intelligence | Transaction analysis |
| 8 | TECHINT | Technical Intelligence | Reverse engineering |

---

## 5. External Data Sources

### 5.1 MISP Integration

```rust
pub struct MispClient {
    pub url: String,
    pub api_key: String,
}

impl MispClient {
    pub async fn stream_events(&self) -> impl Stream<Item = MispEvent> {
        // Real-time event streaming from MISP
    }
    
    pub async fn search_indicators(&self, query: &str) -> Vec<Indicator> {
        // Search MISP for IOCs
    }
}
```

### 5.2 ExploitDB Streaming

```rust
pub struct ExploitDbClient {
    pub feed_url: String,
}

impl ExploitDbClient {
    pub async fn stream_cves(&self) -> impl Stream<Item = Cve> {
        // Stream new CVE entries
    }
}
```

### 5.3 Google Earth Engine (GEE)

```rust
pub struct GeeClient {
    pub project_id: String,
    pub credentials: GeeCredentials,
}

impl GeeClient {
    pub async fn get_imagery(&self, bounds: &GeoBounds, date_range: &DateRange) 
        -> GeeImageCollection {
        // Retrieve satellite imagery for area
    }
}
```

---

## 6. Media Sites & Scraping

### 6.1 Content Retention System

| Source | Type | Retention |
|--------|------|-----------|
| Library of Congress API | Historical documents | Permanent |
| Internet Archive (Wayback) | Web snapshots | Reference |
| Social media | Posts, profiles | 90 days |
| News feeds | Articles | 30 days |

### 6.2 Steganographic Analysis

```rust
pub struct StegoAnalyzer {
    pub detectors: Vec<Box<dyn StegoDetector>>,
}

impl StegoAnalyzer {
    /// Check for hidden data in emoji/Unicode
    pub fn analyze_unicode(&self, text: &str) -> Option<StegoPayload> {
        // Detect Unicode steganography
        // Variation selectors, zero-width chars, etc.
    }
    
    /// Check for blockchain evidence
    pub fn analyze_blockchain(&self, tx: &Transaction) -> Option<Evidence> {
        // Extract embedded data from transactions
    }
}
```

---

## 7. SurrealDB Intel Schema

```surql
-- Ground station with WASM deployment
DEFINE TABLE ground_station SCHEMAFULL;
DEFINE FIELD station_id ON ground_station TYPE string;
DEFINE FIELD location ON ground_station TYPE geometry(point);
DEFINE FIELD wasm_microkernel_deployed ON ground_station TYPE bool;
DEFINE FIELD collection_capabilities ON ground_station TYPE array;
DEFINE FIELD active_ints ON ground_station TYPE array;
DEFINE FIELD last_heartbeat ON ground_station TYPE datetime;

-- Intelligence product
DEFINE TABLE intel_product SCHEMAFULL;
DEFINE FIELD trivariate ON intel_product TYPE string;
DEFINE FIELD source_int ON intel_product TYPE string;
DEFINE FIELD source_station ON intel_product TYPE record(ground_station);
DEFINE FIELD confidence ON intel_product TYPE float;
DEFINE FIELD classification ON intel_product TYPE string;
DEFINE FIELD content ON intel_product TYPE object;
DEFINE FIELD eei_satisfies ON intel_product TYPE array;

-- EEI requirement
DEFINE TABLE eei_requirement SCHEMAFULL;
DEFINE FIELD requirement_id ON eei_requirement TYPE string;
DEFINE FIELD description ON eei_requirement TYPE string;
DEFINE FIELD priority ON eei_requirement TYPE int;
DEFINE FIELD assigned_ints ON eei_requirement TYPE array;
DEFINE FIELD status ON eei_requirement TYPE string;
DEFINE FIELD satisfied_by ON eei_requirement TYPE array;
```

---

## 8. Node Interview System

### 8.1 Purpose

Systematic questioning of graph nodes to identify EEI gaps.

### 8.2 Interview Structure

```rust
pub struct NodeInterview {
    pub node_id: Uuid,
    pub questions: Vec<InterviewQuestion>,
    pub responses: Vec<InterviewResponse>,
    pub eei_gaps: Vec<EeiGap>,
}

#[derive(Debug)]
pub struct InterviewQuestion {
    pub dimension: Realm,      // Which realm
    pub lens: AnalyticalLens,  // WHAT/WHO/WHERE/etc.
    pub query: String,
    pub required_confidence: f64,
}

impl NodeInterview {
    pub fn identify_gaps(&self) -> Vec<EeiGap> {
        self.questions
            .iter()
            .zip(&self.responses)
            .filter(|(q, r)| r.confidence < q.required_confidence)
            .map(|(q, r)| EeiGap {
                dimension: q.dimension,
                lens: q.lens.clone(),
                current_confidence: r.confidence,
                required_confidence: q.required_confidence,
            })
            .collect()
    }
}
```

---

## 9. Collection Tasking

### 9.1 Task Structure

```rust
pub struct CollectionTask {
    pub task_id: Uuid,
    pub eei_requirement: Uuid,
    pub assigned_station: Option<String>,
    pub assigned_int: IntelligenceDiscipline,
    pub priority: u8,
    pub deadline: Option<DateTime<Utc>>,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
}
```

### 9.2 Station Selection

```rust
impl CollectionTask {
    pub fn assign_optimal_station(&mut self, stations: &[GroundStation]) {
        // Select station based on:
        // 1. Geographic proximity to target
        // 2. Required INT capabilities
        // 3. Current workload
        // 4. Communication link quality
        
        let optimal = stations.iter()
            .filter(|s| s.has_capability(self.assigned_int))
            .min_by_key(|s| s.current_workload);
            
        if let Some(station) = optimal {
            self.assigned_station = Some(station.station_id.clone());
            self.status = TaskStatus::Assigned;
        }
    }
}
```

---

## 10. NATS Integration

```rust
// Intel subjects
pub mod intel {
    pub const COLLECTION_TASK: &str = "sx9.intel.task.>";
    pub const COLLECTION_RESULT: &str = "sx9.intel.result.>";
    pub const EEI_GAP: &str = "sx9.intel.eei.gap.>";
    pub const STATION_HEARTBEAT: &str = "sx9.intel.station.heartbeat.>";
}
```

---

## Critical Constraints

- **257 ground stations** (updated count)
- **WASM microkernel** at each station
- **9 INT disciplines** aligned to Nonagon
- **SurrealDB** for intel graph
- **EEI extraction** before collection tasking

---

## References

- RFC-9302: Nonagon Analytic Node
- RFC-9303: Crystal Realms
- RFC-9400: NATS Architecture
- RFC-9500: Database Architecture
