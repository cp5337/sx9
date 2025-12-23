# Intelligence System Architecture

The Prompt Forge Intelligence System provides real-time background queries to **Leptose** (Rust inference engine) and **ChromaDB** (vector store) for pattern suggestions, tool recommendations, and threat intelligence.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Prompt Forge UI                             │
│  ┌───────────────┐  ┌──────────────┐  ┌───────────────┐       │
│  │ Harness Tab   │  │ Code Editor  │  │ Intelligence  │       │
│  │ Persona Tab   │  │              │  │ Panel         │       │
│  │ Inference Tab │  │              │  │               │       │
│  └───────────────┘  └──────────────┘  └───────────────┘       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Intelligence Store (Zustand)                    │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Redux-style Reducer + Actions + Selectors + Middleware  │  │
│  │  - Debounced queries (500ms)                             │  │
│  │  - Connection management                                 │  │
│  │  - Result caching                                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    NATS Message Bus                              │
│  Subjects:                                                       │
│  - leptose.pattern.query  → Similar crate patterns             │
│  - leptose.tool.query     → Kali tool recommendations          │
│  - leptose.threat.query   → MITRE ATT&CK scenarios             │
│  - eei.query              → Knowledge graph Q&A                │
└─────────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
┌────────────────────────────┐  ┌──────────────────────────┐
│  Leptose (Rust Inference)  │  │  ChromaDB (Vector Store) │
│  - Pattern matching        │  │  - Interview embeddings  │
│  - Tool entropy (TETH)     │  │  - Tool descriptions     │
│  - Threat correlation      │  │  - Scenario vectors      │
└────────────────────────────┘  └──────────────────────────┘
```

## Key Components

### 1. Intelligence Store (`src/store/intelligenceStore.ts`)

Zustand wrapper around Redux-style reducer for simpler setup:

```typescript
const { dispatch } = useIntelligenceActions();

// Connect to services
dispatch(connectLeptose());
dispatch(connectChromaDB());

// Query patterns (debounced 500ms)
dispatch(queryPatterns("reactor pattern authentication", 5));
```

### 2. State Structure (`src/store/intelligence/types.ts`)

```typescript
interface IntelligenceState {
  leptose: LeptoseConnection;      // Rust inference status
  chromadb: ChromaDBConnection;    // Vector store status
  patterns: QueryState<PatternSuggestion>;  // Cached results
  tools: QueryState<ToolRecommendation>;
  threats: QueryState<ThreatScenario>;
  eei: { question, answer, loading, error };
}
```

### 3. Middleware (`src/store/intelligence/middleware.ts`)

Handles:
- **Debouncing** - 500ms delay on pattern/tool/threat queries
- **NATS messaging** - Publishes to message bus (placeholder for now)
- **Connection management** - Tracks service status
- **Response handling** - Updates state on query completion

### 4. Intelligence Panel (`src/components/IntelligencePanel.tsx`)

Bottom panel in Prompt Forge with 4 tabs:

| Tab | Purpose | Data Source |
|-----|---------|-------------|
| **Patterns** | Similar crate interviews from past forge sessions | ChromaDB embeddings |
| **Tools** | Kali tools ranked by TETH entropy | Leptose inference |
| **Threats** | MITRE ATT&CK scenarios matching query | Leptose + MITRE database |
| **EEI** | Knowledge graph Q&A (non-debounced) | Leptose reasoning |

## Query Types

### Pattern Suggestions

Find similar crate interviews based on vector similarity:

```typescript
dispatch(queryPatterns("reactor pattern for API gateway", 5));

// Returns:
interface PatternSuggestion {
  interview_id: string;          // UUID of similar interview
  pattern: string;               // "Reactor", "Gateway", etc.
  similarity: number;            // 0.0-1.0 vector similarity
  voice_narrative: string;       // First-person description
  metadata?: { created_at, forge_version };
}
```

**Use case:** User types "authentication with JWT" → Get similar past implementations

### Tool Recommendations

Kali tools ranked by relevance and entropy:

```typescript
dispatch(queryTools("network reconnaissance", 10));

// Returns:
interface ToolRecommendation {
  tool_name: string;             // "nmap", "metasploit"
  category: string;              // "Network Scanner"
  entropy: number;               // TETH score (0.0-1.0)
  similarity: number;            // Vector match to query
  why_relevant: string;          // Explanation
  capabilities?: string[];       // Tool features
}
```

**Use case:** User describes attack scenario → Get ranked tool suggestions

### Threat Scenarios

MITRE ATT&CK context for APT groups:

```typescript
dispatch(queryThreats("spear phishing", 5));

// Returns:
interface ThreatScenario {
  scenario_id: string;
  apt_group: string;             // "APT28", "Lazarus Group"
  techniques: string[];          // ["T1566.001", "T1059.001"]
  tools_used: string[];          // Known APT tools
  detection_rules: string[];     // Sigma/YARA rules
  description?: string;
}
```

**Use case:** User researching attack vector → Get APT context + detection rules

### EEI (Essential Elements of Information)

Knowledge graph reasoning (non-debounced, immediate):

```typescript
dispatch(askEEI("How does APT28 typically exfiltrate data?"));

// Returns:
interface EEIAnswer {
  answer: string;                // Natural language response
  confidence: number;            // 0.0-1.0 confidence
  sources: string[];             // Source documents
  graph_path: string[];          // Path through knowledge graph
  timestamp: number;
}
```

**Use case:** Specific intelligence questions requiring reasoning

## Integration with Prompt Forge

### Connection Status

Displayed in Intelligence Panel status bar:

```typescript
const leptoseStatus = selectLeptoseStatus({ intelligence });
const chromaStatus = selectChromaStatus({ intelligence });

// Status: "offline" | "connecting" | "ready" | "querying" | "error"
```

Status colors:
- **Green** - Ready
- **Yellow** - Querying
- **Red** - Error
- **Gray** - Offline

### Auto-Query on Text Changes

Intelligence Panel watches `harness.promptText` or editor content:

```typescript
useEffect(() => {
  if (query.length >= 3) {
    switch (activeTab) {
      case "patterns": dispatch(queryPatterns(query, 5)); break;
      case "tools": dispatch(queryTools(query, 10)); break;
      case "threats": dispatch(queryThreats(query, 5)); break;
      case "eei": dispatch(askEEI(query)); break;
    }
  }
}, [query, activeTab]);
```

Minimum 3 characters required to avoid noise.

### Applying Results

**Pattern Application:**
```typescript
const handleApplyPattern = (pattern: PatternSuggestion) => {
  setEditorContent(pattern.voice_narrative);
};
```

**Tool Selection:**
```typescript
const handleSelectTool = (tool: ToolRecommendation) => {
  setHarnessField("promptText", `${harness.promptText}\n\nTool: ${tool.tool_name}`);
};
```

## NATS Message Bus

### Current Status: Placeholder

Middleware currently logs actions but doesn't connect to actual NATS. To implement:

```typescript
import { connect } from 'nats';

const natsConnection = await connect({ 
  servers: 'nats://localhost:4222' 
});

// Publish query
natsConnection.publish('leptose.pattern.query', JSON.stringify({
  text: query,
  nResults: 5
}));

// Subscribe to responses
const sub = natsConnection.subscribe('leptose.pattern.response');
for await (const msg of sub) {
  const response = JSON.parse(msg.data);
  dispatch(patternsSuccess(response));
}
```

### NATS Subject Convention

```
leptose.pattern.query     → leptose.pattern.response
leptose.tool.query        → leptose.tool.response
leptose.threat.query      → leptose.threat.response
eei.query                 → eei.response
```

## Performance Characteristics

| Operation | Latency | Caching | Debounce |
|-----------|---------|---------|----------|
| Pattern Query | ~50-200ms | Yes | 500ms |
| Tool Query | ~50-200ms | Yes | 500ms |
| Threat Query | ~100-300ms | Yes | 500ms |
| EEI Query | ~200-500ms | No | None |

### Latency Tracking

```typescript
const latency = selectLatestLatency({ intelligence });
// Used for performance monitoring
```

## Selectors (Optimized Access)

Instead of direct state access, use memoized selectors:

```typescript
import { 
  selectPatternResults, 
  selectIsQuerying, 
  selectAreServicesReady 
} from '../store/intelligence/selectors';

const patterns = selectPatternResults({ intelligence });
const isQuerying = selectIsQuerying({ intelligence });
const servicesReady = selectAreServicesReady({ intelligence });
```

## Error Handling

Each query type has error state:

```typescript
patterns: {
  query: string | null,
  results: PatternSuggestion[],
  loading: boolean,
  error: string | null  // Error message if query failed
}
```

Display errors in UI:

```typescript
{patterns.error && (
  <Text style={styles.errorText}>{patterns.error}</Text>
)}
```

## Testing

### Mock Responses

Middleware includes placeholder responses for development:

```typescript
// Pattern query returns empty array
dispatch(patternsSuccess({
  query: "test query",
  results: [],
  latencyMs: 50
}));
```

### Testing Connection

```typescript
// Connect both services
dispatch(connectLeptose());
dispatch(connectChromaDB());

// Check status
const leptoseReady = selectIsLeptoseReady({ intelligence });
const chromaReady = selectIsChromaReady({ intelligence });
```

## Future Enhancements

1. **WebSocket Support** - Real-time streaming for EEI answers
2. **Persistent Caching** - Store results in IndexedDB
3. **Feedback Loop** - Users rate result relevance
4. **Multi-language** - Support for YAML, JSON, Python prompts
5. **Graph Visualization** - Render knowledge graph paths
6. **Export Results** - Save intelligence reports

## Related Files

```
src/store/intelligence/
├── types.ts           # TypeScript interfaces
├── actions.ts         # Action creators
├── reducer.ts         # State updates
├── selectors.ts       # Memoized accessors
└── middleware.ts      # NATS integration

src/store/
└── intelligenceStore.ts   # Zustand wrapper

src/components/
└── IntelligencePanel.tsx  # UI component

src/lib/
├── graphActions.ts    # Nonagon/cross-reference generation
├── graphCRUD.ts       # Supabase graph operations
└── validators.ts      # Graph validation utilities
```

## Best Practices

1. **Always use selectors** - Never access state directly
2. **Debounce user input** - Middleware handles query debouncing
3. **Check connection status** - Don't query if services offline
4. **Cache aggressively** - Results are stored until next query
5. **Handle errors gracefully** - Show user-friendly error messages
6. **Monitor latency** - Track performance for optimization

---

**The intelligence system transforms Prompt Forge from a static editor into an active reasoning assistant, providing context-aware suggestions based on historical patterns, tool capabilities, and threat intelligence.**
