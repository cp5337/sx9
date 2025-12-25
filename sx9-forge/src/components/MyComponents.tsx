/**
 * My Components Panel - Lightweight component status view for Forge
 *
 * Shows what's unlocked for the user's license tier.
 * Links to sx9-ops-main marketplace for upgrades.
 */

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

// Component access status from gateway
type AccessStatus = 'available' | 'loaded' | 'upgrade' | 'pending_heartbeat';

interface ComponentInfo {
  id: string;
  name: string;
  description: string;
  category: string;
  required_tier: string;
  version: string;
  wasm_size: number | null;
  requires_heartbeat: boolean;
  icon: string;
  capabilities: string[];
  access_status: AccessStatus;
}

interface LicenseInfo {
  valid: boolean;
  tier: string;
  tier_level: number;
  days_remaining: number | null;
  warning: string | null;
}

// Category icons (using text/emoji for simplicity, could swap for lucide-react)
const CATEGORY_ICONS: Record<string, string> = {
  toolbars: '🔧',
  infrastructure: '🏗️',
  admin: '⚙️',
  redteam: '🎯',
  analysis: '📊',
  visualization: '🧠',
  maps: '🗺️',
  intel: '🛡️',
  ops: '⚡',
};

// Tier colors
const TIER_COLORS: Record<string, string> = {
  free: 'text-gray-400 border-gray-600',
  pro: 'text-blue-400 border-blue-600',
  enterprise: 'text-purple-400 border-purple-600',
  government: 'text-green-400 border-green-600',
};

// Status badges
const STATUS_CONFIG: Record<AccessStatus, { label: string; color: string; icon: string }> = {
  available: { label: 'Available', color: 'bg-green-900/50 text-green-400', icon: '✓' },
  loaded: { label: 'Loaded', color: 'bg-blue-900/50 text-blue-400', icon: '●' },
  upgrade: { label: 'Upgrade', color: 'bg-yellow-900/50 text-yellow-400', icon: '↑' },
  pending_heartbeat: { label: 'Validating', color: 'bg-orange-900/50 text-orange-400', icon: '◐' },
};

// Mock data for development (gateway not connected)
const MOCK_COMPONENTS: ComponentInfo[] = [
  {
    id: 'filter-panel',
    name: 'Filter Panel',
    description: 'Advanced filtering with sectors/phases',
    category: 'toolbars',
    required_tier: 'free',
    version: '1.0.0',
    wasm_size: null,
    requires_heartbeat: false,
    icon: 'Search',
    capabilities: ['filtering', 'search'],
    access_status: 'available',
  },
  {
    id: 'db-panel',
    name: 'Database Panel',
    description: 'Unified database management',
    category: 'infrastructure',
    required_tier: 'free',
    version: '1.0.0',
    wasm_size: null,
    requires_heartbeat: false,
    icon: 'Database',
    capabilities: ['supabase', 'neon', 'sled'],
    access_status: 'loaded',
  },
  {
    id: 'cognigraph',
    name: 'Cognigraph',
    description: 'Cognitive graph visualization',
    category: 'visualization',
    required_tier: 'pro',
    version: '2.0.0',
    wasm_size: 2300000,
    requires_heartbeat: true,
    icon: 'Brain',
    capabilities: ['graph', 'visualization', 'glaf'],
    access_status: 'available',
  },
  {
    id: 'hash-composer',
    name: 'Hash Composer',
    description: 'Trivariate hash composition & analysis',
    category: 'analysis',
    required_tier: 'pro',
    version: '1.5.0',
    wasm_size: 1200000,
    requires_heartbeat: true,
    icon: 'Terminal',
    capabilities: ['hashing', 'analysis', 'trivariate'],
    access_status: 'available',
  },
  {
    id: 'redteam-runner',
    name: 'Red Team Runner',
    description: 'Automated red team operations',
    category: 'redteam',
    required_tier: 'enterprise',
    version: '3.0.0',
    wasm_size: 5600000,
    requires_heartbeat: true,
    icon: 'Target',
    capabilities: ['redteam', 'automation', 'atomic'],
    access_status: 'upgrade',
  },
  {
    id: 'kali-tools',
    name: 'Kali Tools',
    description: 'Full Kali Linux tool launcher',
    category: 'ops',
    required_tier: 'enterprise',
    version: '2.1.0',
    wasm_size: null,
    requires_heartbeat: true,
    icon: 'Server',
    capabilities: ['kali', 'pentest', 'tools'],
    access_status: 'upgrade',
  },
];

const MOCK_LICENSE: LicenseInfo = {
  valid: true,
  tier: 'pro',
  tier_level: 1,
  days_remaining: 45,
  warning: null,
};

interface MyComponentsProps {
  onClose?: () => void;
  marketplaceUrl?: string;
}

export default function MyComponents({
  onClose,
  marketplaceUrl = 'https://ops.synaptix9.com/gallery'
}: MyComponentsProps) {
  const [components, setComponents] = useState<ComponentInfo[]>([]);
  const [license, setLicense] = useState<LicenseInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<'all' | 'available' | 'locked'>('all');
  const [gatewayConnected, setGatewayConnected] = useState(false);

  // Fetch components from gateway
  const fetchComponents = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      // Try to connect to gateway
      // For now, use mock data since gateway may not be running
      // In production: const result = await invoke('get_components');

      // Simulate gateway check
      const gatewayAvailable = false; // TODO: Check actual gateway

      if (gatewayAvailable) {
        // Real gateway call would go here
        // const result = await invoke('gateway_get_components');
        // setComponents(result.components);
        // setLicense(result.license);
        setGatewayConnected(true);
      } else {
        // Use mock data for development
        setComponents(MOCK_COMPONENTS);
        setLicense(MOCK_LICENSE);
        setGatewayConnected(false);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch components');
      setComponents(MOCK_COMPONENTS);
      setLicense(MOCK_LICENSE);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchComponents();
  }, [fetchComponents]);

  // Filter components
  const filteredComponents = components.filter(comp => {
    if (filter === 'all') return true;
    if (filter === 'available') return comp.access_status === 'available' || comp.access_status === 'loaded';
    if (filter === 'locked') return comp.access_status === 'upgrade' || comp.access_status === 'pending_heartbeat';
    return true;
  });

  // Group by category
  const groupedComponents = filteredComponents.reduce((acc, comp) => {
    if (!acc[comp.category]) acc[comp.category] = [];
    acc[comp.category].push(comp);
    return acc;
  }, {} as Record<string, ComponentInfo[]>);

  // Format file size
  const formatSize = (bytes: number | null) => {
    if (!bytes) return null;
    if (bytes < 1024) return `${bytes}B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)}MB`;
  };

  // Open marketplace
  const openMarketplace = () => {
    // Open in default browser
    window.open(marketplaceUrl, '_blank');
  };

  return (
    <div className="h-full flex flex-col bg-gray-900 text-gray-100">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
        <div className="flex items-center gap-3">
          <span className="text-lg font-semibold">My Components</span>
          {license && (
            <span className={`text-xs px-2 py-0.5 rounded border ${TIER_COLORS[license.tier] || TIER_COLORS.free}`}>
              {license.tier.toUpperCase()}
            </span>
          )}
          {!gatewayConnected && (
            <span className="text-xs px-2 py-0.5 rounded bg-gray-800 text-gray-500">
              Offline Mode
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          {license?.days_remaining && license.days_remaining < 30 && (
            <span className="text-xs text-yellow-400">
              {license.days_remaining} days remaining
            </span>
          )}
          {onClose && (
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-white p-1"
            >
              ✕
            </button>
          )}
        </div>
      </div>

      {/* Filter tabs */}
      <div className="flex items-center gap-1 px-4 py-2 border-b border-gray-800">
        {(['all', 'available', 'locked'] as const).map(f => (
          <button
            key={f}
            onClick={() => setFilter(f)}
            className={`px-3 py-1 text-xs rounded transition-colors ${
              filter === f
                ? 'bg-blue-600 text-white'
                : 'text-gray-400 hover:bg-gray-800 hover:text-white'
            }`}
          >
            {f.charAt(0).toUpperCase() + f.slice(1)}
            <span className="ml-1 opacity-60">
              ({f === 'all'
                ? components.length
                : f === 'available'
                  ? components.filter(c => c.access_status === 'available' || c.access_status === 'loaded').length
                  : components.filter(c => c.access_status === 'upgrade' || c.access_status === 'pending_heartbeat').length
              })
            </span>
          </button>
        ))}
        <div className="flex-1" />
        <button
          onClick={openMarketplace}
          className="px-3 py-1 text-xs rounded bg-purple-600 hover:bg-purple-500 text-white flex items-center gap-1"
        >
          <span>Marketplace</span>
          <span>↗</span>
        </button>
      </div>

      {/* Component list */}
      <div className="flex-1 overflow-y-auto p-4">
        {loading ? (
          <div className="flex items-center justify-center h-32 text-gray-500">
            Loading components...
          </div>
        ) : error ? (
          <div className="flex flex-col items-center justify-center h-32 text-red-400">
            <span>{error}</span>
            <button
              onClick={fetchComponents}
              className="mt-2 text-xs text-blue-400 hover:underline"
            >
              Retry
            </button>
          </div>
        ) : filteredComponents.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-gray-500">
            No components found
          </div>
        ) : (
          <div className="space-y-6">
            {Object.entries(groupedComponents).map(([category, comps]) => (
              <div key={category}>
                {/* Category header */}
                <div className="flex items-center gap-2 mb-2 text-sm text-gray-400">
                  <span>{CATEGORY_ICONS[category] || '📦'}</span>
                  <span className="capitalize">{category}</span>
                  <span className="text-xs opacity-60">({comps.length})</span>
                </div>

                {/* Component cards */}
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-2">
                  {comps.map(comp => {
                    const status = STATUS_CONFIG[comp.access_status];
                    const isLocked = comp.access_status === 'upgrade' || comp.access_status === 'pending_heartbeat';

                    return (
                      <div
                        key={comp.id}
                        className={`p-3 rounded border transition-colors ${
                          isLocked
                            ? 'bg-gray-800/50 border-gray-700 opacity-75'
                            : 'bg-gray-800 border-gray-700 hover:border-gray-600'
                        }`}
                      >
                        {/* Card header */}
                        <div className="flex items-start justify-between mb-2">
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2">
                              <span className="font-medium text-sm truncate">{comp.name}</span>
                              <span className={`text-[10px] px-1.5 py-0.5 rounded ${status.color}`}>
                                {status.icon} {status.label}
                              </span>
                            </div>
                            <p className="text-xs text-gray-500 mt-0.5 truncate">{comp.description}</p>
                          </div>
                        </div>

                        {/* Card details */}
                        <div className="flex items-center gap-3 text-[10px] text-gray-500">
                          <span>v{comp.version}</span>
                          <span className={`px-1 py-0.5 rounded border ${TIER_COLORS[comp.required_tier] || TIER_COLORS.free}`}>
                            {comp.required_tier}
                          </span>
                          {comp.wasm_size && (
                            <span>{formatSize(comp.wasm_size)}</span>
                          )}
                          {comp.requires_heartbeat && (
                            <span title="Requires heartbeat validation">💓</span>
                          )}
                        </div>

                        {/* Capabilities */}
                        {comp.capabilities.length > 0 && (
                          <div className="flex flex-wrap gap-1 mt-2">
                            {comp.capabilities.slice(0, 3).map(cap => (
                              <span
                                key={cap}
                                className="text-[9px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-400"
                              >
                                {cap}
                              </span>
                            ))}
                            {comp.capabilities.length > 3 && (
                              <span className="text-[9px] text-gray-500">
                                +{comp.capabilities.length - 3}
                              </span>
                            )}
                          </div>
                        )}

                        {/* Upgrade prompt for locked components */}
                        {isLocked && (
                          <button
                            onClick={openMarketplace}
                            className="mt-2 w-full text-xs py-1 rounded bg-yellow-900/30 text-yellow-400 hover:bg-yellow-900/50 transition-colors"
                          >
                            Upgrade to {comp.required_tier}
                          </button>
                        )}
                      </div>
                    );
                  })}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="px-4 py-2 border-t border-gray-800 text-xs text-gray-500 flex items-center justify-between">
        <span>
          {components.filter(c => c.access_status === 'available' || c.access_status === 'loaded').length} / {components.length} components available
        </span>
        <button
          onClick={fetchComponents}
          className="text-gray-400 hover:text-white"
          title="Refresh"
        >
          ↻ Refresh
        </button>
      </div>
    </div>
  );
}
