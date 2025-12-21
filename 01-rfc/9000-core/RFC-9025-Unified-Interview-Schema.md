# RFC-9025: Unified Node/Crate Interview Schema

**Status:** Canonical
**Author:** Charlie Payne
**Version:** 7.3.1
**Related:** RFC-9020 (HD4), RFC-9021 (Convergence), RFC-9024 (Neurological Foundation)

---

## Abstract

This RFC defines the canonical schema for both **Node Interviews** (CTAS task narratives) and **Crate Interviews** (Rust component narratives). The unified schema ensures interoperability between threat analysis nodes and implementation crates while feeding GLAF algorithms, H1/H2 hashing, and graph convergence systems.

---

## 1. Design Principles

### 1.1 Unified First-Person Voice
All interviews speak in first person: "I am X. I provide Y. I transform Z."

### 1.2 Dual Perspective (RFC-9020)
Every interview contains both:
- **1n Perspective**: Defender view (what am I seeing?)
- **2n Perspective**: Adversary view (why do I need this?)

### 1.3 GLAF Algorithm Compatibility
Schema fields map directly to GLAF algorithms:
- `risk_dimensions` → `teth.entropy()`
- `behavioral_sequence` → `lstar.learn()`
- `relationships` → `matroid.rank()`
- `capabilities.performance` → `genome.fingerprint()`

### 1.4 Trivariate Hash Integration
Every interview generates:
- **H1 Operational Hash**: Fast, real-time signal
- **H2 Semantic Hash**: Slow, contextual pattern
- **Unicode Visual Address**: Human-readable identifier

---

## 2. Canonical Schema

```json
{
  "metadata": {
    "interview_id": "uuid-v4",
    "task_id": "uuid-000-000-001",
    "task_seq": 1,
    "hash_id": "blake3-operational-hash",
    "semantic_hash": "blake3-semantic-hash",
    "unicode_assembly_ref": "\\u{E0001}",
    "interview_version": "7.3.1",
    "created_at": "ISO-8601-timestamp",
    "hd4_phase": "Hunt|Detect|Disrupt|Disable|Dominate"
  },

  "identity": {
    "name": "Task/Crate Name",
    "type": "node|crate",
    "category": "CTAS Category or Crate Tier",
    "description": "I am X, I provide Y"
  },

  "voice": {
    "narrative": "Full first-person narrative (3-5 paragraphs)",
    "purpose": "Single sentence: what this exists to accomplish",
    "ownership": "Single sentence: what this owns in attack/defense chain"
  },

  "perspectives": {
    "defender_1n": "What defenders see, how to detect/prevent",
    "adversary_2n": "Why adversary needs this, what success looks like"
  },

  "capabilities": {
    "primary": "I transform X into Y operations",
    "operational": "I operate through Z mechanisms",
    "integration": "I integrate with A components via B protocols",
    "performance": {
      "threat_effectiveness": 0.95,
      "complexity_constraints": 0.97
    }
  },

  "limitations": {
    "vulnerabilities": "I am vulnerable to X and Y",
    "dependencies": "I require A, B, C to function effectively",
    "constraints": "I am limited by X detection risk",
    "failure_modes": "I fail when X or Y is compromised"
  },

  "needs": {
    "required": ["Capability 1", "Capability 2", "Capability 3"],
    "optional": ["Nice-to-have 1", "Nice-to-have 2"]
  },

  "counters": {
    "detection": ["Detection method 1", "Detection method 2"],
    "prevention": ["Prevention method 1", "Prevention method 2"]
  },

  "outcomes": {
    "success": ["Success indicator 1", "Success indicator 2"],
    "failure": ["Failure indicator 1", "Failure indicator 2"]
  },

  "tactical_profile": {
    "mitre_tactics": ["TA0043", "TA0001"],
    "mitre_techniques": ["T1595.001", "T1595.002", "T1190"],
    "d3fend_countermeasures": ["D3-NTA", "D3-SYSM"],
    "ttps": [
      "Tactic: X - description",
      "Technique: T1055 - operations",
      "Procedure: Execute X with Y coordination"
    ],
    "attack_vectors": ["vector: network exploitation - path description"]
  },

  "skills": {
    "required": [
      {
        "skill_id": "skill-recon-001",
        "skill_name": "Network Reconnaissance",
        "skill_category": "reconnaissance",
        "proficiency_level": "intermediate",
        "unicode_trigger": "\\u{E100}",
        "description": "Ability to enumerate network topology and services"
      },
      {
        "skill_id": "skill-exploit-001",
        "skill_name": "Vulnerability Exploitation",
        "skill_category": "exploitation",
        "proficiency_level": "advanced",
        "unicode_trigger": "\\u{E200}",
        "description": "Ability to leverage vulnerabilities for access"
      }
    ],
    "optional": [
      {
        "skill_id": "skill-opsec-001",
        "skill_name": "Operational Security",
        "skill_category": "evasion",
        "proficiency_level": "intermediate",
        "unicode_trigger": "\\u{E300}",
        "description": "Ability to avoid detection during operations"
      }
    ],
    "skill_chain": ["skill-recon-001", "skill-exploit-001", "skill-opsec-001"],
    "skill_dependencies": {
      "skill-exploit-001": ["skill-recon-001"]
    }
  },

  "toolchain": {
    "kali": [
      {
        "tool": "nmap",
        "use": "Port scanning",
        "flags": "-sS -sV",
        "skill_ref": "skill-recon-001",
        "unicode_trigger": "\\u{E100}",
        "microkernel": "ctas7/port-scanner:latest",
        "wasm_module": "port_scanner.wasm"
      },
      {
        "tool": "metasploit",
        "use": "Exploitation",
        "module": "exploit/multi/handler",
        "skill_ref": "skill-exploit-001",
        "unicode_trigger": "\\u{E200}",
        "microkernel": "ctas7/msf-handler:latest",
        "wasm_module": "msf_handler.wasm"
      }
    ],
    "osint": [
      {
        "tool": "Shodan",
        "use": "Passive recon",
        "api": true,
        "skill_ref": "skill-recon-001",
        "unicode_trigger": "\\u{E101}"
      },
      {
        "tool": "Maltego",
        "use": "Link analysis",
        "transforms": ["dns", "whois"],
        "skill_ref": "skill-recon-001",
        "unicode_trigger": "\\u{E102}"
      }
    ],
    "commercial": [
      {"tool": "Wazuh", "rule_category": "network_scan"},
      {"tool": "Splunk", "detection_rule": "correlation_search"}
    ],
    "refs": [
      "primary_tool: X - tactical integration",
      "support_tool: Y - operational support"
    ],
    "chain_definition": {
      "chain_id": "chain-initial-access-001",
      "chain_name": "Initial Access Kill Chain",
      "steps": [
        {"order": 1, "tool": "nmap", "skill_ref": "skill-recon-001"},
        {"order": 2, "tool": "metasploit", "skill_ref": "skill-exploit-001"}
      ],
      "hd4_phase": "Hunt",
      "hermetic_execution": true
    }
  },

  "intelligence": {
    "eei": {
      "detection": {
        "priority": "high|medium|low",
        "questions": ["What IP ranges?", "What attack pattern?"],
        "collection_methods": ["Network flow logs", "IDS alerts"]
      },
      "execution": {
        "priority": "high|medium|low",
        "questions": ["What is target?", "What tools needed?"],
        "collection_methods": ["Active scanning", "OSINT"]
      }
    },
    "indicators": {
      "network": ["Specific network indicator 1"],
      "behavioral": ["Behavioral pattern 1"],
      "temporal": ["Timing pattern 1"],
      "observables": ["Observable: X patterns - behavioral detection"],
      "patterns": ["Pattern: X signature with 0.9 confidence"]
    },
    "historical_reference": "I mirror APT29 SolarWinds operations"
  },

  "relationships": {
    "prerequisites": ["uuid-000-000-000"],
    "enables": ["uuid-000-000-002", "uuid-000-000-003"],
    "combined_with": ["uuid-000-000-004"],
    "related_tasks": ["uuid-000-000-005"],
    "dependencies": "I require upstream X components for tactical functions",
    "provides_to": "I provide Y capabilities to downstream operations",
    "coordinates_with": "I coordinate with peer Z components via tactical protocols",
    "escalates_to": "I escalate to authority components when threshold exceeded"
  },

  "operational_integration": {
    "legion_ecs_entity": "Entity-164-000-7145",
    "ground_station_mapping": "Association with ground station cluster 0",
    "memory_mesh_coordination": "Memory Mesh v2.0 RC1 integration",
    "voice_activation": "Voice command: 'Interview X' triggers tactical analysis"
  },

  "component": {
    "unicode_address": "\\u{E000}",
    "state": "compiled|pending|failed",
    "cascade_dependencies": ["\\u{E001}", "\\u{E002}"],
    "execution_order": 1
  },

  "escalation": {
    "escalation_unicode": "\\u{F000}",
    "escalation_level": "tactical|operational|strategic",
    "escalation_trigger": "detection_threshold",
    "escalation_target": "\\u{F001}"
  },

  "classification": {
    "task_label": "mandatory|desirable|optional",
    "task_label_rationale": "Why this classification",
    "is_key_indicator": true,
    "is_interdiction_point": false,
    "ctas_taxonomy": {
      "node_id": "164-000",
      "threat_category": "SCH-IDE",
      "adversary_model": "Threat actor behavioral model"
    }
  },

  "glaf_inputs": {
    "target_sectors": ["Finance", "Energy", "Healthcare", "Government"],
    "actor_types": ["State-Sponsored", "Criminal", "Hacktivist", "Insider"],
    "risk_dimensions": {
      "likelihood": 0.7,
      "impact": 0.5,
      "detectability": 0.8,
      "reversibility": 0.3
    },
    "behavioral_sequence": [
      "Observable action 1",
      "Observable action 2",
      "Observable action 3"
    ],
    "adversary_persona": "I represent X adversaries with Y intent and Z capability level",
    "attack_lifecycle": "I operate during reconnaissance to achieve X objectives",
    "counter_operations": "I am countered by X systems and Y methods"
  },

  "time_of_value": {
    "collection_window": "hours to days",
    "actionable_window": "days to weeks",
    "decay_rate": "fast|medium|slow",
    "persistence_condition": "When intel remains valid",
    "refresh_trigger": "What triggers re-collection"
  },

  "historical": {
    "apt_examples": [
      {"apt": "APT29", "campaign": "SolarWinds", "year": 2020, "note": "Specific use"}
    ],
    "case_studies": ["Famous incident where this was used"]
  },

  "search": {
    "keywords": ["searchable term 1", "searchable term 2"],
    "synonyms": ["alternative name 1", "alternative name 2"]
  },

  "unicode_references": {
    "tools": ["\\u{E800}", "\\u{E801}"],
    "attack_techniques": ["\\u{E900}"],
    "caldera_scenarios": ["\\u{E950}"],
    "eei_requirements": ["\\u{F800}", "\\u{F801}"]
  }
}
```

---

## 2.1 Task Pointer Integration (RFC-9100 Compliance)

Every Node Interview MUST include a pointer to the canonical CTAS Task definition:

```json
{
  "task_pointer": {
    "source_file": "ctas_tasks_with_primitive_type.csv",
    "task_id": "uuid-001-006-001",
    "task_seq": 1006001,
    "primitive_type": "Object",
    "hd4_phase": "Hunt",
    "category": "Cyber Reconnaissance",
    "ptcc_primitives": ["READ", "CONNECT", "VALIDATE"],
    "unicode_triggers": ["U+E401", "U+E40C", "U+E407"],
    "skill_requirements": ["skill-recon-001", "skill-recon-003"],
    "tool_chain_ref": "chain-initial-access-001"
  }
}
```

### 2.1.1 Primitive Type → PTCC Mapping

| Semantic Type | PTCC Category | Primary Primitives |
|---------------|---------------|--------------------|
| **Concept** | Cognitive/Control | BRANCH, LOOP, CALL, RETURN |
| **Actor** | Coordination | COORDINATE, SYNCHRONIZE, SIGNAL, WAIT |
| **Object** | Data/Network | TRANSFORM, VALIDATE, CONNECT, ROUTE, FILTER |
| **Event** | State/Security | CHECKPOINT, SAVE, AUTHENTICATE, AUTHORIZE |
| **Attribute** | Security | ENCRYPT, DECRYPT, LOCK, UNLOCK |
| **Unclassified** | CRUD/Resource | CREATE, READ, UPDATE, DELETE, ALLOCATE |

### 2.1.2 Skill Requirement Schema

```json
{
  "skill_id": "skill-recon-001",
  "skill_name": "Network Reconnaissance",
  "skill_category": "reconnaissance",
  "proficiency_level": "intermediate",
  "unicode_trigger": "\\u{E100}",
  "ptcc_primitives": ["READ", "CONNECT", "RECEIVE"],
  "tools": [
    {
      "tool": "nmap",
      "flags": "-sS -sV",
      "microkernel": "ctas7/port-scanner:latest",
      "wasm_module": "port_scanner.wasm"
    }
  ],
  "description": "Ability to enumerate network topology and services"
}
```

---

## 3. Field Requirements by Interview Type

### 3.1 Node Interview (CTAS Task) - Required Fields

| Section | Fields | GLAF Algorithm |
|---------|--------|----------------|
| metadata | interview_id, task_id, task_seq, hd4_phase | - |
| identity | name, type, category, description | - |
| voice | narrative, purpose, ownership | H2 semantic |
| perspectives | defender_1n, adversary_2n | 1n/2n weave |
| tactical_profile | mitre_tactics, mitre_techniques | - |
| relationships | prerequisites, enables | matroid.rank() |
| classification | task_label, is_key_indicator, is_interdiction_point | CTAS analysis |
| glaf_inputs | risk_dimensions, behavioral_sequence | teth.entropy(), lstar.learn() |
| time_of_value | decay_rate, collection_window | cholinesterase model |

### 3.2 Crate Interview (Rust Component) - Required Fields

| Section | Fields | Purpose |
|---------|--------|---------|
| metadata | interview_id, hash_id, semantic_hash, unicode_assembly_ref | Trivariate identity |
| identity | name, type (crate), description | Component identity |
| capabilities | primary, operational, integration, performance | Functional analysis |
| limitations | vulnerabilities, dependencies, constraints, failure_modes | Risk analysis |
| operational_integration | legion_ecs_entity, ground_station_mapping | System integration |
| component | unicode_address, state, cascade_dependencies, execution_order | Build system |
| escalation | escalation_unicode, escalation_level, escalation_trigger | Alert routing |

### 3.3 Unified Interview (Both) - Full Schema

When a crate implements threat capabilities, use the full unified schema.

---

## 4. Hash Generation

### 4.1 H1 Operational Hash (Blake3)

Fast hash for real-time signal detection:

```rust
fn generate_h1(interview: &Interview) -> H1Hash {
    let mut hasher = blake3::Hasher::new();

    // Core identity
    hasher.update(interview.identity.name.as_bytes());
    hasher.update(interview.metadata.hd4_phase.as_bytes());

    // Tactical signature
    for technique in &interview.tactical_profile.mitre_techniques {
        hasher.update(technique.as_bytes());
    }

    // Risk dimensions
    hasher.update(&interview.glaf_inputs.risk_dimensions.likelihood.to_le_bytes());
    hasher.update(&interview.glaf_inputs.risk_dimensions.impact.to_le_bytes());

    H1Hash(hasher.finalize().to_hex().to_string())
}
```

### 4.2 H2 Semantic Hash (Blake3 + Semantic Features)

Slow hash for pattern matching and context:

```rust
fn generate_h2(interview: &Interview) -> H2Hash {
    let mut hasher = blake3::Hasher::new();

    // Full narrative (semantic content)
    hasher.update(interview.voice.narrative.as_bytes());

    // Relationships (graph structure)
    for prereq in &interview.relationships.prerequisites {
        hasher.update(prereq.as_bytes());
    }
    for enables in &interview.relationships.enables {
        hasher.update(enables.as_bytes());
    }

    // Historical context
    for apt in &interview.historical.apt_examples {
        hasher.update(apt.apt.as_bytes());
        hasher.update(apt.campaign.as_bytes());
    }

    // Keywords (search semantics)
    for keyword in &interview.search.keywords {
        hasher.update(keyword.as_bytes());
    }

    H2Hash(hasher.finalize().to_hex().to_string())
}
```

### 4.3 Unicode Visual Address

Human-readable identifier in Unicode Private Use Area:

```rust
fn generate_unicode_address(task_seq: u32, interview_type: InterviewType) -> String {
    let base = match interview_type {
        InterviewType::Node => 0xE000,   // U+E000-U+E0FF for nodes
        InterviewType::Crate => 0xE100,  // U+E100-U+E1FF for crates
    };

    format!("\\u{{{:04X}}}", base + task_seq)
}
```

---

## 5. GLAF Algorithm Mapping

### 5.1 teth.entropy() Input

```json
{
  "risk_dimensions": {
    "likelihood": 0.7,
    "impact": 0.5,
    "detectability": 0.8,
    "reversibility": 0.3
  }
}
```

Formula: `entropy = likelihood * impact * (1 - detectability) * (1 - reversibility)`

### 5.2 lstar.learn() Input

```json
{
  "behavioral_sequence": [
    "DNS query to suspicious domain",
    "HTTP POST to C2 endpoint",
    "SMB lateral movement attempt",
    "Credential dump via LSASS"
  ]
}
```

L* algorithm learns automaton from these observable traces.

### 5.3 matroid.rank() Input

```json
{
  "relationships": {
    "prerequisites": ["uuid-000-000-001"],
    "enables": ["uuid-000-000-003", "uuid-000-000-004"]
  }
}
```

Calculates independent sets in dependency graph.

### 5.4 genome.fingerprint() Input

```json
{
  "capabilities": {
    "performance": {
      "threat_effectiveness": 0.95,
      "complexity_constraints": 0.97
    }
  },
  "tactical_profile": {
    "mitre_techniques": ["T1595.001", "T1595.002"]
  }
}
```

Generates capability fingerprint for similarity matching.

---

## 6. CTAS Classification Rules

### 6.1 Task Labels

| Label | Definition | Example |
|-------|------------|---------|
| **mandatory** | Absolutely and logically necessary | Acquiring explosive precursors |
| **desirable** | Adds significant value (OPSEC, effectiveness) | Counter-surveillance training |
| **optional** | Operational preference | Choice of detonation method |

### 6.2 Key Indicator

Activity indicating **significant probability of developing plot** that merits investigation.

Criteria:
- Unusual for the individual's baseline behavior
- Correlates with known attack preparation patterns
- Timing aligns with operational windows

### 6.3 Interdiction Point

Point where **law enforcement can intervene**, often through lesser crimes:
- Fraud/identity theft
- Weapons violations
- Immigration violations
- Material support charges

---

## 7. Storage Schema

### 7.1 Supabase (PostgreSQL)

```sql
CREATE TABLE unified_interviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    interview_id UUID NOT NULL UNIQUE,
    task_id VARCHAR(50),
    task_seq INTEGER,
    interview_type VARCHAR(10) CHECK (interview_type IN ('node', 'crate', 'unified')),

    -- Hashes
    h1_operational VARCHAR(64),
    h2_semantic VARCHAR(64),
    unicode_address VARCHAR(20),

    -- Core content (JSONB for flexibility)
    metadata JSONB NOT NULL,
    identity JSONB NOT NULL,
    voice JSONB NOT NULL,
    perspectives JSONB,
    capabilities JSONB,
    limitations JSONB,
    tactical_profile JSONB,
    relationships JSONB,
    classification JSONB,
    glaf_inputs JSONB,

    -- Search optimization
    search_vector tsvector GENERATED ALWAYS AS (
        to_tsvector('english',
            COALESCE(identity->>'name', '') || ' ' ||
            COALESCE(voice->>'narrative', '') || ' ' ||
            COALESCE(identity->>'description', '')
        )
    ) STORED,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_interviews_task_id ON unified_interviews(task_id);
CREATE INDEX idx_interviews_hd4_phase ON unified_interviews((metadata->>'hd4_phase'));
CREATE INDEX idx_interviews_task_label ON unified_interviews((classification->>'task_label'));
CREATE INDEX idx_interviews_key_indicator ON unified_interviews((classification->>'is_key_indicator'));
CREATE INDEX idx_interviews_search ON unified_interviews USING GIN(search_vector);
CREATE INDEX idx_interviews_mitre ON unified_interviews USING GIN((tactical_profile->'mitre_techniques'));
```

### 7.2 Neo4j (SlotGraph)

```cypher
// Node labels by interview type
CREATE (n:UnifiedInterview:NodeInterview { ... })
CREATE (c:UnifiedInterview:CrateInterview { ... })

// Phase labels for visualization
CREATE (n:UnifiedInterview:HuntPhase { ... })
CREATE (n:UnifiedInterview:DetectPhase { ... })

// Relationships
(a)-[:ENABLES {weight: 1.0}]->(b)
(a)-[:REQUIRES]->(b)
(a)-[:COMBINED_WITH]->(b)
(a)-[:RELATED_TO]->(b)
(a)-[:IN_PHASE]->(phase:HD4Phase)
```

---

## 8. Migration Path

### 8.1 Existing Node Interviews

Transform flat fields to nested structure:
```python
def migrate_node_interview(old: dict) -> dict:
    return {
        "metadata": {
            "interview_id": str(uuid4()),
            "task_id": old["task_id"],
            "task_seq": old["task_seq"],
            "hd4_phase": old["hd4_phase"]
        },
        "identity": {
            "name": old["task_name"],
            "type": "node",
            "category": old["category"]
        },
        "voice": {
            "narrative": old["voice"],
            "purpose": old["purpose"],
            "ownership": old["ownership"]
        },
        # ... continue mapping
    }
```

### 8.2 Existing Crate Interviews

Already structured - add missing GLAF fields:
```python
def migrate_crate_interview(old: dict) -> dict:
    migrated = old.copy()

    # Add GLAF inputs if missing
    if "glaf_inputs" not in migrated:
        migrated["glaf_inputs"] = {
            "risk_dimensions": calculate_risk_from_capabilities(old),
            "behavioral_sequence": extract_behavioral_from_ttps(old)
        }

    # Add classification if missing
    if "classification" not in migrated:
        migrated["classification"] = {
            "task_label": infer_task_label(old),
            "is_key_indicator": False,
            "is_interdiction_point": False
        }

    return migrated
```

---

## 9. Validation Rules

### 9.1 Required Field Validation

```python
REQUIRED_NODE_FIELDS = [
    "metadata.task_id",
    "metadata.hd4_phase",
    "identity.name",
    "voice.narrative",
    "perspectives.defender_1n",
    "perspectives.adversary_2n",
    "tactical_profile.mitre_techniques",
    "classification.task_label",
    "glaf_inputs.risk_dimensions"
]

REQUIRED_CRATE_FIELDS = [
    "metadata.hash_id",
    "metadata.unicode_assembly_ref",
    "identity.name",
    "capabilities.primary",
    "limitations.vulnerabilities",
    "component.unicode_address",
    "component.execution_order"
]
```

### 9.2 MITRE Validation

All MITRE IDs must be valid:
- Tactics: `TA0001` - `TA0043`
- Techniques: `T1XXX` or `T1XXX.YYY`
- D3FEND: `D3-XXX`

### 9.3 Risk Dimension Validation

All values must be in range `[0.0, 1.0]`.

---

## 10. References

- RFC-9020: HD4 Framework
- RFC-9021: Graph Convergence Theory
- RFC-9024: Neurological Foundation
- CTAS Reference
- MITRE ATT&CK Framework
- MITRE D3FEND Framework

---

## Changelog

- **v7.3.1** (2025-11-27): Initial unified schema merging node + crate interviews
