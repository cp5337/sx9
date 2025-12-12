export interface WinmilMessage {
  id: string;
  timestamp: string;
  type: 'trace' | 'alert' | 'status' | 'command';
  data: unknown;
  source: string;
  target?: string;
}

export interface WinmilConfig {
  endpoint: string;
  reconnectInterval: number;
  maxRetries: number;
}

export class WinmilService {
  private ws: WebSocket | null = null;
  private config: WinmilConfig;
  private listeners: Map<string, (message: WinmilMessage) => void> = new Map();
  private reconnectTimer: NodeJS.Timeout | null = null;
  private retryCount = 0;

  constructor(config: WinmilConfig) {
    this.config = config;
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.config.endpoint);
        
        this.ws.onopen = () => {
          console.log('Connected to Winmil pub/sub system');
          this.retryCount = 0;
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const message: WinmilMessage = JSON.parse(event.data);
            this.handleMessage(message);
          } catch (error) {
            console.error('Failed to parse Winmil message:', error);
          }
        };

        this.ws.onerror = (error) => {
          console.error('Winmil WebSocket error:', error);
          reject(error);
        };

        this.ws.onclose = () => {
          console.log('Disconnected from Winmil pub/sub system');
          this.scheduleReconnect();
        };

      } catch (error) {
        reject(error);
      }
    });
  }

  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  private scheduleReconnect(): void {
    if (this.retryCount < this.config.maxRetries) {
      this.retryCount++;
      this.reconnectTimer = setTimeout(() => {
        console.log(`Attempting to reconnect to Winmil (attempt ${this.retryCount})`);
        this.connect().catch(console.error);
      }, this.config.reconnectInterval);
    }
  }

  subscribe(topic: string, callback: (message: WinmilMessage) => void): void {
    this.listeners.set(topic, callback);
    
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({
        action: 'subscribe',
        topic: topic
      }));
    }
  }

  unsubscribe(topic: string): void {
    this.listeners.delete(topic);
    
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({
        action: 'unsubscribe',
        topic: topic
      }));
    }
  }

  publish(topic: string, message: Omit<WinmilMessage, 'id' | 'timestamp'>): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      const fullMessage: WinmilMessage = {
        ...message,
        id: `msg-${Date.now()}-${Math.random()}`,
        timestamp: new Date().toISOString()
      };
      
      this.ws.send(JSON.stringify({
        action: 'publish',
        topic: topic,
        message: fullMessage
      }));
    }
  }

  private handleMessage(message: WinmilMessage): void {
    // Route message to appropriate listeners based on type
    const listener = this.listeners.get(message.type);
    if (listener) {
      listener(message);
    }
    
    // Also check for topic-based listeners
    const topicListener = this.listeners.get(`topic:${message.type}`);
    if (topicListener) {
      topicListener(message);
    }
  }

  isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  getConnectionStatus(): 'connected' | 'connecting' | 'disconnected' {
    if (!this.ws) return 'disconnected';
    
    switch (this.ws.readyState) {
      case WebSocket.CONNECTING:
        return 'connecting';
      case WebSocket.OPEN:
        return 'connected';
      default:
        return 'disconnected';
    }
  }
}

// Default Winmil service instance
export const winmilService = new WinmilService({
  endpoint: 'ws://localhost:8080/winmil',
  reconnectInterval: 5000,
  maxRetries: 5
});

