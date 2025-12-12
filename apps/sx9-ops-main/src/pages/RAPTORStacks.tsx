import React, { useState } from 'react';
import { Layers, Target, Database } from 'lucide-react';
import StackManagement from '@/components/RAPTORStacks/StackManagement';
import StackAnalytics from '@/components/RAPTORStacks/StackAnalytics';

import RAPTORStackManagement from '@/components/RAPTORStacks/RAPTORStackManagement';

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

const RAPTORStacks: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'management' | 'analytics' | 'raptor-management'>('management');
  const [stacks, setStacks] = useState<Stack[]>([
    { id: '1', name: 'RAPTOR (Energy Sector)', status: 'Active', attackSurface: 'SCADA Systems', hd4Mission: 'Hunt', target: 'Texas Power Grid', vRavenInstance: 'RAPTOR-Energy-1', elasticSearch: true, k8sConfig: 'energy-cluster' },
    { id: '2', name: 'RAPTOR (Texas Grid)', status: 'Active', attackSurface: 'Smart Meters', hd4Mission: 'Detect', target: 'ERCOT', vRavenInstance: 'RAPTOR-TexasGrid-1', elasticSearch: true, k8sConfig: 'texas-cluster' },
    { id: '3', name: 'RAPTOR (Medical Systems)', status: 'Inactive', attackSurface: 'Hospital IoT Devices', hd4Mission: 'Disable', target: 'Medical Center', vRavenInstance: 'RAPTOR-Medical-1', elasticSearch: false, k8sConfig: 'healthcare-cluster' },
    { id: '4', name: 'RAPTOR (Law Enforcement)', status: 'Active', attackSurface: 'Police Database', hd4Mission: 'Dominate', target: 'Law Enforcement HQ', vRavenInstance: 'RAPTOR-LE-1', elasticSearch: true, k8sConfig: 'law-enforcement-cluster' },
    { id: '5', name: 'RAPTOR (Financial)', status: 'Inactive', attackSurface: 'Financial Systems', hd4Mission: 'Disrupt', target: 'Financial Institution', vRavenInstance: 'RAPTOR-Financial-1', elasticSearch: true, k8sConfig: 'financial-cluster' },
    { id: '6', name: 'RAPTOR (Cybercrime)', status: 'Active', attackSurface: 'Cybercrime Systems', hd4Mission: 'Hunt', target: 'Cybercrime Division', vRavenInstance: 'RAPTOR-Cyber-1', elasticSearch: true, k8sConfig: 'cyber-cluster' },
  ]);

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen text-xs">
      <h1 className="text-xs font-semibold mb-4 text-gray-800 dark:text-white flex items-center">
        <Layers className="mr-2" size={12} />
        Offensive RAPTOR Stacks
      </h1>
      
      <div className="mb-4 flex">
        <button
          className={`mr-2 px-2 py-1 text-xs rounded ${activeTab === 'management' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('management')}
        >
          Stack Management
        </button>
        <button
          className={`mr-2 px-2 py-1 text-xs rounded ${activeTab === 'analytics' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('analytics')}
        >
          Stack Analytics
        </button>

        <button
                      className={`px-2 py-1 text-xs rounded ${activeTab === 'raptor-management' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
            onClick={() => setActiveTab('raptor-management')}
          >
            RAPTOR Management
        </button>
      </div>

      {activeTab === 'management' && (
        <StackManagement stacks={stacks} setStacks={setStacks} />
      )}

      {activeTab === 'analytics' && (
        <StackAnalytics stacks={stacks} />
      )}

      

              {activeTab === 'raptor-management' && (
          <RAPTORStackManagement stacks={stacks} setStacks={setStacks} />
        )}
    </div>
  );
};

export default RAPTORStacks;