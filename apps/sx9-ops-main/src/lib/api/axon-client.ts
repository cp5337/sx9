// AXON Threat Intelligence Client
// Connects to AXON service for real-time threat normalization

export interface AxonThreat {
  id: string;
  timestamp: string;
  level: "critical" | "high" | "medium" | "low";
  description: string;
  agentId: string;
  ruleId: string;
  mitreTactic?: string;
  mitreId?: string;
}

export class AxonClient {
  private baseUrl: string;
  private eventSource: EventSource | null = null;

  constructor(baseUrl: string = "http://localhost:18180") {
    this.baseUrl = baseUrl;
  }

  // Connect to AXON SSE stream
  connect(onThreat: (threat: AxonThreat) => void, onError?: (error: Error) => void): void {
    try {
      this.eventSource = new EventSource(`${this.baseUrl}/api/threats/stream`);
      
      this.eventSource.onmessage = (event) => {
        try {
          const threat = JSON.parse(event.data) as AxonThreat;
          onThreat(threat);
        } catch (err) {
          console.error("Failed to parse threat:", err);
        }
      };

      this.eventSource.onerror = (error) => {
        console.error("AXON connection error:", error);
        if (onError) {
          onError(new Error("AXON connection failed"));
        }
      };
    } catch (err) {
      console.error("Failed to connect to AXON:", err);
      if (onError) {
        onError(err as Error);
      }
    }
  }

  // Disconnect from AXON
  disconnect(): void {
    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }
  }

  // Check if AXON is available
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

export const axonClient = new AxonClient();
