import React, { useState } from 'react';
import { Target, Zap, Shield, Globe, Activity, Database, Terminal, Settings, Layers } from 'lucide-react';

interface RaptorStack {
  id: string;
  name: string;
  description: string;
  status: 'active' | 'inactive' | 'error';
  type: 'offensive' | 'defensive' | 'intelligence' | 'infrastructure';
  components: string[];
  lastDeployed: string;
}

const Raptor: React.FC = () => {
  const [activeTab, setActiveTab] = useState('stacks');
  const [selectedStack, setSelectedStack] = useState<RaptorStack | null>(null);
  const [raptorStacks, setRaptorStacks] = React.useState<RaptorStack[]>([]);
  const [stackMetrics, setStackMetrics] = React.useState({ active: 0, total: 0, successRate: 0 });

  // Fetch real Raptor stacks from Docker/API
  React.useEffect(() => {
    const fetchStacks = async () => {
      try {
        const response = await fetch('http://localhost:18450/api/raptor/stacks');
        if (response.ok) {
          const data = await response.json();
          console.log(`✅ Raptor: Loaded ${data.stacks?.length || 0} real stacks`);
          setRaptorStacks(data.stacks || []);
          setStackMetrics({
            active: data.metrics?.active || 0,
            total: data.metrics?.total || 0,
            successRate: data.metrics?.successRate || 0
          });
        } else {
          console.warn('⚠️  Raptor: API not available');
          setRaptorStacks([]);
        }
      } catch (err) {
        console.error('❌ Raptor: Failed to fetch stacks:', err);
        setRaptorStacks([]);
      }
    };

    fetchStacks();
    const interval = setInterval(fetchStacks, 15000);
    return () => clearInterval(interval);
  }, []);

  const getTypeIcon = (type: RaptorStack['type']) => {
    switch (type) {
      case 'offensive': return <Target className="text-red-500" size={16} />;
      case 'defensive': return <Shield className="text-blue-500" size={16} />;
      case 'intelligence': return <Globe className="text-green-500" size={16} />;
      case 'infrastructure': return <Database className="text-purple-500" size={16} />;
    }
  };

  const getStatusColor = (status: RaptorStack['status']) => {
    switch (status) {
      case 'active': return 'bg-green-500';
      case 'inactive': return 'bg-gray-500';
      case 'error': return 'bg-red-500';
    }
  };

  return (
    <div className="h-full bg-gray-900 text-gray-300 p-4">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold flex items-center">
            <Target className="mr-2" size={24} />
            Raptor Security Stacks
          </h1>
          <p className="text-gray-400 text-sm">Advanced security stack management and deployment</p>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="flex space-x-1 mb-6 bg-gray-800 rounded-lg p-1">
        {[
          { id: 'stacks', name: 'Stacks', icon: <Layers size={16} /> },
          { id: 'deploy', name: 'Deploy', icon: <Zap size={16} /> },
          { id: 'monitor', name: 'Monitor', icon: <Activity size={16} /> },
          { id: 'config', name: 'Config', icon: <Settings size={16} /> }
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`flex items-center px-4 py-2 rounded-md text-sm font-medium transition-colors ${
              activeTab === tab.id
                ? 'bg-blue-600 text-white'
                : 'text-gray-400 hover:text-white hover:bg-gray-700'
            }`}
          >
            {tab.icon}
            <span className="ml-2">{tab.name}</span>
          </button>
        ))}
      </div>

      {/* Content */}
      {activeTab === 'stacks' && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {raptorStacks.map(stack => (
            <div
              key={stack.id}
              className="bg-gray-800 rounded-lg p-4 hover:bg-gray-700 transition-colors cursor-pointer"
              onClick={() => setSelectedStack(stack)}
            >
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center">
                  {getTypeIcon(stack.type)}
                  <h3 className="ml-2 font-semibold">{stack.name}</h3>
                </div>
                <div className={`w-3 h-3 rounded-full ${getStatusColor(stack.status)}`} />
              </div>
              
              <p className="text-sm text-gray-400 mb-3">{stack.description}</p>
              
              <div className="mb-3">
                <div className="text-xs text-gray-500 mb-1">Components:</div>
                <div className="flex flex-wrap gap-1">
                  {stack.components.slice(0, 3).map(component => (
                    <span key={component} className="px-2 py-1 bg-gray-700 rounded text-xs">
                      {component}
                    </span>
                  ))}
                  {stack.components.length > 3 && (
                    <span className="px-2 py-1 bg-gray-700 rounded text-xs">
                      +{stack.components.length - 3} more
                    </span>
                  )}
                </div>
              </div>
              
              <div className="text-xs text-gray-500">
                Last deployed: {stack.lastDeployed}
              </div>
            </div>
          ))}
        </div>
      )}

      {activeTab === 'deploy' && (
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4">Deploy New Stack</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-2">Stack Type</label>
              <select className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm">
                <option>Offensive Security</option>
                <option>Defensive Security</option>
                <option>Intelligence Gathering</option>
                <option>Infrastructure Monitoring</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Target Environment</label>
              <input
                type="text"
                placeholder="Enter target environment"
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm"
              />
            </div>
            <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-sm font-medium">
              Deploy Stack
            </button>
          </div>
        </div>
      )}

      {activeTab === 'monitor' && (
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4">Stack Monitoring</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-gray-700 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium">Active Stacks</span>
                <span className="text-2xl font-bold text-green-400">{stackMetrics.active}</span>
              </div>
            </div>
            <div className="bg-gray-700 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium">Total Deployments</span>
                <span className="text-2xl font-bold text-blue-400">{stackMetrics.total}</span>
              </div>
            </div>
            <div className="bg-gray-700 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium">Success Rate</span>
                <span className="text-2xl font-bold text-green-400">{stackMetrics.successRate}%</span>
              </div>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'config' && (
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-semibold mb-4">Configuration</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-2">Default Deployment Environment</label>
              <input
                type="text"
                defaultValue="production"
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Auto-deploy on Changes</label>
              <input type="checkbox" className="mr-2" />
              <span className="text-sm">Enable automatic deployment</span>
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Notification Settings</label>
              <select className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm">
                <option>Email notifications</option>
                <option>Slack notifications</option>
                <option>Webhook notifications</option>
              </select>
            </div>
          </div>
        </div>
      )}

      {/* Stack Details Modal */}
      {selectedStack && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-xl font-semibold">{selectedStack.name}</h3>
              <button
                onClick={() => setSelectedStack(null)}
                className="text-gray-400 hover:text-white"
              >
                ✕
              </button>
            </div>
            <p className="text-gray-400 mb-4">{selectedStack.description}</p>
            <div className="space-y-4">
              <div>
                <h4 className="font-medium mb-2">Components</h4>
                <div className="grid grid-cols-2 gap-2">
                  {selectedStack.components.map(component => (
                    <div key={component} className="bg-gray-700 px-3 py-2 rounded text-sm">
                      {component}
                    </div>
                  ))}
                </div>
              </div>
              <div className="flex space-x-2">
                <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-sm">
                  Deploy
                </button>
                <button className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded text-sm">
                  Edit
                </button>
                <button className="px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm">
                  Delete
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Raptor;
