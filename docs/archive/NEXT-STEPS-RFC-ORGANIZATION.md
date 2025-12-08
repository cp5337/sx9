# Next Steps: RFC Organization and Gateway Implementation

**Date:** December 2025  
**Status:** Action Plan  
**Priority:** High (Copyright Attorney Preparation)

---

## Immediate Next Steps (Priority Order)

### 1. Update RFC Registry

**Action:** Add RFC-9114 to REGISTRY.md

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/01-rfc/REGISTRY.md`

**Changes Required:**
- Add RFC-9114 to Integration Series (9100-9149)
- Update NEXT_AVAILABLE to 9115
- Add RFC-9114 entry with status, location, gateway compliance

**Estimated Time:** 5 minutes

---

### 2. Fix Critical Technical Issues

#### 2.1 Remove Blake3 from RFC-9025

**Action:** Replace Blake3 references with Murmur3-64 in RFC-9025-Unified-Interview-Schema.md

**Location:** `/Users/cp5337/Developer/ctas-7-shipyard-staging/01-rfc/core/RFC-9025-Unified-Interview-Schema.md`

**Changes Required:**
- Replace `blake3-operational-hash` → `murmur3-operational-hash`
- Replace `blake3-semantic-hash` → `murmur3-semantic-hash`
- Update H1/H2 hash generation examples to use Murmur3-64
- Add RFC-9001 compliance note

**Estimated Time:** 15 minutes

#### 2.2 Resolve RFC-9025 Duplicate

**Action:** Determine canonical version and archive the other

**Files:**
- `core/RFC-9025-Node-Interview-Schema.md` (Node interviews, TOML format)
- `core/RFC-9025-Unified-Interview-Schema.md` (Unified schema, JSON format, has Blake3 issue)

**Decision:** Keep `Unified-Interview-Schema.md` (more comprehensive), archive `Node-Interview-Schema.md` to `cognitive-session-reference/`

**Estimated Time:** 10 minutes

---

### 3. Resolve Duplicate RFCs

#### 3.1 RFC-9300, RFC-9303, RFC-9304 Duplicates

**Action:** Delete `files/` versions, keep root versions, move to proper locations

**Files to Delete:**
- `files/RFC-9300-HD4-Canonical-Specification.md`
- `files/RFC-9303-Crystal-Realms-Kinematics.md`
- `files/RFC-9304-SX9-Workbench.md`

**Files to Move:**
- `RFC-9300-HD4-Canonical-Specification.md` (root) → `core/RFC-9300-HD4-Canonical-Specification.md`
- `RFC-9303-Crystal-Realms-Kinematics.md` (root) → `core/RFC-9303-Crystal-Realms-Kinematics.md`
- `RFC-9304-SX9-Workbench.md` (root) → `application/RFC-9304-SX9-Workbench.md`

**Also Move:**
- `files/RFC-9301-Thyristor-Crystal-RingBus.md` → `core/RFC-9301-Thyristor-Crystal-RingBus.md`
- `files/RFC-9302-Nonagon-Analytic-Node.md` → `core/RFC-9302-Nonagon-Analytic-Node.md`

**Estimated Time:** 20 minutes

---

### 4. Fix File Locations

#### 4.1 Move RFC-9112

**Action:** Move from root to `integration/`

**From:** `RFC-9112-v3.0-Deterministic-Prompt-Engineering.md` (root)  
**To:** `integration/RFC-9112-Deterministic-Prompt-Engineering.md`

**Estimated Time:** 2 minutes

#### 4.2 Move RFC-9200

**Action:** Move from `integration/` to `platform/`

**From:** `integration/RFC-9200-SX9-Development-Center.md`  
**To:** `platform/RFC-9200-SX9-Development-Center.md` (create `platform/` directory if needed)

**Estimated Time:** 2 minutes

#### 4.3 Move RFC-9023

**Action:** Move from `integration/` to `core/` (or renumber to 9100+)

**From:** `integration/RFC-9023-Security-Framework-Integration-Map.md`  
**To:** `core/RFC-9023-Security-Framework-Integration-Map.md` (or renumber to RFC-9115+)

**Decision:** Move to `core/` since it's numbered 9023 (cognitive series)

**Estimated Time:** 2 minutes

---

### 5. Add Missing RFC to Registry

#### 5.1 Add RFC-9030

**Action:** Add RFC-9030 to Integration Series in REGISTRY.md

**RFC:** RFC-9030: Unified Linear Agent Infrastructure  
**Location:** `integration/RFC-9030-Unified-Linear-Agent-Infrastructure.md`  
**Status:** Draft

**Estimated Time:** 3 minutes

---

### 6. Naming Consistency (Large Task)

#### 6.1 Systematic CTAS-7 → SX9/Synaptix9 Replacement

**Scope:** 400+ instances across 38 RFC files

**Strategy:**
1. **High Priority (Core RFCs):** RFC-9001, RFC-9004, RFC-9026, RFC-9000
2. **Medium Priority (Integration RFCs):** RFC-9100 through RFC-9113, RFC-9130, RFC-9876
3. **Low Priority (Application RFCs):** RFC-9150, RFC-9151

**Replacement Rules:**
- "CTAS-7" → "Synaptix9" (full system name)
- "CTAS7" → "SX9" (technical abbreviation)
- "ctas-7" → "synaptix9"
- "ctas7" → "sx9"
- Preserve version numbers (e.g., "v7.3.1" can remain)
- Preserve component names for backward compatibility (e.g., "ctas7-foundation-core" can remain or become "sx9-foundation-core")

**Estimated Time:** 2-3 hours (can be done incrementally)

---

### 7. Update RFC Cross-References

#### 7.1 Verify All RFC References

**Action:** Check all RFC cross-references in all RFCs for accuracy

**Issues to Check:**
- Non-existent RFC numbers
- Incorrect RFC numbers
- Missing RFC references
- Outdated RFC references

**Estimated Time:** 1 hour

---

### 8. Prepare for Copyright Attorney

#### 8.1 Create Final Package

**Documents to Include:**
1. `RFC-COMPREHENSIVE-ANALYSIS-FOR-ATTORNEY.md` — Complete analysis
2. `RFC-ORGANIZATION-COMPLETE.md` — RFC organization
3. `RFC-9114-SX9-Gateway-Architecture.md` — Gateway RFC
4. Updated `REGISTRY.md` — Complete RFC registry
5. List of all 58 RFCs with status, location, compliance

**Estimated Time:** 30 minutes

---

## Recommended Execution Order

### Phase 1: Critical Fixes (1-2 hours)
1. ✅ Update REGISTRY.md (add RFC-9114, RFC-9030)
2. ✅ Remove Blake3 from RFC-9025
3. ✅ Resolve RFC-9025 duplicate
4. ✅ Resolve RFC-9300/9303/9304 duplicates
5. ✅ Move files to correct locations

### Phase 2: Naming Consistency (2-3 hours)
6. ⚠️ Systematic CTAS-7 → SX9/Synaptix9 replacement (can be incremental)

### Phase 3: Final Verification (1 hour)
7. ⚠️ Update RFC cross-references
8. ⚠️ Prepare copyright attorney package

---

## Quick Wins (Do First)

**These can be done immediately:**

1. **Update REGISTRY.md** — Add RFC-9114 and RFC-9030 (5 min)
2. **Move RFC-9112** — From root to integration/ (2 min)
3. **Move RFC-9200** — From integration/ to platform/ (2 min)
4. **Delete duplicate files** — RFC-9300/9303/9304 in files/ (5 min)

**Total Quick Wins:** ~15 minutes

---

## Blockers

**None identified** — All tasks are independent and can proceed in parallel or sequentially.

---

## Success Criteria

**RFC System Ready for Copyright Attorney When:**
- ✅ All RFCs properly organized and located
- ✅ No duplicate RFCs
- ✅ All RFCs in registry
- ✅ Naming consistency (CTAS-7 → SX9/Synaptix9)
- ✅ Technical compliance (no Blake3, Murmur3 only)
- ✅ All cross-references verified
- ✅ Gateway RFC (RFC-9114) complete
- ✅ Documentation package prepared

---

**Next Immediate Action:** Update REGISTRY.md to add RFC-9114



