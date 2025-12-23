/**
 * Redux Intelligence Selectors
 *
 * Memoized selectors for accessing intelligence state.
 * Use these in components instead of direct state access.
 */

import type { IntelligenceState } from "./types"

// ============================================================================
// Root Selector
// ============================================================================

/**
 * Select the entire intelligence state
 * Use this as the base for other selectors
 */
export const selectIntelligence = (state: {
  intelligence: IntelligenceState
}) => state.intelligence

// ============================================================================
// Connection Selectors
// ============================================================================

export const selectLeptoseStatus = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.leptose

export const selectChromaStatus = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.chromadb

export const selectIsLeptoseReady = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.leptose.status === "ready"

export const selectIsChromaReady = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.chromadb.status === "ready"

export const selectAreServicesReady = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.leptose.status === "ready" && state.intelligence.chromadb.status === "ready"

// ============================================================================
// Pattern Selectors
// ============================================================================

export const selectPatterns = (state: { intelligence: IntelligenceState }) => state.intelligence.patterns

export const selectPatternResults = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.patterns.results

export const selectPatternsLoading = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.patterns.loading

export const selectHasPatterns = (state: { intelligence: IntelligenceState }) =>
  state.intelligence.patterns.results.length > 0

// ============================================================================
// Tool Selectors
// ============================================================================

export const selectTools = (state: { intelligence: IntelligenceState }) => state.intelligence.tools

export const selectToolResults = (state: { intelligence: IntelligenceState }) => state.intelligence.tools.results

export const selectToolsLoading = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.tools.loading

export const selectHasTools = (state: { intelligence: IntelligenceState }) =>
  state.intelligence.tools.results.length > 0

// ============================================================================
// Threat Selectors
// ============================================================================

export const selectThreats = (state: { intelligence: IntelligenceState }) => state.intelligence.threats

export const selectThreatResults = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.threats.results

export const selectThreatsLoading = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.threats.loading

export const selectHasThreats = (state: { intelligence: IntelligenceState }) =>
  state.intelligence.threats.results.length > 0

// ============================================================================
// EEI Selectors
// ============================================================================

export const selectEEI = (state: { intelligence: IntelligenceState }) => state.intelligence.eei

export const selectEEIAnswer = (state: { intelligence: IntelligenceState }) => state.intelligence.eei.answer

export const selectEEILoading = (state: { intelligence: IntelligenceState }) => state.intelligence.eei.loading

export const selectHasEEIAnswer = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.eei.answer !== null

// ============================================================================
// Composite Selectors
// ============================================================================

/**
 * Check if any query is currently loading
 */
export const selectIsQuerying = (state: { intelligence: IntelligenceState }) =>
  state.intelligence.patterns.loading ||
  state.intelligence.tools.loading ||
  state.intelligence.threats.loading ||
  state.intelligence.eei.loading

/**
 * Check if there are any results to display
 */
export const selectHasAnyResults = (state: {
  intelligence: IntelligenceState
}) =>
  state.intelligence.patterns.results.length > 0 ||
  state.intelligence.tools.results.length > 0 ||
  state.intelligence.threats.results.length > 0 ||
  state.intelligence.eei.answer !== null

/**
 * Get the most recent query latency (for performance monitoring)
 */
export const selectLatestLatency = (state: {
  intelligence: IntelligenceState
}) => state.intelligence.leptose.latencyMs || state.intelligence.chromadb.latencyMs
