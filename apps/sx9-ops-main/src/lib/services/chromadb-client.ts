/**
 * Layer 2a: ChromaDB Vector Search Service (RFC-9021)
 *
 * 384-dimensional vector search using all-MiniLM-L6-v2 embeddings.
 * Target latency: <50ms P95, <100ms P99
 *
 * Collections:
 * - plasma_threats: Threat description embeddings
 * - plasma_indicators: IOC embeddings for similarity matching
 * - plasma_scenarios: Attack scenario embeddings
 * - techniques: MITRE ATT&CK technique descriptions (existing)
 * - detection_rules: Sigma/YARA/Wazuh rules (existing)
 *
 * NVNN: ChromaDBClient provides semantic similarity search for Layer 2a
 */

import type { VectorSearchResult, Threat, CognitiveDomain } from "../../types/plasma"

// ChromaDB and embedding service endpoints
const CHROMADB_ENDPOINT = import.meta.env.VITE_CHROMADB_URL || "http://localhost:8000"
const EMBEDDING_ENDPOINT = import.meta.env.VITE_EMBEDDING_URL || "http://localhost:18117"

// RFC-9021: 384-dimensional embeddings (all-MiniLM-L6-v2)
const EMBEDDING_DIM = 384

// NVNN: PlasmaCollections defines available vector collections
export const PlasmaCollections = {
  THREATS: "plasma_threats",
  INDICATORS: "plasma_indicators",
  SCENARIOS: "plasma_scenarios",
  TECHNIQUES: "techniques",
  DETECTION_RULES: "detection_rules",
  TOOLS: "tools",
  INTERVIEWS: "interviews",
} as const

export type PlasmaCollection = (typeof PlasmaCollections)[keyof typeof PlasmaCollections]

// NVNN: ChromaDBConfig defines client configuration
interface ChromaDBConfig {
  endpoint: string
  embeddingEndpoint: string
  timeoutMs: number
}

// NVNN: QueryRequest structures ChromaDB query input
interface QueryRequest {
  query_embeddings: number[][]
  n_results: number
  where?: Record<string, unknown>
  include: string[]
}

// NVNN: QueryResponse matches ChromaDB API response
interface QueryResponse {
  ids: string[][]
  distances?: number[][]
  documents?: (string | null)[][]
  metadatas?: Record<string, unknown>[][]
}

// NVNN: AddRequest structures document addition input
interface AddRequest {
  ids: string[]
  embeddings: number[][]
  documents?: string[]
  metadatas?: Record<string, unknown>[]
}

/**
 * Generate embedding vector from text using embedding service
 * NVNN: generateEmbedding converts text to 384-dim vector
 */
async function generateEmbedding(
  text: string,
  config: ChromaDBConfig
): Promise<number[]> {
  const response = await fetch(`${config.embeddingEndpoint}/api/v1/embed`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ text }),
  })

  if (!response.ok) {
    throw new Error(`Embedding service error: ${response.status}`)
  }

  const data = await response.json()

  if (data.embedding.length !== EMBEDDING_DIM) {
    throw new Error(
      `Embedding dimension mismatch: expected ${EMBEDDING_DIM}, got ${data.embedding.length}`
    )
  }

  return data.embedding
}

/**
 * Query ChromaDB collection for similar vectors
 * NVNN: queryCollection executes semantic similarity search
 */
export async function queryCollection(
  collection: PlasmaCollection,
  queryText: string,
  options?: {
    nResults?: number
    filter?: Record<string, unknown>
    config?: Partial<ChromaDBConfig>
  }
): Promise<VectorSearchResult[]> {
  const cfg: ChromaDBConfig = {
    endpoint: options?.config?.endpoint || CHROMADB_ENDPOINT,
    embeddingEndpoint: options?.config?.embeddingEndpoint || EMBEDDING_ENDPOINT,
    timeoutMs: options?.config?.timeoutMs || 50,
  }

  try {
    // Generate query embedding
    const embedding = await generateEmbedding(queryText, cfg)

    // Query ChromaDB
    const queryRequest: QueryRequest = {
      query_embeddings: [embedding],
      n_results: options?.nResults || 10,
      include: ["documents", "metadatas", "distances"],
    }

    if (options?.filter) {
      queryRequest.where = options.filter
    }

    const response = await fetch(
      `${cfg.endpoint}/api/v1/collections/${collection}/query`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(queryRequest),
      }
    )

    if (!response.ok) {
      throw new Error(`ChromaDB query error: ${response.status}`)
    }

    const data: QueryResponse = await response.json()

    // Transform response to VectorSearchResult format
    const results: VectorSearchResult[] = []
    const ids = data.ids[0] || []
    const distances = data.distances?.[0] || []
    const documents = data.documents?.[0] || []
    const metadatas = data.metadatas?.[0] || []

    for (let i = 0; i < ids.length; i++) {
      // Convert distance to similarity score (1 - distance for cosine)
      const score = distances[i] !== undefined ? 1 - distances[i] : 0

      results.push({
        id: ids[i],
        score,
        document: documents[i] || undefined,
        metadata: metadatas[i] || {},
      })
    }

    return results
  } catch (error) {
    console.error(`[ChromaDB] Query error for ${collection}:`, error)
    return []
  }
}

/**
 * Add documents to ChromaDB collection
 * NVNN: addDocuments inserts new vectors into collection
 */
export async function addDocuments(
  collection: PlasmaCollection,
  documents: Array<{
    id: string
    text: string
    metadata?: Record<string, unknown>
  }>,
  config?: Partial<ChromaDBConfig>
): Promise<boolean> {
  const cfg: ChromaDBConfig = {
    endpoint: config?.endpoint || CHROMADB_ENDPOINT,
    embeddingEndpoint: config?.embeddingEndpoint || EMBEDDING_ENDPOINT,
    timeoutMs: config?.timeoutMs || 5000,
  }

  try {
    // Generate embeddings for all documents
    const embeddings = await Promise.all(
      documents.map((doc) => generateEmbedding(doc.text, cfg))
    )

    const addRequest: AddRequest = {
      ids: documents.map((d) => d.id),
      embeddings,
      documents: documents.map((d) => d.text),
      metadatas: documents.map((d) => d.metadata || {}),
    }

    const response = await fetch(
      `${cfg.endpoint}/api/v1/collections/${collection}/add`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(addRequest),
      }
    )

    if (!response.ok) {
      throw new Error(`ChromaDB add error: ${response.status}`)
    }

    return true
  } catch (error) {
    console.error(`[ChromaDB] Add error for ${collection}:`, error)
    return false
  }
}

/**
 * Index a threat in ChromaDB
 * NVNN: indexThreat stores threat embedding with metadata
 */
export async function indexThreat(
  threat: Threat,
  config?: Partial<ChromaDBConfig>
): Promise<string | null> {
  const embeddingText = `${threat.level} threat: ${threat.description}. Indicators: ${threat.indicators.join(", ")}. MITRE: ${threat.mitre.join(", ")}`

  const success = await addDocuments(
    PlasmaCollections.THREATS,
    [
      {
        id: `threat-${threat.id}`,
        text: embeddingText,
        metadata: {
          threat_id: threat.id,
          level: threat.level,
          source: threat.source,
          target: threat.target,
          mitre: threat.mitre,
          confidence: threat.confidence,
          timestamp: threat.timestamp,
          trivariate_hash: threat.trivariate_hash,
        },
      },
    ],
    config
  )

  return success ? `threat-${threat.id}` : null
}

/**
 * Find similar threats using semantic search
 * NVNN: findSimilarThreats queries vector space for related threats
 */
export async function findSimilarThreats(
  threat: Threat,
  options?: {
    nResults?: number
    minScore?: number
    domainFilter?: CognitiveDomain[]
    config?: Partial<ChromaDBConfig>
  }
): Promise<VectorSearchResult[]> {
  const queryText = `${threat.level} threat: ${threat.description}. MITRE: ${threat.mitre.join(", ")}`

  const results = await queryCollection(PlasmaCollections.THREATS, queryText, {
    nResults: options?.nResults || 10,
    config: options?.config,
  })

  // Filter by minimum score
  const minScore = options?.minScore || 0.7
  return results.filter((r) => r.score >= minScore)
}

/**
 * Query MITRE techniques related to threat
 * NVNN: findRelatedTechniques searches technique embeddings
 */
export async function findRelatedTechniques(
  description: string,
  nResults = 5,
  config?: Partial<ChromaDBConfig>
): Promise<VectorSearchResult[]> {
  return queryCollection(PlasmaCollections.TECHNIQUES, description, {
    nResults,
    config,
  })
}

/**
 * Query detection rules for threat indicators
 * NVNN: findDetectionRules searches detection rule embeddings
 */
export async function findDetectionRules(
  indicators: string[],
  nResults = 5,
  config?: Partial<ChromaDBConfig>
): Promise<VectorSearchResult[]> {
  const queryText = indicators.join(" | ")
  return queryCollection(PlasmaCollections.DETECTION_RULES, queryText, {
    nResults,
    config,
  })
}

/**
 * Create or ensure collection exists
 * NVNN: ensureCollection creates collection if not present
 */
export async function ensureCollection(
  collection: PlasmaCollection,
  config?: Partial<ChromaDBConfig>
): Promise<boolean> {
  const cfg: ChromaDBConfig = {
    endpoint: config?.endpoint || CHROMADB_ENDPOINT,
    embeddingEndpoint: config?.embeddingEndpoint || EMBEDDING_ENDPOINT,
    timeoutMs: config?.timeoutMs || 5000,
  }

  try {
    const response = await fetch(
      `${cfg.endpoint}/api/v1/collections`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name: collection,
          metadata: {
            description: `Plasma ${collection} collection`,
            embedding_dim: EMBEDDING_DIM,
            model: "all-MiniLM-L6-v2",
          },
        }),
      }
    )

    // 409 means collection already exists (which is fine)
    if (response.ok || response.status === 409) {
      return true
    }

    throw new Error(`Failed to create collection: ${response.status}`)
  } catch (error) {
    console.error(`[ChromaDB] Collection creation error:`, error)
    return false
  }
}

/**
 * Health check for ChromaDB service
 * NVNN: healthCheck verifies ChromaDB availability
 */
export async function healthCheck(
  config?: Partial<ChromaDBConfig>
): Promise<boolean> {
  const endpoint = config?.endpoint || CHROMADB_ENDPOINT

  try {
    const response = await fetch(`${endpoint}/api/v1/heartbeat`)
    return response.ok
  } catch {
    return false
  }
}

/**
 * Get collection statistics
 * NVNN: getCollectionStats retrieves collection metadata
 */
export async function getCollectionStats(
  collection: PlasmaCollection,
  config?: Partial<ChromaDBConfig>
): Promise<{ count: number } | null> {
  const endpoint = config?.endpoint || CHROMADB_ENDPOINT

  try {
    const response = await fetch(
      `${endpoint}/api/v1/collections/${collection}`
    )

    if (!response.ok) {
      return null
    }

    const data = await response.json()
    return { count: data.count || 0 }
  } catch {
    return null
  }
}
