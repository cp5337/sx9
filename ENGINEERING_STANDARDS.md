# Engineering Standards & Workflows

**Version**: 1.0.0
**Status**: Adopted
**Enforcement**: Strict

This document defines the standard operating procedures for the SX9 Engineering Team. All Agents and Human Operators must adhere to these protocols to ensure technical correctness and auditability.

---

## 1. Linear Integration Protocol (The "Why")

Every unit of work must be traced to a **Linear Issue**.

- **Format**: `L-{ID}` (e.g., `L-101`)
- **Hierarchy**:
  - **Initiative (L1)**: Strategic Goal (e.g., "Operation Restoration").
  - **Objective (L2)**: Concrete Build (e.g., "Implement GLAF Engine").
  - **Task (L3)**: Atom of Work (e.g., "Fix RFC Headers").

## 2. Branching Strategy (The "Where")

We use a modified **Git Flow** centered on Feature Branches. **Direct commits to `main` are PROHIBITED** (except for initial repo initialization).

### Naming Convention

`{type}/{linear-id}/{short-description}`

- `type`:
  - `feat`: New feature (minor capability).
  - `fix`: Bug fix.
  - `chore`: Maintenance, refactoring, docs.
  - `perf`: Optimization.
- `linear-id`: The Ticket ID (e.g., `L-101`). Use `NO-ID` if hotfixing without ticket (discouraged).
- `short-description`: Kebab-case, 2-4 words.

**Examples**:

- `feat/L-101/implement-glaf-hashing`
- `fix/L-102/resolve-audit-links`
- `chore/L-103/upgrade-dependencies`

## 3. Commit Protocol (The "What")

We use **Conventional Commits** to enable automated changelogs and semantic versioning.

### Format

```
type(scope): subject

[optional body]

[optional footer(s)]
```

- **Type**: `feat`, `fix`, `chore`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`.
- **Scope**: The module affected (e.g., `glaf`, `rfc`, `api`, `ui`).
- **Subject**: Imperative mood, lowercase, no period (e.g., "add trivariate hashing support").
- **Footer**: Reference the Linear ticket.

**Example**:

```text
feat(glaf): implement trivariate hashing for nonagon nodes

- Implements Murmur3-64
- Adds Base96 encoding
- Updates RFC-9305 references

Closes L-101
```

## 4. Pull Request (PR) Lifecycle

1.  **Draft**: Open PR as "Draft" while WIP.
2.  **Title**: Match the Commit Subject (e.g., `feat(glaf): implement trivariate hashing`).
3.  **Description**:
    - **Context**: Link to Linear Ticket.
    - **Changes**: Bullet points of what changed.
    - **Verification**: How to test (commands, screenshots).
4.  **Review**:
    - **Automated**: CI must pass (Lint, Test, Audit).
    - **Manual**: Peer review (Agent or User) required before merge.
5.  **Merge**: Squash and Merge.

## 5. Agent Protocol (The "How")

When an AI Agent is tasked:

1.  **Read Ticket**: Understanding the Initiative/Objective.
2.  **Create Branch**: `git checkout -b ...`
3.  **Execute -> Verify**: Loop until tests pass.
4.  **Commit**: Atomic commits with proper messages.
5.  **Handover**: Inform User "Branch is ready for Review/Merge".

---

**Signed-off-by**:

- Antigravity (Agent)
- User (Lead Engineer)
