import React, { useState } from 'react';
import { Network, Server, Database, Shield, Zap, Globe, Activity } from 'lucide-react';

interface SystemComponent {
  id: string;
  name: string;
  type: 'frontend' | 'backend' | 'database' | 'service' | 'api' | 'security';
  status: 'online' | 'offline' | 'warning' | 'maintenance';
  description: string;
  connections: string[];
}

const CTASSystemDiagram: React.FC = () => {
  const [selectedComponent, setSelectedComponent] = useState<SystemComponent | null>(null);

  const systemComponents: SystemComponent[] = [
    {
      id: 'frontend',
      name: 'CTAS Frontend',
      type: 'frontend',
      status: 'online',
      description: 'React-based user interface for threat analysis and management',
      connections: ['backend', 'api']
    },
    {
      id: 'backend',
      name: 'CTAS Backend',
      type: 'backend',
      status: 'online',
      description: 'Node.js/TypeScript server handling business logic and API requests',
      connections: ['frontend', 'database', 'security', 'api']
    },
    {
      id: 'database',
      name: 'Knowledge Database',
      type: 'database',
      status: 'online',
      description: 'Neo4j graph database storing threat intelligence and relationships',
      connections: ['backend', 'api']
    },
    {
      id: 'security',
      name: 'Security Layer',
      type: 'security',
      status: 'online',
      description: 'Authentication, authorization, and threat detection services',
      connections: ['backend', 'api']
    },
    {
      id: 'api',
      name: 'External APIs',
      type: 'api',
      status: 'online',
      description: 'Integration with Shodan, OSINT tools, and threat feeds',
      connections: ['frontend', 'backend', 'database']
    },
    {
      id: 'service',
      name: 'Analytics Engine',
      type: 'service',
      status: 'online',
      description: 'AI/ML services for threat analysis and pattern recognition',
      connections: ['backend', 'database']
    }
  ];

  const getComponentIcon = (type: string) => {
    switch (type) {
      case 'frontend':
        return <Globe className="w-6 h-6 text-blue-500" />;
      case 'backend':
        return <Server className="w-6 h-6 text-green-500" />;
      case 'database':
        return <Database className="w-6 h-6 text-purple-500" />;
      case 'security':
        return <Shield className="w-6 h-6 text-red-500" />;
      case 'api':
        return <Network className="w-6 h-6 text-orange-500" />;
      case 'service':
        return <Zap className="w-6 h-6 text-yellow-500" />;
      default:
        return <Activity className="w-6 h-6 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online':
        return 'bg-green-100 text-green-800 border-green-200';
      case 'offline':
        return 'bg-red-100 text-red-800 border-red-200';
      case 'warning':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'maintenance':
        return 'bg-blue-100 text-blue-800 border-blue-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'online':
        return <div className="w-2 h-2 bg-green-500 rounded-full" />;
      case 'offline':
        return <div className="w-2 h-2 bg-red-500 rounded-full" />;
      case 'warning':
        return <div className="w-2 h-2 bg-yellow-500 rounded-full" />;
      case 'maintenance':
        return <div className="w-2 h-2 bg-blue-500 rounded-full" />;
      default:
        return <div className="w-2 h-2 bg-gray-500 rounded-full" />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Network className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">CTAS System Architecture</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-blue-100 text-blue-800 px-3 py-1 rounded text-sm font-semibold">
                System Overview
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Interactive system architecture diagram showing CTAS components and their relationships.
          </p>

          {/* System Status Summary */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <div className="w-2 h-2 bg-green-500 rounded-full" />
                <span className="text-sm font-medium text-green-800">Online</span>
              </div>
              <span className="text-2xl font-bold text-green-900">
                {systemComponents.filter(c => c.status === 'online').length}
              </span>
            </div>
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <div className="w-2 h-2 bg-red-500 rounded-full" />
                <span className="text-sm font-medium text-red-800">Offline</span>
              </div>
              <span className="text-2xl font-bold text-red-900">
                {systemComponents.filter(c => c.status === 'offline').length}
              </span>
            </div>
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <div className="w-2 h-2 bg-yellow-500 rounded-full" />
                <span className="text-sm font-medium text-yellow-800">Warning</span>
              </div>
              <span className="text-2xl font-bold text-yellow-900">
                {systemComponents.filter(c => c.status === 'warning').length}
              </span>
            </div>
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <div className="w-2 h-2 bg-blue-500 rounded-full" />
                <span className="text-sm font-medium text-blue-800">Maintenance</span>
              </div>
              <span className="text-2xl font-bold text-blue-900">
                {systemComponents.filter(c => c.status === 'maintenance').length}
              </span>
            </div>
          </div>
        </div>

        {/* System Diagram */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6">System Components</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {systemComponents.map((component) => (
              <div
                key={component.id}
                className={`border-2 rounded-lg p-4 cursor-pointer transition-all duration-300 hover:shadow-lg hover:scale-105 ${getStatusColor(component.status)}`}
                onClick={() => setSelectedComponent(component)}
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-3">
                    {getComponentIcon(component.type)}
                    <div>
                      <h3 className="font-semibold text-gray-900">{component.name}</h3>
                      <div className="flex items-center gap-2 mt-1">
                        {getStatusIcon(component.status)}
                        <span className="text-sm capitalize">{component.status}</span>
                      </div>
                    </div>
                  </div>
                </div>
                
                <p className="text-sm text-gray-600 mb-3">
                  {component.description}
                </p>
                
                <div className="text-xs text-gray-500">
                  <span className="font-medium">Connections:</span> {component.connections.length}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Component Detail Modal */}
        {selectedComponent && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    {getComponentIcon(selectedComponent.type)}
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedComponent.name}
                    </h2>
                  </div>
                  <button
                    onClick={() => setSelectedComponent(null)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                
                <div className="space-y-4">
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Type</h3>
                    <p className="text-gray-900 mt-1 capitalize">{selectedComponent.type}</p>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Status</h3>
                    <div className="flex items-center gap-2 mt-1">
                      {getStatusIcon(selectedComponent.status)}
                      <span className="capitalize">{selectedComponent.status}</span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Description</h3>
                    <p className="text-gray-900 mt-1">{selectedComponent.description}</p>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Connected Components</h3>
                    <div className="flex flex-wrap gap-2 mt-1">
                      {selectedComponent.connections.map((connection) => (
                        <span 
                          key={connection} 
                          className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm"
                        >
                          {connection}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default CTASSystemDiagram;