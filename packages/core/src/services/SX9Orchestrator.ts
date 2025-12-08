import { ScriptExecutionCoordinator, CoordinationPlan } from './ScriptExecutionCoordinator';
import { LegionExecutionEngine } from './LegionExecutionEngine';
import { DatabaseMuxConnector } from './DatabaseMuxConnector';
import { HashingEngineConnector } from './HashingEngineConnector';

export interface SX9SystemStatus {
  overall: 'initializing' | 'ready' | 'connecting' | 'connected' | 'error';
  components: {
    legionEngine: 'offline' | 'initializing' | 'ready' | 'error';
    databaseMux: 'offline' | 'connecting' | 'connected' | 'partial' | 'error';
    scriptCoordinator: 'offline' | 'ready' | 'executing' | 'error';
    hashing: 'offline' | 'connected' | 'error';
  };
  connectivity: {
    surrealdb: boolean;
    supabase: boolean;
    sled: boolean;
    legionSlotGraph: boolean;
    hashingEngine: boolean;
  };
  statistics: {
    totalTasks: number;
    activeTasks: number;
    completedTasks: number;
    failedTasks: number;
    activeConnections: number;
    runningScripts: number;
  };
}

export interface ConnectionTarget {
  id: string;
  name: string;
  type: 'frontend' | 'backend' | 'database' | 'api';
  status: 'disconnected' | 'connecting' | 'connected' | 'error';
  dependencies: string[];
  component?: string;
  endpoint?: string;
}

export class SX9Orchestrator {
  private scriptCoordinator: ScriptExecutionCoordinator;
  private legionEngine: LegionExecutionEngine;
  private dbMux: DatabaseMuxConnector;
  private hashingEngine: HashingEngineConnector;
  private initialized = false;
  private connectionTargets: Map<string, ConnectionTarget> = new Map();

  constructor() {
    this.scriptCoordinator = new ScriptExecutionCoordinator();
    this.legionEngine = new LegionExecutionEngine();
    this.dbMux = new DatabaseMuxConnector();
    this.hashingEngine = new HashingEngineConnector();
    this.initializeConnectionTargets();
  }

  private initializeConnectionTargets(): void {
    const targets: ConnectionTarget[] = [
      // Database Connections
      {
        id: 'surrealdb',
        name: 'SurrealDB Connection',
        type: 'database',
        status: 'disconnected',
        dependencies: [],
        endpoint: 'http://localhost:11451'
      },
      {
        id: 'supabase',
        name: 'Supabase Connection',
        type: 'database',
        status: 'disconnected',
        dependencies: [],
        endpoint: 'https://lgdatoqcajaqhtbyzfef.supabase.co'
      },
      {
        id: 'sled',
        name: 'Sled KV Store',
        type: 'database',
        status: 'disconnected',
        dependencies: []
      },
      {
        id: 'legion-slot-graph',
        name: 'Legion Slot Graph',
        type: 'database',
        status: 'disconnected',
        dependencies: []
      },

      // API Connections
      {
        id: 'hashing-engine',
        name: 'Containerized Hashing Engine',
        type: 'api',
        status: 'disconnected',
        dependencies: [],
        endpoint: 'http://localhost:18105'
      },

      // Frontend Component Connections
      {
        id: 'dashboard-stats',
        name: 'Dashboard Statistics',
        type: 'frontend',
        status: 'disconnected',
        dependencies: ['legion-slot-graph', 'database-mux'],
        component: 'Dashboard.tsx'
      },
      {
        id: 'hd4-task-views',
        name: 'HD4 Task Views',
        type: 'frontend',
        status: 'disconnected',
        dependencies: ['legion-slot-graph', 'database-mux'],
        component: 'HD4TaskView.tsx'
      },
      {
        id: 'view-switching',
        name: 'GIS/Grid/Graph View Switching',
        type: 'frontend',
        status: 'disconnected',
        dependencies: ['hd4-task-views'],
        component: 'HD4PhaseContent.tsx'
      },
      {
        id: 'database-console',
        name: 'Live Database Console',
        type: 'frontend',
        status: 'disconnected',
        dependencies: ['database-mux'],
        component: 'LiveDatabaseConsole.tsx'
      },
      {
        id: 'legion-visualization',
        name: 'Legion Cytoscape Visualization',
        type: 'frontend',
        status: 'disconnected',
        dependencies: ['legion-slot-graph'],
        component: 'LegionSlotGraphCytoscape.tsx'
      },

      // Backend Services
      {
        id: 'database-mux',
        name: 'Database Mux Connector',
        type: 'backend',
        status: 'disconnected',
        dependencies: ['surrealdb', 'supabase', 'sled'],
        component: 'DatabaseMuxConnector.ts'
      },
      {
        id: 'legion-engine',
        name: 'Legion Execution Engine',
        type: 'backend',
        status: 'disconnected',
        dependencies: ['database-mux', 'legion-slot-graph'],
        component: 'LegionExecutionEngine.ts'
      },
      {
        id: 'script-coordinator',
        name: 'Script Execution Coordinator',
        type: 'backend',
        status: 'disconnected',
        dependencies: ['legion-engine', 'database-mux'],
        component: 'ScriptExecutionCoordinator.ts'
      }
    ];

    targets.forEach(target => {
      this.connectionTargets.set(target.id, target);
    });

    console.log(`🎯 Initialized ${targets.length} connection targets`);
  }

  async initialize(): Promise<boolean> {
    if (this.initialized) {
      console.log('⚠️ SX9 Orchestrator already initialized');
      return true;
    }

    console.log('🚀 Initializing SX9 Orchestrator...');

    try {
      // Initialize script coordinator (which initializes everything else)
      await this.scriptCoordinator.initialize();

      // Initialize hashing engine
      await this.hashingEngine.initialize();

      this.initialized = true;
      console.log('✅ SX9 Orchestrator initialized successfully');

      // Update connection statuses
      await this.updateConnectionStatuses();

      return true;
    } catch (error) {
      console.error('❌ Failed to initialize SX9 Orchestrator:', error);
      return false;
    }
  }

  async startSystemConnection(): Promise<boolean> {
    if (!this.initialized) {
      console.error('❌ Orchestrator not initialized. Call initialize() first.');
      return false;
    }

    console.log('🔗 Starting system-wide connection process...');

    try {
      // Execute the main HD4 connection plan
      const success = await this.scriptCoordinator.executeCoordinationPlan('hd4-connection-plan');

      if (success) {
        console.log('✅ HD4 connection plan completed successfully');

        // Execute Legion coordination plan
        const legionSuccess = await this.scriptCoordinator.executeCoordinationPlan('legion-coordination-plan');

        if (legionSuccess) {
          console.log('✅ Legion coordination plan completed successfully');
        } else {
          console.warn('⚠️ Legion coordination plan had issues');
        }
      }

      await this.updateConnectionStatuses();
      return success;

    } catch (error) {
      console.error('❌ System connection failed:', error);
      return false;
    }
  }

  private async updateConnectionStatuses(): Promise<void> {
    try {
      // Check database connections
      const dbConnections = await this.dbMux.checkAllConnections();

      Object.entries(dbConnections).forEach(([dbName, status]) => {
        const target = this.connectionTargets.get(dbName.replace('_', '-'));
        if (target) {
          target.status = status === 'connected' ? 'connected' : 'error';
        }
      });

      // Check hashing engine
      const hashingTarget = this.connectionTargets.get('hashing-engine');
      if (hashingTarget) {
        hashingTarget.status = this.hashingEngine.isOnline() ? 'connected' : 'error';
      }

      // Update backend service statuses
      const systemStatus = await this.scriptCoordinator.getSystemStatus();

      const dbMuxTarget = this.connectionTargets.get('database-mux');
      if (dbMuxTarget) {
        dbMuxTarget.status = systemStatus.databaseMux === 'connected' ? 'connected' : 'error';
      }

      const legionTarget = this.connectionTargets.get('legion-engine');
      if (legionTarget) {
        legionTarget.status = systemStatus.legionEngine === 'ready' ? 'connected' : 'error';
      }

      const scriptTarget = this.connectionTargets.get('script-coordinator');
      if (scriptTarget) {
        scriptTarget.status = 'connected';
      }

      // Update frontend component statuses based on dependencies
      this.updateFrontendStatuses();

    } catch (error) {
      console.error('Failed to update connection statuses:', error);
    }
  }

  private updateFrontendStatuses(): void {
    const frontendTargets = Array.from(this.connectionTargets.values())
      .filter(target => target.type === 'frontend');

    frontendTargets.forEach(target => {
      const dependenciesMet = target.dependencies.every(depId => {
        const dependency = this.connectionTargets.get(depId);
        return dependency?.status === 'connected';
      });

      target.status = dependenciesMet ? 'connected' : 'disconnected';
    });
  }

  async getSystemStatus(): Promise<SX9SystemStatus> {
    const scriptStatus = await this.scriptCoordinator.getSystemStatus();
    const dbConnections = await this.dbMux.checkAllConnections();

    // Calculate overall status
    const connectedTargets = Array.from(this.connectionTargets.values())
      .filter(target => target.status === 'connected').length;
    const totalTargets = this.connectionTargets.size;

    let overall: SX9SystemStatus['overall'] = 'ready';
    if (!this.initialized) {
      overall = 'initializing';
    } else if (connectedTargets === totalTargets) {
      overall = 'connected';
    } else if (connectedTargets > 0) {
      overall = 'connecting';
    }

    return {
      overall,
      components: {
        legionEngine: scriptStatus.legionEngine === 'ready' ? 'ready' : 'error',
        databaseMux: scriptStatus.databaseMux === 'connected' ? 'connected' : 'partial',
        scriptCoordinator: scriptStatus.runningExecutions > 0 ? 'executing' : 'ready',
        hashing: this.connectionTargets.get('hashing-engine')?.status === 'connected' ? 'connected' : 'offline'
      },
      connectivity: {
        surrealdb: dbConnections.surrealdb === 'connected',
        supabase: dbConnections.supabase === 'connected',
        sled: dbConnections.sled === 'connected',
        legionSlotGraph: dbConnections.legion_slot_graph === 'connected',
        hashingEngine: this.connectionTargets.get('hashing-engine')?.status === 'connected'
      },
      statistics: {
        totalTasks: 165, // Legion total tasks
        activeTasks: scriptStatus.runningExecutions,
        completedTasks: connectedTargets,
        failedTasks: Array.from(this.connectionTargets.values())
          .filter(target => target.status === 'error').length,
        activeConnections: Object.values(dbConnections)
          .filter(status => status === 'connected').length,
        runningScripts: scriptStatus.runningExecutions
      }
    };
  }

  getConnectionTargets(): ConnectionTarget[] {
    return Array.from(this.connectionTargets.values());
  }

  getConnectionPlan(): CoordinationPlan[] {
    return this.scriptCoordinator.getAllCoordinationPlans();
  }

  async testConnection(targetId: string): Promise<boolean> {
    const target = this.connectionTargets.get(targetId);
    if (!target) {
      console.error(`❌ Unknown connection target: ${targetId}`);
      return false;
    }

    console.log(`🧪 Testing connection: ${target.name}`);
    target.status = 'connecting';

    try {
      let success = false;

      switch (target.type) {
        case 'database':
          if (target.endpoint) {
            const response = await fetch(target.endpoint);
            success = response.ok;
          } else {
            success = true; // For local databases like Sled
          }
          break;

        case 'api':
          if (target.endpoint) {
            const response = await fetch(`${target.endpoint}/health`);
            success = response.ok;
          }
          break;

        case 'backend':
        case 'frontend':
          // These are tested through their dependencies
          success = target.dependencies.every(depId => {
            const dep = this.connectionTargets.get(depId);
            return dep?.status === 'connected';
          });
          break;
      }

      target.status = success ? 'connected' : 'error';
      console.log(`${success ? '✅' : '❌'} Connection test: ${target.name}`);

      return success;

    } catch (error) {
      target.status = 'error';
      console.error(`❌ Connection test failed: ${target.name}`, error);
      return false;
    }
  }

  // Hashing Engine Access Methods
  getHashingEngine(): HashingEngineConnector {
    return this.hashingEngine;
  }

  async hashThreatIntelligence(indicators: string[]): Promise<Record<string, string>> {
    return await this.hashingEngine.hashForThreatIntelligence(indicators);
  }

  async hashDocuments(documents: Array<{id: string, content: string}>): Promise<Array<{id: string, hash: string, compressed: boolean}>> {
    return await this.hashingEngine.hashForDocumentManager(documents);
  }

  async hashLegionTasks(tasks: Array<{id: string, script: string, world: string}>): Promise<Array<{taskId: string, scriptHash: string, worldHash: string}>> {
    return await this.hashingEngine.hashForLegionTasks(tasks);
  }

  async getHashingPerformance(): Promise<{
    requestsPerSecond: number;
    averageProcessingTime: number;
    compressionEfficiency: number;
    uptime: number;
  } | null> {
    return await this.hashingEngine.getPerformanceMetrics();
  }

  async shutdown(): Promise<void> {
    console.log('🛑 Shutting down SX9 Orchestrator...');

    // Shutdown hashing engine
    await this.hashingEngine.shutdown();

    // Reset all connection statuses
    this.connectionTargets.forEach(target => {
      target.status = 'disconnected';
    });

    this.initialized = false;
    console.log('✅ SX9 Orchestrator shutdown complete');
  }
}