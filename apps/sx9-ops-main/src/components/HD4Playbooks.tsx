import React, { useState } from 'react';
import { Book, Check, AlertTriangle, Play, Shield, Clock } from 'lucide-react';

interface PlaybookStep {
  id: string;
  name: string;
  description: string;
  automatable: boolean;
  requiresApproval: boolean;
  estimatedTime: string;
  status: 'pending' | 'in_progress' | 'completed';
}

interface Playbook {
  id: string;
  name: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  difficulty: 'junior' | 'intermediate' | 'senior';
  automationLevel: number;
  steps: PlaybookStep[];
  status: 'available' | 'in_progress' | 'completed';
  lastRun?: string;
}

interface HD4PlaybooksProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const HD4Playbooks: React.FC<HD4PlaybooksProps> = ({ hd4Action }) => {
  const [selectedPlaybook, setSelectedPlaybook] = useState<string | null>(null);

  const playbooks: Playbook[] = [
    {
      id: 'pb1',
      name: `${hd4Action} Infrastructure Analysis`,
      description: `Automated ${hd4Action.toLowerCase()} phase analysis of target infrastructure`,
      phase: hd4Action,
      difficulty: 'senior',
      automationLevel: 80,
      status: 'available',
      steps: [
        {
          id: 's1',
          name: 'Initial Reconnaissance',
          description: 'Automated scanning of target infrastructure',
          automatable: true,
          requiresApproval: false,
          estimatedTime: '30m',
          status: 'pending'
        },
        {
          id: 's2',
          name: 'Threat Assessment',
          description: 'AI-driven analysis of potential threats',
          automatable: true,
          requiresApproval: true,
          estimatedTime: '1h',
          status: 'pending'
        }
      ]
    },
    {
      id: 'pb2',
      name: `${hd4Action} Network Defense`,
      description: `Guided ${hd4Action.toLowerCase()} phase for network protection`,
      phase: hd4Action,
      difficulty: 'junior',
      automationLevel: 40,
      status: 'in_progress',
      lastRun: '2023-06-15T10:30:00Z',
      steps: [
        {
          id: 's1',
          name: 'System Inventory',
          description: 'Guided scanning of systems',
          automatable: false,
          requiresApproval: true,
          estimatedTime: '45m',
          status: 'in_progress'
        }
      ]
    }
  ];

  const getDifficultyColor = (difficulty: string) => {
    switch (difficulty) {
      case 'junior': return 'bg-green-500';
      case 'intermediate': return 'bg-yellow-500';
      case 'senior': return 'bg-red-500';
      default: return 'bg-gray-500';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed': return <Check className="w-3 h-3 text-green-500" />;
      case 'in_progress': return <Clock className="w-3 h-3 text-yellow-500" />;
      default: return <AlertTriangle className="w-3 h-3 text-gray-500" />;
    }
  };

  return (
    <div className="bg-gray-800 p-4 rounded-lg">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-sm font-semibold flex items-center">
          <Book className="w-4 h-4 mr-2" />
          {hd4Action} Playbooks
        </h2>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {playbooks.map(playbook => (
          <div 
            key={playbook.id} 
            className={`bg-gray-700 p-3 rounded-lg cursor-pointer transition-all duration-200 ${
              selectedPlaybook === playbook.id ? 'ring-2 ring-blue-500' : ''
            }`}
            onClick={() => setSelectedPlaybook(playbook.id)}
          >
            <div className="flex justify-between items-center mb-2">
              <h3 className="font-semibold text-sm">{playbook.name}</h3>
              <div className="flex items-center space-x-2">
                <span className={`text-xs px-1.5 py-0.5 rounded ${getDifficultyColor(playbook.difficulty)}`}>
                  {playbook.difficulty}
                </span>
                <span className="text-xs bg-blue-500 px-1.5 py-0.5 rounded">
                  {playbook.automationLevel}% Auto
                </span>
              </div>
            </div>

            <p className="text-xs text-gray-400 mb-2">{playbook.description}</p>

            {selectedPlaybook === playbook.id && (
              <div className="mt-3 border-t border-gray-600 pt-2">
                <h4 className="text-xs font-semibold mb-2">Steps:</h4>
                <div className="space-y-2">
                  {playbook.steps.map(step => (
                    <div key={step.id} className="bg-gray-800 p-2 rounded">
                      <div className="flex items-center justify-between mb-1">
                        <div className="flex items-center">
                          <Shield className="w-3 h-3 mr-1 text-blue-500" />
                          <span className="text-xs font-medium">{step.name}</span>
                        </div>
                        <div className="flex items-center space-x-2">
                          {step.requiresApproval && (
                            <AlertTriangle className="w-3 h-3 text-yellow-500" />
                          )}
                          <span className="text-xs text-gray-400">{step.estimatedTime}</span>
                          {getStatusIcon(step.status)}
                        </div>
                      </div>
                      <p className="text-xs text-gray-400">{step.description}</p>
                    </div>
                  ))}
                </div>

                <div className="mt-3 flex justify-end">
                  <button className="bg-blue-500 text-white px-2 py-1 rounded text-xs flex items-center">
                    <Play className="w-3 h-3 mr-1" />
                    Run Playbook
                  </button>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default HD4Playbooks;