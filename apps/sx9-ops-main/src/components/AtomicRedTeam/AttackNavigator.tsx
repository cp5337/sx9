import React, { useState } from 'react';
import { Target, AlertTriangle, Shield, Zap, Globe } from 'lucide-react';

interface AttackTechnique {
  id: string;
  name: string;
  tactic: string;
  description: string;
  atomicTests: number;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  platforms: string[];
}

const AttackNavigator: React.FC = () => {
  const [techniques] = useState<AttackTechnique[]>([
    {
      id: 'T1547.001',
      name: 'Registry Run Keys / Startup Folder',
      tactic: 'Persistence',
      description: 'Adversaries may achieve persistence by adding a program to a startup folder or referencing it with a Registry run key.',
      atomicTests: 3,
      phase: 'Hunt',
      platforms: ['windows']
    },
    {
      id: 'T1016',
      name: 'System Network Configuration Discovery',
      tactic: 'Discovery',
      description: 'Adversaries may look for details about the network configuration and settings of systems they access.',
      atomicTests: 2,
      phase: 'Detect',
      platforms: ['linux', 'macos', 'windows']
    }
  ]);

  const [selectedTactic, setSelectedTactic] = useState<string | null>(null);

  const tactics = Array.from(new Set(techniques.map(t => t.tactic)));

  const getPhaseIcon = (phase: AttackTechnique['phase']) => {
    switch (phase) {
      case 'Hunt': return <Target className="text-blue-500" size={14} />;
      case 'Detect': return <Shield className="text-green-500" size={14} />;
      case 'Disable': return <Zap className="text-yellow-500" size={14} />;
      case 'Disrupt': return <AlertTriangle className="text-red-500" size={14} />;
      case 'Dominate': return <Globe className="text-purple-500" size={14} />;
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-2 mb-4">
        {tactics.map(tactic => (
          <button
            key={tactic}
            onClick={() => setSelectedTactic(tactic === selectedTactic ? null : tactic)}
            className={`px-3 py-1 rounded-full text-xs ${
              selectedTactic === tactic
                ? 'bg-blue-500 text-white'
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
            }`}
          >
            {tactic}
          </button>
        ))}
      </div>

      <div className="grid grid-cols-1 gap-4">
        {techniques
          .filter(t => !selectedTactic || t.tactic === selectedTactic)
          .map(technique => (
            <div
              key={technique.id}
              className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md"
            >
              <div className="flex items-start justify-between mb-2">
                <div>
                  <div className="flex items-center">
                    {getPhaseIcon(technique.phase)}
                    <h3 className="ml-2 font-semibold text-sm">{technique.name}</h3>
                  </div>
                  <div className="flex items-center mt-1 space-x-2">
                    <span className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-xs">
                      {technique.id}
                    </span>
                    <span className="px-2 py-0.5 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded text-xs">
                      {technique.tactic}
                    </span>
                  </div>
                </div>
                <span className="px-2 py-0.5 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-xs">
                  {technique.atomicTests} tests
                </span>
              </div>

              <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                {technique.description}
              </p>

              <div className="flex flex-wrap gap-2">
                {technique.platforms.map(platform => (
                  <span
                    key={platform}
                    className="px-2 py-0.5 bg-gray-100 dark:bg-gray-600 rounded text-xs"
                  >
                    {platform}
                  </span>
                ))}
              </div>
            </div>
          ))}
      </div>
    </div>
  );
};

export default AttackNavigator;