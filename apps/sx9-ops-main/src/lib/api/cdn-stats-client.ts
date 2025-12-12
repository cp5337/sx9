// CTAS v7.3 CDN Statistical Service Client
// Connects to ctas7-cdn-statistical on port 18108

export interface CDNStats {
  threats: {
    total: number;
    critical: number;
    high: number;
    medium: number;
    low: number;
  };
  tools: {
    executions: number;
    active: number;
  };
  legion: {
    entities: number;
  };
  hd4: {
    hunt: number;
    detect: number;
    disrupt: number;
    disable: number;
    dominate: number;
  };
  events: {
    rate: number;
  };
  usim: {
    total: number;
  };
  timestamp: string;
}

export interface CDNMetrics {
  threats_total: number;
  threats_critical: number;
  threats_high: number;
  tool_executions: number;
  legion_entities: number;
  hd4_hunt: number;
  hd4_detect: number;
  hd4_disrupt: number;
  hd4_disable: number;
  hd4_dominate: number;
  event_rate: number;
  usim_total: number;
}

export class CDNStatsClient {
  private baseUrl: string;
  private wsUrl: string;
  private ws: WebSocket | null = null;

  constructor(baseUrl: string = "http://localhost:18108") {
    this.baseUrl = baseUrl;
    this.wsUrl = baseUrl.replace("http", "ws");
  }

  // Get current stats snapshot
  async getStats(): Promise<CDNStats> {
    try {
      const response = await fetch(`${this.baseUrl}/api/stats/current`, {
        method: 'GET',
        signal: AbortSignal.timeout(5000)
      });
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error("Failed to fetch CDN stats:", error);
      throw error;
    }
  }

  // Get threat stats
  async getThreatStats(): Promise<CDNStats['threats']> {
    try {
      const response = await fetch(`${this.baseUrl}/api/stats/threats`, {
        method: 'GET',
        signal: AbortSignal.timeout(5000)
      });
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error("Failed to fetch threat stats:", error);
      throw error;
    }
  }

  // Get HD4 phase stats
  async getHD4Stats(): Promise<CDNStats['hd4']> {
    try {
      const response = await fetch(`${this.baseUrl}/api/stats/hd4`, {
        method: 'GET',
        signal: AbortSignal.timeout(5000)
      });
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error("Failed to fetch HD4 stats:", error);
      throw error;
    }
  }

  // Get Prometheus metrics
  async getMetrics(): Promise<string> {
    try {
      const response = await fetch(`${this.baseUrl}/metrics`, {
        method: 'GET',
        signal: AbortSignal.timeout(5000)
      });
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      
      return await response.text();
    } catch (error) {
      console.error("Failed to fetch metrics:", error);
      throw error;
    }
  }

  // Parse Prometheus metrics to object
  parseMetrics(metricsText: string): CDNMetrics {
    const metrics: Partial<CDNMetrics> = {};
    const lines = metricsText.split('\n');
    
    for (const line of lines) {
      if (line.startsWith('#') || !line.trim()) continue;
      
      const [name, value] = line.split(' ');
      if (name && value) {
        const key = name as keyof CDNMetrics;
        metrics[key] = parseFloat(value);
      }
    }
    
    return metrics as CDNMetrics;
  }

  // Connect to WebSocket stream
  connectStream(
    onStats: (stats: CDNStats) => void,
    onError?: (error: Error) => void
  ): void {
    try {
      this.ws = new WebSocket(`${this.wsUrl}/stream/stats`);
      
      this.ws.onopen = () => {
        console.log("✅ CDN stats stream connected");
      };
      
      this.ws.onmessage = (event) => {
        try {
          const stats = JSON.parse(event.data) as CDNStats;
          onStats(stats);
        } catch (err) {
          console.error("Failed to parse stats:", err);
        }
      };
      
      this.ws.onerror = (error) => {
        console.error("CDN stats stream error:", error);
        if (onError) {
          onError(new Error("WebSocket error"));
        }
      };
      
      this.ws.onclose = () => {
        console.log("CDN stats stream closed");
        // Auto-reconnect after 5 seconds
        setTimeout(() => {
          console.log("Reconnecting to CDN stats stream...");
          this.connectStream(onStats, onError);
        }, 5000);
      };
    } catch (err) {
      console.error("Failed to connect to CDN stats stream:", err);
      if (onError) {
        onError(err as Error);
      }
    }
  }

  // Disconnect from WebSocket
  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  // Check if CDN service is available
  async isAvailable(): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/health`, {
        method: 'GET',
        signal: AbortSignal.timeout(2000)
      });
      return response.ok;
    } catch {
      return false;
    }
  }
}

export const cdnStatsClient = new CDNStatsClient();

