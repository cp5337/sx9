import React, { useState } from 'react';
import { CheckCircle, Target, Info, Shield, Play } from 'lucide-react';

interface AtomicTest {
  id: string;
  name: string;
  description: string;
  technique: string;
  tactic: string;
  platform: string[];
  executor: string;
  command: string;
  status: 'ready' | 'running' | 'completed' | 'failed';
  cleanupCommand?: string;
}

interface AtomicTestRunnerProps {
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  nodeId?: string;
}

const AtomicTestRunner: React.FC<AtomicTestRunnerProps> = () => {
  const [tests, setTests] = useState<AtomicTest[]>([
    {
      id: '1',
      name: 'Windows Registry Run Keys / Startup Folder',
      description: 'Establish persistence via Windows Registry Run Keys',
      technique: 'T1547.001',
      tactic: 'Persistence',
      platform: ['windows'],
      executor: 'command_prompt',
      command: 'REG ADD "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run" /V "Atomic Red Team" /t REG_SZ /F /D "C:\\Path\\AtomicRedTeam.exe"',
      status: 'ready',
      cleanupCommand: 'REG DELETE "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run" /V "Atomic Red Team" /f'
    },
    {
      id: '2',
      name: 'System Network Configuration Discovery',
      description: 'Identify network configuration using built-in commands',
      technique: 'T1016',
      tactic: 'Discovery',
      platform: ['linux', 'macos', 'windows'],
      executor: 'sh',
      command: 'ifconfig || ip addr',
      status: 'ready'
    }
  ]);

  // // const [selectedTest, setSelectedTest] = useState(...);

  const handleRunTest = (testId: string) => {
    setTests(tests.map(test =>
      test.id === testId ? { ...test, status: 'running' } : test
    ));

    setTimeout(() => {
      setTests(tests.map(test =>
        test.id === testId ? { ...test, status: 'completed' } : test
      ));
    }, 3000);
  };

  const getStatusIcon = (status: AtomicTest['status']) => {
    switch (status) {
      case 'ready':
        return <Target className="text-blue-500" size={16} />;
      case 'running':
        return <Shield className="text-yellow-500 animate-pulse" size={16} />;
      case 'completed':
        return <CheckCircle className="text-green-500" size={16} />;
      case 'failed':
        return <Target className="text-red-500" size={16} />;
    }
  };

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-1 gap-4">
        {tests.map(test => (
          <div
            key={test.id}
            className={`bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md ${
              ''
            }`}
          >
            <div className="flex justify-between items-start mb-2">
              <div>
                <div className="flex items-center">
                  {getStatusIcon(test.status)}
                  <h3 className="ml-2 font-semibold text-sm">{test.name}</h3>
                </div>
                <div className="flex items-center mt-1 space-x-2">
                  <span className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-xs">
                    {test.technique}
                  </span>
                  <span className="px-2 py-0.5 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded text-xs">
                    {test.tactic}
                  </span>
                </div>
              </div>
              <button
                onClick={() => handleRunTest(test.id)}
                disabled={test.status === 'running'}
                className="p-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50"
              >
                <Play size={14} />
              </button>
            </div>

            <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
              {test.description}
            </p>

            <div className="space-y-2">
              <div className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
                <div className="flex items-center mb-1">
                  <Info size={12} className="mr-1" />
                  <span className="text-xs font-semibold">Command</span>
                </div>
                <code className="text-xs font-mono block overflow-x-auto">
                  {test.command}
                </code>
              </div>

              {test.cleanupCommand && (
                <div className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
                  <div className="flex items-center mb-1">
                    <Info size={12} className="mr-1" />
                    <span className="text-xs font-semibold">Cleanup</span>
                  </div>
                  <code className="text-xs font-mono block overflow-x-auto">
                    {test.cleanupCommand}
                  </code>
                </div>
              )}

              <div className="flex flex-wrap gap-2">
                {test.platform.map(platform => (
                  <span
                    key={platform}
                    className="px-2 py-0.5 bg-gray-100 dark:bg-gray-600 rounded text-xs"
                  >
                    {platform}
                  </span>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default AtomicTestRunner;