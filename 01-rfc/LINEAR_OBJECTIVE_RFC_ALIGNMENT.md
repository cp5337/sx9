# [OBJ] RFC Infrastructure & Compliance Alignment

## Operational Concept (SV-1)

Establish a robust infrastructure for the **System RFC Library** to ensure long-term consistency, alignment, and enforceability. This addresses "Item 9" of the Integration Plan.

The goal is to transition RFCs from "Static Documents" to "Living Contracts" that are verified by CI/CD.

## Tech Stack

- **Linting**: `markdownlint` (Structure), `cspell` (Terminology).
- **Testing**: Python/Rust Harness to parse RFCs and extract requirements.
- **Cross-Ref**: Automated link checker for `RFC-XXXX` citations (e.g. ensure 9100 links to 9305 correctly).
- **Git**: Pre-commit hooks for RFC formatting.

## Scope of Work

1.  **Alignment Audit**:
    - [ ] Scan all 26+ RFCs for broken links.
    - [ ] Verify "Status" fields (Draft vs Final).
    - [ ] Verify "Next Available" in Registry matches reality.

2.  **Infrastructure Implementation**:
    - [ ] Implement `scripts/rfc-lint.sh`.
    - [ ] Implement `scripts/rfc-graph.py` (Visualize dependencies).
    - [ ] Add GitHub Action / Pre-commit hook.

3.  **Content Compliance**:
    - [ ] update `RFC-9130` (Kali) to explicitly reference `RFC-9305` (Converge).
    - [ ] Ensure `RFC-9001` (Hashing) is cited in all Data Models.

## Success Criteria

- [ ] `npm run rfc:check` passes with 0 errors.
- [ ] Dependency Graph generated.
- [ ] All "Draft" RFCs have a clear owner.
