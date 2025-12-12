import React, { useState } from 'react';
import {
  Network,
  GitBranch,
  MapPin,
  Filter,
  Database,
  Layers,
  Eye,
  Shield,
  Server,
  Globe,
  Database as DatabaseIcon,
  Satellite
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export interface DataVisualizationToolbarProps {
  onNetworkView?: () => void;
  onTaskGraph?: () => void;
  onSectors?: () => void;
  onFilters?: () => void;
  onDataSources?: () => void;
  onGisLayers?: () => void;
  onOsintNodes?: () => void;
  onThreatIntel?: () => void;
  onInfrastructure?: () => void;
  onGeoIP?: () => void;
  onSupabase?: () => void;
  onSurrealDB?: () => void;
  onGEE?: () => void;
  enabledActions?: string[]; // Whitelist which actions to show
  className?: string;
  variant?: 'default' | 'compact' | 'minimal';
}

const actionConfig = {
  networkView: {
    label: 'Network View',
    icon: Network,
    description: 'Network topology visualization',
    defaultEnabled: true
  },
  taskGraph: {
    label: 'Task Graph',
    icon: GitBranch,
    description: 'Task dependency graph',
    defaultEnabled: true
  },
  sectors: {
    label: 'Sectors',
    icon: MapPin,
    description: 'Sector/region filtering',
    defaultEnabled: true
  },
  filters: {
    label: 'Filters',
    icon: Filter,
    description: 'Advanced filtering controls',
    defaultEnabled: true
  },
  dataSources: {
    label: 'Data Sources',
    icon: Database,
    description: 'Data source management',
    defaultEnabled: true
  },
  gisLayers: {
    label: 'GIS Layers',
    icon: Layers,
    description: 'Map layer toggles',
    defaultEnabled: true
  },
  osintNodes: {
    label: 'OSINT Nodes',
    icon: Eye,
    description: 'OSINT node visualization',
    defaultEnabled: true
  },
  threatIntel: {
    label: 'Threat Intel',
    icon: Shield,
    description: 'Threat intelligence display',
    defaultEnabled: true
  },
  infrastructure: {
    label: 'Infrastructure',
    icon: Server,
    description: 'Infrastructure view',
    defaultEnabled: true
  },
  geoIP: {
    label: 'GeoIP',
    icon: Globe,
    description: 'GeoIP lookup service',
    defaultEnabled: true
  },
  supabase: {
    label: 'Supabase',
    icon: DatabaseIcon,
    description: 'Supabase database connection',
    defaultEnabled: true
  },
  surrealdb: {
    label: 'SurrealDB',
    icon: DatabaseIcon,
    description: 'SurrealDB graph database',
    defaultEnabled: true
  },
  gee: {
    label: 'GEE (KMZ)',
    icon: Satellite,
    description: 'Google Earth Engine integration',
    defaultEnabled: true
  }
};

export const DataVisualizationToolbar: React.FC<DataVisualizationToolbarProps> = ({
  onNetworkView,
  onTaskGraph,
  onSectors,
  onFilters,
  onDataSources,
  onGisLayers,
  onOsintNodes,
  onThreatIntel,
  onInfrastructure,
  onGeoIP,
  onSupabase,
  onSurrealDB,
  onGEE,
  enabledActions,
  className = '',
  variant = 'default'
}) => {
  const [activeActions, setActiveActions] = useState<Set<string>>(new Set());

  // Determine which actions to show
  const visibleActions = enabledActions 
    ? Object.keys(actionConfig).filter(key => enabledActions.includes(key))
    : Object.keys(actionConfig);

  const handleAction = (actionKey: string, handler?: () => void) => {
    if (!handler) return;
    
    // Toggle active state
    const newActive = new Set(activeActions);
    if (newActive.has(actionKey)) {
      newActive.delete(actionKey);
    } else {
      newActive.add(actionKey);
    }
    setActiveActions(newActive);
    
    // Call handler
    handler();
  };

  const actions = {
    networkView: { handler: onNetworkView, key: 'networkView' },
    taskGraph: { handler: onTaskGraph, key: 'taskGraph' },
    sectors: { handler: onSectors, key: 'sectors' },
    filters: { handler: onFilters, key: 'filters' },
    dataSources: { handler: onDataSources, key: 'dataSources' },
    gisLayers: { handler: onGisLayers, key: 'gisLayers' },
    osintNodes: { handler: onOsintNodes, key: 'osintNodes' },
    threatIntel: { handler: onThreatIntel, key: 'threatIntel' },
    infrastructure: { handler: onInfrastructure, key: 'infrastructure' },
    geoIP: { handler: onGeoIP, key: 'geoIP' },
    supabase: { handler: onSupabase, key: 'supabase' },
    surrealdb: { handler: onSurrealDB, key: 'surrealdb' },
    gee: { handler: onGEE, key: 'gee' }
  };

  if (variant === 'minimal') {
    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="sm" className={className}>
            <Layers className="h-4 w-4 mr-2" />
            Visualization Tools
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-56">
          {visibleActions.map((actionKey) => {
            const config = actionConfig[actionKey as keyof typeof actionConfig];
            const action = actions[actionKey as keyof typeof actions];
            const Icon = config.icon;
            
            return (
              <DropdownMenuItem
                key={actionKey}
                onClick={() => handleAction(action.key, action.handler)}
                className={activeActions.has(action.key) ? 'bg-blue-600/20' : ''}
              >
                <Icon className="h-4 w-4 mr-2" />
                <span>{config.label}</span>
              </DropdownMenuItem>
            );
          })}
        </DropdownMenuContent>
      </DropdownMenu>
    );
  }

  if (variant === 'compact') {
    return (
      <div className={`flex flex-wrap gap-1 ${className}`}>
        {visibleActions.map((actionKey) => {
          const config = actionConfig[actionKey as keyof typeof actionConfig];
          const action = actions[actionKey as keyof typeof actions];
          const Icon = config.icon;
          const isActive = activeActions.has(action.key);
          
          return (
            <Button
              key={actionKey}
              variant={isActive ? 'default' : 'outline'}
              size="sm"
              onClick={() => handleAction(action.key, action.handler)}
              title={config.description}
              className="h-8 px-2"
            >
              <Icon className="h-3.5 w-3.5" />
            </Button>
          );
        })}
      </div>
    );
  }

  // Default variant - full toolbar
  return (
    <div className={`flex flex-wrap items-center gap-2 p-2 bg-gray-800 rounded-lg border border-gray-700 ${className}`}>
      {visibleActions.map((actionKey) => {
        const config = actionConfig[actionKey as keyof typeof actionConfig];
        const action = actions[actionKey as keyof typeof actions];
        const Icon = config.icon;
        const isActive = activeActions.has(action.key);
        
        return (
          <Button
            key={actionKey}
            variant={isActive ? 'default' : 'outline'}
            size="sm"
            onClick={() => handleAction(action.key, action.handler)}
            title={config.description}
            className="h-9"
          >
            <Icon className="h-4 w-4 mr-2" />
            <span>{config.label}</span>
          </Button>
        );
      })}
    </div>
  );
};

export default DataVisualizationToolbar;



