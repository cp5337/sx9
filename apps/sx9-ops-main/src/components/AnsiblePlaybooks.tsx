import React, { useState } from 'react';
import { Book, CheckCircle, ChevronDown, ChevronRight, Target, Shield, Zap, Globe, Terminal, AlertTriangle, Play, Trash2, Plus } from 'lucide-react';

interface AnsibleTask {
  id: string;
  name: string;
  description: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

interface AnsiblePlaybook {
  id: string;
  name: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  status: 'ready' | 'running' | 'completed' | 'failed';
  inventory: string;
  tasks: AnsibleTask[];
  lastRun?: string;
  output?: string;
  nodeId?: string;
}

const AnsiblePlaybooks: React.FC = () => {
  const [playbooks, setPlaybooks] = useState<AnsiblePlaybook[]>([
    {
      id: '1',
      name: 'Network Reconnaissance',
      description: 'Automated network discovery and mapping',
      phase: 'Hunt',
      status: 'ready',
      inventory: 'hunt_nodes',
      tasks: [
        { id: '1-1', name: 'Network scan', description: 'Perform network scan', status: 'pending' },
        { id: '1-2', name: 'Port discovery', description: 'Identify open ports', status: 'pending' }
      ]
    },
    {
      id: '2',
      name: 'Threat Detection Setup',
      description: 'Deploy threat detection systems',
      phase: 'Detect',
      status: 'completed',
      inventory: 'detection_nodes',
      tasks: [
        { id: '2-1', name: 'IDS deployment', description: 'Deploy intrusion detection', status: 'completed' },
        { id: '2-2', name: 'Log aggregation', description: 'Setup log collection', status: 'completed' }
      ],
      lastRun: '2023-06-15 10:30:00'
    },
    {
      id: '3',
      name: 'System Hardening',
      description: 'Security hardening procedures',
      phase: 'Disable',
      status: 'ready',
      inventory: 'target_systems',
      tasks: [
        { id: '3-1', name: 'Firewall rules', description: 'Configure firewall', status: 'pending' },
        { id: '3-2', name: 'Service lockdown', description: 'Disable unnecessary services', status: 'pending' }
      ]
    }
  ]);

  const [expandedPhases, setExpandedPhases] = useState<string[]>(['Hunt']);
  // const [selectedPlaybook, setSelectedPlaybook] = useState<string | null>(null);

  const [newPlaybook, setNewPlaybook] = useState<Omit<AnsiblePlaybook, 'id' | 'status' | 'tasks'>>({
    name: '',
    description: '',
    phase: 'Hunt',
    inventory: '',
  });

  // // const [newTask, setNewTask] = useState(...);

  const handleAddPlaybook = () => {
    if (newPlaybook.name && newPlaybook.inventory) {
      setPlaybooks([...playbooks, {
        ...newPlaybook,
        id: crypto.randomUUID(),
        status: 'ready',
        tasks: []
      }]);
      setNewPlaybook({
        name: '',
        description: '',
        phase: 'Hunt',
        inventory: '',
      });
    }
  };

  // const handleAddTask = (playbookId: string) => {
  //   if (newTask.name) {
  //     setPlaybooks(playbooks.map(pb => 
  //       pb.id === playbookId ? {
  //         ...pb,
  //         tasks: [...pb.tasks, {
  //           id: crypto.randomUUID(),
  //           ...newTask,
  //           status: 'pending'
  //         }]
  //       } : pb
  //     ));
  //     setNewTask({ name: '', description: '' });
  //   }
  // };

  const handleRemovePlaybook = (id: string) => {
    setPlaybooks(playbooks.filter(pb => pb.id !== id));
  };

  const handleRunPlaybook = (id: string) => {
    setPlaybooks(playbooks.map(pb =>
      pb.id === id ? { ...pb, status: 'running' } : pb
    ));

    // Simulate playbook execution
    setTimeout(() => {
      setPlaybooks(playbooks.map(pb =>
        pb.id === id ? {
          ...pb,
          status: 'completed',
          lastRun: new Date().toISOString().slice(0, 19).replace('T', ' '),
          tasks: pb.tasks.map(task => ({ ...task, status: 'completed' }))
        } : pb
      ));
    }, 3000);
  };

  const togglePhase = (phase: string) => {
    setExpandedPhases(prev =>
      prev.includes(phase) ? prev.filter(p => p !== phase) : [...prev, phase]
    );
  };

  const getPhaseIcon = (phase: AnsiblePlaybook['phase']) => {
    switch (phase) {
      case 'Hunt': return <Target className="text-blue-500" size={16} />;
      case 'Detect': return <Shield className="text-green-500" size={16} />;
      case 'Disable': return <Zap className="text-yellow-500" size={16} />;
      case 'Disrupt': return <Terminal className="text-red-500" size={16} />;
      case 'Dominate': return <Globe className="text-purple-500" size={16} />;
    }
  };

  const getStatusIcon = (status: AnsiblePlaybook['status'] | AnsibleTask['status']) => {
    switch (status) {
      case 'ready':
      case 'pending':
        return <Book className="text-blue-500" size={16} />;
      case 'running':
        return <Terminal className="text-yellow-500" size={16} />;
      case 'completed':
        return <CheckCircle className="text-green-500" size={16} />;
      case 'failed':
        return <AlertTriangle className="text-red-500" size={16} />;
    }
  };

  const phases: AnsiblePlaybook['phase'][] = ['Hunt', 'Detect', 'Disable', 'Disrupt', 'Dominate'];

  return (
    <div className="space-y-4">
      {/* Phase-based Playbook List */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md">
        {phases.map(phase => (
          <div key={phase} className="border-b border-gray-200 dark:border-gray-700 last:border-0">
            <button
              onClick={() => togglePhase(phase)}
              className="w-full px-4 py-2 flex items-center justify-between hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <div className="flex items-center">
                {getPhaseIcon(phase)}
                <span className="ml-2 font-semibold">{phase} Phase</span>
                <span className="ml-2 text-xs text-gray-500">
                  ({playbooks.filter(pb => pb.phase === phase).length} playbooks)
                </span>
              </div>
              {expandedPhases.includes(phase) ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
            </button>
            
            {expandedPhases.includes(phase) && (
              <div className="p-4 space-y-4">
                {playbooks
                  .filter(pb => pb.phase === phase)
                  .map(playbook => (
                    <div
                      key={playbook.id}
                      className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4"
                    >
                      <div className="flex justify-between items-center mb-2">
                        <div className="flex items-center">
                          {getStatusIcon(playbook.status)}
                          <h3 className="ml-2 font-semibold">{playbook.name}</h3>
                        </div>
                        <div className="flex space-x-2">
                          <button
                            onClick={() => handleRunPlaybook(playbook.id)}
                            disabled={playbook.status === 'running'}
                            className="p-1 rounded bg-blue-500 text-white disabled:opacity-50"
                          >
                            <Play size={12} />
                          </button>
                          <button
                            onClick={() => handleRemovePlaybook(playbook.id)}
                            className="p-1 rounded bg-red-500 text-white"
                          >
                            <Trash2 size={12} />
                          </button>
                        </div>
                      </div>
                      
                      <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                        {playbook.description}
                      </p>
                      
                      <div className="text-xs">
                        <p><span className="font-semibold">Inventory:</span> {playbook.inventory}</p>
                        {playbook.lastRun && (
                          <p><span className="font-semibold">Last Run:</span> {playbook.lastRun}</p>
                        )}
                      </div>

                      <div className="mt-2">
                        <h4 className="text-xs font-semibold mb-1">Tasks:</h4>
                        <ul className="text-xs space-y-1">
                          {playbook.tasks.map((task) => (
                            <li key={task.id} className="flex items-center justify-between bg-gray-100 dark:bg-gray-600 p-2 rounded">
                              <div className="flex items-center">
                                {getStatusIcon(task.status)}
                                <span className="ml-2">{task.name}</span>
                              </div>
                              <span className="text-gray-500 dark:text-gray-400">{task.description}</span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    </div>
                  ))}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Add New Playbook Form */}
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-4">Add New Playbook</h2>
        <div className="grid grid-cols-2 gap-4 mb-4">
          <input
            type="text"
            placeholder="Playbook Name"
            value={newPlaybook.name}
            onChange={(e) => setNewPlaybook({ ...newPlaybook, name: e.target.value })}
            className="p-2 border rounded text-sm"
          />
          <select
            value={newPlaybook.phase}
            onChange={(e) => setNewPlaybook({ ...newPlaybook, phase: e.target.value as AnsiblePlaybook['phase'] })}
            className="p-2 border rounded text-sm"
          >
            {phases.map(phase => (
              <option key={phase} value={phase}>{phase}</option>
            ))}
          </select>
          <input
            type="text"
            placeholder="Inventory"
            value={newPlaybook.inventory}
            onChange={(e) => setNewPlaybook({ ...newPlaybook, inventory: e.target.value })}
            className="p-2 border rounded text-sm"
          />
          <textarea
            placeholder="Description"
            value={newPlaybook.description}
            onChange={(e) => setNewPlaybook({ ...newPlaybook, description: e.target.value })}
            className="p-2 border rounded text-sm col-span-2"
            rows={2}
          />
        </div>

        <button
          onClick={handleAddPlaybook}
          className="w-full bg-blue-500 text-white px-4 py-2 rounded text-sm flex items-center justify-center"
        >
          <Plus size={16} className="mr-2" />
          Create Playbook
        </button>
      </div>
    </div>
  );
};

export default AnsiblePlaybooks;