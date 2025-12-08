# CTAS-7 Hashing Engine v7.3.1

**Microservice for Murmur3 trivariate hashing, Unicode compression, and USIM generation**

## Overview

The CTAS-7 Hashing Engine is a high-performance microservice that provides:

- **Trivariate Hashing:** SCH+CUID+UUID (48-char Base96)
- **Unicode Compression:** 75% size reduction for graph traversal
- **USIM Generation:** Universal Symbolic Message headers
- **Dual Hashing:** SHA-256 (integrity) + Murmur3 (addressing)
- **Multiple Formats:** Full, Footer, Index, Minimal

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  CTAS-7 Hashing Engine                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  REST API (Port 8002)                                       │
│  ├─ /health              Health check                       │
│  ├─ /hash                Single hash generation             │
│  ├─ /hash/batch          Batch processing                   │
│  ├─ /hash/stream         WebSocket streaming                │
│  ├─ /usim                USIM generation                    │
│  ├─ /usim/header         Printable headers (4 formats)      │
│  └─ /metrics             Prometheus metrics                 │
│                                                             │
│  Foundation Integration                                     │
│  └─ ctas7-foundation-core (Murmur3 trivariate)             │
│                                                             │
│  Dual Hashing System                                        │
│  ├─ SHA-256: File integrity, tamper detection              │
│  └─ Murmur3: Content addressing, graph traversal           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Build & Run

```bash
# Build
cargo build --release

# Run locally
cargo run --release

# Run in Docker
docker build -t ctas7-hashing-engine .
docker run -p 8002:8002 ctas7-hashing-engine

# Deploy in OrbStack
orb create ctas7-hashing-engine --image ctas7-hashing-engine:latest
```

### Health Check

```bash
curl http://localhost:8002/health
# {"status":"healthy","version":"7.3.1"}
```

## API Endpoints

### 1. Single Hash Generation

```bash
curl -X POST http://localhost:8002/hash \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello CTAS-7",
    "context": "test",
    "primitive_type": "message",
    "compress_unicode": true
  }'
```

**Response:**
```json
{
  "trivariate_hash": "A7x9K2mP4vQ8wR1tY5nB3cD6fG0hJ2kL7mN9pS4uV8xZ",
  "sch": "A7x9K2mP4vQ8wR1t",
  "cuid": "Y5nB3cD6fG0hJ2kL",
  "uuid": "7mN9pS4uV8xZ1234",
  "unicode_compressed": "󰄀󰄁󰄂󰄃󰄠󰄡󰄢󰄣󰄰󰄱󰄲󰄳",
  "generation_time_ms": 0.123
}
```

### 2. Batch Hash Generation

```bash
curl -X POST http://localhost:8002/hash/batch \
  -H "Content-Type: application/json" \
  -d '{
    "items": [
      {"id": "tool1", "content": "nmap", "context": "scanning", "primitive_type": "tool"},
      {"id": "tool2", "content": "rustscan", "context": "scanning", "primitive_type": "tool"}
    ],
    "compress_unicode": true,
    "preserve_context": true,
    "batch_context": "kali_tools_list"
  }'
```

**Response:**
```json
{
  "batch_hash": "B8y0L3nQ5wS9xT2uZ6oC4eE7gH1iK3lM8oP0rU5vX9zA",
  "batch_context": "kali_tools_list",
  "items": [
    {
      "id": "tool1",
      "trivariate_hash": "A7x9K2mP4vQ8wR1tY5nB3cD6fG0hJ2kL7mN9pS4uV8xZ",
      "sch": "A7x9K2mP4vQ8wR1t",
      "cuid": "Y5nB3cD6fG0hJ2kL",
      "uuid": "7mN9pS4uV8xZ1234",
      "unicode_compressed": "󰄀󰄁󰄂󰄃󰄠󰄡󰄢󰄣󰄰󰄱󰄲󰄳",
      "batch_index": 0
    }
  ],
  "total_generation_time_ms": 0.456,
  "items_per_second": 4385.96
}
```

### 3. WebSocket Streaming

```javascript
const ws = new WebSocket('ws://localhost:8002/hash/stream');

ws.onopen = () => {
  ws.send(JSON.stringify({
    content: "Real-time threat feed",
    context: "intel",
    primitive_type: "stream",
    compress_unicode: true
  }));
};

ws.onmessage = (event) => {
  const hash = JSON.parse(event.data);
  console.log('Hash:', hash.trivariate_hash);
  console.log('Unicode:', hash.unicode_compressed);
};
```

### 4. USIM Generation

```bash
curl -X POST http://localhost:8002/usim \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "src/main.rs",
    "content": "fn main() { ... }",
    "domain": "Backend Services"
  }'
```

**Response:**
```json
{
  "usim_hash": "A7x9K2mP4vQ8wR1tY5nB3cD6fG0hJ2kL7mN9pS4uV8xZ",
  "integrity_hash": "3a5f7b9d1e2c4a6f8b0d2e4f6a8c0e2f4a6b8c0d2e4f6a8b0c2d4e6f8a0b2c4d",
  "unicode_compressed": "󰄀󰄁󰄂󰄃󰄠󰄡󰄢󰄣󰄰󰄱󰄲󰄳",
  "sch": "A7x9K2mP4vQ8wR1t",
  "cuid": "Y5nB3cD6fG0hJ2kL",
  "uuid": "7mN9pS4uV8xZ1234",
  "generation_time_ms": 0.234
}
```

### 5. USIM Header Generation (4 Formats)

See [USIM_EXAMPLES.md](./USIM_EXAMPLES.md) for detailed examples.

**Formats:**
- `full` - Complete technical header
- `footer` - Legal document footer
- `index` - Catalog/registry style
- `minimal` - Compact reference

```bash
curl -X POST http://localhost:8002/usim/header \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "contracts/nda-2024.pdf",
    "content": "...",
    "domain": "Legal",
    "description": "NDA - Acme Corp",
    "format": "footer"
  }'
```

## Use Cases

### 1. Law Firms
- **Discovery Management:** Unicode indices for 50,000+ documents
- **Chain of Custody:** SHA-256 integrity verification
- **Cross-Reference:** Hash-based document relationships
- **Compliance:** Immutable audit trails

### 2. Government Agencies
- **Classified Tracking:** Non-invasive document indexing
- **Integrity Verification:** Tamper detection via SHA-256
- **Content Addressing:** Murmur3 for deduplication
- **Retention Policies:** TTL tracking in USIM headers

### 3. Research Institutions
- **Paper Archives:** Content-based retrieval
- **Duplicate Detection:** Hash-based deduplication
- **Citation Networks:** Graph traversal via Unicode
- **Knowledge Graphs:** Semantic relationship mapping

### 4. CTAS-7 Internal
- **Smart Crate Orchestration:** Hash-based execution
- **Playbook Execution:** Unicode assembly language
- **Threat Intelligence:** Real-time feed hashing
- **OSINT Processing:** Document vectorization

## Performance

- **Single Hash:** <1ms
- **Batch Processing:** 1000+ hashes/second
- **WebSocket Streaming:** Real-time, <5ms latency
- **Unicode Compression:** 75% size reduction
- **Memory:** <50MB baseline

## Integration

### With CTAS-7 Foundation
```rust
use ctas7_foundation_core::hashing::TrivariteHashEngine;

let engine = TrivariteHashEngine::new();
let hash = engine.generate_trivariate_hash(
    "content",
    "context",
    "primitive_type"
);
```

### With Multi-Tier Repository
```
Hash Engine → USIM Registry → SlotGraph → Sledis → Memory Fabric
```

### With Voice System
```
Voice Command → Playbook → Hash Lookup → Unicode Execution
```

## Deployment

### Docker
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/hash-service /usr/local/bin/
EXPOSE 8002
CMD ["hash-service"]
```

### OrbStack (Recommended)
```bash
orb create ctas7-hashing-engine \
  --image ctas7-hashing-engine:latest \
  --port 8002:8002 \
  --restart always
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ctas7-hashing-engine
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: hashing-engine
        image: ctas7-hashing-engine:7.3.1
        ports:
        - containerPort: 8002
```

## Monitoring

### Prometheus Metrics
```bash
curl http://localhost:8002/metrics
# ctas7_hash_requests_total 12345
```

### Health Checks
```bash
# Liveness
curl http://localhost:8002/health

# Readiness (check foundation integration)
curl http://localhost:8002/hash -d '{"content":"test","context":"health","primitive_type":"check"}'
```

## Development

### Prerequisites
- Rust 1.75+
- ctas7-foundation-core v7.3.1

### Build
```bash
cargo build
cargo test
cargo check
```

### Run Tests
```bash
cargo test --all-features
```

### Benchmarks
```bash
cargo bench
```

## Configuration

### Environment Variables
```bash
RUST_LOG=info                    # Logging level
HASH_ENGINE_PORT=8002            # REST API port
HASH_ENGINE_WORKERS=4            # Thread pool size
UNICODE_COMPRESSION=true         # Enable Unicode compression
```

### Cargo.toml Features
```toml
[features]
default = ["unicode-assembly"]
unicode-assembly = ["ctas7-foundation-core/unicode-assembly"]
```

## Roadmap

- [x] Trivariate hash generation
- [x] Unicode compression
- [x] USIM generation
- [x] Multi-format headers
- [x] Batch processing
- [x] WebSocket streaming
- [ ] gRPC API
- [ ] Redis caching
- [ ] Distributed hashing
- [ ] GPU acceleration

## License

CTAS-7 Proprietary License

## See Also

- [USIM_EXAMPLES.md](./USIM_EXAMPLES.md) - Detailed usage examples
- [ctas7-foundation-core](../ctas7-foundation-core/) - Core hashing implementation
- [ctas7-usim-system](../ctas7-usim-system/) - USIM registry and crawling

---

**🔖 CTAS-7 Hashing Engine v7.3.1 - Non-invasive, format-flexible, legally compliant**
