# SX9 RFC PIPELINE - REPOSITORY DISCOVERY

**Date**: December 17, 2025  
**Branch**: rfc-alignment-phase1  
**Repo**: /Users/cp5337/Developer/sx9

---

## 🎯 CRITICAL FINDING: EXISTING EA DOCUMENT PIPELINE

**Location**: `tools/abe/iac/abe-qa-system/ea-documents/ea_document_generator.py`

**Status**: COMPLETE BUT BURIED (886 lines)

**Capabilities**:

- ✅ Zotero Integration (RFC-9010 Prior-Art Check)
- ✅ LaTeX Generation (DoD DevSecOps + RFC + SDD templates)
- ✅ Overleaf Git Integration
- ✅ BibTeX Export
- ✅ RFC Markdown → LaTeX Extraction
- ✅ Linear Document Tracking

**Supporting Files**:

- `tools/abe/iac/output/real_extraction/CONSOLIDATED_BIBTEX.bib` - Existing BibTeX
- `tools/abe/iac/abe-qa-system/ea-documents/templates/dod-devsecops-sdd.tex` - DoD template
- `tools/abe/iac/abe-qa-system/ea-documents/rfc-needles.json` - RFC metadata
- `sx9-conda/query_zotero.py` - Zotero DB query tool
- `sx9-conda/RESEARCH_PLAN_ZOTERO.md` - Zotero research plan

---

## 📊 RFC INVENTORY

**Total RFC Files Found**: 102

**Locations**:

- `01-rfc/` - Main RFC directory (organized by category)
- `01-rfc/collected-bundle/` - 27 RFCs from SX9_RFC_COLLECTED_BUNDLE.zip
- `01-rfc/harvest-phase1/` - 4 new RFCs from SX9_RFC_HARVEST_PHASE1.zip
- `01-rfc/rfc-alignment/` - 4 ECS/Plasma docs from RFC Alignment.zip

**Categories in 01-rfc/**:

- 9000-core/
- 9010-pipeline/
- 9100-integration/
- 9300-cognitive/
- 9400-application/
- 9500-platform/
- 9800-operational/

---

## 🛠️ BUILD SYSTEM DETECTED

**Primary**: Rust (Cargo.toml workspace)  
**Secondary**: Node.js (package.json + pnpm)  
**Scripting**: Python 3 (sx9-conda, tools/abe)

**Build Tools**:

- ❌ No Makefile/justfile found
- ✅ Cargo workspace
- ✅ pnpm for Node packages
- ✅ Python scripts in tools/

**LaTeX/Pandoc**:

- ⚠️ Pandoc: Not checked (cancelled)
- ⚠️ pdflatex: Not checked (cancelled)
- ✅ LaTeX templates exist in tools/abe/iac

---

## 🎯 CONSOLIDATION PLAN

**Action**: Pull all RFC-related assets into organized staging areas

**Directories Created**:

- `05-sx9-converge/` - All CONVERGE files
- `05-sx9-legacy-finds/` - Existing LaTeX/Zotero pipeline

**Next Steps**:

1. Move EA document generator to 05-sx9-legacy-finds
2. Copy all .tex and .bib files to 05-sx9-legacy-finds
3. Consolidate CONVERGE zips to 05-sx9-converge
4. Process RFC bundles into 01-rfc structure
5. Create unified RFC inventory
6. Build new LaTeX pipeline leveraging existing code

---

## 🔧 ADAPTATION DECISIONS

1. **Use Python** for RFC processing (existing ea_document_generator.py)
2. **Leverage existing Zotero integration** (don't rebuild)
3. **Reuse LaTeX templates** from tools/abe/iac
4. **No Makefile** - use Python scripts or cargo xtask
5. **Overleaf optional** - emit LaTeX for manual upload

**Rationale**: Don't reinvent the wheel - the pipeline exists, just needs organization and exposure.
