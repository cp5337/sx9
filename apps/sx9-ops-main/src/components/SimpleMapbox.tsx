import React, { useEffect, useRef, useState } from 'react';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

interface SimpleMapboxProps {
  className?: string;
}

/**
 * Dead simple Mapbox implementation following official docs
 * https://docs.mapbox.com/mapbox-gl-js/guides/
 */
const SimpleMapbox: React.FC<SimpleMapboxProps> = ({ className = '' }) => {
  const mapContainer = useRef<HTMLDivElement>(null);
  const map = useRef<mapboxgl.Map | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (map.current) return; // Initialize map only once

    const token = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;
    
    if (!token) {
      setError('No Mapbox token found in VITE_MAPBOX_ACCESS_TOKEN');
      setIsLoading(false);
      return;
    }

    if (!mapContainer.current) {
      setError('Map container ref is null');
      setIsLoading(false);
      return;
    }

    try {
      console.log('🗺️  SimpleMapbox: Initializing with token:', token.substring(0, 20) + '...');
      
      // Set access token (required by Mapbox GL JS)
      mapboxgl.accessToken = token;

      // Create map instance
      map.current = new mapboxgl.Map({
        container: mapContainer.current,
        style: 'mapbox://styles/mapbox/dark-v11',
        center: [-98.5795, 39.8283], // Center of US
        zoom: 4
      });

      // Navigation controls removed - mouse wheel zoom is enabled by default

      // Handle load event
      map.current.on('load', () => {
        console.log('✅ SimpleMapbox: Map loaded successfully');
        setIsLoading(false);
        setError(null);
      });

      // Handle errors
      map.current.on('error', (e) => {
        console.error('❌ SimpleMapbox: Map error:', e);
        setError(`Mapbox error: ${e.error?.message || 'Unknown error'}`);
        setIsLoading(false);
      });

    } catch (err) {
      console.error('❌ SimpleMapbox: Failed to initialize:', err);
      setError(err instanceof Error ? err.message : 'Failed to initialize map');
      setIsLoading(false);
    }

    // Cleanup on unmount
    return () => {
      if (map.current) {
        map.current.remove();
        map.current = null;
      }
    };
  }, []);

  // Handle window resize
  useEffect(() => {
    if (!map.current) return;

    const handleResize = () => {
      map.current?.resize();
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  if (error) {
    return (
      <div className={`w-full h-full flex items-center justify-center bg-gray-900 ${className}`}>
        <div className="text-center text-white p-8">
          <div className="text-red-500 text-6xl mb-4">⚠️</div>
          <h3 className="text-xl font-bold mb-2">Mapbox Error</h3>
          <p className="text-gray-400 text-sm mb-4">{error}</p>
          <div className="text-xs text-gray-500">
            <p>Check:</p>
            <ul className="list-disc list-inside mt-2">
              <li>VITE_MAPBOX_ACCESS_TOKEN in .env</li>
              <li>Token is valid (not expired)</li>
              <li>Token starts with "pk."</li>
              <li>Browser console for details</li>
            </ul>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`relative w-full h-full ${className}`}>
      {/* Map Container */}
      <div 
        ref={mapContainer} 
        className="absolute inset-0 w-full h-full"
      />

      {/* Loading Overlay */}
      {isLoading && (
        <div className="absolute inset-0 bg-gray-900/90 flex items-center justify-center z-50">
          <div className="text-center text-white">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
            <p className="text-sm">Loading Mapbox...</p>
          </div>
        </div>
      )}
    </div>
  );
};

export default SimpleMapbox;

