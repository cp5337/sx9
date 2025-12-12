import React, { useState, useEffect } from 'react';
import { Brain, Network, Layers, Target, Zap, Globe, Activity, Database } from 'lucide-react';

interface CognitiveNode {
  id: string;
  type: string;
  properties: {
    physical: any;
    temporal: any;
    energetic: any;
    spatial: any;
    relational: any;
    economic: any;
  };
  position: { x: number; y: number };
  connections: string[];
}

interface CognigraphProps {
  className?: string;
}

const Cognigraph: React.FC<CognigraphProps> = ({ className = '' }) => {
  const [nodes, setNodes] = useState<CognitiveNode[]>([]);
  const [selectedNode, setSelectedNode] = useState<CognitiveNode | null>(null);
  const [viewMode, setViewMode] = useState<'graph' | 'table' | 'analysis'>('graph');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Simulate cognitive graph data
    const mockNodes: CognitiveNode[] = [
      {
        id: 'threat-actor-001',
        type: 'ThreatActor',
        properties: {
          physical: { mass: 1, cost: 1000 },
          temporal: { activation: '2024-01-01', duration: 3600 },
          energetic: { consumption: 50, generation: 0 },
          spatial: { radius: 100, volume: 1000 },
          relational: { connectivity: 5, dependencies: ['infrastructure-001'] },
          economic: { setup: 5000, maintenance: 100 }
        },
        position: { x: 100, y: 100 },
        connections: ['infrastructure-001', 'target-001']
      },
      {
        id: 'infrastructure-001',
        type: 'Infrastructure',
        properties: {
          physical: { mass: 10, cost: 5000 },
          temporal: { activation: '2024-01-01', duration: 86400 },
          energetic: { consumption: 200, generation: 0 },
          spatial: { radius: 500, volume: 5000 },
          relational: { connectivity: 3, dependencies: [] },
          economic: { setup: 10000, maintenance: 500 }
        },
        position: { x: 300, y: 200 },
        connections: ['threat-actor-001', 'target-001']
      },
      {
        id: 'target-001',
        type: 'Target',
        properties: {
          physical: { mass: 5, cost: 2000 },
          temporal: { activation: '2024-01-01', duration: 7200 },
          energetic: { consumption: 100, generation: 0 },
          spatial: { radius: 200, volume: 2000 },
          relational: { connectivity: 2, dependencies: ['infrastructure-001'] },
          economic: { setup: 3000, maintenance: 200 }
        },
        position: { x: 500, y: 150 },
        connections: ['threat-actor-001', 'infrastructure-001']
      }
    ];

    setNodes(mockNodes);
    setIsLoading(false);
  }, []);

  const getNodeIcon = (type: string) => {
    switch (type) {
      case 'ThreatActor': return <Target size={16} />;
      case 'Infrastructure': return <Network size={16} />;
      case 'Target': return <Globe size={16} />;
      case 'Tool': return <Zap size={16} />;
      case 'Activity': return <Activity size={16} />;
      default: return <Brain size={16} />;
    }
  };

  const getNodeColor = (type: string) => {
    switch (type) {
      case 'ThreatActor': return 'bg-red-500';
      case 'Infrastructure': return 'bg-blue-500';
      case 'Target': return 'bg-green-500';
      case 'Tool': return 'bg-yellow-500';
      case 'Activity': return 'bg-purple-500';
      default: return 'bg-gray-500';
    }
  };

  if (isLoading) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg shadow p-6 ${className}`}>
        <div className="flex items-center justify-center h-64">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow ${className}`}>
      {/* Header */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Brain size={20} className="text-blue-500" />
            <h2 className="text-lg font-semibold">Cognitive Graph</h2>
          </div>
          
          <div className="flex space-x-2">
            <button
              onClick={() => setViewMode('graph')}
              className={`px-3 py-1 rounded text-xs ${
                viewMode === 'graph' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
              }`}
            >
              <Network size={12} className="mr-1" />
              Graph
            </button>
            <button
              onClick={() => setViewMode('table')}
              className={`px-3 py-1 rounded text-xs ${
                viewMode === 'table' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
              }`}
            >
              <Layers size={12} className="mr-1" />
              Table
            </button>
            <button
              onClick={() => setViewMode('analysis')}
              className={`px-3 py-1 rounded text-xs ${
                viewMode === 'analysis' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
              }`}
            >
              <Database size={12} className="mr-1" />
              Analysis
            </button>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="p-4">
        {viewMode === 'graph' && (
          <div className="relative h-96 bg-gray-50 dark:bg-gray-900 rounded-lg overflow-hidden">
            {/* Graph Visualization */}
            <svg className="w-full h-full">
              {/* Connections */}
              {nodes.map((node) =>
                node.connections.map((connectionId) => {
                  const targetNode = nodes.find(n => n.id === connectionId);
                  if (!targetNode) return null;
                  
                  return (
                    <line
                      key={`${node.id}-${connectionId}`}
                      x1={node.position.x}
                      y1={node.position.y}
                      x2={targetNode.position.x}
                      y2={targetNode.position.y}
                      stroke="#6B7280"
                      strokeWidth="2"
                      opacity="0.5"
                    />
                  );
                })
              )}
              
              {/* Nodes */}
              {nodes.map((node) => (
                <g key={node.id}>
                  <circle
                    cx={node.position.x}
                    cy={node.position.y}
                    r="20"
                    className={`${getNodeColor(node.type)} cursor-pointer hover:opacity-80 transition-opacity`}
                    onClick={() => setSelectedNode(node)}
                  />
                  <text
                    x={node.position.x}
                    y={node.position.y + 5}
                    textAnchor="middle"
                    className="text-xs fill-white font-medium"
                  >
                    {node.type.slice(0, 3)}
                  </text>
                </g>
              ))}
            </svg>
            
            {/* Node Details Panel */}
            {selectedNode && (
              <div className="absolute top-4 right-4 bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 w-64">
                <div className="flex items-center justify-between mb-3">
                  <h3 className="font-semibold">{selectedNode.id}</h3>
                  <button
                    onClick={() => setSelectedNode(null)}
                    className="text-gray-500 hover:text-gray-700"
                  >
                    ×
                  </button>
                </div>
                
                <div className="space-y-2 text-xs">
                  <div>
                    <span className="font-medium">Type:</span> {selectedNode.type}
                  </div>
                  <div>
                    <span className="font-medium">Connections:</span> {selectedNode.connections.length}
                  </div>
                  <div>
                    <span className="font-medium">Physical Cost:</span> ${selectedNode.properties.physical.cost}
                  </div>
                  <div>
                    <span className="font-medium">Energetic Consumption:</span> {selectedNode.properties.energetic.consumption}
                  </div>
                </div>
              </div>
            )}
          </div>
        )}

        {viewMode === 'table' && (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-200 dark:border-gray-700">
                  <th className="text-left py-2">ID</th>
                  <th className="text-left py-2">Type</th>
                  <th className="text-left py-2">Connections</th>
                  <th className="text-left py-2">Cost</th>
                  <th className="text-left py-2">Consumption</th>
                </tr>
              </thead>
              <tbody>
                {nodes.map((node) => (
                  <tr key={node.id} className="border-b border-gray-100 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700">
                    <td className="py-2">{node.id}</td>
                    <td className="py-2">
                      <div className="flex items-center space-x-2">
                        {getNodeIcon(node.type)}
                        <span>{node.type}</span>
                      </div>
                    </td>
                    <td className="py-2">{node.connections.length}</td>
                    <td className="py-2">${node.properties.physical.cost}</td>
                    <td className="py-2">{node.properties.energetic.consumption}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {viewMode === 'analysis' && (
          <div className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
                <h4 className="font-semibold text-blue-800 dark:text-blue-200">Total Nodes</h4>
                <p className="text-2xl font-bold text-blue-600">{nodes.length}</p>
              </div>
              
              <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                <h4 className="font-semibold text-green-800 dark:text-green-200">Total Connections</h4>
                <p className="text-2xl font-bold text-green-600">
                  {nodes.reduce((sum, node) => sum + node.connections.length, 0)}
                </p>
              </div>
              
              <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
                <h4 className="font-semibold text-purple-800 dark:text-purple-200">Total Cost</h4>
                <p className="text-2xl font-bold text-purple-600">
                  ${nodes.reduce((sum, node) => sum + node.properties.physical.cost, 0)}
                </p>
              </div>
            </div>
            
            <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
              <h4 className="font-semibold mb-2">Node Type Distribution</h4>
              <div className="space-y-2">
                {Array.from(new Set(nodes.map(n => n.type))).map(type => (
                  <div key={type} className="flex items-center justify-between">
                    <span className="flex items-center space-x-2">
                      {getNodeIcon(type)}
                      <span>{type}</span>
                    </span>
                    <span className="font-semibold">
                      {nodes.filter(n => n.type === type).length}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Cognigraph;
