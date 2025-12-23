/**
 * Redux Intelligence Store Types
 *
 * State management for Leptose (Rust inference) and ChromaDB (vector store)
 * background intelligence queries. All queries are debounced and non-blocking.
 */

// ============================================================================
// Connection Status Types
// ============================================================================

export type ConnectionStatus =
  | "offline" // Not connected
  | "connecting" // Attempting connection
  | "ready" // Connected and idle
  | "querying" // Currently processing query
  | "error" // Connection or query error

export interface LeptoseConnection {
  status: ConnectionStatus
  lastQuery: number | null // Timestamp of last query
  latencyMs: number | null // Last query latency
  error: string | null // Error message if status === 'error'
}

export interface ChromaDBConnection {
  status: ConnectionStatus
  collections: string[] // Available collections
  lastQuery: number | null
  latencyMs: number | null
  error: string | null
}

// ============================================================================
// Query Result Types
// ============================================================================

/**
 * Pattern Suggestion - Similar crate interviews from ChromaDB
 */
export interface PatternSuggestion {
  interview_id: string // UUID of similar interview
  pattern: string // Design pattern (Reactor, Gateway, etc.)
  similarity: number // Vector similarity score (0.0-1.0)
  voice_narrative: string // First-person narrative from interview
  metadata?: {
    created_at?: string
    forge_version?: string
  }
}

/**
 * Tool Recommendation - Kali tools ranked by relevance
 */
export interface ToolRecommendation {
  tool_name: string // e.g., "nmap", "metasploit"
  category: string // e.g., "Network Scanner", "Exploit Framework"
  entropy: number // TETH score (0.0-1.0)
  similarity: number // Vector similarity to query
  why_relevant: string // Explanation of relevance
  capabilities?: string[] // Tool capabilities
}

/**
 * Threat Scenario - MITRE ATT&CK context
 */
export interface ThreatScenario {
  scenario_id: string // UUID
  apt_group: string // e.g., "APT28", "Lazarus Group"
  techniques: string[] // MITRE ATT&CK IDs (e.g., "T1566.001")
  tools_used: string[] // Tools associated with this APT
  detection_rules: string[] // Sigma/YARA rules
  description?: string // Scenario description
}

/**
 * EEI Answer - Knowledge graph query response
 */
export interface EEIAnswer {
  answer: string // Natural language answer
  confidence: number // Confidence score (0.0-1.0)
  sources: string[] // Source documents/nodes
  graph_path: string[] // Path through knowledge graph
  timestamp: number // When answer was generated
}

// ============================================================================
// Query State Types
// ============================================================================

export interface QueryState<T> {
  query: string | null // Last query text
  results: T[] // Query results
  loading: boolean // Currently loading
  error: string | null // Error message if failed
}

// ============================================================================
// Main Intelligence State
// ============================================================================

export interface IntelligenceState {
  // Connection status
  leptose: LeptoseConnection
  chromadb: ChromaDBConnection

  // Query results (cached)
  patterns: QueryState<PatternSuggestion>
  tools: QueryState<ToolRecommendation>
  threats: QueryState<ThreatScenario>

  // EEI (Essential Elements of Information) - single Q&A
  eei: {
    question: string | null
    answer: EEIAnswer | null
    loading: boolean
    error: string | null
  }
}

// ============================================================================
// Action Payload Types
// ============================================================================

export interface QueryPatternsPayload {
  text: string
  nResults?: number // Default: 5
}

export interface QueryToolsPayload {
  text: string
  nResults?: number // Default: 10
}

export interface QueryThreatsPayload {
  text: string
  nResults?: number // Default: 5
}

export interface AskEEIPayload {
  question: string
}

export interface PatternsSuccessPayload {
  query: string
  results: PatternSuggestion[]
  latencyMs: number
}

export interface ToolsSuccessPayload {
  query: string
  results: ToolRecommendation[]
  latencyMs: number
}

export interface ThreatsSuccessPayload {
  query: string
  results: ThreatScenario[]
  latencyMs: number
}

export interface EEIAnswerPayload {
  question: string
  answer: EEIAnswer
  latencyMs: number
}
