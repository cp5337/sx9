import React from 'react';
import { Activity } from 'lucide-react';

const SystemOverview: React.FC = () => {
  const components = [
    { name: 'Threat Analysis Engine', status: 'Operational' },
    { name: 'Data Ingestion Service', status: 'Operational' },
    { name: 'User Authentication', status: 'Operational' },
    { name: 'API Gateway', status: 'Degraded' },
    { name: 'Reporting Module', status: 'Maintenance' },
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Operational':
        return 'bg-green-500';
      case 'Degraded':
        return 'bg-yellow-500';
      case 'Maintenance':
        return 'bg-blue-500';
      default:
        return 'bg-red-500';
    }
  };

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-4">System Overview</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {components.map((component, index) => (
          <div key={index} className="bg-white p-4 rounded-lg shadow">
            <div className="flex items-center justify-between">
              <span className="font-medium">{component.name}</span>
              <div className={`w-3 h-3 rounded-full ${getStatusColor(component.status)}`}></div>
            </div>
            <div className="mt-2 text-sm text-gray-600">{component.status}</div>
          </div>
        ))}
      </div>
      <div className="mt-8">
        <h3 className="text-xl font-semibold mb-4">System Health</h3>
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="flex items-center">
            <Activity className="w-6 h-6 text-blue-500 mr-2" />
            <span className="text-lg font-medium">Overall System Health: Good</span>
          </div>
          <p className="mt-2 text-gray-600">
            The system is operating normally with minor issues in the API Gateway. Maintenance is scheduled for the Reporting Module.
          </p>
        </div>
      </div>
    </div>
  );
};

export default SystemOverview;