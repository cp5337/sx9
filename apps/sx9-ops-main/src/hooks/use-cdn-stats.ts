import { useState, useEffect, useCallback } from 'react';
import { cdnStatsClient, type CDNStats } from '../lib/api/cdn-stats-client';

interface UseCDNStatsReturn {
  stats: CDNStats | null;
  loading: boolean;
  error: Error | null;
  connected: boolean;
  refresh: () => Promise<void>;
}

export function useCDNStats(useStream: boolean = true): UseCDNStatsReturn {
  const [stats, setStats] = useState<CDNStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [connected, setConnected] = useState(false);

  const fetchStats = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await cdnStatsClient.getStats();
      setStats(data);
      setConnected(true);
    } catch (err) {
      setError(err as Error);
      setConnected(false);
      console.error("Failed to fetch CDN stats:", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    // Check if service is available
    cdnStatsClient.isAvailable().then((available) => {
      if (!available) {
        console.warn("⚠️  CDN Statistical Service not available, using fallback");
        setConnected(false);
        setLoading(false);
        return;
      }

      if (useStream) {
        // Connect to WebSocket stream
        cdnStatsClient.connectStream(
          (newStats) => {
            setStats(newStats);
            setConnected(true);
            setLoading(false);
            setError(null);
          },
          (err) => {
            setError(err);
            setConnected(false);
          }
        );

        return () => {
          cdnStatsClient.disconnect();
        };
      } else {
        // Fetch once
        fetchStats();
      }
    });
  }, [useStream, fetchStats]);

  return {
    stats,
    loading,
    error,
    connected,
    refresh: fetchStats
  };
}

