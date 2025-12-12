/**
 * @deprecated This component duplicates SetupScripts.tsx "scripts" tab.
 * Use SetupScripts instead which includes Ansible and Atomic Red Team tabs.
 * TODO: Remove this page and update routes to point to /setup-scripts
 */
import React, { useState } from 'react';
import { Code, Play, Plus, Database } from 'lucide-react';

interface AnonymizationScript {
  id: string;
  name: string;
  description: string;
  target: string;
}

const QuickScripts: React.FC = () => {
  const [scripts, setScripts] = useState<AnonymizationScript[]>([
    { id: '1', name: 'IP Anonymizer', description: 'Replaces IP addresses with pseudonyms in log files', target: 'Network Logs' },
    { id: '2', name: 'PII Scrubber', description: 'Removes personally identifiable information from documents', target: 'Text Documents' },
    { id: '3', name: 'Database Sanitizer', description: 'Anonymizes sensitive fields in database exports', target: 'Database Dumps' },
    { id: '4', name: 'Image Metadata Cleaner', description: 'Strips metadata from images to protect privacy', target: 'Image Files' },
    { id: '5', name: 'Email Address Masker', description: 'Replaces email addresses with hashed versions', target: 'Email Logs' },
  ]);

  const [newScript, setNewScript] = useState<Omit<AnonymizationScript, 'id'>>({
    name: '',
    description: '',
    target: '',
  });

  const handleAddScript = () => {
    const id = (scripts.length + 1).toString();
    setScripts([...scripts, { ...newScript, id }]);
    setNewScript({ name: '', description: '', target: '' });
  };

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen">
      <h1 className="text-xs font-semibold mb-4 text-gray-800 dark:text-white flex items-center">
        <Code className="mr-2" size={12} />
        Anonymization Quick Scripts
      </h1>
      
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
        <h2 className="font-semibold mb-2 text-xs">Add New Anonymization Script</h2>
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
    </div>
  );
};

export default QuickScripts;