# Intelligence Platform Integration Plan
## CTAS7 Intelligence Generator → SX9 Workflow System

**Date:** December 2025  
**Status:** Planning Phase  
**Components:** Python Intelligence Platform + Rust SX9 System + GNN Inference Engine

---

## Executive Summary

The CTAS7 Cybersecurity Intelligence Platform (Python-based) needs to integrate with the SX9 Workflow System (Rust-based) to provide unified threat intelligence, ATT&CK/Caldera YAML processing, and GNN-powered inference capabilities.

**Key Integration Points:**
- Python Intelligence Generator → Rust SX9 System (via REST/WebSocket)
- ATT&CK/Caldera YAML Processing → DSL Conversion → SX9 Threat Intelligence
- Neo4j Graph Database → Threat relationship analysis
- GNN Inference Engine → GCP High-GPU Processing → SX9 ANN Engine
- Kali Tools Integration → Exploit Arsenal → SX9 Operations
- Plasma Viewer → Real-time Intelligence Display

---

## 1. Current Architecture

### 1.1 Python Intelligence Platform (`ctas7-intelligence-generator`)

**Location:** `/Users/cp5337/Developer/ctas7-command-center/ctas7-intelligence-generator/`

**Components:**
- **Unified Threat Intelligence Orchestrator**: Multi-source intelligence aggregation
- **GNN OSINT Intelligence**: Graph Neural Network for OSINT analysis
- **Cybersecurity Streams Plasma**: Real-time threat intelligence streaming
- **CrowdStrike Integration**: Premium threat actor intelligence
- **VirusTotal/MISP/AlienVault**: Multi-source IOC correlation
- **Plasma Display**: WebSocket-based real-time visualization

**Key Files:**
- `unified_threat_intelligence_orchestrator.py` - Master orchestrator
- `gnn_osint_intelligence.py` - GNN inference engine
- `cybersecurity_streams_plasma.py` - Real-time streaming
- `crowdstrike_threat_intelligence.py` - Premium intelligence
- `plasma_display.html` - Visualization interface

### 1.2 Rust SX9 System (`synaptix9-workflow-system`)

**Location:** `/Users/cp5337/Developer/synaptix9-workflow-system/`

**Components:**
- **sx9-plasma-defender**: Security monitoring with OSSEC integration
- **sx9-atlas-daemon**: OODA loop cognitive engine
- **sx9-ann-engine**: Artificial Neural Network engine
- **sx9-atlas-bus**: PlasmaState with crystal/thyristor gating
- **sx9-gateway-primary**: Primary gateway with neural retrofit

**Key Integration Points:**
- PlasmaState for threat gating
- ANN daemon for cognitive analysis
- OSSEC for host-based detection
- WebSocket streaming (sx9-plasma-health)

### 1.3 Rust CTAS7 Components (`ctas-7-shipyard-staging`)

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/`

**Components:**
- **ctas7-cdn-threat-reaction**: Threat response routing and caching
- **ctas7-exploit-arsenal**: Caldera integration, ATT&CK mapping, **Kali Tools Integration**
- **ctas7-plasma-viewer**: Next.js tactical display (Port 15175)

**Key Features:**
- Caldera integration for adversary emulation
- ATT&CK technique mapping
- **Kali Tools Inventory**: Comprehensive Kali Linux tool integration
- **Kali Tool Manager**: Execution and orchestration of all Kali tools
- Threat intelligence engine
- Docker Borg Assimilator

### 1.4 Graph Database Infrastructure

**Neo4j Instances:**
- **Neo4j Main Ops** (Port 7687): Threat extraction, interviews, YAMLs, operational data
  - Database: `sx9_threat_extraction`
  - Auth: `neo4j/ctas7_graph`
  - Browser: Port 7474
- **Neo4j ATL Physical** (Port 7688): ATL Physical training data
  - Database: `atl_physical`
  - Auth: `neo4j/atl_physical_graph`
  - Browser: Port 7475

**Purpose:**
- Threat relationship graph analysis
- ATT&CK technique mapping
- Campaign attribution
- IOC correlation

### 1.5 GCP High-GPU Processing

**Location:** `ctas7-intelligence-generator/marcus_gcp_neural_bridge.py`

**Components:**
- **Marcus GCP Neural Bridge**: High-GPU processing for GNN inference
- **GCP Vertex AI**: Cloud-based neural processing
- **Google Cloud Storage**: Graph data storage

**Purpose:**
- Heavy GNN graph processing
- Large-scale OSINT analysis
- Neural network inference
- Graph embedding generation

---

## 2. Integration Architecture

### 2.1 Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│              Python Intelligence Platform                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Intelligence Sources:                                          │
│  └─ CrowdStrike, CISA KEV, VirusTotal, MISP, AlienVault         │
│                                                                 │
│  Processing:                                                     │
│  └─ GNN OSINT Analysis                                          │
│  └─ Unified Threat Orchestration                                │
│  └─ ATT&CK/Caldera YAML Processing                              │
│                                                                 │
│  Output:                                                         │
│  └─ WebSocket: ws://localhost:8765 (Plasma Display)            │
│  └─ REST API: http://localhost:8000/api/intelligence           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/WebSocket
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│              Rust SX9 Workflow System                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  sx9-plasma-defender:                                           │
│  └─ Receives intelligence from Python platform                 │
│  └─ Integrates with PlasmaState (crystal/thyristor)            │
│  └─ OSSEC alert correlation                                    │
│  └─ ANN daemon for cognitive analysis                          │
│                                                                 │
│  sx9-atlas-daemon:                                              │
│  └─ OODA loop processing                                        │
│  └─ Threat escalation decisions                                 │
│                                                                 │
│  sx9-gateway-primary:                                           │
│  └─ Unified gateway for all intelligence                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ WebSocket
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│              Plasma Viewer (Next.js)                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Port 15175: Real-time tactical display                         │
│  └─ Threat intelligence visualization                          │
│  └─ ATT&CK technique mapping                                    │
│  └─ Campaign tracking                                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Integration Points

#### 2.2.1 Python → Rust Bridge

**Option A: REST API Bridge**
```rust
// In sx9-plasma-defender or new sx9-intelligence-bridge crate
pub struct IntelligenceBridge {
    python_api_url: String,
    http_client: reqwest::Client,
}

impl IntelligenceBridge {
    pub async fn fetch_threat_intelligence(&self) -> Result<ThreatIntelligence> {
        // Call Python platform REST API
        let response = self.http_client
            .get(format!("{}/api/intelligence/latest", self.python_api_url))
            .send()
            .await?;
        
        response.json().await
    }
    
    pub async fn stream_intelligence(&self) -> Result<WebSocketStream> {
        // Connect to Python WebSocket
        let ws = connect_async(format!("ws://localhost:8765")).await?;
        Ok(ws)
    }
}
```

**Option B: NATS Message Bus**
```rust
// Use NATS for async message passing
// Python publishes to: sx9.intelligence.threats
// Rust subscribes to: sx9.intelligence.threats
```

#### 2.2.2 ATT&CK/Caldera YAML Processing & DSL Conversion

**YAML to DSL Conversion:**
- **Nuclei YAML** → DSL conversion (already completed)
- **Other YAML sources** → DSL conversion
- **Raw YAML files** still available for reference

**Python Side:**
```python
# In ctas7-intelligence-generator
class ATTACKYAMLProcessor:
    def load_attack_techniques(self, yaml_path: str) -> List[ATTACKTechnique]:
        """Load MITRE ATT&CK techniques from YAML"""
        with open(yaml_path, 'r') as f:
            data = yaml.safe_load(f)
        return self.parse_attack_techniques(data)
    
    def load_caldera_adversaries(self, yaml_path: str) -> List[CalderaAdversary]:
        """Load Caldera adversary profiles from YAML"""
        with open(yaml_path, 'r') as f:
            data = yaml.safe_load(f)
        return self.parse_caldera_adversaries(data)
    
    def convert_to_dsl(self, yaml_data: Dict) -> str:
        """Convert YAML threat intelligence to DSL format"""
        # DSL conversion logic
        return dsl_output
```

**Rust Side:**
```rust
// In sx9-plasma-defender or new sx9-attack-processor crate
use serde_yaml;

pub struct ATTACKProcessor {
    techniques: HashMap<String, ATTACKTechnique>,
    tactics: HashMap<String, ATTACKTactic>,
    dsl_engine: Option<DSLEngine>,  // DSL processing
}

impl ATTACKProcessor {
    pub fn load_from_yaml(&mut self, yaml_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(yaml_path)?;
        let techniques: Vec<ATTACKTechnique> = serde_yaml::from_str(&content)?;
        
        for technique in techniques {
            self.techniques.insert(technique.id.clone(), technique);
        }
        
        Ok(())
    }
    
    pub fn load_from_dsl(&mut self, dsl_path: &Path) -> Result<()> {
        // Load DSL-converted threat intelligence
        let dsl_content = std::fs::read_to_string(dsl_path)?;
        // Parse DSL and convert to ATT&CK techniques
        Ok(())
    }
    
    pub fn map_threat_to_attack(&self, threat: &Threat) -> Vec<String> {
        // Map threat indicators to ATT&CK techniques
        // Use GNN inference if available
        vec![]
    }
}
```

#### 2.2.3 Neo4j Graph Integration

**Neo4j Connection:**
```rust
// In sx9-plasma-defender or new sx9-neo4j-bridge crate
use neo4j::GraphDatabase;

pub struct Neo4jBridge {
    main_ops: GraphDatabase,  // Port 7687
    atl_physical: GraphDatabase,  // Port 7688 (optional)
}

impl Neo4jBridge {
    pub async fn query_threat_relationships(&self, threat_id: &str) -> Result<Vec<ThreatRelationship>> {
        let query = format!(
            "MATCH (t:Threat {{id: $threat_id}})-[r]->(related)
             RETURN r, related",
            threat_id = threat_id
        );
        
        // Execute Cypher query
        // Return threat relationships
    }
    
    pub async fn map_to_attack_techniques(&self, threat_id: &str) -> Result<Vec<String>> {
        let query = format!(
            "MATCH (t:Threat {{id: $threat_id}})-[:USES]->(tech:Technique)
             RETURN tech.id",
            threat_id = threat_id
        );
        
        // Return ATT&CK technique IDs
    }
}
```

#### 2.2.4 Kali Tools Integration

**Kali Tool Execution:**
```rust
// In ctas7-exploit-arsenal (already implemented)
use crate::kali_integration::KaliToolManager;

pub struct KaliIntegration {
    tool_manager: KaliToolManager,
    inventory: KaliToolsInventory,
}

impl KaliIntegration {
    pub async fn execute_kali_tool(&self, tool_name: &str, target: &str) -> Result<KaliOperation> {
        // Execute Kali tool via KaliToolManager
        // Returns operation result
    }
    
    pub fn get_tools_for_attack_technique(&self, technique_id: &str) -> Vec<KaliTool> {
        // Map ATT&CK technique to recommended Kali tools
        // From kali_tools_inventory.rs
    }
}
```

#### 2.2.5 GNN Inference Integration with GCP High-GPU

**Python GNN Engine with GCP:**
```python
# In gnn_osint_intelligence.py and marcus_gcp_neural_bridge.py
class GNNOSINTModel:
    def infer_threat_relationships(self, threat_graph: OSINTGraph) -> Dict[str, Any]:
        """Use GNN to infer threat relationships"""
        # Check if high-GPU processing is needed
        if self.should_use_high_gpu(threat_graph):
            # Use Marcus GCP Neural Bridge
            return await self.high_gpu_inference(threat_graph)
        else:
            # Local processing
            embeddings = self.model(threat_graph)
        
        # Return inference results
        return {
            "threat_score": embeddings.threat_score,
            "related_techniques": embeddings.techniques,
            "campaign_attribution": embeddings.attribution,
        }
    
    async def high_gpu_inference(self, graph: OSINTGraph) -> Dict[str, Any]:
        """Submit to GCP high-GPU system"""
        bridge = MarcusGCPNeuralBridge()
        job = await bridge.submit_high_gpu_job(
            graph_data=graph.to_dict(),
            model_config={"gpu_tier": "heavy"},
            job_type="gnn_osint_analysis"
        )
        
        # Wait for completion
        results = await bridge.get_job_results(job.job_id)
        return results
```

**Rust Integration:**
```rust
// In sx9-ann-engine or new sx9-gnn-bridge
pub struct GNNBridge {
    python_api_url: String,
    gcp_enabled: bool,
    http_client: reqwest::Client,
}

impl GNNBridge {
    pub async fn infer_threat(&self, threat_data: &ThreatData) -> Result<GNNInference> {
        // Check if high-GPU processing is needed
        let use_gcp = self.gcp_enabled && self.requires_high_gpu(threat_data);
        
        let endpoint = if use_gcp {
            format!("{}/api/gnn/infer-high-gpu", self.python_api_url)
        } else {
            format!("{}/api/gnn/infer", self.python_api_url)
        };
        
        // Send threat data to Python GNN engine
        let response = self.http_client
            .post(&endpoint)
            .json(threat_data)
            .send()
            .await?;
        
        response.json().await
    }
    
    fn requires_high_gpu(&self, threat_data: &ThreatData) -> bool {
        // Determine if high-GPU processing is needed
        // Based on graph size, complexity, etc.
        threat_data.graph_size > 10000 || threat_data.complexity > 0.8
    }
}
```

---

## 3. Implementation Plan

### 3.1 Phase 1: REST API Bridge (Week 1)

**Tasks:**
1. **Create Python REST API Wrapper**
   - Add FastAPI/Flask REST API to `ctas7-intelligence-generator`
   - Endpoints:
     - `GET /api/intelligence/latest` - Latest threat intelligence
     - `GET /api/intelligence/threats` - All active threats
     - `POST /api/intelligence/correlate` - IOC correlation
     - `GET /api/attack/techniques` - ATT&CK techniques
     - `POST /api/gnn/infer` - GNN inference endpoint

2. **Create Rust Intelligence Bridge Crate**
   - New crate: `sx9-intelligence-bridge`
   - HTTP client for Python API
   - WebSocket client for real-time streaming
   - Data structures for threat intelligence

3. **Integrate with sx9-plasma-defender**
   - Add intelligence bridge to plasma-defender
   - Fetch intelligence on startup
   - Stream intelligence updates via WebSocket

**Files to Create:**
- `synaptix9-workflow-system/crates/sx9-intelligence-bridge/Cargo.toml`
- `synaptix9-workflow-system/crates/sx9-intelligence-bridge/src/lib.rs`
- `synaptix9-workflow-system/crates/sx9-intelligence-bridge/src/python_bridge.rs`
- `ctas7-command-center/ctas7-intelligence-generator/api_server.py`

### 3.2 Phase 2: ATT&CK/Caldera YAML Processing (Week 2)

**Tasks:**
1. **YAML Discovery and Cataloging**
   - Scan `ctas7-intelligence-generator` for all YAML files
   - Catalog ATT&CK techniques, tactics, procedures
   - Catalog Caldera adversaries, abilities, operations

2. **Create ATT&CK Processor**
   - Rust crate: `sx9-attack-processor`
   - YAML parsing for ATT&CK techniques
   - Technique-to-threat mapping
   - TTP extraction and correlation

3. **Integrate with Threat Intelligence**
   - Map intelligence indicators to ATT&CK techniques
   - Provide technique recommendations
   - Track technique usage in campaigns

**Files to Create:**
- `synaptix9-workflow-system/crates/sx9-attack-processor/Cargo.toml`
- `synaptix9-workflow-system/crates/sx9-attack-processor/src/lib.rs`
- `synaptix9-workflow-system/crates/sx9-attack-processor/src/technique.rs`
- `synaptix9-workflow-system/crates/sx9-attack-processor/src/tactic.rs`

### 3.3 Phase 3: GNN Inference Integration (Week 3)

**Tasks:**
1. **GNN API Endpoint**
   - Expose GNN inference via REST API
   - Accept threat graphs as input
   - Return inference results (threat scores, relationships)

2. **Rust GNN Bridge**
   - Create `sx9-gnn-bridge` crate
   - HTTP client for GNN API
   - Integration with ANN engine
   - Cache inference results

3. **ANN Engine Integration**
   - Connect GNN inference to sx9-ann-engine
   - Use GNN results for threat scoring
   - Combine with local ANN processing

**Files to Create:**
- `synaptix9-workflow-system/crates/sx9-gnn-bridge/Cargo.toml`
- `synaptix9-workflow-system/crates/sx9-gnn-bridge/src/lib.rs`
- `synaptix9-workflow-system/crates/sx9-gnn-bridge/src/inference.rs`

### 3.4 Phase 4: Plasma Viewer Integration (Week 4)

**Tasks:**
1. **Connect Plasma Viewer to Intelligence Streams**
   - Update `ctas7-plasma-viewer` to consume SX9 intelligence
   - Connect to sx9-plasma-health WebSocket
   - Display ATT&CK technique mappings

2. **Real-time Intelligence Display**
   - Threat actor intelligence from CrowdStrike
   - ATT&CK technique visualization
   - Campaign tracking and attribution
   - GNN inference results

**Files to Update:**
- `ctas-7-shipyard-staging/ctas7-plasma-viewer/app/intelligence/page.tsx`
- `ctas-7-shipyard-staging/ctas7-plasma-viewer/components/ThreatMap.tsx`
- `ctas-7-shipyard-staging/ctas7-plasma-viewer/components/ATTACKTechniques.tsx`

---

## 4. Data Structures

### 4.1 Threat Intelligence (Rust)

```rust
// In sx9-intelligence-bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    pub threat_id: String,
    pub title: String,
    pub description: String,
    pub threat_level: ThreatLevel,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub indicators: Vec<String>,
    pub attack_techniques: Vec<String>,  // ATT&CK technique IDs
    pub threat_actor: Option<String>,
    pub campaign: Option<String>,
    pub gnn_inference: Option<GNNInference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNInference {
    pub threat_score: f32,
    pub related_techniques: Vec<String>,
    pub campaign_attribution: Option<String>,
    pub confidence: f32,
}
```

### 4.2 ATT&CK Technique (Rust)

```rust
// In sx9-attack-processor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATTACKTechnique {
    pub technique_id: String,  // e.g., "T1055"
    pub name: String,
    pub description: String,
    pub tactic: String,  // e.g., "Defense Evasion"
    pub platforms: Vec<String>,
    pub detection_rules: Vec<String>,
    pub mitigation: Vec<String>,
}
```

---

## 5. API Endpoints

### 5.1 Python Intelligence Platform API

**Base URL:** `http://localhost:8000/api`

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/intelligence/latest` | GET | Latest threat intelligence |
| `/intelligence/threats` | GET | All active threats |
| `/intelligence/correlate` | POST | IOC correlation across sources |
| `/attack/techniques` | GET | ATT&CK techniques |
| `/attack/technique/{id}` | GET | Specific ATT&CK technique |
| `/caldera/adversaries` | GET | Caldera adversary profiles |
| `/gnn/infer` | POST | GNN inference for threat graph |
| `/gnn/embeddings` | POST | Generate graph embeddings |

### 5.2 WebSocket Endpoints

| Endpoint | Description |
|----------|-------------|
| `ws://localhost:8765` | Real-time threat intelligence stream |
| `ws://localhost:8766` | CrowdStrike premium intelligence |
| `ws://localhost:8767` | Cross-source correlation updates |

---

## 6. Configuration

### 6.1 Python Platform Config

```python
# config/intelligence_config.json
{
    "python_api": {
        "host": "0.0.0.0",
        "port": 8000,
        "websocket_port": 8765
    },
    "intelligence_sources": {
        "crowdstrike": {"enabled": true, "priority": 10},
        "cisa_kev": {"enabled": true, "priority": 10},
        "virustotal": {"enabled": true, "priority": 9},
        "misp": {"enabled": true, "priority": 8}
    },
    "gnn": {
        "enabled": true,
        "mode": "HYBRID",
        "high_gpu_url": "https://marcus-gcp.example.com/api/gnn"
    },
    "attack_yaml_path": "./attack_data",
    "caldera_yaml_path": "./caldera_data"
}
```

### 6.2 Rust SX9 Config

```toml
# In sx9-plasma-defender or sx9-gateway-primary
[intelligence]
python_api_url = "http://localhost:8000/api"
websocket_url = "ws://localhost:8765"
enable_gnn = true
enable_attack_mapping = true
attack_yaml_path = "/path/to/attack/yaml"
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

- **Python:** Test YAML parsing, GNN inference, API endpoints
- **Rust:** Test intelligence bridge, ATT&CK processor, data structures

### 7.2 Integration Tests

- **End-to-end:** Python → Rust → Plasma Viewer
- **WebSocket:** Real-time streaming validation
- **GNN Inference:** Threat graph processing

### 7.3 Performance Tests

- **Latency:** Intelligence fetch time
- **Throughput:** Concurrent threat processing
- **GNN:** Inference time for large graphs

---

## 8. Deployment

### 8.1 Python Platform

```bash
# Start Python intelligence platform
cd /Users/cp5337/Developer/ctas7-command-center/ctas7-intelligence-generator
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python api_server.py  # Start REST API + WebSocket
```

### 8.2 Rust SX9 System

```bash
# Build and run SX9 system with intelligence integration
cd /Users/cp5337/Developer/synaptix9-workflow-system
cargo build --release
./target/release/sx9-gateway-primary  # Start gateway with intelligence bridge
```

### 8.3 Plasma Viewer

```bash
# Start Next.js plasma viewer
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-plasma-viewer
npm install
npm run dev  # Runs on port 15175
```

---

## 9. Next Steps

1. **Immediate:**
   - Create `sx9-intelligence-bridge` crate
   - Add REST API to Python platform
   - Test basic HTTP communication

2. **Short-term:**
   - Implement ATT&CK YAML processing
   - Integrate GNN inference
   - Connect to Plasma Viewer

3. **Long-term:**
   - Optimize GNN inference latency
   - Add caching layer for intelligence
   - Implement intelligence persistence (SLEDIS/GLAF)

---

## 10. References

- **CTAS7 Intelligence Platform:** `ctas7-command-center/ctas7-intelligence-generator/CTAS7_Cybersecurity_Intelligence_Platform.md`
- **Plasma Defender:** `synaptix9-workflow-system/docs/PLASMA-TECHNICAL-ASSESSMENT.md`
- **Caldera Integration:** `ctas-7-shipyard-staging/ctas7-exploit-arsenal/src/caldera_integration.rs`
- **GNN OSINT:** `ctas7-command-center/ctas7-intelligence-generator/gnn_osint_intelligence.py`

---

**Status:** Ready for implementation  
**Priority:** High  
**Estimated Completion:** 4 weeks

