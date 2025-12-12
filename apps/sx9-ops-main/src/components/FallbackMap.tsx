import React from 'react';

const FallbackMap: React.FC = () => {
  return (
    <div className="h-full w-full bg-gray-900 rounded-lg overflow-hidden relative">
      {/* World Map Background */}
      <div className="absolute inset-0 bg-gradient-to-br from-blue-900 via-blue-800 to-blue-900">
        {/* Continents */}
        <div className="absolute top-1/4 left-1/4 w-1/3 h-1/3 bg-green-800 rounded-full opacity-30"></div>
        <div className="absolute top-1/3 right-1/4 w-1/4 h-1/4 bg-green-800 rounded-full opacity-30"></div>
        <div className="absolute bottom-1/4 left-1/3 w-1/3 h-1/4 bg-green-800 rounded-full opacity-30"></div>
        
        {/* Grid Lines */}
        <div className="absolute inset-0 opacity-20">
          {Array.from({ length: 10 }, (_, i) => (
            <div
              key={`h-${i}`}
              className="absolute w-full border-t border-white"
              style={{ top: `${i * 10}%` }}
            />
          ))}
          {Array.from({ length: 20 }, (_, i) => (
            <div
              key={`v-${i}`}
              className="absolute h-full border-l border-white"
              style={{ left: `${i * 5}%` }}
            />
          ))}
        </div>
      </div>

      {/* Demo Data Points */}
      <div className="absolute top-1/4 left-1/3 w-3 h-3 bg-red-500 rounded-full animate-pulse">
        <div className="absolute -top-8 -left-8 bg-red-600 text-white text-xs px-2 py-1 rounded whitespace-nowrap">
          APT29
        </div>
      </div>
      
      <div className="absolute top-1/2 right-1/3 w-3 h-3 bg-green-500 rounded-full animate-pulse">
        <div className="absolute -top-8 -left-8 bg-green-600 text-white text-xs px-2 py-1 rounded whitespace-nowrap">
          C2 Server
        </div>
      </div>
      
      <div className="absolute bottom-1/3 left-1/2 w-3 h-3 bg-blue-500 rounded-full animate-pulse">
        <div className="absolute -top-8 -left-8 bg-blue-600 text-white text-xs px-2 py-1 rounded whitespace-nowrap">
          Target
        </div>
      </div>

      {/* Overlay */}
      <div className="absolute inset-0 bg-black bg-opacity-20 flex items-center justify-center">
        <div className="text-center text-white">
          <h3 className="text-lg font-semibold mb-2">Fallback Map View</h3>
          <p className="text-sm opacity-80">
            Mapbox is not available. This is a simplified view.
          </p>
          <div className="mt-4 flex justify-center space-x-4 text-xs">
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-red-500 rounded-full"></div>
              <span>Threat Actors</span>
            </div>
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-green-500 rounded-full"></div>
              <span>Infrastructure</span>
            </div>
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
              <span>Targets</span>
            </div>
          </div>
        </div>
      </div>

      {/* Controls */}
      <div className="absolute top-4 right-4 bg-gray-800 bg-opacity-75 rounded-lg p-2">
        <div className="text-white text-xs">
          <div className="font-semibold mb-1">Demo Mode</div>
          <div>Layers: 3</div>
          <div>Points: 3</div>
        </div>
      </div>
    </div>
  );
};

export default FallbackMap;
