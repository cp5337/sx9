import React from 'react';
import EnhancedMap from './EnhancedMap';

interface DeployedAsset {
  id: string;
  name: string;
  type: 'raptor' | 'vkali' | 'database' | 'service';
  status: 'active' | 'inactive' | 'deploying' | 'error';
  location: {
    lat: number;
    lng: number;
  };
  metrics: {
    cpu: number;
    memory: number;
    network: number;
  };
  lastSeen: string;
}

interface GISMapProps {
  deployedAssets?: DeployedAsset[];
}

const GISMap: React.FC<GISMapProps> = ({ deployedAssets = [] }) => {
  return (
    <div className="h-full w-full">
      <EnhancedMap 
        showLayerControls={true}
        showDemoData={true}
        className="h-full w-full"
      />
    </div>
  );
};

export default GISMap;
