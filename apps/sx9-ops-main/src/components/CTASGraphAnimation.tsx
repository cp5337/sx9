import React, { useEffect, useRef } from 'react';

const CTASGraphAnimation: React.FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  // // const [dimensions, setDimensions] = useState(...);
  // // const [hoveredNode, setHoveredNode] = useState(...);
  // const [graphData] = useState(getGraphData());

  useEffect(() => {
    const updateDimensions = () => {
      // if (containerRef.current) {
      //   setDimensions({
      //     width: containerRef.current.clientWidth,
      //     height: containerRef.current.clientHeight
      //   });
      // }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  return (
    <div ref={containerRef} className="w-full h-full bg-gray-900 rounded-lg shadow-lg overflow-hidden relative">
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="text-white text-center">
          <h3 className="text-xl font-bold mb-2">Graph Visualization</h3>
          <p className="text-gray-400">Initializing graph visualization...</p>
        </div>
      </div>

      {/* {hoveredNode && (
        <div 
          className="absolute bg-gray-800 text-white px-3 py-2 rounded-lg shadow-lg border border-gray-700 text-xs"
          style={{ left: '1rem', bottom: '1rem' }}
        >
          <div className="font-semibold mb-1">{hoveredNode.name}</div>
          <div className="text-gray-300">{hoveredNode.group}</div>
        </div>
      )} */}
    </div>
  );
};

export default CTASGraphAnimation;