import { useEffect, useState } from 'react';
import { Server, PlayCircle, StopCircle, AlertCircle, RefreshCw } from 'lucide-react';
import type { IACManifold } from '../types/rfc-9005.types';
import { globalEventBus } from '../lib/eventBus';
import { supabase } from '../lib/supabase';

export default function IACDashboard() {
  const [manifolds, setManifolds] = useState<IACManifold[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadManifolds();

    const unsubscribeSpawn = globalEventBus.on('iac:spawn_complete', () => {
      loadManifolds();
    });

    const unsubscribeTeardown = globalEventBus.on('iac:teardown', () => {
      loadManifolds();
    });

    return () => {
      unsubscribeSpawn();
      unsubscribeTeardown();
    };
  }, []);

  const loadManifolds = async () => {
    try {
      setLoading(true);
      const { data, error } = await supabase.from('iac_manifolds').select('*').order('created_at', { ascending: false });

      if (error) throw error;
      setManifolds(data || []);
    } catch (err) {
      console.error('Error loading IAC manifolds:', err);
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      active: 'text-green-400 bg-green-900/30 border-green-700',
      spawning: 'text-blue-400 bg-blue-900/30 border-blue-700',
      scaling: 'text-orange-400 bg-orange-900/30 border-orange-700',
      teardown: 'text-gray-400 bg-gray-800/30 border-gray-700',
      dormant: 'text-gray-500 bg-gray-800/20 border-gray-700',
      error: 'text-red-400 bg-red-900/30 border-red-700',
    };
    return colors[status] || 'text-gray-400 bg-gray-800/30 border-gray-700';
  };

  const getManifoldTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
      abe_customer_env: 'ABE Customer Environment',
      plasma_burst_compute: 'PLASMA Burst Compute',
      cuda_parallel_cluster: 'CUDA Parallel Cluster',
      cdn_edge_node: 'CDN Edge Node',
      validation_cluster: 'Validation Cluster',
      monte_carlo_cluster: 'Monte Carlo Cluster',
      smart_crate_overflow: 'Smart Crate Overflow',
      port_expansion: 'Port Expansion',
      edge_node: 'Edge Node',
    };
    return labels[type] || type;
  };

  const totalCost = manifolds.reduce((sum, m) => sum + (m.total_cost ?? 0), 0);
  const activeManifolds = manifolds.filter((m) => m.current_status === 'active').length;
  const totalRuntime = manifolds.reduce((sum, m) => sum + (m.total_runtime_minutes ?? 0), 0);

  return (
    <div className="flex flex-col h-full">
      <div className="p-3 border-b border-gray-200 dark:border-dark-border space-y-3">
        <div className="grid grid-cols-2 gap-2">
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Total</div>
            <div className="text-lg font-bold text-gray-900 dark:text-dark-text-primary">{manifolds.length}</div>
          </div>
          <div className="bg-green-50 dark:bg-green-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Active</div>
            <div className="text-lg font-bold text-green-600 dark:text-green-400">{activeManifolds}</div>
          </div>
          <div className="bg-orange-50 dark:bg-orange-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Runtime</div>
            <div className="text-sm font-bold text-gray-900 dark:text-dark-text-primary">
              {totalRuntime < 60 ? `${totalRuntime}m` : `${(totalRuntime / 60).toFixed(1)}h`}
            </div>
          </div>
          <div className="bg-gray-50 dark:bg-gray-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Cost</div>
            <div className="text-sm font-bold text-gray-900 dark:text-dark-text-primary">${totalCost.toFixed(2)}</div>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-auto p-3">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <RefreshCw className="animate-spin text-blue-500" size={24} />
          </div>
        ) : manifolds.length === 0 ? (
          <div className="text-center py-8">
            <Server className="text-gray-400 dark:text-gray-600 mx-auto mb-2" size={32} />
            <p className="text-sm text-gray-600 dark:text-dark-text-secondary">No IAC Manifolds</p>
            <p className="text-xs text-gray-500 dark:text-dark-text-secondary mt-1">
              Manifolds will appear when spawned
            </p>
          </div>
        ) : (
          <div className="space-y-2">
            {manifolds.map((manifold) => (
              <div
                key={manifold.id}
                className="bg-white dark:bg-dark-elevated rounded-lg border border-gray-200 dark:border-dark-border p-3"
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="text-sm font-semibold text-gray-900 dark:text-dark-text-primary truncate">
                        {getManifoldTypeLabel(manifold.manifold_type)}
                      </h4>
                      <span className={`px-1.5 py-0.5 text-2xs font-medium rounded border ${getStatusColor(manifold.current_status ?? 'dormant')}`}>
                        {manifold.current_status ?? 'dormant'}
                      </span>
                    </div>
                    <p className="text-2xs text-gray-500 dark:text-dark-text-secondary truncate">
                      {manifold.manifold_id}
                    </p>
                  </div>
                  <div className="flex gap-1">
                    {manifold.current_status === 'dormant' && (
                      <button className="p-1 text-green-600 dark:text-green-400 hover:bg-green-900/20 rounded" title="Spawn">
                        <PlayCircle size={14} />
                      </button>
                    )}
                    {manifold.current_status === 'active' && (
                      <button className="p-1 text-red-600 dark:text-red-400 hover:bg-red-900/20 rounded" title="Teardown">
                        <StopCircle size={14} />
                      </button>
                    )}
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-2 text-2xs">
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">GPU:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {manifold.gpu_allocation}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">Memory:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {manifold.memory_gb}GB
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">Runtime:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {manifold.total_runtime_minutes}m
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">Cost:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      ${(manifold.total_cost ?? 0).toFixed(2)}
                    </span>
                  </div>
                </div>

                {manifold.spawn_time_ms && manifold.spawn_time_ms > 30000 && (
                  <div className="mt-2 flex items-center gap-1 text-2xs text-orange-600 dark:text-orange-400">
                    <AlertCircle size={12} />
                    <span>Slow spawn: {manifold.spawn_time_ms}ms</span>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
