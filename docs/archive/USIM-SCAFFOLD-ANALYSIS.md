# USIM Scaffold and Schema Analysis
## Evaluation of usim_rust_scaffold and usim_schema.py

**Date:** December 2025  
**Status:** Scaffold Evaluation  
**Purpose:** Determine if scaffold/schema contain useful code for production USIM implementation

---

## Executive Summary

**Findings:**
- ✅ **Python Schema (`usim_schema.py`)**: **WORTHWHILE** - Complete Pydantic schema with HD4 phase, PTH metrics, toolchain integration
- ⚠️ **Rust Scaffold (`usim_rust_scaffold`)**: **MINIMAL** - Basic structure only, needs production code from ctas-6-6-mono

**Recommendation:**
- Use Python schema as reference for Rust USIM data structures
- Port production implementation from ctas-6-6-mono
- Integrate schema concepts (HD4 phases, PTH metrics, toolchains) into production code

---

## 1. Python Schema Analysis (`usim_schema.py`)

### 1.1 Status: ✅ **WORTHWHILE**

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/01-rfc/usim_schema.py`

**Quality:** Production-ready Pydantic schema

### 1.2 Key Features

**UsimData Structure:**
```python
class UsimData(BaseModel):
    # Core identifiers
    usim_id: str
    sch_id: str = Field(..., regex=r"^SCH\d{3}-\d{3}$")  # SCH format validation
    cuid: Optional[str]
    uuid: str
    
    # Task metadata
    task_name: str
    description: str
    category: str
    phase: str  # HD4 phase: Hunt, Detect, Disable, Disrupt, Dominate
    
    # Probabilistic metrics (PTH)
    probability: float = Field(..., ge=0.0, le=1.0)      # P metric
    transition: float = Field(..., ge=0.0, le=1.0)        # T metric
    human_skill_index: float = Field(..., ge=0.0, le=1.0) # H metric
    
    # Operational semantics
    capabilities: List[str]
    limitations: List[str]
    ttp: List[str]                    # MITRE ATT&CK TTPs
    indicators: List[str]             # Threat indicators
    graph_links: List[GraphLink]      # Graph relationships
    toolchains: List[Toolchain]       # Tool integration
    
    # Optional: semantic or persona narrative
    persona_narrative: Optional[str]
```

**Critical Elements:**
1. ✅ **HD4 Phase Integration** - Explicit phase field (Hunt, Detect, Disable, Disrupt, Dominate)
2. ✅ **PTH Metrics** - Probability, Transition, Human skill index (matches CTAS task CSV)
3. ✅ **Toolchain Integration** - Toolchain structure with execution modes
4. ✅ **Graph Links** - Relationship types (REQUIRES, SUPPORTS, FOLLOWED_BY)
5. ✅ **TTP Integration** - MITRE ATT&CK technique tracking
6. ✅ **SCH Format Validation** - Regex validation for SCH ID format

**UsimHeader Structure:**
```python
class UsimHeader(BaseModel):
    usim_version: str = "6.5"
    timestamp: str
    source: str  # e.g., "CTAS > CLI > :simulate > :breach"
    hash_level: str = "synaptic"  # synaptic, semantic, symbolic
    type: str = "node_inject"
```

**Loader Functions:**
- ✅ `load_usim_json()` - JSON loading with validation
- ✅ `load_usim_yaml()` - YAML loading with header parsing
- ✅ `extract_usim_header()` - Header extraction from YAML comments

### 1.3 Integration Points

**CTAS Task Integration:**
- Schema matches CTAS task CSV structure (`ctas_tasks_with_primitive_type.csv`)
- PTH metrics align with task CSV columns
- HD4 phase matches task CSV `hd4_phase` field

**Graph Integration:**
- `graph_links` supports Slot Graph relationships
- Relationship types align with GLAF network types

**Toolchain Integration:**
- Toolchain structure supports execution mode (CLI, GUI, script, daemon)
- Aligns with `ctas7-cognitive-execution-tool` requirements

### 1.4 Value Assessment

**✅ WORTHWHILE - Use as Reference:**
- Complete data structure definition
- HD4 phase integration
- PTH metrics (matches CTAS task structure)
- Toolchain integration
- Graph relationship support
- Schema validation

**Missing (needs to be added):**
- TTL management for ephemeral intelligence
- Time-of-value decay fields
- NATS streaming integration
- LISP expression generation
- Cryptographic signatures

---

## 2. Rust Scaffold Analysis (`usim_rust_scaffold`)

### 2.1 Status: ⚠️ **MINIMAL - Needs Production Code**

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/01-rfc/usim_rust_scaffold/`

**Quality:** Basic scaffold only

### 2.2 Current Structure

**Cargo.toml:**
```toml
[package]
name = "usim_scaffold"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
murmur3 = "0.5.1"  # ✅ RFC-9001 compliant
uuid = { version = "1", features = ["v4"] }
```

**Files:**
- `src/lib.rs` - Library entry point
- `src/main.rs` - Binary entry point
- `src/usim_core.rs` - Core USIM types

### 2.3 Implementation Status

**Current Code:**

**`src/usim_core.rs`:**
```rust
#[derive(Serialize)]
pub struct UsimRecord {
    pub cuid: String,
    pub uuid: String,
    pub sch: u32,           // Murmur3 hash (u32)
    pub ttl: u64,          // ✅ TTL already included!
    pub raw: String,
}

pub fn usimify_record(input: String) -> UsimRecord {
    let cuid = Uuid::new_v4().to_string();
    let uuid = Uuid::new_v4().to_string();
    
    let mut hasher = MurmurHasher::default();
    input.hash(&mut hasher);
    let sch = hasher.finish() as u32;
    
    UsimRecord {
        cuid,
        uuid,
        sch,
        ttl: 86400,  // Default 24 hours
        raw: input,
    }
}
```

**Key Features:**
- ✅ **TTL Field** - Already includes TTL (86400 = 24 hours default)
- ✅ **Murmur3 Hashing** - RFC-9001 compliant
- ✅ **Basic Structure** - CUID, UUID, SCH, TTL, raw data
- ⚠️ **Minimal** - Only basic record structure, no reactive engine

**Issues:**
- ❌ Uses `u32` for SCH (should be 64-bit per RFC-9001)
- ❌ No Base96 encoding
- ❌ No LISP expressions
- ❌ No priority scheduling
- ❌ No OODA integration
- ❌ No cryptographic signatures

### 2.4 Value Assessment

**⚠️ MINIMAL - Has TTL but Needs Production Code:**
- ✅ TTL field already included (ephemeral intelligence support)
- ✅ Murmur3 hashing (correct algorithm)
- ✅ Basic structure exists
- ❌ Needs production implementation from ctas-6-6-mono
- ❌ Should integrate Python schema concepts (HD4, PTH, toolchains)
- ❌ SCH should be 64-bit, not u32

---

## 3. Integration Plan

### 3.1 Recommended Approach

**Step 1: Port Production Implementation**
- Copy `ctas-6-6-mono/crates/ctas-usim-headers/src/usim.rs` to scaffold
- Replace stub in `ctas7-usim-system/src/usim.rs`

**Step 2: Integrate Python Schema Concepts**
- Add HD4 phase field to `USIMMessage`
- Add PTH metrics (probability, transition, human_skill_index)
- Add toolchain structure
- Add graph links structure
- Add TTP and indicators fields

**Step 3: Add Missing Features**
- TTL management (ephemeral intelligence)
- Time-of-value decay (RFC-9026)
- NATS streaming integration
- EEI integration

### 3.2 Enhanced USIMMessage Structure

**Based on Python Schema + Production Code + Scaffold TTL:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USIMMessage {
    // Core identifiers (from production)
    pub id: String,
    pub sch_hash: String,          // Trivariate SCH (64-bit Base96)
    pub uuid: String,              // Trivariate UUID
    pub cuid: String,              // Trivariate CUID
    
    // Task metadata (from Python schema)
    pub task_name: String,
    pub description: String,
    pub category: String,
    pub phase: HD4Phase,           // Hunt, Detect, Disable, Disrupt, Dominate
    
    // Probabilistic metrics (from Python schema)
    pub probability: f64,          // P metric (0.0-1.0)
    pub transition: f64,          // T metric (0.0-1.0)
    pub human_skill_index: f64,    // H metric (0.0-1.0)
    
    // Operational semantics (from Python schema)
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub ttp: Vec<String>,          // MITRE ATT&CK TTPs
    pub indicators: Vec<String>,    // Threat indicators
    pub graph_links: Vec<GraphLink>,
    pub toolchains: Vec<Toolchain>,
    
    // LISP and signatures (from production)
    pub lisp_expression: String,
    pub priority: MessagePriority,
    pub ooda_state: String,
    pub target_persona: Option<String>,
    pub created_at: DateTime<Utc>,
    pub signature: Option<String>,
    
    // TTL management (from scaffold - already has this!)
    pub ttl: u64,                  // Time-to-live in seconds (from scaffold)
    pub expires_at: Option<DateTime<Utc>>,  // Calculated expiration
    pub ephemeral: bool,           // Ephemeral intelligence flag
    
    // Optional (from Python schema)
    pub persona_narrative: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HD4Phase {
    Hunt,
    Detect,
    Disable,
    Disrupt,
    Dominate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphLink {
    pub target_uuid: String,
    pub relationship: String,  // REQUIRES, SUPPORTS, FOLLOWED_BY
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toolchain {
    pub name: String,
    pub tool_type: String,  // network, exploit, recon
    pub version: Option<String>,
    pub execution_mode: String,  // CLI, GUI, script, daemon
}
```

---

## 4. Action Items

### 4.1 Immediate Actions

1. **Read Rust Scaffold Files:**
   - Review `src/lib.rs`, `src/usim_core.rs`, `src/main.rs`
   - Determine if any useful code exists

2. **Port Production Implementation:**
   - Copy from `ctas-6-6-mono/crates/ctas-usim-headers/src/usim.rs`
   - Integrate Python schema concepts
   - Add TTL/ephemeral intelligence management

3. **Create Enhanced USIM Structure:**
   - Combine production code + Python schema
   - Add HD4 phase, PTH metrics, toolchains
   - Add TTL management

### 4.2 Integration Tasks

1. **CTAS Task Integration:**
   - Map CTAS task CSV to USIMMessage
   - Generate USIM messages from CTAS tasks
   - Link to Slot Graph via graph_links

2. **Gateway Integration:**
   - Publish USIM messages to NATS
   - Apply time-of-value decay
   - TTL-based expiration

3. **EEI Integration:**
   - USIM fulfills EEI requirements
   - EEI affects backplane/crystal decisions
   - USIM feeds EEI decision engine

---

## 5. Summary

### 5.1 Python Schema (`usim_schema.py`)

**Status:** ✅ **WORTHWHILE**

**Value:**
- Complete data structure definition
- HD4 phase integration
- PTH metrics (matches CTAS task CSV)
- Toolchain integration
- Graph relationship support
- Schema validation

**Use As:**
- Reference for Rust USIM data structures
- Validation schema for USIM data
- Integration point with CTAS tasks

### 5.2 Rust Scaffold (`usim_rust_scaffold`)

**Status:** ⚠️ **MINIMAL - Has TTL but Needs Production Code**

**Value:**
- ✅ **TTL field included** - Already supports ephemeral intelligence (86400 default)
- ✅ Basic structure exists (`UsimRecord` with CUID, UUID, SCH, TTL, raw)
- ✅ Correct dependencies (Murmur3, serde, uuid)
- ⚠️ SCH is `u32` (should be 64-bit per RFC-9001)
- ❌ No reactive engine, LISP, signatures, priority scheduling
- ❌ Needs production implementation from ctas-6-6-mono

**Use As:**
- Starting point for porting production code
- TTL field can be preserved (ephemeral intelligence support)
- Integration target for Python schema concepts
- Foundation for enhanced USIM implementation

### 5.3 Recommended Next Steps

1. Read Rust scaffold files to assess current state
2. Port production implementation from ctas-6-6-mono
3. Integrate Python schema concepts (HD4, PTH, toolchains)
4. Add TTL/ephemeral intelligence management
5. Integrate with gateway and EEI systems

---

**Status:** Python schema is worthwhile, Rust scaffold needs production code

**Next Steps:** Read Rust scaffold files and port production implementation

