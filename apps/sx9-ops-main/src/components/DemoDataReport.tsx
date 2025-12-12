import React, { useState, useEffect } from 'react';
import { 
  BarChart3, 
  Users, 
  Activity, 
  TrendingUp, 
  Clock, 
  Download, 
  RefreshCw, 
  Eye, 
  Search, 
  Copy, 
  FileText,
  AlertTriangle,
  CheckCircle,
  X,
  Calendar,
  Filter,
  PieChart,
  LineChart,
  Target,
  Zap,
  Database,
  Globe,
  Smartphone,
  Monitor,
  Server,
  Map
} from 'lucide-react';
import { demoDataTracker } from '@/utils/demoDataTracker';
import type { DemoDataReport as DemoDataReportType, DemoDataAnalytics, DemoDataUsageEvent } from '@/types';

interface DemoDataReportProps {
  className?: string;
}

const DemoDataReport: React.FC<DemoDataReportProps> = ({ className = '' }) => {
  const [report, setReport] = useState<DemoDataReportType | null>(null);
  const [analytics, setAnalytics] = useState<DemoDataAnalytics | null>(null);
  const [selectedPeriod, setSelectedPeriod] = useState<DemoDataReportType['period']>('daily');
  const [isLoading, setIsLoading] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [showRawData, setShowRawData] = useState(false);
  const [rawEvents, setRawEvents] = useState<DemoDataUsageEvent[]>([]);

  // Generate report and analytics
  const generateReport = () => {
    setIsLoading(true);
    try {
      const newReport = demoDataTracker.generateReport(selectedPeriod);
      const newAnalytics = demoDataTracker.getRealTimeAnalytics();
      setReport(newReport);
      setAnalytics(newAnalytics);
      setRawEvents(demoDataTracker.getEvents());
    } catch (error) {
      console.error('Failed to generate report:', error);
    } finally {
      setIsLoading(false);
    }
  };

  // Auto-refresh every 30 seconds
  useEffect(() => {
    generateReport();

    if (autoRefresh) {
      const interval = setInterval(generateReport, 30000);
      return () => clearInterval(interval);
    }
  }, [selectedPeriod, autoRefresh]);

  // Export report to JSON
  const exportReport = () => {
    if (!report) return;

    const dataStr = JSON.stringify(report, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `demo-data-report-${selectedPeriod}-${new Date().toISOString().split('T')[0]}.json`;
    link.click();
    URL.revokeObjectURL(url);
  };

  // Export raw events
  const exportRawEvents = () => {
    const dataStr = demoDataTracker.exportEvents();
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `demo-events-${new Date().toISOString().split('T')[0]}.json`;
    link.click();
    URL.revokeObjectURL(url);
  };

  const getDataTypeIcon = (dataType: string) => {
    switch (dataType) {
      case 'exploitDB': return <Database className="w-4 h-4" />;
      case 'shodan': return <Globe className="w-4 h-4" />;
      case 'osint': return <Search className="w-4 h-4" />;
      case 'threatIntel': return <AlertTriangle className="w-4 h-4" />;
      case 'knowledgeGraph': return <Target className="w-4 h-4" />;
      case 'infoStreams': return <Activity className="w-4 h-4" />;
      case 'n8nWorkflows': return <Zap className="w-4 h-4" />;
      case 'geospatialData': return <Map className="w-4 h-4" />;
      default: return <FileText className="w-4 h-4" />;
    }
  };

  const getActionIcon = (action: string) => {
    switch (action) {
      case 'search': return <Search className="w-4 h-4" />;
      case 'view': return <Eye className="w-4 h-4" />;
      case 'copy': return <Copy className="w-4 h-4" />;
      case 'download': return <Download className="w-4 h-4" />;
      case 'export': return <FileText className="w-4 h-4" />;
      case 'statistics': return <BarChart3 className="w-4 h-4" />;
      default: return <Activity className="w-4 h-4" />;
    }
  };

  const formatDuration = (ms: number): string => {
    if (ms < 1000) return `${ms}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
    return `${(ms / 60000).toFixed(1)}m`;
  };

  const formatBytes = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  if (!report || !analytics) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 ${className}`}>
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-sm ${className}`}>
      {/* Header */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white flex items-center">
              <BarChart3 className="mr-2" />
              Demo Data Usage Report
            </h2>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Generated at {new Date(report.generatedAt).toLocaleString()}
            </p>
          </div>
          <div className="flex items-center space-x-2">
            <select
              value={selectedPeriod}
              onChange={(e) => setSelectedPeriod(e.target.value as DemoDataReportType['period'])}
              className="px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
            >
              <option value="hourly">Last Hour</option>
              <option value="daily">Last 24 Hours</option>
              <option value="weekly">Last Week</option>
              <option value="monthly">Last Month</option>
            </select>
            <button
              onClick={generateReport}
              disabled={isLoading}
              className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 disabled:opacity-50"
              title="Refresh Report"
            >
              <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
            </button>
            <button
              onClick={() => setAutoRefresh(!autoRefresh)}
              className={`p-2 rounded ${autoRefresh ? 'text-blue-500' : 'text-gray-400'}`}
              title="Auto Refresh"
            >
              <Clock className="w-4 h-4" />
            </button>
            <button
              onClick={exportReport}
              className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              title="Export Report"
            >
              <Download className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Real-time Stats */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Real-time Statistics</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <Users className="w-6 h-6 text-blue-500" />
              <div className="ml-3">
                <p className="text-sm font-medium text-blue-600 dark:text-blue-400">Active Users</p>
                <p className="text-2xl font-bold text-blue-900 dark:text-blue-100">{analytics.realTimeStats.activeUsers}</p>
              </div>
            </div>
          </div>
          <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <Activity className="w-6 h-6 text-green-500" />
              <div className="ml-3">
                <p className="text-sm font-medium text-green-600 dark:text-green-400">Current Sessions</p>
                <p className="text-2xl font-bold text-green-900 dark:text-green-100">{analytics.realTimeStats.currentSessions}</p>
              </div>
            </div>
          </div>
          <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <TrendingUp className="w-6 h-6 text-purple-500" />
              <div className="ml-3">
                <p className="text-sm font-medium text-purple-600 dark:text-purple-400">Events This Hour</p>
                <p className="text-2xl font-bold text-purple-900 dark:text-purple-100">{analytics.realTimeStats.eventsThisHour}</p>
              </div>
            </div>
          </div>
          <div className="bg-orange-50 dark:bg-orange-900/20 p-4 rounded-lg">
            <div className="flex items-center">
              <Clock className="w-6 h-6 text-orange-500" />
              <div className="ml-3">
                <p className="text-sm font-medium text-orange-600 dark:text-orange-400">Avg Response</p>
                <p className="text-2xl font-bold text-orange-900 dark:text-orange-100">{formatDuration(analytics.realTimeStats.averageResponseTime)}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Summary */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Summary</h3>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{report.summary.totalEvents}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Total Events</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{report.summary.uniqueUsers}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Unique Users</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{report.summary.uniqueSessions}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Sessions</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{report.summary.mostPopularDataType}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Popular Data Type</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{formatDuration(report.summary.averageSessionDuration)}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Avg Session</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-gray-900 dark:text-white">{report.summary.successRate.toFixed(1)}%</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Success Rate</p>
          </div>
        </div>
      </div>

      {/* Data Type Breakdown */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Data Type Usage</h3>
        <div className="space-y-3">
          {report.dataTypeBreakdown.map((item, index) => (
            <div key={item.dataType} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <div className="flex items-center">
                {getDataTypeIcon(item.dataType)}
                <span className="ml-2 font-medium text-gray-900 dark:text-white">{item.dataType}</span>
              </div>
              <div className="flex items-center space-x-4 text-sm">
                <span className="text-gray-600 dark:text-gray-400">{item.eventCount} events</span>
                <span className="text-gray-600 dark:text-gray-400">{item.uniqueUsers} users</span>
                <span className="text-gray-600 dark:text-gray-400">{formatDuration(item.averageDuration)}</span>
                <span className={`font-medium ${item.successRate >= 95 ? 'text-green-600' : item.successRate >= 80 ? 'text-yellow-600' : 'text-red-600'}`}>
                  {item.successRate.toFixed(1)}%
                </span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Action Breakdown */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Action Breakdown</h3>
        <div className="space-y-3">
          {report.actionBreakdown.map((item, index) => (
            <div key={item.action} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <div className="flex items-center">
                {getActionIcon(item.action)}
                <span className="ml-2 font-medium text-gray-900 dark:text-white capitalize">{item.action}</span>
              </div>
              <div className="flex items-center space-x-4 text-sm">
                <span className="text-gray-600 dark:text-gray-400">{item.eventCount} events</span>
                <span className="text-gray-600 dark:text-gray-400">{item.uniqueUsers} users</span>
                <span className="text-gray-600 dark:text-gray-400">{formatDuration(item.averageDuration)}</span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Top Queries */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Top Queries</h3>
        <div className="space-y-2">
          {report.topQueries.map((query, index) => (
            <div key={index} className="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700 rounded">
              <span className="text-sm text-gray-900 dark:text-white font-mono truncate flex-1">{query.query}</span>
              <div className="flex items-center space-x-3 text-sm text-gray-600 dark:text-gray-400">
                <span>{query.count} times</span>
                <span>{formatDuration(query.averageDuration)}</span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Performance Metrics */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Performance Metrics</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="text-center">
            <p className="text-lg font-bold text-gray-900 dark:text-white">{formatDuration(report.performanceMetrics.averageResponseTime)}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Avg Response Time</p>
          </div>
          <div className="text-center">
            <p className="text-lg font-bold text-gray-900 dark:text-white">{report.performanceMetrics.peakUsageTime}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Peak Usage Time</p>
          </div>
          <div className="text-center">
            <p className="text-lg font-bold text-gray-900 dark:text-white">{formatBytes(report.performanceMetrics.totalDataTransferred)}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Data Transferred</p>
          </div>
          <div className="text-center">
            <p className="text-lg font-bold text-gray-900 dark:text-white">{report.performanceMetrics.errorRate.toFixed(1)}%</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">Error Rate</p>
          </div>
        </div>
      </div>

      {/* Recommendations */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Recommendations</h3>
        <div className="space-y-2">
          {report.recommendations.map((recommendation, index) => (
            <div key={index} className="flex items-start p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <CheckCircle className="w-5 h-5 text-blue-500 mt-0.5 mr-3 flex-shrink-0" />
              <p className="text-sm text-blue-900 dark:text-blue-100">{recommendation}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Raw Data Toggle */}
      <div className="p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium text-gray-900 dark:text-white">Raw Event Data</h3>
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setShowRawData(!showRawData)}
              className="flex items-center px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600"
            >
              {showRawData ? <X className="w-4 h-4 mr-1" /> : <Eye className="w-4 h-4 mr-1" />}
              {showRawData ? 'Hide' : 'Show'} Raw Data
            </button>
            <button
              onClick={exportRawEvents}
              className="flex items-center px-3 py-1 text-sm bg-blue-100 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 rounded hover:bg-blue-200 dark:hover:bg-blue-900/30"
            >
              <Download className="w-4 h-4 mr-1" />
              Export Events
            </button>
          </div>
        </div>
        
        {showRawData && (
          <div className="bg-gray-900 text-gray-100 rounded-lg p-4 overflow-x-auto max-h-96">
            <pre className="text-xs font-mono">
              {JSON.stringify(rawEvents.slice(0, 50), null, 2)}
              {rawEvents.length > 50 && `\n... and ${rawEvents.length - 50} more events`}
            </pre>
          </div>
        )}
      </div>
    </div>
  );
};

export default DemoDataReport;
