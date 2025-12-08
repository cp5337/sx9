import { LegionExecutionEngine, LegionTask } from './LegionExecutionEngine';
import { DatabaseMuxConnector } from './DatabaseMuxConnector';

export interface ScriptExecution {
  id: string;
  name: string;
  type: 'node' | 'crate' | 'database' | 'coordination';
  script: string;
  dependencies: string[];
  world: 'Cyber' | 'Geographical' | 'Space' | 'Maritime';
  hd4Phase: 'Hunt' | 'Detect' | 'Disrupt' | 'Disable' | 'Dominate';
  status: 'pending' | 'running' | 'completed' | 'failed';
  output?: string;
  error?: string;
  startTime?: Date;
  endTime?: Date;
}

export interface CoordinationPlan {
  id: string;
  name: string;
  description: string;
  executions: ScriptExecution[];
  legionTasks: LegionTask[];
  totalSteps: number;
  completedSteps: number;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

export class ScriptExecutionCoordinator {
  private legionEngine: LegionExecutionEngine;
  private dbMux: DatabaseMuxConnector;
  private runningExecutions: Map<string, ScriptExecution> = new Map();
  private coordinationPlans: Map<string, CoordinationPlan> = new Map();

  constructor() {
    this.legionEngine = new LegionExecutionEngine();
    this.dbMux = new DatabaseMuxConnector();
  }

  async initialize(): Promise<void> {
    console.log('🎯 Initializing Script Execution Coordinator...');

    await this.legionEngine.initializeEngine();
    await this.dbMux.initialize();

    // Load predefined coordination plans
    await this.loadCoordinationPlans();

    console.log('✅ Script Execution Coordinator initialized');
  }

  private async loadCoordinationPlans(): Promise<void> {
    // HD4 Phase Connection Plan
    const hd4ConnectionPlan: CoordinationPlan = {
      id: 'hd4-connection-plan',
      name: 'HD4 Frontend-Backend Connection',
      description: 'Connect HD4 pages to Legion Slot Graph and Database systems',
      executions: [
        {
          id: 'connect-dashboard-stats',
          name: 'Connect Dashboard Statistics',
          type: 'coordination',
          script: 'connectDashboardToRealData',
          dependencies: [],
          world: 'Cyber',
          hd4Phase: 'Hunt',
          status: 'pending'
        },
        {
          id: 'connect-hd4-tasks',
          name: 'Connect HD4 Task Views',
          type: 'coordination',
          script: 'connectHD4TasksToLegion',
          dependencies: ['connect-dashboard-stats'],
          world: 'Cyber',
          hd4Phase: 'Hunt',
          status: 'pending'
        },
        {
          id: 'connect-view-switching',
          name: 'Connect View Switching (GIS/Grid/Graph)',
          type: 'coordination',
          script: 'connectViewSwitching',
          dependencies: ['connect-hd4-tasks'],
          world: 'Cyber',
          hd4Phase: 'Hunt',
          status: 'pending'
        },
        {
          id: 'connect-database-mux',
          name: 'Connect Database Mux Throughout App',
          type: 'database',
          script: 'connectDatabaseMuxGlobally',
          dependencies: [],
          world: 'Cyber',
          hd4Phase: 'Hunt',
          status: 'pending'
        }
      ],
      legionTasks: [],
      totalSteps: 4,
      completedSteps: 0,
      status: 'pending'
    };

    this.coordinationPlans.set(hd4ConnectionPlan.id, hd4ConnectionPlan);

    // Legion Task Coordination Plan
    const legionCoordinationPlan: CoordinationPlan = {
      id: 'legion-coordination-plan',
      name: 'Legion 1n/2n Task Coordination',
      description: 'Coordinate adversary (1n) vs counter-adversary (2n) tasks across 4 worlds',
      executions: [
        {
          id: 'map-adversary-tasks',
          name: 'Map Adversary Tasks (1n Form)',
          type: 'node',
          script: 'mapAdversaryTasks',
          dependencies: [],
          world: 'Cyber',
          hd4Phase: 'Hunt',
          status: 'pending'
        },
        {
          id: 'map-counter-tasks',
          name: 'Map Counter-Adversary Tasks (2n Form)',
          type: 'node',
          script: 'mapCounterAdversaryTasks',
          dependencies: ['map-adversary-tasks'],
          world: 'Cyber',
          hd4Phase: 'Detect',
          status: 'pending'
        },
        {
          id: 'coordinate-task-pairs',
          name: 'Coordinate 1n/2n Task Pairs',
          type: 'coordination',
          script: 'coordinateTaskPairs',
          dependencies: ['map-adversary-tasks', 'map-counter-tasks'],
          world: 'Cyber',
          hd4Phase: 'Disrupt',
          status: 'pending'
        }
      ],
      legionTasks: [],
      totalSteps: 3,
      completedSteps: 0,
      status: 'pending'
    };

    this.coordinationPlans.set(legionCoordinationPlan.id, legionCoordinationPlan);

    console.log(`📋 Loaded ${this.coordinationPlans.size} coordination plans`);
  }

  async executeCoordinationPlan(planId: string): Promise<boolean> {
    const plan = this.coordinationPlans.get(planId);
    if (!plan) {
      console.error(`❌ Coordination plan not found: ${planId}`);
      return false;
    }

    console.log(`🚀 Executing coordination plan: ${plan.name}`);
    plan.status = 'running';

    try {
      // Execute scripts in dependency order
      const executionOrder = this.calculateExecutionOrder(plan.executions);

      for (const execution of executionOrder) {
        const success = await this.executeScript(execution);
        if (!success) {
          plan.status = 'failed';
          return false;
        }
        plan.completedSteps++;
      }

      plan.status = 'completed';
      console.log(`✅ Coordination plan completed: ${plan.name}`);
      return true;

    } catch (error) {
      console.error(`❌ Coordination plan failed: ${plan.name}`, error);
      plan.status = 'failed';
      return false;
    }
  }

  private calculateExecutionOrder(executions: ScriptExecution[]): ScriptExecution[] {
    const executed = new Set<string>();
    const ordered: ScriptExecution[] = [];

    while (ordered.length < executions.length) {
      for (const execution of executions) {
        if (executed.has(execution.id)) continue;

        // Check if all dependencies are executed
        const dependenciesMet = execution.dependencies.every(dep => executed.has(dep));

        if (dependenciesMet) {
          ordered.push(execution);
          executed.add(execution.id);
        }
      }
    }

    return ordered;
  }

  async executeScript(execution: ScriptExecution): Promise<boolean> {
    console.log(`📝 Executing script: ${execution.name} (${execution.type})`);

    execution.status = 'running';
    execution.startTime = new Date();
    this.runningExecutions.set(execution.id, execution);

    try {
      let result = false;

      switch (execution.script) {
        case 'connectDashboardToRealData':
          result = await this.connectDashboardToRealData();
          break;
        case 'connectHD4TasksToLegion':
          result = await this.connectHD4TasksToLegion();
          break;
        case 'connectViewSwitching':
          result = await this.connectViewSwitching();
          break;
        case 'connectDatabaseMuxGlobally':
          result = await this.connectDatabaseMuxGlobally();
          break;
        case 'mapAdversaryTasks':
          result = await this.mapAdversaryTasks();
          break;
        case 'mapCounterAdversaryTasks':
          result = await this.mapCounterAdversaryTasks();
          break;
        case 'coordinateTaskPairs':
          result = await this.coordinateTaskPairs();
          break;
        default:
          console.warn(`⚠️ Unknown script: ${execution.script}`);
          result = false;
      }

      execution.status = result ? 'completed' : 'failed';
      execution.endTime = new Date();

      if (result) {
        console.log(`✅ Script completed: ${execution.name}`);
      } else {
        console.error(`❌ Script failed: ${execution.name}`);
      }

      return result;

    } catch (error) {
      execution.status = 'failed';
      execution.error = error instanceof Error ? error.message : String(error);
      execution.endTime = new Date();
      console.error(`❌ Script execution error: ${execution.name}`, error);
      return false;
    }
  }

  // Script implementations
  private async connectDashboardToRealData(): Promise<boolean> {
    console.log('🔌 Connecting Dashboard to real data sources...');

    try {
      // Get real Legion task count
      const tasks = await this.legionEngine.getAllRunningTasks();
      console.log(`📊 Found ${tasks.length} active Legion tasks`);

      // Get database connection status
      const dbStatus = await this.dbMux.checkAllConnections();
      console.log('💾 Database status:', dbStatus);

      // This would update the Dashboard component to use real data
      return true;
    } catch (error) {
      console.error('Failed to connect Dashboard:', error);
      return false;
    }
  }

  private async connectHD4TasksToLegion(): Promise<boolean> {
    console.log('🎯 Connecting HD4 Task Views to Legion Slot Graph...');

    try {
      // Query Legion tasks by HD4 phase
      for (const phase of ['Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate']) {
        const tasks = await this.legionEngine.getTasksByHD4Phase(phase);
        console.log(`📋 ${phase}: ${tasks.length} tasks`);
      }

      return true;
    } catch (error) {
      console.error('Failed to connect HD4 tasks:', error);
      return false;
    }
  }

  private async connectViewSwitching(): Promise<boolean> {
    console.log('🔄 Connecting View Switching (GIS/Grid/Graph)...');

    // This would implement real view switching logic
    console.log('📍 GIS view: Connect to geographical data');
    console.log('📊 Grid view: Connect to tabular task data');
    console.log('🕸️ Graph view: Connect to Cytoscape visualization');

    return true;
  }

  private async connectDatabaseMuxGlobally(): Promise<boolean> {
    console.log('🔗 Connecting Database Mux throughout application...');

    const connections = await this.dbMux.checkAllConnections();
    const allConnected = Object.values(connections).every(status => status === 'connected');

    console.log('💾 Global database connectivity:', allConnected ? '✅' : '❌');
    return allConnected;
  }

  private async mapAdversaryTasks(): Promise<boolean> {
    console.log('⚔️ Mapping Adversary Tasks (1n Form)...');

    // This would map adversary tasks across all 4 worlds
    for (const world of ['Cyber', 'Geographical', 'Space', 'Maritime']) {
      const tasks = await this.legionEngine.getTasksByWorld(world);
      const adversaryTasks = tasks.filter(t => t.form === '1n');
      console.log(`🌍 ${world}: ${adversaryTasks.length} adversary tasks`);
    }

    return true;
  }

  private async mapCounterAdversaryTasks(): Promise<boolean> {
    console.log('🛡️ Mapping Counter-Adversary Tasks (2n Form)...');

    for (const world of ['Cyber', 'Geographical', 'Space', 'Maritime']) {
      const tasks = await this.legionEngine.getTasksByWorld(world);
      const counterTasks = tasks.filter(t => t.form === '2n');
      console.log(`🌍 ${world}: ${counterTasks.length} counter-adversary tasks`);
    }

    return true;
  }

  private async coordinateTaskPairs(): Promise<boolean> {
    console.log('🔄 Coordinating 1n/2n Task Pairs...');

    // This would implement the actual 1n/2n coordination logic
    return true;
  }

  // Status and monitoring methods
  getCoordinationPlan(planId: string): CoordinationPlan | undefined {
    return this.coordinationPlans.get(planId);
  }

  getAllCoordinationPlans(): CoordinationPlan[] {
    return Array.from(this.coordinationPlans.values());
  }

  getRunningExecutions(): ScriptExecution[] {
    return Array.from(this.runningExecutions.values()).filter(e => e.status === 'running');
  }

  async getSystemStatus(): Promise<{
    legionEngine: string;
    databaseMux: string;
    runningExecutions: number;
    activePlans: number;
  }> {
    const dbConnections = await this.dbMux.checkAllConnections();
    const allDbConnected = Object.values(dbConnections).every(status => status === 'connected');

    return {
      legionEngine: 'ready',
      databaseMux: allDbConnected ? 'connected' : 'partial',
      runningExecutions: this.getRunningExecutions().length,
      activePlans: Array.from(this.coordinationPlans.values()).filter(p => p.status === 'running').length
    };
  }
}