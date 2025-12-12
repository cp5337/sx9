import React, { useState, useEffect } from 'react';
import { Network, Database, RefreshCw } from 'lucide-react';

interface GraphStats {
  nodes: number;
  relationships: number;
  labels: string[];
  relationshipTypes: string[];
}

const Neo4jKnowledgeGraph: React.FC = () => {
  const [graphStats, setGraphStats] = useState<GraphStats>({
    nodes: 0,
    relationships: 0,
    labels: [],
    relationshipTypes: [],
  });

  const [lastUpdated, setLastUpdated] = useState<string>('');

  useEffect(() => {
    fetchGraphStats();
  }, []);

  const fetchGraphStats = () => {
    // Simulating API call to Neo4j
    setTimeout(() => {
      setGraphStats({
        nodes: 1000000,
        relationships: 5000000,
        labels: ['ThreatActor', 'Vulnerability', 'Asset', 'Attack'],
        relationshipTypes: ['EXPLOITS', 'TARGETS', 'USES', 'AFFECTS'],
      });
      setLastUpdated(new Date().toLocaleString());
    }, 1000);
  };

  return (
    <div className="bg-gray-800 text-white p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Network className="mr-2" size={24} />
        Neo4j Knowledge Graph
      </h2>
      <div className="grid grid-cols-2 gap-4">
        <div className="bg-gray-700 p-3 rounded-lg">
          <h3 className="font-semibold mb-2 flex items-center">
            <Database className="mr-2" size={16} />
            Graph Statistics
          </h3>
          <p>Nodes: {graphStats.nodes.toLocaleString()}</p>
          <p>Relationships: {graphStats.relationships.toLocaleString()}</p>
        </div>
        <div className="bg-gray-700 p-3 rounded-lg">
          <h3 className="font-semibold mb-2">Labels</h3>
          <ul className="list-disc list-inside">
            {graphStats.labels.map((label, index) => (
              <li key={index}>{label}</li>
            ))}
          </ul>
        </div>
        <div className="bg-gray-700 p-3 rounded-lg">
          <h3 className="font-semibold mb-2">Relationship Types</h3>
          <ul className="list-disc list-inside">
            {graphStats.relationshipTypes.map((type, index) => (
              <li key={index}>{type}</li>
            ))}
          </ul>
        </div>
        <div className="bg-gray-700 p-3 rounded-lg flex flex-col justify-between">
          <p>Last Updated: {lastUpdated}</p>
          <button 
            onClick={fetchGraphStats}
            className="mt-2 bg-blue-500 text-white px-4 py-2 rounded flex items-center justify-center hover:bg-blue-600"
          >
            <RefreshCw className="mr-2" size={16} />
            Refresh Stats
          </button>
        </div>
      </div>
    </div>
  );
};

export default Neo4jKnowledgeGraph;