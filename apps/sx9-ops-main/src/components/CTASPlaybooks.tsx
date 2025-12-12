import React, { useState } from 'react';
import { Book, Target, Play, Trash2, Plus, Shield, Zap, Globe, Eye } from 'lucide-react';

interface PlaybookTask {
  id: string;
  name: string;
  description: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  automatable: boolean;
  requiresApproval: boolean;
}

interface CTASPlaybook {
  id: string;
  name: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  status: 'ready' | 'running' | 'completed' | 'failed';
  inventory: string;
  tasks: PlaybookTask[];
}

const CTASPlaybooks: React.FC = () => {
  const [playbooks, setPlaybooks] = useState<CTASPlaybook[]>([
    // Hunt Phase Playbooks
    {
      id: '1',
      name: 'Network Reconnaissance',
      description: 'Automated network discovery and mapping',
      phase: 'Hunt',
      status: 'ready',
      inventory: 'hunt_nodes',
      tasks: [
        { id: '1-1', name: 'Network scan', description: 'Perform network scan', status: 'pending', automatable: true, requiresApproval: false },
        { id: '1-2', name: 'Port discovery', description: 'Identify open ports', status: 'pending', automatable: true, requiresApproval: false },
        { id: '1-3', name: 'Service enumeration', description: 'Identify running services', status: 'pending', automatable: true, requiresApproval: false },
        { id: '1-4', name: 'Vulnerability scan', description: 'Scan for known vulnerabilities', status: 'pending', automatable: true, requiresApproval: true }
      ]
    },
    // ... (previous playbooks remain the same)
  ]);

  const [selectedPlaybook, setSelectedPlaybook] = useState<string | null>(null);

  const handleRunPlaybook = (id: string) => {
    setPlaybooks(prevPlaybooks =>
      prevPlaybooks.map(playbook =>
        playbook.id === id ? { ...playbook, status: 'running' } : playbook
      )
    );

    setTimeout(() => {
      setPlaybooks(prevPlaybooks =>
        prevPlaybooks.map(playbook =>
          playbook.id === id ? {
            ...playbook,
            status: 'completed',
            tasks: playbook.tasks.map(task => ({ ...task, status: 'completed' }))
          } : playbook
        )
      );
    }, 3000);
  };

  const handleRemovePlaybook = (id: string) => {
    setPlaybooks(prevPlaybooks => prevPlaybooks.filter(playbook => playbook.id !== id));
  };

  const getPhaseIcon = (phase: CTASPlaybook['phase']) => {
    switch (phase) {
      case 'Hunt': return <Target className="text-blue-500" size={16} />;
      case 'Detect': return <Shield className="text-green-500" size={16} />;
      case 'Disable': return <Zap className="text-yellow-500" size={16} />;
      case 'Disrupt': return <Shield className="text-red-500" size={16} />;
      case 'Dominate': return <Globe className="text-purple-500" size={16} />;
    }
  };

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {playbooks.map(playbook => (
          <div
            key={playbook.id}
            className={`bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md ${
              selectedPlaybook === playbook.id ? 'ring-2 ring-blue-500' : ''
            }`}
            onClick={() => setSelectedPlaybook(playbook.id)}
          >
            <div className="flex justify-between items-start mb-2">
              <div>
                <div className="flex items-center">
                  {getPhaseIcon(playbook.phase)}
                  <h3 className="ml-2 font-semibold text-sm">{playbook.name}</h3>
                </div>
                <div className="flex items-center mt-1 space-x-2">
                  <span className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-xs">
                    {playbook.phase}
                  </span>
                  <span className="px-2 py-0.5 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded text-xs">
                    {playbook.inventory}
                  </span>
                </div>
              </div>
              <div className="flex space-x-2">
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRunPlaybook(playbook.id);
                  }}
                  disabled={playbook.status === 'running'}
                  className="p-1 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50"
                >
                  <Play size={12} />
                </button>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRemovePlaybook(playbook.id);
                  }}
                  className="p-1 rounded bg-red-500 text-white hover:bg-red-600"
                >
                  <Trash2 size={12} />
                </button>
              </div>
            </div>

            <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
              {playbook.description}
            </p>

            {selectedPlaybook === playbook.id && (
              <div className="mt-4 space-y-2">
                <h4 className="text-xs font-semibold mb-2">Tasks:</h4>
                {playbook.tasks.map(task => (
                  <div key={task.id} className="bg-gray-100 dark:bg-gray-700 p-2 rounded">
                    <div className="flex items-center justify-between">
                      <span className="text-xs font-medium">{task.name}</span>
                      <div className="flex items-center space-x-2">
                        {task.requiresApproval && (
                          <Shield size={12} className="text-yellow-500" />
                        )}
                        <Eye size={12} className="text-blue-500" />
                      </div>
                    </div>
                    <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                      {task.description}
                    </p>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-2 flex items-center">
          <Book size={14} className="mr-2" />
          Add New Playbook
        </h2>
        <button className="bg-blue-500 text-white px-2 py-1 rounded text-xs flex items-center">
          <Plus size={12} className="mr-1" />
          Create Playbook
        </button>
      </div>
    </div>
  );
};

export default CTASPlaybooks;