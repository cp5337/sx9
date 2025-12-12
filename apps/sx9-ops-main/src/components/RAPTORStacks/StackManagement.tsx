import React, { useState } from 'react';
import { Target, Play, Trash2, Plus, Shield, Zap, Globe, Square } from 'lucide-react';

interface Stack {
  id: string;
  name: string;
  status: 'Active' | 'Inactive';
  attackSurface: string;
  hd4Mission: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  target: string;
  vRavenInstance: string;
  elasticSearch: boolean;
  k8sConfig: string;
}

interface StackManagementProps {
  stacks: Stack[];
  setStacks: React.Dispatch<React.SetStateAction<Stack[]>>;
}

const StackManagement: React.FC<StackManagementProps> = ({ stacks, setStacks }) => {
  const [newStack, setNewStack] = useState<Omit<Stack, 'id'>>({
    name: '',
    status: 'Inactive',
    attackSurface: '',
    hd4Mission: 'Hunt',
    target: '',
    vRavenInstance: '',
    elasticSearch: false,
    k8sConfig: '',
  });

  const handleAddStack = () => {
    const id = (stacks.length + 1).toString();
    setStacks([...stacks, { ...newStack, id }]);
    setNewStack({
      name: '',
      status: 'Inactive',
      attackSurface: '',
      hd4Mission: 'Hunt',
      target: '',
      vRavenInstance: '',
      elasticSearch: false,
      k8sConfig: '',
    });
  };

  const handleRemoveStack = (id: string) => {
    setStacks(stacks.filter(stack => stack.id !== id));
  };

  const handleToggleStatus = (id: string) => {
    setStacks(stacks.map(stack => 
      stack.id === id ? { ...stack, status: stack.status === 'Active' ? 'Inactive' : 'Active' } : stack
    ));
  };

  const getMissionIcon = (mission: Stack['hd4Mission']) => {
    switch (mission) {
      case 'Hunt': return <Target className="text-blue-500" />;
      case 'Detect': return <Shield className="text-green-500" />;
      case 'Disable': return <Zap className="text-yellow-500" />;
      case 'Disrupt': return <Square className="text-red-500" />;
      case 'Dominate': return <Globe className="text-purple-500" />;
    }
  };

  return (
    <>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {stacks.map(stack => (
          <div key={stack.id} className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
            <div className="flex justify-between items-center mb-2">
              <h2 className="text-sm font-semibold">{stack.name}</h2>
              <div className="flex space-x-2">
                <button onClick={() => handleToggleStatus(stack.id)} className={`p-1 rounded ${stack.status === 'Active' ? 'bg-green-500' : 'bg-gray-500'}`}>
                  {stack.status === 'Active' ? <Play size={12} /> : <Square size={12} />}
                </button>
                <button onClick={() => handleRemoveStack(stack.id)} className="p-1 rounded bg-red-500">
                  <Trash2 size={12} />
                </button>
              </div>
            </div>
            <div className="text-xs space-y-1">
              <p><span className="font-semibold">Attack Surface:</span> {stack.attackSurface}</p>
              <p className="flex items-center">
                <span className="font-semibold mr-1">HD4 Mission:</span>
                {getMissionIcon(stack.hd4Mission)}
                {stack.hd4Mission}
              </p>
              <p><span className="font-semibold">Target:</span> {stack.target}</p>
              <p><span className="font-semibold">vRaven Instance:</span> {stack.vRavenInstance}</p>
              <p><span className="font-semibold">Elasticsearch:</span> {stack.elasticSearch ? 'Enabled' : 'Disabled'}</p>
              <p><span className="font-semibold">K8s Config:</span> {stack.k8sConfig}</p>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-4 bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
                  <h2 className="text-sm font-semibold mb-2">Add New RAPTOR Stack</h2>
        <div className="grid grid-cols-2 gap-2">
          <input
            type="text"
            placeholder="Name"
            value={newStack.name}
            onChange={(e) => setNewStack({ ...newStack, name: e.target.value })}
            className="p-1 border rounded text-xs"
          />
          <input
            type="text"
            placeholder="Attack Surface"
            value={newStack.attackSurface}
            onChange={(e) => setNewStack({ ...newStack, attackSurface: e.target.value })}
            className="p-1 border rounded text-xs"
          />
          <select
            value={newStack.hd4Mission}
            onChange={(e) => setNewStack({ ...newStack, hd4Mission: e.target.value as Stack['hd4Mission'] })}
            className="p-1 border rounded text-xs"
          >
            <option value="Hunt">Hunt</option>
            <option value="Detect">Detect</option>
            <option value="Disable">Disable</option>
            <option value="Disrupt">Disrupt</option>
            <option value="Dominate">Dominate</option>
          </select>
          <input
            type="text"
            placeholder="Target"
            value={newStack.target}
            onChange={(e) => setNewStack({ ...newStack, target: e.target.value })}
            className="p-1 border rounded text-xs"
          />
          <input
            type="text"
            placeholder="vRaven Instance"
            value={newStack.vRavenInstance}
            onChange={(e) => setNewStack({ ...newStack, vRavenInstance: e.target.value })}
            className="p-1 border rounded text-xs"
          />
          <input
            type="text"
            placeholder="K8s Config"
            value={newStack.k8sConfig}
            onChange={(e) => setNewStack({ ...newStack, k8sConfig: e.target.value })}
            className="p-1 border rounded text-xs"
          />
        </div>
        <div className="mt-2 flex items-center">
          <label className="flex items-center text-xs">
            <input
              type="checkbox"
              checked={newStack.elasticSearch}
              onChange={(e) => setNewStack({ ...newStack, elasticSearch: e.target.checked })}
              className="mr-1"
            />
            Enable Elasticsearch
          </label>
          <button onClick={handleAddStack} className="ml-auto bg-blue-500 text-white px-2 py-1 rounded text-xs flex items-center">
            <Plus size={12} className="mr-1" />
            Add Stack
          </button>
        </div>
      </div>
    </>
  );
};

export default StackManagement;