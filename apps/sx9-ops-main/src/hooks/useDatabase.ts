import { useState, useEffect } from 'react';
import { getCTASTasks, getThreatActors } from '@/utils/database';

export const useDatabase = () => {
  const [connectionStatus, setConnectionStatus] = useState({
    supabase: false,
    surrealdb: false,
    sled: false,
    slotgraph: false,
    bevy: false,
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        setIsLoading(true);
        // Test database connections by trying to fetch data
        await getCTASTasks();
        await getThreatActors();
        setConnectionStatus({
          supabase: true,
          surrealdb: true,
          sled: true,
          slotgraph: true,
          bevy: true,
        });
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to check database connections');
        setConnectionStatus({
          supabase: false,
          surrealdb: false,
          sled: false,
          slotgraph: false,
          bevy: false,
        });
      } finally {
        setIsLoading(false);
      }
    };

    checkStatus();
    const interval = setInterval(checkStatus, 30000);

    return () => clearInterval(interval);
  }, []);

  return { connectionStatus, isLoading, error };
};