# SX9 Canonical Pattern Discovery + QA (End-to-End Bundle)

This bundle implements the locked v1.0 architecture:

- Canonical blocks (read-only, N-V-N-N header)
- OrbStack A: Static QA (cold truth)
- OrbStack B: Semantic/archaeology (warm annotation)
- OrbStack C: Canonical discovery + GLAF + vector/memory plumbing (hot intelligence)
- Agent + sub-agent prompt specs (Prompt Forge / RFC-9112 compatible)
- GLAF schemas (JSON) for findings, pattern matches, graph deltas, load sets
- Neo4j schema (Cypher constraints + recommended labels/relationships)

## Quick start (local)
1) Put your repo you want to analyze at: `./work/repo`
2) Run in order:
   - `./orbstack-static/run.sh`
   - `./orbstack-semantic/run.sh`
   - `./orbstack-discovery/run.sh`
3) Load Neo4j schema:
   - `cat ./neo4j/schema.cypher | cypher-shell -u neo4j -p <password>`

## Invariants (non-negotiable)
- Agents never modify code.
- Canonical blocks are the only autofill seeds.
- Static QA outputs are treated as fact.
- Semantic outputs are annotation only.
- Memory stores patterns/constraints/telemetry; never raw source code.

## Directories
- `sx9-canonical/blocks/`  Canonical blocks (read-only)
- `orbstack-static/`       Cold truth stack
- `orbstack-semantic/`     Semantic/archaeology stack
- `orbstack-discovery/`    Canonical discovery + GLAF + vector/memory stack
- `agents/`                Primary agent + sub-agent prompt specs
- `schemas/`               JSON schemas (contracts)
- `neo4j/`                 Cypher schema + recommended model
- `work/`                  Mount points for your target repo + outputs

## Notes
- Container images in compose files are placeholders (sx9/*:latest). Wire them to your real images.
- Reference scripts under `reference_impl/` show deterministic file contracts and can be swapped with Rust binaries.

## Optional Neo4j OrbStack
- Start Neo4j + auto-load schema:
  `cd orbstack-neo4j && ./run.sh`

## Output Validation
- Validate discovery outputs:
  `reference_impl/sx9-validate . work/discovery`
