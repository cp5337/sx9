import { useState, useMemo } from "react";
import {
  Database,
  Search,
  Trash2,
  Edit2,
  Check,
  X,
  LayoutList,
  LayoutGrid,
  GitBranch,
} from "lucide-react";
import type { GraphNode, GraphRelationship } from "../lib/database.types";

interface EntityCRUDPanelProps {
  nodes: GraphNode[];
  relationships?: GraphRelationship[];
  onUpdateNode: (node: GraphNode) => void;
  onDeleteNode: (nodeId: string) => void;
  onSelectNode: (node: GraphNode | null) => void;
  onDeleteRelationship?: (relId: string) => void;
}

export default function EntityCRUDPanel({
  nodes,
  relationships = [],
  onUpdateNode,
  onDeleteNode,
  onSelectNode,
  onDeleteRelationship,
}: EntityCRUDPanelProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [viewMode, setViewMode] = useState<"list" | "grid" | "relationships">(
    "list"
  );
  const [editingNodeId, setEditingNodeId] = useState<string | null>(null);
  const [editForm, setEditForm] = useState<Partial<GraphNode>>({});

  // Filter nodes based on search
  const filteredNodes = useMemo(() => {
    if (!searchQuery.trim()) return nodes;
    const query = searchQuery.toLowerCase();
    return nodes.filter(
      (n) =>
        n.label.toLowerCase().includes(query) ||
        n.id.toLowerCase().includes(query) ||
        JSON.stringify(n.properties).toLowerCase().includes(query)
    );
  }, [nodes, searchQuery]);

  // Filter relationships based on search
  const filteredRelationships = useMemo(() => {
    if (!searchQuery.trim()) return relationships;
    const query = searchQuery.toLowerCase();
    return relationships.filter(
      (r) =>
        r.type.toLowerCase().includes(query) ||
        r.id.toLowerCase().includes(query)
    );
  }, [relationships, searchQuery]);

  // Extract all unique property keys for Grid View columns
  const propertyKeys = useMemo(() => {
    const keys = new Set<string>();
    filteredNodes.forEach((node) => {
      Object.keys(node.properties || {}).forEach((k) => keys.add(k));
    });
    return Array.from(keys).sort();
  }, [filteredNodes]);

  const startEditing = (node: GraphNode) => {
    setEditingNodeId(node.id);
    setEditForm({ ...node });
    onSelectNode(node);
  };

  const cancelEditing = () => {
    setEditingNodeId(null);
    setEditForm({});
    onSelectNode(null);
  };

  const saveNode = () => {
    if (editingNodeId && editForm) {
      // Ensure properties is treated as a generic Record<string, any>
      // and safely merge.
      const updatedNode: GraphNode = {
        ...nodes.find((n) => n.id === editingNodeId)!,
        ...editForm,
        label: editForm.label || "Unknown",
        properties: editForm.properties || {},
      };
      onUpdateNode(updatedNode);
      setEditingNodeId(null);
      setEditForm({});
    }
  };

  const getNodeLabel = (id: string) => {
    const node = nodes.find((n) => n.id === id);
    return node ? node.label : id;
  };

  // Helper to generate consistent pastel colors from a string
  const getLabelColor = (label: string) => {
    let hash = 0;
    for (let i = 0; i < label.length; i++) {
      hash = label.charCodeAt(i) + ((hash << 5) - hash);
    }
    const h = Math.abs(hash) % 360;
    return `hsl(${h}, 70%, 90%)`; // Pastel background
  };

  const getLabelText = (label: string) => {
    let hash = 0;
    for (let i = 0; i < label.length; i++) {
      hash = label.charCodeAt(i) + ((hash << 5) - hash);
    }
    const h = Math.abs(hash) % 360;
    return `hsl(${h}, 70%, 30%)`; // Darker text
  };

  return (
    <div className="flex flex-col h-full bg-gray-50 dark:bg-dark-bg">
      {/* Header */}
      <div className="bg-white dark:bg-dark-surface border-b border-gray-200 dark:border-dark-border px-4 py-3">
        <div className="flex items-center gap-2 mb-3">
          <Database className="text-blue-600 dark:text-blue-400" size={20} />
          <h3 className="font-semibold text-gray-900 dark:text-dark-text-primary">
            Entity Manager
          </h3>
          <span className="ml-auto text-xs bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full">
            {viewMode === "relationships"
              ? `${relationships.length} Edges`
              : `${nodes.length} Nodes`}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <div className="relative flex-1">
            <Search
              className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
              size={14}
            />
            <input
              type="text"
              placeholder={
                viewMode === "relationships"
                  ? "Search relationships..."
                  : "Search entities..."
              }
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-3 py-2 text-sm bg-gray-100 dark:bg-dark-elevated border-none rounded-md focus:ring-1 focus:ring-blue-500 text-gray-900 dark:text-dark-text-primary"
            />
          </div>
          <div className="flex bg-gray-100 dark:bg-dark-elevated rounded p-1 gap-1">
            <button
              onClick={() => setViewMode("list")}
              className={`p-1.5 rounded ${
                viewMode === "list"
                  ? "bg-white dark:bg-dark-surface shadow text-blue-600"
                  : "text-gray-400 hover:text-gray-600"
              }`}
              title="List View"
            >
              <LayoutList size={16} />
            </button>
            <button
              onClick={() => setViewMode("grid")}
              className={`p-1.5 rounded ${
                viewMode === "grid"
                  ? "bg-white dark:bg-dark-surface shadow text-blue-600"
                  : "text-gray-400 hover:text-gray-600"
              }`}
              title="Grid View"
            >
              <LayoutGrid size={16} />
            </button>
            <div className="w-px bg-gray-300 dark:bg-dark-border mx-1" />
            <button
              onClick={() => setViewMode("relationships")}
              className={`p-1.5 rounded ${
                viewMode === "relationships"
                  ? "bg-white dark:bg-dark-surface shadow text-blue-600"
                  : "text-gray-400 hover:text-gray-600"
              }`}
              title="Relationships"
            >
              <GitBranch size={16} />
            </button>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-2">
        {viewMode === "relationships" ? (
          // RELATIONSHIPS VIEW
          relationships.length === 0 ? (
            <div className="text-center py-8 text-gray-500 text-sm">
              No relationships found.
            </div>
          ) : (
            <div className="bg-white dark:bg-dark-surface border border-gray-200 dark:border-dark-border rounded-lg overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead className="text-xs text-gray-500 uppercase bg-gray-50 dark:bg-dark-elevated border-b border-gray-200 dark:border-dark-border">
                  <tr>
                    <th className="px-4 py-3 font-medium min-w-[120px]">
                      Source Node
                    </th>
                    <th className="px-4 py-3 font-medium min-w-[100px]">
                      Edge Type
                    </th>
                    <th className="px-4 py-3 font-medium min-w-[120px]">
                      Target Node
                    </th>
                    <th className="px-4 py-3 font-medium w-[80px]">Actions</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200 dark:divide-dark-border">
                  {filteredRelationships.map((rel) => {
                    const sourceLabel = getNodeLabel(rel.source);
                    const targetLabel = getNodeLabel(rel.target);
                    return (
                      <tr
                        key={rel.id}
                        className="hover:bg-gray-50 dark:hover:bg-dark-elevated group"
                      >
                        <td className="px-4 py-2 font-mono text-xs text-gray-600 dark:text-gray-400 truncate max-w-[150px]">
                          <span
                            className="px-2 py-1 rounded-full text-[10px] font-semibold border border-opacity-20"
                            style={{
                              backgroundColor: getLabelColor(sourceLabel),
                              color: getLabelText(sourceLabel),
                              borderColor: getLabelText(sourceLabel),
                            }}
                          >
                            {sourceLabel}
                          </span>
                          <span className="ml-2 text-gray-400 text-[10px]">
                            {rel.source.substring(0, 4)}...
                          </span>
                        </td>
                        <td className="px-4 py-2 font-medium text-blue-600 dark:text-blue-400 text-xs uppercase tracking-wider">
                          {rel.type}
                        </td>
                        <td className="px-4 py-2 font-mono text-xs text-gray-600 dark:text-gray-400 truncate max-w-[150px]">
                          <span
                            className="px-2 py-1 rounded-full text-[10px] font-semibold border border-opacity-20"
                            style={{
                              backgroundColor: getLabelColor(targetLabel),
                              color: getLabelText(targetLabel),
                              borderColor: getLabelText(targetLabel),
                            }}
                          >
                            {targetLabel}
                          </span>
                          <span className="ml-2 text-gray-400 text-[10px]">
                            {rel.target.substring(0, 4)}...
                          </span>
                        </td>
                        <td className="px-4 py-2">
                          <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                            {onDeleteRelationship && (
                              <button
                                onClick={() => onDeleteRelationship(rel.id)}
                                className="p-1 hover:bg-red-50 text-gray-400 hover:text-red-600 rounded"
                                title="Delete Edge"
                              >
                                <Trash2 size={14} />
                              </button>
                            )}
                          </div>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )
        ) : filteredNodes.length === 0 ? (
          <div className="text-center py-8 text-gray-500 text-sm">
            No entities found.
          </div>
        ) : viewMode === "list" ? (
          // LIST VIEW
          <div className="space-y-2">
            {filteredNodes.map((node) => (
              <div
                key={node.id}
                className={`bg-white dark:bg-dark-surface rounded-md shadow-sm border ${
                  editingNodeId === node.id
                    ? "border-blue-500 ring-1 ring-blue-500"
                    : "border-gray-200 dark:border-dark-border"
                } p-3 transition-all`}
              >
                {editingNodeId === node.id ? (
                  <div className="space-y-3">
                    {/* Edit Mode Form */}
                    <div>
                      <label className="text-xs font-medium text-gray-500 uppercase">
                        Label
                      </label>
                      <input
                        type="text"
                        value={editForm.label || ""}
                        onChange={(e) =>
                          setEditForm((prev) => ({
                            ...prev,
                            label: e.target.value,
                          }))
                        }
                        className="w-full text-sm p-1 border rounded bg-gray-50 dark:bg-dark-elevated dark:text-white dark:border-dark-border"
                      />
                    </div>
                    <div>
                      <label className="text-xs font-medium text-gray-500 uppercase">
                        Description (Prop)
                      </label>
                      <input
                        type="text"
                        value={
                          editForm.properties?.description ||
                          editForm.properties?.name ||
                          ""
                        }
                        onChange={(e) =>
                          setEditForm((prev) => ({
                            ...prev,
                            properties: {
                              ...prev.properties,
                              description: e.target.value,
                            },
                          }))
                        }
                        className="w-full text-sm p-1 border rounded bg-gray-50 dark:bg-dark-elevated dark:text-white dark:border-dark-border"
                      />
                    </div>

                    <div className="flex gap-2 justify-end pt-2">
                      <button
                        onClick={cancelEditing}
                        className="p-1 hover:bg-gray-100 rounded text-gray-600"
                      >
                        <X size={16} />
                      </button>
                      <button
                        onClick={saveNode}
                        className="p-1 bg-blue-100 hover:bg-blue-200 rounded text-blue-700"
                      >
                        <Check size={16} />
                      </button>
                    </div>
                  </div>
                ) : (
                  <div className="flex items-start justify-between group">
                    <div
                      className="cursor-pointer flex-1"
                      onClick={() => onSelectNode(node)}
                    >
                      <div className="font-medium text-gray-900 dark:text-dark-text-primary text-sm flex items-center gap-2">
                        {node.label}
                      </div>
                      <div className="text-xs text-gray-500 mt-1 line-clamp-2">
                        {node.properties?.description ||
                          node.properties?.name ||
                          JSON.stringify(node.properties)}
                      </div>
                      <div className="text-[10px] text-gray-400 mt-1 font-mono">
                        {node.id}
                      </div>
                    </div>

                    <div className="flex items-center opacity-0 group-hover:opacity-100 transition-opacity gap-1">
                      <button
                        onClick={() => startEditing(node)}
                        className="p-1.5 hover:bg-blue-50 text-gray-400 hover:text-blue-600 rounded"
                        title="Edit"
                      >
                        <Edit2 size={14} />
                      </button>
                      <button
                        onClick={() => onDeleteNode(node.id)}
                        className="p-1.5 hover:bg-red-50 text-gray-400 hover:text-red-600 rounded"
                        title="Delete"
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        ) : (
          // GRID VIEW (Nodes)
          <div className="bg-white dark:bg-dark-surface border border-gray-200 dark:border-dark-border rounded-lg overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-gray-500 uppercase bg-gray-50 dark:bg-dark-elevated border-b border-gray-200 dark:border-dark-border">
                <tr>
                  <th className="px-4 py-3 font-medium min-w-[120px]">ID</th>
                  <th className="px-4 py-3 font-medium min-w-[100px]">Label</th>
                  {propertyKeys.map((key) => (
                    <th
                      key={key}
                      className="px-4 py-3 font-medium min-w-[150px]"
                    >
                      {key}
                    </th>
                  ))}
                  <th className="px-4 py-3 font-medium w-[80px]">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-dark-border">
                {filteredNodes.map((node) => (
                  <tr
                    key={node.id}
                    className="hover:bg-gray-50 dark:hover:bg-dark-elevated group"
                  >
                    <td className="px-4 py-2 font-mono text-xs text-gray-500 truncate max-w-[120px]">
                      {node.id}
                    </td>
                    <td className="px-4 py-2 font-medium text-gray-900 dark:text-dark-text-primary">
                      {node.label}
                    </td>
                    {propertyKeys.map((key) => (
                      <td
                        key={key}
                        className="px-4 py-2 text-gray-600 dark:text-dark-text-secondary truncate max-w-[200px]"
                      >
                        {typeof node.properties?.[key] === "object"
                          ? JSON.stringify(node.properties[key])
                          : String(node.properties?.[key] || "-")}
                      </td>
                    ))}
                    <td className="px-4 py-2">
                      <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button
                          onClick={() => startEditing(node)}
                          className="p-1 hover:bg-blue-50 text-gray-400 hover:text-blue-600 rounded"
                          title="Edit"
                        >
                          <Edit2 size={14} />
                        </button>
                        <button
                          onClick={() => onDeleteNode(node.id)}
                          className="p-1 hover:bg-red-50 text-gray-400 hover:text-red-600 rounded"
                          title="Delete"
                        >
                          <Trash2 size={14} />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
