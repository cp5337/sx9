/**
 * SX9 Task Types with Dual Trivariate Hash Architecture
 *
 * SX9 7.3.1 Specification: Dual Hash System
 * ALL HASHES USE MurmurHash3 with SCH-CUID-UUID trivariate structure
 *
 * OPERATIONAL HASH (op_*): MurmurHash3 SCH-CUID-UUID for routing/caching
 * SEMANTIC HASH (sem_*): MurmurHash3 SCH-CUID-UUID for content/semantic analysis
 *
 * SCH-CUID-UUID Structure (48 characters, Base96 encoded):
 * - Characters 1-16: SCH (Short-Hand Concept) - MurmurHash3 seed 0
 * - Characters 17-32: CUID (Contextual Unique ID) - MurmurHash3 seed + timestamp
 * - Characters 33-48: UUID (Universal Unique ID) - MurmurHash3 random seed
 *
 * Naming Convention:
 * - op_*  = Operational routing hashes (MurmurHash3 trivariate)
 * - sem_* = Semantic understanding hashes (MurmurHash3 trivariate)
 */

export type PrimitiveType =
  | "Concept" // Abstract ideas, strategies, methodologies
  | "Actor" // Human entities, agents, personas
  | "Object" // Physical/digital objects, tools, systems
  | "Event" // Actions, occurrences, incidents
  | "Attribute" // Properties, characteristics, qualities
  | "Unclassified"; // Not yet classified or multi-type

export type HD4Phase =
  | "Hunt" // Intelligence gathering, reconnaissance
  | "Detect" // Identification, monitoring, discovery
  | "Disrupt" // Interference, degradation, obstruction
  | "Disable" // Neutralization, destruction, elimination
  | "Dominate" // Control, supremacy, sustained operations
  | "All"; // Cross-phase or universal operations

/**
 * Operational Hash Fields (MurmurHash3 SCH-CUID-UUID trivariate, Base96 encoded)
 * Used for deterministic routing, caching, and system coordination
 * Each hash is 48 characters: SCH(16) + CUID(16) + UUID(16)
 */
export interface OperationalHashFields {
  /** MurmurHash3 SCH-CUID-UUID from task content (name + description + category) */
  op_content_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID from primitive_type */
  op_primitive_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID from predecessors + successors chain */
  op_chain_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID from p/t/h metrics combined */
  op_pth_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID composite of all operational hashes */
  op_composite_hash: string | null;
}

/**
 * Semantic Hash Fields (MurmurHash3 SCH-CUID-UUID trivariate, Base96 encoded)
 * Used for content similarity, semantic search, and ML operations
 * Each hash is 48 characters: SCH(16) + CUID(16) + UUID(16)
 */
export interface SemanticHashFields {
  /** MurmurHash3 SCH-CUID-UUID from semantic content analysis */
  sem_content_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID from semantic category/classification */
  sem_category_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID from semantic relationships and context */
  sem_relationship_hash: string | null;

  /** MurmurHash3 SCH-CUID-UUID composite semantic fingerprint */
  sem_composite_hash: string | null;
}

/**
 * PTH Metrics (Probability, Time, Hazard)
 * Statistical measures for task analysis and risk assessment
 */
export interface PTHMetrics {
  /** Probability factor (0.0 - 1.0) */
  p_probability: number;

  /** Time factor (0.0 - 1.0) */
  t_time: number;

  /** Hazard factor (0.0 - 1.0) */
  h_hazard: number;
}

/**
 * Complete SX9 Task Structure
 * Combines all operational, semantic, and analytical components
 */
export interface SX9Task
  extends OperationalHashFields,
    SemanticHashFields,
    PTHMetrics {
  // Core Identification
  id?: string; // Supabase UUID (auto-generated)
  hash_id: string; // Original task identifier from CSV
  task_name: string; // Human-readable task name
  description: string; // Detailed task description

  // Classification
  category: string; // Task category/domain
  hd4_phase: HD4Phase; // HD4 operational phase
  primitive_type: PrimitiveType; // PTCC primitive classification

  // Graph Relationships
  predecessors: string[]; // Array of predecessor hash_ids
  successors: string[]; // Array of successor hash_ids

  // Sequencing
  task_seq: number; // Sequential ordering number

  // Metadata
  created_at?: string; // ISO timestamp
  updated_at?: string; // ISO timestamp
}

/**
 * Task Query Filters
 * Common filter patterns for task retrieval
 */
export interface TaskQueryFilters {
  hd4_phase?: HD4Phase | HD4Phase[];
  primitive_type?: PrimitiveType | PrimitiveType[];
  category?: string | string[];

  // Hash-based filters
  op_composite_hash?: string; // Find by operational hash
  sem_composite_hash?: string; // Find by semantic hash

  // PTH range filters
  p_min?: number;
  p_max?: number;
  t_min?: number;
  t_max?: number;
  h_min?: number;
  h_max?: number;
}

/**
 * Hash Generation Request
 * Used when requesting hash computation from hashing engine
 */
export interface HashGenerationRequest {
  task_id: string; // Task identifier
  task_data: Partial<SX9Task>; // Task data to hash

  // Hash type selection
  generate_operational: boolean; // Generate op_* hashes
  generate_semantic: boolean; // Generate sem_* hashes

  // Algorithm specification (both use MurmurHash3 with SCH-CUID-UUID trivariate)
  operational_algorithm: "murmur3";
  semantic_algorithm: "murmur3";

  // Output format
  format: "base96" | "hex" | "base64";
}

/**
 * Hash Generation Response
 * Returned from hashing engine with computed hashes
 */
export interface HashGenerationResponse {
  task_id: string;

  // Computed hashes
  operational_hashes?: OperationalHashFields;
  semantic_hashes?: SemanticHashFields;

  // Metadata
  algorithm_versions: {
    murmur3_operational?: string;
    murmur3_semantic?: string;
  };
  computation_time_ms: number;
  format: string;
  timestamp: string;
}

/**
 * Task Graph Node
 * Used for graph visualization and analysis
 */
export interface TaskGraphNode {
  task: SX9Task;

  // Computed graph properties
  depth: number; // Distance from root
  in_degree: number; // Number of predecessors
  out_degree: number; // Number of successors

  // Derived classifications
  is_leaf: boolean; // No successors
  is_root: boolean; // No predecessors
  is_critical: boolean; // On critical path

  // Hash-based clustering
  operational_cluster?: string; // Cluster ID from op hashes
  semantic_cluster?: string; // Cluster ID from sem hashes
}

/**
 * Task Statistics
 * Aggregate statistics for task analysis
 */
export interface TaskStatistics {
  total_tasks: number;

  // Phase distribution
  by_phase: Record<HD4Phase, number>;

  // Type distribution
  by_primitive: Record<PrimitiveType, number>;

  // PTH statistics
  pth_averages: {
    probability: number;
    time: number;
    hazard: number;
  };

  // Graph statistics
  max_depth: number;
  avg_predecessors: number;
  avg_successors: number;

  // Hash coverage
  hash_coverage: {
    operational: number; // Percentage with op hashes
    semantic: number; // Percentage with sem hashes
  };
}
