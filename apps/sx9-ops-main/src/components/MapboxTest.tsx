import React, { useEffect, useRef, useState } from 'react';

const MapboxTest: React.FC = () => {
  const mapContainer = useRef<HTMLDivElement>(null);
  const map = useRef<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initMap = async () => {
      if (map.current) return; // Initialize map only once

      try {
        setStatus('Loading mapbox-gl...');
        
        // Check token
        const token = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;
        console.log('Token check:', {
          token: token ? `${token.substring(0, 20)}...` : 'NOT SET',
          tokenLength: token?.length || 0
        });

        if (!token) {
          throw new Error('No Mapbox access token found');
        }

        // Import mapbox-gl
        const mapboxgl = await import('mapbox-gl');
        setStatus('Mapbox GL loaded, creating map...');

        // Set access token
        mapboxgl.accessToken = token;

        // Create map
        map.current = new mapboxgl.Map({
          container: mapContainer.current!,
          style: 'mapbox://styles/mapbox/dark-v11',
          center: [-74.006, 40.7128], // New York
          zoom: 10
        });

        setStatus('Map created, waiting for load...');

        map.current.on('load', () => {
          setStatus('Map loaded successfully!');
          console.log('Map loaded successfully');
        });

        map.current.on('error', (e: any) => {
          setError(`Map error: ${e.error?.message || 'Unknown error'}`);
          console.error('Map error:', e);
        });

      } catch (err: any) {
        setError(`Initialization error: ${err.message}`);
        console.error('MapboxTest error:', err);
      }
    };

    initMap();

    return () => {
      if (map.current) {
        map.current.remove();
      }
    };
  }, []);

  return (
    <div className="bg-gray-800 p-4 rounded-lg">
      <h3 className="text-lg font-semibold text-white mb-4">Mapbox Direct Test</h3>
      
      <div className="mb-4 text-sm">
        <div className="text-gray-300 mb-2">
          <strong>Status:</strong> {status}
        </div>
        {error && (
          <div className="text-red-400 mb-2">
            <strong>Error:</strong> {error}
          </div>
        )}
        <div className="text-gray-400">
          <strong>Token:</strong> {import.meta.env.VITE_MAPBOX_ACCESS_TOKEN ? 
            `${import.meta.env.VITE_MAPBOX_ACCESS_TOKEN.substring(0, 20)}...` : 
            'NOT SET'
          }
        </div>
      </div>

      <div 
        ref={mapContainer} 
        className="w-full h-64 bg-gray-700 rounded border border-gray-600"
        style={{ minHeight: '256px' }}
      />
    </div>
  );
};

export default MapboxTest;
