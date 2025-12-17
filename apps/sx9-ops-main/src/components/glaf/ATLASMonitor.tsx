import { useEffect, useState } from 'react';
import { Brain, AlertTriangle, RefreshCw } from 'lucide-react';
import type { ATLASNode } from '../types/rfc-9005.types';
import { globalEventBus } from '../lib/eventBus';
import { supabase } from '../lib/supabase';

export default function ATLASMonitor() {
  const [atlasNodes, setAtlasNodes] = useState<ATLASNode[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadATLASNodes();

    const unsubscribeStatus = globalEventBus.on('atlas:status_change', () => {
      loadATLASNodes();
    });

    return () => {
      unsubscribeStatus();
    };
  }, []);

  const loadATLASNodes = async () => {
    try {
      setLoading(true);
      const { data, error } = await supabase.from('atlas_nodes').select('*').order('created_at', { ascending: false });

      if (error) throw error;
      setAtlasNodes((data as ATLASNode[]) || []);
    } catch (err) {
      console.error('Error loading ATLAS nodes:', err);
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
      active: 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/30',
      burst: 'text-orange-600 bg-orange-100 dark:text-orange-400 dark:bg-orange-900/30',
      initializing: 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/30',
      dormant: 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-800/30',
      cooling: 'text-cyan-600 bg-cyan-100 dark:text-cyan-400 dark:bg-cyan-900/30',
      error: 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/30',
    };
    return colors[status] || 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-800/30';
  };

  const formatLatency = (us: number | null) => {
    if (!us) return 'N/A';
    if (us < 1000) return `${us.toFixed(1)}μs`;
    return `${(us / 1000).toFixed(2)}ms`;
  };

  const averageTickRate = atlasNodes.reduce((sum, node) => sum + (node.cognitive_tick_rate_us ?? 0), 0) / (atlasNodes.length || 1);
  const activeNodes = atlasNodes.filter((n) => n.node_status === 'active').length;
  const totalTicks = atlasNodes.reduce((sum, node) => sum + (node.ticks_processed ?? 0), 0);

  return (
    <div className="flex flex-col h-full">
      <div className="p-3 border-b border-gray-200 dark:border-dark-border space-y-3">
        <div className="grid grid-cols-2 gap-2">
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Nodes</div>
            <div className="text-lg font-bold text-gray-900 dark:text-dark-text-primary">{atlasNodes.length}</div>
          </div>
          <div className="bg-green-50 dark:bg-green-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Active</div>
            <div className="text-lg font-bold text-green-600 dark:text-green-400">{activeNodes}</div>
          </div>
          <div className="bg-orange-50 dark:bg-orange-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Total Ticks</div>
            <div className="text-sm font-bold text-gray-900 dark:text-dark-text-primary">{totalTicks.toLocaleString()}</div>
          </div>
          <div className="bg-gray-50 dark:bg-gray-900/20 rounded p-2">
            <div className="text-2xs text-gray-600 dark:text-dark-text-secondary">Avg Rate</div>
            <div className="text-sm font-bold text-gray-900 dark:text-dark-text-primary">{formatLatency(averageTickRate)}</div>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-auto p-3">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <RefreshCw className="animate-spin text-blue-500" size={24} />
          </div>
        ) : atlasNodes.length === 0 ? (
          <div className="text-center py-8">
            <Brain className="text-gray-400 dark:text-gray-600 mx-auto mb-2" size={32} />
            <p className="text-sm text-gray-600 dark:text-dark-text-secondary">No ATLAS Nodes</p>
            <p className="text-xs text-gray-500 dark:text-dark-text-secondary mt-1">
              Nodes will appear when created
            </p>
          </div>
        ) : (
          <div className="space-y-2">
            {atlasNodes.map((node) => (
              <div
                key={node.id}
                className="bg-white dark:bg-dark-elevated rounded-lg border border-gray-200 dark:border-dark-border p-3"
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="text-sm font-semibold text-gray-900 dark:text-dark-text-primary truncate">
                        {node.entity_id.substring(0, 12)}...
                      </h4>
                      <span className={`px-1.5 py-0.5 text-2xs font-medium rounded ${getStatusColor(node.node_status ?? 'dormant')}`}>
                        {node.node_status ?? 'dormant'}
                      </span>
                    </div>
                    <div className="text-2xs text-gray-500 dark:text-dark-text-secondary">
                      {(node.ticks_processed ?? 0).toLocaleString()} ticks
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-2 text-2xs mb-2">
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">Tick Rate:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {formatLatency(node.cognitive_tick_rate_us ?? 0)}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">Mux Latency:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {node.neural_mux_latency_ns ?? 0}ns
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">CUDA:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {node.cuda_compute_slots ?? 0}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-dark-text-secondary">IAC:</span>
                    <span className="ml-1 font-medium text-gray-900 dark:text-dark-text-primary">
                      {(node.iac_manifold_capabilities ?? []).length}
                    </span>
                  </div>
                </div>

                {(node.neural_mux_latency_ns ?? 0) > 250 && (
                  <div className="flex items-center gap-1 text-2xs text-orange-600 dark:text-orange-400">
                    <AlertTriangle size={10} />
                    <span>High latency</span>
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
