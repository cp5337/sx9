import React from "react";
import { Eye, EyeOff, Lock, Activity, GitGraph, Trash2 } from "lucide-react";
import type { GraphNode } from "../lib/database.types";

export interface ContextMenuProps {
  x: number;
  y: number;
  node: GraphNode | null;
  onClose: () => void;
  onAction: (action: string, nodeId?: string) => void;
  isOpen: boolean;
}

export default function ContextMenu({
  x,
  y,
  node,
  onClose,
  onAction,
  isOpen,
}: ContextMenuProps) {
  if (!isOpen) return null;

  const actions = node
    ? [
        {
          id: "inspect",
          icon: Eye,
          label: "Inspect Details",
          color: "text-blue-400",
        },
        {
          id: "correlate",
          icon: Activity,
          label: "Correlate (Nonagon)",
          color: "text-purple-400",
        },
        {
          id: "trace",
          icon: GitGraph,
          label: "Trace Path",
          color: "text-green-400",
        },
        {
          id: "lock",
          icon: Lock,
          label: "Lock Position",
          color: "text-yellow-400",
        },
        {
          id: "hide",
          icon: EyeOff,
          label: "Hide Node",
          color: "text-gray-400",
        },
        {
          id: "delete",
          icon: Trash2,
          label: "Delete Entity",
          color: "text-red-400",
          divider: true,
        },
      ]
    : [
        {
          id: "add_node",
          icon: Eye,
          label: "Add Node Here",
          color: "text-white",
        },
        {
          id: "reset_view",
          icon: Activity,
          label: "Reset View",
          color: "text-white",
        },
      ];

  return (
    <div
      className="absolute z-50 pointer-events-none"
      style={{ left: x, top: y }}
    >
      {/* Backdrop to catch clicks outside */}
      <div
        className="fixed inset-0 pointer-events-auto"
        onClick={onClose}
        onContextMenu={(e) => {
          e.preventDefault();
          onClose();
        }}
      />

      <div className="relative pointer-events-auto bg-gray-900 border border-gray-700 rounded-md shadow-2xl w-48 overflow-hidden animate-in fade-in zoom-in-95 duration-100">
        {node && (
          <div className="px-3 py-2 bg-gray-800 border-b border-gray-700 text-xs text-gray-400 font-mono uppercase tracking-wider">
            {node.label} / {node.id.slice(0, 8)}...
          </div>
        )}

        <div className="py-1">
          {actions.map((action) => (
            <React.Fragment key={action.id}>
              {action.divider && <div className="h-px bg-gray-800 my-1 mx-2" />}
              <button
                onClick={() => {
                  onAction(action.id, node?.id);
                  onClose();
                }}
                className="w-full text-left px-3 py-2 text-sm text-gray-200 hover:bg-gray-800 flex items-center gap-2 group transition-colors"
              >
                <action.icon
                  size={14}
                  className={`${action.color} group-hover:scale-110 transition-transform`}
                />
                {action.label}
              </button>
            </React.Fragment>
          ))}
        </div>
      </div>
    </div>
  );
}
