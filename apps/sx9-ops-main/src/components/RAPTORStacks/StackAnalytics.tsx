import React from 'react';
import { PieChart, AlertTriangle, Activity, BarChart2 } from 'lucide-react';

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

interface StackAnalyticsProps {
  stacks: Stack[];
}

const StackAnalytics: React.FC<StackAnalyticsProps> = ({ stacks }) => {
  const activeStacks = stacks.filter(stack => stack.status === 'Active').length;
  const inactiveStacks = stacks.length - activeStacks;

  const missionCounts = stacks.reduce((acc, stack) => {
    acc[stack.hd4Mission] = (acc[stack.hd4Mission] || 0) + 1;
    return acc;
  }, {} as Record<Stack['hd4Mission'], number>);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-2 flex items-center">
          <BarChart2 size={14} className="mr-2" />
          Stack Status
        </h2>
        <div className="flex justify-around">
          <div className="text-center">
            <p className="text-2xl font-bold text-green-500">{activeStacks}</p>
            <p className="text-xs text-gray-500">Active</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-red-500">{inactiveStacks}</p>
            <p className="text-xs text-gray-500">Inactive</p>
          </div>
        </div>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-2 flex items-center">
          <PieChart size={14} className="mr-2" />
          HD4 Mission Distribution
        </h2>
        <ul className="text-xs">
          {Object.entries(missionCounts).map(([mission, count]) => (
            <li key={mission} className="flex justify-between items-center mb-1">
              <span>{mission}</span>
              <span className="font-semibold">{count}</span>
            </li>
          ))}
        </ul>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-2 flex items-center">
          <Activity size={14} className="mr-2" />
          Stack Performance
        </h2>
        <p className="text-xs text-gray-500 mb-2">Average response time: 250ms</p>
        <p className="text-xs text-gray-500">Uptime: 99.9%</p>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-2 flex items-center">
          <AlertTriangle size={14} className="mr-2" />
          Recent Alerts
        </h2>
        <ul className="text-xs">
          <li className="mb-1">High CPU usage on vRaven-Energy-1</li>
          <li className="mb-1">Unusual network activity detected on vRaven-NYPD-1</li>
          <li>Elasticsearch index corruption on vRaven-TexasGrid-1</li>
        </ul>
      </div>
    </div>
  );
};

export default StackAnalytics;