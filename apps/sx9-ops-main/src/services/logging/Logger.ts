export enum LogLevel {
  DEBUG = 0,
  INFO = 1,
  WARN = 2,
  ERROR = 3,
  FATAL = 4
}

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  context?: string;
  data?: unknown;
  error?: Error;
}

export interface LoggerConfig {
  level: LogLevel;
  enableConsole: boolean;
  enableRemote: boolean;
  remoteEndpoint?: string;
  maxEntries?: number;
}

export class Logger {
  private config: LoggerConfig;
  private entries: LogEntry[] = [];
  private context: string;

  constructor(context: string, config?: Partial<LoggerConfig>) {
    this.context = context;
    this.config = {
      level: LogLevel.INFO,
      enableConsole: true,
      enableRemote: false,
      maxEntries: 1000,
      ...config
    };
  }

  private shouldLog(level: LogLevel): boolean {
    return level >= this.config.level;
  }

  private formatMessage(level: LogLevel, message: string, data?: unknown, error?: Error): string {
    const timestamp = new Date().toISOString();
    const levelName = LogLevel[level];
    let formatted = `[${timestamp}] ${levelName} [${this.context}] ${message}`;
    
    if (data) {
      formatted += ` | Data: ${JSON.stringify(data)}`;
    }
    
    if (error) {
      formatted += ` | Error: ${error.message}`;
    }
    
    return formatted;
  }

  private addEntry(entry: LogEntry): void {
    this.entries.push(entry);
    
    // Keep only the last maxEntries
    if (this.entries.length > (this.config.maxEntries || 1000)) {
      this.entries = this.entries.slice(-(this.config.maxEntries || 1000));
    }
  }

  private logToConsole(entry: LogEntry): void {
    if (!this.config.enableConsole) return;

    const message = this.formatMessage(entry.level, entry.message, entry.data, entry.error);
    
    switch (entry.level) {
      case LogLevel.DEBUG:
        console.debug(message);
        break;
      case LogLevel.INFO:
        console.info(message);
        break;
      case LogLevel.WARN:
        console.warn(message);
        break;
      case LogLevel.ERROR:
      case LogLevel.FATAL:
        console.error(message);
        break;
    }
  }

  private async logToRemote(entry: LogEntry): Promise<void> {
    if (!this.config.enableRemote || !this.config.remoteEndpoint) return;

    try {
      await fetch(this.config.remoteEndpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(entry),
      });
    } catch (error) {
      // Fallback to console if remote logging fails
      console.error('Failed to send log to remote endpoint:', error);
    }
  }

  debug(message: string, data?: unknown): void {
    if (!this.shouldLog(LogLevel.DEBUG)) return;
    
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level: LogLevel.DEBUG,
      message,
      context: this.context,
      data
    };
    
    this.addEntry(entry);
    this.logToConsole(entry);
    this.logToRemote(entry);
  }

  info(message: string, data?: unknown): void {
    if (!this.shouldLog(LogLevel.INFO)) return;
    
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level: LogLevel.INFO,
      message,
      context: this.context,
      data
    };
    
    this.addEntry(entry);
    this.logToConsole(entry);
    this.logToRemote(entry);
  }

  warn(message: string, data?: unknown): void {
    if (!this.shouldLog(LogLevel.WARN)) return;
    
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level: LogLevel.WARN,
      message,
      context: this.context,
      data
    };
    
    this.addEntry(entry);
    this.logToConsole(entry);
    this.logToRemote(entry);
  }

  error(message: string, error?: Error, data?: unknown): void {
    if (!this.shouldLog(LogLevel.ERROR)) return;
    
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level: LogLevel.ERROR,
      message,
      context: this.context,
      data,
      ...(error && { error })
    };
    
    this.addEntry(entry);
    this.logToConsole(entry);
    this.logToRemote(entry);
  }

  fatal(message: string, error?: Error, data?: unknown): void {
    if (!this.shouldLog(LogLevel.FATAL)) return;
    
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level: LogLevel.FATAL,
      message,
      context: this.context,
      data,
      ...(error && { error })
    };
    
    this.addEntry(entry);
    this.logToConsole(entry);
    this.logToRemote(entry);
  }

  getEntries(level?: LogLevel): LogEntry[] {
    if (level !== undefined) {
      return this.entries.filter(entry => entry.level >= level);
    }
    return [...this.entries];
  }

  clear(): void {
    this.entries = [];
  }
}

// Default logger instance
export const logger = new Logger('CTAS', {
  level: import.meta.env.DEV ? LogLevel.DEBUG : LogLevel.INFO,
  enableConsole: true,
  enableRemote: false
});

