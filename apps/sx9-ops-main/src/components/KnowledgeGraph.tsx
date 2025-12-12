import React, { useState, useEffect } from 'react';
import { Network, Target, Shield, Zap, Globe } from 'lucide-react';
import { KnowledgeNode } from '../types';
import { getDemoData } from '../utils/demoDataProvider';

const KnowledgeGraph: React.FC = () => {
  const [nodes, setNodes] = useState<KnowledgeNode[]>([]);
  const [selectedNode, setSelectedNode] = useState<KnowledgeNode | null>(null);
  const [filter, setFilter] = useState('all');

  useEffect(() => {
    // Use centralized demo data provider
    const demoNodes = getDemoData<KnowledgeNode[]>('knowledgeNodes', []);
    setNodes(demoNodes);
  }, []);

  const getNodeIcon = (type: string) => {
    switch (type) {
      case 'threat-actor':
        return <Target className="w-4 h-4 text-red-500" />;
      case 'infrastructure':
        return <Network className="w-4 h-4 text-blue-500" />;
      case 'technique':
        return <Zap className="w-4 h-4 text-yellow-500" />;
      case 'tool':
        return <Shield className="w-4 h-4 text-green-500" />;
      default:
        return <Globe className="w-4 h-4 text-gray-500" />;
    }
  };

  const getNodeColor = (type: string) => {
    switch (type) {
      case 'threat-actor':
        return 'border-red-200 bg-red-50';
      case 'infrastructure':
        return 'border-blue-200 bg-blue-50';
      case 'technique':
        return 'border-yellow-200 bg-yellow-50';
      case 'tool':
        return 'border-green-200 bg-green-50';
      default:
        return 'border-gray-200 bg-gray-50';
    }
  };

  const filteredNodes = nodes.filter(node => {
    if (filter === 'all') return true;
    return node.type === filter;
  });

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Network className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">Knowledge Graph</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-blue-100 text-blue-800 px-3 py-1 rounded text-sm font-semibold">
                Threat Intelligence
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Interactive knowledge graph showing relationships between threat actors, infrastructure, and techniques.
          </p>

          {/* Filter Controls */}
          <div className="flex gap-4 mb-6">
            <button
              onClick={() => setFilter('all')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'all' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              All ({nodes.length})
            </button>
            <button
              onClick={() => setFilter('threat-actor')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'threat-actor' 
                  ? 'bg-red-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Threat Actors ({nodes.filter(n => n.type === 'threat-actor').length})
            </button>
            <button
              onClick={() => setFilter('infrastructure')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'infrastructure' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Infrastructure ({nodes.filter(n => n.type === 'infrastructure').length})
            </button>
            <button
              onClick={() => setFilter('technique')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'technique' 
                  ? 'bg-yellow-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Techniques ({nodes.filter(n => n.type === 'technique').length})
            </button>
          </div>
        </div>

        {/* Graph Visualization */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6">Graph Visualization</h2>
          
          {/* Simple graph representation */}
          <div className="relative h-96 bg-gray-100 rounded-lg overflow-hidden">
            {filteredNodes.map((node) => (
              <div
                key={node.id}
                className={`absolute cursor-pointer transition-all duration-300 hover:scale-110 ${getNodeColor(node.type)} border-2 rounded-lg p-3 shadow-md`}
                style={{
                  left: `${node.position.x}px`,
                  top: `${node.position.y}px`,
                  minWidth: '120px'
                }}
                onClick={() => setSelectedNode(node)}
              >
                <div className="flex items-center gap-2 mb-2">
                  {getNodeIcon(node.type)}
                  <span className="text-sm font-semibold text-gray-900">
                    {node.label}
                  </span>
                </div>
                <div className="text-xs text-gray-600">
                  {node.type.replace('-', ' ')}
                </div>
              </div>
            ))}
            
            {/* Connection lines would go here in a real graph implementation */}
            <svg className="absolute inset-0 pointer-events-none">
              {/* This is where you'd draw the connection lines */}
            </svg>
          </div>
        </div>

        {/* Node Details Modal */}
        {selectedNode && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    {getNodeIcon(selectedNode.type)}
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedNode.label}
                    </h2>
                  </div>
                  <button
                    onClick={() => setSelectedNode(null)}
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
                    <p className="text-gray-900 mt-1 capitalize">
                      {selectedNode.type.replace('-', ' ')}
                    </p>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Properties</h3>
                    <div className="mt-2 space-y-2">
                      {Object.entries(selectedNode.properties).map(([key, value]) => (
                        <div key={key} className="flex justify-between">
                          <span className="text-sm font-medium text-gray-600 capitalize">
                            {key.replace(/([A-Z])/g, ' $1').trim()}:
                          </span>
                          <span className="text-sm text-gray-900">
                            {Array.isArray(value) ? value.join(', ') : String(value)}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Position</h3>
                    <p className="text-gray-900 mt-1">
                      X: {selectedNode.position.x}, Y: {selectedNode.position.y}
                    </p>
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

export default KnowledgeGraph;
