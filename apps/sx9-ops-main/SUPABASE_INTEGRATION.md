# SX9 Ops-Main Supabase Integration

**Version:** 1.0  
**Date:** December 14, 2025  
**Status:** Active

## RFC Compliance

This integration implements the following RFCs:

- **RFC-9001:** Trivariate Hashing Standard (Murmur3-64, Base96)
- **RFC-9005:** Unified Schema Specification (Supabase as primary store)
- **RFC-9101:** Smart Crate System (Crate grouping and orchestration)
- **RFC-9114:** SX9 Gateway Neural Retrofit (Modal inventory integration)
- **RFC-9115:** SX9 Frontend Adapter Standard (Component mapping)

---

## Overview

This directory contains the complete Supabase integration for SX9 Ops-Main, including:

1. **Modal Inventory** - UI elements with trivariate hashes from Playwright scans
2. **DSL Orchestration** - Crate grouping and operational intelligence mapping
3. **Migration Scripts** - Automated data migration from JSON/XML to Supabase
4. **Schema Definition** - Complete PostgreSQL schema with RLS policies

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SX9 OPS-MAIN                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Playwright Scanner                                              │
│  ├─ Scans UI pages                                              │
│  ├─ Generates trivariate hashes (RFC-9001)                      │
│  └─ Outputs: inventory.json                                     │
│                                                                  │
│  DSL Orchestration                                               │
│  ├─ Defines crate groups (RFC-9101)                             │
│  ├─ Maps operational capabilities                               │
│  └─ Outputs: dsl-crate-grouping-system.dsl                      │
│                                                                  │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    MIGRATION SCRIPTS                            │
├─────────────────────────────────────────────────────────────────┤
│  migrate-inventory-to-supabase.js                               │
│  migrate-dsl-to-supabase.js                                     │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SUPABASE (RFC-9005)                          │
├─────────────────────────────────────────────────────────────────┤
│  Tables:                                                         │
│  ├─ ui_pages (17 pages)                                         │
│  ├─ ui_buttons (110+ buttons)                                   │
│  ├─ ui_modals                                                   │
│  ├─ ui_forms                                                    │
│  ├─ ui_screenshots                                              │
│  ├─ crate_groups (7 groups)                                     │
│  ├─ crates (96+ crates)                                         │
│  ├─ operational_capabilities                                    │
│  ├─ component_mappings                                          │
│  └─ crate_relationships                                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Files

### Schema

- **`supabase_schema.sql`** - Complete PostgreSQL schema
  - 10 tables with RFC-9001 trivariate hash support
  - Indexes for performance
  - RLS policies for security
  - Views for common queries
  - Triggers for timestamp management

### Migration Scripts

- **`scripts/migrate-inventory-to-supabase.js`** - Modal inventory migration
  - Migrates UI pages, buttons, modals, forms, screenshots
  - Validates trivariate hashes (RFC-9001)
  - Generates Unicode shortcuts (RFC-9002)
- **`scripts/migrate-dsl-to-supabase.js`** - DSL orchestration migration
  - Migrates crate groups, crates, capabilities
  - Parses XML DSL format
  - Maps frontend components

### Source Data

- **`modal-inventory-foundation/inventory.json`** - Playwright scan results
  - 17 pages with trivariate hashes
  - 110+ buttons
  - Screenshots

- **`DSL-orchestration/dsl-crate-grouping-system.dsl`** - Crate grouping XML
  - 7 crate groups (Foundation, Intelligence, Operations, etc.)
  - 96+ crates
  - Operational capability mappings

- **`playwright-connection-results/connection-test-results.json`** - Connection tests
  - API connectivity validation
  - Component detection

---

## Setup

### 1. Install Dependencies

```bash
cd /Users/cp5337/Developer/sx9/apps/sx9-ops-main

# Install Node.js dependencies
npm install @supabase/supabase-js xml2js
```

### 2. Configure Environment

Create `.env` file:

```bash
# Supabase Configuration
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-service-role-key
```

### 3. Deploy Schema

```bash
# Connect to Supabase and run schema
psql $DATABASE_URL < supabase_schema.sql

# Or use Supabase CLI
supabase db push
```

### 4. Run Migrations

```bash
# Migrate modal inventory
node scripts/migrate-inventory-to-supabase.js

# Migrate DSL orchestration
node scripts/migrate-dsl-to-supabase.js
```

---

## Database Schema

### Modal Inventory Tables

#### `ui_pages`

Stores UI pages with trivariate hashes (RFC-9001).

| Column           | Type        | Description           |
| ---------------- | ----------- | --------------------- |
| id               | UUID        | Primary key           |
| trivariate_hash  | TEXT        | Full 48-char hash     |
| sch_hash         | TEXT        | Murmur3-64 (16 chars) |
| name             | TEXT        | Page name             |
| path             | TEXT        | URL path (unique)     |
| url              | TEXT        | Full URL              |
| unicode_shortcut | TEXT        | e.g., "🔹d1ba"        |
| usim_header      | TEXT        | USIM documentation    |
| last_scanned_at  | TIMESTAMPTZ | Last Playwright scan  |

#### `ui_buttons`

Stores buttons with page relationships.

| Column          | Type    | Description             |
| --------------- | ------- | ----------------------- |
| id              | UUID    | Primary key             |
| page_id         | UUID    | Foreign key to ui_pages |
| trivariate_hash | TEXT    | Button hash             |
| text            | TEXT    | Button text             |
| selector        | TEXT    | CSS selector            |
| enabled         | BOOLEAN | Is enabled              |
| position        | INTEGER | Order on page           |

#### `ui_modals`, `ui_forms`, `ui_screenshots`

Similar structure for modals, forms, and screenshots.

### DSL Orchestration Tables

#### `crate_groups`

Stores Smart Crate groups (RFC-9101).

| Column                   | Type  | Description                      |
| ------------------------ | ----- | -------------------------------- |
| id                       | UUID  | Primary key                      |
| group_id                 | TEXT  | Group identifier (unique)        |
| group_name               | TEXT  | Display name                     |
| group_type               | TEXT  | Type (core_infrastructure, etc.) |
| criteria                 | JSONB | Classification criteria          |
| operational_capabilities | JSONB | Capabilities array               |

#### `crates`

Stores individual crates.

| Column              | Type    | Description                 |
| ------------------- | ------- | --------------------------- |
| id                  | UUID    | Primary key                 |
| crate_name          | TEXT    | Crate name (unique)         |
| group_id            | UUID    | Foreign key to crate_groups |
| trivariate_hash     | TEXT    | Crate hash (optional)       |
| port                | INTEGER | Allocated port              |
| smart_crate_version | TEXT    | Version (e.g., "1.2.0")     |
| tesla_grade         | BOOLEAN | Tesla-grade certification   |

#### `operational_capabilities`

Maps operational intelligence capabilities.

| Column          | Type  | Description              |
| --------------- | ----- | ------------------------ |
| id              | UUID  | Primary key              |
| capability_name | TEXT  | Capability name (unique) |
| capability_type | TEXT  | Type                     |
| description     | TEXT  | Description              |
| group_mapping   | JSONB | Array of group_ids       |
| crate_mapping   | TEXT  | Crate mapping logic      |
| assessment      | TEXT  | Assessment criteria      |

#### `component_mappings`

Maps frontend components to crate groups (RFC-9115).

| Column                | Type    | Description                 |
| --------------------- | ------- | --------------------------- |
| id                    | UUID    | Primary key                 |
| mapping_name          | TEXT    | Mapping name (unique)       |
| group_id              | UUID    | Foreign key to crate_groups |
| ui_component          | TEXT    | React component name        |
| dashboard_integration | TEXT    | Dashboard type              |
| real_time_data        | BOOLEAN | Real-time updates           |

---

## Views

### `v_page_inventory`

Complete page inventory with counts.

```sql
SELECT * FROM v_page_inventory;
```

Returns: Page details + button_count, modal_count, form_count, screenshot_count

### `v_crate_group_summary`

Crate group summary with crate lists.

```sql
SELECT * FROM v_crate_group_summary;
```

Returns: Group details + crate_count, crates array

### `v_operational_intelligence_map`

Operational capability mapping.

```sql
SELECT * FROM v_operational_intelligence_map;
```

Returns: Capability details + component_mapping_count

---

## Usage Examples

### Query Pages by Unicode Shortcut

```sql
-- Find page by Unicode shortcut (RFC-9002)
SELECT * FROM ui_pages WHERE unicode_shortcut = '🔹d1ba';
```

### Query Crates by Group

```sql
-- Get all Foundation crates
SELECT c.*
FROM crates c
JOIN crate_groups cg ON c.group_id = cg.id
WHERE cg.group_id = 'foundation';
```

### Query Operational Capabilities

```sql
-- Get threat emulation capabilities
SELECT * FROM operational_capabilities
WHERE capability_type = 'threat_emulation_capability';
```

### Insert New Page (with trivariate hash)

```sql
INSERT INTO ui_pages (
  name, path, url, trivariate_hash, sch_hash, unicode_shortcut
) VALUES (
  'New Page',
  '/new-page',
  'http://localhost:18601/new-page',
  'abc123def456ghi789jkl012mno345pqr678stu901vwx234',
  'abc123def456ghi7',
  '🔹abc1'
);
```

---

## RFC Compliance Details

### RFC-9001: Trivariate Hashing

All UI elements and crates support trivariate hashing:

- **Format:** `[SCH]_[CUID]_[UUID]` (48 characters)
- **Algorithm:** Murmur3-64
- **Encoding:** Base96
- **Storage:** `trivariate_hash` (full), `sch_hash` (first 16 chars)

### RFC-9002: Unicode Routing

Unicode shortcuts for voice navigation:

- **Format:** `🔹{first 4 chars of SCH}`
- **Example:** `🔹d1ba` for Hunt Phase
- **Storage:** `unicode_shortcut` column

### RFC-9005: Unified Schema

Supabase as primary data store:

- **Primary:** PostgreSQL (Supabase)
- **Secondary:** SurrealDB (graph relationships)
- **Tertiary:** Sled KVS (local cache)

### RFC-9101: Smart Crate System

Crate grouping and classification:

- **Groups:** Foundation, Intelligence, Operations, Specialized, Testing, AI-CLI, Tools, Infrastructure
- **Criteria:** JSONB array of classification criteria
- **Capabilities:** JSONB array of operational capabilities

### RFC-9114: Gateway Integration

Modal inventory for gateway UI:

- **Pages:** 17 HD4 phase pages + operational pages
- **Buttons:** 110+ interactive elements
- **Screenshots:** Playwright-captured UI states

### RFC-9115: Frontend Adapter

Component mapping for vertical frontends:

- **Dashboards:** Foundation, Intelligence, Operations, Specialized, Testing
- **Real-time:** WebSocket integration flags
- **UI Components:** React component names

---

## Maintenance

### Re-scan UI

```bash
# Run Playwright scanner (from modal-inventory-foundation)
npm run scan

# Migrate updated data
node scripts/migrate-inventory-to-supabase.js
```

### Update DSL

```bash
# Edit DSL file
vim DSL-orchestration/dsl-crate-grouping-system.dsl

# Migrate updated data
node scripts/migrate-dsl-to-supabase.js
```

### Backup Database

```bash
# Export schema + data
pg_dump $DATABASE_URL > backup-$(date +%Y%m%d).sql

# Or use Supabase CLI
supabase db dump > backup.sql
```

---

## Troubleshooting

### Migration Fails

```bash
# Check Supabase connection
node -e "console.log(process.env.SUPABASE_URL)"

# Test connection
psql $DATABASE_URL -c "SELECT 1"
```

### Duplicate Key Errors

Migrations use `upsert` with `onConflict`, so duplicates are handled automatically. If you see errors, check:

- `ui_pages.path` must be unique
- `crate_groups.group_id` must be unique
- `crates.crate_name` must be unique

### Missing Dependencies

```bash
npm install @supabase/supabase-js xml2js
```

---

## Next Steps

1. ✅ Deploy schema to Supabase
2. ✅ Run modal inventory migration
3. ✅ Run DSL orchestration migration
4. ⏳ Integrate with SX9 Gateway (RFC-9114)
5. ⏳ Implement R2 CDN sync (RFC-9005 v1.2)
6. ⏳ Add frontend adapter queries (RFC-9115)

---

**Maintained by:** CTAS Core Engineering Group  
**Last Updated:** December 14, 2025
