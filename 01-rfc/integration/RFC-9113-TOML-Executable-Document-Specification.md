# RFC-9113 — TOML Executable Document Specification

**Status:** CANONICAL  
**Version:** 1.0  
**Date:** 03 December 2025  
**Author:** CTAS Core Engineering + Synaptix9 Architectures  
**Implements:** RFC-9102 (Executable Document Framework)  
**Conforms To:** RFC-9112 (Deterministic Prompt Engineering)  
**Supersedes:** None

---

## Abstract

This RFC specifies the **TOML Executable Document** format for SX9 system orchestration. The document format provides a deterministic, machine-parseable specification for:

- System bootstrap and initialization
- Pipeline execution ordering
- Data fabric configuration (zero-license stack)
- GPU escalation triggers
- Bernoulli zone compliance
- Dual trivariate hash computation

The canonical implementation is `sx9-executable-document.toml`.

---

## Table of Contents

1. [Document Structure](#1-document-structure)
2. [Section Specifications](#2-section-specifications)
3. [Embedded Manifest Schema](#3-embedded-manifest-schema)
4. [Validation Rules](#4-validation-rules)
5. [Execution Semantics](#5-execution-semantics)
6. [Hash Computation](#6-hash-computation)
7. [Error Handling](#7-error-handling)
8. [Conformance](#8-conformance)
9. [Reference Implementation](#9-reference-implementation)

---

## 1. Document Structure

### 1.1 Top-Level Sections

A conforming TOML Executable Document MUST contain these sections:

```toml
[document]           # Document metadata
[execution]          # Execution configuration
[input]              # Input validation
[embedded.sx9_manifest]  # SX9-specific configuration
[actions]            # Enabled actions
[outputs]            # Output paths
[security]           # Security constraints
[review]             # Review requirements
```

### 1.2 Section Ordering

Sections MUST appear in the order specified above. Parsers MAY reject documents with out-of-order sections.

### 1.3 TOML Version

Documents MUST conform to TOML v1.0.0 specification.

---

## 2. Section Specifications

### 2.1 [document] Section

```toml
[document]
id = "string"                    # REQUIRED: Unique document identifier
version = "semver"               # REQUIRED: Semantic version
classification = "string"        # REQUIRED: Document classification
kind = "Executable-Document"     # REQUIRED: Must be "Executable-Document"
schema = "RFC-9102"              # REQUIRED: Must be "RFC-9102"
owner = "string"                 # REQUIRED: Owning system/team
created_at = "auto:now"          # REQUIRED: Creation timestamp or "auto:now"
hash = "auto:trivariate"         # REQUIRED: Hash value or "auto:trivariate"
validated = false                # REQUIRED: Validation status
```

**Constraints:**
- `id`: `[a-z][a-z0-9-]*`, max 64 characters
- `version`: Valid semver (e.g., "1.0.0")
- `kind`: Exactly "Executable-Document"
- `schema`: Exactly "RFC-9102"

### 2.2 [execution] Section

```toml
[execution]
mode = "deterministic"           # REQUIRED: "deterministic" | "adaptive"
strict = true                    # REQUIRED: Boolean
max_time_ms = 5000               # REQUIRED: Maximum execution time
approved = false                 # REQUIRED: Approval status
requires_human_gate = true       # REQUIRED: Human approval required

description = """                # OPTIONAL: Multi-line description
Description text here.
"""
```

**Constraints:**
- `mode`: Must be "deterministic" for RFC-9112 compliance
- `max_time_ms`: 1000–3600000 (1 second to 1 hour)

### 2.3 [input] Section

```toml
[input]
accepted_formats = ["prompt", "toml"]  # REQUIRED: Accepted input formats
requires_rfc_context = true            # REQUIRED: RFC context required
rfc_window = [                         # REQUIRED: RFC dependency window
    "RFC-9100",
    "RFC-9101",
    # ... additional RFCs
]

[input.validation]
checks = ["schema", "bnf-grammar", "trivariate-hash"]  # REQUIRED
on_failure = "abort"                                    # REQUIRED: "abort" | "warn"
```

**Constraints:**
- `rfc_window`: Each entry must match `RFC-\d{4}[A-Z]?`
- `checks`: Must include at least "schema"

### 2.4 [embedded.sx9_manifest] Section

See Section 3 for complete schema.

### 2.5 [actions] Section

```toml
[actions]
resolve_rfc_dependencies = true
generate_prompt_map = true
activate_spires = true
# ... additional action flags

[actions.execution_order]
steps = [
    "validate-document",
    "hash-document",
    "resolve-rfc-dependencies",
    # ... additional steps
]
```

**Constraints:**
- All action keys must be boolean
- `execution_order.steps` must be a list of strings
- Steps must be valid action names (kebab-case)

### 2.6 [outputs] Section

```toml
[outputs]
deterministic_overview = "outputs/deterministic_overview.md"
prompt_map = "outputs/prompt_map/"
iso_layer2_manifests = "manifests/layer2/"
# ... additional output paths
```

**Constraints:**
- All values must be valid relative paths
- Paths must not contain `..` or absolute references

### 2.7 [security] Section

```toml
[security]
strip_secrets = true
allowed_placeholders = ["${google_secret_manager:*}"]
forbidden_content = ["api_keys", "plaintext_credentials", "tokens"]
```

**Constraints:**
- `strip_secrets`: Must be true for production documents
- `forbidden_content`: Validated against document content

### 2.8 [review] Section

```toml
[review]
required = true
reviewer = "human"
summary_format = "markdown"
diff_required = true
apply_protection = true
```

---

## 3. Embedded Manifest Schema

### 3.1 [embedded.sx9_manifest.agent]

Agent identity and RFC compliance declaration.

```toml
[embedded.sx9_manifest.agent]
id = "sx9-onboarding-agent"
version = "7.3.1"
classification = "deterministic-bootstrap"
owner = "Synaptix9 Core"
created_at = "auto:now"
hash_algorithm = "murmur3-128"
rfc_compliance = ["9000", "9001", "9002", "9003", "9004", "9005", "9100", "9101", "9102", "9110"]
```

### 3.2 [embedded.sx9_manifest.data_fabric]

Zero-license data fabric configuration.

```toml
[embedded.sx9_manifest.data_fabric]
primary_database = "surrealdb"       # REQUIRED: "surrealdb"
cache_layer = "sledis"               # REQUIRED: "sledis" (NOT redis)
message_fabric = "nats"              # REQUIRED: "nats" (NOT kafka)
visualization = "sx9-foundation-visualizer"
graph_compute = "glaf-matroid-core"
ecs_engine = "legion"

[embedded.sx9_manifest.data_fabric.surrealdb]
host = "localhost"
port = 8000
namespace = "ctas7"
database = "glaf"
license = "MIT"

[embedded.sx9_manifest.data_fabric.sledis]
host = "localhost"
port = 6380
license = "MIT"
use_cases = ["session_cache", "hot_query_cache", "rate_limiting", "sorted_sets"]

[embedded.sx9_manifest.data_fabric.nats]
host = "localhost"
port = 4222
http_port = 8222
jetstream = true
license = "Apache-2.0"
subjects = [
    "sx9.tick",
    "sx9.workflow.spawned",
    "sx9.workflow.completed",
    "sx9.threat.detected",
    "sx9.visualizer.query",
    "sx9.ecs.sync"
]

[embedded.sx9_manifest.data_fabric.eliminated]
neo4j = { reason = "AGPL/Commercial license trap", replacement = "surrealdb" }
redis = { reason = "SSPL since 2024", replacement = "sledis" }
elasticsearch = { reason = "SSPL", replacement = "surrealdb" }
kafka = { reason = "Heavy, complex", replacement = "nats" }
```

**Zero-License Requirement:**  
Documents MUST NOT reference Neo4j, Redis, Elasticsearch, MongoDB, or Kafka as primary data stores. These are replaced by:
- Neo4j → SurrealDB
- Redis → Sledis
- Elasticsearch → SurrealDB
- Kafka → NATS

### 3.3 [embedded.sx9_manifest.ports]

Port allocation per RFC-9004.

```toml
[embedded.sx9_manifest.ports.infrastructure]
port_manager = 18104
trivariate_hash_engine = 18105
atlas_daemon = 18106
neural_mux = 18107
health_dashboard = 18108
lightning_qa = 18109
plasma_monitor = 18110
smart_crate_orchestrator = 18111

[embedded.sx9_manifest.ports.cdn]
node_1 = 18112
node_2 = 18113
node_3 = 18114
burst_start = 18115
burst_end = 18119

[embedded.sx9_manifest.ports.databases]
surrealdb = 8000
sledis = 6380
nats = 4222
nats_http = 8222

[embedded.sx9_manifest.ports.glaf]
glaf_core = 18019
glaf_analytics = 18025
visualizer = 18050

[embedded.sx9_manifest.ports.dynamic]
min = 1800
max = 1900
description = "Smart Crate dynamic allocation"
```

### 3.4 [embedded.sx9_manifest.bernoulli_zones]

Bernoulli zone latency bounds per RFC-9004.

```toml
[embedded.sx9_manifest.bernoulli_zones.tactical]
max_latency_us = 50
operations = ["trivariate_hash", "legion_ecs_tick", "slotgraph_update", "hd4_phase_transition"]

[embedded.sx9_manifest.bernoulli_zones.operational]
max_latency_us = 1000
operations = ["atlas_cognitive_tick", "ooda_decision", "health_aggregation", "neural_mux_route_update"]

[embedded.sx9_manifest.bernoulli_zones.analytical]
max_latency_ms = 100
operations = ["glaf_graph_analysis", "secondary_trivariate", "cdn_retrieval", "lightning_qa_validation"]

[embedded.sx9_manifest.bernoulli_zones.infrastructure]
max_latency_s = 60
operations = ["iac_manifold_spawn", "container_orchestration", "terraform_apply", "gpu_cluster_provision"]
```

### 3.5 [embedded.sx9_manifest.pipeline]

Pipeline stage configuration.

```toml
[embedded.sx9_manifest.pipeline.spires]
enabled = true
template_dir = "04-abe-iac/linkml_templates"
extractor = "04-abe-iac/spires_threat_extractor.py"
outputs = ["techniques", "actors", "rules", "tools", "campaigns"]

[embedded.sx9_manifest.pipeline.needle]
enabled = true
description = "Needle extraction for fine-grained entity resolution"
output_format = "json"

[embedded.sx9_manifest.pipeline.repo_scan]
enabled = true
targets = ["ctas7-*", "sx9-*"]
exclude = ["node_modules", "target", ".git", "__pycache__"]

[embedded.sx9_manifest.pipeline.data_spine]
enabled = true
description = "Extract data spines from scanned repositories"
output = "inventory/data-spine/"

[embedded.sx9_manifest.pipeline.ann_embedding]
enabled = true
model = "sentence-transformers/all-MiniLM-L6-v2"
vector_db = "surrealdb"
dimension = 384

[embedded.sx9_manifest.pipeline.glaf]
enabled = true
matroid_core = "ctas7-glaf-matroid-core"
hawkes_process = true
convergence_scoring = true
output = "graph/glaf/"

[embedded.sx9_manifest.pipeline.iso_layer2]
enabled = true
manifest_dir = "manifests/layer2/"
schema = "RFC-9100"
```

### 3.6 [embedded.sx9_manifest.gpu_escalation]

GPU escalation via ABE IAC.

```toml
[embedded.sx9_manifest.gpu_escalation]
enabled = true
trigger_threshold = 0.8
provider = "google_cloud"
machine_type = "n1-standard-8"
gpu_type = "nvidia-tesla-t4"
gpu_count = 1
preemptible = true
auto_shutdown_minutes = 30
billing_account = "${google_secret_manager:billing_account}"

[embedded.sx9_manifest.gpu_escalation.unicode_triggers]
spawn_gpu = '\u{EA02}'
spawn_conda = '\u{EA03}'
spawn_cdn = '\u{EA10}'
spawn_validation = '\u{EA11}'

[embedded.sx9_manifest.gpu_escalation.manifolds]
gpu_cluster = "04-abe-iac/sx9-model-serving-gcp/modules/"
conda_env = "04-abe-iac/cognetix-abe/firefly-deployment/"
terraform_registry = "04-abe-iac/ctas7-terraform-Registry/"
```

### 3.7 [embedded.sx9_manifest.visualizer]

Visualizer integration.

```toml
[embedded.sx9_manifest.visualizer]
name = "sx9-foundation-visualizer"
repository = "https://github.com/cp5337/sx9-foundation-visualizer"
crates = [
    "sx9-viz-core",
    "sx9-cdn-fabric",
    "sx9-adapters",
    "sx9-mcp-server",
    "sx9-cesium-bridge"
]
formats = ["graph", "table", "geojson", "cypher", "surql", "flow"]

[embedded.sx9_manifest.visualizer.adapters]
surrealdb = { enabled = true, priority = 1 }
postgres = { enabled = true, priority = 2 }
neo4j = { enabled = false, reason = "visualization-only, deprecated" }
geojson = { enabled = true, priority = 3 }
slotgraph = { enabled = true, priority = 1 }
network_flow = { enabled = true, priority = 2 }
```

### 3.8 [embedded.sx9_manifest.legion_ecs]

Legion ECS integration.

```toml
[embedded.sx9_manifest.legion_ecs]
world_crate = "ctas7-world-ecs"
slotgraph_crate = "ctas7-slotgraph-engine"
tick_rate_ms = 1
deterministic = true

[embedded.sx9_manifest.legion_ecs.components]
triptyx_id = true
position = true
ooda_phase = true
activity_state = true
convergence = true
timestamp = true
velocity = true
core_primitive = true
pole_primitive = true
ontology_mode = true
conceptual_atom = true
```

### 3.9 [embedded.sx9_manifest.qa]

Quality assurance configuration per RFC-9103.

```toml
[embedded.sx9_manifest.qa]
lightning_qa_port = 18109
phd_analyzer = "ctas7-qa-analyzer"
results_dir = "qa-analysis-results/"

[embedded.sx9_manifest.qa.thresholds]
ptcc_entropy_min = 0.7
tesla_loc_max = 200
nvnn_density_min = 0.05
complexity_max = 50

[embedded.sx9_manifest.qa.layers]
layer_1 = "lightning-qa-engine"
layer_2 = ["phd-analyzer", "zencoder-expert-qa", "claude-meta-agents"]
layer_3 = "unified-results-store"
layer_4 = ["statistical-cdn", "linear-integration", "iac-manifolds"]
```

### 3.10 [embedded.sx9_manifest.hd4]

HD4 framework configuration per RFC-9020.

```toml
[embedded.sx9_manifest.hd4]
phases = ["hunt", "detect", "disrupt", "disable", "dominate"]
active_phase = "detect"
escalation_enabled = true

[embedded.sx9_manifest.hd4.hunt]
description = "Proactive threat hunting and intelligence gathering"
tools = ["spires", "needle", "osint"]

[embedded.sx9_manifest.hd4.detect]
description = "Anomaly detection and threat identification"
tools = ["plasma", "wazuh", "sigma"]

[embedded.sx9_manifest.hd4.disrupt]
description = "Active disruption of threat actor operations"
requires_authorization = true

[embedded.sx9_manifest.hd4.disable]
description = "Neutralization of threat capabilities"
requires_authorization = true

[embedded.sx9_manifest.hd4.dominate]
description = "Full spectrum dominance and control"
requires_authorization = true
```

### 3.11 [embedded.sx9_manifest.metrics]

Metrics and observability.

```toml
[embedded.sx9_manifest.metrics]
enabled = true
export_format = "prometheus"
scrape_interval_s = 15

[embedded.sx9_manifest.metrics.targets]
neural_mux_latency_ns = { target = 250, alert_threshold = 500 }
atlas_tick_ms = { target = 1, alert_threshold = 2 }
port_utilization = { target = 0.7, alert_threshold = 0.9 }
glaf_convergence = { target = 0.8, alert_threshold = 0.6 }
qa_score = { target = 0.85, alert_threshold = 0.7 }
```

---

## 4. Validation Rules

### 4.1 Structural Validation

1. **Section Presence:** All required sections MUST be present
2. **Section Order:** Sections MUST appear in specified order
3. **Key Types:** All keys MUST have correct value types
4. **Required Keys:** All required keys MUST be present

### 4.2 Semantic Validation

1. **RFC References:** All RFC references MUST match `RFC-\d{4}[A-Z]?`
2. **Port Ranges:** Ports MUST be in valid ranges (1-65535)
3. **Bernoulli Zones:** Operations MUST be assigned to valid zones
4. **Path References:** All paths MUST be valid relative paths

### 4.3 Security Validation

1. **Forbidden Content:** Document MUST NOT contain forbidden content
2. **Placeholder Format:** Placeholders MUST match allowed patterns
3. **Secret Stripping:** Secrets MUST be stripped if `strip_secrets = true`

### 4.4 Determinism Validation

1. **Execution Order:** Steps MUST be deterministically ordered
2. **Hash Reproducibility:** Same input MUST produce same hash
3. **No External State:** Document MUST NOT depend on external mutable state

---

## 5. Execution Semantics

### 5.1 Execution Flow

```
1. Load document
2. Validate structure
3. Validate semantics
4. Validate security
5. Compute trivariate hash
6. Connect data fabric
7. Execute actions in order
8. Emit completion event
```

### 5.2 Action Execution

Actions execute in the order specified by `[actions.execution_order].steps`. Each action:

1. Checks if enabled in `[actions]`
2. Validates preconditions
3. Executes handler
4. Records result
5. Emits NATS event (if enabled)

### 5.3 Failure Handling

Per RFC-9112 Section 7:

- `E_VALIDATE_*`: Reject document, do not execute
- `E_RUNTIME_*`: Log, attempt recovery, continue or halt based on `strict` mode
- `E_FATAL_*`: Halt immediately, snapshot state

---

## 6. Hash Computation

### 6.1 Document Hash

Per RFC-9112 Section 4, the document hash is dual trivariate:

```
H1 = Murmur3_128(tick || mode || escalation)
H2 = Murmur3_128(intent || lineage || graph_delta)
```

For documents:
- `tick`: Document version as u64
- `mode`: 0x30 (ANALYZE)
- `escalation`: 0
- `intent`: SHA3-256 of document content
- `lineage`: UUID of document
- `graph_delta`: 0 (initial)

### 6.2 Auto-Hash Computation

When `hash = "auto:trivariate"`:

```python
def compute_auto_hash(document: dict) -> str:
    content = toml.dumps(document)
    h = hashlib.sha256(content.encode()).hexdigest()
    return f"sch:{h[:10]}:cuid:{h[10:20]}:uuid:{h[20:30]}"
```

---

## 7. Error Handling

### 7.1 Error Codes

| Code | Category | Description |
|------|----------|-------------|
| E_DOC_1001 | Structure | Missing required section |
| E_DOC_1002 | Structure | Invalid section order |
| E_DOC_1003 | Structure | Missing required key |
| E_DOC_1004 | Structure | Invalid value type |
| E_DOC_1101 | Semantic | Invalid RFC reference |
| E_DOC_1102 | Semantic | Invalid port range |
| E_DOC_1103 | Semantic | Invalid path reference |
| E_DOC_1201 | Security | Forbidden content detected |
| E_DOC_1202 | Security | Invalid placeholder |
| E_DOC_1301 | Execution | Action handler not found |
| E_DOC_1302 | Execution | Action precondition failed |
| E_DOC_1303 | Execution | Action timeout |

### 7.2 Error Response

```json
{
  "error_code": "E_DOC_1001",
  "message": "Missing required section: [embedded.sx9_manifest]",
  "location": "line 45",
  "severity": "fatal",
  "recovery": "Add required section"
}
```

---

## 8. Conformance

### 8.1 Conformance Levels

| Level | Requirements |
|-------|--------------|
| **L1: Basic** | Structure + Type validation |
| **L2: Standard** | L1 + Semantic validation |
| **L3: Strict** | L2 + Security validation |
| **L4: Certified** | L3 + Determinism verification |

### 8.2 Certification

Documents claiming RFC-9113 conformance MUST:

1. Pass all L4 validation checks
2. Include `schema = "RFC-9102"` in `[document]`
3. Include RFC-9113 in `rfc_compliance` list
4. Be parseable by reference implementation

---

## 9. Reference Implementation

### 9.1 Python Implementation

Located at: `sx9-conda/python-packages/sx9_orchestrator/`

```python
from sx9_orchestrator import ExecutableDocument, SX9Orchestrator

# Load and validate
doc = ExecutableDocument.load("sx9-executable-document.toml")
if doc.validate():
    # Execute
    orchestrator = SX9Orchestrator(OrchestratorConfig(
        document_path="sx9-executable-document.toml"
    ))
    results = await orchestrator.run()
```

### 9.2 CLI Usage

```bash
# Via sx9 CLI
sx9 orchestrate run sx9-executable-document.toml

# Validate only
sx9 orchestrate validate sx9-executable-document.toml

# Show hash
sx9 orchestrate hash sx9-executable-document.toml
```

### 9.3 Conda Package

```bash
# Install
conda install sx9-orchestrator

# Or via pip
pip install sx9-orchestrator
```

---

## Appendix A: Complete Example Document

See: `/Users/cp5337/Developer/sx9/sx9-executable-document.toml`

---

## Appendix B: RFC Dependencies

```
RFC-9113 requires:
├── RFC-9102  [Executable Document Framework]
│   └── Document structure and execution semantics
├── RFC-9112  [Deterministic Prompt Engineering]
│   └── Hash computation, error semantics
├── RFC-9004  [Deterministic Routing]
│   └── Port allocation, Bernoulli zones
├── RFC-9020  [HD4 Framework]
│   └── Phase definitions
└── RFC-9106  [sx9-conda]
    └── Python execution layer
```

---

## Appendix C: Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-03 | Initial specification |

---

**End of RFC-9113**

---

*"TOML is config. Document is program. Execution is deterministic."*


