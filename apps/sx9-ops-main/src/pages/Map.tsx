import React, { useState } from 'react';
import EnhancedMap from '@/components/EnhancedMap';
import WorkingMap from '@/components/WorkingMap';
import MapTest from '@/components/MapTest';
import MapDiagnostic from '@/components/MapDiagnostic';
import MapboxTest from '@/components/MapboxTest';
import SimpleMap from '@/components/SimpleMap';

const Map: React.FC = () => {
  const [showTest, setShowTest] = useState(false);
  const [showDiagnostic, setShowDiagnostic] = useState(false);
  const [showMapboxTest, setShowMapboxTest] = useState(false);
  const [mapType, setMapType] = useState<'enhanced' | 'simple' | 'working'>('working');

  return (
    <div className="h-full w-full bg-gray-900">
      <div className="h-full flex flex-col">
        <div className="flex-none p-4 border-b border-gray-700">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-xl font-semibold text-gray-300">CTAS Geospatial Intelligence</h1>
              <p className="text-sm text-gray-400 mt-1">
                Interactive map showing threat actors, infrastructure, targets, and security events
              </p>
            </div>
            <div className="flex space-x-2">
              <div className="flex items-center space-x-2">
                <span className="text-sm text-gray-400">Map Type:</span>
                <select
                  value={mapType}
                  onChange={(e) => setMapType(e.target.value as 'enhanced' | 'simple' | 'working')}
                  className="px-2 py-1 bg-gray-700 text-white rounded text-sm border border-gray-600"
                >
                  <option value="working">Working (Mapbox)</option>
                  <option value="enhanced">Enhanced (Mapbox)</option>
                  <option value="simple">Simple (CSS)</option>
                </select>
              </div>
              <button
                onClick={() => setShowDiagnostic(!showDiagnostic)}
                className="px-3 py-1 bg-yellow-600 text-white rounded text-sm hover:bg-yellow-700"
              >
                {showDiagnostic ? 'Hide Diagnostic' : 'Diagnostic'}
              </button>
              <button
                onClick={() => setShowMapboxTest(!showMapboxTest)}
                className="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700"
              >
                {showMapboxTest ? 'Hide Direct Test' : 'Direct Test'}
              </button>
              <button
                onClick={() => setShowTest(!showTest)}
                className="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700"
              >
                {showTest ? 'Hide Test' : 'Show Test'}
              </button>
            </div>
          </div>
        </div>
        
        {showDiagnostic && (
          <div className="flex-none p-4 border-b border-gray-700">
            <MapDiagnostic />
          </div>
        )}
        
        {showMapboxTest && (
          <div className="flex-none p-4 border-b border-gray-700">
            <MapboxTest />
          </div>
        )}
        
        {showTest && (
          <div className="flex-none p-4 border-b border-gray-700">
            <MapTest />
          </div>
        )}
        
        <div className="flex-1 p-4">
          {mapType === 'working' ? (
            <WorkingMap />
          ) : mapType === 'enhanced' ? (
            <EnhancedMap 
              showLayerControls={true}
              showDemoData={true}
              className="h-full w-full"
              height="600px"
              width="100%"
              resizable={true}
            />
          ) : (
            <SimpleMap />
          )}
        </div>
      </div>
    </div>
  );
};

export default Map;