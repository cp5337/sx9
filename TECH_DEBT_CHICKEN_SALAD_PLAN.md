# 🐔 Tech Debt → Chicken Salad Plan

**Date:** December 16, 2025  
**Status:** Active  
**Philosophy:** Every stumble is an opportunity to systematize

---

## 📋 PHASE 1: RFC Alignment (Foundation)

### 1.1 RFC Updates Required

| RFC | Update | Priority |
|-----|--------|----------|
| **RFC-9001** | Add Section 10 (HashRef), Section 11 (Heredity) | 🔴 Critical |
| **RFC-9002** | New Unicode allocation (Class T/P/S/H) | 🔴 Critical |
| **RFC-9003** | Map 32 primitives to triggers | 🟡 High |
| **RFC-9016** | Delta angles: degrees → normalized (0.0-1.0) | 🟡 High |

### 1.2 Code Updates Required

```
sx9-foundation-core/src/
├── primitives.rs        → Change 0xE400 to 0xE500
├── hash_ref.rs          → NEW: HashRef struct (16 bytes)
├── heredity.rs          → NEW: Lisp operators (U+E800-E80C)
├── delta_position.rs    → UPDATE: 6-decimal normalized
└── unicode_registry.rs  → UPDATE: New class allocations
```

### 1.3 Source Files for Updates

```
/Users/cp5337/Developer/sx9/01-rfc/shuttle_folder/tasks/04-rfc-updates/
├── RFC_ALIGNMENT_ANALYSIS.md       ← Diff specs
├── RFC_9016_DELTA_ANGLE_FIX.md     ← Delta conversion
├── ANTIGRAVITY_PROMPT.md           ← Full correction prompt
├── smart-crate.toml                ← Authoritative Unicode allocation
└── Bundle and Corrections.zip      ← Additional materials

/Users/cp5337/Developer/sx9/01-rfc/rfc_alignment_temp/
├── ECS_ALIGNMENT_MASTER.md         ← 3-layer ECS spec
├── ECS_SECURITY_INTEGRATION.md     ← Security stack
├── PLASMA_DEFENDER_ECS_INTEGRATION.md
└── SX9_PLASMA_DEFENDER_CODEBASE_MAPPING.md
```

---

## 📋 PHASE 2: Tool Output Harvester (Docker)

### 2.1 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  TOOL OUTPUT HARVESTER (Docker Container)                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │ Kali Tool   │───▶│ Output      │───▶│ NATS/       │         │
│  │ Execution   │    │ Parser      │    │ JetStream   │         │
│  └─────────────┘    └─────────────┘    └─────────────┘         │
│        │                  │                  │                  │
│        │                  │                  │                  │
│        ▼                  ▼                  ▼                  │
│  ┌─────────────────────────────────────────────────────┐       │
│  │  Rust FFI Layer (Axum + Aya eBPF)                   │       │
│  │  - Tool invocation via FFI (no shell)               │       │
│  │  - Output capture to ring buffer                    │       │
│  │  - Unicode encoding (trivariate hash)               │       │
│  │  - NATS publish (lock-free)                         │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Initial 10 Tools for Harvesting

| Tool | Category | Output Format | Parser Complexity |
|------|----------|---------------|-------------------|
| **nmap** | Recon | XML/JSON | Medium |
| **masscan** | Recon | JSON | Low |
| **nuclei** | Vuln Scan | JSON | Low |
| **nikto** | Web | CSV/JSON | Medium |
| **sqlmap** | Web | JSON | Medium |
| **hashcat** | Password | Stdout | High |
| **john** | Password | Stdout | High |
| **theHarvester** | OSINT | JSON | Low |
| **amass** | OSINT | JSON | Low |
| **subfinder** | OSINT | JSON | Low |

### 2.3 Harvester Container Spec

```dockerfile
# sx9-tool-harvester/Dockerfile
FROM kalilinux/kali-rolling:latest

# Install tools
RUN apt-get update && apt-get install -y \
    nmap masscan nuclei nikto sqlmap \
    hashcat john theharvester amass subfinder \
    && rm -rf /var/lib/apt/lists/*

# Install Rust runtime
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy Rust FFI harness
COPY harvester/ /opt/harvester/
WORKDIR /opt/harvester
RUN cargo build --release

# Axum server on 8080
EXPOSE 8080

# NATS connection
ENV NATS_URL="nats://host.docker.internal:4222"

ENTRYPOINT ["/opt/harvester/target/release/sx9-harvester"]
```

### 2.4 Output Schema (Per Tool)

```toml
[tool_output]
tool_name = "nmap"
tool_version = "7.94"
execution_id = "uuid-v7"
trivariate_hash = "triv:SCH_CUID_UUID"
timestamp_ns = 1734350000000000000
duration_ms = 12500

[tool_output.input]
target = "192.168.1.0/24"
args = ["-sV", "-p1-1000"]

[tool_output.result]
format = "json"
size_bytes = 45678
checksum_sha256 = "abc123..."

[tool_output.parsed]
hosts_up = 15
ports_open = 47
services_detected = 23

[tool_output.nats]
subject = "sx9.tool.nmap.completed"
stream = "SX9_TOOL_OUTPUTS"
sequence = 12345
```

---

## 📋 PHASE 3: Custom Kali ISO

### 3.1 Current Status

```
✅ Build script exists: tools/kali-plasma/scripts/build-iso.sh
✅ eBPF tools scaffolded: tools/kali-plasma/ebpf-tools/
✅ Agent structure: tools/kali-plasma/agent/
❌ Axum server not embedded
❌ Rust FFI not integrated
❌ NATS client not embedded
❌ ISO not rolled
```

### 3.2 ISO Components to Add

```
kali-plasma-iso/
├── /opt/sx9/
│   ├── bin/
│   │   ├── sx9-harvester          ← Rust binary (tool harvester)
│   │   ├── sx9-axum-gateway       ← Axum HTTP/gRPC server
│   │   └── sx9-nats-bridge        ← NATS client
│   ├── lib/
│   │   ├── libsx9_ffi.so          ← Rust FFI for tools
│   │   └── libsx9_ebpf.so         ← eBPF loader
│   ├── ebpf/
│   │   ├── xdp_unicode_filter.o   ← Unicode trigger filter
│   │   └── tc_output_capture.o    ← Traffic capture
│   └── config/
│       ├── smart-crate.toml       ← Unicode allocation
│       └── nats.toml              ← NATS config
├── /etc/systemd/system/
│   ├── sx9-gateway.service
│   └── sx9-harvester.service
└── /usr/local/bin/
    └── sx9 → /opt/sx9/bin/sx9-harvester
```

### 3.3 Axum Gateway Spec

```rust
// sx9-axum-gateway/src/main.rs
use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // Health
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        
        // Tool execution
        .route("/tool/:name/execute", post(execute_tool))
        .route("/tool/:name/status/:id", get(tool_status))
        .route("/tool/:name/output/:id", get(tool_output))
        
        // NATS bridge
        .route("/nats/publish", post(nats_publish))
        .route("/nats/subscribe", get(nats_subscribe))
        
        // Unicode routing
        .route("/unicode/:rune/trigger", post(unicode_trigger))
        
        .layer(CorsLayer::permissive());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## 📋 PHASE 4: Execution Order

### Week 1: RFC Foundation
```
Day 1-2: Apply RFC-9001 updates (HashRef, heredity)
Day 3-4: Apply RFC-9002 updates (new Unicode classes)
Day 5:   Apply RFC-9016 updates (delta normalization)
```

### Week 2: Code Updates
```
Day 1-2: Update primitives.rs (0xE400 → 0xE500)
Day 3-4: Create HashRef + heredity modules
Day 5:   Update delta_position to normalized
```

### Week 3: Tool Harvester
```
Day 1-2: Create Docker container base
Day 3-4: Implement 5 tool parsers (nmap, masscan, nuclei, nikto, sqlmap)
Day 5:   Implement 5 tool parsers (hashcat, john, theHarvester, amass, subfinder)
```

### Week 4: ISO Integration
```
Day 1-2: Embed Axum gateway in ISO
Day 3-4: Integrate Rust FFI layer
Day 5:   Test full pipeline (tool → harvester → NATS → JetStream)
```

---

## 📋 PHASE 5: Validation Checklist

### RFC Validation
- [ ] RFC-9001 has Section 10 (HashRef) and Section 11 (Heredity)
- [ ] RFC-9002 has Class T/P/S/H allocations
- [ ] RFC-9016 uses normalized (0.0-1.0) delta angles
- [ ] smart-crate.toml is referenced as authoritative

### Code Validation
- [ ] primitives.rs uses 0xE500 base
- [ ] HashRef struct exists (16 bytes)
- [ ] Lisp heredity operators at U+E800-E80C
- [ ] Delta angles are 6-decimal normalized

### Harvester Validation
- [ ] Docker container builds
- [ ] All 10 tools execute via FFI
- [ ] Output captured to NATS
- [ ] Trivariate hashes generated

### ISO Validation
- [ ] ISO boots
- [ ] Axum gateway responds on :8080
- [ ] NATS bridge connects
- [ ] Tool execution works end-to-end

---

## 🔗 Related Files

```
# RFC Updates (source)
01-rfc/shuttle_folder/tasks/04-rfc-updates/

# RFC Updates (extracted)
01-rfc/rfc_alignment_temp/

# Kali Plasma (ISO)
tools/kali-plasma/

# Tool Scraper
tools/abe/iac/scrape-all-kali-tools.sh
tools/abe/iac/node-interview-generator/kali_tools_scraper.py

# Foundation Core (code updates)
crates/sx9-foundation-core/src/
```

---

**Next Action:** Start with Phase 1.1 - RFC-9001 HashRef update




