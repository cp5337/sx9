import React, { useState } from 'react';
import { Code, Book, Target, Play, Plus, Shield, Database } from 'lucide-react';
import AnsiblePlaybooks from '@/components/AnsiblePlaybooks';
import AtomicTestRunner from '@/components/AtomicRedTeam/AtomicTestRunner';
import AttackNavigator from '@/components/AtomicRedTeam/AttackNavigator';

interface SetupScript {
  id: string;
  name: string;
  description: string;
  target: string;
}

const SetupScripts: React.FC = () => {
  const [scripts, setScripts] = useState<SetupScript[]>([
    { id: '1', name: 'IP Anonymizer', description: 'Replaces IP addresses with pseudonyms in log files', target: 'Network Logs' },
    { id: '2', name: 'PII Scrubber', description: 'Removes personally identifiable information from documents', target: 'Text Documents' },
    { id: '3', name: 'Database Sanitizer', description: 'Anonymizes sensitive fields in database exports', target: 'Database Dumps' },
    { id: '4', name: 'Image Metadata Cleaner', description: 'Strips metadata from images to protect privacy', target: 'Image Files' },
    { id: '5', name: 'Email Address Masker', description: 'Replaces email addresses with hashed versions', target: 'Email Logs' },
  ]);

  const [newScript, setNewScript] = useState<Omit<SetupScript, 'id'>>({
    name: '',
    description: '',
    target: '',
  });

  const handleAddScript = () => {
    const id = (scripts.length + 1).toString();
    setScripts([...scripts, { ...newScript, id }]);
    setNewScript({ name: '', description: '', target: '' });
  };

  const [activeTab, setActiveTab] = useState<'scripts' | 'ansible' | 'atomic'>('scripts');

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen">
      <div className="mb-6">
        <h1 className="text-xs font-semibold text-gray-800 dark:text-white flex items-center mb-4">
          <Code className="mr-2" size={12} />
          Setup Scripts
        </h1>
        <div className="flex border-b border-gray-200 dark:border-gray-700">
          <button
            onClick={() => setActiveTab('scripts')}
            className={`flex items-center px-4 py-2 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'scripts'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            <Code size={12} className="mr-2" />
            Setup Scripts
          </button>
          <button
            onClick={() => setActiveTab('ansible')}
            className={`flex items-center px-4 py-2 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'ansible'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            <Book size={12} className="mr-2" />
            Ansible Playbooks
          </button>
          <button
            onClick={() => setActiveTab('atomic')}
            className={`flex items-center px-4 py-2 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'atomic'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            <Target size={12} className="mr-2" />
            Atomic Red Team
          </button>
        </div>
      </div>

      {activeTab === 'scripts' && (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
            {scripts.map(script => (
              <div key={script.id} className="bg-white dark:bg-gray-800 p-3 rounded-lg shadow-md text-xs">
                <div className="flex justify-between items-center mb-2">
                  <h2 className="font-semibold">{script.name}</h2>
                  <button className="bg-blue-500 text-white px-2 py-1 rounded text-xxs flex items-center">
                    <Play size={10} className="mr-1" />
                    Run
                  </button>
                </div>
                <p className="text-gray-600 dark:text-gray-400 mb-1">{script.description}</p>
                <p className="text-gray-500 dark:text-gray-500">Target: {script.target}</p>
              </div>
            ))}
          </div>

          <div className="bg-white dark:bg-gray-800 p-3 rounded-lg shadow-md">
            <h2 className="font-semibold mb-2 text-xs">Add New Setup Script</h2>
            <div className="grid grid-cols-1 gap-2">
              <input
                type="text"
                placeholder="Script Name"
                value={newScript.name}
                onChange={(e) => setNewScript({ ...newScript, name: e.target.value })}
                className="p-1 border rounded text-xs"
              />
              <input
                type="text"
                placeholder="Description"
                value={newScript.description}
                onChange={(e) => setNewScript({ ...newScript, description: e.target.value })}
                className="p-1 border rounded text-xs"
              />
              <input
                type="text"
                placeholder="Target"
                value={newScript.target}
                onChange={(e) => setNewScript({ ...newScript, target: e.target.value })}
                className="p-1 border rounded text-xs"
              />
            </div>
            <div className="mt-2 flex items-center">
              <button onClick={handleAddScript} className="bg-green-500 text-white px-2 py-1 rounded text-xs flex items-center">
                <Plus size={10} className="mr-1" />
                Add Script
              </button>
            </div>
          </div>
        </>
      )}

      {activeTab === 'ansible' && <AnsiblePlaybooks />}

      {activeTab === 'atomic' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <div>
            <h2 className="text-sm font-semibold mb-4 flex items-center">
              <Target size={14} className="mr-2" />
              ATT&CK Navigator
            </h2>
            <AttackNavigator />
          </div>
          <div>
            <h2 className="text-sm font-semibold mb-4 flex items-center">
              <Shield size={14} className="mr-2" />
              Test Runner
            </h2>
            <AtomicTestRunner phase="Hunt" />
          </div>
        </div>
      )}
    </div>
  );
};

export default SetupScripts;