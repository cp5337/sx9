import React, { useState } from 'react';
import { Network, Target, Shield, Activity, Clock, Terminal } from 'lucide-react';
import NetworkCLI from './NetworkCLI';
import NetworkMap from './NetworkMap';

interface NetworkDeployment {
  id: string;
  cidr: string;
  type: 'honeytrap' | 'vraven' | 'redteam';
  status: 'active' | 'inactive';
  ttl: number;
  lastActivity: string;
  description: string;
}

const NetworksControl: React.FC = () => {
  const [networks, setNetworks] = useState<NetworkDeployment[]>([
    {
      id: 'ht-1',
      cidr: '10.0.1.0/24',
      type: 'honeytrap',
      status: 'active',
      ttl: 86400,
      lastActivity: '2023-06-15T10:30:00Z',
      description: 'Primary deception network'
    },
    {
      id: 'vr-1',
      cidr: '172.16.0.0/16',
      type: 'vraven',
      status: 'active',
      ttl: 43200,
      lastActivity: '2023-06-15T11:45:00Z',
      description: 'vRaven deployment network'
    },
    {
      id: 'rt-1',
      cidr: '192.168.1.0/24',
      type: 'redteam',
      status: 'active',
      ttl: 28800,
      lastActivity: '2023-06-15T09:15:00Z',
      description: 'Yankee Stadium Red Team'
    }
  ]);

  const [selectedNetwork, setSelectedNetwork] = useState<string | null>(null);
  const [showMap, setShowMap] = useState(true);

  const getNetworkTypeIcon = (type: string) => {
    switch (type) {
      case 'honeytrap': return <Shield size={12} className="text-yellow-500" />;
      case 'vraven': return <Network size={12} className="text-blue-500" />;
      case 'redteam': return <Target size={12} className="text-red-500" />;
      default: return null;
    }
  };

  const formatTTL = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <div className="space-y-4">
        <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
          <h2 className="text-sm font-semibold mb-4 flex items-center">
            <Network size={14} className="mr-2" />
            Network Deployments
          </h2>
          <div className="space-y-2">
            {networks.map(network => (
              <div 
                key={network.id}
                className={`p-2 rounded-lg cursor-pointer ${
                  selectedNetwork === network.id 
                    ? 'bg-blue-100 dark:bg-blue-900' 
                    : 'bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600'
                }`}
                onClick={() => setSelectedNetwork(network.id)}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    {getNetworkTypeIcon(network.type)}
                    <span className="ml-2 font-medium">{network.cidr}</span>
                  </div>
                  <span className={`px-2 py-0.5 rounded-full text-xs ${
                    network.status === 'active' ? 'bg-green-500' : 'bg-red-500'
                  } text-white`}>
                    {network.status}
                  </span>
                </div>
                <div className="mt-1 text-xs text-gray-600 dark:text-gray-400">
                  <div className="flex items-center">
                    <Clock size={10} className="mr-1" />
                    TTL: {formatTTL(network.ttl)}
                  </div>
                  <p className="mt-0.5">{network.description}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-sm font-semibold flex items-center">
              <Terminal size={14} className="mr-2" />
              Network CLI
            </h2>
          </div>
          <NetworkCLI 
            selectedNetwork={selectedNetwork ? networks.find(n => n.id === selectedNetwork) || null : null} 
          />
        </div>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <NetworkMap />
      </div>
    </div>
  );
};

export default NetworksControl;