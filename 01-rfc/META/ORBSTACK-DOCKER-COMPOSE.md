# CTAS 7.3.1 / SX9 - OrbStack Docker Compose

**Recovered:** 2025-12-24  
**Status:** From conversation history  

---

## Database Stack (Post-SurrealDB Deprecation)

| Database | Purpose | Port |
|----------|---------|------|
| PostgreSQL (pgvector) | Supabase - ACID transactions | 5432 |
| Redis | Sledis stand-in | 6379 |
| Neo4j | Graph operations (SlotGraph) | 7474/7687 |
| ChromaDB | Vector embeddings | 8000 |

---

## docker-compose.yml

```yaml
# SX9 (SYNAPTIX9) - RFC-9005 COMPLIANT BACKEND STACK
# Save as: ~/Developer/ctas-container/docker-compose.yml

version: '3.8'

services:
  # ============================================
  # SUPABASE (PostgreSQL 15 + pgvector)
  # ============================================
  supabase:
    image: pgvector/pgvector:pg15
    container_name: sx9-supabase
    ports:
      - "5432:5432"
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: sx9_local_dev
      POSTGRES_DB: sx9
    volumes:
      - supabase_data:/var/lib/postgresql/data
      - ./init/supabase:/docker-entrypoint-initdb.d:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 3s
      retries: 3
    networks:
      - sx9-network

  # ============================================
  # REDIS (Sledis stand-in)
  # ============================================
  redis:
    image: redis:7-alpine
    container_name: sx9-sledis
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes --maxmemory 512mb --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5
    networks:
      - sx9-network

  # ============================================
  # NEO4J (SlotGraph)
  # ============================================
  neo4j:
    image: neo4j:5-community
    container_name: sx9-neo4j
    ports:
      - "7474:7474"  # HTTP Browser
      - "7687:7687"  # Bolt Protocol
    environment:
      NEO4J_AUTH: neo4j/sx9_local_dev
      NEO4J_PLUGINS: '["apoc", "graph-data-science"]'
      NEO4J_dbms_memory_heap_initial__size: 512m
      NEO4J_dbms_memory_heap_max__size: 1G
    volumes:
      - neo4j_data:/data
      - neo4j_logs:/logs
    healthcheck:
      test: ["CMD", "cypher-shell", "-u", "neo4j", "-p", "sx9_local_dev", "RETURN 1"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - sx9-network

  # ============================================
  # CHROMADB (Vector Database)
  # ============================================
  chromadb:
    image: chromadb/chroma:latest
    container_name: sx9-chromadb
    ports:
      - "8000:8000"
    environment:
      IS_PERSISTENT: "TRUE"
      ANONYMIZED_TELEMETRY: "FALSE"
    volumes:
      - chromadb_data:/chroma/chroma
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/api/v1/heartbeat"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - sx9-network

  # ============================================
  # UNIFIED DB API (Bun)
  # ============================================
  db-api:
    build: ./services/db-api
    container_name: sx9-db-api
    ports:
      - "18889:18889"
    environment:
      SUPABASE_URL: postgresql://postgres:sx9_local_dev@supabase:5432/sx9
      REDIS_URL: redis://redis:6379
      NEO4J_URL: bolt://neo4j:7687
      NEO4J_USER: neo4j
      NEO4J_PASS: sx9_local_dev
      CHROMA_URL: http://chromadb:8000
    depends_on:
      supabase:
        condition: service_healthy
      redis:
        condition: service_healthy
      neo4j:
        condition: service_healthy
      chromadb:
        condition: service_healthy
    networks:
      - sx9-network

  # ============================================
  # HEALTH DASHBOARD
  # ============================================
  health:
    build: ./services/health-dashboard
    container_name: sx9-health
    ports:
      - "18888:18888"
    environment:
      DB_API_URL: http://db-api:18889
    depends_on:
      - db-api
    networks:
      - sx9-network

volumes:
  supabase_data:
    name: sx9-supabase-data
  redis_data:
    name: sx9-redis-data
  neo4j_data:
    name: sx9-neo4j-data
  neo4j_logs:
    name: sx9-neo4j-logs
  chromadb_data:
    name: sx9-chromadb-data

networks:
  sx9-network:
    driver: bridge
    name: sx9-network
```

---

## DB API Dockerfile (Bun)

```dockerfile
# services/db-api/Dockerfile
FROM oven/bun:1-alpine
WORKDIR /app
COPY package.json bun.lockb* ./
RUN bun install --frozen-lockfile
COPY .
EXPOSE 18889
CMD ["bun", "run", "src/index.ts"]
```

---

## QA Analyzer Container

```dockerfile
# qa-analyzer/Dockerfile
FROM rust:1.75-slim

RUN apt-get update && apt-get install -y \
    python3 python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-clippy cargo-audit

WORKDIR /workspace

COPY qa-config.yml /etc/qa-config.yml

ENTRYPOINT ["cargo", "clippy", "--message-format=json"]
```

---

## Quick Start

```bash
# Install OrbStack
brew install orbstack

# Create directory structure
mkdir -p ~/Developer/ctas-container/{init/supabase,services/{db-api,health-dashboard},qa-analyzer}

# Copy schema
cp /path/to/SUPABASE-RFC9005-SCHEMA.sql ~/Developer/ctas-container/init/supabase/01-schema.sql

# Start stack
cd ~/Developer/ctas-container
docker-compose up -d

# Verify
docker-compose ps
```

---

## Port Assignments

| Service | Port | Purpose |
|---------|------|---------|
| Supabase | 5432 | PostgreSQL |
| Redis/Sledis | 6379 | Cache |
| Neo4j HTTP | 7474 | Browser UI |
| Neo4j Bolt | 7687 | Driver protocol |
| ChromaDB | 8000 | Vector API |
| DB API | 18889 | Unified access |
| Health | 18888 | Dashboard |

---

## Environment Variables (.env)

```bash
# Cloud databases (external)
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-supabase-anon-key
NEON_DATABASE_URL=postgres://user:pass@host.neon.tech/db

# Local defaults
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=sx9_local_dev
SLEDIS_HOST=localhost
SLEDIS_PORT=6379
CHROMADB_HOST=localhost
CHROMADB_PORT=8000
```

---

**Document Status:** RECOVERED  
**Recovery Date:** 2025-12-24
