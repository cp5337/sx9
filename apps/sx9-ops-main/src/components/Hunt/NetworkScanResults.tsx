import React from 'react';
import { ScanResult } from '@/components/../types/huntTypes';
import { AlertTriangle, Shield, Globe, Server } from 'lucide-react';


interface NetworkScanResultsProps {
  results: ScanResult;
}

const NetworkScanResults: React.FC<NetworkScanResultsProps> = ({ results }) => {
  return (
    <div className="space-y-4">
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-sm font-semibold flex items-center">
            <Globe className="w-4 h-4 mr-2" />
            Target: {results.target}
          </h2>
          <span className="text-xs text-gray-500">
            Scanned: {new Date(results.timestamp).toLocaleString()}
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="space-y-2">
            <h3 className="text-xs font-semibold flex items-center mb-2">
              <Server className="w-3 h-3 mr-1" />
              Open Ports & Services
            </h3>
            {results.ports.map((port) => (
              <div key={port.port} className="bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs">
                <div className="flex justify-between">
                  <span>Port {port.port}</span>
                  <span className="text-blue-500">{port.service}</span>
                </div>
              </div>
            ))}
          </div>

          <div className="space-y-2">
            <h3 className="text-xs font-semibold flex items-center mb-2">
              <AlertTriangle className="w-3 h-3 mr-1" />
              Vulnerabilities
            </h3>
            {results.vulnerabilities.length > 0 ? (
              results.vulnerabilities.map((vuln, index) => (
                <div key={index} className="bg-red-50 dark:bg-red-900/20 p-2 rounded text-xs">
                  <span className="text-red-600 dark:text-red-400">{vuln}</span>
                </div>
              ))
            ) : (
              <div className="bg-green-50 dark:bg-green-900/20 p-2 rounded text-xs">
                <span className="text-green-600 dark:text-green-400">No vulnerabilities detected</span>
              </div>
            )}
          </div>
        </div>

        <div className="mt-4">
          <h3 className="text-xs font-semibold flex items-center mb-2">
            <Shield className="w-3 h-3 mr-1" />
            System Information
          </h3>
          <div className="bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs">
            <p><span className="font-semibold">OS:</span> {results.osInfo.name} {results.osInfo.version}</p>
            <p><span className="font-semibold">Detection Accuracy:</span> {results.osInfo.accuracy}%</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NetworkScanResults;