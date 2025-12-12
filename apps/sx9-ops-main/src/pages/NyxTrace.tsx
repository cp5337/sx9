import React, { useState, useEffect } from 'react';
import { Target, Map, Network, Newspaper, Brain, Search, BarChart3, Layers, Monitor, Globe2, TrendingUp, Settings, ExternalLink, AlertTriangle, Shield, Zap, Globe, Database, RefreshCw, Eye, Users } from 'lucide-react';

interface NyxModule {
  id: string;
  name: string;
  category: string;
  description: string;
  status: 'active' | 'inactive' | 'error';
  icon: React.ReactNode;
  metrics: {
    throughput: number;
    latency: number;
    accuracy: number;
  };
}

interface TraceData {
  id: string;
  timestamp: string;
  source: string;
  target: string;
  action: string;
  result: 'success' | 'failure' | 'pending';
  metadata: Record<string, any>;
}

const NyxTrace: React.FC = () => {
  const [modules, setModules] = useState<NyxModule[]>([]);
  const [traces, setTraces] = useState<TraceData[]>([]);
  const [selectedModule, setSelectedModule] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    loadNyxModules();
    loadTraceData();
  }, []);

  const loadNyxModules = async () => {
    setIsLoading(true);
    try {
      // Production implementation would load from API
      const demoModules = getDemoModules();
      setModules(demoModules);
    } catch (error) {
      console.error('Failed to load Nyx modules:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const loadTraceData = async () => {
    try {
      // Production implementation would load from API
      const demoTraces = getDemoTraceData();
      setTraces(demoTraces);
    } catch (error) {
      console.error('Failed to load trace data:', error);
    }
  };

  const getDemoModules = (): NyxModule[] => [
    {
      id: 'intelligence',
      name: 'Intelligence Engine',
      category: 'Core',
      description: 'AI-powered threat intelligence analysis',
      status: 'active',
      icon: <Brain className="w-5 h-5" />,
      metrics: { throughput: 95, latency: 120, accuracy: 98 }
    },
    {
      id: 'network',
      name: 'Network Monitor',
      category: 'Monitoring',
      description: 'Real-time network traffic analysis',
      status: 'active',
      icon: <Network className="w-5 h-5" />,
      metrics: { throughput: 87, latency: 45, accuracy: 94 }
    },
    {
      id: 'threat',
      name: 'Threat Detection',
      category: 'Security',
      description: 'Advanced threat detection algorithms',
      status: 'active',
      icon: <Shield className="w-5 h-5" />,
      metrics: { throughput: 92, latency: 78, accuracy: 96 }
    },
    {
      id: 'analytics',
      name: 'Analytics Engine',
      category: 'Core',
      description: 'Data analytics and pattern recognition',
      status: 'active',
      icon: <BarChart3 className="w-5 h-5" />,
      metrics: { throughput: 89, latency: 156, accuracy: 97 }
    },
    {
      id: 'geospatial',
      name: 'Geospatial Analysis',
      category: 'Analysis',
      description: 'Geographic threat mapping and analysis',
      status: 'active',
      icon: <Map className="w-5 h-5" />,
      metrics: { throughput: 76, latency: 234, accuracy: 91 }
    },
    {
      id: 'social',
      name: 'Social Intelligence',
      category: 'Intelligence',
      description: 'Social media and OSINT analysis',
      status: 'active',
      icon: <Users className="w-5 h-5" />,
      metrics: { throughput: 82, latency: 189, accuracy: 93 }
    }
  ];

  const getDemoTraceData = (): TraceData[] => [
    {
      id: '1',
      timestamp: new Date(Date.now() - 1000 * 60 * 5).toISOString(),
      source: 'intelligence',
      target: 'threat-db',
      action: 'threat_analysis',
      result: 'success',
      metadata: { confidence: 0.95, threat_level: 'high' }
    },
    {
      id: '2',
      timestamp: new Date(Date.now() - 1000 * 60 * 3).toISOString(),
      source: 'network',
      target: 'firewall',
      action: 'traffic_analysis',
      result: 'success',
      metadata: { packets_analyzed: 15420, anomalies: 3 }
    },
    {
      id: '3',
      timestamp: new Date(Date.now() - 1000 * 60 * 1).toISOString(),
      source: 'threat',
      target: 'alert-system',
      action: 'threat_detection',
      result: 'success',
      metadata: { threat_id: 'APT-2024-001', severity: 'critical' }
    }
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800';
      case 'inactive':
        return 'bg-gray-100 text-gray-800';
      case 'error':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <div className="w-2 h-2 bg-green-500 rounded-full" />;
      case 'inactive':
        return <div className="w-2 h-2 bg-gray-500 rounded-full" />;
      case 'error':
        return <div className="w-2 h-2 bg-red-500 rounded-full" />;
      default:
        return <div className="w-2 h-2 bg-gray-500 rounded-full" />;
    }
  };

  const getResultColor = (result: string) => {
    switch (result) {
      case 'success':
        return 'text-green-600';
      case 'failure':
        return 'text-red-600';
      case 'pending':
        return 'text-yellow-600';
      default:
        return 'text-gray-600';
    }
  };

  const filteredModules = selectedModule 
    ? modules.filter(m => m.category === selectedModule)
    : modules;

  const groupedModules = modules.reduce((acc, module) => {
    if (!acc[module.category]) {
      acc[module.category] = [];
    }
    acc[module.category]?.push(module);
    return acc;
  }, {} as Record<string, NyxModule[]>);

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Brain className="w-8 h-8 text-purple-600" />
              <h1 className="text-3xl font-bold text-gray-900">Nyx Trace</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-purple-100 text-purple-800 px-3 py-1 rounded text-sm font-semibold">
                AI Intelligence Platform
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Advanced AI-powered threat intelligence and trace analysis platform.
          </p>

          {/* System Overview */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Database className="w-4 h-4 text-blue-500" />
                <span className="text-sm font-medium text-blue-800">Active Modules</span>
              </div>
              <span className="text-2xl font-bold text-blue-900">
                {modules.filter(m => m.status === 'active').length}
              </span>
            </div>
            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <TrendingUp className="w-4 h-4 text-green-500" />
                <span className="text-sm font-medium text-green-800">Avg Throughput</span>
              </div>
              <span className="text-2xl font-bold text-green-900">
                {Math.round(modules.reduce((sum, m) => sum + m.metrics.throughput, 0) / modules.length)}%
              </span>
            </div>
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Zap className="w-4 h-4 text-yellow-500" />
                <span className="text-sm font-medium text-yellow-800">Avg Latency</span>
              </div>
              <span className="text-2xl font-bold text-yellow-900">
                {Math.round(modules.reduce((sum, m) => sum + m.metrics.latency, 0) / modules.length)}ms
              </span>
            </div>
            <div className="bg-purple-50 border border-purple-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Target className="w-4 h-4 text-purple-500" />
                <span className="text-sm font-medium text-purple-800">Avg Accuracy</span>
              </div>
              <span className="text-2xl font-bold text-purple-900">
                {Math.round(modules.reduce((sum, m) => sum + m.metrics.accuracy, 0) / modules.length)}%
              </span>
            </div>
          </div>
        </div>

        {/* Module Categories */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6">Module Categories</h2>
          
          <div className="flex gap-4 mb-6">
            <button
              onClick={() => setSelectedModule(null)}
              className={`px-4 py-2 rounded-md font-medium ${
                selectedModule === null 
                  ? 'bg-purple-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              All Categories
            </button>
            {Object.keys(groupedModules).map((category) => (
              <button
                key={category}
                onClick={() => setSelectedModule(category)}
                className={`px-4 py-2 rounded-md font-medium ${
                  selectedModule === category 
                    ? 'bg-purple-600 text-white' 
                    : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                }`}
              >
                {category} ({groupedModules[category]?.length || 0})
              </button>
            ))}
          </div>

          {/* Modules Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredModules.map((module) => (
              <div key={module.id} className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-purple-100 rounded-lg">
                      {module.icon}
                    </div>
                    <div>
                      <h3 className="font-semibold text-gray-900">{module.name}</h3>
                      <div className="flex items-center gap-2 mt-1">
                        {getStatusIcon(module.status)}
                        <span className={`px-2 py-1 rounded text-xs font-semibold ${getStatusColor(module.status)}`}>
                          {module.status}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
                
                <p className="text-sm text-gray-600 mb-4">
                  {module.description}
                </p>
                
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-500">Throughput:</span>
                    <span className="font-medium">{module.metrics.throughput}%</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-500">Latency:</span>
                    <span className="font-medium">{module.metrics.latency}ms</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-500">Accuracy:</span>
                    <span className="font-medium">{module.metrics.accuracy}%</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Trace Data */}
        <div className="bg-white rounded-lg shadow-xl p-6">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-semibold text-gray-800">Recent Traces</h2>
            <button
              onClick={loadTraceData}
              className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors"
            >
              <RefreshCw className="w-4 h-4" />
              Refresh
            </button>
          </div>
          
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-200">
                  <th className="text-left py-3 px-4 font-semibold text-gray-900">Timestamp</th>
                  <th className="text-left py-3 px-4 font-semibold text-gray-900">Source</th>
                  <th className="text-left py-3 px-4 font-semibold text-gray-900">Target</th>
                  <th className="text-left py-3 px-4 font-semibold text-gray-900">Action</th>
                  <th className="text-left py-3 px-4 font-semibold text-gray-900">Result</th>
                </tr>
              </thead>
              <tbody>
                {traces.map((trace) => (
                  <tr key={trace.id} className="border-b border-gray-100 hover:bg-gray-50">
                    <td className="py-3 px-4 text-sm text-gray-600">
                      {new Date(trace.timestamp).toLocaleTimeString()}
                    </td>
                    <td className="py-3 px-4 text-sm font-medium text-gray-900">
                      {trace.source}
                    </td>
                    <td className="py-3 px-4 text-sm text-gray-600">
                      {trace.target}
                    </td>
                    <td className="py-3 px-4 text-sm text-gray-600">
                      {trace.action.replace('_', ' ')}
                    </td>
                    <td className="py-3 px-4">
                      <span className={`px-2 py-1 rounded text-xs font-semibold ${getResultColor(trace.result)}`}>
                        {trace.result}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NyxTrace;
