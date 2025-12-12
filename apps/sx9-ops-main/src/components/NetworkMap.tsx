import React, { useState } from 'react';
import Map, { NavigationControl, Marker } from 'react-map-gl';
import { Target, AlertTriangle } from 'lucide-react';
import 'mapbox-gl/dist/mapbox-gl.css';

interface GeoCoordinate {
  lat: number;
  lon: number;
}

interface VRavenInstance {
  id: string;
  name: string;
  location: GeoCoordinate;
}

const NetworkMap: React.FC = () => {
  const [error, setError] = useState<string | null>(null);
  const [viewState, setViewState] = useState({
    latitude: 20,
    longitude: 0,
    zoom: 1.5,
    bearing: 0,
    pitch: 0
  });

  // Use demo mode if no token is available
  const DEMO_MODE = !import.meta.env.VITE_MAPBOX_TOKEN;

  if (DEMO_MODE) {
    return (
      <div className="h-full w-full bg-gray-800 flex items-center justify-center text-center p-4">
        <div>
          <AlertTriangle className="w-8 h-8 text-yellow-500 mx-auto mb-2" />
          <p className="text-white mb-2">Demo Mode - Network Map</p>
          <p className="text-gray-400 text-sm">
            Add VITE_MAPBOX_TOKEN to your environment variables to enable live maps
          </p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="h-full w-full bg-gray-800 flex items-center justify-center text-red-500 p-4">
        <div className="text-center">
          <AlertTriangle className="w-8 h-8 mx-auto mb-2" />
          <p>{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full w-full relative">
      <Map
        {...viewState}
        onMove={evt => setViewState(evt.viewState)}
        mapStyle="mapbox://styles/mapbox/dark-v11"
        mapboxAccessToken={import.meta.env.VITE_MAPBOX_TOKEN}
        style={{ width: '100%', height: '100%' }}
        onError={(e) => setError(e.error.message)}
      >
        <NavigationControl position="top-right" />
      </Map>

      {/* Legend */}
      <div className="absolute bottom-8 left-8 bg-black bg-opacity-75 p-4 rounded-lg text-white">
        <div className="flex items-center mb-2">
          <AlertTriangle size={16} className="text-red-500 mr-2" />
          <span className="text-xs">Threat Actors</span>
        </div>
        <div className="flex items-center">
          <div className="w-4 h-4 bg-green-500 rounded-full border-2 border-white flex items-center justify-center mr-2">
            <span className="text-white text-xxs font-bold">v</span>
          </div>
          <span className="text-xs">vRaven Deployments</span>
        </div>
      </div>
    </div>
  );
};

export default NetworkMap;