import { DemoDataUsageEvent, DemoDataReport, DemoDataAnalytics } from '@/types';

// Demo Data Tracking Service
export class DemoDataTracker {
  private static instance: DemoDataTracker;
  private events: DemoDataUsageEvent[] = [];
  private sessions: Map<string, { startTime: string; lastActivity: string; events: number }> = new Map();
  private isTrackingEnabled: boolean = true;

  private constructor() {
    this.loadEventsFromStorage();
    this.cleanupOldEvents();
  }

  public static getInstance(): DemoDataTracker {
    if (!DemoDataTracker.instance) {
      DemoDataTracker.instance = new DemoDataTracker();
    }
    return DemoDataTracker.instance;
  }

  // Track a demo data usage event
  public trackEvent(event: Omit<DemoDataUsageEvent, 'id' | 'timestamp' | 'sessionId' | 'userAgent'>): void {
    if (!this.isTrackingEnabled) return;

    const sessionId = this.getOrCreateSessionId();
    const fullEvent: DemoDataUsageEvent = {
      ...event,
      id: this.generateId(),
      timestamp: new Date().toISOString(),
      sessionId,
      userAgent: navigator.userAgent
    };

    this.events.push(fullEvent);
    this.updateSessionActivity(sessionId);
    this.saveEventsToStorage();
    this.cleanupOldEvents();
  }

  // Track search events
  public trackSearch(dataType: DemoDataUsageEvent['dataType'], query?: string, filters?: Record<string, any>, resultCount?: number, duration?: number): void {
    this.trackEvent({
      dataType,
      action: 'search',
      query: query || '',
      filters: filters || {},
      resultCount: resultCount || 0,
      duration: duration || 0,
      success: true
    });
  }

  // Track view events
  public trackView(dataType: DemoDataUsageEvent['dataType'], itemId?: string, duration?: number): void {
    this.trackEvent({
      dataType,
      action: 'view',
      query: itemId || '',
      duration: duration || 0,
      success: true
    });
  }

  // Track copy events
  public trackCopy(dataType: DemoDataUsageEvent['dataType'], itemId?: string): void {
    this.trackEvent({
      dataType,
      action: 'copy',
      query: itemId || '',
      success: true
    });
  }

  // Track statistics view
  public trackStatistics(dataType: DemoDataUsageEvent['dataType'], duration?: number): void {
    this.trackEvent({
      dataType,
      action: 'statistics',
      duration: duration || 0,
      success: true
    });
  }

  // Track errors
  public trackError(dataType: DemoDataUsageEvent['dataType'], action: DemoDataUsageEvent['action'], errorMessage: string, query?: string): void {
    this.trackEvent({
      dataType,
      action,
      query: query || '',
      success: false,
      errorMessage
    });
  }

  // Generate comprehensive report
  public generateReport(period: DemoDataReport['period'] = 'daily'): DemoDataReport {
    const now = new Date();
    const { startDate, endDate } = this.getDateRange(period);
    
    const periodEvents = this.events.filter(event => 
      event.timestamp >= startDate && event.timestamp <= endDate
    );

    const uniqueUsers = new Set(periodEvents.map(e => e.userId).filter(Boolean)).size;
    const uniqueSessions = new Set(periodEvents.map(e => e.sessionId)).size;

    // Calculate data type breakdown
    const dataTypeBreakdown = this.calculateDataTypeBreakdown(periodEvents);
    
    // Calculate action breakdown
    const actionBreakdown = this.calculateActionBreakdown(periodEvents);
    
    // Get top queries
    const topQueries = this.getTopQueries(periodEvents);
    
    // Get user activity
    const userActivity = this.getUserActivity(periodEvents);
    
    // Calculate performance metrics
    const performanceMetrics = this.calculatePerformanceMetrics(periodEvents);
    
    // Generate recommendations
    const recommendations = this.generateRecommendations(periodEvents, dataTypeBreakdown, actionBreakdown);

    return {
      id: this.generateId(),
      generatedAt: now.toISOString(),
      period,
      startDate,
      endDate,
      summary: {
        totalEvents: periodEvents.length,
        uniqueUsers,
        uniqueSessions,
        mostPopularDataType: dataTypeBreakdown[0]?.dataType || 'N/A',
        mostPopularAction: actionBreakdown[0]?.action || 'N/A',
        averageSessionDuration: this.calculateAverageSessionDuration(periodEvents),
        successRate: this.calculateSuccessRate(periodEvents)
      },
      dataTypeBreakdown,
      actionBreakdown,
      topQueries,
      userActivity,
      performanceMetrics,
      recommendations
    };
  }

  // Get real-time analytics
  public getRealTimeAnalytics(): DemoDataAnalytics {
    const now = new Date();
    const oneHourAgo = new Date(now.getTime() - 60 * 60 * 1000).toISOString();
    const oneDayAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000).toISOString();

    const recentEvents = this.events.filter(e => e.timestamp >= oneDayAgo);
    const hourlyEvents = this.events.filter(e => e.timestamp >= oneHourAgo);

    return {
      realTimeStats: {
        activeUsers: this.getActiveUsers(hourlyEvents),
        currentSessions: this.sessions.size,
        eventsThisHour: hourlyEvents.length,
        averageResponseTime: this.calculateAverageResponseTime(hourlyEvents)
      },
      trends: {
        dailyEvents: this.getDailyEvents(recentEvents),
        popularDataTypes: this.getPopularDataTypes(recentEvents),
        userGrowth: this.getUserGrowth(recentEvents)
      },
      insights: {
        peakUsageHours: this.getPeakUsageHours(recentEvents),
        mostActiveUsers: this.getMostActiveUsers(recentEvents),
        commonQueries: this.getCommonQueries(recentEvents)
      }
    };
  }

  // Get all events for a specific period
  public getEvents(startDate?: string, endDate?: string): DemoDataUsageEvent[] {
    if (!startDate && !endDate) {
      return [...this.events];
    }

    return this.events.filter(event => {
      if (startDate && event.timestamp < startDate) return false;
      if (endDate && event.timestamp > endDate) return false;
      return true;
    });
  }

  // Export events to JSON
  public exportEvents(startDate?: string, endDate?: string): string {
    const events = this.getEvents(startDate, endDate);
    return JSON.stringify(events, null, 2);
  }

  // Clear old events (keep last 30 days)
  public cleanupOldEvents(): void {
    const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString();
    this.events = this.events.filter(event => event.timestamp >= thirtyDaysAgo);
    this.saveEventsToStorage();
  }

  // Enable/disable tracking
  public setTrackingEnabled(enabled: boolean): void {
    this.isTrackingEnabled = enabled;
  }

  public isTracking(): boolean {
    return this.isTrackingEnabled;
  }

  // Private helper methods
  private generateId(): string {
    return `event_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private getOrCreateSessionId(): string {
    const existingSessionId = sessionStorage.getItem('ctas_demo_session_id');
    if (existingSessionId) {
      return existingSessionId;
    }

    const sessionId = `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    sessionStorage.setItem('ctas_demo_session_id', sessionId);
    return sessionId;
  }

  private updateSessionActivity(sessionId: string): void {
    const now = new Date().toISOString();
    const existing = this.sessions.get(sessionId);
    
    if (existing) {
      existing.lastActivity = now;
      existing.events++;
    } else {
      this.sessions.set(sessionId, {
        startTime: now,
        lastActivity: now,
        events: 1
      });
    }
  }

  private getDateRange(period: DemoDataReport['period']): { startDate: string; endDate: string } {
    const now = new Date();
    let startDate: Date;

    switch (period) {
      case 'hourly':
        startDate = new Date(now.getTime() - 60 * 60 * 1000);
        break;
      case 'daily':
        startDate = new Date(now.getTime() - 24 * 60 * 60 * 1000);
        break;
      case 'weekly':
        startDate = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
        break;
      case 'monthly':
        startDate = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
        break;
      default:
        startDate = new Date(now.getTime() - 24 * 60 * 60 * 1000);
    }

    return {
      startDate: startDate.toISOString(),
      endDate: now.toISOString()
    };
  }

  private calculateDataTypeBreakdown(events: DemoDataUsageEvent[]) {
    const dataTypeBreakdown = new Map<string, { count: number; users: Set<string>; durations: number[]; successes: number }>();

    events.forEach(event => {
      const existing = dataTypeBreakdown.get(event.dataType) || {
        count: 0,
        users: new Set<string>(),
        durations: [],
        successes: 0
      };

      existing.count++;
      if (event.userId) existing.users.add(event.userId);
      if (event.duration) existing.durations.push(event.duration);
      if (event.success) existing.successes++;

      dataTypeBreakdown.set(event.dataType, existing);
    });

    return Array.from(dataTypeBreakdown.entries()).map(([dataType, data]) => ({
      dataType,
      eventCount: data.count,
      uniqueUsers: data.users.size,
      averageDuration: data.durations.length > 0 ? data.durations.reduce((a, b) => a + b, 0) / data.durations.length : 0,
      successRate: data.count > 0 ? (data.successes / data.count) * 100 : 0
    })).sort((a, b) => b.eventCount - a.eventCount);
  }

  private calculateActionBreakdown(events: DemoDataUsageEvent[]) {
    const actionBreakdown = new Map<string, { count: number; users: Set<string>; durations: number[] }>();

    events.forEach(event => {
      const existing = actionBreakdown.get(event.action) || {
        count: 0,
        users: new Set<string>(),
        durations: []
      };

      existing.count++;
      if (event.userId) existing.users.add(event.userId);
      if (event.duration) existing.durations.push(event.duration);

      actionBreakdown.set(event.action, existing);
    });

    return Array.from(actionBreakdown.entries()).map(([action, data]) => ({
      action,
      eventCount: data.count,
      uniqueUsers: data.users.size,
      averageDuration: data.durations.length > 0 ? data.durations.reduce((a, b) => a + b, 0) / data.durations.length : 0
    })).sort((a, b) => b.eventCount - a.eventCount);
  }

  private getTopQueries(events: DemoDataUsageEvent[]) {
    const topQueryCounts = new Map<string, { count: number; durations: number[] }>();

    events.forEach(event => {
      if (event.query) {
        const existing = topQueryCounts.get(event.query) || { count: 0, durations: [] };
        existing.count++;
        if (event.duration) existing.durations.push(event.duration);
        topQueryCounts.set(event.query, existing);
      }
    });

    return Array.from(topQueryCounts.entries())
      .map(([query, data]) => ({
        query,
        count: data.count,
        averageDuration: data.durations.length > 0 ? data.durations.reduce((a, b) => a + b, 0) / data.durations.length : 0
      }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);
  }

  private getUserActivity(events: DemoDataUsageEvent[]) {
    const userActivityMap = new Map<string, {
      eventCount: number;
      sessions: Set<string>;
      lastActivity: string;
      dataTypes: Map<string, number>;
    }>();

    events.forEach(event => {
      if (event.userId) {
        const existing = userActivityMap.get(event.userId) || {
          eventCount: 0,
          sessions: new Set<string>(),
          lastActivity: '',
          dataTypes: new Map<string, number>()
        };

        existing.eventCount++;
        existing.sessions.add(event.sessionId);
        if (event.timestamp > existing.lastActivity) {
          existing.lastActivity = event.timestamp;
        }

        const currentCount = existing.dataTypes.get(event.dataType) || 0;
        existing.dataTypes.set(event.dataType, currentCount + 1);

        userActivityMap.set(event.userId, existing);
      }
    });

    return Array.from(userActivityMap.entries()).map(([userId, data]) => ({
      userId,
      eventCount: data.eventCount,
      sessionCount: data.sessions.size,
      lastActivity: data.lastActivity,
      favoriteDataType: Array.from(data.dataTypes.entries())
        .sort((a, b) => b[1] - a[1])[0]?.[0] || 'N/A'
    })).sort((a, b) => b.eventCount - a.eventCount);
  }

  private calculatePerformanceMetrics(events: DemoDataUsageEvent[]) {
    const durations = events.map(e => e.duration).filter(Boolean) as number[];
    const errors = events.filter(e => !e.success).length;
    const totalDataTransferred = events.length * 1024; // Estimate 1KB per event

    return {
      averageResponseTime: durations.length > 0 ? durations.reduce((a, b) => a + b, 0) / durations.length : 0,
      peakUsageTime: this.findPeakUsageTime(events),
      totalDataTransferred,
      errorRate: events.length > 0 ? (errors / events.length) * 100 : 0
    };
  }

  private calculateAverageSessionDuration(events: DemoDataUsageEvent[]): number {
    const sessionDurations = new Map<string, { start: string; end: string }>();

    events.forEach(event => {
      const existing = sessionDurations.get(event.sessionId);
      if (!existing) {
        sessionDurations.set(event.sessionId, { start: event.timestamp, end: event.timestamp });
      } else {
        if (event.timestamp < existing.start) existing.start = event.timestamp;
        if (event.timestamp > existing.end) existing.end = event.timestamp;
      }
    });

    const durations = Array.from(sessionDurations.values()).map(session => 
      new Date(session.end).getTime() - new Date(session.start).getTime()
    );

    return durations.length > 0 ? durations.reduce((a, b) => a + b, 0) / durations.length : 0;
  }

  private calculateSuccessRate(events: DemoDataUsageEvent[]): number {
    const successes = events.filter(e => e.success).length;
    return events.length > 0 ? (successes / events.length) * 100 : 0;
  }

  private generateRecommendations(events: DemoDataUsageEvent[], dataTypeBreakdown: any[], actionBreakdown: any[]): string[] {
    const recommendations: string[] = [];

    // Analyze usage patterns
    const totalEvents = events.length;
    const searchEvents = events.filter(e => e.action === 'search').length;
    const viewEvents = events.filter(e => e.action === 'view').length;

    if (searchEvents > viewEvents * 2) {
      recommendations.push('Consider improving search result relevance as users are performing many searches');
    }

    if (dataTypeBreakdown.length > 0) {
      const mostPopular = dataTypeBreakdown[0];
      const leastPopular = dataTypeBreakdown[dataTypeBreakdown.length - 1];
      
      if (mostPopular.eventCount > leastPopular.eventCount * 5) {
        recommendations.push(`Promote ${leastPopular.dataType} features as ${mostPopular.dataType} is heavily used`);
      }
    }

    const errorRate = this.calculateSuccessRate(events);
    if (errorRate < 95) {
      recommendations.push('Investigate and fix errors to improve user experience');
    }

    if (totalEvents > 1000) {
      recommendations.push('Consider implementing caching to improve performance');
    }

    return recommendations;
  }

  private getActiveUsers(events: DemoDataUsageEvent[]): number {
    const uniqueUsers = new Set(events.map(e => e.userId).filter(Boolean));
    return uniqueUsers.size;
  }

  private calculateAverageResponseTime(events: DemoDataUsageEvent[]): number {
    const durations = events.map(e => e.duration).filter(Boolean) as number[];
    return durations.length > 0 ? durations.reduce((a, b) => a + b, 0) / durations.length : 0;
  }

  private getDailyEvents(events: DemoDataUsageEvent[]): Array<{ date: string; count: number }> {
    const dailyCounts = new Map<string, number>();

    events.forEach(event => {
      const date = event.timestamp.split('T')[0];
      if (date) {
        dailyCounts.set(date, (dailyCounts.get(date) || 0) + 1);
      }
    });

    return Array.from(dailyCounts.entries())
      .map(([date, count]) => ({ date, count }))
      .sort((a, b) => a.date.localeCompare(b.date));
  }

  private getPopularDataTypes(events: DemoDataUsageEvent[]): Array<{ dataType: string; percentage: number }> {
    const dataTypeCounts = new Map<string, number>();
    const total = events.length;

    events.forEach(event => {
      dataTypeCounts.set(event.dataType, (dataTypeCounts.get(event.dataType) || 0) + 1);
    });

    return Array.from(dataTypeCounts.entries())
      .map(([dataType, count]) => ({
        dataType,
        percentage: total > 0 ? (count / total) * 100 : 0
      }))
      .sort((a, b) => b.percentage - a.percentage);
  }

  private getUserGrowth(events: DemoDataUsageEvent[]): Array<{ date: string; users: number }> {
    const dailyUsers = new Map<string, Set<string>>();

    events.forEach(event => {
      if (event.userId) {
        const date = event.timestamp.split('T')[0];
        if (date) {
          if (!dailyUsers.has(date)) {
            dailyUsers.set(date, new Set());
          }
          dailyUsers.get(date)!.add(event.userId);
        }
      }
    });

    return Array.from(dailyUsers.entries())
      .map(([date, users]) => ({ date, users: users.size }))
      .sort((a, b) => a.date.localeCompare(b.date));
  }

  private getPeakUsageHours(events: DemoDataUsageEvent[]): Array<{ hour: number; count: number }> {
    const hourlyCounts = new Map<number, number>();

    events.forEach(event => {
      const hour = new Date(event.timestamp).getHours();
      hourlyCounts.set(hour, (hourlyCounts.get(hour) || 0) + 1);
    });

    return Array.from(hourlyCounts.entries())
      .map(([hour, count]) => ({ hour, count }))
      .sort((a, b) => a.hour - b.hour);
  }

  private getMostActiveUsers(events: DemoDataUsageEvent[]): Array<{ userId: string; activity: number }> {
    const mostActiveUsersMap = new Map<string, number>();

    events.forEach(event => {
      if (event.userId) {
        mostActiveUsersMap.set(event.userId, (mostActiveUsersMap.get(event.userId) || 0) + 1);
      }
    });

    return Array.from(mostActiveUsersMap.entries())
      .map(([userId, activity]) => ({ userId, activity }))
      .sort((a, b) => b.activity - a.activity)
      .slice(0, 10);
  }

  private getCommonQueries(events: DemoDataUsageEvent[]): Array<{ query: string; frequency: number }> {
    const commonQueryCounts = new Map<string, number>();

    events.forEach(event => {
      if (event.query) {
        commonQueryCounts.set(event.query, (commonQueryCounts.get(event.query) || 0) + 1);
      }
    });

    return Array.from(commonQueryCounts.entries())
      .map(([query, frequency]) => ({ query, frequency }))
      .sort((a, b) => b.frequency - a.frequency)
      .slice(0, 10);
  }

  private findPeakUsageTime(events: DemoDataUsageEvent[]): string {
    if (events.length === 0) return 'N/A';

    const hourlyCounts = new Map<number, number>();
    events.forEach(event => {
      const hour = new Date(event.timestamp).getHours();
      hourlyCounts.set(hour, (hourlyCounts.get(hour) || 0) + 1);
    });

    const peakHour = Array.from(hourlyCounts.entries())
      .sort((a, b) => b[1] - a[1])[0]?.[0] || 0;

    return `${peakHour.toString().padStart(2, '0')}:00`;
  }

  private saveEventsToStorage(): void {
    try {
      localStorage.setItem('ctas_demo_events', JSON.stringify(this.events));
    } catch (error) {
      console.warn('Failed to save demo events to localStorage:', error);
    }
  }

  private loadEventsFromStorage(): void {
    try {
      const stored = localStorage.getItem('ctas_demo_events');
      if (stored) {
        this.events = JSON.parse(stored);
      }
    } catch (error) {
      console.warn('Failed to load demo events from localStorage:', error);
      this.events = [];
    }
  }
}

// Export singleton instance
export const demoDataTracker = DemoDataTracker.getInstance();
