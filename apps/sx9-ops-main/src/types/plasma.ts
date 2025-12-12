// =============================================================================
// Core Plasma Types
// =============================================================================

export type ThreatLevel = "critical" | "high" | "medium" | "low"
export type ToolStatus = "running" | "success" | "failed" | "queued"
export type AgentStatus = "active" | "disconnected" | "never_connected" | "pending"
export type AgentOS = "linux" | "windows" | "macos" | "unknown"

// =============================================================================
// RFC-9021 Cognitive Inference Types
// =============================================================================

// Layer 1: Thalamic Filter (DistilBERT)
export type GateDecision = "reflexive" | "full_processing"
export type CognitivePathway = "threat_analysis" | "operational" | "informational" | "creative"
export type CognitivePriority = "low" | "medium" | "high" | "critical"
export type CognitiveDomain =
  | "apt_attribution"
  | "technique_mapping"
  | "detection"
  | "incident_response"
  | "threat_hunting"
  | "vulnerability_assessment"

export interface ThalamicOutput {
  gate_decision: GateDecision
  pathway: CognitivePathway
  priority: CognitivePriority
  activated_domains: CognitiveDomain[]
  inference_ms?: number
}

// Layer 2c: GLAF Matroid Convergence
export interface GlafScores {
  h1_operational: number  // Hawkes process temporal score (0-1)
  h2_semantic: number     // Semantic similarity score (0-1)
  combined: number        // Weighted combination (0-1)
  fragment_count?: number
  matroid_independent?: boolean
}

// Layer 4: Phi-3 Generative Response
export interface Phi3Analysis {
  summary: string
  recommendations: string[]
  related_techniques: string[]  // MITRE ATT&CK IDs
  confidence: number
  inference_ms?: number
  token_count?: number
}

// Unified Context (Layer 3 Assembly)
export interface UnifiedContext {
  threat: Threat
  thalamic: ThalamicOutput
  similar_threats: VectorSearchResult[]
  gnn_embedding?: number[]  // 768-dim GNN embedding
  glaf_scores: GlafScores
  mitre_context: MitreContext[]
}

// Vector Search Result (ChromaDB)
export interface VectorSearchResult {
  id: string
  score: number  // Cosine similarity (0-1)
  document?: string
  metadata: Record<string, unknown>
}

// MITRE ATT&CK Context
export interface MitreContext {
  technique_id: string
  technique_name: string
  tactic: string
  description: string
  detection?: string
  mitigations?: string[]
}

// Cognitive Status (attached to agents/threats)
export interface CognitiveStatus {
  thalamic?: ThalamicOutput
  glaf?: GlafScores
  phi3?: Phi3Analysis
  last_inference?: string
  embedding_id?: string
}

// =============================================================================
// Enhanced Threat Type with Cognitive Fields
// =============================================================================

export interface Threat {
  id: string
  timestamp: string
  level: ThreatLevel
  source: string
  target: string
  description: string
  indicators: string[]
  mitre: string[]
  confidence: number
  raw?: unknown
  // RFC-9021 Cognitive fields
  trivariate_hash?: string
  thalamic_output?: ThalamicOutput
  glaf_scores?: GlafScores
  phi3_analysis?: Phi3Analysis
  embedding_id?: string
  source_feed?: string
}

export interface ToolExecution {
  id: string
  tool: string
  status: ToolStatus
  startTime: string
  endTime?: string
  target: string
  output?: string
  error?: string
}

export interface Entity {
  id: string
  type: "ip" | "domain" | "hash" | "user" | "process"
  value: string
  firstSeen: string
  lastSeen: string
  threatCount: number
  reputation: number
  related: string[]
  tags: string[]
}

export interface WazuhAgent {
  id: string
  name: string
  ip: string
  status: AgentStatus
  os: AgentOS
  osVersion: string
  version: string
  lastKeepAlive?: string
  group: string[]
  manager: string
  registerDate: string
  configSum?: string
  mergedSum?: string
  alertCount: number
  // RFC-9021 Cognitive fields
  trivariate_hash?: string
  cognitive_status?: CognitiveStatus
  last_inference?: string
}

export interface WazuhManager {
  id: string
  name: string
  url: string
  apiKey: string
  enabled: boolean
  agentCount: number
  lastSync?: string
}

export interface LLMConfig {
  provider: "local" | "remote"
  modelPath?: string
  apiUrl?: string
  apiKey?: string
  modelName: string
  loaded: boolean
}

export interface AgentDeployment {
  os: AgentOS
  installCommand: string
  configTemplate: string
}
