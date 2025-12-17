# sx9-glaf-core

**Graph Learning & Analytics Fabric - Core Engine**

This crate implements the in-memory graph structure and basic query operations for GLAF.

> **ℹ️ Documentation**: Full architecture documentation is available in `../../docs/glaf`.
> See [GLAF Architecture](../../docs/glaf/README.md).

## Features

- **Dual-Layer Node Model**: Supports `Node` (User) and `InternalNode` (System) types.
- **Discriminated Union Changes**: `NodeChange` enum for efficient, type-safe graph updates.
- **Batch Processing**: `apply_changes` function for transactional updates.
- **Serialization**: Full JSON support via `serde` for persistence.
