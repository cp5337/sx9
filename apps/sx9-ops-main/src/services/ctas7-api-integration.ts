// SX9 Gateway Frontend API Integration
// Connects to SX9 Gateway via WebSocket (ws://localhost:18600/ws)

export const CTAS7_API_ENDPOINTS = {
  // Core Infrastructure
  atlas: 'http://localhost:18500',
  portManager: 'http://localhost:18103',
  
  // CDN Services
  cdnStatistical: 'http://localhost:18108',
  cdnMonitoring: 'http://localhost:18109',
  cdnGateway: 'http://localhost:18110',
  
  // MCP Servers
  mcpLinear: 'http://localhost:15182',
  mcpCesium: 'http://localhost:15183',
  mcpBackend: 'http://localhost:18600',
  
  // Backend Services
  axon: 'http://localhost:15176',
  legionEcs: 'http://localhost:15177',
  
  // Databases
  surrealdb: 'http://localhost:8000',
  supabase: 'http://localhost:5432',
  sledis: 'redis://localhost:6379',
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
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceConfig),
    });
    return response.ok;
  } catch (error) {
    console.error('Port Manager registration failed:', error);
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
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(issueData),
  });
  return response.json();
}
