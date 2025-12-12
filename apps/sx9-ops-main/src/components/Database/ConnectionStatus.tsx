import React from 'react';
import { CheckCircle, XCircle, Database } from 'lucide-react';
import { useSimulatedData } from '@/components/../hooks/useSimulatedData';

const ConnectionStatus: React.FC = () => {
  const { isConnected, error } = useSimulatedData();

  return (
    <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
      <div className="flex items-center justify-between">
        <div className="flex items-center">
          <Database className="w-4 h-4 mr-2" />
          <h2 className="text-sm font-semibold">Database Connection Status</h2>
        </div>
        <div className="flex items-center">
          {isConnected ? (
            <>
              <CheckCircle className="w-4 h-4 text-green-500 mr-1" />
              <span className="text-green-500 text-xs">Connected</span>
            </>
          ) : (
            <>
              <XCircle className="w-4 h-4 text-red-500 mr-1" />
              <span className="text-red-500 text-xs">Disconnected</span>
            </>
          )}
        </div>
      </div>
      {error && (
        <div className="mt-2 text-red-500 text-xs">
          Error: {error}
        </div>
      )}
    </div>
  );
};

export default ConnectionStatus;