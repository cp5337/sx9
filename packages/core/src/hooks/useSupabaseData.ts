import { useEffect, useState } from 'react';
import { supabase } from '@/lib/supabase';
import type { GroundNode, Satellite, QKDMetric } from '@/types';

export function useSupabaseData<T>(tableName: string) {
  const [data, setData] = useState<T[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchData() {
      try {
        const { data: fetchedData, error: fetchError } = await supabase
          .from(tableName)
          .select('*');

        if (fetchError) throw fetchError;
        setData(fetchedData || []);
      } catch (err) {
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    }

    fetchData();

    const channel = supabase
      .channel(`${tableName}_changes`)
      .on(
        'postgres_changes',
        { event: '*', schema: 'public', table: tableName },
        () => {
          fetchData();
        }
      )
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, [tableName]);

  return { data, loading, error };
}

// Mock data for demo when Supabase is not configured
const getMockGroundNodes = (): GroundNode[] => [
  { id: '1', name: 'New York GS', latitude: 40.7128, longitude: -74.0060, tier: 1, demand_gbps: 10.5, weather_score: 0.85 },
  { id: '2', name: 'London GS', latitude: 51.5074, longitude: -0.1278, tier: 1, demand_gbps: 12.3, weather_score: 0.75 },
  { id: '3', name: 'Tokyo GS', latitude: 35.6762, longitude: 139.6503, tier: 1, demand_gbps: 15.2, weather_score: 0.90 },
  { id: '4', name: 'Sydney GS', latitude: -33.8688, longitude: 151.2093, tier: 2, demand_gbps: 8.7, weather_score: 0.95 },
  { id: '5', name: 'São Paulo GS', latitude: -23.5505, longitude: -46.6333, tier: 2, demand_gbps: 9.1, weather_score: 0.70 },
  { id: '6', name: 'Dubai GS', latitude: 25.2048, longitude: 55.2708, tier: 2, demand_gbps: 11.4, weather_score: 0.88 },
];

export function useGroundNodes() {
  const [nodes, setNodes] = useState<GroundNode[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchNodes() {
      try {
        const { data, error: fetchError } = await supabase
          .from('ground_nodes')
          .select('*')
          .order('name');

        if (fetchError) throw fetchError;
        setNodes(data || []);
      } catch (err) {
        console.error('Error loading ground nodes:', err);
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    }

    fetchNodes();

    const channel = supabase
      .channel('ground_nodes_changes')
      .on(
        'postgres_changes',
        { event: '*', schema: 'public', table: 'ground_nodes' },
        () => {
          fetchNodes();
        }
      )
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, []);

  return { nodes, loading, error };
}

export function useSatellites() {
  const [satellites, setSatellites] = useState<Satellite[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchSatellites() {
      try {
        const { data, error: fetchError } = await supabase
          .from('satellites')
          .select('*')
          .order('name');

        if (fetchError) throw fetchError;
        setSatellites(data || []);
      } catch (err) {
        console.error('Error loading satellites:', err);
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    }

    fetchSatellites();

    const channel = supabase
      .channel('satellites_changes')
      .on(
        'postgres_changes',
        { event: '*', schema: 'public', table: 'satellites' },
        () => {
          fetchSatellites();
        }
      )
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, []);

  return { satellites, loading, error };
}

export function useQKDMetrics(satelliteId?: string) {
  const [metrics, setMetrics] = useState<QKDMetric[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchMetrics() {
      try {
        let query = supabase
          .from('qkd_metrics')
          .select('*')
          .order('timestamp', { ascending: false })
          .limit(100);

        if (satelliteId) {
          query = query.eq('satellite_id', satelliteId);
        }

        const { data, error: fetchError } = await query;

        if (fetchError) throw fetchError;
        setMetrics(data || []);
      } catch (err) {
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    }

    fetchMetrics();
  }, [satelliteId]);

  return { metrics, loading, error };
}
