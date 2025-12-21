# [OBJ] GLAF Converge Implementation & Hashing Standard

## Operational Concept (SV-1)

Implementation of the **Genome Link Analysis Fabric (GLAF)** Core Engine and the **Converge** logic layer, underpinned by the **Trivariate Hashing Standard**.

This objective covers:

1.  **RFC Restoration**: Recovery of the "Lost" 93XX series (Core, Geometry, Selection, Integration).
2.  **Numbering Fix**: Canonical indexing in `REGISTRY.md` (9305-9308).
3.  **Hashing Standard**: Implementation of `RFC-9001` (Murmur3-64 + Base96) with standard seeds (`0xC7A5...`).
4.  **Rust Core**: Detailed `glaf-core` crate structure (NonagonNode, TethEdge).

## Tech Stack

- **Language**: Rust (Edition 2021)
- **Storage**: `sled` (Embedded KVS)
- **Graph**: `petgraph` (Adjacency)
- **Hashing**: `murmur3` + `uuid` (v7)

## RFC Reference

- **RFC-9001**: Trivariate Hashing Standard (Final)
- **RFC-9304**: GLAF Graph Engine Specification (Draft -> Implemented)
- **RFC-9305**: CONVERGE Core Specification (Draft)
- **RFC-9306**: CONVERGE Geometry Specification (Draft)
- **RFC-9307**: CONVERGE Selection Logic (Draft)
- **RFC-9308**: CONVERGE Integration Standard (Draft)

## Implementation Status

- [x] Restore RFCs from Bundle
- [x] Update Registry No.
- [x] Verify Hashing Standard Compliance (RFC-9305 §7)
- [x] Create `glaf-core` Crate (Placeholder/Structure)
