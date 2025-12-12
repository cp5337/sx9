import React, { useEffect, useRef } from 'react';

const MapTest: React.FC = () => {
  const mapContainer = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const initMap = async () => {
      try {
        console.log('MapTest: Starting simple map test...');
        
        // Check if mapbox-gl is available
        const mapboxgl = await import('mapbox-gl');
        console.log('MapTest: mapbox-gl imported successfully');
        
        if (!mapContainer.current) {
          console.error('MapTest: Container not found');
          return;
        }

        const token = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;
        console.log('MapTest: Token available:', !!token);

        const map = new mapboxgl.Map({
          container: mapContainer.current,
          style: 'mapbox://styles/mapbox/dark-v11',
          center: [-74.006, 40.7128],
          zoom: 10,
          accessToken: token || 'demo-token'
        });

        map.on('load', () => {
          console.log('MapTest: Map loaded successfully!');
        });

        map.on('error', (error) => {
          console.error('MapTest: Map error:', error);
        });

      } catch (error) {
        console.error('MapTest: Failed to initialize map:', error);
      }
    };

    initMap();
  }, []);

  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Map Test</h2>
      <div 
        ref={mapContainer} 
        className="w-full h-96 border border-gray-300 rounded-lg"
        style={{ minHeight: '400px' }}
      />
    </div>
  );
};

export default MapTest;
