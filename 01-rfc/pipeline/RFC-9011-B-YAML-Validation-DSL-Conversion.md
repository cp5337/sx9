# RFC-9011-B — YAML Validation & DSL Conversion Pipeline

**Version:** 1.0  
**Status:** Implementation Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9011, RFC-9011-A, RFC-9100

---

## 1. Abstract

This RFC specifies the pipeline for validating heterogeneous YAML threat content (Sigma, YARA, Nuclei, Caldera, Atomic Red Team) and converting it to the canonical **SX9 DSL** format for playbook execution.

**Key Principle:** Never trust external YAML. Validate before processing.

---

## 2. Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                    YAML → DSL CONVERSION PIPELINE                                    │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                      │
│  EXTERNAL SOURCES                                                                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                       │
│  │ Sigma   │ │ Nuclei  │ │ Caldera │ │ Atomic  │ │ Custom  │                       │
│  │ Rules   │ │Templates│ │Abilities│ │Red Team │ │ YAML    │                       │
│  │ ~4000   │ │ ~8000   │ │ ~500    │ │ ~1000   │ │ ???     │                       │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘                       │
│       │           │           │           │           │                             │
│       └───────────┴───────────┼───────────┴───────────┘                             │
│                               │                                                      │
│                               ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐│
│  │  PHASE 1: SYNTAX VALIDATION                                                     ││
│  │  ──────────────────────────                                                     ││
│  │                                                                                  ││
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                      ││
│  │  │ YAML Parser  │───▶│ Schema Check │───▶│ Lint Rules   │                      ││
│  │  │ (ruamel.yaml)│    │ (jsonschema) │    │ (yamllint)   │                      ││
│  │  └──────────────┘    └──────────────┘    └──────────────┘                      ││
│  │                                                                                  ││
│  │  Output: Valid YAML + Validation Report                                         ││
│  │  Reject: Malformed, schema violations, lint failures                            ││
│  └──────────────────────────────────────────────────────────────────────────────────┘│
│                               │                                                      │
│                               ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐│
│  │  PHASE 2: SEMANTIC VALIDATION                                                   ││
│  │  ────────────────────────────                                                   ││
│  │                                                                                  ││
│  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                      ││
│  │  │ ATT&CK ID    │───▶│ Field        │───▶│ Duplicate    │                      ││
│  │  │ Verification │    │ Completeness │    │ Detection    │                      ││
│  │  └──────────────┘    └──────────────┘    └──────────────┘                      ││
│  │                                                                                  ││
│  │  Output: Semantically valid content                                             ││
│  │  Reject: Invalid ATT&CK refs, missing required fields, duplicates              ││
│  └──────────────────────────────────────────────────────────────────────────────────┘│
│                               │                                                      │
│                               ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐│
│  │  PHASE 3: DSL CONVERSION (Crosswalk)                                            ││
│  │  ───────────────────────────────────                                            ││
│  │                                                                                  ││
│  │  Source YAML ──→ Canonical JSON ──→ PTCC Mapping ──→ SX9 DSL                   ││
│  │                                                                                  ││
│  │  ┌──────────────────────────────────────────────────────────────────────────┐  ││
│  │  │  SX9 DSL Output Structure:                                                │  ││
│  │  │  {                                                                        │  ││
│  │  │    "dsl_version": "1.0",                                                  │  ││
│  │  │    "source_ref": "sigma:abc123",                                          │  ││
│  │  │    "ptcc_primitive": "0x08",  // EXECUTE                                  │  ││
│  │  │    "hd4_phase": "DISRUPT",                                                │  ││
│  │  │    "playbook_actions": [...],                                             │  ││
│  │  │    "trivariate": { "sch_t": "...", "cuid_t": "...", "uuid": "..." }       │  ││
│  │  │  }                                                                        │  ││
│  │  └──────────────────────────────────────────────────────────────────────────┘  ││
│  └──────────────────────────────────────────────────────────────────────────────────┘│
│                               │                                                      │
│                               ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐│
│  │  PHASE 4: PLAYBOOK GENERATION                                                   ││
│  │  ────────────────────────────                                                   ││
│  │                                                                                  ││
│  │  SX9 DSL ──→ Executable Playbook (Rust struct / JSON)                          ││
│  │                                                                                  ││
│  │  Ready for: ATLAS Daemon execution, HD4 phase operations                        ││
│  └──────────────────────────────────────────────────────────────────────────────────┘│
│                                                                                      │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Phase 1: Syntax Validation

### 3.1 YAML Parsing

```python
# yaml_validator.py

import ruamel.yaml
from pathlib import Path
from typing import Tuple, Optional, Dict, Any

yaml = ruamel.yaml.YAML()
yaml.preserve_quotes = True

class YamlValidator:
    """Phase 1: Syntax validation for external YAML files."""
    
    def parse_yaml(self, file_path: Path) -> Tuple[Optional[Dict], Optional[str]]:
        """
        Parse YAML file and return content or error.
        
        Returns:
            (content, None) on success
            (None, error_message) on failure
        """
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = yaml.load(f)
            return (content, None)
        except ruamel.yaml.YAMLError as e:
            return (None, f"YAML parse error: {e}")
        except UnicodeDecodeError as e:
            return (None, f"Encoding error: {e}")
```

### 3.2 Schema Validation

Each source type has a defined JSON Schema:

```python
# schemas/sigma_schema.json
SIGMA_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["title", "logsource", "detection"],
    "properties": {
        "title": {"type": "string", "minLength": 1},
        "id": {"type": "string", "pattern": "^[a-f0-9-]{36}$"},
        "status": {"enum": ["stable", "test", "experimental", "deprecated"]},
        "level": {"enum": ["informational", "low", "medium", "high", "critical"]},
        "logsource": {
            "type": "object",
            "properties": {
                "category": {"type": "string"},
                "product": {"type": "string"},
                "service": {"type": "string"}
            }
        },
        "detection": {"type": "object"},
        "tags": {
            "type": "array",
            "items": {"type": "string"}
        }
    }
}

# schemas/nuclei_schema.json
NUCLEI_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["id", "info", "requests"],
    "properties": {
        "id": {"type": "string", "pattern": "^[a-z0-9-]+$"},
        "info": {
            "type": "object",
            "required": ["name", "severity"],
            "properties": {
                "name": {"type": "string"},
                "severity": {"enum": ["info", "low", "medium", "high", "critical"]},
                "tags": {"type": "array"}
            }
        }
    }
}

# schemas/caldera_schema.json
CALDERA_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["id", "name", "tactic", "technique"],
    "properties": {
        "id": {"type": "string"},
        "name": {"type": "string"},
        "tactic": {"type": "string"},
        "technique": {
            "type": "object",
            "properties": {
                "attack_id": {"type": "string", "pattern": "^T\\d{4}(\\.\\d{3})?$"}
            }
        },
        "platforms": {"type": "object"}
    }
}
```

### 3.3 Schema Dispatch

```python
from jsonschema import validate, ValidationError

SCHEMA_MAP = {
    "sigma": SIGMA_SCHEMA,
    "nuclei": NUCLEI_SCHEMA,
    "caldera": CALDERA_SCHEMA,
    "atomic": ATOMIC_SCHEMA,
}

def validate_schema(content: Dict, source_type: str) -> Tuple[bool, Optional[str]]:
    """Validate content against source-specific schema."""
    schema = SCHEMA_MAP.get(source_type)
    if not schema:
        return (False, f"Unknown source type: {source_type}")
    
    try:
        validate(instance=content, schema=schema)
        return (True, None)
    except ValidationError as e:
        return (False, f"Schema violation: {e.message}")
```

---

## 4. Phase 2: Semantic Validation

### 4.1 ATT&CK ID Verification

```python
# Cached ATT&CK technique IDs (loaded from MITRE STIX bundle)
VALID_TECHNIQUE_IDS: Set[str] = set()  # Populated at startup

def verify_attack_id(technique_id: str) -> bool:
    """Verify ATT&CK technique ID exists in official catalog."""
    # Normalize: T1059.001 → T1059.001
    normalized = technique_id.upper().strip()
    return normalized in VALID_TECHNIQUE_IDS

def extract_attack_refs(content: Dict, source_type: str) -> List[str]:
    """Extract ATT&CK references from content based on source type."""
    refs = []
    
    if source_type == "sigma":
        # Sigma uses tags like "attack.t1059.001"
        for tag in content.get("tags", []):
            if tag.startswith("attack.t"):
                refs.append(tag.replace("attack.", "").upper())
    
    elif source_type == "nuclei":
        # Nuclei uses classification.cve-id or tags
        tags = content.get("info", {}).get("tags", [])
        for tag in tags:
            if tag.upper().startswith("T"):
                refs.append(tag.upper())
    
    elif source_type == "caldera":
        # Caldera has explicit technique.attack_id
        attack_id = content.get("technique", {}).get("attack_id")
        if attack_id:
            refs.append(attack_id.upper())
    
    elif source_type == "atomic":
        # Atomic Red Team has attack_technique array
        for tech in content.get("attack_technique", []):
            refs.append(tech.upper())
    
    return refs
```

### 4.2 Field Completeness Check

```python
REQUIRED_FIELDS = {
    "sigma": ["title", "logsource", "detection"],
    "nuclei": ["id", "info.name", "info.severity"],
    "caldera": ["id", "name", "tactic", "technique.attack_id"],
    "atomic": ["name", "attack_technique", "test_number"],
}

def check_completeness(content: Dict, source_type: str) -> List[str]:
    """Return list of missing required fields."""
    missing = []
    required = REQUIRED_FIELDS.get(source_type, [])
    
    for field in required:
        if "." in field:
            # Nested field
            parts = field.split(".")
            val = content
            for part in parts:
                val = val.get(part, {}) if isinstance(val, dict) else None
            if not val:
                missing.append(field)
        else:
            if not content.get(field):
                missing.append(field)
    
    return missing
```

### 4.3 Duplicate Detection

```python
import hashlib

def compute_content_hash(content: Dict) -> str:
    """Compute semantic hash for duplicate detection."""
    # Normalize and hash key fields
    canonical = json.dumps(content, sort_keys=True, default=str)
    return hashlib.sha256(canonical.encode()).hexdigest()[:16]

class DuplicateDetector:
    def __init__(self):
        self.seen_hashes: Dict[str, str] = {}  # hash → source_file
    
    def check(self, content: Dict, source_file: str) -> Optional[str]:
        """Returns original file path if duplicate, None otherwise."""
        content_hash = compute_content_hash(content)
        
        if content_hash in self.seen_hashes:
            return self.seen_hashes[content_hash]
        
        self.seen_hashes[content_hash] = source_file
        return None
```

---

## 5. Phase 3: DSL Conversion

### 5.1 Canonical JSON Normalization

```python
@dataclass
class CanonicalThreatContent:
    """Normalized intermediate format."""
    source_id: str          # Original ID (sigma UUID, nuclei ID, etc.)
    source_type: str        # sigma, nuclei, caldera, atomic
    source_file: str        # Original file path
    name: str               # Human-readable name
    description: str        # Description/summary
    attack_refs: List[str]  # ATT&CK technique IDs
    severity: str           # Normalized: info, low, medium, high, critical
    platforms: List[str]    # windows, linux, macos, network, ics
    data_sources: List[str] # Required log sources
    raw_content: Dict       # Original YAML content
```

### 5.2 PTCC Mapping (Crosswalk)

```python
# From ctas7-crosswalk-lib.rs, Python port for pipeline

TACTIC_TO_PTCC = {
    "reconnaissance": (0x16, "SCAN", "HUNT"),
    "initial-access": (0x0C, "CONNECT", "DETECT"),
    "execution": (0x08, "EXECUTE", "DISRUPT"),
    "persistence": (0x18, "INSTALL", "DISRUPT"),
    "privilege-escalation": (0x12, "ELEVATE", "DISABLE"),
    "defense-evasion": (0x1C, "OBFUSCATE", "DETECT"),
    "credential-access": (0x10, "AUTHENTICATE", "DISABLE"),
    "discovery": (0x14, "ENUMERATE", "HUNT"),
    "lateral-movement": (0x0C, "CONNECT", "DISABLE"),
    "collection": (0x03, "READ", "DISRUPT"),
    "command-and-control": (0x0C, "CONNECT", "DOMINATE"),
    "exfiltration": (0x04, "SEND", "DOMINATE"),
    "impact": (0x02, "UPDATE", "DOMINATE"),
}

def map_to_ptcc(attack_ref: str, tactic: Optional[str] = None) -> Tuple[int, str, str]:
    """
    Map ATT&CK reference to PTCC primitive and HD4 phase.
    
    Returns: (ptcc_code, ptcc_name, hd4_phase)
    """
    # Direct technique mappings (high confidence)
    DIRECT_MAPPINGS = {
        "T1003": (0x10, "AUTHENTICATE", "DISABLE"),
        "T1059": (0x08, "EXECUTE", "DISRUPT"),
        "T1041": (0x04, "SEND", "DOMINATE"),
        "T1055": (0x1B, "INJECT", "DISRUPT"),
        "T1068": (0x12, "ELEVATE", "DISABLE"),
        "T1027": (0x1C, "OBFUSCATE", "DETECT"),
        "T1486": (0x1D, "ENCRYPT", "DOMINATE"),
        # ... more direct mappings
    }
    
    # Check direct mapping first
    base_technique = attack_ref.split(".")[0]  # T1059.001 → T1059
    if base_technique in DIRECT_MAPPINGS:
        return DIRECT_MAPPINGS[base_technique]
    
    # Fall back to tactic-based mapping
    if tactic and tactic in TACTIC_TO_PTCC:
        return TACTIC_TO_PTCC[tactic]
    
    # Default: UNKNOWN
    return (0x20, "UNKNOWN", "HUNT")
```

### 5.3 SX9 DSL Generation

```python
@dataclass
class Sx9DslArtifact:
    """Final SX9 DSL format for playbook generation."""
    dsl_version: str = "1.0"
    source_ref: str = ""           # e.g., "sigma:abc123"
    source_type: str = ""          # sigma, nuclei, caldera, atomic
    name: str = ""
    description: str = ""
    
    # PTCC mapping
    ptcc_primitive: int = 0x20     # Hex code
    ptcc_name: str = "UNKNOWN"     # Human-readable
    hd4_phase: str = "HUNT"        # HD4 phase affinity
    
    # Attack references
    attack_refs: List[str] = field(default_factory=list)
    
    # Execution context
    platforms: List[str] = field(default_factory=list)
    severity: str = "medium"
    confidence: float = 0.5
    
    # Playbook actions (derived from source)
    playbook_actions: List[Dict] = field(default_factory=list)
    
    # Trivariate hash (generated after imputation)
    trivariate: Optional[Dict] = None
    
    # Metadata
    validated_at: str = ""
    validation_status: str = "pending"

def convert_to_dsl(canonical: CanonicalThreatContent) -> Sx9DslArtifact:
    """Convert canonical content to SX9 DSL artifact."""
    
    # Get primary ATT&CK ref for PTCC mapping
    primary_ref = canonical.attack_refs[0] if canonical.attack_refs else ""
    tactic = infer_tactic(canonical)  # From tags or content analysis
    
    ptcc_code, ptcc_name, hd4_phase = map_to_ptcc(primary_ref, tactic)
    
    # Extract playbook actions based on source type
    playbook_actions = extract_playbook_actions(canonical)
    
    return Sx9DslArtifact(
        source_ref=f"{canonical.source_type}:{canonical.source_id}",
        source_type=canonical.source_type,
        name=canonical.name,
        description=canonical.description,
        ptcc_primitive=ptcc_code,
        ptcc_name=ptcc_name,
        hd4_phase=hd4_phase,
        attack_refs=canonical.attack_refs,
        platforms=canonical.platforms,
        severity=canonical.severity,
        playbook_actions=playbook_actions,
        validated_at=datetime.utcnow().isoformat(),
        validation_status="validated",
    )
```

---

## 6. Phase 4: Playbook Generation

### 6.1 Playbook Actions by Source Type

```python
def extract_playbook_actions(canonical: CanonicalThreatContent) -> List[Dict]:
    """Extract executable playbook actions from source content."""
    
    if canonical.source_type == "sigma":
        return extract_sigma_actions(canonical.raw_content)
    elif canonical.source_type == "nuclei":
        return extract_nuclei_actions(canonical.raw_content)
    elif canonical.source_type == "caldera":
        return extract_caldera_actions(canonical.raw_content)
    elif canonical.source_type == "atomic":
        return extract_atomic_actions(canonical.raw_content)
    
    return []

def extract_sigma_actions(content: Dict) -> List[Dict]:
    """Convert Sigma detection to playbook query actions."""
    actions = []
    
    detection = content.get("detection", {})
    logsource = content.get("logsource", {})
    
    actions.append({
        "type": "DETECT",
        "engine": "siem",
        "query_type": "sigma",
        "logsource": logsource,
        "detection_logic": detection,
        "level": content.get("level", "medium"),
    })
    
    return actions

def extract_caldera_actions(content: Dict) -> List[Dict]:
    """Convert Caldera ability to playbook execution actions."""
    actions = []
    
    for platform, executors in content.get("platforms", {}).items():
        for executor_type, executor_config in executors.items():
            actions.append({
                "type": "EXECUTE",
                "platform": platform,
                "executor": executor_type,  # sh, psh, cmd
                "command": executor_config.get("command", ""),
                "cleanup": executor_config.get("cleanup", ""),
                "payloads": executor_config.get("payloads", []),
            })
    
    return actions

def extract_atomic_actions(content: Dict) -> List[Dict]:
    """Convert Atomic Red Team test to playbook actions."""
    actions = []
    
    for executor in content.get("executor", {}).get("steps", []):
        actions.append({
            "type": "EXECUTE",
            "executor": content.get("executor", {}).get("name", "manual"),
            "command": executor.get("command", ""),
            "elevation_required": executor.get("elevation_required", False),
            "cleanup_command": content.get("executor", {}).get("cleanup_command", ""),
        })
    
    return actions
```

### 6.2 Final Playbook Structure

```python
@dataclass
class Sx9Playbook:
    """Executable playbook derived from SX9 DSL."""
    playbook_id: str              # UUID
    name: str
    description: str
    
    # HD4 context
    hd4_phase: str                # Primary phase
    ptcc_primitive: int           # Primary primitive
    
    # Source references
    source_artifacts: List[str]   # List of source_refs
    attack_techniques: List[str]  # Covered ATT&CK IDs
    
    # Execution
    actions: List[Dict]           # Ordered actions
    platforms: List[str]          # Target platforms
    
    # Requirements
    prerequisites: List[str]      # Required capabilities
    data_sources: List[str]       # Required log sources
    
    # Metadata
    severity: str
    confidence: float
    created_at: str
    trivariate_hash: Dict         # SCH-T, CUID-T, UUID
```

---

## 7. Validation Report

### 7.1 Report Structure

```python
@dataclass
class ValidationReport:
    """Summary report for batch validation run."""
    total_files: int
    passed: int
    failed_syntax: int
    failed_schema: int
    failed_semantic: int
    duplicates: int
    
    details: List[ValidationDetail]
    
    @property
    def pass_rate(self) -> float:
        return self.passed / self.total_files if self.total_files > 0 else 0.0

@dataclass
class ValidationDetail:
    file_path: str
    source_type: str
    status: str  # passed, failed_syntax, failed_schema, failed_semantic, duplicate
    errors: List[str]
    warnings: List[str]
```

### 7.2 Example Output

```json
{
  "validation_report": {
    "total_files": 4523,
    "passed": 4312,
    "failed_syntax": 45,
    "failed_schema": 89,
    "failed_semantic": 62,
    "duplicates": 15,
    "pass_rate": 0.953,
    "run_at": "2025-11-28T03:45:00Z"
  },
  "by_source": {
    "sigma": { "total": 3800, "passed": 3712 },
    "nuclei": { "total": 500, "passed": 423 },
    "caldera": { "total": 150, "passed": 142 },
    "atomic": { "total": 73, "passed": 35 }
  }
}
```

---

## 8. CLI Interface

```bash
# Validate only (no conversion)
python yaml_dsl_pipeline.py validate --source sigma --input ./sigma-rules/

# Validate and convert to DSL
python yaml_dsl_pipeline.py convert --source sigma --input ./sigma-rules/ --output ./dsl_output/

# Full pipeline (validate → convert → generate playbooks)
python yaml_dsl_pipeline.py full --input ./threat_content/ --output ./playbooks/

# Generate validation report
python yaml_dsl_pipeline.py report --input ./threat_content/ --format json > validation_report.json
```

---

## 9. Decision: Trust or Validate?

### Option A: Assume Perfect (Fast, Risky)

```python
# Skip validation, direct conversion
for file in yaml_files:
    content = yaml.safe_load(file)
    dsl = convert_to_dsl(content)  # May fail on bad input
```

**Risk:** Malformed YAML crashes pipeline, bad data pollutes graph.

### Option B: Full Validation (Slow, Safe) ✅ RECOMMENDED

```python
# Full validation pipeline
for file in yaml_files:
    result = validate_syntax(file)
    if not result.ok:
        log_error(result); continue
    
    result = validate_schema(result.content, source_type)
    if not result.ok:
        log_error(result); continue
    
    result = validate_semantic(result.content, source_type)
    if not result.ok:
        log_warning(result)  # May still convert with warnings
    
    dsl = convert_to_dsl(result.content)
    persist(dsl)
```

**Benefit:** Clean data, reliable playbooks, audit trail.

### Option C: Trust-but-Verify (Balanced)

```python
# Quick syntax check, defer semantic validation
for file in yaml_files:
    try:
        content = yaml.safe_load(file)
        dsl = convert_to_dsl(content)
        dsl.validation_status = "pending_review"
        persist(dsl)
    except Exception as e:
        quarantine(file, e)
```

**Use for:** Initial bulk load, followed by batch validation.

---

## 10. Implementation Status

| Component | File | Status |
|-----------|------|--------|
| YAML Validator | `yaml_validator.py` | 🔴 Need |
| Schema Definitions | `schemas/*.json` | 🔴 Need |
| Semantic Validator | `semantic_validator.py` | 🔴 Need |
| DSL Converter | `dsl_converter.py` | 🔴 Need |
| Playbook Generator | `playbook_generator.py` | 🔴 Need |
| CLI Pipeline | `yaml_dsl_pipeline.py` | 🔴 Need |

---

**End of RFC-9011-B**
