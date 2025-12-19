1. Repo Facts

Repo root path

Current branch

Primary languages detected (ranked)

Existing build system(s)

Existing scripting conventions

2. RFC Reality Check

Confirm 01-rfc/ exists and is canonical

Count RFC files by extension (.md / .txt / .docx)

Note any RFCs outside 01-rfc/ (do not move them)

3. Tooling Compatibility Matrix
Capability	Found?	Notes
Make / Just	Yes / No	
Rust toolchain	Yes / No	
Python	Yes / No	
Pandoc	Yes / No	
TeX Live	Yes / No	
Existing BibTeX	Yes / No	
4. Constraints Discovered

Explicitly list:

anything that would break if you assumed the wrong build system

any scripts you must integrate with instead of replacing

anything that suggests Rust over Python (or vice versa)

ADAPTATION RULE

After writing REPO_DISCOVERY.md:

You MUST adapt the RFC → LaTeX pipeline plan to the actual repo, not vice-versa.

Examples:

If the repo already uses cargo xtask, add an xtask instead of a bash script

If justfile exists, add Just targets instead of Make

If Python is dominant, Rust is allowed only for performance-critical pieces

If no TeX is installed, support a “LaTeX emit only” mode

Document every adaptation decision in 02-sx9-latex/BUILD_NOTES.md.

RESUME MAIN PIPELINE PROMPT

Once the scan and adaptation notes are complete, resume execution of:

CURSOR MASTER RUN PROMPT — SX9 RFC PIPELINE (PULL→ANALYZE→RESEARCH→PLAN→IMPLEMENT→WALK→COMMIT)

starting at PHASE 1, but using real repo facts.

WHY THIS MATTERS (DO NOT SKIP)

SX9 is:

multi-year

multi-persona

multi-domain

legally sensitive (IP + academic)

A wrong assumption early creates silent entropy later.

You are acting as:

a release engineer for a cognitive system, not a README generator.