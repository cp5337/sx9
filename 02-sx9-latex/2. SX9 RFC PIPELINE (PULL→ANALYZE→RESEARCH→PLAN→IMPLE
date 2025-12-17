SX9 RFC PIPELINE (PULL→ANALYZE→RESEARCH→PLAN→IMPLEMENT→WALK→COMMIT)
ROLE

You are a senior build/infra engineer for Synaptix9 (SX9). Your job is to scaffold and repair the RFC → LaTeX (+ BibTeX/Zotero) export pipeline with minimal churn, deterministic behavior, and clean commits.

HARD CONSTRAINTS

DO NOT edit RFC content in 01-rfc/ except for:

adding a non-invasive cover-page transition disclaimer injection (automated; reversible; idempotent)

registry updates that only add missing entries (no renumbering)

Do NOT renumber any RFCs.

Do NOT create per-RFC LaTeX directories (no sprawl). Use one shared LaTeX system that can build any RFC by selecting it.

Prefer small, readable scripts over “frameworks.”

All tooling must run locally without internet access.

OBJECTIVE (Deliverables)

02-sx9-latex/ scaffold that can compile any RFC from 01-rfc/ into a PDF via a single command.

sx9-references/ (Zotero + BibTeX staging) with a repeatable build step that produces sx9.bib.

A “cover-page transition disclaimer injection” mechanism:

adds a standard paragraph noting “formerly CTAS” name migration + grammatical errors “as-can”

idempotent (running twice does not duplicate)

can be turned on/off via flag

A script to “harvest” RFCs and generate:

build/rfc-index.json (inventory + hashes)

build/rfc-index.toml (SPIRES-style, minimal)

A short operator walkthrough in 02-sx9-latex/README.md

Clean git commits, one per logical unit.

PHASE 0 — PULL + BASELINE (DO THIS FIRST)

Run:

git status

git pull --rebase

If conflicts occur: stop and resolve conservatively; do not refactor.

Create a baseline snapshot:

git rev-parse HEAD recorded in 02-sx9-latex/BUILD_NOTES.md

tree -L 2 (or equivalent) recorded in same file

PHASE 1 — ANALYZE REPO (Inventory + Truth)

Write a script tools/sx9_rfc_inventory.rs (Rust) OR tools/sx9_rfc_inventory.py (only if Rust is impossible) that:

Scans 01-rfc/ recursively

Detects RFC files by regex: RFC-\d{4}.*\.(md|txt|docx)$

Computes:

sha256 file hash

byte size

last modified time

Outputs:

build/rfc-index.json

build/rfc-index.toml (minimal: rfc_number, title, path, sha256)

Prints a concise summary to stdout.

Add a make inventory or just inventory target (whichever the repo uses).

PHASE 2 — RESEARCH (Local Only, No Web)

Locate existing references to:

Zotero / BibTeX / Overleaf pipeline

any prior sx9-references directory mention

any “export to LaTeX” logic

any existing disclaimers or cover-page templates

Use ripgrep:

rg -n "zotero|bibtex|overleaf|latex|pdflatex|xelatex|biber|biblatex|citation|\\.bib" -S .
Capture findings in 02-sx9-latex/BUILD_NOTES.md under “Repo Findings”.

If Zotero pipeline is “broken”, do not guess—identify what’s missing and rebuild minimally.

PHASE 3 — PLAN (Write the plan before coding)

Create 02-sx9-latex/PLAN.md that includes:

Goals (as above)

Non-goals (no per-RFC sprawl, no RFC rewrites)

Proposed directory structure

Build commands

“Failure modes” section (fonts, missing TeX packages, docx conversion)

Commit plan (Commit A/B/C…)

Do not start implementation until PLAN.md exists.

PHASE 4 — IMPLEMENT (Deterministic Scaffold)
A) Directory Structure (create)
02-sx9-latex/
  README.md
  PLAN.md
  BUILD_NOTES.md
  templates/
    rfc.tex                # main template
    sx9.sty                # macros
  assets/
    cover_disclaimer.tex   # injected paragraph block
  scripts/
    build_rfc.sh           # builds one RFC by number
    inject_disclaimer.sh   # idempotent injection into generated LaTeX
  build/                   # ignored by git
sx9-references/
  README.md
  zotero/                  # optional: exported library or better-bibtex output
  bibtex/
    sx9.bib                # generated/curated
  scripts/
    build_bib.sh

B) Markdown → LaTeX conversion

Prefer pandoc if available.

If pandoc isn’t available, implement a minimal converter:

headings, code blocks, lists, basic tables

fallback: embed markdown as verbatim (last resort)
Document the chosen approach.

C) Build command

Implement:

./02-sx9-latex/scripts/build_rfc.sh 9027
Behavior:

Finds matching RFC file(s) for RFC-9027* under 01-rfc/

Converts to LaTeX intermediate in 02-sx9-latex/build/9027/

Injects disclaimer (if enabled)

Compiles PDF to 02-sx9-latex/build/9027/RFC-9027.pdf

Writes a build log file

D) Disclaimer injection (idempotent)

The disclaimer text must be a standalone LaTeX block kept in:

02-sx9-latex/assets/cover_disclaimer.tex

Injection method:

Insert after \begin{document} or in a cover section

Guard with marker comments:

% SX9_DISCLAIMER_BEGIN

% SX9_DISCLAIMER_END

If markers exist, do nothing.

Disclaimer content (verbatim meaning, wording can be tightened):

“As part of the Synaptix9 (SX9) transition, an automated name-change process was applied to legacy CTAS documents. Minor grammatical artifacts may be present and will be corrected on an ‘as-can’ basis.”

E) Zotero/BibTeX pipeline (minimal viable)

Create sx9-references/README.md explaining:

expected Zotero export format (Better BibTeX if used)

where exported .bib should be dropped

how build_bib.sh produces bibtex/sx9.bib

build_bib.sh should:

concatenate + de-dup .bib inputs

run basic lint (optional)

produce stable ordering (sort entries)

F) Make/Just targets

Add targets:

make inventory

make bib

make rfc RFC=9027

PHASE 5 — WALK-THROUGH (Operator Grade)

Write 02-sx9-latex/README.md with:

prerequisites (pandoc/texlive) and fallback behavior

“Build RFC-9027” exact command

where output PDF lands

how references work

how to add citations later (LaTeX-ready)

Also include a “Troubleshooting” section:

missing fonts

missing pandoc

latex compile errors

docx sources (what we do if an RFC is only docx)

PHASE 6 — TEST (Local)

Run:

inventory generation

bib build

build RFC-9027 end-to-end

build a second RFC (any other) as sanity check

Record outcomes in 02-sx9-latex/BUILD_NOTES.md with timestamps.

PHASE 7 — COMMIT (Clean, Logical)

Create commits in this order:

“Add RFC inventory index generator”

“Add SX9 LaTeX scaffold and RFC build script”

“Add sx9-references BibTeX pipeline”

“Add disclaimer injection + docs”

Each commit must include:

what changed

how to run

expected output

Finally:

git status clean

git log -5 --oneline shown

STOP CONDITIONS (Ask Me Only If Blocked)

Only stop and ask if:

build requires a tool not available and no fallback is possible

conflicts with existing repo standards you can’t infer

LaTeX compilation fails due to missing packages and you can’t vendor a minimal solution

Otherwise: execute the phases and produce the changes.