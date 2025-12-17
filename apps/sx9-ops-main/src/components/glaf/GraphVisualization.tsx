// components/GraphVisualization.tsx
import { useRef, useState, useCallback } from "react";
import type { GraphNode, GraphRelationship } from "../lib/database.types";
import { useCytoscape } from "../hooks/useCytoscape";
import RadialMenu from "./RadialMenu";
import ContextMenu from "./ContextMenu";

interface GraphVisualizationProps {
  nodes: GraphNode[];
  relationships: GraphRelationship[];
  onNodeClick?: (node: GraphNode) => void;
  onLinkClick?: (link: GraphRelationship) => void;
  onBackgroundClick?: () => void;
  onLinkCreate?: (sourceId: string, targetId: string) => void;
  onCorrelate?: (nodeId: string) => void;
  onCrossReference?: (nodeId: string, database: string) => void;
}

interface TooltipState {
  x: number;
  y: number;
  data: GraphNode;
}

interface MenuState {
  x: number;
  y: number;
  node: GraphNode;
}

interface ContextMenuState {
  x: number;
  y: number;
  node: GraphNode | null;
}

export default function GraphVisualization({
  nodes,
  relationships,
  onNodeClick,
  onLinkClick,
  onBackgroundClick,
  onLinkCreate,
  onCorrelate,
  onCrossReference,
}: GraphVisualizationProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  // UI State
  const [tooltip, setTooltip] = useState<TooltipState | null>(null);
  const [menuState, setMenuState] = useState<MenuState | null>(null);
  const [contextMenuState, setContextMenuState] = useState<ContextMenuState | null>(null);

  // Cytoscape hook
  const { lockNode, hideNode } = useCytoscape(containerRef, nodes, relationships, {
    onNodeTap: useCallback((node: GraphNode, pos: { x: number; y: number }) => {
      setMenuState({ x: pos.x, y: pos.y, node });
    }, []),

    onEdgeTap: useCallback((rel: GraphRelationship) => {
      onLinkClick?.(rel);
    }, [onLinkClick]),

    onBackgroundTap: useCallback(() => {
      setMenuState(null);
      setContextMenuState(null);
      onBackgroundClick?.();
    }, [onBackgroundClick]),

    onLinkCreate,

    onNodeHover: useCallback((node: GraphNode | null, pos: { x: number; y: number } | null) => {
      if (node && pos) {
        setTooltip({ x: pos.x, y: pos.y, data: node });
      } else {
        setTooltip(null);
      }
    }, []),

    onContextMenu: useCallback((node: GraphNode | null, pos: { x: number; y: number }) => {
      setContextMenuState({ x: pos.x, y: pos.y, node });
    }, []),
  });

  // Radial menu action handler
  const handleRadialAction = useCallback((action: string) => {
    const node = menuState?.node;
    setMenuState(null);
    if (!node) return;

    switch (action) {
      case "inspect":
        onNodeClick?.(node);
        break;
      case "correlate":
        onCorrelate?.(node.id);
        break;
      case "xref_sledis":
        onCrossReference?.(node.id, "sledis");
        break;
    }
  }, [menuState, onNodeClick, onCorrelate, onCrossReference]);

  // Context menu action handler
  const handleContextAction = useCallback((action: string, nodeId?: string) => {
    setContextMenuState(null);
    if (!nodeId) return;

    switch (action) {
      case "inspect":
        const n = nodes.find((x) => x.id === nodeId);
        if (n) onNodeClick?.(n);
        break;
      case "correlate":
        onCorrelate?.(nodeId);
        break;
      case "lock":
        lockNode(nodeId);
        break;
      case "hide":
        hideNode(nodeId);
        break;
    }
  }, [nodes, onNodeClick, onCorrelate, lockNode, hideNode]);

  const showTooltip = tooltip && !menuState && !contextMenuState;

  return (
    <div className="relative w-full h-full">
      {/* Cytoscape Container */}
      <div
        ref={containerRef}
        className="w-full h-full bg-gray-900"
        style={{ minHeight: "400px" }}
      />

      {/* Tooltip */}
      {showTooltip && (
        <div
          className="absolute z-10 pointer-events-none bg-dark-elevated/90 backdrop-blur border border-blue-500/30 text-xs text-white p-2 rounded shadow-xl"
          style={{
            left: tooltip.x,
            top: tooltip.y,
            transform: "translate(-50%, -100%)",
          }}
        >
          <div className="font-bold text-blue-400">{tooltip.data.label}</div>
          {tooltip.data.properties?.name && (
            <div className="text-gray-300">{tooltip.data.properties.name}</div>
          )}
        </div>
      )}

      {/* Radial Menu */}
      <RadialMenu
        isOpen={!!menuState}
        x={menuState?.x || 0}
        y={menuState?.y || 0}
        onClose={() => setMenuState(null)}
        onAction={handleRadialAction}
      />

      {/* Context Menu */}
      <ContextMenu
        isOpen={!!contextMenuState}
        x={contextMenuState?.x || 0}
        y={contextMenuState?.y || 0}
        node={contextMenuState?.node || null}
        onClose={() => setContextMenuState(null)}
        onAction={handleContextAction}
      />
    </div>
  );
}
