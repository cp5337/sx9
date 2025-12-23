/**
 * Redux Intelligence Actions
 *
 * Action creators for Leptose/ChromaDB intelligence queries.
 * All queries are debounced in middleware to prevent excessive API calls.
 */

import type {
  QueryPatternsPayload,
  QueryToolsPayload,
  QueryThreatsPayload,
  AskEEIPayload,
  PatternsSuccessPayload,
  ToolsSuccessPayload,
  ThreatsSuccessPayload,
  EEIAnswerPayload,
  PatternSuggestion,
} from "./types"

// ============================================================================
// Action Types (String Constants)
// ============================================================================

export const intelligenceActions = {
  // Connection management
  LEPTOSE_CONNECT: "intelligence/leptose/connect",
  LEPTOSE_CONNECTED: "intelligence/leptose/connected",
  LEPTOSE_DISCONNECTED: "intelligence/leptose/disconnected",
  LEPTOSE_ERROR: "intelligence/leptose/error",

  CHROMADB_CONNECT: "intelligence/chromadb/connect",
  CHROMADB_CONNECTED: "intelligence/chromadb/connected",
  CHROMADB_DISCONNECTED: "intelligence/chromadb/disconnected",
  CHROMADB_ERROR: "intelligence/chromadb/error",

  // Pattern queries
  QUERY_PATTERNS: "intelligence/patterns/query",
  PATTERNS_LOADING: "intelligence/patterns/loading",
  PATTERNS_SUCCESS: "intelligence/patterns/success",
  PATTERNS_ERROR: "intelligence/patterns/error",
  PATTERN_APPLY: "intelligence/patterns/apply",

  // Tool queries
  QUERY_TOOLS: "intelligence/tools/query",
  TOOLS_LOADING: "intelligence/tools/loading",
  TOOLS_SUCCESS: "intelligence/tools/success",
  TOOLS_ERROR: "intelligence/tools/error",
  TOOL_SELECT: "intelligence/tools/select",

  // Threat queries
  QUERY_THREATS: "intelligence/threats/query",
  THREATS_LOADING: "intelligence/threats/loading",
  THREATS_SUCCESS: "intelligence/threats/success",
  THREATS_ERROR: "intelligence/threats/error",

  // EEI queries
  EEI_ASK: "intelligence/eei/ask",
  EEI_LOADING: "intelligence/eei/loading",
  EEI_ANSWER: "intelligence/eei/answer",
  EEI_ERROR: "intelligence/eei/error",

  // Status bar
  REFRESH_STATUS: "intelligence/status/refresh",
} as const

// ============================================================================
// Connection Action Creators
// ============================================================================

export const connectLeptose = () => ({
  type: intelligenceActions.LEPTOSE_CONNECT,
})

export const leptoseConnected = () => ({
  type: intelligenceActions.LEPTOSE_CONNECTED,
})

export const leptoseDisconnected = () => ({
  type: intelligenceActions.LEPTOSE_DISCONNECTED,
})

export const leptoseError = (error: string) => ({
  type: intelligenceActions.LEPTOSE_ERROR,
  payload: error,
})

export const connectChromaDB = () => ({
  type: intelligenceActions.CHROMADB_CONNECT,
})

export const chromaDBConnected = (collections: string[]) => ({
  type: intelligenceActions.CHROMADB_CONNECTED,
  payload: { collections },
})

export const chromaDBDisconnected = () => ({
  type: intelligenceActions.CHROMADB_DISCONNECTED,
})

export const chromaDBError = (error: string) => ({
  type: intelligenceActions.CHROMADB_ERROR,
  payload: error,
})

// ============================================================================
// Pattern Query Action Creators
// ============================================================================

/**
 * Query similar patterns from ChromaDB
 * Debounced in middleware (500ms)
 */
export const queryPatterns = (text: string, nResults = 5) => ({
  type: intelligenceActions.QUERY_PATTERNS,
  payload: { text, nResults } as QueryPatternsPayload,
})

export const patternsLoading = () => ({
  type: intelligenceActions.PATTERNS_LOADING,
})

export const patternsSuccess = (payload: PatternsSuccessPayload) => ({
  type: intelligenceActions.PATTERNS_SUCCESS,
  payload,
})

export const patternsError = (error: string) => ({
  type: intelligenceActions.PATTERNS_ERROR,
  payload: error,
})

/**
 * Apply a pattern suggestion to the current interview
 * This will populate the interview form with the pattern's data
 */
export const applyPattern = (pattern: PatternSuggestion) => ({
  type: intelligenceActions.PATTERN_APPLY,
  payload: pattern,
})

// ============================================================================
// Tool Query Action Creators
// ============================================================================

/**
 * Query tool recommendations from Leptose
 * Debounced in middleware (500ms)
 */
export const queryTools = (text: string, nResults = 10) => ({
  type: intelligenceActions.QUERY_TOOLS,
  payload: { text, nResults } as QueryToolsPayload,
})

export const toolsLoading = () => ({
  type: intelligenceActions.TOOLS_LOADING,
})

export const toolsSuccess = (payload: ToolsSuccessPayload) => ({
  type: intelligenceActions.TOOLS_SUCCESS,
  payload,
})

export const toolsError = (error: string) => ({
  type: intelligenceActions.TOOLS_ERROR,
  payload: error,
})

/**
 * Select a tool recommendation
 * This will add the tool to the interview's capabilities
 */
export const selectTool = (toolName: string) => ({
  type: intelligenceActions.TOOL_SELECT,
  payload: { toolName },
})

// ============================================================================
// Threat Query Action Creators
// ============================================================================

/**
 * Query threat scenarios from Leptose
 * Debounced in middleware (500ms)
 */
export const queryThreats = (text: string, nResults = 5) => ({
  type: intelligenceActions.QUERY_THREATS,
  payload: { text, nResults } as QueryThreatsPayload,
})

export const threatsLoading = () => ({
  type: intelligenceActions.THREATS_LOADING,
})

export const threatsSuccess = (payload: ThreatsSuccessPayload) => ({
  type: intelligenceActions.THREATS_SUCCESS,
  payload,
})

export const threatsError = (error: string) => ({
  type: intelligenceActions.THREATS_ERROR,
  payload: error,
})

// ============================================================================
// EEI Action Creators
// ============================================================================

/**
 * Ask an intelligence question (EEI - Essential Elements of Information)
 * Not debounced - immediate query
 */
export const askEEI = (question: string) => ({
  type: intelligenceActions.EEI_ASK,
  payload: { question } as AskEEIPayload,
})

export const eeiLoading = () => ({
  type: intelligenceActions.EEI_LOADING,
})

export const eeiAnswer = (payload: EEIAnswerPayload) => ({
  type: intelligenceActions.EEI_ANSWER,
  payload,
})

export const eeiError = (error: string) => ({
  type: intelligenceActions.EEI_ERROR,
  payload: error,
})

// ============================================================================
// Status Action Creators
// ============================================================================

/**
 * Refresh connection status for both Leptose and ChromaDB
 */
export const refreshStatus = () => ({
  type: intelligenceActions.REFRESH_STATUS,
})
