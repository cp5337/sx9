# RFC-9123: Gold Disk Reference Architecture & Disaster Recovery

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Depends On:** RFC-9120, RFC-9121, RFC-9122

---

## Abstract

RFC-9123 specifies the Gold Disk architecture — a disaster recovery and reference implementation system that eliminates protracted debugging sessions when code fratricide occurs. The core principle: **if it doesn't compile in 3 tries, restore from blessed baseline — no heroics.**

When an LLM under pressure starts generating "simple solutions" (fake code, placeholder stubs, TODO-riddled garbage), the system detects it immediately and reverts to the last known-good state. No extended "let me try one more thing" loops. No debugging marathons. Restore. Verify. Continue.

**Operational Philosophy:** You don't field-repair equipment in a firefight. You swap in the spare and keep moving.

---

## 1. Core Principles

### 1.1 The Three Laws of Gold Disk

1. **Never debug what you can restore.** If the blessed baseline exists and compiles, use it.

2. **Never trust rushed code.** Code generated under time pressure or after multiple failures is suspect until proven otherwise.

3. **Never accumulate technical debt during recovery.** Quick fixes become permanent problems. Restore clean, then iterate properly.

### 1.2 Anti-Patterns This Prevents

| Anti-Pattern | Symptom | Gold Disk Response |
|--------------|---------|-------------------|
| **Fake Code** | `todo!()`, `unimplemented!()`, placeholder returns | Detect and reject immediately |
| **Simplification Spiral** | "Let me just remove that feature for now" | Restore full implementation |
| **Debug Marathon** | 10+ attempts to fix compile errors | Hard stop at 3, restore |
| **Frankencode** | Mixing fixed code with broken code | Full restore, no partial |
| **Pressure Shortcuts** | "I'll add error handling later" | Reject incomplete code |

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         GOLD DISK ARCHITECTURE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      GOLD DISK REGISTRY                               │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │  │
│  │  │ sx9-core│ │sx9-nats │ │sx9-atlas│ │sx9-forge│ │sx9-teth │        │  │
│  │  │  v1.2.0 │ │  v1.1.0 │ │  v1.0.0 │ │  v0.9.0 │ │  v1.0.0 │        │  │
│  │  │ ✓ GOLD  │ │ ✓ GOLD  │ │ ✓ GOLD  │ │ ✓ GOLD  │ │ ✓ GOLD  │        │  │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘        │  │
│  │                     Hash-verified, test-certified                    │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         │  On disaster: instant restore                                    │
│         ▼                                                                   │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      DOCKER SHEET STACK                               │  │
│  │                                                                       │  │
│  │  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐                │  │
│  │  │   Layer 0   │   │   Layer 1   │   │   Layer 2   │                │  │
│  │  │  Base OS    │ → │  Runtime    │ → │ Application │                │  │
│  │  │  (distroless)│   │  (Rust/Py)  │   │  (Crates)   │                │  │
│  │  └─────────────┘   └─────────────┘   └─────────────┘                │  │
│  │                                                                       │  │
│  │  Compose orchestration for full stack restore                        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         │  Detection triggers                                              │
│         ▼                                                                   │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      CANARY DETECTION                                 │  │
│  │                                                                       │  │
│  │  • Fake code patterns (todo!, unimplemented!, placeholder)           │  │
│  │  • Compile failure count > 3                                         │  │
│  │  • Test coverage drop > 20%                                          │  │
│  │  • Cyclomatic complexity spike > 50%                                 │  │
│  │  • Missing error handling in new code                                │  │
│  │  • Removed functionality without RFC                                 │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         │  Automatic response                                              │
│         ▼                                                                   │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      RESTORE PROTOCOL                                 │  │
│  │                                                                       │  │
│  │  1. HALT current operation                                           │  │
│  │  2. SNAPSHOT failed state (forensics)                                │  │
│  │  3. RESTORE from Gold Disk                                           │  │
│  │  4. VERIFY compilation and tests                                     │  │
│  │  5. RESUME from known-good state                                     │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Gold Disk Registry

### 3.1 What Makes a "Gold" Release

A crate earns Gold status when:

```toml
[gold.criteria]
# All must be true

compilation = "clean"           # Zero warnings, zero errors
test_coverage = ">= 80%"        # Minimum test coverage
qa_grade = "A"                  # Lightning QA Grade A (85+)
security_audit = "pass"         # cargo audit clean
birth_certificate = "valid"     # crate_interview.json present and verified
integration_tests = "pass"      # Full integration suite green
performance_baseline = "set"    # Benchmarks recorded
documentation = "complete"      # All public APIs documented
```

### 3.2 Gold Disk Manifest

```toml
# gold-disk-manifest.toml
# The blessed baseline of the entire system

[manifest]
version = "2025.12.20"
created_at = "2025-12-20T00:00:00Z"
created_by = "release-pipeline"
hash = "abc123..."  # Hash of entire manifest

[system]
name = "sx9-platform"
environment = "production"

# ═══════════════════════════════════════════════════════════════════════════════
# GOLD CRATES - Blessed versions of all components
# ═══════════════════════════════════════════════════════════════════════════════

[crates.sx9-core]
version = "1.2.0"
source = "registry"
hash = "sha256:abc123..."
docker_image = "sx9/sx9-core:1.2.0-gold"
gold_certified = "2025-12-15T00:00:00Z"
qa_grade = "A"
qa_score = 94

[crates.sx9-nats-router]
version = "1.1.0"
source = "registry"
hash = "sha256:def456..."
docker_image = "sx9/sx9-nats-router:1.1.0-gold"
gold_certified = "2025-12-14T00:00:00Z"
qa_grade = "A"
qa_score = 91

[crates.sx9-atlas]
version = "1.0.0"
source = "registry"
hash = "sha256:ghi789..."
docker_image = "sx9/sx9-atlas:1.0.0-gold"
gold_certified = "2025-12-10T00:00:00Z"
qa_grade = "A"
qa_score = 89

[crates.sx9-sledis]
version = "1.0.0"
source = "registry"
hash = "sha256:jkl012..."
docker_image = "sx9/sx9-sledis:1.0.0-gold"
gold_certified = "2025-12-08T00:00:00Z"
qa_grade = "A"
qa_score = 92

[crates.sx9-forge]
version = "0.9.0"
source = "registry"
hash = "sha256:mno345..."
docker_image = "sx9/sx9-forge:0.9.0-gold"
gold_certified = "2025-12-18T00:00:00Z"
qa_grade = "A"
qa_score = 87

[crates.sx9-lightning-qa]
version = "1.0.0"
source = "registry"
hash = "sha256:pqr678..."
docker_image = "sx9/sx9-lightning-qa:1.0.0-gold"
gold_certified = "2025-12-12T00:00:00Z"
qa_grade = "A"
qa_score = 95

[crates.sx9-teth]
version = "1.0.0"
source = "registry"
hash = "sha256:stu901..."
docker_image = "sx9/sx9-teth:1.0.0-gold"
gold_certified = "2025-12-11T00:00:00Z"
qa_grade = "A"
qa_score = 90

# ═══════════════════════════════════════════════════════════════════════════════
# INFRASTRUCTURE - External dependencies
# ═══════════════════════════════════════════════════════════════════════════════

[infrastructure.nats]
version = "2.10.0"
docker_image = "nats:2.10.0-alpine"
config_hash = "sha256:xyz..."

[infrastructure.surrealdb]
version = "1.1.0"
docker_image = "surrealdb/surrealdb:1.1.0"
config_hash = "sha256:xyz..."

[infrastructure.postgres]
version = "16.0"
docker_image = "postgres:16.0-alpine"
config_hash = "sha256:xyz..."

# ═══════════════════════════════════════════════════════════════════════════════
# RESTORE COMMANDS - One-liners for each component
# ═══════════════════════════════════════════════════════════════════════════════

[restore]
# Full system restore
full = "sx9-gold restore --manifest gold-disk-manifest.toml --all"

# Individual crate restore
sx9-core = "sx9-gold restore --crate sx9-core --version 1.2.0"
sx9-nats-router = "sx9-gold restore --crate sx9-nats-router --version 1.1.0"
sx9-atlas = "sx9-gold restore --crate sx9-atlas --version 1.0.0"

# Infrastructure restore
infrastructure = "docker compose -f gold-disk-compose.yml up -d"
```

---

## 4. Docker Sheet Stack

### 4.1 Base Image Hierarchy

```dockerfile
# ═══════════════════════════════════════════════════════════════════════════════
# LAYER 0: Base OS (Distroless)
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/base/Dockerfile.base

FROM gcr.io/distroless/cc-debian12:latest AS base

# Nothing else. Minimal attack surface.
# No shell, no package manager, no nothing.

LABEL org.sx9.layer="0"
LABEL org.sx9.purpose="base"


# ═══════════════════════════════════════════════════════════════════════════════
# LAYER 1: Rust Runtime
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/runtime/Dockerfile.rust

FROM rust:1.74-slim AS builder

WORKDIR /build

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build actual code
COPY src ./src
COPY crate_interview.json smartcrate.toml ./
RUN cargo build --release

# Final image
FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /build/target/release/sx9-* /usr/local/bin/

LABEL org.sx9.layer="1"
LABEL org.sx9.runtime="rust"


# ═══════════════════════════════════════════════════════════════════════════════
# LAYER 1: Python Runtime (for tools)
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/runtime/Dockerfile.python

FROM python:3.11-slim AS builder

WORKDIR /build

# Install dependencies
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt --target /deps

# Copy application
COPY src ./src
COPY crate_interview.json smartcrate.toml ./

# Final image
FROM gcr.io/distroless/python3-debian12:latest

COPY --from=builder /deps /deps
COPY --from=builder /build/src /app/src
COPY --from=builder /build/*.json /build/*.toml /app/

ENV PYTHONPATH=/deps
WORKDIR /app

LABEL org.sx9.layer="1"
LABEL org.sx9.runtime="python"
```

### 4.2 Component Docker Sheets

```dockerfile
# ═══════════════════════════════════════════════════════════════════════════════
# SX9-CORE
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/crates/Dockerfile.sx9-core

FROM sx9/runtime-rust:latest AS builder

WORKDIR /build
COPY crates/sx9-core ./

# Verify birth certificate before build
RUN test -f crate_interview.json || (echo "FATAL: No birth certificate" && exit 1)

# Build
RUN cargo build --release

# Verify post-build
RUN cargo test --release
RUN sx9-lightning-qa analyze . --ci --min-grade A

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /build/target/release/libsx9_core.so /usr/local/lib/
COPY --from=builder /build/crate_interview.json /etc/sx9/
COPY --from=builder /build/smartcrate.toml /etc/sx9/

LABEL org.sx9.crate="sx9-core"
LABEL org.sx9.version="1.2.0"
LABEL org.sx9.gold="true"
LABEL org.sx9.qa_grade="A"


# ═══════════════════════════════════════════════════════════════════════════════
# SX9-NATS-ROUTER
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/crates/Dockerfile.sx9-nats-router

FROM sx9/runtime-rust:latest AS builder

WORKDIR /build
COPY crates/sx9-nats-router ./

# Verify birth certificate
RUN test -f crate_interview.json || (echo "FATAL: No birth certificate" && exit 1)

# Build with NATS feature
RUN cargo build --release --features "nats"

# Verify
RUN cargo test --release
RUN sx9-lightning-qa analyze . --ci --min-grade A

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /build/target/release/sx9-nats-router /usr/local/bin/
COPY --from=builder /build/crate_interview.json /etc/sx9/
COPY --from=builder /build/smartcrate.toml /etc/sx9/

EXPOSE 4222

ENTRYPOINT ["/usr/local/bin/sx9-nats-router"]

LABEL org.sx9.crate="sx9-nats-router"
LABEL org.sx9.version="1.1.0"
LABEL org.sx9.gold="true"


# ═══════════════════════════════════════════════════════════════════════════════
# SX9-FORGE (Dev Forge)
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/crates/Dockerfile.sx9-forge

FROM sx9/runtime-rust:latest AS rust-builder
WORKDIR /build/backend
COPY crates/sx9-forge/backend ./
RUN cargo build --release

FROM node:20-slim AS ui-builder
WORKDIR /build/ui
COPY crates/sx9-forge/ui ./
RUN npm ci && npm run build

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=rust-builder /build/backend/target/release/sx9-forge-api /usr/local/bin/
COPY --from=ui-builder /build/ui/dist /var/www/forge/

EXPOSE 8080

LABEL org.sx9.crate="sx9-forge"
LABEL org.sx9.version="0.9.0"
LABEL org.sx9.gold="true"


# ═══════════════════════════════════════════════════════════════════════════════
# SX9-LIGHTNING-QA
# ═══════════════════════════════════════════════════════════════════════════════
# File: docker/crates/Dockerfile.sx9-lightning-qa

FROM sx9/runtime-rust:latest AS builder

WORKDIR /build
COPY crates/sx9-lightning-qa ./

RUN cargo build --release
RUN cargo test --release

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /build/target/release/sx9-lightning-qa /usr/local/bin/
COPY --from=builder /build/schemas /etc/sx9/schemas/
COPY --from=builder /build/teth-rules /etc/sx9/teth/

ENTRYPOINT ["/usr/local/bin/sx9-lightning-qa"]

LABEL org.sx9.crate="sx9-lightning-qa"
LABEL org.sx9.version="1.0.0"
LABEL org.sx9.gold="true"
```

### 4.3 Gold Disk Compose

```yaml
# gold-disk-compose.yml
# Full system orchestration from blessed images

version: "3.9"

name: sx9-gold-disk

services:
  # ═══════════════════════════════════════════════════════════════════════════
  # INFRASTRUCTURE
  # ═══════════════════════════════════════════════════════════════════════════
  
  nats:
    image: nats:2.10.0-alpine
    container_name: sx9-nats
    ports:
      - "4222:4222"
      - "8222:8222"
    command: ["--config", "/etc/nats/nats.conf"]
    volumes:
      - ./config/nats.conf:/etc/nats/nats.conf:ro
      - nats-data:/data
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:8222/healthz"]
      interval: 5s
      timeout: 3s
      retries: 3
    labels:
      org.sx9.component: "infrastructure"
      org.sx9.gold: "true"

  surrealdb:
    image: surrealdb/surrealdb:1.1.0
    container_name: sx9-surrealdb
    ports:
      - "8000:8000"
    command: ["start", "--user", "root", "--pass", "${SURREAL_PASS}", "file:/data/sx9.db"]
    volumes:
      - surreal-data:/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 5s
      timeout: 3s
      retries: 3
    labels:
      org.sx9.component: "infrastructure"
      org.sx9.gold: "true"

  postgres:
    image: postgres:16.0-alpine
    container_name: sx9-postgres
    ports:
      - "5432:5432"
    environment:
      POSTGRES_DB: sx9
      POSTGRES_USER: sx9
      POSTGRES_PASSWORD: ${POSTGRES_PASS}
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./config/postgres-init.sql:/docker-entrypoint-initdb.d/init.sql:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U sx9"]
      interval: 5s
      timeout: 3s
      retries: 3
    labels:
      org.sx9.component: "infrastructure"
      org.sx9.gold: "true"

  # ═══════════════════════════════════════════════════════════════════════════
  # SX9 CORE SERVICES
  # ═══════════════════════════════════════════════════════════════════════════

  sx9-core:
    image: sx9/sx9-core:1.2.0-gold
    container_name: sx9-core
    depends_on:
      nats:
        condition: service_healthy
    environment:
      NATS_URL: nats://nats:4222
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-core"
      org.sx9.gold: "true"

  sx9-nats-router:
    image: sx9/sx9-nats-router:1.1.0-gold
    container_name: sx9-nats-router
    depends_on:
      nats:
        condition: service_healthy
      sx9-core:
        condition: service_started
    ports:
      - "8080:8080"
    environment:
      NATS_URL: nats://nats:4222
      RUST_LOG: info
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 10s
      timeout: 5s
      retries: 3
    labels:
      org.sx9.crate: "sx9-nats-router"
      org.sx9.gold: "true"

  sx9-atlas:
    image: sx9/sx9-atlas:1.0.0-gold
    container_name: sx9-atlas
    depends_on:
      nats:
        condition: service_healthy
    environment:
      NATS_URL: nats://nats:4222
      TICK_INTERVAL_MS: 100
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-atlas"
      org.sx9.gold: "true"

  sx9-sledis:
    image: sx9/sx9-sledis:1.0.0-gold
    container_name: sx9-sledis
    depends_on:
      nats:
        condition: service_healthy
    ports:
      - "6379:6379"
    volumes:
      - sledis-data:/data
    environment:
      NATS_URL: nats://nats:4222
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-sledis"
      org.sx9.gold: "true"

  # ═══════════════════════════════════════════════════════════════════════════
  # FORGE PIPELINE
  # ═══════════════════════════════════════════════════════════════════════════

  sx9-forge:
    image: sx9/sx9-forge:0.9.0-gold
    container_name: sx9-forge
    depends_on:
      - sx9-nats-router
      - sx9-sledis
    ports:
      - "3000:3000"
    environment:
      NATS_URL: nats://nats:4222
      SLEDIS_URL: redis://sx9-sledis:6379
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-forge"
      org.sx9.gold: "true"

  sx9-lightning-qa:
    image: sx9/sx9-lightning-qa:1.0.0-gold
    container_name: sx9-lightning-qa
    depends_on:
      - nats
    environment:
      NATS_URL: nats://nats:4222
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-lightning-qa"
      org.sx9.gold: "true"

  sx9-factory-agent:
    image: sx9/sx9-factory-agent:1.0.0-gold
    container_name: sx9-factory-agent
    depends_on:
      - sx9-forge
      - sx9-lightning-qa
    volumes:
      - ./workspaces:/workspaces
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      NATS_URL: nats://nats:4222
      WORKSPACE_ROOT: /workspaces
      MAX_RETRIES: 3
      RUST_LOG: info
    labels:
      org.sx9.crate: "sx9-factory-agent"
      org.sx9.gold: "true"

volumes:
  nats-data:
  surreal-data:
  postgres-data:
  sledis-data:

networks:
  default:
    name: sx9-gold-network
```

---

## 5. Fake Code Detection (Canary System)

### 5.1 Fake Code Patterns

```rust
// canary_detector.rs
// Detects "fake ass code" patterns

pub struct CanaryDetector {
    patterns: Vec<FakeCodePattern>,
}

#[derive(Debug)]
pub enum FakeCodePattern {
    // Rust patterns
    TodoMacro,                    // todo!()
    UnimplementedMacro,           // unimplemented!()
    PanicPlaceholder,             // panic!("not implemented")
    EmptyImpl,                    // impl Trait for X {} with no methods
    StubReturn,                   // return Default::default() everywhere
    CommentedLogic,               // Large blocks of // TODO: implement
    
    // Python patterns
    PassStatement,                // def foo(): pass
    NotImplementedError,          // raise NotImplementedError()
    EllipsisBody,                 // def foo(): ...
    
    // Universal patterns
    HardcodedTestData,            // return vec![1, 2, 3] in production code
    MissingErrorHandling,         // No Result/Option handling in fallible code
    EmptyExceptBlock,             // except: pass / catch {}
    MagicReturnValues,            // return 42, return "test"
    
    // Structural patterns
    SuddenSimplification,         // Function went from 50 lines to 5
    FeatureRemoval,               // Public API removed without deprecation
    TestRemoval,                  // Tests deleted to make CI pass
}

pub struct CanaryResult {
    pub pattern: FakeCodePattern,
    pub location: String,
    pub severity: CanarySeverity,
    pub evidence: String,
}

pub enum CanarySeverity {
    Critical,   // Immediate halt, restore required
    High,       // Block merge, require fix
    Medium,     // Warning, require justification
    Low,        // Advisory
}

impl CanaryDetector {
    pub fn scan_rust(&self, source: &str, path: &Path) -> Vec<CanaryResult> {
        let mut results = Vec::new();
        
        // todo!() detection
        let todo_regex = Regex::new(r"todo!\s*\(\s*\)").unwrap();
        for (line_num, line) in source.lines().enumerate() {
            if todo_regex.is_match(line) {
                results.push(CanaryResult {
                    pattern: FakeCodePattern::TodoMacro,
                    location: format!("{}:{}", path.display(), line_num + 1),
                    severity: CanarySeverity::Critical,
                    evidence: line.trim().to_string(),
                });
            }
        }
        
        // unimplemented!() detection
        let unimpl_regex = Regex::new(r"unimplemented!\s*\(").unwrap();
        for (line_num, line) in source.lines().enumerate() {
            if unimpl_regex.is_match(line) {
                results.push(CanaryResult {
                    pattern: FakeCodePattern::UnimplementedMacro,
                    location: format!("{}:{}", path.display(), line_num + 1),
                    severity: CanarySeverity::Critical,
                    evidence: line.trim().to_string(),
                });
            }
        }
        
        // Default::default() as stub return
        let default_regex = Regex::new(r"return\s+Default::default\(\)").unwrap();
        for (line_num, line) in source.lines().enumerate() {
            if default_regex.is_match(line) {
                // Check if this is in a complex function (likely a stub)
                results.push(CanaryResult {
                    pattern: FakeCodePattern::StubReturn,
                    location: format!("{}:{}", path.display(), line_num + 1),
                    severity: CanarySeverity::High,
                    evidence: line.trim().to_string(),
                });
            }
        }
        
        // Empty error handling: .unwrap() chains
        let unwrap_chain = Regex::new(r"\.unwrap\(\).*\.unwrap\(\).*\.unwrap\(\)").unwrap();
        for (line_num, line) in source.lines().enumerate() {
            if unwrap_chain.is_match(line) {
                results.push(CanaryResult {
                    pattern: FakeCodePattern::MissingErrorHandling,
                    location: format!("{}:{}", path.display(), line_num + 1),
                    severity: CanarySeverity::High,
                    evidence: line.trim().to_string(),
                });
            }
        }
        
        results
    }
    
    pub fn scan_for_simplification(&self, before: &str, after: &str) -> Option<CanaryResult> {
        let before_lines = before.lines().count();
        let after_lines = after.lines().count();
        
        // If code shrank by more than 50%, it's suspicious
        if before_lines > 20 && after_lines < before_lines / 2 {
            return Some(CanaryResult {
                pattern: FakeCodePattern::SuddenSimplification,
                location: "diff".to_string(),
                severity: CanarySeverity::Critical,
                evidence: format!(
                    "Code reduced from {} to {} lines ({}% reduction)",
                    before_lines,
                    after_lines,
                    ((before_lines - after_lines) * 100) / before_lines
                ),
            });
        }
        
        None
    }
    
    pub fn scan_for_feature_removal(&self, before_api: &[String], after_api: &[String]) -> Vec<CanaryResult> {
        let mut results = Vec::new();
        
        for api in before_api {
            if !after_api.contains(api) {
                results.push(CanaryResult {
                    pattern: FakeCodePattern::FeatureRemoval,
                    location: api.clone(),
                    severity: CanarySeverity::Critical,
                    evidence: format!("Public API '{}' was removed", api),
                });
            }
        }
        
        results
    }
}
```

### 5.2 Canary CI Gate

```yaml
# .github/workflows/canary-check.yml

name: Canary Detection

on:
  pull_request:
    branches: [develop, main]

jobs:
  canary:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Need history for comparison
      
      - name: Run Canary Detector
        id: canary
        run: |
          sx9-canary scan . --format json > canary-report.json
          
          CRITICAL=$(jq '[.results[] | select(.severity == "Critical")] | length' canary-report.json)
          HIGH=$(jq '[.results[] | select(.severity == "High")] | length' canary-report.json)
          
          echo "critical=$CRITICAL" >> $GITHUB_OUTPUT
          echo "high=$HIGH" >> $GITHUB_OUTPUT
          
          if [ "$CRITICAL" -gt 0 ]; then
            echo "::error::CANARY ALERT: $CRITICAL critical fake code patterns detected"
            cat canary-report.json | jq '.results[] | select(.severity == "Critical")'
            exit 1
          fi
          
          if [ "$HIGH" -gt 3 ]; then
            echo "::error::CANARY ALERT: Too many high-severity patterns ($HIGH)"
            exit 1
          fi
      
      - name: Check for Simplification
        run: |
          # Compare changed files against their previous versions
          git diff origin/develop --name-only -- '*.rs' '*.py' | while read file; do
            if [ -f "$file" ]; then
              BEFORE=$(git show origin/develop:"$file" 2>/dev/null | wc -l || echo "0")
              AFTER=$(wc -l < "$file")
              
              if [ "$BEFORE" -gt 20 ] && [ "$AFTER" -lt $((BEFORE / 2)) ]; then
                echo "::error file=$file::Suspicious simplification: $BEFORE -> $AFTER lines"
                exit 1
              fi
            fi
          done
      
      - name: Alert on Failure
        if: failure()
        run: |
          sx9-slack-notify \
            --channel "#sx9-alerts" \
            --message "🚨 CANARY ALERT: Fake code detected in PR #${{ github.event.pull_request.number }}\n\nThis may indicate LLM under pressure generating placeholder code.\n\nRecommendation: RESTORE FROM GOLD DISK"
```

---

## 6. Restore Protocol

### 6.1 Restore Decision Tree

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         RESTORE DECISION TREE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  TRIGGER DETECTED ────────────────────────────────────────────────────────▶│
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Q1: Is this a compile failure?                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│         │                                                                   │
│    YES  │  NO                                                               │
│    ▼    └──────────────────────────────────────▶                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Q2: Is this attempt #3 or higher?                                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│         │                                                                   │
│    YES  │  NO                                                               │
│    ▼    └──────────────────────────────────────▶ Try to fix (1 more try)   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ACTION: RESTORE FROM GOLD DISK                                      │   │
│  │ DO NOT attempt further fixes                                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  TRIGGER DETECTED (Canary) ───────────────────────────────────────────────▶│
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Q3: Is the canary severity CRITICAL?                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│         │                                                                   │
│    YES  │  NO                                                               │
│    ▼    └──────────────────────────────────────▶ Flag for review           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ACTION: IMMEDIATE RESTORE                                           │   │
│  │ Fake code detected - do not merge                                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  TRIGGER DETECTED (QA Grade D/F) ─────────────────────────────────────────▶│
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ACTION: RESTORE FROM GOLD DISK                                      │   │
│  │ Fundamental architectural issues                                    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Restore Commands

```bash
#!/bin/bash
# sx9-gold - Gold Disk restore utility

set -euo pipefail

GOLD_MANIFEST="${GOLD_MANIFEST:-gold-disk-manifest.toml}"
GOLD_REGISTRY="${GOLD_REGISTRY:-registry.sx9.dev}"

# ═══════════════════════════════════════════════════════════════════════════════
# Full system restore
# ═══════════════════════════════════════════════════════════════════════════════
restore_full() {
    echo "🔄 FULL SYSTEM RESTORE FROM GOLD DISK"
    echo "======================================"
    
    # 1. Stop all services
    echo "Stopping all services..."
    docker compose -f gold-disk-compose.yml down
    
    # 2. Pull gold images
    echo "Pulling gold images..."
    docker compose -f gold-disk-compose.yml pull
    
    # 3. Start infrastructure first
    echo "Starting infrastructure..."
    docker compose -f gold-disk-compose.yml up -d nats surrealdb postgres
    sleep 5
    
    # 4. Start core services
    echo "Starting core services..."
    docker compose -f gold-disk-compose.yml up -d sx9-core sx9-sledis sx9-atlas
    sleep 3
    
    # 5. Start application services
    echo "Starting application services..."
    docker compose -f gold-disk-compose.yml up -d
    
    # 6. Verify
    echo "Verifying restore..."
    verify_system
    
    echo "✅ RESTORE COMPLETE"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Single crate restore
# ═══════════════════════════════════════════════════════════════════════════════
restore_crate() {
    local crate=$1
    local version=$2
    
    echo "🔄 Restoring $crate to gold version $version"
    
    # 1. Get gold image
    local image="sx9/${crate}:${version}-gold"
    docker pull "$image"
    
    # 2. Stop current
    docker stop "sx9-${crate}" 2>/dev/null || true
    docker rm "sx9-${crate}" 2>/dev/null || true
    
    # 3. Start gold version
    docker compose -f gold-disk-compose.yml up -d "${crate}"
    
    # 4. Verify
    echo "Verifying $crate..."
    sleep 2
    docker exec "sx9-${crate}" /health-check.sh || {
        echo "❌ Health check failed after restore"
        exit 1
    }
    
    echo "✅ $crate restored to $version"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Code restore (git)
# ═══════════════════════════════════════════════════════════════════════════════
restore_code() {
    local crate=$1
    
    echo "🔄 Restoring code for $crate from gold tag"
    
    # 1. Get gold version from manifest
    local version=$(toml get "$GOLD_MANIFEST" "crates.${crate}.version" --raw)
    local gold_tag="gold/${crate}/v${version}"
    
    # 2. Snapshot current state (forensics)
    local snapshot_branch="snapshot/$(date +%Y%m%d-%H%M%S)/${crate}"
    git checkout -b "$snapshot_branch"
    git add -A
    git commit -m "Snapshot before restore: $crate" || true
    git push origin "$snapshot_branch"
    
    # 3. Restore from gold tag
    git checkout develop
    git checkout "$gold_tag" -- "crates/${crate}"
    git commit -m "RESTORE: $crate from gold version $version"
    
    echo "✅ Code restored. Snapshot saved to $snapshot_branch"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Verification
# ═══════════════════════════════════════════════════════════════════════════════
verify_system() {
    echo "Running system verification..."
    
    # Check all containers are running
    local expected=$(docker compose -f gold-disk-compose.yml config --services | wc -l)
    local running=$(docker compose -f gold-disk-compose.yml ps --status running | tail -n +2 | wc -l)
    
    if [ "$running" -ne "$expected" ]; then
        echo "❌ Expected $expected services, only $running running"
        docker compose -f gold-disk-compose.yml ps
        exit 1
    fi
    
    # Check health endpoints
    curl -sf http://localhost:8080/health > /dev/null || {
        echo "❌ NATS router health check failed"
        exit 1
    }
    
    # Verify gold hashes
    for crate in sx9-core sx9-nats-router sx9-atlas sx9-sledis; do
        local expected_hash=$(toml get "$GOLD_MANIFEST" "crates.${crate}.hash" --raw)
        local actual_hash=$(docker exec "sx9-${crate}" cat /etc/sx9/crate_interview.json | sha256sum | cut -d' ' -f1)
        
        if [ "$expected_hash" != "sha256:$actual_hash" ]; then
            echo "❌ Hash mismatch for $crate"
            echo "   Expected: $expected_hash"
            echo "   Actual:   sha256:$actual_hash"
            exit 1
        fi
    done
    
    echo "✅ All verifications passed"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════════════════════
case "${1:-}" in
    restore)
        case "${2:-}" in
            --all|--full)
                restore_full
                ;;
            --crate)
                restore_crate "${3:-}" "${4:-}"
                ;;
            --code)
                restore_code "${3:-}"
                ;;
            *)
                echo "Usage: sx9-gold restore [--all|--crate NAME VERSION|--code NAME]"
                exit 1
                ;;
        esac
        ;;
    verify)
        verify_system
        ;;
    *)
        echo "Usage: sx9-gold [restore|verify]"
        exit 1
        ;;
esac
```

### 6.3 Emergency Restore Protocol

```yaml
# EMERGENCY RESTORE PROTOCOL
# When: Compile failures > 3, Canary critical, QA Grade D/F
# Who: Automated or on-call engineer
# Time: < 5 minutes to stable state

emergency_restore:
  name: "Gold Disk Emergency Restore"
  triggers:
    - compile_failures >= 3
    - canary_severity == "Critical"
    - qa_grade in ["D", "F"]
    - factory_agent_halted
  
  steps:
    - name: "HALT"
      action: |
        # Stop all factory agents immediately
        sx9-factory halt --all
        
        # Notify
        sx9-slack-notify --channel "#sx9-alerts" --message "🚨 EMERGENCY RESTORE INITIATED"
    
    - name: "SNAPSHOT"
      action: |
        # Save failed state for forensics
        SNAPSHOT_ID=$(date +%Y%m%d-%H%M%S)
        mkdir -p /forensics/${SNAPSHOT_ID}
        
        # Copy failed code
        cp -r /workspaces/current /forensics/${SNAPSHOT_ID}/code
        
        # Copy logs
        docker compose logs > /forensics/${SNAPSHOT_ID}/docker-logs.txt
        
        # Copy build output
        cp -r /tmp/build-* /forensics/${SNAPSHOT_ID}/ 2>/dev/null || true
        
        echo "Snapshot saved: /forensics/${SNAPSHOT_ID}"
    
    - name: "RESTORE"
      action: |
        # Full system restore from gold disk
        sx9-gold restore --all
    
    - name: "VERIFY"
      action: |
        # Run verification suite
        sx9-gold verify
        
        # Run smoke tests
        sx9-test smoke --quick
    
    - name: "RESUME"
      action: |
        # Restart factory agents with clean state
        sx9-factory start --clean
        
        # Notify
        sx9-slack-notify --channel "#sx9-alerts" --message "✅ Emergency restore complete. System stable."
  
  post_incident:
    - "Review forensic snapshot"
    - "Identify root cause of failure"
    - "Update canary patterns if new fake code pattern found"
    - "File incident report in Linear"
```

---

## 7. Gold Certification Pipeline

### 7.1 Certification Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      GOLD CERTIFICATION PIPELINE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CANDIDATE VERSION ───────────────────────────────────────────────────────▶│
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │   Build     │  Clean compilation, zero warnings                         │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │   Test      │  100% tests pass, coverage >= 80%                         │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │ Lightning   │  Grade A (85+), no critical anti-patterns                 │
│  │    QA       │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │  Security   │  cargo audit clean, no known CVEs                         │
│  │   Audit     │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │  Canary     │  No fake code patterns                                    │
│  │   Scan      │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │Integration  │  Full integration test suite                              │
│  │   Tests     │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │ Performance │  Benchmarks within 10% of baseline                        │
│  │  Baseline   │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────┐                                                           │
│  │   Soak      │  24-hour stability test                                   │
│  │   Test      │                                                           │
│  └──────┬──────┘                                                           │
│         │ PASS                                                              │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    ✓ GOLD CERTIFIED                                 │   │
│  │                                                                       │  │
│  │  • Tag: gold/{crate}/v{version}                                      │  │
│  │  • Image: sx9/{crate}:{version}-gold                                 │  │
│  │  • Manifest: gold-disk-manifest.toml updated                         │  │
│  │  • Registry: Published to gold registry                              │  │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Certification Workflow

```yaml
# .github/workflows/gold-certification.yml

name: Gold Certification

on:
  workflow_dispatch:
    inputs:
      crate:
        description: 'Crate to certify'
        required: true
      version:
        description: 'Version to certify'
        required: true

jobs:
  certify:
    runs-on: ubuntu-latest
    timeout-minutes: 120  # Includes soak test
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Build
        run: |
          cd crates/${{ inputs.crate }}
          cargo build --release 2>&1 | tee build.log
          
          # Check for warnings
          if grep -q "warning:" build.log; then
            echo "::error::Build has warnings, cannot certify as gold"
            exit 1
          fi
      
      - name: Test
        run: |
          cd crates/${{ inputs.crate }}
          cargo test --release
          
          # Check coverage
          cargo tarpaulin --out Json > coverage.json
          COVERAGE=$(jq '.coverage_percentage' coverage.json)
          if (( $(echo "$COVERAGE < 80" | bc -l) )); then
            echo "::error::Coverage $COVERAGE% is below 80%"
            exit 1
          fi
      
      - name: Lightning QA
        run: |
          sx9-lightning-qa analyze crates/${{ inputs.crate }} --ci --min-grade A
      
      - name: Security Audit
        run: |
          cd crates/${{ inputs.crate }}
          cargo audit --deny warnings
      
      - name: Canary Scan
        run: |
          sx9-canary scan crates/${{ inputs.crate }} --fail-on-critical
      
      - name: Integration Tests
        run: |
          # Start test environment
          docker compose -f test-compose.yml up -d
          sleep 10
          
          # Run integration tests
          cargo test --release --features integration
          
          # Cleanup
          docker compose -f test-compose.yml down
      
      - name: Performance Baseline
        run: |
          cd crates/${{ inputs.crate }}
          cargo bench -- --save-baseline gold-${{ inputs.version }}
          
          # Compare to previous gold baseline
          cargo bench -- --baseline gold-previous --threshold 10
      
      - name: Soak Test (24h simulation)
        run: |
          # Run accelerated soak test (simulate 24h in ~30min)
          sx9-soak-test --crate ${{ inputs.crate }} --duration 30m --acceleration 48x
      
      - name: Certify
        run: |
          # Tag in git
          git tag "gold/${{ inputs.crate }}/v${{ inputs.version }}"
          git push origin "gold/${{ inputs.crate }}/v${{ inputs.version }}"
          
          # Build and push gold image
          docker build -t sx9/${{ inputs.crate }}:${{ inputs.version }}-gold \
            -f docker/crates/Dockerfile.${{ inputs.crate }} \
            crates/${{ inputs.crate }}
          docker push sx9/${{ inputs.crate }}:${{ inputs.version }}-gold
          
          # Update manifest
          sx9-gold-manifest update \
            --crate ${{ inputs.crate }} \
            --version ${{ inputs.version }} \
            --hash $(sha256sum crates/${{ inputs.crate }}/crate_interview.json | cut -d' ' -f1)
      
      - name: Notify
        run: |
          sx9-slack-notify \
            --channel "#sx9-releases" \
            --message "🏆 GOLD CERTIFIED: ${{ inputs.crate }} v${{ inputs.version }}\n\nThis version is now the blessed baseline for disaster recovery."
```

---

## 8. Integration with Forge Pipeline

### 8.1 Factory Agent with Gold Disk Fallback

```rust
// factory_agent.rs

pub async fn build_with_gold_fallback(
    prompt: &CanonicalPrompt,
    gold_manifest: &GoldManifest,
) -> Result<BuildResult, FactoryError> {
    let max_attempts = 3;
    let mut attempt = 0;
    
    loop {
        attempt += 1;
        log::info!("Build attempt {}/{}", attempt, max_attempts);
        
        // Build
        let result = build_from_prompt(prompt).await;
        
        match result {
            Ok(crate_path) => {
                // Run canary check
                let canary_result = canary_detector.scan(&crate_path)?;
                
                if canary_result.has_critical() {
                    log::error!("CANARY: Critical fake code detected");
                    
                    // Don't retry - this is fake code, not a fixable error
                    return Err(FactoryError::FakeCodeDetected {
                        patterns: canary_result.critical_patterns(),
                        recommendation: "RESTORE FROM GOLD DISK".to_string(),
                    });
                }
                
                // Run QA
                let qa_result = lightning_qa.analyze(&crate_path)?;
                
                match qa_result.grade {
                    Grade::A => {
                        return Ok(BuildResult::Success(crate_path));
                    }
                    Grade::B | Grade::C if attempt < max_attempts => {
                        log::warn!("QA Grade {}, attempting refactor", qa_result.grade);
                        apply_refactor_directives(&crate_path, &qa_result.directives).await?;
                        continue;
                    }
                    _ => {
                        // Grade D/F or max attempts reached
                        log::error!("QA Grade {} after {} attempts", qa_result.grade, attempt);
                        
                        // TRIGGER GOLD DISK RESTORE
                        trigger_gold_restore(prompt, gold_manifest).await?;
                        
                        return Err(FactoryError::QualityGateFailed {
                            grade: qa_result.grade,
                            restored_to_gold: true,
                        });
                    }
                }
            }
            
            Err(BuildError::CompilationFailed(errors)) if attempt < max_attempts => {
                log::warn!("Compilation failed, attempt {}", attempt);
                
                // Try to fix compile errors
                let fix_result = attempt_compile_fix(&errors).await;
                
                if fix_result.is_err() {
                    continue; // Let next iteration try fresh
                }
            }
            
            Err(e) => {
                log::error!("Build failed: {:?}", e);
                
                if attempt >= max_attempts {
                    // TRIGGER GOLD DISK RESTORE
                    log::error!("Max attempts reached, triggering gold disk restore");
                    
                    trigger_gold_restore(prompt, gold_manifest).await?;
                    
                    return Err(FactoryError::MaxRetriesExceeded {
                        last_error: Box::new(e),
                        restored_to_gold: true,
                    });
                }
            }
        }
    }
}

async fn trigger_gold_restore(
    prompt: &CanonicalPrompt,
    gold_manifest: &GoldManifest,
) -> Result<(), RestoreError> {
    // 1. Notify
    slack_notify(
        "#sx9-alerts",
        &format!(
            "🚨 GOLD DISK RESTORE TRIGGERED\n\n\
             Crate: {}\n\
             Reason: Build/QA failure after max retries\n\n\
             Restoring to gold baseline...",
            prompt.crate_name
        ),
    ).await?;
    
    // 2. Snapshot failed state
    let snapshot_id = create_forensic_snapshot(&prompt.workspace).await?;
    log::info!("Forensic snapshot: {}", snapshot_id);
    
    // 3. Restore gold version if exists
    if let Some(gold_version) = gold_manifest.get_crate(&prompt.crate_name) {
        restore_from_gold(&prompt.crate_name, &gold_version).await?;
    } else {
        // No gold version exists - this is a new crate
        // Clean the workspace and mark for human review
        clean_workspace(&prompt.workspace).await?;
        
        slack_notify(
            "#sx9-decisions",
            &format!(
                "⚠️ New crate {} failed build with no gold baseline.\n\n\
                 Forensic snapshot: {}\n\n\
                 Human review required.",
                prompt.crate_name,
                snapshot_id
            ),
        ).await?;
    }
    
    // 4. Log incident
    create_linear_incident(
        &format!("Gold restore triggered: {}", prompt.crate_name),
        &format!(
            "Build/QA failure after max retries.\n\n\
             Forensic snapshot: {}\n\
             Restored to: {:?}",
            snapshot_id,
            gold_manifest.get_crate(&prompt.crate_name)
        ),
    ).await?;
    
    Ok(())
}
```

---

## 9. Implementation Checklist

### Phase 1: Gold Registry (Week 1)

- [ ] Gold manifest schema
- [ ] Gold certification criteria
- [ ] Docker image tagging convention
- [ ] Registry storage (container registry)

### Phase 2: Docker Sheets (Week 2)

- [ ] Base image (distroless)
- [ ] Runtime images (Rust, Python)
- [ ] Component Dockerfiles (all crates)
- [ ] Gold Disk Compose file

### Phase 3: Canary Detection (Week 3)

- [ ] Fake code pattern scanner (Rust)
- [ ] Fake code pattern scanner (Python)
- [ ] Simplification detector
- [ ] Feature removal detector
- [ ] CI integration

### Phase 4: Restore Protocol (Week 4)

- [ ] sx9-gold CLI tool
- [ ] Full system restore
- [ ] Single crate restore
- [ ] Code restore (git)
- [ ] Verification suite

### Phase 5: Certification Pipeline (Week 5)

- [ ] Gold certification workflow
- [ ] Soak test framework
- [ ] Performance baseline tracking
- [ ] Manifest auto-update

---

## 10. References

- RFC-9120: Prompt Forge v4
- RFC-9121: Lightning QA Engine
- RFC-9122: Git Workflow
- Google Distroless Images
- Docker Multi-stage Builds Best Practices

---

## Appendix A: Fake Code Pattern Reference

| Pattern | Language | Detection | Severity |
|---------|----------|-----------|----------|
| `todo!()` | Rust | Regex | Critical |
| `unimplemented!()` | Rust | Regex | Critical |
| `panic!("not impl")` | Rust | Regex | Critical |
| `Default::default()` return | Rust | AST | High |
| `pass` as function body | Python | AST | Critical |
| `raise NotImplementedError` | Python | AST | Critical |
| `...` as function body | Python | AST | Critical |
| Empty `except`/`catch` | Both | AST | High |
| Hardcoded test data | Both | Heuristic | Medium |
| 50%+ code reduction | Both | Diff | Critical |
| Public API removal | Both | Diff | Critical |
| Test deletion | Both | Diff | Critical |

---

## Appendix B: Gold Disk Quick Reference

```bash
# Full system restore
sx9-gold restore --all

# Single crate restore
sx9-gold restore --crate sx9-nats-router --version 1.1.0

# Code restore from gold tag
sx9-gold restore --code sx9-nats-router

# Verify system
sx9-gold verify

# View gold manifest
cat gold-disk-manifest.toml

# Start gold disk compose
docker compose -f gold-disk-compose.yml up -d

# Check gold certification status
sx9-gold status sx9-core
```

---

## Appendix C: Emergency Contacts

```yaml
# Escalation for gold disk failures

escalation:
  level_1:
    trigger: "Automated restore successful"
    action: "Log incident, continue"
    notify: "#sx9-factory"
  
  level_2:
    trigger: "Automated restore failed"
    action: "Page on-call"
    notify: "#sx9-alerts @oncall"
  
  level_3:
    trigger: "System unrecoverable"
    action: "All hands"
    notify: "#sx9-alerts @channel"
```

---

*End of RFC-9123*
