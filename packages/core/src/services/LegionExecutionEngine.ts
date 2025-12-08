import { DatabaseMuxConnector } from './DatabaseMuxConnector';
import type { Database } from '../../database/supabase_schema';

// MCP Event interfaces for schema updates
interface McpEvent {
  event_type: string;
  timestamp: number;
  payload: Record<string, any>;
  source_node: string;
}

interface SchemaUpdateEvent extends McpEvent {
  event_type: 'schema_updated' | 'schema_nochange';
  payload: {
    status: 'updated' | 'no_change';
    schema_path: string;
    new_hash?: string;
    old_hash?: string;
    table_count?: number;
  };
}

export interface LegionTask {
  id: string;
  name: string;
  world: 'Cyber' | 'Geographical' | 'Space' | 'Maritime';
  form: '1n' | '2n'; // Adversary vs Counter-adversary
  hd4Phase: 'Hunt' | 'Detect' | 'Disrupt' | 'Disable' | 'Dominate';
  crates: string[];
  databases: string[];
  nodes: string[];
  executionScript: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

export interface ExecutionContext {
  taskId: string;
  world: string;
  form: '1n' | '2n';
  hd4Phase: string;
  availableCrates: Record<string, any>;
  databaseConnections: Record<string, any>;
  nodeCapabilities: Record<string, any>;
}

export class LegionExecutionEngine {
  private dbMux: DatabaseMuxConnector;
  private runningTasks: Map<string, LegionTask> = new Map();
  private schemaUpdateListeners: Array<(event: SchemaUpdateEvent) => void> = [];
  private currentSchemaHash: string | null = null;
  private lastSchemaUpdate: number = 0;

  constructor() {
    this.dbMux = new DatabaseMuxConnector();
    this.initializeSchemaWatcher();
  }

  // Initialize MCP schema update watcher
  private initializeSchemaWatcher(): void {
    console.log('🔍 [LEGION] Initializing schema update watcher...');

    // Simulate MCP event subscription
    this.subscribeToMcpEvents();
  }

  // Subscribe to MCP events (simulated)
  private subscribeToMcpEvents(): void {
    // In production, this would connect to the actual MCP event bus
    console.log('📡 [LEGION] Subscribed to MCP schema update events');

    // Simulate periodic schema check
    setInterval(() => {
      this.checkForSchemaUpdates();
    }, 30000); // Check every 30 seconds
  }

  // Handle incoming schema update events
  public handleSchemaUpdateEvent(event: SchemaUpdateEvent): void {
    console.log(`🔄 [LEGION] Received schema update event: ${event.event_type}`);

    if (event.event_type === 'schema_updated') {
      this.onSchemaUpdated(event);
    } else if (event.event_type === 'schema_nochange') {
      this.onSchemaNoChange(event);
    }

    // Notify all listeners
    this.notifySchemaUpdateListeners(event);
  }

  // Handle schema updated events
  private async onSchemaUpdated(event: SchemaUpdateEvent): Promise<void> {
    console.log('📝 [LEGION] Schema update detected - reloading database types...');

    const oldHash = this.currentSchemaHash;
    this.currentSchemaHash = event.payload.new_hash || null;
    this.lastSchemaUpdate = event.timestamp;

    try {
      // Reload database connections with new schema
      await this.reloadDatabaseConnections();

      // Update task execution context with new types
      await this.updateTaskExecutionContext();

      // Emit telemetry about the schema update
      this.emitSchemaUpdateTelemetry(event, 'success');

      console.log(`✅ [LEGION] Schema reload complete`);
      console.log(`   Old hash: ${oldHash?.substring(0, 8) || 'none'}`);
      console.log(`   New hash: ${this.currentSchemaHash?.substring(0, 8) || 'none'}`);

    } catch (error) {
      console.error('❌ [LEGION] Schema reload failed:', error);
      this.emitSchemaUpdateTelemetry(event, 'failed', error);
    }
  }

  // Handle schema no-change events
  private onSchemaNoChange(event: SchemaUpdateEvent): void {
    console.log('✅ [LEGION] Schema verified - no changes detected');
    this.emitSchemaUpdateTelemetry(event, 'verified');
  }

  // Reload database connections with new schema types
  private async reloadDatabaseConnections(): Promise<void> {
    console.log('🔄 [LEGION] Reloading database connections...');

    // Reinitialize database multiplexer
    await this.dbMux.initialize();

    // Reload available crates with new types
    await this.loadAvailableCrates();

    // Update node capabilities matrix
    await this.initializeNodeCapabilities();

    console.log('✅ [LEGION] Database connections reloaded');
  }

  // Update task execution context with new schema
  private async updateTaskExecutionContext(): Promise<void> {
    console.log('🔄 [LEGION] Updating task execution contexts...');

    // Update running tasks with new database types
    for (const [taskId, task] of this.runningTasks) {
      if (task.status === 'running') {
        console.log(`   Updating context for running task: ${taskId}`);
        // In production, this would update the task's execution context
        // with the new database schema types
      }
    }

    console.log('✅ [LEGION] Task execution contexts updated');
  }

  // Emit telemetry about schema update operations
  private emitSchemaUpdateTelemetry(
    event: SchemaUpdateEvent,
    status: 'success' | 'failed' | 'verified',
    error?: any
  ): void {
    const telemetryEvent = {
      event_type: 'legion_schema_telemetry',
      timestamp: Math.floor(Date.now() / 1000),
      payload: {
        original_event: event.event_type,
        schema_hash: this.currentSchemaHash,
        update_status: status,
        running_tasks: this.runningTasks.size,
        last_update: this.lastSchemaUpdate,
        error_details: error ? error.message : null,
        affected_systems: ['LegionExecutionEngine', 'DatabaseMuxConnector', 'SlotGraphQueryEngine']
      },
      source_node: 'legion_execution_engine'
    };

    console.log('📊 [LEGION] Emitting schema update telemetry:', telemetryEvent.payload.update_status);

    // In production, this would be sent to the telemetry bridge
    this.forwardToTelemetryBridge(telemetryEvent);
  }

  // Forward events to telemetry bridge (simulated)
  private forwardToTelemetryBridge(event: any): void {
    // Simulate sending to telemetry bridge
    console.log('📡 [LEGION] → Telemetry Bridge: ' + event.event_type);
  }

  // Add schema update listener
  public addSchemaUpdateListener(listener: (event: SchemaUpdateEvent) => void): void {
    this.schemaUpdateListeners.push(listener);
  }

  // Remove schema update listener
  public removeSchemaUpdateListener(listener: (event: SchemaUpdateEvent) => void): void {
    const index = this.schemaUpdateListeners.indexOf(listener);
    if (index > -1) {
      this.schemaUpdateListeners.splice(index, 1);
    }
  }

  // Notify all schema update listeners
  private notifySchemaUpdateListeners(event: SchemaUpdateEvent): void {
    this.schemaUpdateListeners.forEach(listener => {
      try {
        listener(event);
      } catch (error) {
        console.error('❌ [LEGION] Schema update listener error:', error);
      }
    });
  }

  // Check for schema updates (simulated periodic check)
  private async checkForSchemaUpdates(): Promise<void> {
    // In production, this would check with the schema sync node
    // For now, we'll just log that we're checking
    console.log('🔍 [LEGION] Checking for schema updates...');
  }

  // Get current schema status
  public getSchemaStatus(): {
    current_hash: string | null;
    last_update: number;
    listeners_count: number;
    is_watching: boolean;
  } {
    return {
      current_hash: this.currentSchemaHash,
      last_update: this.lastSchemaUpdate,
      listeners_count: this.schemaUpdateListeners.length,
      is_watching: true
    };
  }

  async initializeEngine(): Promise<void> {
    console.log('🏛️ Initializing Legion Execution Engine...');

    // Initialize database connections
    await this.dbMux.initialize();

    // Load available crates from Legion Slot Graph
    await this.loadAvailableCrates();

    // Initialize node capabilities matrix
    await this.initializeNodeCapabilities();

    console.log('✅ Legion Execution Engine initialized');
  }

  async loadAvailableCrates(): Promise<void> {
    try {
      const crateQuery = `
        SELECT * FROM comprehensive_crate_interviews
        WHERE type = 'capability'
        ORDER BY world, form, hd4_phase
      `;

      const results = await this.dbMux.query('surrealdb', crateQuery);
      console.log(`📦 Loaded ${results.length} crate capabilities`);
    } catch (error) {
      console.error('Failed to load crates:', error);
    }
  }

  async initializeNodeCapabilities(): Promise<void> {
    // Load node interview data for 1n/2n form mapping
    try {
      const nodeQuery = `
        SELECT * FROM node_interviews
        WHERE interview_type IN ['1n_form', '2n_form']
        ORDER BY world, adversary_task_id
      `;

      const results = await this.dbMux.query('legion_slot_graph', nodeQuery);
      console.log(`🎯 Loaded ${results.length} node capabilities`);
    } catch (error) {
      console.error('Failed to load node capabilities:', error);
    }
  }

  async executeTask(task: LegionTask): Promise<boolean> {
    console.log(`🚀 Executing Legion Task: ${task.name} (${task.world}/${task.form})`);

    this.runningTasks.set(task.id, { ...task, status: 'running' });

    try {
      // Build execution context
      const context = await this.buildExecutionContext(task);

      // Execute coordination script
      const result = await this.executeCoordinationScript(task, context);

      // Update task status
      this.runningTasks.set(task.id, {
        ...task,
        status: result ? 'completed' : 'failed'
      });

      return result;
    } catch (error) {
      console.error(`❌ Task execution failed: ${task.id}`, error);
      this.runningTasks.set(task.id, { ...task, status: 'failed' });
      return false;
    }
  }

  private async buildExecutionContext(task: LegionTask): Promise<ExecutionContext> {
    // Get crates for this world/phase
    const availableCrates = await this.getCratesForTask(task);

    // Get database connections needed
    const databaseConnections = await this.getDatabaseConnectionsForTask(task);

    // Get node capabilities
    const nodeCapabilities = await this.getNodeCapabilitiesForTask(task);

    return {
      taskId: task.id,
      world: task.world,
      form: task.form,
      hd4Phase: task.hd4Phase,
      availableCrates,
      databaseConnections,
      nodeCapabilities
    };
  }

  private async getCratesForTask(task: LegionTask): Promise<Record<string, any>> {
    const query = `
      SELECT * FROM comprehensive_crate_interviews
      WHERE world = '${task.world}'
      AND hd4_phase = '${task.hd4Phase}'
      AND form = '${task.form}'
    `;

    const results = await this.dbMux.query('surrealdb', query);
    return results.reduce((acc: any, crate: any) => {
      acc[crate.crate_name] = crate;
      return acc;
    }, {});
  }

  private async getDatabaseConnectionsForTask(task: LegionTask): Promise<Record<string, any>> {
    const connections: Record<string, any> = {};

    for (const dbName of task.databases) {
      try {
        connections[dbName] = await this.dbMux.getConnection(dbName as any);
      } catch (error) {
        console.warn(`Failed to get connection for ${dbName}:`, error);
      }
    }

    return connections;
  }

  private async getNodeCapabilitiesForTask(task: LegionTask): Promise<Record<string, any>> {
    const query = `
      SELECT * FROM node_interviews
      WHERE world = '${task.world}'
      AND form = '${task.form}'
      AND hd4_phase = '${task.hd4Phase}'
    `;

    const results = await this.dbMux.query('legion_slot_graph', query);
    return results.reduce((acc: any, node: any) => {
      acc[node.node_id] = node;
      return acc;
    }, {});
  }

  private async executeCoordinationScript(task: LegionTask, context: ExecutionContext): Promise<boolean> {
    console.log(`📋 Executing coordination script for task: ${task.name}`);

    // This would execute the actual coordination logic
    // For now, simulate execution with validation
    const validationResults = {
      cratesAvailable: Object.keys(context.availableCrates).length > 0,
      databasesConnected: Object.keys(context.databaseConnections).length > 0,
      nodesReady: Object.keys(context.nodeCapabilities).length > 0
    };

    console.log('🔍 Validation Results:', validationResults);

    // Log execution context
    console.log(`🌍 World: ${context.world}`);
    console.log(`⚔️ Form: ${context.form}`);
    console.log(`🎯 HD4 Phase: ${context.hd4Phase}`);
    console.log(`📦 Available Crates: ${Object.keys(context.availableCrates).length}`);
    console.log(`💾 Database Connections: ${Object.keys(context.databaseConnections).length}`);
    console.log(`🎯 Node Capabilities: ${Object.keys(context.nodeCapabilities).length}`);

    // Return success if all validations pass
    return Object.values(validationResults).every(Boolean);
  }

  async getTaskStatus(taskId: string): Promise<LegionTask | null> {
    return this.runningTasks.get(taskId) || null;
  }

  async getAllRunningTasks(): Promise<LegionTask[]> {
    return Array.from(this.runningTasks.values()).filter(task => task.status === 'running');
  }

  async getTasksByWorld(world: string): Promise<LegionTask[]> {
    return Array.from(this.runningTasks.values()).filter(task => task.world === world);
  }

  async getTasksByHD4Phase(phase: string): Promise<LegionTask[]> {
    return Array.from(this.runningTasks.values()).filter(task => task.hd4Phase === phase);
  }

  // Coordination between 1n and 2n forms
  async coordinateAdversaryCounterOps(adversaryTaskId: string, counterTaskId: string): Promise<boolean> {
    const adversaryTask = this.runningTasks.get(adversaryTaskId);
    const counterTask = this.runningTasks.get(counterTaskId);

    if (!adversaryTask || !counterTask) {
      console.error('❌ Cannot coordinate: missing tasks');
      return false;
    }

    if (adversaryTask.form !== '1n' || counterTask.form !== '2n') {
      console.error('❌ Invalid form coordination: expected 1n/2n pair');
      return false;
    }

    console.log(`🔄 Coordinating ${adversaryTask.name} (1n) vs ${counterTask.name} (2n)`);

    // Execute coordination logic
    return true;
  }
}