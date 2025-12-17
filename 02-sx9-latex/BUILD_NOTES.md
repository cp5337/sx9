# SX9 RFC → LaTeX Pipeline - BUILD NOTES

**Date Started**: December 17, 2025  
**Branch**: rfc-alignment-phase1  
**Baseline Commit**: 72e4136

---

## 📊 PHASE 0: BASELINE (COMPLETE)

**Git Status**:

- Branch: `rfc-alignment-phase1`
- Baseline commit: `72e4136` (feat: plasma-defender commissioning)
- Clean working directory after consolidation

**Repository Structure**:

```
sx9/
├── 01-rfc/                    # 105 RFC files
├── 02-sx9-latex/              # LaTeX pipeline (this directory)
├── 05-sx9-converge/           # CONVERGE consolidation
├── 05-sx9-legacy-finds/       # Existing EA document pipeline
├── 0X-sx9-ontology-master/    # Ontology files (11 files)
├── build/                     # RFC inventory outputs
├── sx9-conda/                 # Python packages + scripts
└── tools/                     # Build tools
```

---

## 📋 PHASE 1: ANALYZE REPO (COMPLETE)

**RFC Inventory Results**:

- **Total RFCs**: 105 files
- **Formats**: 101 .md, 2 .txt, 2 .docx
- **Categories**: 12 categories (9000-core through harvest-phase1)
- **Inventory Files**:
  - `build/rfc-index.json` (57KB, detailed)
  - `build/rfc-index.toml` (25KB, SPIRES-style)
- **Inventory Script**: `sx9-conda/sx9_rfc_inventory.py` (Python, 300+ lines)

---

## 🔍 PHASE 2: RESEARCH (IN PROGRESS)

### **CRITICAL DISCOVERY: Existing EA Document Pipeline**

**Location**: `tools/abe/iac/abe-qa-system/ea-documents/`

**Main File**: `ea_document_generator.py` (886 lines)

**Capabilities**:

1. ✅ **Zotero Integration** (RFC-9010 Prior-Art Check)
   - pyzotero library
   - Local Zotero 7 server support
   - Remote API support
   - BibTeX export

2. ✅ **LaTeX Generation**
   - DoD DevSecOps template
   - RFC template
   - SDD (Software Design Document) template
   - IEEE-compliant formatting

3. ✅ **Overleaf Integration**
   - Git bridge support
   - Auto-push to Overleaf projects

4. ✅ **RFC Extraction**
   - Markdown → LaTeX conversion
   - Metadata extraction (title, status, version)
   - Section parsing

5. ✅ **Linear Integration**
   - Document tracking
   - Review workflow

**Supporting Files**:

- `CONSOLIDATED_BIBTEX.bib` (34KB) - Existing references
- `templates/dod-devsecops-sdd.tex` (11.5KB) - DoD template
- `rfc-needles.json` (16.5KB) - RFC metadata
- `document_fingerprint_system.py` - Document hashing

**Zotero Configuration** (✅ CONFIRMED):

- **Primary Instance**: Mac (this machine)
- **Local API**: `http://localhost:23119/api/`
- **Data Directory**: `/Users/cp5337/Zotero/`
- **Windows**: Not installed (Mac is primary)

**Zotero Tools**:

- `sx9-conda/query_zotero.py` - Zotero DB query
- `sx9-conda/RESEARCH_PLAN_ZOTERO.md` - Research plan

---

## 🛠️ BUILD SYSTEM ANALYSIS

### **Primary Languages** (Ranked):

1. **Rust** - Cargo workspace (primary)
2. **Python** - sx9-conda, tools/abe
3. **TypeScript** - Node.js packages (pnpm)

### **Build Tools**:

- ❌ **No Makefile/justfile** - Will use Python scripts
- ✅ **Cargo workspace** - Rust projects
- ✅ **pnpm** - Node package management
- ✅ **Python 3** - Scripting and tools

### **LaTeX/Pandoc**:

- ⚠️ **Pandoc**: Not verified (may not be installed)
- ⚠️ **pdflatex**: Not verified (may not be installed)
- ✅ **LaTeX templates**: Exist in tools/abe/iac
- ✅ **BibTeX**: CONSOLIDATED_BIBTEX.bib available

---

## 📝 ADAPTATION DECISIONS

### **Decision 1: Use Existing Python Pipeline**

**Rationale**: Don't rebuild what exists. The ea_document_generator.py is complete and functional.

**Action**:

- Move/adapt ea_document_generator.py to 02-sx9-latex/
- Expose as primary RFC → LaTeX tool
- Add disclaimer injection capability

### **Decision 2: Python Over Rust for LaTeX**

**Rationale**:

- Existing pipeline is Python
- LaTeX generation is I/O-bound (not performance-critical)
- Zotero integration (pyzotero) is Python-only

**Action**: Keep Python for LaTeX pipeline, use Rust only for performance-critical inventory/hashing

### **Decision 3: No Makefile - Use Python Scripts**

**Rationale**: Repo doesn't use Make/Just, Python is dominant for scripting

**Action**: Create `build_rfc.py` instead of `build_rfc.sh`

### **Decision 4: LaTeX Emit-Only Mode**

**Rationale**: pdflatex may not be installed on all systems

**Action**:

- Primary mode: Generate .tex files
- Optional mode: Compile to PDF if pdflatex available
- Overleaf upload as alternative

### **Decision 5: Reuse Existing Templates**

**Rationale**: DoD DevSecOps templates already exist and are IEEE-compliant

**Action**: Copy templates from tools/abe/iac to 02-sx9-latex/templates/

---

## 🔧 REUSABLE COMPONENTS

### **From ea_document_generator.py**:

1. `ZoteroClient` class - Zotero API integration
2. `LaTeXGenerator` class - Template-based LaTeX generation
3. `RFCExtractor` class - Markdown → LaTeX conversion
4. `OverleafClient` class - Git bridge integration
5. Template system (DoD, RFC, SDD)

### **From CONSOLIDATED_BIBTEX.bib**:

- 34KB of existing references
- Can be used as base sx9.bib

### **From query_zotero.py**:

- Direct SQLite query to Zotero DB
- Useful for local-only operation

---

## 📋 INTEGRATION REQUIREMENTS

### **Must Integrate With**:

1. **RFC Inventory** (`sx9_rfc_inventory.py`)
   - Use rfc-index.json for RFC discovery
   - Hash verification before LaTeX generation

2. **Disclaimer Injection**
   - Add "formerly CTAS" disclaimer
   - Idempotent (marker-based)
   - Toggle via flag

3. **sx9-conda Structure**
   - Follow existing package conventions
   - Use sx9-conda for Python scripts

### **Must NOT Break**:

1. Existing Cargo workspace
2. pnpm package management
3. tools/abe/iac structure (copy, don't move)

---

## ⚠️ CONSTRAINTS DISCOVERED

### **Technical Constraints**:

1. **No Internet Required** - All tools must work offline
2. **No Pandoc Assumption** - May not be installed
3. **No pdflatex Assumption** - LaTeX emit-only mode required
4. **Preserve Existing Tools** - Don't break tools/abe/iac

### **Organizational Constraints**:

1. **Multi-year Project** - Decisions have long-term impact
2. **Multi-persona** - Multiple teams/users
3. **Legally Sensitive** - IP + academic considerations
4. **No RFC Renumbering** - RFCs are immutable identifiers

---

## 🚀 NEXT STEPS (PHASE 3: PLAN)

1. Create `02-sx9-latex/PLAN.md` with:
   - Goals and non-goals
   - Directory structure
   - Build commands
   - Failure modes
   - Commit strategy

2. Design disclaimer injection mechanism
3. Plan template adaptation
4. Define build script interface

---

## 📊 TOOLING COMPATIBILITY MATRIX

| Capability      | Found?     | Notes                          |
| --------------- | ---------- | ------------------------------ |
| Make / Just     | ❌ No      | Will use Python scripts        |
| Rust toolchain  | ✅ Yes     | Cargo workspace                |
| Python          | ✅ Yes     | Python 3, sx9-conda            |
| Pandoc          | ⚠️ Unknown | Fallback: custom converter     |
| TeX Live        | ⚠️ Unknown | Emit-only mode if missing      |
| Existing BibTeX | ✅ Yes     | CONSOLIDATED_BIBTEX.bib (34KB) |
| Zotero          | ✅ Yes     | pyzotero + local DB            |
| Overleaf        | ✅ Yes     | Git bridge integration         |

---

## 🎯 PIPELINE ARCHITECTURE (PLANNED)

```
RFC Markdown (.md)
    ↓
[sx9_rfc_inventory.py] → rfc-index.json
    ↓
[build_rfc.py] → Select RFC by number
    ↓
[RFCExtractor] → Parse markdown, extract metadata
    ↓
[ZoteroClient] → Search for related references
    ↓
[LaTeXGenerator] → Generate .tex + .bib
    ↓
[Disclaimer Injection] → Add "formerly CTAS" note
    ↓
Output: RFC-XXXX.tex + RFC-XXXX.bib
    ↓
[Optional: pdflatex] → RFC-XXXX.pdf
    OR
[Optional: Overleaf] → Push to Git bridge
```

---

## ✅ PHASE 2 COMPLETE

**Findings Documented**: ✅  
**Existing Pipeline Located**: ✅  
**Reusable Components Identified**: ✅  
**Adaptation Decisions Made**: ✅  
**Constraints Documented**: ✅

**Ready for Phase 3: PLAN**
