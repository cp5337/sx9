import React, { useState } from 'react';
import { Zap, Activity, Database, Shield, Settings } from 'lucide-react';

const SledManagement: React.FC = () => {
  const [isConnected, setIsConnected] = useState(true);

  const stats = [
    { label: 'Key-Value Pairs', value: '2.1M', icon: <Database size={16} /> },
    { label: 'Operations/sec', value: '45K', icon: <Activity size={16} /> },
    { label: 'Storage Used', value: '1.8 GB', icon: <Zap size={16} /> },
    { label: 'Integrity Score', value: '99.9%', icon: <Shield size={16} /> }
  ];

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg p-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold flex items-center">
          <Zap className="mr-2 text-yellow-500" size={20} />
          Sled KVR Management
        </h2>
        <div className="flex items-center space-x-2">
          <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`} />
          <span className="text-sm">{isConnected ? 'Connected' : 'Disconnected'}</span>
        </div>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
        {stats.map((stat, index) => (
          <div key={index} className="bg-gray-50 dark:bg-gray-700 rounded-lg p-3">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600 dark:text-gray-400">{stat.label}</p>
                <p className="text-xl font-bold">{stat.value}</p>
              </div>
              <div className="text-yellow-500">{stat.icon}</div>
            </div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <button className="bg-yellow-500 hover:bg-yellow-600 text-white p-3 rounded-lg text-sm">
          Bulk Import
        </button>
        <button className="bg-blue-500 hover:bg-blue-600 text-white p-3 rounded-lg text-sm">
          Performance Test
        </button>
        <button className="bg-green-500 hover:bg-green-600 text-white p-3 rounded-lg text-sm">
          Integrity Check
        </button>
      </div>
    </div>
  );
};

export default SledManagement;
