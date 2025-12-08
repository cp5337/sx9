# CTAS7 GLAF Browser

**Fork of Neo4j Browser for GLAF (Genome Link Analysis Fabric)**

## What This Is

This is a fork of [Neo4j Browser](https://github.com/neo4j/neo4j-browser) adapted to work with GLAF's SurrealDB backend instead of Neo4j. We keep the excellent property graph visualization but replace the inferior math with GLAF's superior:

- **Matroid rank calculation** (information independence)
- **Hawkes process** (self-exciting event clustering)
- **TETH entropy analysis** (threat heuristic scoring)
- **Trivariate hashing** (48-position Base96)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    GLAF Browser (This Fork)                 │
│              Neo4j Browser UI + Visualization               │
├─────────────────────────────────────────────────────────────┤
│  REPLACED:                                                  │
│  ├── services/bolt/* → services/glaf/*                     │
│  ├── Cypher queries → SurrealQL queries                    │
│  └── Neo4j driver → SurrealDB.js client                    │
│                                                             │
│  KEPT:                                                      │
│  ├── neo4j-arc/graph-visualization (D3 rendering)          │
│  ├── UI components (React)                                 │
│  └── Redux state management                                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      GLAF Core (SurrealDB)                  │
│                       Port 18019                            │
├─────────────────────────────────────────────────────────────┤
│  • ptcc_configurations (510 threat actors)                 │
│  • teth_entropy_analysis                                   │
│  • attack_scenarios                                        │
│  • osint_tools (1,284 tools)                              │
│  • threat_correlations                                     │
└─────────────────────────────────────────────────────────────┘
```

## Key Changes

### 1. Connection Layer (`src/shared/services/glaf/`)

- `glafConnection.ts` - SurrealDB WebSocket connection
- `glafClient.ts` - High-level GLAF API

### 2. Query Language

| Neo4j (Cypher) | GLAF (SurrealQL) |
|----------------|------------------|
| `MATCH (n:Person)` | `SELECT * FROM person` |
| `MATCH (a)-[r]->(b)` | `SELECT *, ->relates->* FROM a` |
| `CREATE (n:Node)` | `CREATE node:id SET ...` |
| `MERGE` | `UPSERT` |

### 3. GLAF-Specific Features

```typescript
// Get threat actors with TETH entropy
await glafClient.getThreatActors(50);

// Traverse graph with Hawkes intensity
await glafClient.traverseWithEntropy('threat_actor:volt_typhoon', 3);

// Calculate matroid rank for information independence
await glafClient.calculateMatroidRank(['node:1', 'node:2', 'node:3']);
```

## Development

```bash
# Install dependencies
yarn install

# Add SurrealDB client
yarn add surrealdb.js

# Start development server
yarn start

# Build for production
yarn build
```

## Configuration

Connect to GLAF Core:

```typescript
const client = new GlafClient({
  host: 'http://localhost:18019',
  namespace: 'ctas7',
  database: 'glaf',
  username: 'root',
  password: 'root',
});
```

## License

GPL-3.0 (inherited from Neo4j Browser)

## Credits

- Original Neo4j Browser: Neo4j Sweden AB
- GLAF Fork: CTAS-7 Team
- Matroid/Hawkes Math: RFC-9021 GLAF Specification


