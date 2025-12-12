import React, { useState, useEffect, useCallback } from 'react';
import { Network, Target, Globe, Database, Terminal, Activity, Settings, Filter, Search } from 'lucide-react';
import ForceGraph2D from 'react-force-graph-2d';

interface GraphNode {
  id: string;
  name: string;
  type: 'raptor' | 'vkali' | 'database' | 'service' | 'threat' | 'asset' | 'user' | 'network';
  group: number;
  size: number;
  color: string;
  status: 'active' | 'inactive' | 'error' | 'warning';
  metrics?: {
    cpu: number;
    memory: number;
    network: number;
  };
}

interface GraphLink {
  source: string;
  target: string;
  type: 'data' | 'control' | 'threat' | 'dependency';
  strength: number;
  color: string;
}

interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

interface GraphVisualizerProps {
  className?: string;
  data?: GraphData;
}

const GraphVisualizer: React.FC<GraphVisualizerProps> = ({ className = '', data }) => {
  const [graphData, setGraphData] = useState<GraphData>({ nodes: [], links: [] });
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [filterType, setFilterType] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState('');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (data) {
      setGraphData(data);
      setIsLoading(false);
    } else {
      // Generate sample graph data
      const sampleData: GraphData = {
        nodes: [
          {
            id: 'raptor-001',
            name: 'RAPTOR Stack Alpha',
            type: 'raptor',
            group: 1,
            size: 20,
            color: '#8b5cf6',
            status: 'active',
            metrics: { cpu: 45, memory: 67, network: 23 }
          },
          {
            id: 'vkali-001',
            name: 'vKali Environment',
            type: 'vkali',
            group: 2,
            size: 18,
            color: '#f97316',
            status: 'active',
            metrics: { cpu: 78, memory: 45, network: 89 }
          },
          {
            id: 'mongodb-001',
            name: 'MongoDB Cluster',
            type: 'database',
            group: 3,
            size: 16,
            color: '#06b6d4',
            status: 'active',
            metrics: { cpu: 23, memory: 34, network: 12 }
          },
          {
            id: 'neo4j-001',
            name: 'Neo4j Graph DB',
            type: 'database',
            group: 3,
            size: 16,
            color: '#06b6d4',
            status: 'active',
            metrics: { cpu: 56, memory: 78, network: 34 }
          },
          {
            id: 'threat-actor-001',
            name: 'APT Group Alpha',
            type: 'threat',
            group: 4,
            size: 14,
            color: '#ef4444',
            status: 'active'
          },
          {
            id: 'network-001',
            name: 'Core Network',
            type: 'network',
            group: 5,
            size: 22,
            color: '#3b82f6',
            status: 'active'
          },
          {
            id: 'user-001',
            name: 'Admin User',
            type: 'user',
            group: 6,
            size: 12,
            color: '#10b981',
            status: 'active'
          }
        ],
        links: [
          { source: 'raptor-001', target: 'mongodb-001', type: 'data', strength: 0.8, color: '#3b82f6' },
          { source: 'raptor-001', target: 'neo4j-001', type: 'data', strength: 0.6, color: '#3b82f6' },
          { source: 'vkali-001', target: 'network-001', type: 'control', strength: 0.9, color: '#f59e0b' },
          { source: 'threat-actor-001', target: 'network-001', type: 'threat', strength: 0.7, color: '#ef4444' },
          { source: 'user-001', target: 'raptor-001', type: 'control', strength: 0.5, color: '#10b981' },
          { source: 'user-001', target: 'vkali-001', type: 'control', strength: 0.5, color: '#10b981' },
          { source: 'mongodb-001', target: 'neo4j-001', type: 'data', strength: 0.4, color: '#3b82f6' }
        ]
      };
      setGraphData(sampleData);
      setIsLoading(false);
    }
  }, [data]);

  const handleNodeClick = useCallback((node: GraphNode) => {
    setSelectedNode(node);
  }, []);

  const filteredData = React.useMemo(() => {
    let filtered = graphData;
    
    if (filterType !== 'all') {
      filtered = {
        nodes: graphData.nodes.filter(node => node.type === filterType),
        links: graphData.links.filter(link => {
          const sourceNode = graphData.nodes.find(n => n.id === link.source);
          const targetNode = graphData.nodes.find(n => n.id === link.target);
          return sourceNode?.type === filterType || targetNode?.type === filterType;
        })
      };
    }
    
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = {
        nodes: filtered.nodes.filter(node => 
          node.name.toLowerCase().includes(query) || 
          node.id.toLowerCase().includes(query)
        ),
        links: filtered.links.filter(link => {
          const sourceNode = filtered.nodes.find(n => n.id === link.source);
          const targetNode = filtered.nodes.find(n => n.id === link.target);
          return sourceNode && targetNode;
        })
      };
    }
    
    return filtered;
  }, [graphData, filterType, searchQuery]);

  const getNodeTypes = () => {
    const types = Array.from(new Set(graphData.nodes.map(n => n.type)));
    return ['all', ...types];
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'text-green-500';
      case 'inactive': return 'text-gray-500';
      case 'error': return 'text-red-500';
      case 'warning': return 'text-yellow-500';
      default: return 'text-gray-500';
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
            <Network size={20} className="text-blue-500" />
            <h2 className="text-lg font-semibold">Network Graph</h2>
          </div>
          
          <div className="flex items-center space-x-4">
            {/* Search */}
            <div className="relative">
              <Search size={16} className="absolute left-2 top-1/2 transform -translate-y-1/2 text-gray-400" />
              <input
                type="text"
                placeholder="Search nodes..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-8 pr-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            
            {/* Filter */}
            <select
              value={filterType}
              onChange={(e) => setFilterType(e.target.value)}
              className="px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {getNodeTypes().map(type => (
                <option key={type} value={type}>
                  {type.charAt(0).toUpperCase() + type.slice(1)}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Graph Container */}
      <div className="relative h-96">
        <ForceGraph2D
          graphData={filteredData}
          nodeLabel="name"
          nodeColor="color"
          nodeVal="size"
          linkColor="color"
          linkWidth={2}
          linkDirectionalParticles={2}
          linkDirectionalParticleSpeed={0.005}
          onNodeClick={handleNodeClick}
          cooldownTicks={100}
          nodeCanvasObject={(node: any, ctx, globalScale) => {
            const label = node.name || 'Unknown';
            const fontSize = 12/globalScale;
            ctx.font = `${fontSize}px Sans-Serif`;
            const textWidth = ctx.measureText(label).width;
            const bckgDimensions = [textWidth, fontSize].map(n => n + fontSize * 0.2);
            const width = bckgDimensions[0] || 0;
            const height = bckgDimensions[1] || 0;

            ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
            ctx.fillRect(
              (node.x || 0) - width / 2, 
              (node.y || 0) - height / 2, 
              width, 
              height
            );

            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillStyle = node.color || '#000000';
            ctx.fillText(label, node.x || 0, node.y || 0);

            node.__bckgDimensions = bckgDimensions;
          }}
        />
        
        {/* Node Details Panel */}
        {selectedNode && (
          <div className="absolute top-4 right-4 bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 w-64 border border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold text-sm">{selectedNode.name}</h3>
              <button
                onClick={() => setSelectedNode(null)}
                className="text-gray-500 hover:text-gray-700 text-lg"
              >
                ×
              </button>
            </div>
            
            <div className="space-y-2 text-xs">
              <div className="flex items-center space-x-2">
                <span className="font-medium">Type:</span>
                <span className="capitalize">{selectedNode.type}</span>
              </div>
              
              <div className="flex items-center space-x-2">
                <span className="font-medium">Status:</span>
                <span className={`${getStatusColor(selectedNode.status)}`}>
                  {selectedNode.status}
                </span>
              </div>
              
              <div className="flex items-center space-x-2">
                <span className="font-medium">Group:</span>
                <span>{selectedNode.group}</span>
              </div>
              
              {selectedNode.metrics && (
                <div className="mt-3 pt-2 border-t border-gray-200 dark:border-gray-700">
                  <div className="font-medium mb-1">Metrics:</div>
                  <div className="space-y-1">
                    <div className="flex justify-between">
                      <span>CPU:</span>
                      <span>{selectedNode.metrics.cpu}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Memory:</span>
                      <span>{selectedNode.metrics.memory}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Network:</span>
                      <span>{selectedNode.metrics.network}%</span>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Legend */}
      <div className="p-4 border-t border-gray-200 dark:border-gray-700">
        <div className="flex flex-wrap gap-4 text-xs">
          {Array.from(new Set(graphData.nodes.map(n => n.type))).map(type => (
            <div key={type} className="flex items-center space-x-1">
              <div 
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: graphData.nodes.find(n => n.type === type)?.color }}
              ></div>
              <span className="capitalize">{type}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default GraphVisualizer;
