/**
 * Layer 3: Context Assembly Service (RFC-9021)
 *
 * Merges outputs from Layers 1 and 2 into unified threat context.
 * Target latency: <10ms P95, <20ms P99
 *
 * Assembles:
 * - Thalamic output (Layer 1) - gate decision, pathway, priority
 * - ChromaDB results (Layer 2a) - similar threats, techniques
 * - GNN embeddings (Layer 2b) - 768-dim relationship vectors
 * - GLAF scores (Layer 2c) - H1/H2 convergence metrics
 *
 * NVNN: ContextAssembler merges Layer 1-2 outputs for Layer 4 generation
 */

import type {
  UnifiedContext,
  Threat,
  ThalamicOutput,
  GlafScores,
  VectorSearchResult,
  MitreContext,
  CognitiveDomain,
} from "../../types/plasma"
import { thalamicFilter, requiresFullProcessing } from "./thalamic-filter"
import { findSimilarThreats, findRelatedTechniques } from "./chromadb-client"

// NVNN: ContextAssemblyConfig configures assembly behavior
interface ContextAssemblyConfig {
  maxSimilarThreats: number
  maxTechniques: number
  minSimilarityScore: number
  enableGnn: boolean
  enableGlaf: boolean
}

const DEFAULT_CONFIG: ContextAssemblyConfig = {
  maxSimilarThreats: 5,
  maxTechniques: 3,
  minSimilarityScore: 0.7,
  enableGnn: false, // GNN fabric not yet connected
  enableGlaf: true,
}

// NVNN: MitreMapping provides technique metadata lookup
const MITRE_TACTICS: Record<string, string> = {
  TA0001: "Initial Access",
  TA0002: "Execution",
  TA0003: "Persistence",
  TA0004: "Privilege Escalation",
  TA0005: "Defense Evasion",
  TA0006: "Credential Access",
  TA0007: "Discovery",
  TA0008: "Lateral Movement",
  TA0009: "Collection",
  TA0010: "Exfiltration",
  TA0011: "Command and Control",
  TA0040: "Impact",
}

/**
 * Extract tactic from technique ID
 * NVNN: getTacticForTechnique maps technique to tactic
 */
function getTacticForTechnique(techniqueId: string): string {
  // Common technique-to-tactic mappings
  const tacticMap: Record<string, string> = {
    T1190: "Initial Access",
    T1059: "Execution",
    T1053: "Execution",
    T1078: "Defense Evasion",
    T1213: "Collection",
    T1110: "Credential Access",
    T1021: "Lateral Movement",
    T1071: "Command and Control",
    T1486: "Impact",
  }
  return tacticMap[techniqueId] || "Unknown"
}

/**
 * Build MITRE context from technique IDs and vector results
 * NVNN: buildMitreContext enriches technique IDs with descriptions
 */
async function buildMitreContext(
  mitreTechniques: string[],
  vectorResults: VectorSearchResult[]
): Promise<MitreContext[]> {
  const contexts: MitreContext[] = []

  for (const techniqueId of mitreTechniques) {
    // Find matching vector result for description
    const matchingResult = vectorResults.find(
      (r) => r.metadata?.technique_id === techniqueId
    )

    contexts.push({
      technique_id: techniqueId,
      technique_name: matchingResult?.metadata?.name as string || techniqueId,
      tactic: getTacticForTechnique(techniqueId),
      description: matchingResult?.document || `MITRE ATT&CK technique ${techniqueId}`,
      detection: matchingResult?.metadata?.detection as string,
      mitigations: matchingResult?.metadata?.mitigations as string[],
    })
  }

  return contexts
}

/**
 * Calculate GLAF scores from threat context
 * NVNN: calculateGlafScores computes H1/H2 convergence metrics
 */
function calculateGlafScores(
  threat: Threat,
  similarThreats: VectorSearchResult[]
): GlafScores {
  // H1 (Operational - Hawkes temporal): Based on threat recency and frequency
  const now = Date.now()
  const threatTime = new Date(threat.timestamp).getTime()
  const hoursSinceThreat = (now - threatTime) / (1000 * 60 * 60)

  // Decay function: more recent = higher score
  const h1_operational = Math.exp(-hoursSinceThreat / 24) // 24-hour decay

  // H2 (Semantic): Average similarity score from vector search
  const h2_semantic =
    similarThreats.length > 0
      ? similarThreats.reduce((sum, t) => sum + t.score, 0) / similarThreats.length
      : 0.5

  // Combined: Weighted average (RFC-9021 defaults)
  const H1_WEIGHT = 0.3
  const H2_WEIGHT = 0.7
  const combined = H1_WEIGHT * h1_operational + H2_WEIGHT * h2_semantic

  return {
    h1_operational: Math.round(h1_operational * 1000) / 1000,
    h2_semantic: Math.round(h2_semantic * 1000) / 1000,
    combined: Math.round(combined * 1000) / 1000,
    fragment_count: similarThreats.length,
    matroid_independent: similarThreats.length < 10, // Simplified check
  }
}

/**
 * Assemble unified context from all Layer 1-2 outputs
 * NVNN: assembleContext creates unified input for Layer 4
 */
export async function assembleContext(
  threat: Threat,
  options?: Partial<ContextAssemblyConfig>
): Promise<UnifiedContext> {
  const config = { ...DEFAULT_CONFIG, ...options }
  const startTime = performance.now()

  // Layer 1: Thalamic Filter
  const thalamic = await thalamicFilter(threat)

  // Fast path: reflexive decisions skip full context assembly
  if (!requiresFullProcessing(thalamic)) {
    return {
      threat,
      thalamic,
      similar_threats: [],
      glaf_scores: {
        h1_operational: 0,
        h2_semantic: 0,
        combined: 0,
      },
      mitre_context: [],
    }
  }

  // Layer 2a: ChromaDB Vector Search (parallel)
  const [similarThreats, techniqueResults] = await Promise.all([
    findSimilarThreats(threat, {
      nResults: config.maxSimilarThreats,
      minScore: config.minSimilarityScore,
    }),
    findRelatedTechniques(threat.description, config.maxTechniques),
  ])

  // Build MITRE context
  const mitre_context = await buildMitreContext(threat.mitre, techniqueResults)

  // Layer 2c: GLAF Scores
  const glaf_scores = config.enableGlaf
    ? calculateGlafScores(threat, similarThreats)
    : { h1_operational: 0, h2_semantic: 0, combined: 0 }

  // Layer 2b: GNN embeddings (placeholder - not yet connected)
  const gnn_embedding = config.enableGnn ? undefined : undefined

  const assemblyTime = performance.now() - startTime
  console.debug(`[Context] Assembly completed in ${assemblyTime.toFixed(2)}ms`)

  return {
    threat,
    thalamic,
    similar_threats: similarThreats,
    gnn_embedding,
    glaf_scores,
    mitre_context,
  }
}

/**
 * Quick context for reflexive (fast-path) decisions
 * NVNN: assembleQuickContext provides minimal context for reflexive path
 */
export function assembleQuickContext(
  threat: Threat,
  thalamic: ThalamicOutput
): UnifiedContext {
  return {
    threat,
    thalamic,
    similar_threats: [],
    glaf_scores: {
      h1_operational: 0,
      h2_semantic: 0,
      combined: 0,
    },
    mitre_context: threat.mitre.map((t) => ({
      technique_id: t,
      technique_name: t,
      tactic: getTacticForTechnique(t),
      description: `MITRE ATT&CK technique ${t}`,
    })),
  }
}

/**
 * Format context for Phi-3 prompt
 * NVNN: formatContextForPrompt serializes context for LLM input
 */
export function formatContextForPrompt(context: UnifiedContext): string {
  const lines: string[] = []

  // Threat summary
  lines.push(`## Threat Analysis Request`)
  lines.push(`Level: ${context.threat.level.toUpperCase()}`)
  lines.push(`Description: ${context.threat.description}`)
  lines.push(`Source: ${context.threat.source} → Target: ${context.threat.target}`)
  lines.push(`Confidence: ${(context.threat.confidence * 100).toFixed(0)}%`)

  // Thalamic classification
  lines.push(`\n## Cognitive Classification`)
  lines.push(`Priority: ${context.thalamic.priority}`)
  lines.push(`Pathway: ${context.thalamic.pathway}`)
  lines.push(`Domains: ${context.thalamic.activated_domains.join(", ")}`)

  // MITRE context
  if (context.mitre_context.length > 0) {
    lines.push(`\n## MITRE ATT&CK Context`)
    for (const technique of context.mitre_context) {
      lines.push(`- ${technique.technique_id} (${technique.tactic}): ${technique.technique_name}`)
    }
  }

  // Similar threats
  if (context.similar_threats.length > 0) {
    lines.push(`\n## Similar Historical Threats (${context.similar_threats.length})`)
    for (const similar of context.similar_threats.slice(0, 3)) {
      lines.push(`- Score ${(similar.score * 100).toFixed(0)}%: ${similar.document?.slice(0, 100) || "No description"}`)
    }
  }

  // GLAF scores
  lines.push(`\n## Convergence Metrics`)
  lines.push(`H1 (Temporal): ${(context.glaf_scores.h1_operational * 100).toFixed(0)}%`)
  lines.push(`H2 (Semantic): ${(context.glaf_scores.h2_semantic * 100).toFixed(0)}%`)
  lines.push(`Combined: ${(context.glaf_scores.combined * 100).toFixed(0)}%`)

  // Indicators
  if (context.threat.indicators.length > 0) {
    lines.push(`\n## Indicators`)
    for (const indicator of context.threat.indicators.slice(0, 5)) {
      lines.push(`- ${indicator}`)
    }
  }

  return lines.join("\n")
}

/**
 * Get context hash for caching
 * NVNN: getContextHash generates cache key from context
 */
export function getContextHash(context: UnifiedContext): string {
  const key = `${context.threat.id}-${context.thalamic.pathway}-${context.glaf_scores.combined}`
  let hash = 0
  for (let i = 0; i < key.length; i++) {
    const char = key.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  return hash.toString(36)
}
