import { useState, useEffect } from 'react';

export const useSimulatedData = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check real database connection via Kali Tools API health endpoint
    const checkConnection = async () => {
      try {
        const response = await fetch('http://localhost:18451/health');
        if (response.ok) {
          const data = await response.json();
          setIsConnected(data.status === 'healthy');
          setError(null);
        } else {
          setIsConnected(false);
          setError(`HTTP ${response.status}`);
        }
      } catch (err) {
        setIsConnected(false);
        setError(err instanceof Error ? err.message : 'Connection failed');
      }
    };

    // Check immediately
    checkConnection();

    // Check every 10 seconds
    const interval = setInterval(checkConnection, 10000);

    return () => clearInterval(interval);
  }, []);

  return { isConnected, error };
};