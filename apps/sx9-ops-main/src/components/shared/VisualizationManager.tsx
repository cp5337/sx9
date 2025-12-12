import React, { useState } from 'react';
import { DataVisualizationToolbar } from './toolbars/DataVisualizationToolbar';
import { NetworkView } from './visualizations/NetworkView';
import { TaskGraph } from './visualizations/TaskGraph';
import { OSINTNodes } from './visualizations/OSINTNodes';
import { DatabaseConnectionPanel } from './toolbars/DatabaseConnectionPanel';
import { FilterPanel } from './toolbars/FilterPanel';
import { X } from 'lucide-react';
import { Button } from '@/components/ui/button';

type ActiveView = 'network' | 'taskgraph' | 'sectors' | 'filters' | 'datasources' | 'gislayers' | 'osint' | 'threatintel' | 'infrastructure' | 'geoip' | 'supabase' | 'surrealdb' | 'gee' | null;

interface VisualizationManagerProps {
  hd4Phase?: string;
  className?: string;
  enabledActions?: string[];
}

/**
 * VisualizationManager - Central component that manages all visualization views
 * Wires up the DataVisualizationToolbar to actual component implementations
 */
export const VisualizationManager: React.FC<VisualizationManagerProps> = ({
  hd4Phase,
  className = '',
  enabledActions
}) => {
  const [activeView, setActiveView] = useState<ActiveView>(null);

  const handleNetworkView = () => {
    setActiveView(activeView === 'network' ? null : 'network');
  };

  const handleTaskGraph = () => {
    setActiveView(activeView === 'taskgraph' ? null : 'taskgraph');
  };

  const handleSectors = () => {
    setActiveView(activeView === 'sectors' ? null : 'sectors');
  };

  const handleFilters = () => {
    setActiveView(activeView === 'filters' ? null : 'filters');
  };

  const handleDataSources = () => {
    setActiveView(activeView === 'datasources' ? null : 'datasources');
  };

  const handleGisLayers = () => {
    setActiveView(activeView === 'gislayers' ? null : 'gislayers');
  };

  const handleOsintNodes = () => {
    setActiveView(activeView === 'osint' ? null : 'osint');
  };

  const handleThreatIntel = () => {
    setActiveView(activeView === 'threatintel' ? null : 'threatintel');
  };

  const handleInfrastructure = () => {
    setActiveView(activeView === 'infrastructure' ? null : 'infrastructure');
  };

  const handleGeoIP = () => {
    setActiveView(activeView === 'geoip' ? null : 'geoip');
  };

  const handleSupabase = () => {
    setActiveView(activeView === 'supabase' ? null : 'supabase');
  };

  const handleSurrealDB = () => {
    setActiveView(activeView === 'surrealdb' ? null : 'surrealdb');
  };

  const handleGEE = () => {
    setActiveView(activeView === 'gee' ? null : 'gee');
  };

  const closeView = () => {
    setActiveView(null);
  };

  return (
    <div className={className}>
      {/* Toolbar */}
      <div className="mb-4">
        <DataVisualizationToolbar
          onNetworkView={handleNetworkView}
          onTaskGraph={handleTaskGraph}
          onSectors={handleSectors}
          onFilters={handleFilters}
          onDataSources={handleDataSources}
          onGisLayers={handleGisLayers}
          onOsintNodes={handleOsintNodes}
          onThreatIntel={handleThreatIntel}
          onInfrastructure={handleInfrastructure}
          onGeoIP={handleGeoIP}
          onSupabase={handleSupabase}
          onSurrealDB={handleSurrealDB}
          onGEE={handleGEE}
          enabledActions={enabledActions || undefined}
        />
      </div>

      {/* Active View Display */}
      {activeView && (
        <div className="mt-4 relative">
          <div className="flex items-center justify-between mb-2">
            <h3 className="text-xs font-mono uppercase tracking-wide text-gray-400">
              {activeView === 'network' && 'Network Topology'}
              {activeView === 'taskgraph' && 'Task Dependency Graph'}
              {activeView === 'sectors' && 'Sector Filter'}
              {activeView === 'filters' && 'Advanced Filters'}
              {activeView === 'datasources' && 'Data Sources'}
              {activeView === 'gislayers' && 'GIS Layers'}
              {activeView === 'osint' && 'OSINT Nodes'}
              {activeView === 'threatintel' && 'Threat Intelligence'}
              {activeView === 'infrastructure' && 'Infrastructure View'}
              {activeView === 'geoip' && 'GeoIP Lookup'}
              {activeView === 'supabase' && 'Supabase Connection'}
              {activeView === 'surrealdb' && 'SurrealDB Connection'}
              {activeView === 'gee' && 'Google Earth Engine'}
            </h3>
            <Button
              variant="ghost"
              size="sm"
              onClick={closeView}
              className="h-6 w-6 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>

          {/* Render active view */}
          {activeView === 'network' && (
            <NetworkView onNodeClick={(nodeId) => console.log('Node clicked:', nodeId)} />
          )}

          {activeView === 'taskgraph' && (
            <TaskGraph hd4Phase={hd4Phase || undefined} onTaskClick={(taskId) => console.log('Task clicked:', taskId)} />
          )}

          {activeView === 'filters' && (
            <FilterPanel />
          )}

          {activeView === 'supabase' && (
            <DatabaseConnectionPanel databases={[{ id: 'supabase', name: 'Supabase', type: 'supabase', status: 'disconnected' }]} />
          )}

          {activeView === 'surrealdb' && (
            <DatabaseConnectionPanel databases={[{ id: 'surrealdb', name: 'SurrealDB', type: 'surrealdb', status: 'disconnected' }]} />
          )}

          {activeView === 'osint' && (
            <OSINTNodes onNodeClick={(nodeId) => console.log('OSINT node clicked:', nodeId)} useNeo4j={true} />
          )}

          {/* Placeholder views for other actions */}
          {(activeView === 'sectors' || 
            activeView === 'datasources' || 
            activeView === 'gislayers' || 
            activeView === 'threatintel' || 
            activeView === 'infrastructure' || 
            activeView === 'geoip' || 
            activeView === 'gee') && (
            <div className="p-8 bg-gray-900 border border-gray-800 rounded text-center">
              <p className="text-sm text-gray-500 font-mono">
                {activeView.toUpperCase()} view coming soon
              </p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default VisualizationManager;

