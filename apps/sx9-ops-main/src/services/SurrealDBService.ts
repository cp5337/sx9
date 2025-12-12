/**
 * SurrealDB Direct Connection Service for Main Ops Platform
 * Connects directly to SurrealDB running on port 8000
 */

export interface SurrealDBConfig {
  url: string;
  namespace: string;
  database: string;
}

export interface SurrealDBQuery {
  query: string;
  vars?: Record<string, any>;
}

export interface SurrealDBResponse<T = any> {
  success: boolean;
  data?: T[];
  error?: string;
  time?: string;
}

export class SurrealDBService {
  private config: SurrealDBConfig;
  private token?: string;

  constructor(config: SurrealDBConfig) {
    this.config = config;
  }

  async connect(user?: string, pass?: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.config.url}/signin`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'NS': this.config.namespace,
          'DB': this.config.database
        },
        body: JSON.stringify({
          user: user || 'root',
          pass: pass || 'root'
        })
      });

      if (response.ok) {
        const data = await response.json();
        this.token = data.token;
        return true;
      }
      return false;
    } catch (error) {
      console.error('SurrealDB connection failed:', error);
      return false;
    }
  }

  async query<T = any>(query: string, vars?: Record<string, any>): Promise<SurrealDBResponse<T>> {
    try {
      const response = await fetch(`${this.config.url}/sql`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'NS': this.config.namespace,
          'DB': this.config.database,
          ...(this.token && { 'Authorization': `Bearer ${this.token}` })
        },
        body: JSON.stringify({
          query,
          vars
        })
      });

      if (response.ok) {
        const result = await response.json();
        return {
          success: true,
          data: result[0]?.result || [],
          time: result[0]?.time
        };
      } else {
        const error = await response.text();
        return {
          success: false,
          error: error || 'Query failed'
        };
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  // Main Ops specific methods
  async getCTASTasks() {
    return this.query(`
      SELECT * FROM tasks WHERE type = 'ctas' ORDER BY created_at DESC;
    `);
  }

  async getThreatActors() {
    return this.query(`
      SELECT * FROM threat_actors ORDER BY last_seen DESC;
    `);
  }

  async getSystemMetrics() {
    return this.query(`
      SELECT * FROM system_metrics ORDER BY timestamp DESC LIMIT 100;
    `);
  }

  async getDeployedAssets() {
    return this.query(`
      SELECT * FROM deployed_assets WHERE status != 'deleted' ORDER BY deployed_at DESC;
    `);
  }

  async createCTASTask(task: any) {
    return this.query(`
      CREATE tasks CONTENT $task;
    `, { task: { ...task, type: 'ctas', created_at: new Date().toISOString() } });
  }

  async updateTaskStatus(taskId: string, status: string) {
    return this.query(`
      UPDATE $taskId SET status = $status, updated_at = $timestamp;
    `, { taskId, status, timestamp: new Date().toISOString() });
  }

  async getGroundStations() {
    return this.query(`
      SELECT * FROM ground_stations WHERE active = true ORDER BY last_contact DESC;
    `);
  }

  async getSatelliteData() {
    return this.query(`
      SELECT * FROM satellites WHERE operational = true ORDER BY last_update DESC;
    `);
  }

  async getSecurityAlerts() {
    return this.query(`
      SELECT * FROM security_alerts WHERE status = 'active' ORDER BY created_at DESC LIMIT 50;
    `);
  }

  async getThreatIntelligence() {
    return this.query(`
      SELECT * FROM threat_intelligence ORDER BY ingested_at DESC LIMIT 100;
    `);
  }
}

// Main Ops SurrealDB instance
export const mainOpsSurrealDB = new SurrealDBService({
  url: import.meta.env.VITE_SURREALDB_URL || 'http://localhost:8000',
  namespace: 'ctas',
  database: 'mainops'
});

// Initialize connection for Main Ops
export const initializeMainOpsDB = async () => {
  const connected = await mainOpsSurrealDB.connect();
  if (connected) {
    console.log('🟢 Main Ops connected to SurrealDB');
    return true;
  } else {
    console.log('🔴 Main Ops failed to connect to SurrealDB');
    return false;
  }
};