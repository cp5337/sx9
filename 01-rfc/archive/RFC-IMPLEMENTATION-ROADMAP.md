# SX9 RFC IMPLEMENTATION ROADMAP

**Version:** 1.0
**Date:** November 26, 2025
**Status:** Active
**Owner:** CTAS-7 Core Engineering Group

---

## EXECUTIVE SUMMARY

This roadmap addresses the 24 gaps identified in the RFC Continuum Audit and provides a phased implementation plan to achieve 95%+ RFC compliance across the SX9 specification series.

**Current Compliance:** 51%
**Target Compliance:** 95%
**Timeline:** 6 weeks (Dec 2 - Jan 13, 2026)

---

## PHASE 1: CRITICAL FOUNDATIONS (Week 1: Dec 2-8)

**Goal:** Fix breaking gaps that block downstream systems
**Compliance Improvement:** 51% → 68%

### TASK-001: Implement RFC-9001 Compliant CUID Generation

**Priority:** P0 - CRITICAL
**Gap:** GAP-001
**Estimate:** 2 days
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Create `src/cuid_rfc9001.rs` with proper Base96 slot mapping
- [ ] Implement exact 16-character slot allocation per RFC-9001 §6.1
- [ ] Add CUID slot extraction functions (slots 1-4, 5-7, etc.)
- [ ] Unit tests for each slot extraction
- [ ] Integration tests with trivariate hash generation

**Acceptance Criteria:**
```rust
let cuid = generate_cuid_rfc9001(&context);
assert_eq!(cuid.len(), 16);
let ts_shard = extract_timestamp_shard(&cuid); // Slots 1-4
let exec_env = extract_exec_env(&cuid);         // Slots 5-7
let agent_id = extract_agent_id(&cuid);         // Slots 8-9
```

**Files Modified:**
- `ctas7-foundation-core/src/cuid_rfc9001.rs` (new)
- `ctas7-foundation-core/src/trivariate_hashing.rs` (update)
- `ctas7-foundation-core/tests/test_cuid.rs` (new)

---

### TASK-002: Add Supersession Tracking to Schema

**Priority:** P0 - CRITICAL
**Gap:** GAP-006
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] SQL migration: Add supersession fields to entities table
- [ ] Add superseded_by/supersedes foreign keys
- [ ] Add supersession_type enum
- [ ] Add delta_angle_at_supersession field
- [ ] Create indexes for supersession queries
- [ ] Backfill existing data with NULL values

**Acceptance Criteria:**
```sql
SELECT e1.trivariate_hash, e2.trivariate_hash as superseded_by
FROM entities e1
LEFT JOIN entities e2 ON e1.superseded_by = e2.id
WHERE e1.supersession_type = 'critical';
```

**Files Modified:**
- `supabase-rfc9005-schema.sql` (migration)
- `migrations/20251202_add_supersession.sql` (new)

---

### TASK-003: Implement Gate Conditions System

**Priority:** P0 - CRITICAL
**Gap:** GAP-014
**Estimate:** 3 days
**Owner:** Security Team

**Deliverables:**
- [ ] Create `escalation_gates` table
- [ ] Define gate types (auth, handoff, resource, delta)
- [ ] Implement gate evaluation engine
- [ ] Add gate check before escalation
- [ ] Unit tests for each gate type
- [ ] Integration tests for escalation flow

**Acceptance Criteria:**
```rust
let gates = evaluate_escalation_gates(from_tier, to_tier, context);
assert!(gates.auth.passed);
assert!(gates.resource.passed);
// Escalation only proceeds if all gates pass
```

**Files Modified:**
- `ctas7-foundation-core/src/escalation/gates.rs` (new)
- `supabase-rfc9005-schema.sql` (escalation_gates table)

---

### TASK-004: Create Crosswalk Mapping Tables

**Priority:** P0 - CRITICAL
**Gap:** GAP-021
**Estimate:** 2 days
**Owner:** Threat Intel Team

**Deliverables:**
- [ ] Create `crosswalk_mappings` table
- [ ] Populate with MITRE ATT&CK mappings
- [ ] Add PTCC primitive mappings
- [ ] Add SX9 primitive mappings
- [ ] Create mapping import script
- [ ] Seed initial data (100+ mappings)

**Acceptance Criteria:**
```sql
SELECT attack_technique_id, sx9_primitive, hd4_phase
FROM crosswalk_mappings
WHERE source_family = 'nuclei'
AND source_technique_id = 'CVE-2021-44228';
```

**Files Modified:**
- `supabase-rfc9005-schema.sql` (crosswalk_mappings table)
- `scripts/seed_crosswalk_mappings.py` (new)

---

## PHASE 2: HIGH PRIORITY FIXES (Week 2: Dec 9-15)

**Goal:** Complete high-severity gaps
**Compliance Improvement:** 68% → 82%

### TASK-005: Enable UUIDv7 Support

**Priority:** P1 - HIGH
**Gap:** GAP-003
**Estimate:** 1 day
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Enable `v7` feature in uuid crate
- [ ] Replace `Uuid::new_v4()` with `Uuid::now_v7()`
- [ ] Update tests
- [ ] Verify timestamp ordering

**Files Modified:**
- `ctas7-foundation-core/Cargo.toml` (uuid feature)
- `ctas7-foundation-core/src/trivariate_hashing.rs`

---

### TASK-006: Add Delta Angle Fields to Schema

**Priority:** P1 - HIGH
**Gap:** GAP-007
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] SQL migration: Add delta angle fields
- [ ] Add delta_angle_value (DECIMAL)
- [ ] Add delta_angle_class (ENUM)
- [ ] Add delta_angle_updated_at (TIMESTAMPTZ)
- [ ] Create indexes

**Files Modified:**
- `migrations/20251209_add_delta_angle.sql` (new)

---

### TASK-007: Add Operation Classification to Schema

**Priority:** P1 - HIGH
**Gap:** GAP-012
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] SQL migration: Add operation_class field
- [ ] Add requires_approval BOOLEAN
- [ ] Add approval_tier TEXT
- [ ] Create indexes
- [ ] Backfill existing data

**Files Modified:**
- `migrations/20251210_add_operation_class.sql` (new)

---

### TASK-008: Create Playbook Schema

**Priority:** P1 - HIGH
**Gap:** GAP-019
**Estimate:** 2 days
**Owner:** Threat Intel Team

**Deliverables:**
- [ ] Create `playbooks` table
- [ ] Support atomic and composite playbooks
- [ ] Store source YAML and DSL
- [ ] Link to MITRE ATT&CK
- [ ] Add HD4 phase mapping

**Files Modified:**
- `supabase-rfc9005-schema.sql` (playbooks table)
- `migrations/20251211_create_playbooks.sql` (new)

---

### TASK-009: Create Escalation Audit Table

**Priority:** P1 - HIGH
**Gap:** GAP-023
**Estimate:** 1 day
**Owner:** Security Team

**Deliverables:**
- [ ] Create `escalation_audit` table
- [ ] Track approval workflow
- [ ] Store requested_by/approved_by
- [ ] Add approval_status enum

**Files Modified:**
- `migrations/20251212_create_escalation_audit.sql` (new)

---

### TASK-010: Implement Unicode Class Derivation

**Priority:** P1 - HIGH
**Gap:** GAP-010
**Estimate:** 1 day
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Create `unicode_class_from_codepoint()` function
- [ ] Map all 8 Unicode classes
- [ ] Add to unicode_assembly.rs
- [ ] Unit tests for each range

**Files Modified:**
- `ctas7-foundation-core/src/unicode_assembly.rs`

---

### TASK-011: Fix SCH Length to 24 Characters

**Priority:** P1 - HIGH
**Gap:** GAP-002
**Estimate:** 1 day
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Use proper Base96 encoding for SCH
- [ ] Ensure exactly 24 characters
- [ ] Update validation logic
- [ ] Regenerate test fixtures

**Files Modified:**
- `ctas7-foundation-core/src/trivariate_hashing.rs`

---

## PHASE 3: MEDIUM PRIORITY (Week 3-4: Dec 16-29)

**Goal:** Complete medium-severity gaps
**Compliance Improvement:** 82% → 91%

### TASK-012: Implement Supersession Logic

**Priority:** P2 - MEDIUM
**Gap:** GAP-005
**Estimate:** 3 days
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Implement `evaluate_supersession()` function
- [ ] Handle all 5 supersession types
- [ ] Automatically regenerate hashes
- [ ] Update database on supersession
- [ ] Add supersession chain queries

**Files Modified:**
- `ctas7-foundation-core/src/trivariate_hashing.rs`
- `ctas7-foundation-core/src/supersession.rs` (new)

---

### TASK-013: Add Domain/Execution Masks to Schema

**Priority:** P2 - MEDIUM
**Gap:** GAP-008
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] Add domain_mask SMALLINT (0-15)
- [ ] Add execution_mask SMALLINT (0-15)
- [ ] Add constraints
- [ ] Backfill existing data

**Files Modified:**
- `migrations/20251216_add_masks.sql` (new)

---

### TASK-014: Add Unicode Codepoint Integer Field

**Priority:** P2 - MEDIUM
**Gap:** GAP-011
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] Add unicode_codepoint INTEGER
- [ ] Add range constraint (E000-E9FF)
- [ ] Create index for range queries
- [ ] Populate from unicode_address TEXT

**Files Modified:**
- `migrations/20251217_add_unicode_codepoint.sql` (new)

---

### TASK-015: Store Escalation Tier Names

**Priority:** P2 - MEDIUM
**Gap:** GAP-013
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] Add escalation_tier_name TEXT field
- [ ] Add CHECK constraint for 7 tier names
- [ ] Update relationships table
- [ ] Backfill existing data

**Files Modified:**
- `migrations/20251218_add_tier_names.sql` (new)

---

### TASK-016: Add Sensory Substrate Fields

**Priority:** P2 - MEDIUM
**Gap:** GAP-017
**Estimate:** 2 days
**Owner:** L2 Ops Team

**Deliverables:**
- [ ] Add haptic profile JSONB
- [ ] Add haptic_intensity/angle
- [ ] Add voice_interval (7-point enum)
- [ ] Add tone_tenor
- [ ] Add suppression_tier
- [ ] Add explanation_depth

**Files Modified:**
- `migrations/20251219_add_sensory_substrate.sql` (new)

---

### TASK-017: Implement GNN Embedding Versioning

**Priority:** P2 - MEDIUM
**Gap:** GAP-018
**Estimate:** 2 days
**Owner:** ML Team

**Deliverables:**
- [ ] Create `gnn_embeddings` table
- [ ] Install pgvector extension
- [ ] Store model_name/version
- [ ] Link to entities
- [ ] Support 6 embedding spaces

**Files Modified:**
- `migrations/20251220_create_gnn_embeddings.sql` (new)

---

### TASK-018: Add Secondary Trivariate Support

**Priority:** P2 - MEDIUM
**Gap:** GAP-024
**Estimate:** 2 days
**Owner:** Foundation Core Team

**Deliverables:**
- [ ] Add trivariate_hash_secondary field
- [ ] Add dual_trivariate_enabled BOOLEAN
- [ ] Implement secondary hash generation
- [ ] Add constraint checking
- [ ] Update ATLAS/PLASMA systems

**Files Modified:**
- `migrations/20251221_add_secondary_trivariate.sql` (new)
- `ctas7-foundation-core/src/trivariate_hashing.rs`

---

### TASK-019: Implement N-V-N-N Grammar Parser

**Priority:** P2 - MEDIUM
**Gap:** GAP-004
**Estimate:** 3 days
**Owner:** NLP Team

**Deliverables:**
- [ ] Implement N-V-N-N tokenizer
- [ ] Handle incomplete grammar normalization
- [ ] Integrate with SCH generation
- [ ] Add NLP dependency (spacy/nltk)
- [ ] Unit tests for grammar cases

**Files Modified:**
- `ctas7-foundation-core/src/nvnn_parser.rs` (new)
- `ctas7-foundation-core/src/trivariate_hashing.rs`

---

## PHASE 4: RFC-9011 THREAT INGESTION (Week 5: Dec 30 - Jan 5)

**Goal:** Complete threat content ingestion system
**Compliance Improvement:** 91% → 94%

### TASK-020: Implement Semantic Conflict Resolver

**Priority:** P2 - MEDIUM
**Gap:** GAP-020
**Estimate:** 3 days
**Owner:** Threat Intel Team

**Deliverables:**
- [ ] Create `content_conflicts` table
- [ ] Implement conflict detection algorithm
- [ ] Define resolution actions (auto-merge/reject/review)
- [ ] Add severity levels
- [ ] Create conflict resolution UI/API

**Files Modified:**
- `migrations/20251230_create_content_conflicts.sql` (new)
- `threat-ingestion/src/conflict_resolver.rs` (new)

---

### TASK-021: Implement Semantic Imputer (SFE)

**Priority:** P2 - MEDIUM
**Gap:** GAP-022
**Estimate:** 4 days
**Owner:** ML Team

**Deliverables:**
- [ ] Create `semantic_imputations` table
- [ ] Integrate GNN for field imputation
- [ ] Integrate Phi-3 for missing fields
- [ ] Add confidence scoring
- [ ] Track imputation audit trail

**Files Modified:**
- `migrations/20251231_create_semantic_imputations.sql` (new)
- `threat-ingestion/src/semantic_imputer.rs` (new)

---

## PHASE 5: COMPLETENESS & POLISH (Week 6: Jan 6-13)

**Goal:** Close remaining gaps and documentation
**Compliance Improvement:** 94% → 95%+

### TASK-022: Expand SX9 Primitive Types

**Priority:** P3 - LOW
**Gap:** GAP-015
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] Expand entity_type enum with 10 SX9 primitives
- [ ] Add sx9_primitive field
- [ ] Add ptcc_primitive field
- [ ] Update constraints
- [ ] Migration script

**Files Modified:**
- `migrations/20260106_expand_sx9_primitives.sql` (new)

---

### TASK-023: Create RFC Compliance Matrix Table

**Priority:** P3 - LOW
**Gap:** GAP-016
**Estimate:** 1 day
**Owner:** Database Team

**Deliverables:**
- [ ] Create `rfc_compliance` table
- [ ] Track per-entity RFC compliance
- [ ] Add compliance_notes field
- [ ] Create compliance dashboard query

**Files Modified:**
- `migrations/20260107_create_rfc_compliance.sql` (new)

---

### TASK-024: Document Tail State Definition

**Priority:** P3 - LOW
**Gap:** GAP-009
**Estimate:** 1 day
**Owner:** Documentation Team

**Deliverables:**
- [ ] Define what "tail state" means in RFC-9001
- [ ] Add tail_state field to schema
- [ ] Update RFC-9001 specification
- [ ] Add COMMENT to schema

**Files Modified:**
- `RFC-9001-Trivariate-Hashing.md` (clarification)
- `migrations/20260108_add_tail_state.sql` (new)

---

### TASK-025: Create Schema Migration Script Suite

**Priority:** P3 - LOW
**Gap:** N/A (infrastructure)
**Estimate:** 2 days
**Owner:** DevOps Team

**Deliverables:**
- [ ] Consolidate all 20+ migrations
- [ ] Create rollback scripts
- [ ] Add migration version tracking
- [ ] Test on staging environment
- [ ] Document migration procedure

**Files Modified:**
- `migrations/README.md` (new)
- `scripts/run_migrations.sh` (new)
- `scripts/rollback_migrations.sh` (new)

---

### TASK-026: Create RFC Compliance Test Suite

**Priority:** P3 - LOW
**Gap:** N/A (validation)
**Estimate:** 3 days
**Owner:** QA Team

**Deliverables:**
- [ ] Test each RFC requirement
- [ ] Automated compliance checker
- [ ] Generate compliance report
- [ ] CI/CD integration
- [ ] Dashboard visualization

**Files Modified:**
- `tests/rfc_compliance/test_rfc9001.rs` (new)
- `tests/rfc_compliance/test_rfc9002.rs` (new)
- `tests/rfc_compliance/test_rfc9003.rs` (new)
- `tests/rfc_compliance/test_rfc9005.rs` (new)

---

## RESOURCE ALLOCATION

### Teams

**Foundation Core Team (3 engineers)**
- TASK-001: CUID Generation (2 days)
- TASK-005: UUIDv7 (1 day)
- TASK-010: Unicode Class (1 day)
- TASK-011: SCH Length (1 day)
- TASK-012: Supersession Logic (3 days)
- TASK-018: Secondary Trivariate (2 days)
**Total:** 10 days (2 weeks)

**Database Team (2 engineers)**
- TASK-002: Supersession Schema (1 day)
- TASK-006: Delta Angle (1 day)
- TASK-007: Operation Class (1 day)
- TASK-013: Masks (1 day)
- TASK-014: Unicode Codepoint (1 day)
- TASK-015: Tier Names (1 day)
- TASK-016: Sensory Substrate (2 days)
- TASK-022: SX9 Primitives (1 day)
- TASK-023: Compliance Matrix (1 day)
- TASK-024: Tail State (1 day)
**Total:** 11 days (2.5 weeks)

**Security Team (2 engineers)**
- TASK-003: Gate Conditions (3 days)
- TASK-009: Escalation Audit (1 day)
**Total:** 4 days (1 week)

**Threat Intel Team (2 engineers)**
- TASK-004: Crosswalk Mappings (2 days)
- TASK-008: Playbook Schema (2 days)
- TASK-020: Conflict Resolver (3 days)
**Total:** 7 days (1.5 weeks)

**ML Team (2 engineers)**
- TASK-017: GNN Versioning (2 days)
- TASK-021: Semantic Imputer (4 days)
**Total:** 6 days (1.5 weeks)

**NLP Team (1 engineer)**
- TASK-019: N-V-N-N Parser (3 days)

**DevOps Team (1 engineer)**
- TASK-025: Migration Scripts (2 days)

**QA Team (2 engineers)**
- TASK-026: Compliance Tests (3 days)

---

## MILESTONES

### M1: Critical Foundations Complete (Dec 8)
- CUID generation RFC-compliant
- Supersession tracking in place
- Gate conditions enforced
- Crosswalk mappings seeded

### M2: High Priority Fixes Complete (Dec 15)
- UUIDv7 enabled
- Delta angle tracking active
- Operation classification working
- Playbook schema deployed
- Escalation audit operational

### M3: Medium Priority Complete (Dec 29)
- Supersession logic functional
- Masks stored
- Unicode codepoint indexed
- Sensory substrate supported
- GNN embeddings versioned
- Secondary trivariate enabled

### M4: Threat Ingestion Complete (Jan 5)
- Semantic conflict resolution working
- Semantic imputer operational
- Full threat content pipeline functional

### M5: 95% Compliance Achieved (Jan 13)
- All gaps closed
- Migrations tested
- Compliance tests passing
- Documentation complete

---

## RISK MANAGEMENT

### HIGH RISKS

**R1: CUID Generation Breaking Changes**
- **Impact:** HIGH - Breaks all existing hashes
- **Mitigation:** Implement parallel generation, migrate in phases
- **Contingency:** Keep old CUID generator as fallback

**R2: Schema Migrations on Large Tables**
- **Impact:** HIGH - Downtime for ALTER TABLE
- **Mitigation:** Use zero-downtime migration techniques
- **Contingency:** Schedule maintenance windows

**R3: N-V-N-N Parser Complexity**
- **Impact:** MEDIUM - May not parse all cases correctly
- **Mitigation:** Start with simple heuristics, improve iteratively
- **Contingency:** Manual grammar correction pipeline

### MEDIUM RISKS

**R4: GNN Model Integration Delays**
- **Impact:** MEDIUM - Semantic imputer delayed
- **Mitigation:** Use simpler rule-based imputation first
- **Contingency:** Manual imputation UI

**R5: Resource Contention**
- **Impact:** MEDIUM - Teams blocked on dependencies
- **Mitigation:** Clear task dependencies, daily standups
- **Contingency:** Shift resources dynamically

---

## SUCCESS CRITERIA

### Quantitative
- ✅ 95%+ RFC compliance achieved
- ✅ All 24 gaps closed
- ✅ 100% critical gaps addressed
- ✅ 500+ unit tests passing
- ✅ Zero regressions in existing functionality

### Qualitative
- ✅ Schema migrations tested and documented
- ✅ RFC compliance automated testing
- ✅ Developer documentation complete
- ✅ Threat ingestion pipeline operational
- ✅ Team trained on new systems

---

## DEPENDENCIES

```
PHASE 1 → PHASE 2 → PHASE 3 → PHASE 4 → PHASE 5
   ↓         ↓         ↓         ↓         ↓
  M1  →    M2   →    M3   →    M4   →    M5
```

**Critical Path:**
1. TASK-001 (CUID) → TASK-010 (Unicode) → TASK-012 (Supersession)
2. TASK-002 (Schema) → TASK-006 (Delta) → TASK-013 (Masks)
3. TASK-003 (Gates) → TASK-009 (Audit)
4. TASK-004 (Crosswalk) → TASK-008 (Playbooks) → TASK-020 (Conflicts) → TASK-021 (Imputer)

---

## MONITORING & REPORTING

### Weekly Reports
- Tasks completed vs planned
- Compliance percentage increase
- Blockers and risks
- Resource utilization

### Daily Standups
- Yesterday's progress
- Today's plan
- Blockers

### Compliance Dashboard
- Real-time RFC compliance metrics
- Gap closure tracking
- Test pass rates
- Migration status

---

## SIGN-OFF

**Roadmap Status:** ✅ APPROVED
**Start Date:** December 2, 2025
**Target Completion:** January 13, 2026
**Engineering Lead:** [To be assigned]
**Product Owner:** CTAS-7 Core Engineering Group

---

**End of Roadmap**

