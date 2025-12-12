import React, { useState } from 'react';
import { Database, Globe, Layers, Zap, Network } from 'lucide-react';
import SupabaseManagement from '@/components/Database/SupabaseManagement';
import SurrealDBManagement from '@/components/Database/SurrealDBManagement';
import SledManagement from '@/components/Database/SledManagement';
import SlotGraphManagement from '@/components/Database/SlotGraphManagement';
import BevyManagement from '@/components/Database/BevyManagement';

const DatabasePage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'supabase' | 'surrealdb' | 'sled' | 'slotgraph' | 'bevy'>('supabase');

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen text-xs">
      <h1 className="text-xs font-semibold mb-4 text-gray-800 dark:text-white flex items-center">
        <Database className="mr-2" size={12} />
        Database Management
      </h1>
      
      <div className="mb-4 flex flex-wrap">
        <button
          className={`mr-2 mb-2 px-2 py-1 text-xs rounded flex items-center ${activeTab === 'supabase' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('supabase')}
        >
          <Database size={10} className="mr-1" />
          Supabase
        </button>
        <button
          className={`mr-2 mb-2 px-2 py-1 text-xs rounded flex items-center ${activeTab === 'surrealdb' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('surrealdb')}
        >
          <Globe size={10} className="mr-1" />
          SurrealDB
        </button>
        <button
          className={`mr-2 mb-2 px-2 py-1 text-xs rounded flex items-center ${activeTab === 'sled' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('sled')}
        >
          <Zap size={10} className="mr-1" />
          Sled KVR
        </button>
        <button
          className={`mr-2 mb-2 px-2 py-1 text-xs rounded flex items-center ${activeTab === 'slotgraph' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('slotgraph')}
        >
          <Network size={10} className="mr-1" />
          SlotGraph
        </button>
        <button
          className={`mr-2 mb-2 px-2 py-1 text-xs rounded flex items-center ${activeTab === 'bevy' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}`}
          onClick={() => setActiveTab('bevy')}
        >
          <Layers size={10} className="mr-1" />
          Bevy Interface
        </button>


      </div>

      <div className="mt-4">
        {activeTab === 'supabase' && <SupabaseManagement />}
        {activeTab === 'surrealdb' && <SurrealDBManagement />}
        {activeTab === 'sled' && <SledManagement />}
        {activeTab === 'slotgraph' && <SlotGraphManagement />}
        {activeTab === 'bevy' && <BevyManagement />}
      </div>
    </div>
  );
};

export default DatabasePage;