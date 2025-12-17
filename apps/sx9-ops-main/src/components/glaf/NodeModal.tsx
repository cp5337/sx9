import { useState, useEffect } from "react";
import { X, Plus, Trash2 } from "lucide-react";
import type { GraphNode } from "../lib/database.types";

interface NodeModalProps {
  node: GraphNode | null;
  isOpen: boolean;
  onClose: () => void;
  onSave: (node: Partial<GraphNode>) => void;
  onDelete?: (nodeId: string) => void;
}

export default function NodeModal({
  node,
  isOpen,
  onClose,
  onSave,
  onDelete,
}: NodeModalProps) {
  const [label, setLabel] = useState("");
  const [properties, setProperties] = useState<
    Array<{ key: string; value: string }>
  >([]);

  useEffect(() => {
    if (node) {
      setLabel(node.label);
      setProperties(
        Object.entries(node.properties).map(([key, value]) => ({
          key,
          value: typeof value === "string" ? value : JSON.stringify(value),
        }))
      );
    } else {
      setLabel("");
      setProperties([]);
    }
  }, [node]);

  if (!isOpen) return null;

  const handleSave = () => {
    const propsObject: Record<string, any> = {};
    properties.forEach(({ key, value }) => {
      if (key.trim()) {
        try {
          propsObject[key] = JSON.parse(value);
        } catch {
          propsObject[key] = value;
        }
      }
    });

    onSave({
      id: node?.id,
      label,
      properties: propsObject,
    });
    onClose();
  };

  const addProperty = () => {
    setProperties([...properties, { key: "", value: "" }]);
  };

  const removeProperty = (index: number) => {
    setProperties(properties.filter((_, i) => i !== index));
  };

  const updateProperty = (
    index: number,
    field: "key" | "value",
    value: string
  ) => {
    const newProps = [...properties];
    newProps[index][field] = value;
    setProperties(newProps);
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50 backdrop-blur-sm">
      <div className="bg-dark-surface rounded-lg shadow-2xl w-full max-w-lg max-h-[85vh] overflow-hidden flex flex-col border border-dark-border animate-in fade-in zoom-in-95 duration-200">
        <div className="flex items-center justify-between p-3 border-b border-dark-border bg-dark-bg/50">
          <div className="flex items-center gap-2">
            <span className="w-2 h-8 bg-blue-500 rounded-full"></span>
            <h2 className="text-lg font-bold text-dark-text-primary tracking-tight">
              {node ? "Edit Node" : "New Node"}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="text-dark-text-secondary hover:text-red-400 transition-colors p-1"
          >
            <X size={20} />
          </button>
        </div>

        <div className="p-4 overflow-y-auto flex-1 custom-scrollbar">
          <div className="mb-4">
            <label className="block text-xs font-medium text-dark-text-secondary uppercase tracking-wider mb-1">
              Label
            </label>
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="w-full px-3 py-1.5 border border-dark-border bg-dark-elevated text-dark-text-primary text-sm rounded focus:ring-1 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
              placeholder="Node Type"
            />
          </div>

          <div className="mb-2">
            <div className="flex items-center justify-between mb-2">
              <label className="block text-xs font-medium text-dark-text-secondary uppercase tracking-wider">
                Properties
              </label>
              <button
                onClick={addProperty}
                className="flex items-center gap-1 px-2 py-0.5 text-xs bg-dark-elevated hover:bg-blue-900/30 text-blue-400 border border-dark-border rounded transition-colors"
              >
                <Plus size={12} />
                Add
              </button>
            </div>

            <div className="space-y-2">
              {properties.map((prop, index) => (
                <div key={index} className="flex gap-2 items-center group">
                  <input
                    type="text"
                    value={prop.key}
                    onChange={(e) =>
                      updateProperty(index, "key", e.target.value)
                    }
                    className="flex-1 min-w-0 px-2 py-1 border border-dark-border bg-dark-elevated text-dark-text-primary text-xs rounded focus:border-blue-500 outline-none"
                    placeholder="Key"
                  />
                  <span className="text-dark-text-secondary text-xs">:</span>
                  <input
                    type="text"
                    value={prop.value}
                    onChange={(e) =>
                      updateProperty(index, "value", e.target.value)
                    }
                    className="flex-[2] min-w-0 px-2 py-1 border border-dark-border bg-dark-elevated text-dark-text-primary text-xs rounded focus:border-blue-500 outline-none"
                    placeholder="Value"
                  />
                  <button
                    onClick={() => removeProperty(index)}
                    className="p-1 text-dark-text-secondary hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
              {properties.length === 0 && (
                <div className="text-center py-4 border-2 border-dashed border-dark-border rounded-lg">
                  <p className="text-xs text-dark-text-secondary">
                    No properties defined
                  </p>
                </div>
              )}
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between p-3 border-t border-dark-border bg-dark-bg/50">
          <div>
            {node && onDelete && (
              <button
                onClick={() => {
                  onDelete(node.id);
                  onClose();
                }}
                className="text-red-400 hover:text-red-300 text-xs font-medium px-2 py-1 rounded hover:bg-red-900/10 transition-colors"
              >
                Delete
              </button>
            )}
          </div>
          <div className="flex gap-2">
            <button
              onClick={onClose}
              className="px-3 py-1.5 text-xs text-dark-text-secondary hover:text-dark-text-primary hover:bg-dark-elevated rounded transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              className="px-4 py-1.5 bg-blue-600 text-white text-xs font-medium rounded hover:bg-blue-500 shadow-lg shadow-blue-900/20 transition-all transform hover:scale-105"
            >
              {node ? "Save Changes" : "Create Node"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
