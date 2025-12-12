import React from 'react';
import { Package, Download } from 'lucide-react';

const APIStore: React.FC = () => {
  const apis = [
    { name: 'Threat Intelligence API', description: 'Access to real-time threat intelligence data', status: 'Available' },
    { name: 'Malware Analysis API', description: 'Automated malware analysis and reporting', status: 'Available' },
    { name: 'Network Traffic Analysis API', description: 'Deep packet inspection and traffic analysis', status: 'In Development' },
    { name: 'Vulnerability Scanner API', description: 'Comprehensive vulnerability assessment tools', status: 'Available' },
    { name: 'Incident Response API', description: 'Automated incident response and mitigation', status: 'Beta' },
  ];

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-4">API Store</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {apis.map((api, index) => (
          <div key={index} className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center mb-4">
              <Package className="w-6 h-6 text-blue-500 mr-2" />
              <h3 className="text-lg font-medium">{api.name}</h3>
            </div>
            <p className="text-gray-600 mb-4">{api.description}</p>
            <div className="flex items-center justify-between">
              <span className={`px-2 py-1 text-xs font-semibold rounded-full ${
                api.status === 'Available' ? 'bg-green-100 text-green-800' :
                api.status === 'Beta' ? 'bg-yellow-100 text-yellow-800' :
                'bg-gray-100 text-gray-800'
              }`}>
                {api.status}
              </span>
              {api.status === 'Available' && (
                <button className="flex items-center text-blue-500 hover:text-blue-700">
                  <Download className="w-4 h-4 mr-1" />
                  Install
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default APIStore;