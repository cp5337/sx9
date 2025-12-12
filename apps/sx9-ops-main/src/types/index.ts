// Core CTAS Types

export interface InfoStream {
  id: string;
  title: string;
  content: string;
  timestamp: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
  source: string;
  tags: string[];
}

export interface KnowledgeNode {
  id: string;
  label: string;
  type: string;
  properties: Record<string, any>;
  position: { x: number; y: number };
}

export interface ShodanResult {
  id: string;
  ip: string;
  port: number;
  hostname: string;
  product: string;
  version: string;
  vulns: string[];
  tags: string[];
  location: {
    country: string;
    city: string;
    coordinates: [number, number];
  };
  lastUpdate: string;
}

export interface SearchResult {
  id: string;
  title: string;
  content: string;
  type: string;
  relevance: number;
  source: string;
  timestamp: string;
}

export interface N8NWorkflow {
  id: string;
  name: string;
  description: string;
  phase: string;
  status: string;
  nodes: number;
  lastRun: string;
  successRate: number;
}

// Task Types
export interface CTASTask {
  id: string;
  number: string;
  title: string;
  description: string;
  status: 'Pending' | 'In Progress' | 'Completed';
  relatedActorId?: string;
  isSection: boolean;
}

export interface ThreatActor {
  id: string;
  name: string;
  type: string;
  country: string;
  capabilities: string[];
  targets: string[];
}

// Hunt Phase Types
export interface NetworkScanResult {
  id: string;
  ip: string;
  port: number;
  service: string;
  version: string;
  banner: string;
  timestamp: string;
}

export interface OSINTResult {
  id: string;
  target: string;
  platform: string;
  username: string;
  url: string;
  followers: number;
  lastActive: string;
  verified: boolean;
}

// Exploit DB Types
export interface ExploitDBExploit {
  id: string;
  title: string;
  description: string;
  author: string;
  date: string;
  type: 'exploit' | 'shellcode' | 'paper' | 'dos';
  platform: string;
  port?: number;
  verified: boolean;
  verifiedDate?: string;
  verifiedBy?: string;
  tags: string[];
  cve?: string[];
  cvss?: number;
  references: string[];
  code?: string;
  rawCode?: string;
  downloadUrl?: string;
  sourceUrl?: string;
  application?: string;
  version?: string;
  language?: string;
  architecture?: string;
  complexity?: 'low' | 'medium' | 'high';
  authentication?: boolean;
  confidentiality?: 'none' | 'partial' | 'complete';
  integrity?: 'none' | 'partial' | 'complete';
  availability?: 'none' | 'partial' | 'complete';
}

export interface ExploitDBSearchParams {
  query?: string;
  type?: 'exploit' | 'shellcode' | 'paper' | 'dos';
  platform?: string;
  author?: string;
  port?: number;
  verified?: boolean;
  cve?: string;
  cvss?: number;
  dateFrom?: string;
  dateTo?: string;
  limit?: number;
  offset?: number;
}

export interface ExploitDBSearchResult {
  exploits: ExploitDBExploit[];
  total: number;
  page: number;
  limit: number;
  hasMore: boolean;
}

export interface ExploitDBStats {
  totalExploits: number;
  totalShellcodes: number;
  totalPapers: number;
  totalDos: number;
  verifiedExploits: number;
  recentExploits: number;
  topPlatforms: Array<{ platform: string; count: number }>;
  topAuthors: Array<{ author: string; count: number }>;
  topCVEs: Array<{ cve: string; count: number }>;
}

// Demo Data Tracking Types
export interface DemoDataUsageEvent {
  id: string;
  timestamp: string;
  dataType: 'exploitDB' | 'shodan' | 'osint' | 'threatIntel' | 'knowledgeGraph' | 'infoStreams' | 'n8nWorkflows' | 'geospatialData';
  action: 'search' | 'view' | 'download' | 'copy' | 'export' | 'statistics';
  query?: string;
  filters?: Record<string, any>;
  resultCount?: number;
  userId?: string;
  sessionId: string;
  userAgent: string;
  ipAddress?: string;
  duration?: number; // milliseconds
  success: boolean;
  errorMessage?: string;
}

export interface DemoDataReport {
  id: string;
  generatedAt: string;
  period: 'hourly' | 'daily' | 'weekly' | 'monthly';
  startDate: string;
  endDate: string;
  summary: {
    totalEvents: number;
    uniqueUsers: number;
    uniqueSessions: number;
    mostPopularDataType: string;
    mostPopularAction: string;
    averageSessionDuration: number;
    successRate: number;
  };
  dataTypeBreakdown: Array<{
    dataType: string;
    eventCount: number;
    uniqueUsers: number;
    averageDuration: number;
    successRate: number;
  }>;
  actionBreakdown: Array<{
    action: string;
    eventCount: number;
    uniqueUsers: number;
    averageDuration: number;
  }>;
  topQueries: Array<{
    query: string;
    count: number;
    averageDuration: number;
  }>;
  userActivity: Array<{
    userId: string;
    eventCount: number;
    sessionCount: number;
    lastActivity: string;
    favoriteDataType: string;
  }>;
  performanceMetrics: {
    averageResponseTime: number;
    peakUsageTime: string;
    totalDataTransferred: number;
    errorRate: number;
  };
  recommendations: string[];
}

export interface DemoDataAnalytics {
  realTimeStats: {
    activeUsers: number;
    currentSessions: number;
    eventsThisHour: number;
    averageResponseTime: number;
  };
  trends: {
    dailyEvents: Array<{ date: string; count: number }>;
    popularDataTypes: Array<{ dataType: string; percentage: number }>;
    userGrowth: Array<{ date: string; users: number }>;
  };
  insights: {
    peakUsageHours: Array<{ hour: number; count: number }>;
    mostActiveUsers: Array<{ userId: string; activity: number }>;
    commonQueries: Array<{ query: string; frequency: number }>;
  };
}

// Stack Types
export interface Stack {
  id: string;
  name: string;
  attackSurface: string;
  target: string;
  vRavenInstance: string;
  k8sConfig: string;
  ipAddress?: string;
}

// Mission Types
export interface Mission {
  id: string;
  name: string;
  target: string;
  status: string;
  progress: number;
  startTime: string;
  endTime?: string;
}
