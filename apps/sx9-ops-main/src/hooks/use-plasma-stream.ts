"use client"

import { useState, useEffect, useCallback } from "react"
import type { Threat, ToolExecution, Entity, ThalamicOutput, GlafScores, Phi3Analysis } from "../types/plasma"
import {
  plasmaCognitiveClient,
  type CognitiveEnrichedThreat,
  type PlasmaMetrics,
} from "../lib/api/plasma-cognitive-client"

// NVNN: CognitiveEnrichedThreatState tracks threat with cognitive data
interface CognitiveState {
  thalamic?: ThalamicOutput
  glaf?: GlafScores
  phi3?: Phi3Analysis
}

// Mock data for fallback mode
const MOCK_THREATS: Threat[] = [
  {
    id: "THR-001",
    timestamp: new Date().toISOString(),
    level: "critical",
    source: "203.0.113.45",
    target: "10.0.1.15",
    description: "SQL injection attempt detected on authentication endpoint",
    indicators: ["UNION SELECT", "admin' OR '1'='1"],
    mitre: ["T1190", "T1059"],
    confidence: 0.95,
    // RFC-9021 cognitive enrichment (mock)
    thalamic_output: {
      gate_decision: "full_processing",
      pathway: "threat_analysis",
      priority: "critical",
      activated_domains: ["technique_mapping", "incident_response"],
    },
    glaf_scores: { h1_operational: 0.92, h2_semantic: 0.88, combined: 0.89 },
  },
  {
    id: "THR-002",
    timestamp: new Date(Date.now() - 120000).toISOString(),
    level: "high",
    source: "198.51.100.22",
    target: "10.0.2.20",
    description: "Unusual database query pattern from internal host",
    indicators: ["SELECT * FROM users", "DROP TABLE"],
    mitre: ["T1213"],
    confidence: 0.87,
    thalamic_output: {
      gate_decision: "full_processing",
      pathway: "threat_analysis",
      priority: "high",
      activated_domains: ["detection", "technique_mapping"],
    },
    glaf_scores: { h1_operational: 0.75, h2_semantic: 0.82, combined: 0.80 },
  },
]

const MOCK_TOOLS: ToolExecution[] = [
  {
    id: "TOOL-001",
    tool: "nmap",
    status: "running",
    startTime: new Date(Date.now() - 45000).toISOString(),
    target: "10.0.1.0/24",
  },
]

const MOCK_ENTITIES: Entity[] = [
  {
    id: "ENT-001",
    type: "ip",
    value: "203.0.113.45",
    firstSeen: "2024-01-15T08:23:00Z",
    lastSeen: new Date().toISOString(),
    threatCount: 15,
    reputation: 0.15,
    related: ["ENT-003"],
    tags: ["attacker", "scanner"],
  },
]

/**
 * Plasma Stream Hook with RFC-9021 Cognitive Enrichment
 * NVNN: usePlasmaStream connects to cognitive threat stream
 */
export function usePlasmaStream(enabled = true) {
  const [threats, setThreats] = useState<Threat[]>(MOCK_THREATS)
  const [tools, setTools] = useState<ToolExecution[]>(MOCK_TOOLS)
  const [entities, setEntities] = useState<Entity[]>(MOCK_ENTITIES)
  const [connected, setConnected] = useState(false)
  const [useMockData, setUseMockData] = useState(true)
  const [metrics, setMetrics] = useState<PlasmaMetrics | null>(null)

  // Request cognitive inference for a threat
  const requestInference = useCallback(async (threat: Threat): Promise<CognitiveState | null> => {
    try {
      const result = await plasmaCognitiveClient.infer({ threat })
      return {
        thalamic: result.thalamic,
        glaf: result.glaf,
        phi3: result.phi3,
      }
    } catch (err) {
      console.warn("[Plasma] Inference request failed:", err)
      return null
    }
  }, [])

  useEffect(() => {
    if (!enabled) return

    let mockInterval: NodeJS.Timeout

    // Try to connect to real cognitive stream
    plasmaCognitiveClient.connectStream({
      onConnect: () => {
        console.log("[Plasma] Connected to cognitive threat stream")
        setConnected(true)
        setUseMockData(false)
      },

      onThreat: (enrichedThreat: CognitiveEnrichedThreat) => {
        // Convert cognitive enriched threat to standard Threat with cognitive fields
        const threat: Threat = {
          ...enrichedThreat,
          thalamic_output: enrichedThreat.cognitive.thalamic,
          glaf_scores: enrichedThreat.cognitive.glaf,
          phi3_analysis: enrichedThreat.cognitive.phi3,
        }
        setThreats((prev) => [threat, ...prev].slice(0, 50))
      },

      onTool: (tool: ToolExecution) => {
        setTools((prev) => {
          const index = prev.findIndex((t) => t.id === tool.id)
          if (index >= 0) {
            const updated = [...prev]
            updated[index] = tool
            return updated
          }
          return [tool, ...prev].slice(0, 20)
        })
      },

      onEntity: (entity: Entity) => {
        setEntities((prev) => {
          const index = prev.findIndex((ent) => ent.id === entity.id)
          if (index >= 0) {
            const updated = [...prev]
            updated[index] = entity
            return updated
          }
          return [entity, ...prev].slice(0, 100)
        })
      },

      onMetrics: (newMetrics: PlasmaMetrics) => {
        setMetrics(newMetrics)
      },

      onError: () => {
        console.log("[Plasma] Connection failed, falling back to mock mode")
        setConnected(false)
        setUseMockData(true)
      },
    })

    // Mock data fallback with simulated cognitive enrichment
    mockInterval = setInterval(() => {
      if (useMockData && Math.random() > 0.7) {
        const levels: Threat["level"][] = ["critical", "high", "medium", "low"]
        const sources = ["203.0.113.45", "198.51.100.22", "192.0.2.10"]
        const mitreTechniques = ["T1190", "T1059", "T1078", "T1213", "T1021"]

        const level = levels[Math.floor(Math.random() * 4)]
        const confidence = Math.random() * 0.5 + 0.5

        // Simulate thalamic output based on threat level
        const thalamic: ThalamicOutput = {
          gate_decision: confidence > 0.7 ? "full_processing" : "reflexive",
          pathway: "threat_analysis",
          priority: level,
          activated_domains: level === "critical" || level === "high"
            ? ["technique_mapping", "incident_response", "detection"]
            : ["detection"],
        }

        // Simulate GLAF scores
        const glaf: GlafScores = {
          h1_operational: Math.random() * 0.3 + 0.6,
          h2_semantic: Math.random() * 0.3 + 0.5,
          combined: Math.random() * 0.3 + 0.55,
        }

        const newThreat: Threat = {
          id: `THR-${Date.now()}`,
          timestamp: new Date().toISOString(),
          level,
          source: sources[Math.floor(Math.random() * sources.length)],
          target: "10.0.1.15",
          description: "Simulated threat event (cognitive mock mode)",
          indicators: ["Mock pattern"],
          mitre: [mitreTechniques[Math.floor(Math.random() * mitreTechniques.length)]],
          confidence,
          // RFC-9021 cognitive enrichment
          thalamic_output: thalamic,
          glaf_scores: glaf,
        }

        setThreats((prev) => [newThreat, ...prev].slice(0, 50))
      }
    }, 5000)

    return () => {
      plasmaCognitiveClient.disconnectStream()
      clearInterval(mockInterval)
      setConnected(false)
    }
  }, [enabled, useMockData])

  return {
    threats,
    tools,
    entities,
    connected,
    useMockData,
    metrics,
    requestInference,
  }
}
