import React, { useState, useEffect } from 'react';
import { Database, Network, Shield, Zap, Globe, Activity, AlertTriangle, CheckCircle, Clock } from 'lucide-react';

interface StreamEvent {
  id: string;
  type: string;
  data: any;
  timestamp: string;
}

const StreamProcessor: React.FC = () => {
  const [events, setEvents] = useState<StreamEvent[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  // const [status, setStatus] = useState(...);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const processEvent = (eventData: any): StreamEvent => {
    return {
      id: Date.now().toString(),
      type: typeof eventData.type === 'string' ? eventData.type : 'unknown',
      data: eventData,
      timestamp: new Date().toISOString()
    };
  };

  // Demo mode: Generate sample events
  useEffect(() => {
    const generateEvent = () => {
      const types = ['threat', 'system', 'alert'];
      const type = types[Math.floor(Math.random() * types.length)];
      
      const event: StreamEvent = {
        id: crypto.randomUUID(),
        type: type as string,
        timestamp: new Date().toISOString(),
        data: {
          source: 'demo-mode',
          message: `Sample ${type} event`,
          severity: Math.floor(Math.random() * 5) + 1
        }
      };

      setEvents(prev => [...prev.slice(-99), event]);
    };

    const interval = setInterval(generateEvent, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-gray-900 text-white p-4 rounded-lg">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-bold flex items-center">
          <Activity className="mr-2" size={20} />
          Stream Processor (Demo Mode)
        </h2>
        <div className="flex items-center">
          {/* status === 'connected' && <CheckCircle size={16} className="text-green-500 mr-1" /> */}
          {/* {status === 'error' && <AlertTriangle size={16} className="text-red-500 mr-1" />} */}
          {/* {status === 'reconnecting' && <RefreshCw size={16} className="text-yellow-500 mr-1 animate-spin" />} */}
          <span className={`text-xs ${
            // status === 'connected' ? 'text-green-500' : 
            // status === 'error' ? 'text-red-500' : 
            // status === 'reconnecting' ? 'text-yellow-500' :
            'text-gray-500'
          }`}>
            {/* {status.charAt(0).toUpperCase() + status.slice(1)} */}
            Processing...
          </span>
        </div>
      </div>

      {errorMessage && (
        <div className="bg-red-900/50 text-red-200 p-2 rounded mb-4 text-xs">
          <p className="font-semibold">Error:</p>
          <p>{errorMessage}</p>
        </div>
      )}

      <div className="space-y-2 max-h-96 overflow-y-auto">
        {events.map((event) => (
          <div key={event.id} className="bg-gray-800 p-2 rounded text-xs">
            <div className="flex justify-between items-center mb-1">
              <span className={`px-2 py-0.5 rounded-full text-xxs ${
                // event.error ? 'bg-red-500' :
                event.type === 'threat' ? 'bg-red-500' :
                event.type === 'system' ? 'bg-blue-500' :
                'bg-yellow-500'
              }`}>
                {/* {event.error ? 'Error' : event.type} */}
                {event.type}
              </span>
              <span className="text-gray-400">{new Date(event.timestamp).toLocaleTimeString()}</span>
            </div>
            {/* {event.error ? (
              <p className="text-red-400">{event.error}</p>
            ) : ( */}
              <pre className="whitespace-pre-wrap break-words">
                {JSON.stringify(event.data, null, 2)}
              </pre>
            {/* )} */}
          </div>
        ))}
      </div>
    </div>
  );
};

export default StreamProcessor;