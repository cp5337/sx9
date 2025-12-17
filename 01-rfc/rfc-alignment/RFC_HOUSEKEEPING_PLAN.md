# RFC HOUSEKEEPING & EXPANSION PLAN

**Date**: December 17, 2025  
**Purpose**: Organize RFC numbering and create path to flesh out incomplete RFCs

---

## 🎯 OBJECTIVES

1. **RFC Numbering Audit** - Identify gaps, duplicates, and inconsistencies
2. **Incomplete RFC Identification** - Find RFCs that need expansion
3. **Expansion Workflow** - Create systematic process to flesh out RFCs using Zotero research

---

## 📊 CURRENT STATE (From Inventory)

**Total RFCs**: 105 files

**Known Issues**:

1. **Multiple RFC-9002 variants**:
   - RFC-9002-Class-E-Promotion-System.md
   - RFC-9002-Unicode-Routing.md
   - RFC-9002-Unicode-Tool-Chains.md

2. **Duplicates** between:
   - collected-bundle/ (29 RFCs)
   - Main categories (9000-core, 9010-pipeline, etc.)

3. **Archive confusion**:
   - \_archive/ (5 RFCs)
   - archive/ (3 RFCs)

---

## 🔍 PHASE 1: NUMBERING AUDIT

### **Action Items**:

1. **Generate RFC Number Map**:

   ```python
   # Script: sx9-conda/rfc_numbering_audit.py
   # Output: build/rfc-numbering-audit.json
   ```

2. **Identify Issues**:
   - [ ] Duplicate RFC numbers
   - [ ] Missing RFC numbers (gaps in sequence)
   - [ ] RFCs outside 01-rfc/ structure
   - [ ] Inconsistent naming conventions

3. **Create Resolution Plan**:
   - **DO NOT RENUMBER** existing RFCs
   - Use suffixes for variants (e.g., RFC-9002A, RFC-9002B)
   - Document canonical vs variant status

---

## 📝 PHASE 2: COMPLETENESS ASSESSMENT

### **Criteria for "Incomplete" RFC**:

1. **Length < 500 words** (stub)
2. **Missing sections**:
   - Abstract
   - Motivation
   - Specification
   - References

3. **Placeholder content**:
   - "TBD"
   - "TODO"
   - "Coming soon"

4. **No implementation details**

### **Assessment Script**:

```python
# Script: sx9-conda/rfc_completeness_check.py
# Checks:
# - Word count
# - Section presence
# - Placeholder detection
# Output: build/rfc-completeness-report.json
```

### **Expected Output**:

```json
{
  "complete": [
    {"rfc": "RFC-9001", "score": 95, "status": "production-ready"},
    ...
  ],
  "needs_expansion": [
    {"rfc": "RFC-9027", "score": 45, "missing": ["implementation", "references"]},
    ...
  ],
  "stubs": [
    {"rfc": "RFC-9XXX", "score": 15, "reason": "placeholder only"},
    ...
  ]
}
```

---

## 🚀 PHASE 3: EXPANSION WORKFLOW

### **Zotero-Driven Research Process**:

**For each incomplete RFC**:

1. **Extract Keywords**:

   ```python
   # From RFC title + abstract
   keywords = extract_keywords(rfc_content)
   ```

2. **Query Zotero**:

   ```python
   # Using local API: http://localhost:23119/api/
   results = zotero_client.search(keywords)
   ```

3. **Generate Prior Art Section**:

   ```markdown
   ## Prior Art

   Based on Zotero research (RFC-9010 compliance):

   - [Citation 1] - Relevant finding
   - [Citation 2] - Related work
   ```

4. **Expand Specification**:
   - Use Zotero findings to inform technical details
   - Add implementation examples
   - Include references

5. **Update BibTeX**:
   ```bash
   # Add to sx9-references/sx9.bib
   ```

---

## 🛠️ TOOLS TO CREATE

### **1. RFC Numbering Audit Script**

**File**: `sx9-conda/rfc_numbering_audit.py`

**Functionality**:

- Scan all RFCs
- Extract RFC numbers
- Detect duplicates
- Identify gaps
- Generate report

### **2. RFC Completeness Checker**

**File**: `sx9-conda/rfc_completeness_check.py`

**Functionality**:

- Word count analysis
- Section detection
- Placeholder scanning
- Scoring algorithm
- JSON report generation

### **3. RFC Expansion Assistant**

**File**: `sx9-conda/rfc_expansion_assistant.py`

**Functionality**:

- Keyword extraction
- Zotero integration (local API)
- Prior art generation
- BibTeX management
- Template-based expansion

---

## 📋 NUMBERING CONVENTIONS

### **Standard Format**:

```
RFC-XXXX-Title-With-Hyphens.md
```

### **Variant Handling**:

```
RFC-9002-Unicode-Routing.md           # Canonical
RFC-9002A-Class-E-Promotion.md        # Variant A (addendum)
RFC-9002B-Unicode-Tool-Chains.md      # Variant B (addendum)
```

### **Revision Handling**:

```
RFC-9302-Nonagon-Analytic-Node.md                    # Original
RFC-9302-Rev1-Nonagon-Analytic-Node-VALIDATED.md    # Revision 1
```

---

## 🎯 PRIORITY RFCS FOR EXPANSION

### **High Priority** (Core Architecture):

1. RFC-9027 - Inference Propagation Kernel (harvest-phase1)
2. RFC-9028 - Uncertainty Confidence Algebra (harvest-phase1)
3. RFC-9029 - Dynamic Domain Adaptation (harvest-phase1)
4. RFC-9305 - Nonagon Execution Semantics (harvest-phase1)

### **Medium Priority** (Integration):

1. RFC-9100 - Dual Trivariate PTCC Integration
2. RFC-9105 - SPIRES Extraction
3. RFC-9107 - Unified Agent Infrastructure

### **Low Priority** (Operational):

1. RFC-9130 - L2 NATS Kali Execution
2. RFC-9131 - Dynamic Resource Escalation

---

## ✅ SUCCESS CRITERIA

1. **Numbering**:
   - [ ] All duplicate numbers resolved
   - [ ] Gaps documented (not filled)
   - [ ] Variant naming consistent

2. **Completeness**:
   - [ ] All RFCs scored for completeness
   - [ ] Expansion priorities identified
   - [ ] Workflow tested on 3 sample RFCs

3. **Zotero Integration**:
   - [ ] Local API connection verified
   - [ ] Prior art generation working
   - [ ] BibTeX export functional

---

## 🚧 CONSTRAINTS

1. **DO NOT RENUMBER** - RFC numbers are immutable identifiers
2. **DO NOT DELETE** - Archive, don't remove
3. **DO NOT MERGE** - Keep variants separate with clear naming
4. **DO PRESERVE** - Maintain git history

---

## 📝 NEXT ACTIONS

1. Create `rfc_numbering_audit.py`
2. Run audit on 105 RFCs
3. Generate numbering report
4. Create `rfc_completeness_check.py`
5. Identify top 10 RFCs needing expansion
6. Test Zotero integration with RFC-9027

**Status**: Ready to begin housekeeping
