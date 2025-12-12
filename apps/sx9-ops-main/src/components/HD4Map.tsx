import React from 'react';
import EnhancedMap from './EnhancedMap';

interface HD4MapProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const HD4Map: React.FC<HD4MapProps> = ({ hd4Action }) => {
  return (
    <EnhancedMap 
      showLayerControls={true}
      showDemoData={true}
      className="h-full w-full"
    />
  );
};

export default HD4Map;