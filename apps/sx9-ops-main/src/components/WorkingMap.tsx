import React, { useEffect, useRef, useState } from 'react';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

const WorkingMap: React.FC = () => {
  const mapContainer = useRef<HTMLDivElement>(null);
  const map = useRef<mapboxgl.Map | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (map.current) return; // Initialize map only once
    if (!mapContainer.current) return; // Wait for container to be ready

    const token = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;
    
    if (!token) {
      setError('Mapbox token not found in environment');
      console.error('❌ Mapbox token missing');
      return;
    }

    console.log('✅ Mapbox token found, initializing map...');
    mapboxgl.accessToken = token;

    try {
      const mapInstance = new mapboxgl.Map({
        container: mapContainer.current,
        style: 'mapbox://styles/mapbox/dark-v11',
        center: [-74.006, 40.7128], // NYC
        zoom: 10,
        attributionControl: false
      });

      map.current = mapInstance;

      // Add navigation controls
      mapInstance.addControl(new mapboxgl.NavigationControl(), 'top-right');

      // Add some demo markers
      mapInstance.on('load', () => {
        console.log('✅ Mapbox map loaded successfully');
        
        // Add a simple marker
        new mapboxgl.Marker({ color: '#FF0000' })
          .setLngLat([-74.006, 40.7128])
          .setPopup(new mapboxgl.Popup().setHTML('<h3>CTAS HQ</h3><p>New York City</p>'))
          .addTo(mapInstance);

        // Add another marker
        new mapboxgl.Marker({ color: '#00FF00' })
          .setLngLat([-118.2437, 34.0522])
          .setPopup(new mapboxgl.Popup().setHTML('<h3>West Coast</h3><p>Los Angeles</p>'))
          .addTo(mapInstance);
      });

      mapInstance.on('error', (e) => {
        console.error('❌ Mapbox error:', e.error);
        setError(`Mapbox error: ${e.error.message}`);
      });

    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to initialize map';
      setError(errorMsg);
      console.error('❌ Map initialization error:', err);
    }

    return () => {
      if (map.current) {
        map.current.remove();
        map.current = null;
      }
    };
  }, []);

  if (error) {
    return (
      <div className="h-full w-full flex items-center justify-center bg-gray-900 text-white">
        <div className="text-center">
          <h2 className="text-xl font-bold mb-2">Map Error</h2>
          <p className="text-gray-400">{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full w-full relative bg-gray-900">
      <div ref={mapContainer} className="absolute inset-0 w-full h-full z-0" />
      <div className="absolute top-4 left-4 z-50 bg-gray-900 bg-opacity-90 p-3 rounded-lg shadow-lg pointer-events-none">
        <h2 className="text-white font-bold text-sm mb-1">🗺️ CTAS Geospatial Intelligence</h2>
        <p className="text-gray-300 text-xs">Mapbox GL JS v7.3.1</p>
        <p className="text-green-400 text-xs mt-1">✅ Token loaded</p>
      </div>
    </div>
  );
};

export default WorkingMap;

