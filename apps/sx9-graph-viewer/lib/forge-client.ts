/**
 * SX9 Forge API Client
 * Connects graph-viewer to sx9-forge backend (port 18350)
 */

const FORGE_BASE_URL = process.env.NEXT_PUBLIC_FORGE_URL || 'http://localhost:18350';

// Types matching Rust sx9-forge definitions
export interface ForgeNode {
  id: string;
  node_type: 'Nonagon' | 'Tool' | 'MissionLoad' | 'OssecRule' | 'DataSource' | 'Result';
  label: string;
  teth_entropy?: number;
  properties: Record<string, string>;
  created_at: string;
  updated_at: string;
}

export interface ForgeEdge {
  id: string;
  edge_type: 'UsesCell' | 'DerivedFrom' | 'ExecutesRule' | 'DataFlow' | 'DependsOn' | 'Contains' | 'Triggers';
  source_id: string;
  target_id: string;
  weight: number;
  properties: Record<string, string>;
}

export interface NonagonCell {
  id: string;
  alpha: { context: number; meaning: number; intent: number };
  beta: { phase: number; intensity: number; duration: number };
  gamma: { historical: number; current: number; predictive: number };
  teth_entropy: number;
  confidence: number;
  vertices: number[];
}

export interface MissionLoad {
  id: string;
  name: string;
  description: string;
  hd4_phase: 'Hunt' | 'Detect' | 'Disrupt' | 'Disable' | 'Dominate';
  clearance: 'Public' | 'Commercial' | 'Restricted' | 'Classified';
  primitives: string[];
  primitive_bitfield: number;
  price_credits: number;
  active: boolean;
}

export interface GeneratedTool {
  id: string;
  name: string;
  description: string;
  mission_load_id: string;
  hd4_phase: string;
  nonagon: NonagonCell;
  primitives: string[];
  ossec_rule_ids: string[];
  execution_count: number;
}

export interface ToolChain {
  id: string;
  name: string;
  description: string;
  nonagon: NonagonCell;
  tool_ids: string[];
  execution_mode: 'Sequential' | 'Parallel' | 'Graph';
}

export interface ForgeStatus {
  crate_name: string;
  version: string;
  ring_bus_node: number;
  l2_execution: boolean;
  graph_nodes: number;
  graph_edges: number;
  mission_loads: number;
  min_teth_entropy: number;
}

export interface ChainExecutionResult {
  chain_id: string;
  success: boolean;
  tool_results: Array<{
    tool_id: string;
    success: boolean;
    output?: string;
    error?: string;
    duration_ms: number;
  }>;
  total_duration_ms: number;
}

// API Client
class ForgeClient {
  private baseUrl: string;

  constructor(baseUrl: string = FORGE_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  private async fetch<T>(path: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });

    if (!response.ok) {
      throw new Error(`Forge API error: ${response.statusText}`);
    }

    return response.json();
  }

  // Health & Status
  async getHealth(): Promise<{ status: string; service: string; version: string }> {
    return this.fetch('/health');
  }

  async getStatus(): Promise<ForgeStatus> {
    return this.fetch('/smart-crate/status');
  }

  // Graph Operations
  async getGraphStats(): Promise<{ node_count: number; edge_count: number }> {
    return this.fetch('/graph');
  }

  async getNodes(): Promise<ForgeNode[]> {
    return this.fetch('/graph/nodes');
  }

  async getNode(id: string): Promise<ForgeNode> {
    return this.fetch(`/graph/nodes/${encodeURIComponent(id)}`);
  }

  // Nonagon Operations
  async createNonagon(id: string): Promise<{
    id: string;
    teth_entropy: number;
    confidence: number;
    is_valid: boolean;
    vertices: number[];
  }> {
    return this.fetch('/nonagon', {
      method: 'POST',
      body: JSON.stringify({ id }),
    });
  }

  async getNonagon(id: string): Promise<{
    id: string;
    teth_entropy: number;
    confidence: number;
    is_valid: boolean;
    vertices: number[];
  }> {
    return this.fetch(`/nonagon/${encodeURIComponent(id)}`);
  }

  async getNonagonEntropy(id: string): Promise<{
    id: string;
    teth_entropy: number;
    min_threshold: number;
    is_valid: boolean;
  }> {
    return this.fetch(`/nonagon/${encodeURIComponent(id)}/entropy`);
  }

  // Mission Load Operations
  async getMissionLoads(): Promise<Array<{
    id: string;
    name: string;
    hd4_phase: string;
    clearance: string;
    price_credits: number;
    primitive_count: number;
  }>> {
    return this.fetch('/mission-loads');
  }

  async getMissionLoad(id: string): Promise<MissionLoad> {
    return this.fetch(`/mission-loads/${encodeURIComponent(id)}`);
  }

  async createToolFromLoad(loadId: string): Promise<GeneratedTool> {
    return this.fetch(`/mission-loads/${encodeURIComponent(loadId)}/tool`, {
      method: 'POST',
    });
  }

  // Tool Operations
  async getTools(): Promise<GeneratedTool[]> {
    return this.fetch('/tools');
  }

  async executeTool(toolId: string): Promise<{ success: boolean; message: string }> {
    return this.fetch(`/tools/${encodeURIComponent(toolId)}/execute`, {
      method: 'POST',
    });
  }

  // Chain Operations
  async createChain(name: string, toolIds: string[]): Promise<ToolChain> {
    return this.fetch('/chains', {
      method: 'POST',
      body: JSON.stringify({ name, tool_ids: toolIds }),
    });
  }

  async executeChain(chainId: string, chain: ToolChain): Promise<ChainExecutionResult> {
    return this.fetch(`/chains/${encodeURIComponent(chainId)}/execute`, {
      method: 'POST',
      body: JSON.stringify(chain),
    });
  }
}

// Singleton instance
export const forgeClient = new ForgeClient();

// Convert Forge nodes to graph-viewer format
export function forgeNodeToGraphNode(node: ForgeNode): import('./graph-data').GraphNode {
  const typeMap: Record<string, import('./graph-data').GraphNode['type']> = {
    'Tool': 'object',
    'MissionLoad': 'task',
    'OssecRule': 'attribute',
    'Nonagon': 'event',
    'DataSource': 'actor',
    'Result': 'event',
  };

  return {
    id: node.id,
    name: node.label,
    category: node.node_type,
    type: typeMap[node.node_type] || 'attribute',
    priority: node.teth_entropy && node.teth_entropy >= 3.0 ? 'high' :
              node.teth_entropy && node.teth_entropy >= 2.5 ? 'medium' : 'low',
    state: 'normal',
    description: node.properties['description'] || `${node.node_type} node`,
    eeiCount: 0,
    relationships: [],
    metadata: {
      attckTechniques: [],
      kaliTools: [],
    },
  };
}

// HD4 phase to color mapping
export function getHD4Color(phase: string): string {
  const colors: Record<string, string> = {
    'Hunt': '#22d3ee',      // cyan
    'Detect': '#10b981',    // green
    'Disrupt': '#eab308',   // yellow
    'Disable': '#f97316',   // orange
    'Dominate': '#ef4444',  // red
  };
  return colors[phase] || '#8b949e';
}

// TETH entropy visualization
export function getTethEntropyColor(entropy: number): string {
  if (entropy >= 3.5) return '#22c55e';  // Excellent - green
  if (entropy >= 2.5) return '#eab308';  // Valid - yellow
  if (entropy >= 1.5) return '#f97316';  // Warning - orange
  return '#ef4444';                       // Invalid - red
}

export default forgeClient;
