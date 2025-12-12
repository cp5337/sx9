import React, { useState, useEffect } from 'react';
import { Server, Cloud, Box, Database, Network, Shield, Zap, Globe, Activity, AlertTriangle, CheckCircle, Clock } from 'lucide-react';


interface ContainerItem {
  id: string;
  name: string;
  status: 'Running' | 'Stopped' | 'Error';
  type: 'Kubernetes' | 'Linode' | 'Docker';
  vulnerabilities: string[];
}

interface ExploitSimulation {
  id: string;
  name: string;
  description: string;
  target: string;
  status: 'Ready' | 'In Progress' | 'Completed';
}

const Containers: React.FC = () => {
  const [containers, setContainers] = useState<ContainerItem[]>([
    { id: 'k8s-1', name: 'web-app', status: 'Running', type: 'Kubernetes', vulnerabilities: ['CVE-2021-44228', 'CVE-2022-22965'] },
    { id: 'k8s-2', name: 'database', status: 'Running', type: 'Kubernetes', vulnerabilities: ['CVE-2021-3449'] },
    { id: 'linode-1', name: 'api-server', status: 'Running', type: 'Linode', vulnerabilities: ['CVE-2021-44228', 'CVE-2021-45046'] },
    { id: 'linode-2', name: 'cache-server', status: 'Stopped', type: 'Linode', vulnerabilities: [] },
    { id: 'docker-1', name: 'nginx-proxy', status: 'Running', type: 'Docker', vulnerabilities: ['CVE-2021-23017'] },
    { id: 'docker-2', name: 'redis', status: 'Running', type: 'Docker', vulnerabilities: ['CVE-2022-0543'] },
    { id: 'docker-3', name: 'test-container', status: 'Error', type: 'Docker', vulnerabilities: ['CVE-2021-41091', 'CVE-2021-41092'] },
  ]);

  const [exploitSimulations, setExploitSimulations] = useState<ExploitSimulation[]>([
    { id: 'sim-1', name: 'Log4Shell Exploit', description: 'Simulating Log4j vulnerability exploitation', target: 'k8s-1', status: 'Ready' },
    { id: 'sim-2', name: 'Spring4Shell', description: 'Testing Spring Framework RCE vulnerability', target: 'k8s-1', status: 'In Progress' },
    { id: 'sim-3', name: 'Redis Unauthorized Access', description: 'Attempting unauthorized access to Redis instance', target: 'docker-2', status: 'Completed' },
  ]);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Running':
        return 'text-green-500';
      case 'Stopped':
        return 'text-yellow-500';
      case 'Error':
        return 'text-red-500';
      default:
        return 'text-gray-500';
    }
  };

  const getIcon = (type: string) => {
    switch (type) {
      case 'Kubernetes':
        return <Server size={16} className="text-blue-500" />;
      case 'Linode':
        return <Cloud size={16} className="text-green-500" />;
      case 'Docker':
        return <Box size={16} className="text-purple-500" />;
      default:
        return null;
    }
  };

  const launchExploitSimulation = (containerId: string) => {
    const targetContainer = containers.find(c => c.id === containerId);
    if (targetContainer && targetContainer.vulnerabilities.length > 0) {
      const newSimulation: ExploitSimulation = {
        id: `sim-${exploitSimulations.length + 1}`,
        name: `Exploit ${targetContainer.vulnerabilities[0]}`,
        description: `Simulating exploitation of ${targetContainer.vulnerabilities[0]} on ${targetContainer.name}`,
        target: containerId,
        status: 'In Progress'
      };
      setExploitSimulations([...exploitSimulations, newSimulation]);
    }
  };

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen">
      <h1 className="text-lg font-semibold mb-4 text-gray-800 dark:text-white">Container Management & Offensive Simulation</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <h2 className="text-md font-semibold mb-2 text-gray-700 dark:text-gray-300">Containers</h2>
          <div className="space-y-2">
            {containers.map((container) => (
              <div key={container.id} className="bg-white dark:bg-gray-800 p-3 rounded-lg shadow">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center">
                    {getIcon(container.type)}
                    <span className="ml-2 text-sm font-medium text-gray-700 dark:text-gray-300">{container.name}</span>
                  </div>
                  <span className={`text-xs font-medium ${getStatusColor(container.status)}`}>
                    {container.status}
                  </span>
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400 mb-2">
                  Vulnerabilities: {container.vulnerabilities.join(', ') || 'None detected'}
                </div>
                <button
                  onClick={() => launchExploitSimulation(container.id)}
                  className="text-xs bg-red-500 text-white px-2 py-1 rounded hover:bg-red-600 transition-colors duration-200"
                >
                  Simulate Exploit
                </button>
              </div>
            ))}
          </div>
        </div>
        
        <div>
          <h2 className="text-md font-semibold mb-2 text-gray-700 dark:text-gray-300">Exploit Simulations</h2>
          <div className="space-y-2">
            {exploitSimulations.map((sim) => (
              <div key={sim.id} className="bg-white dark:bg-gray-800 p-3 rounded-lg shadow">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">{sim.name}</span>
                  <span className={`text-xs font-medium ${
                    sim.status === 'Completed' ? 'text-green-500' :
                    sim.status === 'In Progress' ? 'text-yellow-500' : 'text-blue-500'
                  }`}>
                    {sim.status}
                  </span>
                </div>
                <p className="text-xs text-gray-500 dark:text-gray-400 mb-2">{sim.description}</p>
                <p className="text-xs text-gray-500 dark:text-gray-400">Target: {sim.target}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Containers;