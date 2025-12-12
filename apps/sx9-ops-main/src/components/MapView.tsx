import React, { useState } from 'react';
import { AlertTriangle } from 'lucide-react';


interface MapViewProps {
  selectedSectors: unknown[];
  threatActors?: unknown[];
}

const MapView: React.FC<MapViewProps> = ({ selectedSectors, threatActors = [] }) => {
  const [error, setError] = useState<string | null>(null);

  return (
    <div className="h-full w-full bg-gray-800 flex items-center justify-center text-center p-4">
      <div>
        <AlertTriangle className="w-8 h-8 text-yellow-500 mx-auto mb-2" />
        <p className="text-white mb-2">Demo Mode - Global Operations Map</p>
        <p className="text-gray-400 text-sm">Add VITE_MAPBOX_TOKEN to enable live maps</p>
      </div>
    </div>
  );
};

export default MapView;