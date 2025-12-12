import React from 'react';
import { Network } from 'lucide-react';
import ForceGraph2D from 'react-force-graph-2d';

interface TransformsGraphProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const TransformsGraph: React.FC<TransformsGraphProps> = ({ hd4Action }) => {
  const graphData = {
    nodes: [
      { id: 'task1', name: `${hd4Action} Task 1`, group: 1 },
      { id: 'task2', name: `${hd4Action} Task 2`, group: 1 },
      { id: 'transform1', name: 'Transform 1', group: 2 },
      { id: 'transform2', name: 'Transform 2', group: 2 },
    ],
    links: [
      { source: 'task1', target: 'transform1' },
      { source: 'task2', target: 'transform1' },
      { source: 'task1', target: 'transform2' },
    ]
  };

  return (
    <div className="bg-gray-800 p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Network className="mr-2" size={20} />
        {hd4Action} Transforms Graph
      </h2>
      <div style={{ height: '60vh' }}>
        <ForceGraph2D
          graphData={graphData}
          nodeLabel="name"
          nodeColor={node => node.group === 1 ? '#4299E1' : '#48BB78'}
          linkColor={() => '#718096'}
          nodeCanvasObject={(node, ctx, globalScale) => {
            const label = node.name || 'Unknown';
            const fontSize = 12/globalScale;
            ctx.font = `${fontSize}px Sans-Serif`;
            const textWidth = ctx.measureText(label).width;
            const bckgDimensions = [textWidth, fontSize].map(n => n + fontSize * 0.2);

            ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
            if (node.x !== undefined && node.y !== undefined && bckgDimensions[0] !== undefined && bckgDimensions[1] !== undefined) {
              ctx.fillRect(node.x - bckgDimensions[0] / 2, node.y - bckgDimensions[1] / 2, bckgDimensions[0], bckgDimensions[1]);

              ctx.textAlign = 'center';
              ctx.textBaseline = 'middle';
              ctx.fillStyle = node.color || '#000000';
              ctx.fillText(label, node.x, node.y);
            }
          }}
        />
      </div>
    </div>
  );
};

export default TransformsGraph;