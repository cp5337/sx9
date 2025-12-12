import React from 'react';
import { Package, RefreshCw, AlertTriangle } from 'lucide-react';

const PackageStatus: React.FC = () => {
  const packages = [
    { name: 'react', version: '18.2.0', status: 'Up to date' },
    { name: 'typescript', version: '4.9.5', status: 'Update available' },
    { name: 'tailwindcss', version: '3.3.2', status: 'Up to date' },
    { name: 'neo4j-driver', version: '5.17.0', status: 'Up to date' },
    { name: 'lucide-react', version: '0.344.0', status: 'Up to date' },
    { name: 'uuid', version: '9.0.1', status: 'Security update' },
  ];

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-4">Package Status</h2>
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Package</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Version</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Action</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {packages.map((pkg, index) => (
              <tr key={index}>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center">
                    <Package className="w-5 h-5 text-gray-400 mr-2" />
                    {pkg.name}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">{pkg.version}</td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                    pkg.status === 'Up to date' ? 'bg-green-100 text-green-800' :
                    pkg.status === 'Update available' ? 'bg-yellow-100 text-yellow-800' :
                    'bg-red-100 text-red-800'
                  }`}>
                    {pkg.status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  {pkg.status !== 'Up to date' && (
                    <button className="text-indigo-600 hover:text-indigo-900">
                      {pkg.status === 'Update available' ? (
                        <RefreshCw className="w-5 h-5" />
                      ) : (
                        <AlertTriangle className="w-5 h-5" />
                      )}
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default PackageStatus;