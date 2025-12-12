/**
 * Plasma Cognitive Services Index (RFC-9021)
 *
 * Exports all Layer 1-4 cognitive inference services for Plasma integration.
 *
 * NVNN: Services index provides unified access to cognitive pipeline
 */

// Layer 1: Thalamic Filter (DistilBERT)
export {
  thalamicFilter,
  batchThalamicFilter,
  requiresFullProcessing,
  getReflexiveResponse,
  clearThalamicCache,
  getCacheStats as getThalamicCacheStats,
} from "./thalamic-filter"

// Layer 2a: ChromaDB Vector Search
export {
  PlasmaCollections,
  queryCollection,
  addDocuments,
  indexThreat,
  findSimilarThreats,
  findRelatedTechniques,
  findDetectionRules,
  ensureCollection,
  healthCheck as chromaHealthCheck,
  getCollectionStats,
} from "./chromadb-client"
export type { PlasmaCollection } from "./chromadb-client"

// Layer 3: Context Assembly
export {
  assembleContext,
  assembleQuickContext,
  formatContextForPrompt,
  getContextHash,
} from "./context-assembler"

// Layer 4: Phi-3 Inference
export {
  generateAnalysis,
  generateQuickSummary,
  streamAnalysis,
  clearPhi3Cache,
  getCacheStats as getPhi3CacheStats,
  runCognitivePipeline,
} from "./phi3-inference"

// Re-export types for convenience
export type {
  ThalamicOutput,
  GlafScores,
  Phi3Analysis,
  UnifiedContext,
  VectorSearchResult,
  MitreContext,
  CognitiveStatus,
} from "../../types/plasma"
