/**
 * Layer 1: Thalamic Filter Service (RFC-9021)
 *
 * Rapid pre-classification using DistilBERT + LoRA adapter.
 * Target latency: <10ms P95, <15ms P99
 *
 * Provides fast-path gating for incoming threat data:
 * - Gate decision: reflexive (fast path) | full_processing (cognitive pipeline)
 * - Pathway: threat_analysis | operational | informational | creative
 * - Priority: low | medium | high | critical
 * - Activated domains: apt_attribution, technique_mapping, detection, etc.
 *
 * NVNN: ThalamicFilterService gates incoming threats via DistilBERT inference
 */

import type {
  ThalamicOutput,
  GateDecision,
  CognitivePathway,
  CognitivePriority,
  CognitiveDomain,
  Threat,
} from "../../types/plasma"

// Leptose inference server endpoint (hosts DistilBERT + Phi-3)
const LEPTOSE_ENDPOINT = import.meta.env.VITE_LEPTOSE_URL || "http://localhost:18114"
const THALAMIC_TIMEOUT_MS = 15 // Must complete within 15ms for P99

// NVNN: ThalamicFilterConfig defines filter behavior and thresholds
interface ThalamicFilterConfig {
  endpoint: string
  timeoutMs: number
  enableCache: boolean
  cacheTtlMs: number
}

// NVNN: ThalamicRequest structures the inference input
interface ThalamicRequest {
  text: string
  metadata?: {
    source?: string
    level?: string
    mitre?: string[]
  }
}

// NVNN: ThalamicResponse matches Leptose API response format
interface ThalamicResponse {
  gate_decision: GateDecision
  pathway: CognitivePathway
  priority: CognitivePriority
  activated_domains: CognitiveDomain[]
  inference_ms: number
  model_version: string
}

// In-memory cache for thalamic decisions (fast path)
const thalamicCache = new Map<string, { output: ThalamicOutput; expires: number }>()

/**
 * Hash input text for cache key
 * NVNN: hashInput creates cache key from threat content
 */
function hashInput(text: string): string {
  // Simple hash for cache key (in production use crypto.subtle)
  let hash = 0
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  return hash.toString(36)
}

/**
 * Check cache for existing thalamic decision
 * NVNN: getCachedDecision retrieves fast-path cached results
 */
function getCachedDecision(inputHash: string): ThalamicOutput | null {
  const cached = thalamicCache.get(inputHash)
  if (cached && cached.expires > Date.now()) {
    return cached.output
  }
  thalamicCache.delete(inputHash)
  return null
}

/**
 * Store thalamic decision in cache
 * NVNN: cacheDecision stores result for fast-path retrieval
 */
function cacheDecision(inputHash: string, output: ThalamicOutput, ttlMs: number): void {
  thalamicCache.set(inputHash, {
    output,
    expires: Date.now() + ttlMs,
  })

  // Cleanup old entries periodically
  if (thalamicCache.size > 10000) {
    const now = Date.now()
    for (const [key, value] of thalamicCache.entries()) {
      if (value.expires < now) {
        thalamicCache.delete(key)
      }
    }
  }
}

/**
 * Rule-based fallback when inference service unavailable
 * NVNN: fallbackClassification provides heuristic gating
 */
function fallbackClassification(threat: Threat): ThalamicOutput {
  const startTime = performance.now()

  // Priority based on threat level
  const priorityMap: Record<string, CognitivePriority> = {
    critical: "critical",
    high: "high",
    medium: "medium",
    low: "low",
  }

  // Gate decision based on confidence
  const gateDecision: GateDecision = threat.confidence > 0.8 ? "full_processing" : "reflexive"

  // Pathway based on indicators
  let pathway: CognitivePathway = "informational"
  if (threat.mitre.length > 0) pathway = "threat_analysis"
  if (threat.indicators.some((i) => i.includes("exploit"))) pathway = "operational"

  // Activated domains based on MITRE techniques
  const domains: CognitiveDomain[] = []
  if (threat.mitre.some((t) => t.startsWith("T1"))) domains.push("technique_mapping")
  if (threat.level === "critical" || threat.level === "high") domains.push("incident_response")
  if (threat.confidence > 0.7) domains.push("detection")

  return {
    gate_decision: gateDecision,
    pathway,
    priority: priorityMap[threat.level] || "medium",
    activated_domains: domains.length > 0 ? domains : ["detection"],
    inference_ms: Math.round(performance.now() - startTime),
  }
}

/**
 * Main thalamic filter function
 * Calls Leptose DistilBERT inference with timeout and fallback
 *
 * NVNN: thalamicFilter invokes Layer 1 DistilBERT classification
 */
export async function thalamicFilter(
  threat: Threat,
  config?: Partial<ThalamicFilterConfig>
): Promise<ThalamicOutput> {
  const cfg: ThalamicFilterConfig = {
    endpoint: config?.endpoint || LEPTOSE_ENDPOINT,
    timeoutMs: config?.timeoutMs || THALAMIC_TIMEOUT_MS,
    enableCache: config?.enableCache ?? true,
    cacheTtlMs: config?.cacheTtlMs || 60000, // 1 minute default
  }

  // Construct input text for classification
  const inputText = `${threat.level}: ${threat.description} | Source: ${threat.source} | MITRE: ${threat.mitre.join(", ")}`
  const inputHash = hashInput(inputText)

  // Check cache first (fast path)
  if (cfg.enableCache) {
    const cached = getCachedDecision(inputHash)
    if (cached) {
      return { ...cached, inference_ms: 0 }
    }
  }

  // Attempt inference with timeout
  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), cfg.timeoutMs)

  try {
    const response = await fetch(`${cfg.endpoint}/api/v1/thalamic/classify`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        text: inputText,
        metadata: {
          source: threat.source,
          level: threat.level,
          mitre: threat.mitre,
        },
      } as ThalamicRequest),
      signal: controller.signal,
    })

    clearTimeout(timeoutId)

    if (!response.ok) {
      console.warn(`[Thalamic] Inference failed (${response.status}), using fallback`)
      return fallbackClassification(threat)
    }

    const result: ThalamicResponse = await response.json()

    const output: ThalamicOutput = {
      gate_decision: result.gate_decision,
      pathway: result.pathway,
      priority: result.priority,
      activated_domains: result.activated_domains,
      inference_ms: result.inference_ms,
    }

    // Cache the result
    if (cfg.enableCache) {
      cacheDecision(inputHash, output, cfg.cacheTtlMs)
    }

    return output
  } catch (error) {
    clearTimeout(timeoutId)

    if (error instanceof Error && error.name === "AbortError") {
      console.warn(`[Thalamic] Inference timeout (${cfg.timeoutMs}ms), using fallback`)
    } else {
      console.warn("[Thalamic] Inference error, using fallback:", error)
    }

    return fallbackClassification(threat)
  }
}

/**
 * Batch thalamic filter for multiple threats
 * NVNN: batchThalamicFilter processes multiple threats efficiently
 */
export async function batchThalamicFilter(
  threats: Threat[],
  config?: Partial<ThalamicFilterConfig>
): Promise<Map<string, ThalamicOutput>> {
  const results = new Map<string, ThalamicOutput>()

  // Process in parallel with concurrency limit
  const batchSize = 10
  for (let i = 0; i < threats.length; i += batchSize) {
    const batch = threats.slice(i, i + batchSize)
    const batchResults = await Promise.all(
      batch.map((threat) => thalamicFilter(threat, config))
    )
    batch.forEach((threat, idx) => {
      results.set(threat.id, batchResults[idx])
    })
  }

  return results
}

/**
 * Check if threat requires full cognitive processing
 * NVNN: requiresFullProcessing determines pipeline depth
 */
export function requiresFullProcessing(thalamic: ThalamicOutput): boolean {
  return thalamic.gate_decision === "full_processing"
}

/**
 * Get fast-path response for reflexive decisions
 * NVNN: getReflexiveResponse generates quick response without full pipeline
 */
export function getReflexiveResponse(threat: Threat, thalamic: ThalamicOutput): string {
  if (thalamic.priority === "critical") {
    return `CRITICAL: ${threat.description} - Immediate attention required`
  }
  if (thalamic.priority === "high") {
    return `HIGH: ${threat.description} - Investigation recommended`
  }
  return `${threat.level.toUpperCase()}: ${threat.description}`
}

/**
 * Clear thalamic cache (for testing/debugging)
 */
export function clearThalamicCache(): void {
  thalamicCache.clear()
}

/**
 * Get cache statistics
 */
export function getCacheStats(): { size: number; hitRate: number } {
  return {
    size: thalamicCache.size,
    hitRate: 0, // Would need tracking to calculate
  }
}
