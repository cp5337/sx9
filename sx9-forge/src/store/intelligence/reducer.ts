/**
 * Redux Intelligence Reducer
 *
 * Manages state for Leptose/ChromaDB background intelligence queries.
 * All queries are non-blocking and cached for performance.
 */

import type { AnyAction } from "@reduxjs/toolkit"
import type { IntelligenceState } from "./types"
import { intelligenceActions } from "./actions"

// ============================================================================
// Initial State
// ============================================================================

const initialState: IntelligenceState = {
  // Leptose connection (Rust inference engine)
  leptose: {
    status: "offline",
    lastQuery: null,
    latencyMs: null,
    error: null,
  },

  // ChromaDB connection (vector store)
  chromadb: {
    status: "offline",
    collections: [],
    lastQuery: null,
    latencyMs: null,
    error: null,
  },

  // Pattern suggestions (similar interviews)
  patterns: {
    query: null,
    results: [],
    loading: false,
    error: null,
  },

  // Tool recommendations (Kali tools)
  tools: {
    query: null,
    results: [],
    loading: false,
    error: null,
  },

  // Threat scenarios (MITRE ATT&CK)
  threats: {
    query: null,
    results: [],
    loading: false,
    error: null,
  },

  // EEI (Essential Elements of Information)
  eei: {
    question: null,
    answer: null,
    loading: false,
    error: null,
  },
}

// ============================================================================
// Reducer
// ============================================================================

export function intelligenceReducer(state = initialState, action: AnyAction): IntelligenceState {
  switch (action.type) {
    // ========================================================================
    // Leptose Connection
    // ========================================================================

    case intelligenceActions.LEPTOSE_CONNECT:
      return {
        ...state,
        leptose: { ...state.leptose, status: "connecting", error: null },
      }

    case intelligenceActions.LEPTOSE_CONNECTED:
      return {
        ...state,
        leptose: { ...state.leptose, status: "ready", error: null },
      }

    case intelligenceActions.LEPTOSE_DISCONNECTED:
      return {
        ...state,
        leptose: { ...state.leptose, status: "offline", error: null },
      }

    case intelligenceActions.LEPTOSE_ERROR:
      return {
        ...state,
        leptose: { ...state.leptose, status: "error", error: action.payload },
      }

    // ========================================================================
    // ChromaDB Connection
    // ========================================================================

    case intelligenceActions.CHROMADB_CONNECT:
      return {
        ...state,
        chromadb: { ...state.chromadb, status: "connecting", error: null },
      }

    case intelligenceActions.CHROMADB_CONNECTED:
      return {
        ...state,
        chromadb: {
          ...state.chromadb,
          status: "ready",
          collections: action.payload.collections,
          error: null,
        },
      }

    case intelligenceActions.CHROMADB_DISCONNECTED:
      return {
        ...state,
        chromadb: { ...state.chromadb, status: "offline", error: null },
      }

    case intelligenceActions.CHROMADB_ERROR:
      return {
        ...state,
        chromadb: { ...state.chromadb, status: "error", error: action.payload },
      }

    // ========================================================================
    // Pattern Queries
    // ========================================================================

    case intelligenceActions.PATTERNS_LOADING:
      return {
        ...state,
        leptose: { ...state.leptose, status: "querying" },
        patterns: { ...state.patterns, loading: true, error: null },
      }

    case intelligenceActions.PATTERNS_SUCCESS:
      return {
        ...state,
        leptose: {
          ...state.leptose,
          status: "ready",
          lastQuery: Date.now(),
          latencyMs: action.payload.latencyMs,
        },
        patterns: {
          query: action.payload.query,
          results: action.payload.results,
          loading: false,
          error: null,
        },
      }

    case intelligenceActions.PATTERNS_ERROR:
      return {
        ...state,
        leptose: { ...state.leptose, status: "ready" },
        patterns: {
          ...state.patterns,
          loading: false,
          error: action.payload,
        },
      }

    // ========================================================================
    // Tool Queries
    // ========================================================================

    case intelligenceActions.TOOLS_LOADING:
      return {
        ...state,
        leptose: { ...state.leptose, status: "querying" },
        tools: { ...state.tools, loading: true, error: null },
      }

    case intelligenceActions.TOOLS_SUCCESS:
      return {
        ...state,
        leptose: {
          ...state.leptose,
          status: "ready",
          lastQuery: Date.now(),
          latencyMs: action.payload.latencyMs,
        },
        tools: {
          query: action.payload.query,
          results: action.payload.results,
          loading: false,
          error: null,
        },
      }

    case intelligenceActions.TOOLS_ERROR:
      return {
        ...state,
        leptose: { ...state.leptose, status: "ready" },
        tools: {
          ...state.tools,
          loading: false,
          error: action.payload,
        },
      }

    // ========================================================================
    // Threat Queries
    // ========================================================================

    case intelligenceActions.THREATS_LOADING:
      return {
        ...state,
        leptose: { ...state.leptose, status: "querying" },
        threats: { ...state.threats, loading: true, error: null },
      }

    case intelligenceActions.THREATS_SUCCESS:
      return {
        ...state,
        leptose: {
          ...state.leptose,
          status: "ready",
          lastQuery: Date.now(),
          latencyMs: action.payload.latencyMs,
        },
        threats: {
          query: action.payload.query,
          results: action.payload.results,
          loading: false,
          error: null,
        },
      }

    case intelligenceActions.THREATS_ERROR:
      return {
        ...state,
        leptose: { ...state.leptose, status: "ready" },
        threats: {
          ...state.threats,
          loading: false,
          error: action.payload,
        },
      }

    // ========================================================================
    // EEI Queries
    // ========================================================================

    case intelligenceActions.EEI_LOADING:
      return {
        ...state,
        leptose: { ...state.leptose, status: "querying" },
        eei: { ...state.eei, loading: true, error: null },
      }

    case intelligenceActions.EEI_ANSWER:
      return {
        ...state,
        leptose: {
          ...state.leptose,
          status: "ready",
          lastQuery: Date.now(),
          latencyMs: action.payload.latencyMs,
        },
        eei: {
          question: action.payload.question,
          answer: action.payload.answer,
          loading: false,
          error: null,
        },
      }

    case intelligenceActions.EEI_ERROR:
      return {
        ...state,
        leptose: { ...state.leptose, status: "ready" },
        eei: {
          ...state.eei,
          loading: false,
          error: action.payload,
        },
      }

    // ========================================================================
    // Default
    // ========================================================================

    default:
      return state
  }
}
