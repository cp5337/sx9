/**
 * System Context Provider for LLM Integration
 * 
 * Assembles full system context for Gemini, Claude, and other LLMs
 * so they can see the entire CTAS/SX9 system architecture, services,
 * and capabilities when responding in the MultiCLI chat window.
 * 
 * This is the goal: LLMs see the system in context.
 */

export interface SystemContext {
  architecture: ArchitectureContext;
  services: ServiceContext[];
  agents: AgentContext[];
  databases: DatabaseContext[];
  integrations: IntegrationContext[];
  capabilities: CapabilityContext[];
  currentState: CurrentStateContext;
  hd4Phase?: string;
}

export interface ArchitectureContext {
  threePlane: {
    iac: { provider: string; status: string; modules: string[] };
    containers: { platform: string; status: string; containers: string[] };
    bareMetal: { status: string; services: string[] };
  };
  ecs: {
    apecs: { purpose: string; status: string };
    legion: { purpose: string; status: string };
    atlas: { purpose: string; status: string };
  };
  bridge: {
    apecsLegion: { status: string; natsSubjects: string[] };
  };
}

export interface ServiceContext {
  name: string;
  type: string;
  port?: number;
  status: string;
  description: string;
  endpoints?: string[];
}

export interface AgentContext {
  id: string;
  name: string;
  llm: string;
  specialty: string;
  voiceId?: string;
  port?: number;
}

export interface DatabaseContext {
  name: string;
  type: string;
  status: string;
  purpose: string;
  connectionString?: string;
}

export interface IntegrationContext {
  name: string;
  type: string;
  status: string;
  description: string;
  apiEndpoint?: string;
}

export interface CapabilityContext {
  name: string;
  category: string;
  description: string;
  status: string;
}

export interface CurrentStateContext {
  activeSessions: number;
  recentOperations: string[];
  systemHealth: string;
  activeHD4Phase?: string;
}

/**
 * Assemble full system context for LLM
 */
export async function assembleSystemContext(hd4Phase?: string): Promise<SystemContext> {
  return {
    architecture: await getArchitectureContext(),
    services: await getServicesContext(),
    agents: getAgentsContext(),
    databases: await getDatabasesContext(),
    integrations: await getIntegrationsContext(),
    capabilities: await getCapabilitiesContext(),
    currentState: await getCurrentStateContext(),
    hd4Phase,
  };
}

/**
 * Get architecture context
 */
async function getArchitectureContext(): Promise<ArchitectureContext> {
  return {
    threePlane: {
      iac: {
        provider: 'GCP (Firefly)',
        status: 'active',
        modules: [
          'Firefly IAC Simulator',
          'WASM Microkernels (RustScan, Scorpion, dig, fping, Honeypot, Tarpit)',
          'Terraform API',
          'Cost-optimized deployments',
        ],
      },
      containers: {
        platform: 'OrbStack',
        status: 'active',
        containers: [
          'OSINT Collector',
          'Neo4j',
          'GLAF',
          'Kali Alpine (Ring Bus L2)',
          'MCP Servers',
        ],
      },
      bareMetal: {
        status: 'active',
        services: [
          'APECS → Legion Bridge',
          'ATLAS Daemon',
          'Plasma Defender',
          'SlotGraph Engine',
          'NATS JetStream',
          'Sled KVS',
        ],
      },
    },
    ecs: {
      apecs: {
        purpose: 'Layer 1: Async I/O, WASM-compatible, I/O-bound tasks',
        status: 'active',
      },
      legion: {
        purpose: 'Layer 2: High-performance, deterministic batch processing, hot-path operations',
        status: 'active',
      },
      atlas: {
        purpose: 'Layer 3: Cognitive operations, 1ms OODA loop, crystal resonance gating',
        status: 'active',
      },
    },
    bridge: {
      apecsLegion: {
        status: 'active',
        natsSubjects: [
          'sx9.threat.honeypot',
          'sx9.hotpath.load',
        ],
      },
    },
  };
}

/**
 * Get services context
 */
async function getServicesContext(): Promise<ServiceContext[]> {
  return [
    {
      name: 'CTAS API Gateway',
      type: 'API',
      port: 15180,
      status: 'active',
      description: 'Main API gateway for agent communication',
      endpoints: ['/agent/:agent/chat', '/api/health'],
    },
    {
      name: 'Voice Bridge',
      type: 'Voice',
      port: 19015,
      status: 'active',
      description: 'ElevenLabs TTS/STT integration',
    },
    {
      name: 'Firefly IAC Simulator',
      type: 'IAC',
      status: 'active',
      description: 'IAC deployment simulator for dev/testing',
    },
    {
      name: 'MCP Server',
      type: 'MCP',
      port: 18600,
      status: 'active',
      description: 'Model Context Protocol server',
    },
    {
      name: 'Kali Terminal',
      type: 'Terminal',
      port: 18080,
      status: 'active',
      description: 'Kali ISO terminal execution',
    },
    {
      name: 'Atomic Clipboard',
      type: 'Memory',
      port: 15001,
      status: 'active',
      description: 'Cross-agent memory and command context',
    },
  ];
}

/**
 * Get agents context
 */
function getAgentsContext(): AgentContext[] {
  return [
    {
      id: 'natasha',
      name: 'Natasha',
      llm: 'GPT-4',
      specialty: 'Cyber Intel, Threat Analysis, Security Operations',
      voiceId: 'EXAVITQu4vr4xnSDxMaL',
      port: 18152,
    },
    {
      id: 'marcus',
      name: 'Marcus',
      llm: 'Gemini 2M',
      specialty: 'Operations, IAC, Deployments, Firefly',
      voiceId: '21m00Tcm4TlvDq8ikWAM',
      port: 18151,
    },
    {
      id: 'elena',
      name: 'Elena',
      llm: 'Grok',
      specialty: 'Intelligence, OSINT, Data Analysis',
      voiceId: 'AZnzlk1XvdvUeBnXmlld',
      port: 18153,
    },
    {
      id: 'cove',
      name: 'Cove',
      llm: 'Claude',
      specialty: 'Repository, Code Management, Deployments',
      voiceId: 'pqHfZKP75CvOlQylNhV4',
      port: 18151,
    },
    {
      id: 'kali',
      name: 'Kali ISO',
      llm: 'Terminal',
      specialty: 'Direct Command Execution, Security Tools',
    },
  ];
}

/**
 * Get databases context
 */
async function getDatabasesContext(): Promise<DatabaseContext[]> {
  return [
    {
      name: 'Supabase',
      type: 'PostgreSQL',
      status: 'active',
      purpose: 'Primary storage for processed data, hashes, metadata',
    },
    {
      name: 'Neo4j',
      type: 'Graph',
      status: 'active',
      purpose: 'Graph relationships, task graph, nodes, GLAF',
    },
    {
      name: 'SurrealDB',
      type: 'Multi-Model',
      status: 'active',
      purpose: 'Terms, ontology, full data, SPIRES output',
    },
    {
      name: 'Sled KVS',
      type: 'Key-Value',
      status: 'active',
      purpose: 'Local hash → Unicode lookups, hot-path',
    },
    {
      name: 'ChromaDB',
      type: 'Vector',
      status: 'active',
      purpose: 'Vector search, threat similarity, embeddings',
    },
  ];
}

/**
 * Get integrations context
 */
async function getIntegrationsContext(): Promise<IntegrationContext[]> {
  return [
    {
      name: 'CODEX',
      type: 'Gemini Code Assist',
      status: 'active',
      description: 'Gemini Code Assist with custom commands, 2M context window',
      apiEndpoint: '/.gemini/custom-commands.json',
    },
    {
      name: 'Gemini CLI',
      type: 'CLI',
      status: 'active',
      description: 'Command-line interface to Gemini models',
    },
    {
      name: 'MCP Servers',
      type: 'MCP',
      status: 'active',
      description: 'Model Context Protocol servers for agent coordination',
    },
    {
      name: 'Firefly',
      type: 'IAC',
      status: 'active',
      description: 'Ultra-scalable serverless architecture, Tier 6 Burst Orchestration',
    },
    {
      name: 'Ring Bus',
      type: 'L2',
      status: 'active',
      description: 'Circular interconnect for deterministic latency (RFC-9301)',
    },
    {
      name: 'NATS JetStream',
      type: 'Messaging',
      status: 'active',
      description: 'Hermetic, L2-triggered security tool execution (RFC-9130)',
    },
  ];
}

/**
 * Get capabilities context
 */
async function getCapabilitiesContext(): Promise<CapabilityContext[]> {
  return [
    {
      name: 'Threat Intelligence Ingestion',
      category: 'Intelligence',
      description: 'Download, process, and store MITRE, Atomic Red Team, Nuclei, Sigma, etc.',
      status: 'active',
    },
    {
      name: 'SPIRES Ontology Extraction',
      category: 'Intelligence',
      description: 'Zero-shot semantic extraction, SurrealQL and LinkML generation',
      status: 'active',
    },
    {
      name: 'Dual-Trivariate Hashing',
      category: 'Core',
      description: 'RFC-9001: SCH | CUID | UUID (48-character Base96)',
      status: 'active',
    },
    {
      name: 'Unicode Operational Routing',
      category: 'Core',
      description: 'RFC-9002: Unicode-based routing and execution',
      status: 'active',
    },
    {
      name: 'HD4 Phases',
      category: 'Operations',
      description: 'Hunt, Detect, Disrupt, Disable, Dominate',
      status: 'active',
    },
    {
      name: 'Plasma Defender',
      category: 'Security',
      description: 'Custom HIDS with microsecond reactions, cross-detection rules',
      status: 'active',
    },
    {
      name: 'WASM Microkernels',
      category: 'Execution',
      description: 'RustScan, Scorpion, dig, fping, Honeypot, Tarpit',
      status: 'active',
    },
    {
      name: 'Digital Twins',
      category: 'Deception',
      description: 'Virtual representations for deception and tarpit honeypots',
      status: 'active',
    },
    {
      name: 'Voice/Chat-Driven Operations',
      category: 'Interface',
      description: 'All operations can be driven via voice or chat',
      status: 'active',
    },
  ];
}

/**
 * Get current state context
 */
async function getCurrentStateContext(): Promise<CurrentStateContext> {
  try {
    // Try to get real state from services
    const [clipboardRes] = await Promise.allSettled([
      fetch('http://localhost:15001/api/atomic-clipboard'),
    ]);

    const activeSessions = clipboardRes.status === 'fulfilled' ? 1 : 0;

    return {
      activeSessions,
      recentOperations: [
        'MultiCLI chat active',
        'System context provider initialized',
      ],
      systemHealth: 'healthy',
    };
  } catch {
    return {
      activeSessions: 0,
      recentOperations: [],
      systemHealth: 'unknown',
    };
  }
}

/**
 * Format system context for LLM prompt
 */
export function formatSystemContextForPrompt(context: SystemContext): string {
  const lines: string[] = [];

  lines.push('# CTAS/SX9 System Context');
  lines.push('');
  lines.push('You are operating within the CTAS/SX9 threat intelligence and defense system.');
  lines.push('You have full visibility into the system architecture, services, and capabilities.');
  lines.push('');

  // Architecture
  lines.push('## Architecture');
  lines.push('');
  lines.push('### Three-Plane Operations:');
  lines.push(`- **IAC (GCP)**: ${context.architecture.threePlane.iac.provider} - ${context.architecture.threePlane.iac.status}`);
  lines.push(`  - Modules: ${context.architecture.threePlane.iac.modules.join(', ')}`);
  lines.push(`- **Containers (OrbStack)**: ${context.architecture.threePlane.containers.platform} - ${context.architecture.threePlane.containers.status}`);
  lines.push(`  - Containers: ${context.architecture.threePlane.containers.containers.join(', ')}`);
  lines.push(`- **Bare Metal**: ${context.architecture.bareMetal.status}`);
  lines.push(`  - Services: ${context.architecture.bareMetal.services.join(', ')}`);
  lines.push('');
  lines.push('### ECS Architecture:');
  lines.push(`- **apecs (Layer 1)**: ${context.architecture.ecs.apecs.purpose}`);
  lines.push(`- **legion (Layer 2)**: ${context.architecture.ecs.legion.purpose}`);
  lines.push(`- **atlas (Layer 3)**: ${context.architecture.ecs.atlas.purpose}`);
  lines.push('');

  // Services
  lines.push('## Services');
  for (const service of context.services) {
    lines.push(`- **${service.name}** (${service.type}): ${service.description}`);
    if (service.port) {
      lines.push(`  - Port: ${service.port}`);
    }
  }
  lines.push('');

  // Agents
  lines.push('## Agents');
  for (const agent of context.agents) {
    lines.push(`- **${agent.name}** (${agent.llm}): ${agent.specialty}`);
  }
  lines.push('');

  // Databases
  lines.push('## Databases');
  for (const db of context.databases) {
    lines.push(`- **${db.name}** (${db.type}): ${db.purpose}`);
  }
  lines.push('');

  // Integrations
  lines.push('## Integrations');
  for (const integration of context.integrations) {
    lines.push(`- **${integration.name}** (${integration.type}): ${integration.description}`);
  }
  lines.push('');

  // Capabilities
  lines.push('## Capabilities');
  for (const capability of context.capabilities) {
    lines.push(`- **${capability.name}** (${capability.category}): ${capability.description}`);
  }
  lines.push('');

  // Current State
  lines.push('## Current State');
  lines.push(`- Active Sessions: ${context.currentState.activeSessions}`);
  lines.push(`- System Health: ${context.currentState.systemHealth}`);
  if (context.hd4Phase) {
    lines.push(`- HD4 Phase: ${context.hd4Phase.toUpperCase()}`);
  }
  lines.push('');

  lines.push('---');
  lines.push('');
  lines.push('**You can:**');
  lines.push('- Deploy IAC modules via Firefly');
  lines.push('- Execute microkernels (RustScan, Scorpion, etc.)');
  lines.push('- Query databases (Supabase, Neo4j, SurrealDB)');
  lines.push('- Coordinate with other agents');
  lines.push('- Access threat intelligence data');
  lines.push('- Execute operations across all three planes (IAC, Containers, Bare Metal)');
  lines.push('');
  lines.push('**Use this context to provide accurate, system-aware responses.**');

  return lines.join('\n');
}



