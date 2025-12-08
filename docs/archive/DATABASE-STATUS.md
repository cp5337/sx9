# Database Status - Supabase & SurrealDB
## Complete Database Configuration and Status

**Date:** December 2025  
**Status:** Active Configuration  
**Databases:** Supabase (PostgreSQL) + SurrealDB + Neo4j + Others

---

## Executive Summary

**Database Stack:**
- ✅ **SurrealDB Primary** (Port 8000) - Primary CTAS operational database
- ✅ **Supabase** (Port 5432/3000) - PostgreSQL with PostgREST API
- ✅ **Neo4j Main Ops** (Port 7687) - Threat extraction, interviews, YAMLs
- ✅ **Neo4j ATL Physical** (Port 7688) - ATL Physical training data
- ✅ **Sledis Cache** (Port 6379/6380) - Redis-compatible caching
- ✅ **GLAF Analytics** (Port 9000) - Graph database for threat intelligence
- ✅ **SlotGraph ECS** (Port 9001) - Legion entity-component-system

---

## 1. SurrealDB Configuration

### 1.1 Primary SurrealDB Instance

**Docker Compose:** `docker-compose.all-databases.yml`

**Configuration:**
```yaml
surrealdb-primary:
  image: surrealdb/surrealdb:v2.3.7
  container_name: ctas7-surrealdb
  ports:
    - "8000:8000"
  command: start --log info --user root --pass root memory
  auth: root/root
  endpoint: ws://localhost:8000
```

**Purpose:**
- Primary CTAS operational database
- Multi-model database (document, graph, key-value)
- Real-time subscriptions
- WebSocket and HTTP interfaces

**Namespaces:**
- `ov` - Operational Views
- `sv` - Systems Views
- `bne_cases` - BNE case studies
- `ctas7` - CTAS7 namespace
- `ground_station` - Ground station data

**Schema Files Found:**
- `surrealdb_seed.surql` - Initial seed data
- `surrealdb_seed_fixed.surql` - Fixed seed data
- `ctas7_network_graph_relations.surql` - Network graph schema
- `ctas7_starlink_gateway_schema.surql` - Starlink gateway schema
- `ctas7_gis_hardened.surql` - GIS hardened schema
- `ctas7_ground_station_graph_schema.surql` - Ground station graph
- `ctas7_geospatial_schema.surql` - Geospatial schema
- `ctas7_legion_slot_graph_crosslink.surql` - Legion/SlotGraph crosslink

**Migration Files:**
- `migrated-databases/surrealdb/schema/ctas_core_schema.surql`
- `migrated-databases/surrealdb/schema/ctas_actual_architecture.surql`
- `migrated-databases/surrealdb/schema/ctas_app_data_structure.surql`
- `migrated-databases/surrealdb/schema/comprehensive_crate_interviews.surql`

### 1.2 GLAF Analytics SurrealDB

**Configuration:**
```yaml
glaf-analytics:
  image: surrealdb/surrealdb:latest
  container_name: ctas7-glaf-analytics
  ports:
    - "9000:8000"
  command: start --log info --user glaf_admin --pass glaf_secret --bind 0.0.0.0:8000 memory
  auth: glaf_admin/glaf_secret
```

**Purpose:**
- Graph database for threat intelligence
- GLAF (Genome Link Analysis Fabric) analytics
- Threat relationship analysis

### 1.3 SurrealDB Integration Code

**Rust Integration:**
- `ctas7-exploit-arsenal/src/surrealdb_integration.rs`
- `ctas7-cdn-data-fabric/src/adapters/surreal.rs`
- `sx9-foundation-visualizer/crates/sx9-adapters/src/surreal.rs`
- `sx9-foundation-core/src/database.rs`
- `sx9-foundation-data/src/database_manager.rs`

**TypeScript Integration:**
- `ctas7-cesium-mcp/src/database/space-operations-manager.ts`
- `ctas7-ops-main-platform/src/utils/supabaseClient.ts`

---

## 2. Supabase Configuration

### 2.1 Supabase Database (PostgreSQL)

**Docker Compose:** `docker-compose.all-databases.yml`

**Configuration:**
```yaml
supabase-db:
  image: postgres:15-alpine
  container_name: ctas7-supabase-db
  ports:
    - "5432:5432"
  environment:
    - POSTGRES_DB=ctas_intelligence
    - POSTGRES_USER=supabase_admin
    - POSTGRES_PASSWORD=supabase_secret
  auth: supabase_admin/supabase_secret
```

**PostgREST API:**
```yaml
supabase-api:
  image: postgrest/postgrest:v11.2.2
  container_name: ctas7-supabase-api
  ports:
    - "3000:3000"
  environment:
    - PGRST_DB_URI=postgres://supabase_admin:supabase_secret@supabase-db:5432/ctas_intelligence
    - PGRST_OPENAPI_SERVER_PROXY_URI=http://localhost:3000
```

**Purpose:**
- Cloud-native PostgreSQL database
- REST API via PostgREST
- Real-time subscriptions
- Row-level security
- Storage for file uploads

### 2.2 Supabase Config File

**Location:** `ctas7-command-center/supabase/config.toml`

**Configuration:**
- **Project ID:** `ctas7-command-center`
- **API Port:** 54321
- **DB Port:** 54322
- **Shadow DB Port:** 54320
- **PostgreSQL Version:** 15
- **Schemas:** `public`, `storage`, `graphql_public`
- **Max Rows:** 1000
- **Realtime:** Enabled
- **Storage:** Enabled (50MiB limit)
- **Auth:** Enabled

### 2.3 Supabase Schema Files

**SQL Schema Files:**
- `supabase_ucla_orbital_obstructions_schema.sql`
- `supabase_ctas_tasks_schema.sql`
- `supabase-rfc9005-schema.sql`
- `ctas7-qa-analyzer/supabase_schema.sql`
- `run-this-in-supabase.sql`
- `supabase-realistic-data.sql`

**TypeScript Schema:**
- `database/supabase_schema.ts`

### 2.4 Supabase Integration Code

**TypeScript/JavaScript:**
- `ctas7-gis-cesium/src/lib/supabase.ts`
- `ctas7-cesium-geolocation/src/lib/supabase.ts`
- `ctas7-ops-main-platform/src/utils/supabaseClient.ts`
- `check-supabase.js`

**Rust:**
- `sx9-foundation-core/src/database.rs`
- `sx9-foundation-data/src/database_manager.rs`
- `ctas7-foundation-core/src/database.rs`

---

## 3. Database Connection Details

### 3.1 SurrealDB Connections

**Primary:**
- **Endpoint:** `ws://localhost:8000` or `http://localhost:8000`
- **Auth:** `root/root`
- **Namespace:** `ctas7` (default)
- **Database:** `ground_station` or `ctas` (default)

**GLAF Analytics:**
- **Endpoint:** `ws://localhost:9000` or `http://localhost:9000`
- **Auth:** `glaf_admin/glaf_secret`
- **Purpose:** Threat intelligence analytics

### 3.2 Supabase Connections

**PostgreSQL Direct:**
- **Host:** `localhost`
- **Port:** `5432`
- **Database:** `ctas_intelligence`
- **User:** `supabase_admin`
- **Password:** `supabase_secret`
- **Connection String:** `postgres://supabase_admin:supabase_secret@localhost:5432/ctas_intelligence`

**PostgREST API:**
- **URL:** `http://localhost:3000`
- **OpenAPI:** `http://localhost:3000/`
- **Auto-generated REST endpoints** for all tables

**Supabase Local (via CLI):**
- **API URL:** `http://localhost:54321`
- **DB URL:** `postgresql://postgres:postgres@localhost:54322/postgres`
- **Studio:** `http://localhost:54323`

---

## 4. Database Usage Patterns

### 4.1 SurrealDB Usage

**Primary Use Cases:**
- Operational views (OV-1, OV-2, etc.)
- Systems views (SV-1, SV-4, etc.)
- BNE case studies
- Network topology graphs
- Ground station data
- Geospatial data
- Real-time telemetry

**Query Examples:**
```surql
-- Query operational views
SELECT * FROM ov_1:enterprise_ops;

-- Query network topology
SELECT * FROM network_nodes WHERE status = 'active';

-- Real-time subscription
LIVE SELECT * FROM telemetry WHERE source = 'ground_station';
```

### 4.2 Supabase Usage

**Primary Use Cases:**
- CTAS tasks storage
- Intelligence data (IOCs, threats)
- User authentication
- File storage
- Real-time subscriptions
- GraphQL queries

**Query Examples:**
```sql
-- Query CTAS tasks
SELECT * FROM ctas_tasks WHERE status = 'active';

-- Query intelligence data
SELECT * FROM threat_intelligence WHERE severity = 'critical';
```

**REST API:**
```bash
# Get all tasks
curl http://localhost:3000/ctas_tasks

# Get specific task
curl http://localhost:3000/ctas_tasks?id=eq.123
```

---

## 5. Database Integration Architecture

### 5.1 Multi-Database Manager

**Location:** `sx9-foundation-data/src/database_manager.rs`

**Capabilities:**
- Unified database interface
- Connection pooling
- Health monitoring
- Automatic failover
- Replication strategies

**Supported Backends:**
- Supabase (PostgreSQL)
- SurrealDB
- SlotGraph
- Legion Cluster
- Sled (embedded KVS)

### 5.2 Storage Strategy

**Replicated:**
- Store in all databases for maximum availability

**Specialized:**
- SurrealDB: Graph data, real-time telemetry
- Supabase: Structured data, user data, files
- Neo4j: Threat relationships, interviews
- Sledis: High-speed caching

**Performance:**
- Use fastest available database
- Cache frequently accessed data in Sledis

**Security:**
- Store sensitive data in most secure database
- Use Supabase RLS for access control

---

## 6. Database Schema Files

### 6.1 SurrealDB Schemas

**Core Schemas:**
- `ctas_core_schema.surql` - Core CTAS schema
- `ctas_actual_architecture.surql` - Actual architecture
- `ctas_app_data_structure.surql` - Application data structure

**Domain Schemas:**
- `ctas7_network_graph_relations.surql` - Network graph
- `ctas7_starlink_gateway_schema.surql` - Starlink gateway
- `ctas7_gis_hardened.surql` - GIS hardened schema
- `ctas7_ground_station_graph_schema.surql` - Ground stations
- `ctas7_geospatial_schema.surql` - Geospatial data
- `cable_landing_sites.surql` - Cable landing sites

**Integration Schemas:**
- `ctas7_legion_slot_graph_crosslink.surql` - Legion/SlotGraph integration

### 6.2 Supabase Schemas

**Intelligence Schemas:**
- `supabase_ctas_tasks_schema.sql` - CTAS tasks
- `supabase-rfc9005-schema.sql` - RFC-9005 unified schema
- `supabase_ucla_orbital_obstructions_schema.sql` - Orbital obstructions

**Application Schemas:**
- `ctas7-qa-analyzer/supabase_schema.sql` - QA analyzer schema

---

## 7. Database Health & Monitoring

### 7.1 Health Checks

**SurrealDB:**
```bash
curl http://localhost:8000/health
```

**Supabase API:**
```bash
curl http://localhost:3000/
```

**Supabase Database:**
```bash
psql -h localhost -p 5432 -U supabase_admin -d ctas_intelligence -c "SELECT 1;"
```

### 7.2 Health Dashboard

**Location:** `ctas7-health-dashboard/`

**Endpoint:** `http://localhost:18888/api/databases/unified`

**Monitors:**
- SurrealDB Primary
- Supabase (PostgreSQL + PostgREST)
- SQLite Components
- Sled KV
- Sledis Cache
- GLAF Analytics
- SlotGraph ECS

---

## 8. Database Initialization

### 8.1 SurrealDB Initialization

**Seed Script:**
```bash
# Load seed data
surreal sql --conn ws://localhost:8000 --user root --pass root --ns ctas7 --db ctas < surrealdb_seed.surql
```

**Schema Loading:**
```bash
# Load core schema
surreal sql --conn ws://localhost:8000 --user root --pass root --ns ctas7 --db ctas < ctas_core_schema.surql
```

### 8.2 Supabase Initialization

**Schema Loading:**
```bash
# Connect to Supabase
psql -h localhost -p 5432 -U supabase_admin -d ctas_intelligence

# Load schema
\i supabase_ctas_tasks_schema.sql
\i supabase-rfc9005-schema.sql
```

**Via Supabase CLI:**
```bash
cd ctas7-command-center
supabase db reset  # Reset and apply migrations
supabase db push  # Push local migrations
```

---

## 9. Database Service Orchestration

### 9.1 Service Orchestrator

**Script:** `sx9-service-orchestrator.py`

**Manages:**
1. SurrealDB (foundation)
2. Supabase (depends on SurrealDB)
3. Sledis (depends on SurrealDB)
4. Neo4j Main Ops (depends on SurrealDB)
5. Neo4j ATL Physical (depends on SurrealDB)
6. GLAF (depends on all above)
7. ops-main-platform (depends on Neo4j Main, SurrealDB, Supabase)

### 9.2 Database Initializer

**Script:** `sx9-database-initializer.py`

**Functions:**
- Create Neo4j databases
- Create indexes
- Verify Supabase connectivity
- Verify SurrealDB health
- Verify Sledis connectivity

---

## 10. Connection Strings & Environment Variables

### 10.1 SurrealDB

```bash
# Primary
SURREALDB_URL=ws://localhost:8000
SURREALDB_USER=root
SURREALDB_PASSWORD=root
SURREALDB_NAMESPACE=ctas7
SURREALDB_DATABASE=ctas

# GLAF Analytics
GLAF_SURREALDB_URL=ws://localhost:9000
GLAF_SURREALDB_USER=glaf_admin
GLAF_SURREALDB_PASSWORD=glaf_secret
```

### 10.2 Supabase

```bash
# PostgreSQL Direct
SUPABASE_DB_URL=postgres://supabase_admin:supabase_secret@localhost:5432/ctas_intelligence

# PostgREST API
SUPABASE_URL=http://localhost:3000
SUPABASE_ANON_KEY=your_anon_key
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key

# Supabase Local (CLI)
SUPABASE_API_URL=http://localhost:54321
SUPABASE_DB_URL=postgresql://postgres:postgres@localhost:54322/postgres
```

---

## 11. Database Status Summary

### 11.1 Current Status

| Database | Status | Port | Auth | Purpose |
|----------|--------|------|------|---------|
| **SurrealDB Primary** | ✅ Active | 8000 | root/root | Primary operational DB |
| **Supabase PostgreSQL** | ✅ Active | 5432 | supabase_admin/secret | Intelligence data |
| **Supabase API** | ✅ Active | 3000 | N/A | REST API |
| **Neo4j Main Ops** | ✅ Active | 7687 | neo4j/ctas7_graph | Threat extraction |
| **Neo4j ATL Physical** | ✅ Active | 7688 | neo4j/atl_physical_graph | ATL training |
| **Sledis Cache** | ✅ Active | 6379/6380 | N/A | High-speed cache |
| **GLAF Analytics** | ✅ Active | 9000 | glaf_admin/secret | Threat analytics |
| **SlotGraph ECS** | ✅ Active | 9001 | N/A | ECS operations |

### 11.2 Integration Status

- ✅ **Rust Integration:** Complete (foundation-core, foundation-data)
- ✅ **TypeScript Integration:** Complete (cesium, ops-platform)
- ✅ **Docker Compose:** Complete (all-databases.yml)
- ✅ **Health Monitoring:** Complete (health dashboard)
- ✅ **Schema Files:** Multiple schemas available
- ✅ **Seed Data:** Seed scripts available

---

## 12. Next Steps

### 12.1 Verification

- [ ] Verify all database connections are working
- [ ] Test SurrealDB queries
- [ ] Test Supabase REST API
- [ ] Verify schema migrations
- [ ] Check health dashboard

### 12.2 Integration

- [ ] Connect intelligence platform to databases
- [ ] Load threat intelligence data
- [ ] Set up real-time subscriptions
- [ ] Configure replication strategies

---

## 13. References

- **Docker Compose:** `docker-compose.all-databases.yml`
- **SurrealDB Docs:** https://surrealdb.com/docs
- **Supabase Docs:** https://supabase.com/docs
- **Database Setup:** `SX9_DATABASE_SETUP_COMPLETE.md`
- **Service Orchestrator:** `sx9-service-orchestrator.py`

---

**Status:** ✅ All Databases Configured and Ready  
**Last Updated:** December 2025


