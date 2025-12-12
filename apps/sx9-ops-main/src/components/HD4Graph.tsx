import React, { useEffect, useRef } from 'react';
import ForceGraph2D from 'react-force-graph-2d';

interface HD4GraphProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const HD4Graph: React.FC<HD4GraphProps> = ({ hd4Action }) => {
  const fgRef = useRef<any>();
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (fgRef.current) {
      fgRef.current.d3Force('charge').strength(-300);
      fgRef.current.d3Force('link').distance(50);
      fgRef.current.d3Force('center', null);
      fgRef.current.zoomToFit(400);
    }
  }, []);

  const graphData = {
    nodes: [
      { id: hd4Action, name: hd4Action, val: 20, color: '#4299E1' },
      // Add more nodes as needed
    ],
    links: [
      // Add links as needed
    ]
  };

  return (
    <div ref={containerRef} className="w-full h-full bg-gray-900 rounded-lg shadow-lg overflow-hidden">
      <ForceGraph2D
        ref={fgRef}
        graphData={graphData}
        nodeLabel="name"
        nodeColor={(node: any) => node.color}
        nodeVal={(node: any) => node.val}
        linkColor={() => 'rgba(255,255,255,0.2)'}
        backgroundColor="#1A202C"
        width={containerRef.current?.clientWidth || 800}
        height={containerRef.current?.clientHeight || 600}
      />
    </div>
  );
};

export default HD4Graph;