import React from 'react';
import { Layers, ChevronLeft, ChevronRight, Network, Filter, Database, Globe, Activity, GitBranch, Box, Hammer } from 'lucide-react';
import { useRightPanel } from '../contexts/RightPanelContext';
import { useNavigate } from 'react-router-dom';

interface RightPanelProps {
  // No props needed for now - Global Views only
}

const RightPanel: React.FC<RightPanelProps> = () => {
  const { isOpen, setIsOpen, isCollapsed, setIsCollapsed } = useRightPanel();
  const navigate = useNavigate();
  const [activeDrawer, setActiveDrawer] = React.useState<string | null>(null);

  return (
    <div className={`${isCollapsed ? 'w-12' : 'w-36'} h-full bg-gray-800 text-gray-300 fixed right-0 top-0 overflow-y-auto transition-all duration-300 border-l border-gray-700`}>
      {/* Collapse Toggle - Top Edge */}
      <div className="absolute top-2 left-2 z-50">
        <button
          onClick={() => setIsCollapsed(!isCollapsed)}
          className="text-gray-400 hover:text-white p-1 rounded hover:bg-gray-700 bg-gray-800 border border-gray-700"
          title={isCollapsed ? 'Expand Panel' : 'Collapse Panel'}
        >
          {isCollapsed ? <ChevronLeft size={14} /> : <ChevronRight size={14} />}
        </button>
      </div>

      <div className="p-4 pt-12">
        {!isCollapsed ? (
          <div>
            <h2 className="text-base font-bold">Layers</h2>
          </div>
        ) : (
          <div className="flex justify-center">
            <Layers size={16} className="text-gray-400" />
          </div>
        )}
      </div>

          {/* Global Views Section */}
          {!isCollapsed && (
            <div className="px-3 pb-3 border-b border-gray-700">
              <div className="text-xs text-gray-500 uppercase font-semibold mb-2">Global Views</div>
          <div className="space-y-1 mb-3">
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded bg-gray-700/50">
              <Network size={14} className="mr-2 text-cyan-400" />
              <span>Network View</span>
            </button>
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Activity size={14} className="mr-2 text-yellow-400" />
              <span>Task Graph</span>
            </button>
          </div>
          <div className="border-t border-gray-700 my-3"></div>
          <div className="text-xs text-gray-500 uppercase font-semibold mb-2">Tools</div>
          <div className="space-y-1">
            <button 
              onClick={() => navigate('/sectors')}
              className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded"
            >
              <Filter size={14} className="mr-2" />
              <span>Sectors</span>
            </button>
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Filter size={14} className="mr-2" />
              <span>Filters</span>
            </button>
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Database size={14} className="mr-2" />
              <span>Data Sources</span>
            </button>
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Globe size={14} className="mr-2" />
              <span>GIS Layers</span>
            </button>
          </div>
        </div>
      )}
      
      {!isCollapsed && (
        <nav className="mt-2 px-3">
          <div className="space-y-2">
            <div className="text-xs text-gray-500 uppercase font-semibold mb-2">Map Layers</div>
            
            <button 
              onClick={() => setActiveDrawer(activeDrawer === 'osint' ? null : 'osint')}
              className={`w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded ${activeDrawer === 'osint' ? 'bg-blue-900/50 text-blue-300' : ''}`}
            >
              <Layers size={14} className="mr-2" />
              <span>OSINT Nodes</span>
            </button>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Layers size={14} className="mr-2" />
              <span>Threat Intel</span>
            </button>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Layers size={14} className="mr-2" />
              <span>Infrastructure</span>
            </button>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Layers size={14} className="mr-2" />
              <span>GeoIP</span>
            </button>
            
            <div className="border-t border-gray-700 my-2"></div>
            
            <div className="text-xs text-gray-500 uppercase font-semibold mb-2">Data</div>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Database size={14} className="mr-2" />
              <span>Supabase</span>
            </button>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Database size={14} className="mr-2" />
              <span>SurrealDB</span>
            </button>
            
            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Database size={14} className="mr-2" />
              <span>GEE (KMZ)</span>
            </button>

            <div className="border-t border-gray-700 my-2"></div>

            <div className="text-xs text-gray-500 uppercase font-semibold mb-2">GLAF / Forge</div>

            <button
              onClick={() => setActiveDrawer(activeDrawer === 'glaf' ? null : 'glaf')}
              className={`w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded ${activeDrawer === 'glaf' ? 'bg-purple-900/50 text-purple-300' : ''}`}
            >
              <GitBranch size={14} className="mr-2 text-purple-400" />
              <span>System Graph</span>
            </button>

            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Box size={14} className="mr-2 text-orange-400" />
              <span>Entity Model</span>
            </button>

            <button className="w-full flex items-center px-2 py-1.5 text-xs hover:bg-gray-700 rounded">
              <Hammer size={14} className="mr-2 text-cyan-400" />
              <span>Forge Builder</span>
            </button>
          </div>
        </nav>
      )}

      {/* GLAF System Drawer */}
      {activeDrawer === 'glaf' && !isCollapsed && (
        <div className="fixed right-36 top-0 h-full w-64 bg-gray-900 border-l border-gray-700 shadow-2xl z-40 overflow-y-auto">
          <div className="p-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-bold text-white">GLAF / Forge</h3>
              <button
                onClick={() => setActiveDrawer(null)}
                className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
              >
                <ChevronRight size={14} />
              </button>
            </div>
            <div className="space-y-2">
              <button
                onClick={() => navigate('/glaf-browser')}
                className="w-full p-2 bg-purple-900/30 text-purple-300 rounded hover:bg-purple-900/50 text-xs text-left"
              >
                <div className="font-medium">GLAF Browser</div>
                <div className="text-xxs text-gray-400">ctas7-glaf-browser</div>
              </button>
              <button className="w-full p-2 bg-orange-900/30 text-orange-300 rounded hover:bg-orange-900/50 text-xs text-left">
                <div className="font-medium">Intel System</div>
                <div className="text-xxs text-gray-400">ctas7-intel-system</div>
              </button>
              <button className="w-full p-2 bg-cyan-900/30 text-cyan-300 rounded hover:bg-cyan-900/50 text-xs text-left">
                <div className="font-medium">Forge Builder</div>
                <div className="text-xxs text-gray-400">Entity graph composer</div>
              </button>
            </div>
          </div>
        </div>
      )}

      {/* OSINT Primitives Drawer */}
      {activeDrawer === 'osint' && !isCollapsed && (
        <div className="fixed right-36 top-0 h-full w-64 bg-gray-900 border-l border-gray-700 shadow-2xl z-40 overflow-y-auto">
          <div className="p-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-bold text-white">OSINT Primitives</h3>
              <button
                onClick={() => setActiveDrawer(null)}
                className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
              >
                <ChevronRight size={14} />
              </button>
            </div>

            <div className="space-y-2">
              {/* Task - PRIMARY (what we observe) */}
              <div className="p-3 bg-gray-800 rounded border border-blue-600 hover:border-blue-400 transition-colors cursor-pointer">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">🎯</span>
                  <span className="text-sm font-semibold text-blue-400">Task</span>
                  <span className="text-xxs px-1 py-0.5 bg-blue-900/50 text-blue-300 rounded">Primary</span>
                </div>
                <p className="text-xxs text-gray-400">Actions, operations, procedures (Mandatory/Desirable/Optional)</p>
              </div>

              {/* Actor - SOLVE FOR (who we're hunting) */}
              <div className="p-3 bg-gray-800 rounded border border-gray-700 hover:border-red-500 transition-colors cursor-pointer">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">👤</span>
                  <span className="text-sm font-semibold text-red-400">Actor</span>
                  <span className="text-xxs px-1 py-0.5 bg-red-900/50 text-red-300 rounded">Solve For</span>
                </div>
                <p className="text-xxs text-gray-400">People, groups, entities performing tasks</p>
              </div>

              {/* Object */}
              <div className="p-3 bg-gray-800 rounded border border-gray-700 hover:border-green-500 transition-colors cursor-pointer">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">📦</span>
                  <span className="text-sm font-semibold text-green-400">Object</span>
                </div>
                <p className="text-xxs text-gray-400">Tools, weapons, resources, infrastructure</p>
              </div>

              {/* Event */}
              <div className="p-3 bg-gray-800 rounded border border-gray-700 hover:border-yellow-500 transition-colors cursor-pointer">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">⚡</span>
                  <span className="text-sm font-semibold text-yellow-400">Event</span>
                </div>
                <p className="text-xxs text-gray-400">Incidents, occurrences, temporal markers</p>
              </div>

              {/* Attribute */}
              <div className="p-3 bg-gray-800 rounded border border-gray-700 hover:border-cyan-500 transition-colors cursor-pointer">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">🏷️</span>
                  <span className="text-sm font-semibold text-cyan-400">Attribute</span>
                </div>
                <p className="text-xxs text-gray-400">Properties, indicators, classification metadata</p>
              </div>
            </div>

            <div className="mt-4 p-2 bg-blue-900/20 border border-blue-700/50 rounded">
              <p className="text-xxs text-blue-300">
                <strong>5-Tuple Ontology:</strong> Universal primitives for intelligence classification and graph relationships.
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default RightPanel;

