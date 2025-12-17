# RFC-9304 MERGE PLAN

**Date**: December 17, 2025  
**Status**: REQUIRES MANUAL MERGE

---

## 📋 FILES TO MERGE

### **File A (Dec 17, 2025)**: GLAF Graph Engine Specification

**Path**: `01-rfc/collected-bundle/RFC-9304-GLAF-Graph-Engine-Specification.md`

- **Size**: 29,479 bytes
- **Math Score**: 164 (ring-bus)
- **Date**: Dec 17, 2025 (TODAY - has latest matroid updates)
- **Focus**: Graph engine specification

### **File B (Dec 8, 2025)**: SX9 Workbench

**Path**: `01-rfc/9400-application/RFC-9304-SX9-Workbench.md`

- **Size**: 46,149 bytes (57% LARGER)
- **Math Score**: 188 (delta-angle + ring-bus)
- **Date**: Dec 8, 2025
- **Focus**: Application workbench implementation

---

## 🎯 MERGE STRATEGY

### **Step 1: Review Both Files**

- [ ] Read GLAF spec (Dec 17) for latest matroid theory updates
- [ ] Read Workbench (Dec 8) for implementation details and delta-angle content
- [ ] Identify unique content in each file

### **Step 2: Create Merged Version**

- [ ] Start with Workbench (larger, more complete)
- [ ] Add matroid theory updates from GLAF spec
- [ ] Integrate any new sections from Dec 17 version
- [ ] Preserve delta-angle and ring-bus content from both

### **Step 3: Save Merged File**

- [ ] Save as: `01-rfc/9400-application/RFC-9304-GLAF-Graph-Engine-Specification.md`
- [ ] Update modification date to today
- [ ] Add merge note in header

### **Step 4: Archive Originals**

- [ ] Move `collected-bundle/RFC-9304-GLAF-Graph-Engine-Specification.md` to `_archive/pre-merge/`
- [ ] Move `9400-application/RFC-9304-SX9-Workbench.md` to `_archive/pre-merge/`
- [ ] Keep `9000-core/RFC-9304-GLAF-Graph-Engine-Specification.md` (Dec 8 copy) for reference

---

## 📝 MERGE CHECKLIST

**Content to Preserve from GLAF Spec (Dec 17)**:

- [ ] Latest matroid theory definitions
- [ ] Updated graph engine architecture
- [ ] New ring-bus integration details
- [ ] Any Dec 17 mathematical updates

**Content to Preserve from Workbench (Dec 8)**:

- [ ] Delta-angle implementations
- [ ] Workbench UI/UX specifications
- [ ] Application integration details
- [ ] Implementation examples (16KB extra content)

**Combined Sections**:

- [ ] Abstract (merge both)
- [ ] Architecture (Workbench base + GLAF updates)
- [ ] Mathematical Foundations (combine both)
- [ ] Implementation (Workbench details)
- [ ] Integration (both perspectives)
- [ ] References (combine)

---

## ⚠️ CRITICAL NOTES

1. **Don't lose Dec 17 updates**: The GLAF spec has today's matroid changes
2. **Don't lose Dec 8 details**: The Workbench has 16KB more implementation content
3. **Preserve delta-angle**: Workbench has delta-angle math (score 188)
4. **Keep ring-bus**: Both have ring-bus, ensure consistency

---

## 🚀 EXECUTION

**After merge**:

1. Update `build/rfc-index.json` with merged file
2. Re-run `sx9_rfc_inventory.py` to update hashes
3. Commit merged version
4. Delete pre-merge copies from archive after verification

**Status**: READY FOR MANUAL MERGE
