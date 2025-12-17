# RFC Pipeline - Next Steps

**Date**: December 17, 2025  
**Status**: Phase 0-2 Complete, Ready for Phase 3 or Housekeeping

---

## ✅ CONFIRMED

- **Zotero**: Mac is primary instance (`http://localhost:23119/api/`)
- **Windows**: No Zotero installation
- **Data**: `/Users/cp5337/Zotero/`

---

## 🎯 THREE PATHS FORWARD

### **Path A: Continue RFC Pipeline (Phase 3 - PLAN)**

**Objective**: Design the LaTeX build system

**Tasks**:

1. Create `02-sx9-latex/PLAN.md`
2. Define directory structure
3. Design `build_rfc.py` interface
4. Plan disclaimer injection mechanism
5. Define commit strategy

**Estimated Time**: 1-2 hours  
**Deliverable**: Complete plan document ready for implementation

---

### **Path B: RFC Housekeeping**

**Objective**: Clean up RFC numbering and identify expansion candidates

**Tasks**:

1. Create `sx9-conda/rfc_numbering_audit.py`
2. Run audit on 105 RFCs
3. Identify duplicates (RFC-9002 variants)
4. Create `sx9-conda/rfc_completeness_check.py`
5. Score RFCs for completeness
6. Prioritize top 10 for expansion

**Estimated Time**: 2-3 hours  
**Deliverable**: Audit reports + expansion priorities

---

### **Path C: Commit Current Work**

**Objective**: Save progress before proceeding

**Commands**:

```bash
cd /Users/cp5337/Developer/sx9
git add -A
git status
git commit -m "feat(rfc): Complete Phase 0-2 - Inventory, consolidation, research

- Created RFC inventory (105 RFCs) with SHA256 hashes
- Consolidated ontology files (11 files) to 0X-sx9-ontology-master
- Documented existing EA document pipeline
- Verified H1/H2 distinction and Ring Bus integration
- Created housekeeping and expansion plans
- Confirmed Zotero configuration (Mac primary)"

git push origin rfc-alignment-phase1
```

**Estimated Time**: 5 minutes  
**Deliverable**: Clean commit on feature branch

---

## 💡 RECOMMENDATION

**Suggested Order**:

1. **Commit current work** (Path C) - Save progress
2. **RFC Housekeeping** (Path B) - Clean up numbering issues
3. **Continue Pipeline** (Path A) - Build LaTeX system

**Rationale**:

- Committing now creates a clean checkpoint
- Housekeeping identifies which RFCs need work
- Pipeline implementation can focus on complete RFCs first

---

## 📋 QUICK WINS

If you want something fast before deciding:

1. **Test Zotero Connection**:

   ```bash
   python sx9-conda/query_zotero.py
   ```

2. **View RFC Duplicates**:

   ```bash
   cd 01-rfc/9000-core
   ls -1 | grep RFC-9002
   ```

3. **Check Git Status**:
   ```bash
   git status --short
   ```

---

## ⏭️ YOUR CALL

Which path would you like to take?

- **A**: Design LaTeX build system
- **B**: RFC housekeeping audit
- **C**: Commit and save progress
- **Other**: Something else?
