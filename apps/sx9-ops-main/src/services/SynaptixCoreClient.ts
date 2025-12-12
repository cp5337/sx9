/**
 * Synaptix Core Neural Mux Client
 * Connects main-ops UI to existing neural mux gRPC service (port 50051)
 * Handles CTAS AL expressions, alert streaming, and GNN asset prediction
 * Note: Uses gRPC-Web bridge (port 15001) for browser compatibility
 */

import { SYSTEM_PORTS } from '@/hooks';

interface NeuralMuxResponse {
  success: boolean;
  data: any;
  error?: string;
  muxChannel?: string;
  timestamp: number;
}

// gRPC Service Interfaces (matching the neural mux proto definitions)
interface CTASExpression {
  expression: string;
}

interface ExpressionResult {
  success: boolean;
  output: string;
  error_message?: string;
}

interface AlertRequest {
  alert_hash?: {
    hash: string;
  };
}

interface AlertDetails {
  alert_id: string;
  title: string;
  severity: number;
  timestamp: number;
  context: Record<string, any>;
}

interface AuthRequest {
  // Authentication parameters for alert streaming
  token?: string;
}

// Main-ops specific interfaces
interface ToolExecution {
  tool: 'exploitdb' | 'caldera' | 'atomic-red-team' | 'kali-tools';
  operation: string;
  parameters: Record<string, any>;
  execution_mode: 'ctas_tool_execution' | 'ops_execution';
}

interface AssetPrediction {
  asset_id: string;
  predicted_action: string;
  confidence: string;
  risk_assessment: number;
  cost_impact: number;
}

export class SynaptixCoreClient {
  private grpcBridgeUrl: string;
  private alertStream: EventSource | null = null;
  private alertCallbacks: Map<string, (data: any) => void> = new Map();
  private isConnected: boolean = false;

  constructor() {
    // gRPC-Web bridge or REST API bridge to the neural mux gRPC service
    this.grpcBridgeUrl = `http://localhost:${SYSTEM_PORTS.MAIN_OPS.BRIDGE_SERVICE}`; // CTAS standardized bridge service (15001)
    this.initializeConnection();
  }

  private async initializeConnection() {
    try {
      // Test connection to neural mux via bridge
      const healthCheck = await fetch(`${this.grpcBridgeUrl}/health`);
      if (healthCheck.ok) {
        this.isConnected = true;
        console.log('✅ Connected to Synaptix Core Neural Mux (gRPC Bridge)');
        this.initializeAlertStream();
      } else {
        console.log('⚠️ Neural Mux bridge not available, using fallback mode');
        this.isConnected = false;
      }
    } catch (error) {
      console.log('⚠️ Neural Mux not available, using demo mode:', error);
      this.isConnected = false;
    }
  }

  private initializeAlertStream() {
    try {
      // Initialize Server-Sent Events for alert streaming
      this.alertStream = new EventSource(`${this.grpcBridgeUrl}/alerts/stream`);

      this.alertStream.onopen = () => {
        console.log('📡 Alert stream connected');
      };

      this.alertStream.onmessage = (event) => {
        const alertData = JSON.parse(event.data);
        this.handleAlertMessage(alertData);
      };

      this.alertStream.onerror = (error) => {
        console.error('❌ Alert stream error:', error);
        // Reconnect after 5 seconds
        setTimeout(() => this.initializeAlertStream(), 5000);
      };
    } catch (error) {
      console.error('Failed to initialize alert stream:', error);
    }
  }

  private handleAlertMessage(alertData: any) {
    // Handle incoming alerts from the neural mux
    console.log('🚨 Received alert:', alertData);

    // Broadcast to UI components
    window.dispatchEvent(new CustomEvent('neural-mux-alert', { detail: alertData }));

    // Handle specific alert callbacks
    const callback = this.alertCallbacks.get('alerts');
    if (callback) {
      callback(alertData);
    }
  }

  /**
   * Execute CTAS AL Expression via Neural Mux
   */
  async executeCTASExpression(expression: string): Promise<ExpressionResult> {
    try {
      if (!this.isConnected) {
        return {
          success: false,
          output: '',
          error_message: 'Neural Mux not connected - running in demo mode'
        };
      }

      const response = await fetch(`${this.grpcBridgeUrl}/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Source': 'ctas7-main-ops'
        },
        body: JSON.stringify({
          expression: expression
        } as CTASExpression)
      });

      const result: ExpressionResult = await response.json();
      return result;
    } catch (error) {
      return {
        success: false,
        output: '',
        error_message: `CTAS AL execution failed: ${error}`
      };
    }
  }

  /**
   * CTAS Tool Execution (formerly HFT system) - Via CTAS AL expressions
   */
  async executeCTASTool(execution: ToolExecution): Promise<ExpressionResult> {
    // Convert tool execution to CTAS AL expression
    const alExpression = `exec-tool-${execution.tool}-${execution.operation}(${JSON.stringify(execution.parameters)})`;

    console.log(`🔧 Executing CTAS Tool: ${execution.tool} -> ${alExpression}`);
    return this.executeCTASExpression(alExpression);
  }

  /**
   * OPS Execution (formerly Trading system) - Via CTAS AL expressions
   */
  async executeOpsPlaybook(playbookType: string, parameters: Record<string, any>): Promise<ExpressionResult> {
    // Convert playbook execution to CTAS AL expression
    const alExpression = `exec-playbook-${playbookType}(${JSON.stringify(parameters)})`;

    console.log(`⚡ Executing OPS Playbook: ${playbookType} -> ${alExpression}`);
    return this.executeCTASExpression(alExpression);
  }

  /**
   * Tool Performance Monitoring (formerly Financial dashboard) - Via GNN predictions
   */
  async getToolPerformanceMetrics(): Promise<ExpressionResult> {
    // Use GNN to predict all asset performance
    console.log('📊 Getting tool performance via GNN prediction');
    return this.executeCTASExpression('gnn-predict-all');
  }

  /**
   * GNN Asset Prediction (Neural Mux Feature)
   */
  async predictAssetScaling(assetId: string): Promise<ExpressionResult> {
    console.log(`🧠 GNN Asset Prediction for: ${assetId}`);
    return this.executeCTASExpression(`gnn-predict-${assetId}`);
  }

  async updateAssetMetrics(assetId: string): Promise<ExpressionResult> {
    console.log(`📈 Updating asset metrics for: ${assetId}`);
    return this.executeCTASExpression(`gnn-update-${assetId}`);
  }

  async trainGNNModel(): Promise<ExpressionResult> {
    console.log('🎯 Training GNN model');
    return this.executeCTASExpression('gnn-train');
  }

  /**
   * Alert Streaming for Pub/Sub UI (Streaming Page)
   */
  subscribeToAlerts(callback: (data: any) => void) {
    this.alertCallbacks.set('alerts', callback);
    console.log('📡 Subscribed to neural mux alert stream');
  }

  unsubscribeFromAlerts() {
    this.alertCallbacks.delete('alerts');
    console.log('📡 Unsubscribed from alert stream');
  }

  /**
   * CTAS Tasks Integration (via CTAS AL expressions)
   */
  async getCTASTasks(category?: string): Promise<ExpressionResult> {
    const categoryFilter = category ? `-filter-${category}` : '';
    const alExpression = `query-tasks${categoryFilter}`;

    console.log(`📋 Getting CTAS tasks: ${alExpression}`);
    return this.executeCTASExpression(alExpression);
  }

  async updateCTASTask(taskId: string, updates: any): Promise<ExpressionResult> {
    const alExpression = `update-task-${taskId}(${JSON.stringify(updates)})`;

    console.log(`✏️ Updating CTAS task: ${taskId}`);
    return this.executeCTASExpression(alExpression);
  }

  /**
   * Ground Station Data (Enhanced Geolocation via CTAS AL)
   */
  async getGroundStations(): Promise<ExpressionResult> {
    console.log('🌍 Getting ground stations data');
    return this.executeCTASExpression('query-ground-stations-active');
  }

  async getVKaliDeployments(): Promise<ExpressionResult> {
    console.log('🐉 Getting vKali deployments');
    return this.executeCTASExpression('query-vkali-deployments-active');
  }

  /**
   * Security Tools Integration (Basic boot-up connections)
   */
  async initializeSecurityTools(): Promise<{exploitdb: boolean, caldera: boolean, atomicRedTeam: boolean}> {
    const results = {
      exploitdb: false,
      caldera: false,
      atomicRedTeam: false
    };

    try {
      console.log('🔧 Initializing security tools...');

      // ExploitDB connection test
      const exploitdbResult = await this.executeCTASTool({
        tool: 'exploitdb',
        operation: 'health_check',
        parameters: {},
        execution_mode: 'ctas_tool_execution'
      });
      results.exploitdb = exploitdbResult.success;

      // Caldera connection test
      const calderaResult = await this.executeCTASTool({
        tool: 'caldera',
        operation: 'health_check',
        parameters: {},
        execution_mode: 'ops_execution'
      });
      results.caldera = calderaResult.success;

      // Atomic Red Team connection test
      const atomicResult = await this.executeCTASTool({
        tool: 'atomic-red-team',
        operation: 'health_check',
        parameters: {},
        execution_mode: 'ctas_tool_execution'
      });
      results.atomicRedTeam = atomicResult.success;

      console.log('🔧 Security tools status:', results);
    } catch (error) {
      console.error('Security tools initialization failed:', error);
    }

    return results;
  }

  /**
   * Get Alert Details (Neural Mux Feature)
   */
  async getAlertDetails(alertHash: string): Promise<AlertDetails | null> {
    try {
      if (!this.isConnected) {
        return null;
      }

      const response = await fetch(`${this.grpcBridgeUrl}/alerts/details`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          alert_hash: { hash: alertHash }
        } as AlertRequest)
      });

      const result: AlertDetails = await response.json();
      return result;
    } catch (error) {
      console.error('Failed to get alert details:', error);
      return null;
    }
  }

  /**
   * Connection Status
   */
  isConnectedToNeuralMux(): boolean {
    return this.isConnected;
  }

  getConnectionStatus(): 'connected' | 'connecting' | 'disconnected' | 'error' {
    return this.isConnected ? 'connected' : 'disconnected';
  }

  /**
   * Cleanup
   */
  disconnect() {
    if (this.alertStream) {
      this.alertStream.close();
      this.alertStream = null;
    }
    this.alertCallbacks.clear();
    this.isConnected = false;
    console.log('🔌 Disconnected from Synaptix Core Neural Mux');
  }
}

// Singleton instance for main-ops
export const synaptixCore = new SynaptixCoreClient();

// Export types for use in components
export type {
  NeuralMuxResponse,
  CTASExpression,
  ExpressionResult,
  AlertDetails,
  ToolExecution,
  AssetPrediction
};