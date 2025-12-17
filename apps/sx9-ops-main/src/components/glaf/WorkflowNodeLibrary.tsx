import { useState } from 'react';
import { Search, X } from 'lucide-react';
import { NODE_TYPE_DEFINITIONS, NODE_CATEGORIES, getAllCategories } from '../lib/workflow/nodeTypes';
import { NodeCategory, NodeTypeDefinition } from '../types/workflow.types';

interface WorkflowNodeLibraryProps {
  onNodeSelect: (nodeType: NodeTypeDefinition) => void;
}

export default function WorkflowNodeLibrary({ onNodeSelect }: WorkflowNodeLibraryProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<NodeCategory | 'all'>('all');

  const categories = getAllCategories();

  const filteredNodes = NODE_TYPE_DEFINITIONS.filter(node => {
    const matchesSearch = !searchQuery ||
      node.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      node.description.toLowerCase().includes(searchQuery.toLowerCase());

    const matchesCategory = selectedCategory === 'all' || node.category === selectedCategory;

    return matchesSearch && matchesCategory;
  });

  const nodesByCategory = categories.reduce((acc, category) => {
    acc[category] = filteredNodes.filter(node => node.category === category);
    return acc;
  }, {} as Record<NodeCategory, NodeTypeDefinition[]>);

  return (
    <div className="flex flex-col h-full bg-dark-surface">
      <div className="p-4 border-b border-dark-border">
        <div className="relative">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-dark-text-secondary" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search nodes..."
            className="w-full pl-9 pr-9 py-2 bg-dark-bg border border-dark-border rounded text-sm text-dark-text-primary placeholder-dark-text-secondary focus:outline-none focus:border-blue-500"
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-dark-text-secondary hover:text-dark-text-primary"
            >
              <X size={16} />
            </button>
          )}
        </div>
      </div>

      <div className="flex overflow-x-auto border-b border-dark-border">
        <button
          onClick={() => setSelectedCategory('all')}
          className={`px-4 py-2 text-xs font-medium whitespace-nowrap transition-colors border-b-2 ${
            selectedCategory === 'all'
              ? 'border-blue-500 text-blue-400'
              : 'border-transparent text-dark-text-secondary hover:text-dark-text-primary'
          }`}
        >
          All
        </button>
        {categories.map(category => {
          const count = nodesByCategory[category]?.length || 0;
          return (
            <button
              key={category}
              onClick={() => setSelectedCategory(category)}
              className={`px-4 py-2 text-xs font-medium whitespace-nowrap transition-colors border-b-2 ${
                selectedCategory === category
                  ? 'border-blue-500 text-blue-400'
                  : 'border-transparent text-dark-text-secondary hover:text-dark-text-primary'
              }`}
            >
              {NODE_CATEGORIES[category].name} ({count})
            </button>
          );
        })}
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {selectedCategory === 'all' ? (
          categories.map(category => {
            const nodes = nodesByCategory[category];
            if (nodes.length === 0) return null;

            return (
              <div key={category}>
                <h3 className="text-xs font-semibold text-dark-text-secondary uppercase mb-2">
                  {NODE_CATEGORIES[category].name}
                </h3>
                <div className="space-y-2">
                  {nodes.map(node => (
                    <NodeCard key={node.id} node={node} onClick={() => onNodeSelect(node)} />
                  ))}
                </div>
              </div>
            );
          })
        ) : (
          <div className="space-y-2">
            {nodesByCategory[selectedCategory]?.map(node => (
              <NodeCard key={node.id} node={node} onClick={() => onNodeSelect(node)} />
            ))}
          </div>
        )}

        {filteredNodes.length === 0 && (
          <div className="text-center py-12 text-dark-text-secondary">
            <p className="text-sm">No nodes found</p>
            <p className="text-xs mt-1">Try a different search or category</p>
          </div>
        )}
      </div>
    </div>
  );
}

interface NodeCardProps {
  node: NodeTypeDefinition;
  onClick: () => void;
}

function NodeCard({ node, onClick }: NodeCardProps) {
  const Icon = node.icon;

  return (
    <button
      onClick={onClick}
      className="w-full p-3 bg-dark-elevated border border-dark-border rounded hover:border-blue-500/50 hover:bg-dark-elevated/80 transition-all text-left group"
    >
      <div className="flex items-start gap-3">
        <div
          className="p-2 rounded"
          style={{ backgroundColor: `${node.color}20` }}
        >
          <Icon size={18} style={{ color: node.color }} />
        </div>
        <div className="flex-1 min-w-0">
          <h4 className="text-sm font-medium text-dark-text-primary group-hover:text-blue-400 transition-colors">
            {node.name}
          </h4>
          <p className="text-xs text-dark-text-secondary mt-0.5 line-clamp-2">
            {node.description}
          </p>
          <div className="flex items-center gap-2 mt-2">
            <span className="text-2xs text-dark-text-secondary">
              {node.inputs.length} input{node.inputs.length !== 1 ? 's' : ''}
            </span>
            <span className="text-dark-text-secondary">•</span>
            <span className="text-2xs text-dark-text-secondary">
              {node.outputs.length} output{node.outputs.length !== 1 ? 's' : ''}
            </span>
          </div>
        </div>
      </div>
    </button>
  );
}
