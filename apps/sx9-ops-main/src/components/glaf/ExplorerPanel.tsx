import { useState, useEffect } from 'react';
import { FolderTree, Database, GitBranch, Search } from 'lucide-react';
import type { GraphNode, GraphRelationship } from '../lib/database.types';

interface ExplorerPanelProps {
  nodes: GraphNode[];
  relationships: GraphRelationship[];
  onNodeSelect: (node: GraphNode) => void;
  onRelationshipSelect: (rel: GraphRelationship) => void;
}

type ViewMode = 'nodes' | 'relationships';

export default function ExplorerPanel({
  nodes,
  relationships,
  onNodeSelect,
  onRelationshipSelect
}: ExplorerPanelProps) {
  const [viewMode, setViewMode] = useState<ViewMode>('nodes');
  const [searchQuery, setSearchQuery] = useState('');
  const [filteredNodes, setFilteredNodes] = useState<GraphNode[]>(nodes);
  const [filteredRelationships, setFilteredRelationships] = useState<GraphRelationship[]>(relationships);

  useEffect(() => {
    if (searchQuery.trim() === '') {
      setFilteredNodes(nodes);
      setFilteredRelationships(relationships);
    } else {
      const query = searchQuery.toLowerCase();
      setFilteredNodes(
        nodes.filter(
          (node) =>
            node.label.toLowerCase().includes(query) ||
            JSON.stringify(node.properties).toLowerCase().includes(query)
        )
      );
      setFilteredRelationships(
        relationships.filter(
          (rel) =>
            rel.type.toLowerCase().includes(query) ||
            JSON.stringify(rel.properties).toLowerCase().includes(query)
        )
      );
    }
  }, [searchQuery, nodes, relationships]);

  const groupNodesByLabel = () => {
    const groups = new Map<string, GraphNode[]>();
    filteredNodes.forEach((node) => {
      const existing = groups.get(node.label) || [];
      groups.set(node.label, [...existing, node]);
    });
    return Array.from(groups.entries()).sort(([a], [b]) => a.localeCompare(b));
  };

  const groupRelationshipsByType = () => {
    const groups = new Map<string, GraphRelationship[]>();
    filteredRelationships.forEach((rel) => {
      const existing = groups.get(rel.type) || [];
      groups.set(rel.type, [...existing, rel]);
    });
    return Array.from(groups.entries()).sort(([a], [b]) => a.localeCompare(b));
  };

  return (
    <div className="flex flex-col h-full">
      <div className="p-3 border-b border-gray-200 dark:border-dark-border">
        <div className="relative mb-3">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={16} />
          <input
            type="text"
            placeholder="Search..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-9 pr-3 py-2 text-sm bg-white dark:bg-dark-elevated border border-gray-200 dark:border-dark-border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 dark:text-dark-text-primary"
          />
        </div>

        <div className="flex gap-2">
          <button
            onClick={() => setViewMode('nodes')}
            className={`flex-1 flex items-center justify-center gap-2 px-3 py-2 text-sm rounded-lg transition-colors ${
              viewMode === 'nodes'
                ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400'
                : 'text-gray-600 dark:text-dark-text-secondary hover:bg-gray-100 dark:hover:bg-dark-elevated'
            }`}
          >
            <Database size={16} />
            <span>Nodes ({filteredNodes.length})</span>
          </button>
          <button
            onClick={() => setViewMode('relationships')}
            className={`flex-1 flex items-center justify-center gap-2 px-3 py-2 text-sm rounded-lg transition-colors ${
              viewMode === 'relationships'
                ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400'
                : 'text-gray-600 dark:text-dark-text-secondary hover:bg-gray-100 dark:hover:bg-dark-elevated'
            }`}
          >
            <GitBranch size={16} />
            <span>Links ({filteredRelationships.length})</span>
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-auto">
        {viewMode === 'nodes' ? (
          <div className="p-3 space-y-3">
            {groupNodesByLabel().map(([label, groupNodes]) => (
              <div key={label}>
                <div className="flex items-center gap-2 mb-2 text-xs font-semibold text-gray-500 dark:text-dark-text-secondary uppercase tracking-wider">
                  <FolderTree size={14} />
                  <span>{label}</span>
                  <span className="ml-auto bg-gray-200 dark:bg-dark-elevated px-2 py-0.5 rounded-full">
                    {groupNodes.length}
                  </span>
                </div>
                <div className="space-y-1 ml-4">
                  {groupNodes.map((node) => (
                    <button
                      key={node.id}
                      onClick={() => onNodeSelect(node)}
                      className="w-full text-left px-3 py-2 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-dark-elevated transition-colors group"
                    >
                      <div className="font-medium text-gray-900 dark:text-dark-text-primary">
                        {node.properties?.name || node.properties?.title || node.id.substring(0, 8)}
                      </div>
                      {Object.keys(node.properties || {}).length > 0 && (
                        <div className="text-xs text-gray-500 dark:text-dark-text-secondary mt-1">
                          {Object.keys(node.properties || {}).length} properties
                        </div>
                      )}
                    </button>
                  ))}
                </div>
              </div>
            ))}
            {filteredNodes.length === 0 && (
              <div className="text-center py-8 text-gray-500 dark:text-dark-text-secondary text-sm">
                {searchQuery ? 'No nodes match your search' : 'No nodes in graph'}
              </div>
            )}
          </div>
        ) : (
          <div className="p-3 space-y-3">
            {groupRelationshipsByType().map(([type, groupRels]) => (
              <div key={type}>
                <div className="flex items-center gap-2 mb-2 text-xs font-semibold text-gray-500 dark:text-dark-text-secondary uppercase tracking-wider">
                  <GitBranch size={14} />
                  <span>{type}</span>
                  <span className="ml-auto bg-gray-200 dark:bg-dark-elevated px-2 py-0.5 rounded-full">
                    {groupRels.length}
                  </span>
                </div>
                <div className="space-y-1 ml-4">
                  {groupRels.map((rel) => (
                    <button
                      key={rel.id}
                      onClick={() => onRelationshipSelect(rel)}
                      className="w-full text-left px-3 py-2 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-dark-elevated transition-colors"
                    >
                      <div className="text-gray-900 dark:text-dark-text-primary">
                        {rel.source} → {rel.target}
                      </div>
                      {Object.keys(rel.properties || {}).length > 0 && (
                        <div className="text-xs text-gray-500 dark:text-dark-text-secondary mt-1">
                          {Object.keys(rel.properties || {}).length} properties
                        </div>
                      )}
                    </button>
                  ))}
                </div>
              </div>
            ))}
            {filteredRelationships.length === 0 && (
              <div className="text-center py-8 text-gray-500 dark:text-dark-text-secondary text-sm">
                {searchQuery ? 'No relationships match your search' : 'No relationships in graph'}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
