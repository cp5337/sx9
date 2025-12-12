import React, { useState } from 'react';
import { Target, Link, AlertTriangle, Shield, Zap, Globe, Database } from 'lucide-react';
import PhaseSelector from './PhaseSelector';
import TestList from './TestList';
import RecordList from './RecordList';

interface Phase {
  id: string;
  name: string;
  type: string;
}

interface RedTeamTest {
  id: string;
  name: string;
  tactic: string;
  technique: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

interface PhaseRecord {
  id: string;
  type: string;
  description: string;
  source: string;
  relatedPhases: string[];
}

const PhaseMapping: React.FC = () => {
  const [selectedPhase, setSelectedPhase] = useState<string | null>(null);

  const phases: Phase[] = [
    { id: '1', name: 'Hunt', type: 'phase' },
    { id: '2', name: 'Detect', type: 'phase' },
    { id: '3', name: 'Disable', type: 'phase' },
    { id: '4', name: 'Disrupt', type: 'phase' },
    { id: '5', name: 'Dominate', type: 'phase' }
  ];

  const tests: RedTeamTest[] = [
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

  const records: PhaseRecord[] = [
    {
      id: 'PHASE-001',
      type: 'Network Scan',
      description: 'Detected scanning activity from known threat actor',
      source: 'IDS Logs',
      relatedPhases: ['Hunt', 'Detect']
    },
    {
      id: 'PHASE-002',
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

        <PhaseSelector
          phases={phases}
          selectedPhase={selectedPhase}
          onPhaseSelect={setSelectedPhase}
          getPhaseIcon={getPhaseIcon}
        />

        {selectedPhase && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
            <div>
              <h3 className="text-xs font-semibold mb-2 flex items-center">
                <Target size={12} className="mr-2" />
                Red Team Tests
              </h3>
              <TestList tests={tests.filter(test => test.phase === selectedPhase)} />
            </div>

            <div>
              <h3 className="text-xs font-semibold mb-2 flex items-center">
                <Database size={12} className="mr-2" />
                Phase Records
              </h3>
              <RecordList records={records.filter(record => record.relatedPhases.includes(selectedPhase))} />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default PhaseMapping;