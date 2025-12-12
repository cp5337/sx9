import React, { useState } from 'react';
import { Target, Link, Shield, Zap, AlertTriangle, Globe, Database } from 'lucide-react';
// // import from ../../utils/redisGraph

interface AtomicTest {
  id: string;
  name: string;
  tactic: string;
  technique: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

interface NDEXRecord {
  id: string;
  type: string;
  description: string;
  source: string;
  relatedPhases: string[];
}

const AtomicNDEXMapping: React.FC = () => {
  const [selectedPhase, setSelectedPhase] = useState<string | null>(null);

  // Mock data since redisGraph is not available
  const nodes = [
    { id: '1', name: 'Hunt' },
    { id: '2', name: 'Detect' },
    { id: '3', name: 'Disable' },
    { id: '4', name: 'Disrupt' },
    { id: '5', name: 'Dominate' }
  ];

  const atomicTests: AtomicTest[] = [
    {
      id: 'T1046',
      name: 'Network Service Discovery',
      tactic: 'Discovery',
      technique: 'Network Service Scanning',
      description: 'Adversaries may attempt to get a listing of services running on remote hosts.',
      phase: 'Hunt'
    },
    {
      id: 'T1595',
      name: 'Active Scanning',
      tactic: 'Reconnaissance',
      technique: 'Scanning IP Blocks',
      description: 'Adversaries may scan victims for vulnerable services.',
      phase: 'Hunt'
    }
  ];

  const ndexRecords: NDEXRecord[] = [
    {
      id: 'NDEX-001',
      type: 'Network Scan',
      description: 'Detected scanning activity from known threat actor',
      source: 'IDS Logs',
      relatedPhases: ['Hunt', 'Detect']
    },
    {
      id: 'NDEX-002',
      type: 'Service Discovery',
      description: 'Attempted enumeration of network services',
      source: 'Firewall Logs',
      relatedPhases: ['Hunt', 'Detect']
    }
  ];

  const getPhaseIcon = (phase: string) => {
    switch (phase) {
      case 'Hunt': return <Target className="text-blue-500" size={14} />;
      case 'Detect': return <Shield className="text-green-500" size={14} />;
      case 'Disable': return <Zap className="text-yellow-500" size={14} />;
      case 'Disrupt': return <AlertTriangle className="text-red-500" size={14} />;
      case 'Dominate': return <Globe className="text-purple-500" size={14} />;
      default: return null;
    }
  };

  return (
    <div className="space-y-4">
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-4 flex items-center">
          <Link className="mr-2" size={14} />
          Phase Mapping
        </h2>
        <div className="flex flex-wrap gap-2 mb-4">
          {nodes.map(node => (
            <button
              key={node.id}
              onClick={() => setSelectedPhase(node.name)}
              className={`flex items-center px-3 py-1 rounded-full text-xs ${
                selectedPhase === node.name
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
              }`}
            >
              {getPhaseIcon(node.name)}
              <span className="ml-2">{node.name}</span>
            </button>
          ))}
        </div>

        {selectedPhase && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <div>
              <h3 className="text-xs font-semibold mb-2 flex items-center">
                <Target size={12} className="mr-2" />
                Atomic Red Team Tests
              </h3>
              <div className="space-y-2">
                {atomicTests
                  .filter(test => test.phase === selectedPhase)
                  .map(test => (
                    <div key={test.id} className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
                      <div className="flex items-center justify-between">
                        <span className="text-xs font-semibold">{test.name}</span>
                        <span className="text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-0.5 rounded">
                          {test.id}
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        {test.description}
                      </p>
                      <div className="flex items-center mt-1 space-x-2">
                        <span className="text-xs bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 px-2 py-0.5 rounded">
                          {test.tactic}
                        </span>
                        <span className="text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 px-2 py-0.5 rounded">
                          {test.technique}
                        </span>
                      </div>
                    </div>
                  ))}
              </div>
            </div>

            <div>
              <h3 className="text-xs font-semibold mb-2 flex items-center">
                <Database size={12} className="mr-2" />
                N-DEX Records
              </h3>
              <div className="space-y-2">
                {ndexRecords
                  .filter(record => record.relatedPhases.includes(selectedPhase))
                  .map(record => (
                    <div key={record.id} className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
                      <div className="flex items-center justify-between">
                        <span className="text-xs font-semibold">{record.type}</span>
                        <span className="text-xs bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 px-2 py-0.5 rounded">
                          {record.id}
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        {record.description}
                      </p>
                      <div className="flex items-center mt-1">
                        <span className="text-xs bg-gray-100 dark:bg-gray-600 px-2 py-0.5 rounded">
                          Source: {record.source}
                        </span>
                      </div>
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

export default AtomicNDEXMapping;