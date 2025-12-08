/**
 * SX9 ChromaDB Client
 * ===================
 * 
 * TypeScript client for ChromaDB vector database integration
 * RFC-9021: Layer 2a Vector Search Service
 * 
 * 384-dimensional vector search using all-MiniLM-L6-v2 embeddings.
 * Target latency: <50ms P95, <100ms P99
 */

// ChromaDB and embedding service endpoints
const CHROMADB_ENDPOINT = process.env.CHROMADB_URL || "http://localhost:8000"
const EMBEDDING_ENDPOINT = process.env.EMBEDDING_URL || "http://localhost:18117"

// RFC-9021: 384-dimensional embeddings (all-MiniLM-L6-v2)
const EMBEDDING_DIM = 384

// PlasmaCollections defines available vector collections
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

// ChromaDBConfig defines client configuration
export interface ChromaDBConfig {
  endpoint: string
  embeddingEndpoint: string
  timeoutMs: number
}

// QueryRequest structures ChromaDB query input
interface QueryRequest {
  query_embeddings: number[][]
  n_results: number
  where?: Record<string, unknown>
  include: string[]
}

// QueryResponse matches ChromaDB API response
interface QueryResponse {
  ids: string[][]
  distances?: number[][]
  documents?: (string | null)[][]
  metadatas?: Record<string, unknown>[][]
}

// AddRequest structures document addition input
interface AddRequest {
  ids: string[]
  embeddings: number[][]
  documents?: string[]
  metadatas?: Record<string, unknown>[]
}

// VectorSearchResult represents a search result
export interface VectorSearchResult {
  id: string
  score: number
  document?: string
  metadata: Record<string, unknown>
}

/**
 * Generate embedding vector from text using embedding service
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
 * Health check for ChromaDB service
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

/**
 * Ensure collection exists
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


