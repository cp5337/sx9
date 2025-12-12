import React from 'react';
import { DataVisualizationToolbar, DatabaseConnectionPanel, FilterPanel } from '../index';

/**
 * Example usage of Meta Components
 * 
 * These components can be used across all pages instead of programming
 * button-by-button. They provide consistent UI and functionality.
 */
export const MetaComponentsExample: React.FC = () => {
  return (
    <div className="p-6 space-y-6 bg-gray-900 text-white">
      <div>
        <h2 className="text-2xl font-bold mb-4">Meta Components Example</h2>
        <p className="text-gray-400 mb-6">
          These reusable components eliminate the need to program buttons individually.
          Use them across all pages for consistent functionality.
        </p>
      </div>

      {/* Data Visualization Toolbar */}
      <section>
        <h3 className="text-lg font-semibold mb-2">Data Visualization Toolbar</h3>
        <p className="text-sm text-gray-400 mb-4">
          All 13 common actions that appear on every page. Use this instead of creating
          individual buttons for Network View, Task Graph, Filters, etc.
        </p>
        
        <div className="space-y-4">
          <div>
            <p className="text-xs text-gray-500 mb-2">Default (Full Toolbar)</p>
            <DataVisualizationToolbar
              onNetworkView={() => console.log('Network View clicked')}
              onTaskGraph={() => console.log('Task Graph clicked')}
              onFilters={() => console.log('Filters clicked')}
            />
          </div>

          <div>
            <p className="text-xs text-gray-500 mb-2">Compact (Icon Only)</p>
            <DataVisualizationToolbar
              variant="compact"
              enabledActions={['networkView', 'taskGraph', 'filters', 'gisLayers']}
              onNetworkView={() => console.log('Network View clicked')}
              onTaskGraph={() => console.log('Task Graph clicked')}
            />
          </div>

          <div>
            <p className="text-xs text-gray-500 mb-2">Minimal (Dropdown)</p>
            <DataVisualizationToolbar
              variant="minimal"
              onNetworkView={() => console.log('Network View clicked')}
            />
          </div>
        </div>
      </section>

      {/* Database Connection Panel */}
      <section>
        <h3 className="text-lg font-semibold mb-2">Database Connection Panel</h3>
        <p className="text-sm text-gray-400 mb-4">
          Unified database connection management. Shows connection status and
          provides connect/disconnect functionality.
        </p>
        
        <div className="space-y-4">
          <div>
            <p className="text-xs text-gray-500 mb-2">Full Panel</p>
            <DatabaseConnectionPanel
              onConnect={(dbId) => console.log(`Connecting to ${dbId}`)}
              onDisconnect={(dbId) => console.log(`Disconnecting from ${dbId}`)}
            />
          </div>

          <div>
            <p className="text-xs text-gray-500 mb-2">Compact (Icon Buttons)</p>
            <DatabaseConnectionPanel
              compact
              onConnect={(dbId) => console.log(`Connecting to ${dbId}`)}
            />
          </div>
        </div>
      </section>

      {/* Filter Panel */}
      <section>
        <h3 className="text-lg font-semibold mb-2">Filter Panel</h3>
        <p className="text-sm text-gray-400 mb-4">
          Advanced filtering with sectors, HD4 phases, priorities, and search.
          Use this instead of creating custom filter UIs.
        </p>
        
        <div className="space-y-4">
          <div>
            <p className="text-xs text-gray-500 mb-2">Full Panel</p>
            <FilterPanel
              onFilterChange={(filters) => console.log('Filters changed:', filters)}
              onSectorChange={(sectors) => console.log('Sectors changed:', sectors)}
              onSearchChange={(query) => console.log('Search:', query)}
            />
          </div>

          <div>
            <p className="text-xs text-gray-500 mb-2">Compact (Popover)</p>
            <FilterPanel
              compact
              onFilterChange={(filters) => console.log('Filters changed:', filters)}
            />
          </div>
        </div>
      </section>

      {/* Usage Instructions */}
      <section className="mt-8 p-4 bg-blue-900/20 border border-blue-700 rounded-lg">
        <h3 className="text-lg font-semibold mb-2">How to Use</h3>
        <div className="text-sm text-gray-300 space-y-2">
          <p><strong>1. Import the components:</strong></p>
          <pre className="bg-gray-800 p-2 rounded text-xs overflow-x-auto">
{`import { 
  DataVisualizationToolbar, 
  DatabaseConnectionPanel, 
  FilterPanel 
} from '@/components/shared';`}
          </pre>

          <p className="mt-4"><strong>2. Add to your page:</strong></p>
          <pre className="bg-gray-800 p-2 rounded text-xs overflow-x-auto">
{`<DataVisualizationToolbar
  onNetworkView={() => handleNetworkView()}
  onTaskGraph={() => handleTaskGraph()}
  variant="compact" // or "default" or "minimal"
/>`}
          </pre>

          <p className="mt-4"><strong>3. Benefits:</strong></p>
          <ul className="list-disc list-inside space-y-1 ml-4">
            <li>No need to program 13 buttons individually</li>
            <li>Consistent UI across all pages</li>
            <li>Easy to enable/disable specific actions</li>
            <li>Multiple display variants (full, compact, minimal)</li>
            <li>Built-in state management</li>
          </ul>
        </div>
      </section>
    </div>
  );
};

export default MetaComponentsExample;



