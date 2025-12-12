import React, { useEffect, useRef } from 'react';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

// SUPER SIMPLE TEST - NO BULLSHIT
const MapTest: React.FC = () => {
  const mapContainer = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const token = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;
    
    console.log('🗺️ MapTest: Token exists?', !!token);
    console.log('🗺️ MapTest: Token value:', token);
    
    if (!token) {
      alert('NO MAPBOX TOKEN!');
      return;
    }

    mapboxgl.accessToken = token;

    const map = new mapboxgl.Map({
      container: mapContainer.current!,
      style: 'mapbox://styles/mapbox/dark-v11',
      center: [-74.006, 40.7128],
      zoom: 10
    });

    map.on('load', () => {
      console.log('✅ MapTest: Map loaded successfully!');
      alert('MAP WORKS!');
    });

    map.on('error', (e) => {
      console.error('❌ MapTest: Map error:', e);
      alert('MAP ERROR: ' + e.error.message);
    });

    return () => map.remove();
  }, []);

  return (
    <div style={{ width: '100%', height: '100vh', background: '#000' }}>
      <div style={{
        position: 'absolute',
        top: '20px',
        left: '20px',
        zIndex: 1000,
        background: 'rgba(255,255,255,0.9)',
        padding: '20px',
        borderRadius: '8px'
      }}>
        <h1 style={{ margin: 0, color: '#000' }}>MAPBOX TEST</h1>
        <p style={{ margin: '10px 0 0 0', color: '#000' }}>If you see a map below, it works!</p>
      </div>
      <div ref={mapContainer} style={{ width: '100%', height: '100%' }} />
    </div>
  );
};

export default MapTest;

