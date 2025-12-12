import React, { useState, useEffect, useMemo } from 'react';
import ForceGraph2D from 'react-force-graph-2d';

export interface GraphNode {
  id: string;
  label: string;
  type: string;
  properties?: Record<string, any>;
  x?: number;
  y?: number;
}

export interface GraphLink {
  source: string;
  target: string;
  type: string;
  properties?: Record<string, any>;
}

export interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

interface GraphContainerProps {
  data: GraphData;
  height?: number;
  width?: number;
  showControls?: boolean;
  onNodeClick?: (node: GraphNode) => void;
  onLinkClick?: (link: GraphLink) => void;
  nodeColor?: (node: GraphNode) => string;
  linkColor?: (link: GraphLink) => string;
}

const GraphContainer: React.FC<GraphContainerProps> = ({
  data,
  height = 600,
  width = 800,
  showControls = true,
  onNodeClick,
  onLinkClick,
  nodeColor = () => '#1f2937',
  linkColor = () => '#6b7280'
}) => {
  const [graphData, setGraphData] = useState<GraphData>(data);
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);

  useEffect(() => {
    setGraphData(data);
  }, [data]);

  const handleNodeClick = (node: GraphNode) => {
    setSelectedNode(node);
    onNodeClick?.(node);
  };

  const handleLinkClick = (link: GraphLink) => {
    onLinkClick?.(link);
  };

  const graphConfig = useMemo(() => ({
    nodeRelSize: 6,
    linkWidth: 2,
    linkDirectionalParticles: 2,
    linkDirectionalParticleSpeed: 0.005,
    d3VelocityDecay: 0.3,
    cooldownTicks: 100,
    nodeCanvasObject: (node: GraphNode, ctx: CanvasRenderingContext2D, globalScale: number) => {
      const label = node.label;
      const fontSize = 12/globalScale;
      ctx.font = `${fontSize}px Sans-Serif`;
      const textWidth = ctx.measureText(label).width;
      const bckgDimensions = [textWidth, fontSize].map(n => n + fontSize * 0.2);

      ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
      ctx.fillRect((node.x || 0) - (bckgDimensions[0] || 0) / 2, (node.y || 0) - (bckgDimensions[1] || 0) / 2, bckgDimensions[0] || 0, bckgDimensions[1] || 0);

      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillStyle = nodeColor(node);
      ctx.fillText(label, node.x || 0, node.y || 0);
    }
  }), [nodeColor]);

  return (
    <div className="graph-container">
      {showControls && (
        <div className="graph-controls mb-4 p-2 bg-gray-100 dark:bg-gray-800 rounded">
          <div className="flex items-center justify-between">
            <span className="text-sm">
              Nodes: {graphData.nodes.length} | Links: {graphData.links.length}
            </span>
            {selectedNode && (
              <div className="text-sm">
                Selected: {selectedNode.label} ({selectedNode.type})
              </div>
            )}
          </div>
        </div>
      )}
      
      <div className="graph-renderer border border-gray-300 dark:border-gray-600 rounded">
        <ForceGraph2D
          graphData={graphData}
          height={height}
          width={width}
          onNodeClick={handleNodeClick}
          onLinkClick={handleLinkClick}
          nodeColor={nodeColor}
          linkColor={linkColor}
          {...graphConfig}
        />
      </div>
    </div>
  );
};

export default GraphContainer;

