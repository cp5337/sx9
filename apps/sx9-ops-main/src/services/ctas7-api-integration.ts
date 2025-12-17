// SX9 Gateway Frontend API Integration
// Connects to SX9 Gateway via WebSocket (ws://localhost:18600/ws)

export const CTAS7_API_ENDPOINTS = {
  // Core Infrastructure
  atlas: "http://localhost:18500",
  portManager: "http://localhost:18103",

  // CDN Services
  cdnStatistical: "http://localhost:18108",
  cdnMonitoring: "http://localhost:18109",
  cdnGateway: "http://localhost:18110",

  // MCP Servers
  mcpLinear: "http://localhost:15182",
  mcpCesium: "http://localhost:15183",
  mcpBackend: "http://localhost:18120",

  // Backend Services
  axon: "http://localhost:15176",
  legionEcs: "http://localhost:15177",

  // Databases
  surrealdb: "http://localhost:8000",
  supabase: "http://localhost:18300",
  sledis: "redis://localhost:6379",
};

export const CTAS7_HEALTH_CHECKS = {
  atlas: `${CTAS7_API_ENDPOINTS.atlas}/health`,
  portManager: `${CTAS7_API_ENDPOINTS.portManager}/health`,
  cdnStatistical: `${CTAS7_API_ENDPOINTS.cdnStatistical}/health`,
  cdnMonitoring: `${CTAS7_API_ENDPOINTS.cdnMonitoring}/health`,
  cdnGateway: `${CTAS7_API_ENDPOINTS.cdnGateway}/health`,
  axon: `${CTAS7_API_ENDPOINTS.axon}/health`,
  legionEcs: `${CTAS7_API_ENDPOINTS.legionEcs}/health`,
};

// Health check utility
export async function checkServiceHealth(serviceName: keyof typeof CTAS7_HEALTH_CHECKS) {
  try {
    const response = await fetch(CTAS7_HEALTH_CHECKS[serviceName]);
    return response.ok;
  } catch (error) {
    console.error(`Health check failed for ${serviceName}:`, error);
    return false;
  }
}

// Port Manager integration
export async function registerWithPortManager(serviceConfig: {
  service_name: string;
  port: number;
  protocol: string;
  health_endpoint: string;
}) {
  try {
    const response = await fetch(`${CTAS7_API_ENDPOINTS.portManager}/api/ports/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(serviceConfig),
    });
    return response.ok;
  } catch (error) {
    console.error("Port Manager registration failed:", error);
    return false;
  }
}

// CDN Statistical API
export async function fetchThreatStats() {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.cdnStatistical}/api/stats/threats`);
  return response.json();
}

export async function fetchToolStats() {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.cdnStatistical}/api/stats/tools`);
  return response.json();
}

// CDN Monitoring API
export async function fetchAllServices() {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.cdnMonitoring}/api/services`);
  return response.json();
}

export async function fetchServiceDetails(serviceName: string) {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.cdnMonitoring}/api/services/${serviceName}`);
  return response.json();
}

// MCP Linear API
export async function createLinearIssue(issueData: {
  title: string;
  description: string;
  priority?: number;
}) {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.mcpLinear}/api/issues`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(issueData),
  });
  return response.json();
}

// GLAF Graph API Integration
// Aligned with React Flow / xyflow patterns

export interface Node {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: Record<string, any>;
  draggable?: boolean;
  selectable?: boolean;
  connectable?: boolean;
  hidden?: boolean;
  selected?: boolean;
  measured?: { width: number; height: number };
}

export interface Edge {
  id: string;
  source: string;
  target: string;
  type?: string;
  data?: Record<string, any>;
  selected?: boolean;
  animated?: boolean;
}

export type NodeChange =
  | { type: "position"; id: string; position?: { x: number; y: number }; dragging?: boolean }
  | {
      type: "dimensions";
      id: string;
      dimensions: { width: number; height: number };
      resizing?: boolean;
    }
  | { type: "select"; id: string; selected: boolean }
  | { type: "remove"; id: string }
  | { type: "add"; item: Node; index?: number }
  | { type: "replace"; id: string; item: Node };

export interface GraphResponse {
  nodes: Node[];
  edges: Edge[];
}

export async function fetchGraph(): Promise<GraphResponse> {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.mcpBackend}/mcp`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      session_id: "frontend_session",
      request_type: "GetGraph",
      namespace: "ctas_operational",
      data: {},
    }),
  });

  const result = await response.json();
  return result.data || { nodes: [], edges: [] };
}

export async function applyGraphChanges(changes: NodeChange[]) {
  const response = await fetch(`${CTAS7_API_ENDPOINTS.mcpBackend}/mcp`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      session_id: "frontend_session",
      request_type: "ApplyGraphChanges",
      namespace: "ctas_operational",
      data: changes,
    }),
  });

  return response.json();
}
