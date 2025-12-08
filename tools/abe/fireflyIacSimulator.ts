/**
 * Firefly IAC Simulator
 * 
 * Simulates IAC deployments for dev/testing using Firefly serverless architecture.
 * Provides local simulation without actual GCP deployment.
 */

export interface IACSimulationConfig {
  mode: 'simulation' | 'production';
  projectId?: string;
  region?: string;
  environment: 'dev' | 'staging' | 'prod';
}

export interface IACSimulationResult {
  success: boolean;
  simulationId: string;
  resources: SimulatedResource[];
  estimatedCost: string;
  deploymentTime: string;
  logs: string[];
  endpoints?: Record<string, string>;
}

export interface SimulatedResource {
  type: string;
  name: string;
  status: 'creating' | 'running' | 'stopped' | 'error';
  simulatedEndpoint?: string;
  metadata?: Record<string, any>;
}

class FireflyIACSimulator {
  private simulations: Map<string, IACSimulationResult> = new Map();
  private resourceCounter = 0;

  /**
   * Simulate IAC deployment
   */
  async simulateDeployment(
    moduleId: string,
    config: IACSimulationConfig
  ): Promise<IACSimulationResult> {
    if (config.mode !== 'simulation') {
      throw new Error('This service is for simulation mode only');
    }

    const simulationId = `sim-${Date.now()}-${++this.resourceCounter}`;
    
    // Simulate deployment delay
    await this.delay(1000 + Math.random() * 2000);

    const resources = this.generateSimulatedResources(moduleId);
    const result: IACSimulationResult = {
      success: true,
      simulationId,
      resources,
      estimatedCost: this.calculateEstimatedCost(resources),
      deploymentTime: `${(1000 + Math.random() * 2000).toFixed(0)}ms`,
      logs: this.generateSimulationLogs(moduleId, resources),
      endpoints: this.generateSimulatedEndpoints(moduleId, resources),
    };

    this.simulations.set(simulationId, result);
    return result;
  }

  /**
   * Get simulation status
   */
  getSimulation(simulationId: string): IACSimulationResult | null {
    return this.simulations.get(simulationId) || null;
  }

  /**
   * List all simulations
   */
  listSimulations(): IACSimulationResult[] {
    return Array.from(this.simulations.values());
  }

  /**
   * Stop/cleanup simulation
   */
  async stopSimulation(simulationId: string): Promise<void> {
    const simulation = this.simulations.get(simulationId);
    if (simulation) {
      simulation.resources.forEach(resource => {
        resource.status = 'stopped';
      });
      // Keep in map for inspection, but mark as stopped
    }
  }

  /**
   * Generate simulated resources based on module
   */
  private generateSimulatedResources(moduleId: string): SimulatedResource[] {
    const resourceTemplates: Record<string, SimulatedResource[]> = {
      'abe-core': [
        {
          type: 'Cloud Run',
          name: 'abe-external-api-sim',
          status: 'running',
          metadata: { 
            region: 'us-central1', 
            minInstances: 0, // CHEAP: Scale to zero
            cpu: '0.25', // Minimal CPU
            memory: '128Mi', // Minimal memory
          },
        },
        {
          type: 'Cloud Run',
          name: 'abe-ingestion-sim',
          status: 'running',
          metadata: { 
            region: 'us-central1', 
            minInstances: 0, // CHEAP: Scale to zero
            cpu: '0.25',
            memory: '128Mi',
          },
        },
        {
          type: 'Cloud Storage',
          name: 'abe-documents-sim',
          status: 'running',
          metadata: { 
            bucket: 'abe-documents-sim',
            storageClass: 'STANDARD', // Cheapest storage class
            lifecycle: 'delete after 7 days', // Auto-cleanup
          },
        },
        {
          type: 'Pub/Sub',
          name: 'abe-events-sim',
          status: 'running',
          metadata: { 
            topic: 'abe-events-sim',
            // Free tier
          },
        },
      ],
      'firefly-functions': [
        {
          type: 'Cloud Function',
          name: 'firefly-document-analysis-sim',
          status: 'running',
          metadata: { 
            runtime: 'nodejs20', 
            memory: '128MB', // CHEAPEST memory tier
            timeout: '60s', // Minimal timeout
            maxInstances: 1, // Limit scaling (cheap)
          },
        },
        {
          type: 'Cloud Function',
          name: 'firefly-marc-validation-sim',
          status: 'running',
          metadata: { 
            runtime: 'nodejs20', 
            memory: '128MB', // CHEAPEST
            timeout: '30s',
            maxInstances: 1,
          },
        },
        {
          type: 'Cloud Function',
          name: 'firefly-eei-extraction-sim',
          status: 'running',
          metadata: { 
            runtime: 'nodejs20', 
            memory: '128MB', // CHEAPEST
            timeout: '60s',
            maxInstances: 1,
          },
        },
        {
          type: 'API Gateway',
          name: 'firefly-gateway-sim',
          status: 'running',
          metadata: { 
            apiId: 'firefly-sim-api',
            // Free tier API Gateway
          },
        },
      ],
      'gpu-instance': [
        {
          type: 'Compute Engine',
          name: 'gpu-instance-sim',
          status: 'running',
          metadata: {
            machineType: 'e2-micro', // CHEAPEST
            preemptible: true, // CHEAP SHIT MODE
            zone: 'us-central1-a',
            // No GPU in cheap mode - use CPU only
          },
        },
        {
          type: 'Persistent Disk',
          name: 'gpu-disk-sim',
          status: 'running',
          metadata: { size: '10GB', type: 'pd-standard' }, // Smallest, cheapest
        },
      ],
      'osint-system': [
        {
          type: 'Container',
          name: 'neo4j-sim',
          status: 'running',
          metadata: { image: 'neo4j:latest', port: 7474 },
        },
        {
          type: 'Container',
          name: 'glaf-sim',
          status: 'running',
          metadata: { image: 'glaf:latest', port: 18018 },
        },
        {
          type: 'Container',
          name: 'osint-collector-sim',
          status: 'running',
          metadata: { image: 'osint-collector:latest' },
        },
      ],
      'microkernels': [
        {
          type: 'Cloud Function',
          name: 'microkernel-rustscan-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '128MB', // CHEAPEST
            timeout: '30s',
            maxInstances: 10, // Can scale for parallel ops
            wasmModule: 'rustscan-microkernel.wasm',
          },
        },
        {
          type: 'Cloud Function',
          name: 'microkernel-scorpion-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '128MB',
            timeout: '10s', // Fast ICMP operations
            maxInstances: 100, // High concurrency for ping training
            wasmModule: 'scorpion-microkernel.wasm',
          },
        },
        {
          type: 'Cloud Function',
          name: 'microkernel-dig-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '128MB',
            timeout: '5s', // DNS lookup is fast
            maxInstances: 50,
            wasmModule: 'dig-microkernel.wasm',
          },
        },
        {
          type: 'Cloud Function',
          name: 'microkernel-fping-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '128MB',
            timeout: '5s',
            maxInstances: 50,
            wasmModule: 'fping-microkernel.wasm',
          },
        },
        {
          type: 'Cloud Function',
          name: 'microkernel-honeypot-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '256MB', // Slightly more for honeypot logic
            timeout: '60s',
            maxInstances: 20,
            wasmModule: 'honeypot-microkernel.wasm',
          },
        },
        {
          type: 'Cloud Function',
          name: 'microkernel-tarpit-sim',
          status: 'running',
          metadata: {
            runtime: 'wasm32-wasi',
            memory: '256MB',
            timeout: '300s', // Tarpit needs longer timeout
            maxInstances: 10,
            wasmModule: 'tarpit-microkernel.wasm',
          },
        },
      ],
    };

    return resourceTemplates[moduleId] || [
      {
        type: 'Unknown',
        name: `${moduleId}-sim`,
        status: 'error',
        metadata: { error: 'Unknown module' },
      },
    ];
  }

  /**
   * Generate simulated endpoints
   */
  private generateSimulatedEndpoints(
    moduleId: string,
    resources: SimulatedResource[]
  ): Record<string, string> {
    const endpoints: Record<string, string> = {};

    if (moduleId === 'abe-core') {
      endpoints['external-api'] = 'http://localhost:8080/sim/abe-external-api';
      endpoints['ingestion'] = 'http://localhost:8080/sim/abe-ingestion';
    } else if (moduleId === 'firefly-functions') {
      endpoints['document-analysis'] = 'http://localhost:8080/sim/firefly/document-analysis';
      endpoints['marc-validation'] = 'http://localhost:8080/sim/firefly/marc-validation';
      endpoints['eei-extraction'] = 'http://localhost:8080/sim/firefly/eei-extraction';
      endpoints['gateway'] = 'http://localhost:8080/sim/firefly/gateway';
    } else if (moduleId === 'gpu-instance') {
      endpoints['ssh'] = 'ssh://gpu-instance-sim@localhost:2222';
      endpoints['jupyter'] = 'http://localhost:8888/sim/gpu/jupyter';
    } else if (moduleId === 'osint-system') {
      endpoints['neo4j'] = 'http://localhost:7474';
      endpoints['glaf'] = 'http://localhost:18018';
      endpoints['osint-collector'] = 'http://localhost:8081';
    } else if (moduleId === 'microkernels') {
      endpoints['rustscan'] = 'http://localhost:8080/sim/microkernel/rustscan';
      endpoints['scorpion'] = 'http://localhost:8080/sim/microkernel/scorpion';
      endpoints['dig'] = 'http://localhost:8080/sim/microkernel/dig';
      endpoints['fping'] = 'http://localhost:8080/sim/microkernel/fping';
      endpoints['honeypot'] = 'http://localhost:8080/sim/microkernel/honeypot';
      endpoints['tarpit'] = 'http://localhost:8080/sim/microkernel/tarpit';
    }

    return endpoints;
  }

  /**
   * Calculate estimated cost for simulation
   * CHEAP SHIT MODE: Minimal cost, preemptible instances, smallest sizes
   */
  private calculateEstimatedCost(resources: SimulatedResource[]): string {
    // CHEAP SHIT MODE: Use smallest instances, preemptible, minimal resources
    let baseCost = 0;
    resources.forEach(resource => {
      if (resource.type === 'Cloud Run') {
        // Minimal instances, smallest CPU/memory
        baseCost += 0.01; // $0.01/hour (minimal config)
      } else if (resource.type === 'Cloud Function') {
        // WASM microkernels: 128MB memory, minimal CPU
        // Firefly handles microkernels as serverless functions
        if (resource.metadata?.wasmModule) {
          baseCost += 0.0005; // $0.0005 per invocation (microkernels are tiny)
        } else {
          baseCost += 0.001; // $0.001 per invocation (regular functions)
        }
      } else if (resource.type === 'Compute Engine') {
        // Preemptible e2-micro (cheapest)
        baseCost += 0.01; // $0.01/hour (preemptible e2-micro)
      } else if (resource.type === 'Container') {
        // Local containers (free)
        baseCost += 0;
      } else if (resource.type === 'Cloud Storage') {
        // Minimal storage class
        baseCost += 0.001; // $0.001/GB/month
      } else if (resource.type === 'Pub/Sub') {
        // Free tier
        baseCost += 0;
      }
    });
    return `$${baseCost.toFixed(3)}/hour (CHEAP MODE - simulated)`;
  }

  /**
   * Generate simulation logs
   */
  private generateSimulationLogs(
    moduleId: string,
    resources: SimulatedResource[]
  ): string[] {
    const logs: string[] = [
      `[SIM] Starting IAC simulation for module: ${moduleId}`,
      `[SIM] Mode: Development/Testing`,
      `[SIM] Generating ${resources.length} simulated resources...`,
    ];

    resources.forEach((resource, index) => {
      logs.push(`[SIM] [${index + 1}/${resources.length}] Creating ${resource.type}: ${resource.name}`);
      logs.push(`[SIM] [${index + 1}/${resources.length}] Status: ${resource.status}`);
    });

    logs.push(`[SIM] Simulation complete: ${resources.length} resources created`);
    logs.push(`[SIM] All resources are running in simulation mode`);
    logs.push(`[SIM] Endpoints available at localhost (see endpoints in result)`);

    return logs;
  }

  /**
   * Delay helper
   */
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Singleton instance
export const fireflyIacSimulator = new FireflyIACSimulator();

