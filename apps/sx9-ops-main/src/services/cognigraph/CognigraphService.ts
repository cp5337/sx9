// Note: crypto module is Node.js specific, will need polyfill for browser
// For now using a simple hash function
function simpleHash(input: string): string {
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    const char = input.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash).toString(16);
}

// Core Cognigraph Types based on Universal Cognigraph Paper
export interface PhysicalProperties {
  mass: number;           // m: Mass/resource requirement ∈ ℝ⁺
  resource_cost: number;  // c_r: Resource consumption rate ∈ ℝ⁺
  energy_footprint: number; // e_f: Energy footprint ∈ ℝ⁺
}

export interface TemporalProperties {
  activation_time: number; // t_a: Activation time ∈ ℝ⁺
  duration: number;        // d: Duration ∈ ℝ⁺ ∪ {∞}
  decay: number;          // τ: Decay/cooldown time ∈ ℝ⁺
}

export interface EnergeticProperties {
  consumption: number;     // e_c: Energy consumption rate ∈ ℝ⁺
  generation: number;      // e_g: Energy generation rate ∈ ℝ⁺
  threshold: number;       // e_t: Activation threshold ∈ ℝ⁺
}

export interface SpatialProperties {
  interaction_radius: number; // r_i: Interaction radius ∈ ℝ⁺
  exclusion_radius: number;   // r_e: Exclusion radius ∈ ℝ⁺
  volume: number[];           // V_o: Occupied volume ∈ ℝ³
}

export interface RelationalProperties {
  connectivity: number;       // κ: Connectivity capacity ∈ ℕ
  dependencies: string[];     // δ: Dependency vector ∈ {0,1}ⁿ
  interaction_matrix: number[][]; // ι: Interaction strength matrix ∈ ℝⁿˣⁿ
}

export interface EconomicProperties {
  setup_cost: number;         // c_s: Setup cost ∈ ℝ⁺
  maintenance_cost: number;   // c_m: Maintenance cost rate ∈ ℝ⁺
  opportunity_cost: number;   // c_o: Opportunity cost ∈ ℝ⁺
  depreciation_rate: number;  // c_d: Depreciation rate ∈ [0,1]
}

// The Ten Universal Node Types (B₁ through B₁₀)
export enum NodeType {
  Source = 'Source',           // B₁: Emits resources, data, or energy
  Sink = 'Sink',              // B₂: Absorbs waste, output, or terminal state
  Transformer = 'Transformer', // B₃: Converts inputs to outputs
  Router = 'Router',          // B₄: Controls directional flow
  Buffer = 'Buffer',          // B₅: Temporarily holds state or resources
  Gate = 'Gate',              // B₆: Implements conditional access control
  Monitor = 'Monitor',        // B₇: Observes system behavior
  Catalyst = 'Catalyst',      // B₈: Accelerates interactions
  Inhibitor = 'Inhibitor',    // B₉: Blocks or throttles activity
  Relay = 'Relay'             // B₁₀: Extends interaction range
}

export enum CTASNodeType {
  People = 'People',
  Object = 'Object',
  Location = 'Location',
  Event = 'Event',
  Document = 'Document',
  Signal = 'Signal',
  Activity = 'Activity',
  Plan = 'Plan',
  Path = 'Path',
  Behavior = 'Behavior',
  Capability = 'Capability',
  Resource = 'Resource',
  Intent = 'Intent',
  Belief = 'Belief',
  Narrative = 'Narrative',
  Role = 'Role',
  Need = 'Need',
  Risk = 'Risk',
  Tool = 'Tool'
}

export enum HD4Phase {
  Hunt = 'Hunt',
  Detect = 'Detect',
  Disrupt = 'Disrupt',
  Disable = 'Disable',
  Dominate = 'Dominate',
  CyberKillChain = 'CyberKillChain'
}

export interface MCPContext {
  llm_response?: string;
  vector_db_results: string[];
}

// Cognitive Atom: A = (P, T, E, S, R, Φ)
export interface CognitiveAtom {
  id: string; // SCH identifier
  p: PhysicalProperties;      // P: Physical properties
  t: TemporalProperties;      // T: Temporal characteristics
  e: EnergeticProperties;     // E: Energetic interaction profile
  s: SpatialProperties;       // S: Spatial envelope
  r: RelationalProperties;    // R: Relational profile
  phi: EconomicProperties;    // Φ: Economic/operational costs
  node_type: NodeType;
  ctas_node_type: CTASNodeType;
  att_ck_techniques: string[];
  hd4_phase: HD4Phase;
  cyber_kill_chain?: string;
  mcp_context: MCPContext;
}

// Performance and Environmental Context
export interface PerformanceMetrics {
  efficiency: number;
  throughput: number;
  latency: number;
  reliability: number;
  resource_utilization: number;
  energy_efficiency: number;
  cost_efficiency: number;
}

export enum TemporalPhase {
  Initiation = 'Initiation',
  Growth = 'Growth',
  Maturity = 'Maturity',
  Decline = 'Decline'
}

export enum SpatialConditions {
  Constrained = 'Constrained',
  Balanced = 'Balanced',
  Expansive = 'Expansive'
}

export enum EconomicConditions {
  Scarce = 'Scarce',
  Balanced = 'Balanced',
  Abundant = 'Abundant'
}

export interface EnvironmentalContext {
  system_load: number;
  resource_availability: number;
  external_pressure: number;
  temporal_phase: TemporalPhase;
  spatial_conditions: SpatialConditions;
  economic_conditions: EconomicConditions;
}

// Interaction Types
export enum InteractionType {
  Transfer = 'Transfer',
  Transform = 'Transform',
  Inhibit = 'Inhibit',
  Catalyze = 'Catalyze',
  Monitor = 'Monitor'
}

export enum OutcomeType {
  Success = 'Success',
  Partial = 'Partial',
  Failure = 'Failure',
  Neutral = 'Neutral'
}

export interface InteractionData {
  force_magnitude: number;
  force_direction: [number, number, number];
  energy_transfer: number;
  temporal_duration: number;
  spatial_distance: number;
  interaction_parameters: Map<string, number>;
}

export interface InteractionOutcome {
  actual_force: number;
  actual_success: number;
  actual_duration: number;
  actual_energy_transfer: number;
  outcome_type: OutcomeType;
}

// Graph Edge Parameters: θᵢⱼ = (w, λ, τ, κ)
export interface EdgeParameters {
  weight: number;        // w: Interaction weight ∈ ℝ
  latency: number;       // λ: Latency ∈ ℝ⁺
  throughput: number;    // τ: Throughput capacity ∈ ℝ⁺
  cost_coefficient: number; // κ: Cost coefficient ∈ ℝ⁺
}

// System Graph: G = (V, E)
export interface SystemGraph {
  nodes: CognitiveAtom[];
  edges: Array<{
    source: string;
    target: string;
    parameters: EdgeParameters;
  }>;
}

// Main Cognigraph Service
export class CognigraphService {
  private neo4jDriver: any;
  private supabaseClient: any;
  private llmClient: any;
  private systemGraph: SystemGraph;
  private isDemoMode: boolean;

  constructor() {
    this.neo4jDriver = null;
    this.supabaseClient = null;
    this.llmClient = null;
    this.systemGraph = { nodes: [], edges: [] };
    this.isDemoMode = import.meta.env.VITE_DEMO_MODE === 'true';
  }

  /**
   * Compute Synaptic Convergent Hash (SCH)
   * Based on double hash with metadata
   */
  public computeSCH(meta: any): string {
    const metaString = JSON.stringify(meta);
    const firstHash = simpleHash(metaString);
    const secondHash = simpleHash(firstHash + (meta.slug || ''));
    return secondHash;
  }

  /**
   * Create a new Cognitive Atom with proper initialization
   */
  public async createCognitiveAtom(meta: any): Promise<CognitiveAtom> {
    const slug = meta.slug || 'unknown';
    const id = this.computeSCH(meta);
    const mcpContext = await this.applyMCP(meta);
    const hd4Phase = this.parseHD4Phase(meta.hd4_phase);
    
    // Initialize with default values based on node type
    const nodeType = this.determineNodeType(meta);
    const defaultProps = this.getDefaultProperties(nodeType);
    
    const atom: CognitiveAtom = {
      id,
      p: defaultProps.physical,
      t: defaultProps.temporal,
      e: defaultProps.energetic,
      s: defaultProps.spatial,
      r: {
        connectivity: 0,
        dependencies: meta.dependencies || [],
        interaction_matrix: []
      },
      phi: defaultProps.economic,
      node_type: nodeType,
      ctas_node_type: CTASNodeType.Tool,
      att_ck_techniques: meta.att_ck_techniques || [],
      hd4_phase: hd4Phase,
      cyber_kill_chain: meta.cyber_kill_chain,
      mcp_context: mcpContext
    };

    // Add to system graph
    this.systemGraph.nodes.push(atom);
    
    // Store in databases
    await this.storeInNeo4j(atom, slug);
    await this.storeInSupabase(atom);

    return atom;
  }

  /**
   * Calculate atomic interaction force between two nodes
   * F_ij = k · (P_i · P_j) / r_ij² · compat(R_i, R_j) · temporal_factor(T_i, T_j)
   */
  public calculateInteractionForce(atom1: CognitiveAtom, atom2: CognitiveAtom, distance: number): number {
    const k = 1.0; // Universal interaction constant
    
    // Physical interaction: P_i · P_j
    const physicalInteraction = atom1.p.mass * atom2.p.mass;
    
    // Distance factor: 1 / r_ij²
    const distanceFactor = 1 / (distance * distance);
    
    // Compatibility function: compat(R_i, R_j) ∈ {-1, 0, +1}
    const compatibility = this.calculateCompatibility(atom1, atom2);
    
    // Temporal synchronization factor
    const temporalFactor = this.calculateTemporalFactor(atom1, atom2);
    
    return k * physicalInteraction * distanceFactor * compatibility * temporalFactor;
  }

  /**
   * Calculate compatibility between two atoms
   */
  private calculateCompatibility(atom1: CognitiveAtom, atom2: CognitiveAtom): number {
    // Check for synergistic relationships
    if (this.isSynergistic(atom1, atom2)) return 1;
    
    // Check for conflicting relationships
    if (this.isConflicting(atom1, atom2)) return -1;
    
    // Independent relationship
    return 0;
  }

  /**
   * Calculate temporal synchronization factor
   * temporal_factor(T_i, T_j) = exp(-|t_a^i - t_a^j| / τ_sync)
   */
  private calculateTemporalFactor(atom1: CognitiveAtom, atom2: CognitiveAtom): number {
    const τ_sync = 1.0; // Temporal coupling constant
    const timeDiff = Math.abs(atom1.t.activation_time - atom2.t.activation_time);
    return Math.exp(-timeDiff / τ_sync);
  }

  /**
   * Check if atoms are synergistic (complementary resources)
   */
  private isSynergistic(atom1: CognitiveAtom, atom2: CognitiveAtom): boolean {
    // Example: Source + Sink, Transformer + Buffer
    return (atom1.node_type === NodeType.Source && atom2.node_type === NodeType.Sink) ||
           (atom1.node_type === NodeType.Transformer && atom2.node_type === NodeType.Buffer);
  }

  /**
   * Check if atoms are conflicting (competing for resources)
   */
  private isConflicting(atom1: CognitiveAtom, atom2: CognitiveAtom): boolean {
    // Example: Two Sources competing for same Sink
    return atom1.node_type === NodeType.Source && atom2.node_type === NodeType.Source;
  }

  /**
   * Determine node type based on metadata
   */
  private determineNodeType(meta: any): NodeType {
    if (meta.emits_resources) return NodeType.Source;
    if (meta.absorbs_output) return NodeType.Sink;
    if (meta.transforms_input) return NodeType.Transformer;
    if (meta.routes_flow) return NodeType.Router;
    if (meta.buffers_state) return NodeType.Buffer;
    if (meta.controls_access) return NodeType.Gate;
    if (meta.monitors_behavior) return NodeType.Monitor;
    if (meta.accelerates_process) return NodeType.Catalyst;
    if (meta.inhibits_activity) return NodeType.Inhibitor;
    if (meta.extends_range) return NodeType.Relay;
    
    return NodeType.Transformer; // Default
  }

  /**
   * Get default properties for node type
   */
  private getDefaultProperties(nodeType: NodeType) {
    const defaults: Record<NodeType, any> = {
      [NodeType.Source]: {
        physical: { mass: 1.0, resource_cost: 0.0, energy_footprint: 10.0 },
        temporal: { activation_time: 0.0, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 5.0, generation: 10.0, threshold: 2.0 },
        spatial: { interaction_radius: 5.0, exclusion_radius: 1.0, volume: [1, 1, 1] },
        economic: { setup_cost: 100.0, maintenance_cost: 5.0, opportunity_cost: 0.0, depreciation_rate: 0.1 }
      },
      [NodeType.Sink]: {
        physical: { mass: 2.0, resource_cost: 0.0, energy_footprint: 5.0 },
        temporal: { activation_time: 0.1, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 10.0, generation: 0.0, threshold: 5.0 },
        spatial: { interaction_radius: 3.0, exclusion_radius: 2.0, volume: [2, 2, 2] },
        economic: { setup_cost: 200.0, maintenance_cost: 10.0, opportunity_cost: 0.0, depreciation_rate: 0.15 }
      },
      [NodeType.Transformer]: {
        physical: { mass: 1.5, resource_cost: 5.0, energy_footprint: 15.0 },
        temporal: { activation_time: 0.2, duration: 10.0, decay: 1.0 },
        energetic: { consumption: 15.0, generation: 0.0, threshold: 8.0 },
        spatial: { interaction_radius: 4.0, exclusion_radius: 1.5, volume: [1.5, 1.5, 1.5] },
        economic: { setup_cost: 500.0, maintenance_cost: 20.0, opportunity_cost: 10.0, depreciation_rate: 0.2 }
      },
      [NodeType.Router]: {
        physical: { mass: 1.0, resource_cost: 2.0, energy_footprint: 8.0 },
        temporal: { activation_time: 0.05, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 8.0, generation: 0.0, threshold: 3.0 },
        spatial: { interaction_radius: 6.0, exclusion_radius: 1.0, volume: [1, 1, 1] },
        economic: { setup_cost: 300.0, maintenance_cost: 15.0, opportunity_cost: 5.0, depreciation_rate: 0.15 }
      },
      [NodeType.Buffer]: {
        physical: { mass: 1.2, resource_cost: 1.0, energy_footprint: 6.0 },
        temporal: { activation_time: 0.0, duration: Infinity, decay: 0.5 },
        energetic: { consumption: 3.0, generation: 0.0, threshold: 1.0 },
        spatial: { interaction_radius: 2.0, exclusion_radius: 0.5, volume: [1, 1, 1] },
        economic: { setup_cost: 150.0, maintenance_cost: 8.0, opportunity_cost: 2.0, depreciation_rate: 0.1 }
      },
      [NodeType.Gate]: {
        physical: { mass: 0.8, resource_cost: 0.5, energy_footprint: 4.0 },
        temporal: { activation_time: 0.01, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 2.0, generation: 0.0, threshold: 1.0 },
        spatial: { interaction_radius: 1.0, exclusion_radius: 0.5, volume: [0.5, 0.5, 0.5] },
        economic: { setup_cost: 100.0, maintenance_cost: 5.0, opportunity_cost: 1.0, depreciation_rate: 0.1 }
      },
      [NodeType.Monitor]: {
        physical: { mass: 0.5, resource_cost: 0.2, energy_footprint: 2.0 },
        temporal: { activation_time: 0.0, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 1.0, generation: 0.0, threshold: 0.5 },
        spatial: { interaction_radius: 3.0, exclusion_radius: 0.2, volume: [0.3, 0.3, 0.3] },
        economic: { setup_cost: 50.0, maintenance_cost: 2.0, opportunity_cost: 0.5, depreciation_rate: 0.05 }
      },
      [NodeType.Catalyst]: {
        physical: { mass: 0.3, resource_cost: 0.1, energy_footprint: 1.0 },
        temporal: { activation_time: 0.0, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 0.5, generation: 0.0, threshold: 0.2 },
        spatial: { interaction_radius: 2.0, exclusion_radius: 0.1, volume: [0.2, 0.2, 0.2] },
        economic: { setup_cost: 25.0, maintenance_cost: 1.0, opportunity_cost: 0.2, depreciation_rate: 0.05 }
      },
      [NodeType.Inhibitor]: {
        physical: { mass: 0.4, resource_cost: 0.1, energy_footprint: 1.5 },
        temporal: { activation_time: 0.0, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 0.8, generation: 0.0, threshold: 0.3 },
        spatial: { interaction_radius: 2.5, exclusion_radius: 0.2, volume: [0.25, 0.25, 0.25] },
        economic: { setup_cost: 30.0, maintenance_cost: 1.5, opportunity_cost: 0.3, depreciation_rate: 0.05 }
      },
      [NodeType.Relay]: {
        physical: { mass: 0.6, resource_cost: 0.3, energy_footprint: 3.0 },
        temporal: { activation_time: 0.02, duration: Infinity, decay: 0.0 },
        energetic: { consumption: 1.5, generation: 0.0, threshold: 0.8 },
        spatial: { interaction_radius: 4.0, exclusion_radius: 0.3, volume: [0.4, 0.4, 0.4] },
        economic: { setup_cost: 75.0, maintenance_cost: 3.0, opportunity_cost: 0.8, depreciation_rate: 0.08 }
      }
    };

    return defaults[nodeType] || defaults[NodeType.Transformer];
  }

  /**
   * Track atom properties and performance
   */
  public async trackAtomProperties(
    atomId: string,
    atom: CognitiveAtom,
    performance: PerformanceMetrics,
    context: EnvironmentalContext
  ): Promise<void> {
    console.log(`Tracking atom ${atomId}:`, {
      efficiency: performance.efficiency,
      node_type: atom.node_type,
      hd4_phase: atom.hd4_phase,
      environmental_context: context
    });
  }

  /**
   * Track system properties
   */
  public async trackSystemProperties(
    systemId: string,
    atoms: CognitiveAtom[],
    interactions: any[]
  ): Promise<void> {
    const activeAtoms = atoms.filter(atom => this.isAtomActive(atom));
    const systemEntropy = this.calculateSystemEntropy(atoms);
    
    console.log(`System ${systemId} tracking:`, {
      total_atoms: atoms.length,
      active_atoms: activeAtoms.length,
      system_entropy: systemEntropy,
      total_energy_consumption: atoms.reduce((sum, atom) => sum + atom.e.consumption, 0)
    });
  }

  /**
   * Track interactions between atoms
   */
  public async trackInteraction(
    sourceId: string,
    targetId: string,
    interactionType: InteractionType,
    data: InteractionData,
    outcome: InteractionOutcome
  ): Promise<void> {
    console.log(`Interaction tracked:`, {
      source: sourceId,
      target: targetId,
      type: interactionType,
      success_rate: outcome.actual_success,
      energy_transfer: outcome.actual_energy_transfer,
      force_magnitude: data.force_magnitude
    });
  }

  /**
   * Calculate system entropy
   */
  private calculateSystemEntropy(atoms: CognitiveAtom[]): number {
    if (atoms.length === 0) return 0;
    
    const totalEnergy = atoms.reduce((sum, atom) => sum + atom.e.consumption, 0);
    const avgEnergy = totalEnergy / atoms.length;
    const variance = atoms.reduce((sum, atom) => {
      const diff = atom.e.consumption - avgEnergy;
      return sum + diff * diff;
    }, 0) / atoms.length;
    
    return Math.sqrt(variance) / avgEnergy; // Normalized entropy
  }

  /**
   * Check if atom is active
   */
  private isAtomActive(atom: CognitiveAtom): boolean {
    return atom.e.consumption > atom.e.threshold && atom.t.decay > 0;
  }

  /**
   * Parse HD4 Phase from string
   */
  private parseHD4Phase(phase: string): HD4Phase {
    switch (phase) {
      case 'Hunt': return HD4Phase.Hunt;
      case 'Detect': return HD4Phase.Detect;
      case 'Disrupt': return HD4Phase.Disrupt;
      case 'Disable': return HD4Phase.Disable;
      case 'Dominate': return HD4Phase.Dominate;
      default: return HD4Phase.CyberKillChain;
    }
  }

  /**
   * Apply MCP (Model Context Protocol) - Production implementation with demo fallback
   */
  private async applyMCP(meta: any): Promise<MCPContext> {
    const prompt = `Generate context for tool: ${meta.slug || 'unknown'}`;
    
    try {
      // Production implementation
      const llmResponse = await this.callLLM(prompt);
      const vectorResults = await this.searchVectorDB(prompt);
      
      return {
        llm_response: llmResponse,
        vector_db_results: vectorResults
      };
    } catch (error) {
      // Fallback to demo data if production fails
      if (this.isDemoMode) {
        const { demoDataProvider } = await import('../../utils/demoDataProvider');
        return {
          llm_response: demoDataProvider.getLLMResponse(prompt),
          vector_db_results: demoDataProvider.getVectorDBResults(prompt)
        };
      }
      throw error;
    }
  }

  /**
   * Call LLM service - Production implementation
   */
  private async callLLM(prompt: string): Promise<string> {
    if (this.isDemoMode) {
      const { demoDataProvider } = await import('../../utils/demoDataProvider');
      return demoDataProvider.getLLMResponse(prompt);
    }
    
    // Production LLM integration would go here
    // This is where you'd implement actual LLM API calls
    throw new Error('LLM service not configured');
  }

  /**
   * Search vector database - Production implementation
   */
  private async searchVectorDB(query: string): Promise<string[]> {
    if (this.isDemoMode) {
      const { demoDataProvider } = await import('../../utils/demoDataProvider');
      return demoDataProvider.getVectorDBResults(query);
    }
    
    // Production vector DB search would go here
    // This is where you'd implement actual vector database queries
    throw new Error('Vector database not configured');
  }

  /**
   * Store atom in Neo4j - Production implementation
   */
  private async storeInNeo4j(atom: CognitiveAtom, slug: string): Promise<void> {
    if (this.isDemoMode) {
      const { demoDataProvider } = await import('../../utils/demoDataProvider');
      await demoDataProvider.demoDatabaseOperation('neo4j_store', { atom, slug });
      return;
    }
    
    // Production Neo4j storage would go here
    // This is where you'd implement actual Neo4j operations
    throw new Error('Neo4j not configured');
  }

  /**
   * Store atom in Supabase - Production implementation
   */
  private async storeInSupabase(atom: CognitiveAtom): Promise<void> {
    if (this.isDemoMode) {
      const { demoDataProvider } = await import('../../utils/demoDataProvider');
      await demoDataProvider.demoDatabaseOperation('supabase_store', { atom });
      return;
    }
    
    // Production Supabase storage would go here
    // This is where you'd implement actual Supabase operations
    throw new Error('Supabase not configured');
  }

  /**
   * Get system graph
   */
  public getSystemGraph(): SystemGraph {
    return this.systemGraph;
  }

  /**
   * Add edge to system graph
   */
  public addEdge(sourceId: string, targetId: string, parameters: EdgeParameters): void {
    this.systemGraph.edges.push({
      source: sourceId,
      target: targetId,
      parameters
    });
  }

  /**
   * Optimize system graph - Production implementation
   */
  public async optimizeSystem(): Promise<SystemGraph> {
    if (this.isDemoMode) {
      const { demoDataProvider } = await import('../../utils/demoDataProvider');
      const results = demoDataProvider.getOptimizationResults();
      console.log('Demo optimization results:', results);
      return this.systemGraph;
    }
    
    // Production optimization algorithm would go here
    // This is where you'd implement actual graph optimization
    throw new Error('Optimization algorithm not configured');
  }
}

// Export singleton instance
export const cognigraphService = new CognigraphService();
