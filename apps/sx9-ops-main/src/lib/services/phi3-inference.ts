/**
 * Layer 4: Phi-3 Generative Inference Service (RFC-9021)
 *
 * Natural language threat analysis using Phi-3-mini-4k-instruct with LoRA.
 * Target latency: <500ms P95, <1000ms P99
 *
 * Model: microsoft/Phi-3-mini-4k-instruct (4-bit quantized, ~2GB)
 * LoRA: r=16, alpha=32, threat-tuned
 *
 * Generates:
 * - Summary: Natural language threat summary
 * - Recommendations: Actionable detection/response steps
 * - Related techniques: MITRE ATT&CK IDs
 *
 * NVNN: Phi3InferenceService generates natural language threat analysis
 */

import type {
  Phi3Analysis,
  UnifiedContext,
  Threat,
} from "../../types/plasma"
import { formatContextForPrompt, getContextHash } from "./context-assembler"

// Leptose inference server endpoint (hosts Phi-3)
const LEPTOSE_ENDPOINT = import.meta.env.VITE_LEPTOSE_URL || "http://localhost:18114"
const PHI3_TIMEOUT_MS = 1000 // P99 target

// NVNN: Phi3Config defines inference parameters
interface Phi3Config {
  endpoint: string
  timeoutMs: number
  maxTokens: number
  temperature: number
  enableCache: boolean
  cacheTtlMs: number
}

const DEFAULT_CONFIG: Phi3Config = {
  endpoint: LEPTOSE_ENDPOINT,
  timeoutMs: PHI3_TIMEOUT_MS,
  maxTokens: 512,
  temperature: 0.3, // Lower for more deterministic responses
  enableCache: true,
  cacheTtlMs: 24 * 60 * 60 * 1000, // 24 hours
}

// NVNN: Phi3Request structures the inference input
interface Phi3Request {
  prompt: string
  max_tokens: number
  temperature: number
  stop_sequences: string[]
}

// NVNN: Phi3Response matches Leptose API response format
interface Phi3Response {
  text: string
  tokens_used: number
  inference_ms: number
  model_version: string
}

// In-memory cache for Phi-3 analyses
const phi3Cache = new Map<string, { analysis: Phi3Analysis; expires: number }>()

/**
 * Parse Phi-3 response text into structured analysis
 * NVNN: parseAnalysisResponse extracts structured data from LLM output
 */
function parseAnalysisResponse(responseText: string): Partial<Phi3Analysis> {
  const analysis: Partial<Phi3Analysis> = {
    summary: "",
    recommendations: [],
    related_techniques: [],
  }

  // Extract summary (first paragraph or ## Summary section)
  const summaryMatch = responseText.match(/## Summary\n([\s\S]*?)(?=\n##|$)/)
  if (summaryMatch) {
    analysis.summary = summaryMatch[1].trim()
  } else {
    // Use first paragraph as summary
    const firstParagraph = responseText.split("\n\n")[0]
    analysis.summary = firstParagraph.trim()
  }

  // Extract recommendations
  const recsMatch = responseText.match(/## Recommendations?\n([\s\S]*?)(?=\n##|$)/)
  if (recsMatch) {
    const recsList = recsMatch[1]
      .split("\n")
      .filter((line) => line.trim().startsWith("-") || line.trim().match(/^\d+\./))
      .map((line) => line.replace(/^[-\d.]+\s*/, "").trim())
    analysis.recommendations = recsList
  }

  // Extract MITRE techniques (T#### pattern)
  const techniqueMatches = responseText.match(/T\d{4}(?:\.\d{3})?/g)
  if (techniqueMatches) {
    analysis.related_techniques = [...new Set(techniqueMatches)]
  }

  return analysis
}

/**
 * Build Phi-3 system prompt for threat analysis
 * NVNN: buildSystemPrompt creates instruction prompt for Phi-3
 */
function buildSystemPrompt(): string {
  return `You are a cybersecurity threat analyst assistant. Analyze the provided threat intelligence and generate:

1. A concise summary of the threat (2-3 sentences)
2. Specific, actionable recommendations for detection and response
3. Related MITRE ATT&CK techniques

Format your response with markdown headers:
## Summary
[Your summary here]

## Recommendations
- [Recommendation 1]
- [Recommendation 2]
...

## Related Techniques
- T#### - [Technique Name]
...

Be specific and actionable. Prioritize high-impact recommendations.`
}

/**
 * Fallback analysis when inference unavailable
 * NVNN: fallbackAnalysis generates heuristic response without LLM
 */
function fallbackAnalysis(context: UnifiedContext): Phi3Analysis {
  const threat = context.threat

  // Generate summary from threat data
  const summary = `${threat.level.toUpperCase()} severity threat detected from ${threat.source} targeting ${threat.target}. ${threat.description}. Confidence: ${(threat.confidence * 100).toFixed(0)}%.`

  // Generate recommendations based on MITRE techniques
  const recommendations: string[] = []
  if (threat.mitre.includes("T1190")) {
    recommendations.push("Review web application firewall rules for exploitation attempts")
  }
  if (threat.mitre.includes("T1059")) {
    recommendations.push("Monitor command-line activity on affected systems")
  }
  if (threat.level === "critical" || threat.level === "high") {
    recommendations.push("Initiate incident response procedures")
    recommendations.push("Isolate affected systems if compromise is confirmed")
  }
  recommendations.push("Update detection signatures with observed indicators")
  recommendations.push("Review logs for lateral movement indicators")

  return {
    summary,
    recommendations,
    related_techniques: threat.mitre,
    confidence: 0.5, // Lower confidence for fallback
    inference_ms: 0,
    token_count: 0,
  }
}

/**
 * Main Phi-3 inference function
 * NVNN: generateAnalysis invokes Layer 4 Phi-3 for threat analysis
 */
export async function generateAnalysis(
  context: UnifiedContext,
  config?: Partial<Phi3Config>
): Promise<Phi3Analysis> {
  const cfg: Phi3Config = { ...DEFAULT_CONFIG, ...config }
  const startTime = performance.now()

  // Check cache
  const contextHash = getContextHash(context)
  if (cfg.enableCache) {
    const cached = phi3Cache.get(contextHash)
    if (cached && cached.expires > Date.now()) {
      return { ...cached.analysis, inference_ms: 0 }
    }
    phi3Cache.delete(contextHash)
  }

  // Build prompt
  const systemPrompt = buildSystemPrompt()
  const userPrompt = formatContextForPrompt(context)
  const fullPrompt = `${systemPrompt}\n\n${userPrompt}\n\nAnalysis:`

  // Attempt inference with timeout
  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), cfg.timeoutMs)

  try {
    const response = await fetch(`${cfg.endpoint}/api/v1/phi3/generate`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        prompt: fullPrompt,
        max_tokens: cfg.maxTokens,
        temperature: cfg.temperature,
        stop_sequences: ["## End", "---", "```"],
      } as Phi3Request),
      signal: controller.signal,
    })

    clearTimeout(timeoutId)

    if (!response.ok) {
      console.warn(`[Phi3] Inference failed (${response.status}), using fallback`)
      return fallbackAnalysis(context)
    }

    const result: Phi3Response = await response.json()
    const inferenceMs = performance.now() - startTime

    // Parse response into structured format
    const parsed = parseAnalysisResponse(result.text)

    const analysis: Phi3Analysis = {
      summary: parsed.summary || fallbackAnalysis(context).summary,
      recommendations: parsed.recommendations?.length
        ? parsed.recommendations
        : fallbackAnalysis(context).recommendations,
      related_techniques: parsed.related_techniques?.length
        ? parsed.related_techniques
        : context.threat.mitre,
      confidence: context.glaf_scores.combined,
      inference_ms: Math.round(inferenceMs),
      token_count: result.tokens_used,
    }

    // Cache the result
    if (cfg.enableCache) {
      phi3Cache.set(contextHash, {
        analysis,
        expires: Date.now() + cfg.cacheTtlMs,
      })
    }

    return analysis
  } catch (error) {
    clearTimeout(timeoutId)

    if (error instanceof Error && error.name === "AbortError") {
      console.warn(`[Phi3] Inference timeout (${cfg.timeoutMs}ms), using fallback`)
    } else {
      console.warn("[Phi3] Inference error, using fallback:", error)
    }

    return fallbackAnalysis(context)
  }
}

/**
 * Generate quick summary without full context
 * NVNN: generateQuickSummary provides fast summary for reflexive path
 */
export function generateQuickSummary(threat: Threat): string {
  return `${threat.level.toUpperCase()}: ${threat.description} | Source: ${threat.source} | Confidence: ${(threat.confidence * 100).toFixed(0)}%`
}

/**
 * Stream Phi-3 response (for real-time UI updates)
 * NVNN: streamAnalysis provides streaming inference for responsive UI
 */
export async function* streamAnalysis(
  context: UnifiedContext,
  config?: Partial<Phi3Config>
): AsyncGenerator<string, void, unknown> {
  const cfg: Phi3Config = { ...DEFAULT_CONFIG, ...config }

  const systemPrompt = buildSystemPrompt()
  const userPrompt = formatContextForPrompt(context)
  const fullPrompt = `${systemPrompt}\n\n${userPrompt}\n\nAnalysis:`

  try {
    const response = await fetch(`${cfg.endpoint}/api/v1/phi3/stream`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        prompt: fullPrompt,
        max_tokens: cfg.maxTokens,
        temperature: cfg.temperature,
        stream: true,
      }),
    })

    if (!response.ok || !response.body) {
      yield fallbackAnalysis(context).summary
      return
    }

    const reader = response.body.getReader()
    const decoder = new TextDecoder()

    while (true) {
      const { done, value } = await reader.read()
      if (done) break

      const chunk = decoder.decode(value, { stream: true })
      yield chunk
    }
  } catch (error) {
    console.warn("[Phi3] Stream error, using fallback:", error)
    yield fallbackAnalysis(context).summary
  }
}

/**
 * Clear Phi-3 cache
 */
export function clearPhi3Cache(): void {
  phi3Cache.clear()
}

/**
 * Get cache statistics
 */
export function getCacheStats(): { size: number } {
  return { size: phi3Cache.size }
}

/**
 * Full cognitive inference pipeline
 * Combines Layers 1-4 for complete threat analysis
 * NVNN: runCognitivePipeline executes full RFC-9021 inference
 */
export async function runCognitivePipeline(
  threat: Threat,
  config?: Partial<Phi3Config>
): Promise<{
  analysis: Phi3Analysis
  context: UnifiedContext
  totalMs: number
}> {
  const startTime = performance.now()

  // Import here to avoid circular dependency
  const { assembleContext } = await import("./context-assembler")

  // Layers 1-3: Context assembly
  const context = await assembleContext(threat)

  // Layer 4: Generate analysis
  const analysis = await generateAnalysis(context, config)

  const totalMs = performance.now() - startTime

  return {
    analysis,
    context,
    totalMs: Math.round(totalMs),
  }
}
