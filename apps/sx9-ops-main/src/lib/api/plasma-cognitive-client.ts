/**
 * Plasma Cognitive API Client (RFC-9021)
 *
 * Client for the 4-Layer Cognitive Inference Engine.
 * Provides SSE streaming with cognitive enrichment for threats.
 *
 * NVNN: PlasmaCognitiveClient manages cognitive inference API connections
 */

import type {
  Threat,
  ThalamicOutput,
  GlafScores,
  Phi3Analysis,
  UnifiedContext,
  WazuhAgent,
  Entity,
  ToolExecution,
} from "../../types/plasma"

// API endpoint configuration
const PLASMA_API_BASE = import.meta.env.VITE_PLASMA_API || "/api/plasma"

// NVNN: CognitiveEnrichedThreat extends Threat with all Layer outputs
export interface CognitiveEnrichedThreat extends Threat {
  cognitive: {
    thalamic: ThalamicOutput
    glaf: GlafScores
    phi3?: Phi3Analysis
  }
}

// NVNN: PlasmaStreamEvent represents SSE event types
export type PlasmaStreamEvent =
  | { type: "threat"; data: CognitiveEnrichedThreat }
  | { type: "tool"; data: ToolExecution }
  | { type: "entity"; data: Entity }
  | { type: "metrics"; data: PlasmaMetrics }
  | { type: "heartbeat"; data: { timestamp: string } }

// NVNN: PlasmaMetrics tracks system performance
export interface PlasmaMetrics {
  threats_per_minute: number
  avg_inference_ms: number
  cache_hit_rate: number
  active_agents: number
}

// NVNN: InferenceRequest structures cognitive inference input
export interface InferenceRequest {
  threat: Threat
  skip_phi3?: boolean  // Skip Layer 4 for faster response
  enable_cache?: boolean
}

// NVNN: InferenceResponse contains full cognitive analysis
export interface InferenceResponse {
  threat_id: string
  thalamic: ThalamicOutput
  glaf: GlafScores
  phi3?: Phi3Analysis
  context?: UnifiedContext
  total_ms: number
}

/**
 * Plasma Cognitive API Client
 * NVNN: PlasmaCognitiveClient manages all Plasma API interactions
 */
export class PlasmaCognitiveClient {
  private baseUrl: string
  private eventSource: EventSource | null = null
  private reconnectAttempts = 0
  private maxReconnectAttempts = 3

  constructor(baseUrl: string = PLASMA_API_BASE) {
    this.baseUrl = baseUrl
  }

  // =========================================================================
  // SSE Stream Methods
  // =========================================================================

  /**
   * Connect to cognitive-enriched threat stream
   * NVNN: connectStream establishes SSE connection with cognitive enrichment
   */
  connectStream(
    callbacks: {
      onThreat?: (threat: CognitiveEnrichedThreat) => void
      onTool?: (tool: ToolExecution) => void
      onEntity?: (entity: Entity) => void
      onMetrics?: (metrics: PlasmaMetrics) => void
      onConnect?: () => void
      onError?: (error: Error) => void
    },
    options?: { lastEventId?: string }
  ): void {
    const streamUrl = `${this.baseUrl}/threats/stream`
    const urlWithParams = options?.lastEventId
      ? `${streamUrl}?lastEventId=${encodeURIComponent(options.lastEventId)}`
      : streamUrl

    try {
      this.eventSource = new EventSource(urlWithParams)

      this.eventSource.onopen = () => {
        console.log("[Plasma] Connected to cognitive threat stream")
        this.reconnectAttempts = 0
        callbacks.onConnect?.()
      }

      this.eventSource.addEventListener("threat", (event) => {
        try {
          const threat = JSON.parse(event.data) as CognitiveEnrichedThreat
          callbacks.onThreat?.(threat)
        } catch (err) {
          console.error("[Plasma] Failed to parse threat event:", err)
        }
      })

      this.eventSource.addEventListener("tool", (event) => {
        try {
          const tool = JSON.parse(event.data) as ToolExecution
          callbacks.onTool?.(tool)
        } catch (err) {
          console.error("[Plasma] Failed to parse tool event:", err)
        }
      })

      this.eventSource.addEventListener("entity", (event) => {
        try {
          const entity = JSON.parse(event.data) as Entity
          callbacks.onEntity?.(entity)
        } catch (err) {
          console.error("[Plasma] Failed to parse entity event:", err)
        }
      })

      this.eventSource.addEventListener("metrics", (event) => {
        try {
          const metrics = JSON.parse(event.data) as PlasmaMetrics
          callbacks.onMetrics?.(metrics)
        } catch (err) {
          console.error("[Plasma] Failed to parse metrics event:", err)
        }
      })

      this.eventSource.addEventListener("heartbeat", () => {
        // Keep-alive, no action needed
      })

      this.eventSource.onerror = () => {
        console.warn("[Plasma] Stream connection error")
        this.eventSource?.close()

        if (this.reconnectAttempts < this.maxReconnectAttempts) {
          this.reconnectAttempts++
          console.log(`[Plasma] Reconnecting (attempt ${this.reconnectAttempts})...`)
          setTimeout(() => this.connectStream(callbacks, options), 5000)
        } else {
          callbacks.onError?.(new Error("Max reconnection attempts reached"))
        }
      }
    } catch (err) {
      console.error("[Plasma] Failed to initialize stream:", err)
      callbacks.onError?.(err as Error)
    }
  }

  /**
   * Disconnect from stream
   */
  disconnectStream(): void {
    if (this.eventSource) {
      this.eventSource.close()
      this.eventSource = null
    }
    this.reconnectAttempts = 0
  }

  /**
   * Check if connected to stream
   */
  isConnected(): boolean {
    return this.eventSource?.readyState === EventSource.OPEN
  }

  // =========================================================================
  // REST API Methods
  // =========================================================================

  /**
   * Run cognitive inference on a threat
   * NVNN: infer executes full RFC-9021 cognitive pipeline
   */
  async infer(request: InferenceRequest): Promise<InferenceResponse> {
    const response = await fetch(`${this.baseUrl}/cognitive/infer`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(request),
    })

    if (!response.ok) {
      throw new Error(`Inference failed: ${response.status}`)
    }

    return response.json()
  }

  /**
   * Get all threats with optional filters
   */
  async getThreats(options?: {
    level?: string
    since?: string
    limit?: number
  }): Promise<Threat[]> {
    const params = new URLSearchParams()
    if (options?.level) params.set("level", options.level)
    if (options?.since) params.set("since", options.since)
    if (options?.limit) params.set("limit", options.limit.toString())

    const response = await fetch(`${this.baseUrl}/threats?${params}`)
    if (!response.ok) throw new Error(`Failed to get threats: ${response.status}`)
    return response.json()
  }

  /**
   * Get threat statistics
   */
  async getThreatStats(): Promise<{
    total: number
    by_level: Record<string, number>
    by_hour: { hour: string; count: number }[]
  }> {
    const response = await fetch(`${this.baseUrl}/threats/stats`)
    if (!response.ok) throw new Error(`Failed to get stats: ${response.status}`)
    return response.json()
  }

  /**
   * Get all agents
   */
  async getAgents(): Promise<WazuhAgent[]> {
    const response = await fetch(`${this.baseUrl}/agents`)
    if (!response.ok) throw new Error(`Failed to get agents: ${response.status}`)
    return response.json()
  }

  /**
   * Get agent by ID
   */
  async getAgent(id: string): Promise<WazuhAgent> {
    const response = await fetch(`${this.baseUrl}/agents/${id}`)
    if (!response.ok) throw new Error(`Failed to get agent: ${response.status}`)
    return response.json()
  }

  /**
   * Restart agent
   */
  async restartAgent(id: string): Promise<{ success: boolean }> {
    const response = await fetch(`${this.baseUrl}/agents/${id}/restart`, {
      method: "POST",
    })
    if (!response.ok) throw new Error(`Failed to restart agent: ${response.status}`)
    return response.json()
  }

  /**
   * Get all entities
   */
  async getEntities(options?: {
    type?: string
    minReputation?: number
  }): Promise<Entity[]> {
    const params = new URLSearchParams()
    if (options?.type) params.set("type", options.type)
    if (options?.minReputation) params.set("minReputation", options.minReputation.toString())

    const response = await fetch(`${this.baseUrl}/entities?${params}`)
    if (!response.ok) throw new Error(`Failed to get entities: ${response.status}`)
    return response.json()
  }

  /**
   * Get CDN-style statistics
   */
  async getStats(): Promise<{
    bandwidth: number
    requests: number
    cache_hit_rate: number
    edge_nodes: number
    latency_p50: number
    latency_p99: number
  }> {
    const response = await fetch(`${this.baseUrl}/stats`)
    if (!response.ok) throw new Error(`Failed to get stats: ${response.status}`)
    return response.json()
  }

  /**
   * Health check
   */
  async healthCheck(): Promise<{
    healthy: boolean
    services: {
      supabase: boolean
      chromadb: boolean
      leptose: boolean
    }
  }> {
    const response = await fetch(`${this.baseUrl}/health`)
    if (!response.ok) return { healthy: false, services: { supabase: false, chromadb: false, leptose: false } }
    return response.json()
  }
}

// Singleton instance
export const plasmaCognitiveClient = new PlasmaCognitiveClient()
