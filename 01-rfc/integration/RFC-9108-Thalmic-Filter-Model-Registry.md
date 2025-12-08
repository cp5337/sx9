# RFC-9108: Thalmic Filter & Model Registry

**Status:** Draft
**Authors:** CTAS-7 Team
**Created:** 2024-12-01
**Domain:** L2 Cognitive Fabric, Model Inference, Atomic Clipboard
**Dependencies:** RFC-9001, RFC-9002, RFC-9003, RFC-9101, RFC-9107

---

## 1. Abstract

This RFC defines the **Thalmic Filter** - an always-on inference layer embedded at the L2 cognitive fabric level that intercepts ALL LLM prompts, tool invocations, and voice commands. The filter uses custom-trained small models (DistilBERT MITRE classifier, Phi-3 LoRA explainer) to classify, enrich, and sanitize operations before they reach external LLM providers.

**Key Principle:** The Thalmic Filter is NOT a standalone service - it is baked into the custom ISO at `/opt/ctas7/thalmic-filter/` and operates through the **Atomic Clipboard** as the mandatory state layer for all AI operations.

---

## 2. ISO-Level Integration

### 2.1 Custom ISO Directory Structure

```
/opt/ctas7/
├── thalmic-filter/
│   ├── bin/
│   │   ├── thalmic-intercept         # L2 fabric intercept daemon
│   │   ├── distilbert-classifier     # MITRE technique classifier
│   │   └── phi3-explainer            # Technique enrichment
│   ├── models/
│   │   ├── distilbert-mitre-v1/      # DistilBERT weights (local)
│   │   └── phi3-lora-v1/             # Phi-3 LoRA adapter (local)
│   ├── config/
│   │   ├── thalmic.toml              # Filter configuration
│   │   └── unicode-ops.toml          # Unicode operation mappings
│   └── cache/
│       └── inference-cache.sled      # Local Sled inference cache
├── atomic-clipboard/
│   ├── bin/
│   │   └── clipboard-service         # Atomic Clipboard daemon
│   ├── state/
│   │   └── clipboard.sled            # Persistent clipboard state
│   └── hooks/
│       └── pre-send.d/
│           └── 00-thalmic-filter.sh  # Pre-send hook to thalmic filter
├── dsl-orchestration/
│   ├── playbooks/                    # DSL playbook definitions
│   ├── interceptors/
│   │   └── thalmic-interceptor.wasm  # WASM interceptor for playbooks
│   └── runtime/
│       └── playbook-runner           # Playbook execution engine
└── l2-fabric/
    ├── neural-mux-client             # L2 Neural Mux client
    └── tool-controller               # L2 Tool Controller
```

### 2.2 ISO Build Integration

Add to `custom-iso-builder/scripts/build-ctas-iso.sh`:

```bash
# Install Thalmic Filter components
log "Installing Thalmic Filter subsystem..."
mkdir -p "$CHROOT_DIR/opt/ctas7/thalmic-filter"/{bin,models,config,cache}
mkdir -p "$CHROOT_DIR/opt/ctas7/atomic-clipboard"/{bin,state,hooks/pre-send.d}
mkdir -p "$CHROOT_DIR/opt/ctas7/dsl-orchestration"/{playbooks,interceptors,runtime}
mkdir -p "$CHROOT_DIR/opt/ctas7/l2-fabric"

# Copy pre-built Thalmic Filter binaries
cp /build-artifacts/thalmic-intercept "$CHROOT_DIR/opt/ctas7/thalmic-filter/bin/"
cp /build-artifacts/distilbert-classifier "$CHROOT_DIR/opt/ctas7/thalmic-filter/bin/"
cp /build-artifacts/phi3-explainer "$CHROOT_DIR/opt/ctas7/thalmic-filter/bin/"

# Copy model weights (local tier)
cp -r /build-artifacts/models/distilbert-mitre-v1 "$CHROOT_DIR/opt/ctas7/thalmic-filter/models/"
cp -r /build-artifacts/models/phi3-lora-v1 "$CHROOT_DIR/opt/ctas7/thalmic-filter/models/"

# Copy Atomic Clipboard service
cp /build-artifacts/clipboard-service "$CHROOT_DIR/opt/ctas7/atomic-clipboard/bin/"

# Copy WASM interceptor
cp /build-artifacts/thalmic-interceptor.wasm "$CHROOT_DIR/opt/ctas7/dsl-orchestration/interceptors/"

# Create systemd services
cat > "$CHROOT_DIR/etc/systemd/system/thalmic-filter.service" << 'EOF'
[Unit]
Description=CTAS-7 Thalmic Filter L2 Intercept
After=network.target atomic-clipboard.service
Requires=atomic-clipboard.service

[Service]
Type=simple
User=root
ExecStart=/opt/ctas7/thalmic-filter/bin/thalmic-intercept
Restart=always
RestartSec=5
Environment=THALMIC_MODEL_PATH=/opt/ctas7/thalmic-filter/models
Environment=THALMIC_ESCALATION_TIER=local

[Install]
WantedBy=multi-user.target
EOF

cat > "$CHROOT_DIR/etc/systemd/system/atomic-clipboard.service" << 'EOF'
[Unit]
Description=CTAS-7 Atomic Clipboard Service
After=network.target

[Service]
Type=simple
User=root
ExecStart=/opt/ctas7/atomic-clipboard/bin/clipboard-service
Restart=always
RestartSec=5
Environment=CLIPBOARD_STATE_PATH=/opt/ctas7/atomic-clipboard/state

[Install]
WantedBy=multi-user.target
EOF

# Enable services
chroot "$CHROOT_DIR" systemctl enable thalmic-filter.service
chroot "$CHROOT_DIR" systemctl enable atomic-clipboard.service
```

---

## 3. Thalmic Filter Architecture

### 3.1 L2 Fabric Intercept Layer

```
┌─────────────────────────────────────────────────────────────────┐
│                    Operator Workstation                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │ Prompt Window│  │ Prompt Window│  │ Kali Terminal│           │
│  │    (LLM 1)   │  │    (LLM 2)   │  │              │           │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘           │
│         │                 │                 │                    │
│         └────────────────┬┴─────────────────┘                   │
│                          │                                       │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              ATOMIC CLIPBOARD (State Layer)                  ││
│  │  ┌─────────────────────────────────────────────────────┐    ││
│  │  │                THALMIC FILTER                        │    ││
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │    ││
│  │  │  │ DistilBERT  │  │   Phi-3     │  │   Cache     │  │    ││
│  │  │  │  Classifier │  │  Explainer  │  │   (Sled)    │  │    ││
│  │  │  │  (~50ms)    │  │  (~200ms)   │  │             │  │    ││
│  │  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  │    ││
│  │  │         │                │                │         │    ││
│  │  │         └────────────────┴────────────────┘         │    ││
│  │  │                          │                          │    ││
│  │  │                          ▼                          │    ││
│  │  │                 ENRICHED CONTEXT                    │    ││
│  │  │        (MITRE tags, threat score, explanation)      │    ││
│  │  └─────────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────────┘│
│                          │                                       │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                 DSL/PLAYBOOK EXECUTOR                        ││
│  │  ┌─────────────────────────────────────────────────────┐    ││
│  │  │          WASM Thalmic Interceptor                    │    ││
│  │  │  (Pre-tool validation, Unicode op tagging)          │    ││
│  │  └─────────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────────┘│
│                          │                                       │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              L2 TOOL CONTROLLER                              ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          ││
│  │  │ XSD Validate│  │ Unicode Tag │  │ LISP Encode │          ││
│  │  └─────────────┘  └─────────────┘  └─────────────┘          ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    ESCALATION TIERS                              │
│                                                                  │
│  LOCAL (Tier 0)          GCP (Tier 1)          CLOUDFLARE (Tier 2)
│  ─────────────────       ─────────────         ─────────────────  │
│  /opt/ctas7/models       models.sx9.io         cdn.sx9.io        │
│  <50ms latency           ~100ms latency        ~150ms latency    │
│  DevOps/BNE              Operational           Strategic         │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Atomic Clipboard Integration

The Atomic Clipboard is the mandatory state layer that:

1. **Captures** all prompt text, tool invocations, voice commands
2. **Routes** everything through Thalmic Filter before external dispatch
3. **Persists** enriched context (MITRE tags, threat scores) to Sled
4. **Shares** state across prompt windows, terminals, model inference

```rust
// Atomic Clipboard State Structure
pub struct AtomicClipboardState {
    /// Current clipboard content
    pub content: Vec<u8>,

    /// Thalmic filter result (always populated before dispatch)
    pub thalmic_result: Option<ThalmicResult>,

    /// Unicode operation tag (RFC-9002)
    pub unicode_op: Option<UnicodeOp>,

    /// LISP-encoded payload (U+E500-E5FF)
    pub lisp_payload: Option<LispPayload>,

    /// Trivariate hash (RFC-9001)
    pub trivariate_hash: TrivariateHash,

    /// Escalation level (RFC-9003)
    pub escalation_level: EscalationLevel,
}

#[derive(Debug, Clone)]
pub struct ThalmicResult {
    /// MITRE technique classification
    pub mitre_techniques: Vec<MitreTechnique>,

    /// Threat score (0.0-1.0)
    pub threat_score: f32,

    /// Phi-3 explanation (if escalation >= Operational)
    pub explanation: Option<String>,

    /// Classification latency
    pub latency_ms: u64,

    /// Model tier used (local/gcp/cloudflare)
    pub model_tier: ModelTier,
}
```

---

## 4. DSL/Playbook Integration

### 4.1 Playbook Pre-Step Hook

Every DSL playbook step MUST pass through the Thalmic Filter:

```toml
# Modified playbook format with thalmic integration
[playbook]
name = "network-recon-playbook"
version = "1.1"
description = "Network reconnaissance with 7-tier escalation and thalmic filtering"

[playbook.thalmic]
enabled = true
min_escalation = "tactical"
cache_ttl_seconds = 300
unicode_block = "E540-E55F"  # Thalmic ops range

[playbook.steps.1]
name = "initial-recon"
tier = 1  # WASM
tool = "nmap"
target = "10.133.247.0/24"
unicode_op = "\\u{E800}"  # kali-tool
thalmic_filter = true      # MANDATORY - cannot be disabled

[playbook.steps.2]
name = "deep-scan"
tier = 2  # microkernel
depends_on = ["initial-recon"]
tool = "metasploit"
unicode_op = "\\u{E801}"
thalmic_filter = true
thalmic_escalate = "operational"  # Force Phi-3 explanation
```

### 4.2 WASM Interceptor

```rust
// thalmic-interceptor.wasm - Pre-step validation
#[no_mangle]
pub extern "C" fn intercept_step(step_ptr: *const Step) -> InterceptResult {
    let step = unsafe { &*step_ptr };

    // 1. Send to Thalmic Filter via Atomic Clipboard
    let clipboard_result = atomic_clipboard::filter_and_classify(
        step.tool_invocation.as_bytes(),
        step.escalation_level,
    );

    // 2. Check for blocked techniques
    if clipboard_result.thalmic_result.mitre_techniques
        .iter()
        .any(|t| BLOCKED_TECHNIQUES.contains(&t.id))
    {
        return InterceptResult::Block(
            format!("Blocked technique: {:?}", clipboard_result.thalmic_result.mitre_techniques)
        );
    }

    // 3. Enrich step with thalmic context
    step.context.insert("thalmic_result", clipboard_result.thalmic_result);
    step.context.insert("unicode_op", format!("\\u{{E54{}}}", step.tier));

    InterceptResult::Allow(step)
}
```

---

## 5. Unicode+LISP Integration

### 5.1 Thalmic Unicode Block (U+E540-E55F)

```
Thalmic Filter Operations (U+E540-E55F)
═══════════════════════════════════════
U+E540  THALMIC_CLASSIFY       - Trigger DistilBERT classification
U+E541  THALMIC_EXPLAIN        - Trigger Phi-3 explanation
U+E542  THALMIC_CACHE_HIT      - Cache lookup succeeded
U+E543  THALMIC_CACHE_MISS     - Cache lookup missed
U+E544  THALMIC_ESCALATE       - Escalate to higher model tier
U+E545  THALMIC_BLOCK          - Block operation (threat detected)
U+E546  THALMIC_ENRICH         - Enrich context with MITRE tags
U+E547  THALMIC_LOCAL          - Use local model tier
U+E548  THALMIC_GCP            - Use GCP model tier
U+E549  THALMIC_CLOUDFLARE     - Use Cloudflare model tier
U+E54A  THALMIC_ATOMIC_WRITE   - Write to Atomic Clipboard
U+E54B  THALMIC_ATOMIC_READ    - Read from Atomic Clipboard
U+E54C  THALMIC_LISP_ENCODE    - Encode result as LISP S-expr
U+E54D  THALMIC_TRIVARIATE     - Generate trivariate hash
U+E54E  THALMIC_VOICE_FILTER   - Filter voice command
U+E54F  THALMIC_PROMPT_FILTER  - Filter LLM prompt
```

### 5.2 LISP Encoding for Tool Responses

```lisp
;; Thalmic-enriched tool response (LISP S-expression)
(thalmic-result
  (classification
    (technique "T1046" "Network Service Discovery")
    (technique "T1018" "Remote System Discovery")
    (tactic "TA0007" "Discovery"))
  (threat-score 0.42)
  (explanation "Network reconnaissance to identify live hosts and services.
                Common pre-cursor to lateral movement.")
  (unicode-op #xE546)  ; THALMIC_ENRICH
  (model-tier :local)
  (latency-ms 47))
```

---

## 6. Escalation-Aware Model Serving

### 6.1 Three-Tier Architecture

| Tier | Location | Models | Latency | Use Case |
|------|----------|--------|---------|----------|
| **Local (Tier 0)** | `/opt/ctas7/thalmic-filter/models/` | DistilBERT, Phi-3 LoRA | <50ms | DevOps, BNE, DevSecOps |
| **GCP (Tier 1)** | `models.sx9.io` (Cloud Run GPU) | Full models + ensemble | ~100ms | Operational analysis |
| **Cloudflare (Tier 2)** | `cdn.sx9.io` (Workers AI) | Edge inference | ~150ms | Strategic, global ops |

### 6.2 Model Selection Logic

```rust
impl ThalmicFilter {
    pub async fn classify(&self, input: &str, escalation: EscalationLevel) -> ThalmicResult {
        // 1. Check cache first
        if let Some(cached) = self.cache.get(input).await {
            return cached.with_cache_hit();
        }

        // 2. Select model tier based on escalation
        let tier = match escalation {
            EscalationLevel::Tactical => ModelTier::Local,
            EscalationLevel::Operational => {
                if self.gcp_available() { ModelTier::GCP }
                else { ModelTier::Local }
            },
            EscalationLevel::Strategic => {
                if self.cloudflare_available() { ModelTier::Cloudflare }
                else if self.gcp_available() { ModelTier::GCP }
                else { ModelTier::Local }
            },
        };

        // 3. Run classification
        let mitre_result = self.classify_with_tier(input, tier).await?;

        // 4. Optionally add explanation (Operational+)
        let explanation = if escalation >= EscalationLevel::Operational {
            Some(self.explain_with_phi3(input, &mitre_result, tier).await?)
        } else {
            None
        };

        // 5. Cache and return
        let result = ThalmicResult {
            mitre_techniques: mitre_result.techniques,
            threat_score: mitre_result.score,
            explanation,
            latency_ms: mitre_result.latency,
            model_tier: tier,
        };

        self.cache.insert(input, result.clone()).await;
        result
    }
}
```

---

## 7. DevOps/BNE/DevSecOps Local Availability

### 7.1 Zero-Configuration Local Mode

The custom ISO boots with **all models pre-loaded locally**:

```bash
# On boot, thalmic-filter.service starts with:
Environment=THALMIC_MODEL_PATH=/opt/ctas7/thalmic-filter/models
Environment=THALMIC_ESCALATION_TIER=local
Environment=THALMIC_CACHE_PATH=/opt/ctas7/thalmic-filter/cache

# Models are already present:
/opt/ctas7/thalmic-filter/models/
├── distilbert-mitre-v1/
│   ├── model.safetensors    # ~260MB
│   ├── tokenizer.json
│   └── config.json
└── phi3-lora-v1/
    ├── adapter_model.safetensors  # ~50MB LoRA adapter
    └── adapter_config.json
```

### 7.2 Offline Operation

For air-gapped/BNE operations:

```toml
# /opt/ctas7/thalmic-filter/config/thalmic.toml
[thalmic]
mode = "offline"
model_path = "/opt/ctas7/thalmic-filter/models"
cache_path = "/opt/ctas7/thalmic-filter/cache"

[thalmic.offline]
# Disable external tier fallback
gcp_enabled = false
cloudflare_enabled = false

# Use local models only
distilbert_path = "distilbert-mitre-v1"
phi3_path = "phi3-lora-v1"

# Cache aggressively
cache_ttl_hours = 168  # 1 week
cache_max_entries = 100000

[thalmic.atomic_clipboard]
state_path = "/opt/ctas7/atomic-clipboard/state"
pre_send_hooks = ["/opt/ctas7/atomic-clipboard/hooks/pre-send.d"]
```

### 7.3 Model Update Workflow

```bash
# For DevOps updating models:
sudo systemctl stop thalmic-filter

# Download new model version
ctas7-model-sync pull distilbert-mitre-v2 /opt/ctas7/thalmic-filter/models/

# Update config
sed -i 's/distilbert-mitre-v1/distilbert-mitre-v2/' /opt/ctas7/thalmic-filter/config/thalmic.toml

# Restart service
sudo systemctl start thalmic-filter

# Verify
ctas7-thalmic-test "Test network scan command"
```

---

## 8. Model Training & IP Registry

### 8.1 Training Data Location

```
04-abe-iac/node-interview-generator/output/training_data/
├── distilbert_classification.jsonl   # 1,094 MITRE techniques
├── distilbert_label_map.json         # Technique → ID mapping
└── phi3_lora_training.jsonl          # 227 LoRA training samples
```

### 8.2 Model IP Registry

Models are versioned and tracked as IP assets:

```json
{
  "model_registry": {
    "sx9_distilbert_mitre_v1": {
      "type": "classifier",
      "base_model": "distilbert-base-uncased",
      "fine_tuned_on": "mitre_attck_techniques",
      "training_samples": 1094,
      "tactics": 19,
      "techniques": 1094,
      "accuracy": 0.94,
      "version": "1.0.0",
      "created": "2024-12-01",
      "checksum": "sha256:abc123...",
      "license": "proprietary",
      "deployment_tiers": ["local", "gcp", "cloudflare"]
    },
    "sx9_phi3_lora_v1": {
      "type": "explainer",
      "base_model": "microsoft/phi-3-mini-4k-instruct",
      "adapter_type": "lora",
      "training_samples": 227,
      "version": "1.0.0",
      "created": "2024-12-01",
      "checksum": "sha256:def456...",
      "license": "proprietary",
      "deployment_tiers": ["local", "gcp"]
    }
  }
}
```

---

## 9. Integration Checklist

### 9.1 ISO Build

- [ ] Add thalmic-filter binaries to `/opt/ctas7/thalmic-filter/bin/`
- [ ] Include model weights in `/opt/ctas7/thalmic-filter/models/`
- [ ] Create `thalmic-filter.service` systemd unit
- [ ] Create `atomic-clipboard.service` systemd unit
- [ ] Add WASM interceptor to `/opt/ctas7/dsl-orchestration/interceptors/`

### 9.2 DSL/Playbook

- [ ] Add `thalmic_filter = true` to all playbook steps
- [ ] Implement WASM interceptor pre-step hook
- [ ] Register Unicode ops U+E540-E55F

### 9.3 L2 Fabric

- [ ] Integrate Atomic Clipboard with L2 Tool Controller
- [ ] Add LISP encoding for thalmic results
- [ ] Implement XSD validation for thalmic payloads

### 9.4 Model Serving

- [ ] Deploy local models to ISO
- [ ] Configure GCP Cloud Run endpoint
- [ ] Set up Cloudflare Workers AI edge

---

## 10. References

- **RFC-9001**: Trivariate Hashing Standard
- **RFC-9002**: Unicode Operational Routing
- **RFC-9003**: Operation Classifier & 7-tier Escalation
- **RFC-9101**: Smart Crate System v7.3.1+
- **RFC-9107**: Unified Agent Infrastructure
- **MITRE ATT&CK**: Enterprise Matrix v14
- **Training Data**: `04-abe-iac/node-interview-generator/output/training_data/`
