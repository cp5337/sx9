import { useState, useEffect, useRef } from "react";
import { X, Trash2, Terminal, Save, AlertTriangle } from "lucide-react";
import type { GraphNode, GraphRelationship } from "../lib/database.types";

interface RelationshipModalProps {
  relationship: GraphRelationship | null;
  nodes: GraphNode[];
  isOpen: boolean;
  onClose: () => void;
  onSave: (relationship: Partial<GraphRelationship>) => void;
  onDelete?: (relationshipId: string) => void;
}

export default function RelationshipModal({
  relationship,
  nodes,
  isOpen,
  onClose,
  onSave,
  onDelete,
}: RelationshipModalProps) {
  const [type, setType] = useState("");
  const [sourceId, setSourceId] = useState("");
  const [targetId, setTargetId] = useState("");
  const [properties, setProperties] = useState<
    Array<{ key: string; value: string }>
  >([]);
  const [jsonError, setJsonError] = useState<string | null>(null);

  // Expandable State
  const [detailsOpen, setDetailsOpen] = useState(true);

  // Draggable Logic
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const dragStart = useRef({ x: 0, y: 0 });
  const modalRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = (e: React.MouseEvent) => {
    // Only drag from header
    setIsDragging(true);
    dragStart.current = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;
      setPosition({
        x: e.clientX - dragStart.current.x,
        y: e.clientY - dragStart.current.y,
      });
    };
    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      window.addEventListener("mousemove", handleMouseMove);
      window.addEventListener("mouseup", handleMouseUp);
    }
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging]);

  // Common Relationship Types for Autocomplete
  const knownTypes = [
    "CONTROLS",
    "DEPENDS_ON",
    "CONNECTED_TO",
    "LINKED",
    "OWNED_BY",
    "HOSTS",
    "EXECUTES",
    "STORES",
    "SECURES",
    "MONITORS",
    "REPLICATES_TO",
  ];

  useEffect(() => {
    if (relationship) {
      setType(relationship.type);
      setSourceId(relationship.source);
      setTargetId(relationship.target);
      setProperties(
        Object.entries(relationship.properties).map(([key, value]) => ({
          key,
          value: typeof value === "string" ? value : JSON.stringify(value),
        }))
      );
    } else {
      setType("");
      setSourceId("");
      setTargetId("");
      setProperties([]);
    }
  }, [relationship]);

  // Keyboard Shortcuts: Esc to Close, Enter to Save
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;
      if (e.key === "Escape") onClose();
      if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) handleSave();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, sourceId, targetId, type, properties]);

  if (!isOpen) return null;

  const handleSave = () => {
    if (!sourceId || !targetId) return;

    // Validate JSON in properties
    const propsObject: Record<string, any> = {};
    let hasError = false;

    properties.forEach(({ key, value }) => {
      if (key.trim()) {
        try {
          // Attempt to parse if it looks like JSON/Number/Boolean
          if (
            value.startsWith("{") ||
            value.startsWith("[") ||
            value === "true" ||
            value === "false" ||
            !isNaN(Number(value))
          ) {
            propsObject[key] = JSON.parse(value);
          } else {
            propsObject[key] = value;
          }
        } catch (e) {
          // Keep as string if parse fails, but loose check for obvious syntax errors in objects
          if (value.startsWith("{") || value.startsWith("[")) {
            setJsonError(`Invalid JSON for key: ${key}`);
            hasError = true;
          }
          propsObject[key] = value;
        }
      }
    });

    if (hasError) return;
    setJsonError(null);

    onSave({
      id: relationship?.id,
      type: type.toUpperCase(), // Force upper snake case
      source: sourceId,
      target: targetId,
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
    setJsonError(null); // Clear error on edit
  };

  // Get labels for ASCII visualization
  const sourceNode = nodes.find((n) => n.id === sourceId);
  const targetNode = nodes.find((n) => n.id === targetId);
  const sourceLabel = sourceNode
    ? `${sourceNode.label}:${
        sourceNode.properties.name || sourceNode.id.slice(0, 5)
      }`
    : "UNKNOWN";
  const targetLabel = targetNode
    ? `${targetNode.label}:${
        targetNode.properties.name || targetNode.id.slice(0, 5)
      }`
    : "UNKNOWN";

  return (
    <div className="fixed inset-0 pointer-events-none flex items-center justify-center z-[100] font-mono">
      {/* COMPACT HUD CONATINER: max-w-md instead of 3xl */}
      <div
        ref={modalRef}
        className="bg-gray-950 w-full max-w-lg border border-emerald-900 shadow-[0_0_30px_rgba(16,185,129,0.1)] flex flex-col max-h-[80vh] pointer-events-auto"
        style={{ transform: `translate(${position.x}px, ${position.y}px)` }}
      >
        {/* TERMINAL HEADER - Draggable */}
        <div
          className="flex items-center justify-between px-3 py-1.5 border-b border-emerald-900/50 bg-emerald-950/20 cursor-move select-none"
          onMouseDown={handleMouseDown}
        >
          <div className="flex items-center gap-2 text-emerald-500">
            <Terminal size={12} />
            <span className="text-[10px] font-bold tracking-wider">
              {relationship
                ? `LINK:[ID:${relationship.id.slice(0, 6)}]`
                : "NEW_LINK_SEQ"}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setDetailsOpen(!detailsOpen)}
              className="text-emerald-700 hover:text-emerald-400 text-[10px] border border-emerald-900/50 px-1 rounded bg-black/50"
            >
              {detailsOpen ? "COLLAPSE [-]" : "EXPAND [+]"}
            </button>
            <button
              onClick={onClose}
              className="text-emerald-700 hover:text-emerald-400"
            >
              <X size={12} />
            </button>
          </div>
        </div>

        <div
          className={`p-0 overflow-y-auto flex-1 text-[10px] leading-tight transition-all duration-300 ${
            detailsOpen
              ? "max-h-[500px] opacity-100"
              : "max-h-0 opacity-0 overflow-hidden"
          }`}
        >
          {/* ASCII VISUALIZATION HUD - Very Compact */}
          <div className="bg-black p-2 border-b border-emerald-900/50 flex flex-col items-center justify-center py-4 space-y-1">
            <div className="flex items-center gap-2 text-emerald-400 font-bold opacity-90">
              <div className="border border-emerald-800 bg-emerald-900/20 px-2 py-0.5 rounded-sm text-center min-w-[80px] truncate max-w-[120px]">
                {sourceId ? sourceLabel : "SRC"}
              </div>
              <div className="flex flex-col items-center">
                <span className="text-[8px] text-emerald-700">TYPE</span>
                <div className="flex items-center text-emerald-500 font-bold text-[9px]">
                  ==({type || "?"})==&gt;
                </div>
              </div>
              <div className="border border-emerald-800 bg-emerald-900/20 px-2 py-0.5 rounded-sm text-center min-w-[80px] truncate max-w-[120px]">
                {targetId ? targetLabel : "TGT"}
              </div>
            </div>
          </div>

          {/* HIGH DENSITY INPUT GRID - Stacked for width efficiency */}
          <div className="flex flex-col border-b border-emerald-900/30">
            {/* SETTINGS ROW - compact 3-col */}
            <div className="grid grid-cols-3 gap-2 border-b border-emerald-900/30 bg-emerald-900/5 p-2">
              <div className="space-y-0.5">
                <label className="text-emerald-700 font-bold uppercase text-[8px]">
                  Type
                </label>
                <input
                  list="rel-types"
                  type="text"
                  value={type}
                  onChange={(e) => setType(e.target.value.toUpperCase())}
                  className="w-full bg-black border border-emerald-900 text-emerald-400 px-1 py-0.5 focus:ring-1 focus:ring-emerald-500 outline-none uppercase placeholder-emerald-900 text-[10px]"
                  placeholder="CONTROLS"
                  autoFocus={!relationship}
                />
                <datalist id="rel-types">
                  {knownTypes.map((t) => (
                    <option key={t} value={t} />
                  ))}
                </datalist>
              </div>

              <div className="space-y-0.5">
                <label className="text-emerald-700 font-bold uppercase text-[8px]">
                  Source
                </label>
                <select
                  value={sourceId}
                  onChange={(e) => setSourceId(e.target.value)}
                  className="w-full bg-black border border-emerald-900 text-emerald-400 px-1 py-0.5 focus:ring-1 focus:ring-emerald-500 outline-none text-[10px]"
                  disabled={!!relationship}
                >
                  <option value="">[SRC]</option>
                  {nodes.map((n) => (
                    <option key={n.id} value={n.id}>
                      {n.label.slice(0, 3)}:{n.properties.name || n.id}
                    </option>
                  ))}
                </select>
              </div>

              <div className="space-y-0.5">
                <label className="text-emerald-700 font-bold uppercase text-[8px]">
                  Target
                </label>
                <select
                  value={targetId}
                  onChange={(e) => setTargetId(e.target.value)}
                  className="w-full bg-black border border-emerald-900 text-emerald-400 px-1 py-0.5 focus:ring-1 focus:ring-emerald-500 outline-none text-[10px]"
                  disabled={!!relationship}
                >
                  <option value="">[TGT]</option>
                  {nodes.map((n) => (
                    <option key={n.id} value={n.id}>
                      {n.label.slice(0, 3)}:{n.properties.name || n.id}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            {/* PROPERTIES AREA */}
            <div className="p-2 bg-black/50">
              <div className="flex items-center justify-between mb-1">
                <label className="text-emerald-700 font-bold uppercase text-[8px]">
                  PAYLOAD (JSON)
                </label>
                <button
                  onClick={addProperty}
                  className="text-[8px] bg-emerald-900/30 text-emerald-400 px-1.5 py-px border border-emerald-800 hover:bg-emerald-900/50"
                >
                  + ADD_KEY
                </button>
              </div>

              {jsonError && (
                <div className="mb-1 flex items-center gap-1 text-red-400 bg-red-900/20 border border-red-900/50 px-1 py-0.5 text-[9px]">
                  <AlertTriangle size={10} />
                  <span>{jsonError}</span>
                </div>
              )}

              <div className="border border-emerald-900/30">
                {/* TABLE HEADER */}
                <div className="grid grid-cols-[1fr_2fr_20px] bg-emerald-900/20 text-emerald-600 font-bold px-2 py-0.5 text-[8px]">
                  <div>KEY</div>
                  <div>VALUE</div>
                  <div></div>
                </div>

                {/* ROWS */}
                {properties.length === 0 && (
                  <div className="text-emerald-900 italic px-2 py-2 text-center text-[9px]">
                    No properties.
                  </div>
                )}

                {properties.map((prop, index) => (
                  <div
                    key={index}
                    className="grid grid-cols-[1fr_2fr_20px] border-t border-emerald-900/30 group"
                  >
                    <input
                      type="text"
                      value={prop.key}
                      onChange={(e) =>
                        updateProperty(index, "key", e.target.value)
                      }
                      className="bg-transparent text-emerald-400 px-2 py-0.5 outline-none border-r border-emerald-900/30 placeholder-emerald-900 text-[10px]"
                      placeholder="key"
                    />
                    <input
                      type="text"
                      value={prop.value}
                      onChange={(e) =>
                        updateProperty(index, "value", e.target.value)
                      }
                      className="bg-transparent text-gray-300 px-2 py-0.5 outline-none placeholder-emerald-900/50 text-[10px]"
                      placeholder="value"
                    />
                    <button
                      onClick={() => removeProperty(index)}
                      className="flex items-center justify-center text-emerald-800 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                      <X size={10} />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* FOOTER ACTIONS - Compact */}
        <div className="flex items-center justify-between p-1.5 border-t border-emerald-900 bg-emerald-950/20 text-[9px]">
          <div>
            {relationship && onDelete && (
              <button
                onClick={() => {
                  onDelete(relationship.id);
                  onClose();
                }}
                className="flex items-center gap-1 text-red-500 hover:text-red-400 px-2 py-0.5 border border-transparent hover:border-red-900/50"
              >
                <Trash2 size={10} /> FLUSH
              </button>
            )}
          </div>
          <div className="flex gap-2">
            <button
              onClick={onClose}
              className="px-2 py-0.5 text-emerald-700 hover:text-emerald-500"
            >
              [ESC]
            </button>
            <button
              onClick={handleSave}
              disabled={!sourceId || !targetId || !type}
              className="flex items-center gap-1 px-3 py-0.5 bg-emerald-900/50 text-emerald-400 border border-emerald-700 hover:bg-emerald-800/50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Save size={10} /> [COMMIT]
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
