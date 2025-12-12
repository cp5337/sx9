import React, { useEffect } from 'react';

const MapDiagnostic: React.FC = () => {
  useEffect(() => {
    console.log('🔍 MapDiagnostic mounted');
    console.log('🔍 Mapbox token:', import.meta.env.VITE_MAPBOX_ACCESS_TOKEN ? 'SET' : 'NOT SET');
    console.log('🔍 Token length:', import.meta.env.VITE_MAPBOX_ACCESS_TOKEN?.length || 0);
  }, []);

  return (
    <div style={{
      width: '100%',
      height: '100%',
      background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      color: 'white',
      fontFamily: 'monospace',
      padding: '20px',
      borderRadius: '8px'
    }}>
      <div style={{ textAlign: 'center', maxWidth: '600px' }}>
        <h2 style={{ fontSize: '24px', marginBottom: '20px' }}>🗺️ Map Diagnostic</h2>
        
        <div style={{ 
          background: 'rgba(0,0,0,0.3)', 
          padding: '15px', 
          borderRadius: '8px',
          marginBottom: '15px',
          textAlign: 'left'
        }}>
          <p style={{ margin: '5px 0' }}>
            <strong>Mapbox Token:</strong> {import.meta.env.VITE_MAPBOX_ACCESS_TOKEN ? '✅ SET' : '❌ NOT SET'}
          </p>
          <p style={{ margin: '5px 0' }}>
            <strong>Token Length:</strong> {import.meta.env.VITE_MAPBOX_ACCESS_TOKEN?.length || 0} chars
          </p>
          <p style={{ margin: '5px 0' }}>
            <strong>Token Preview:</strong> {import.meta.env.VITE_MAPBOX_ACCESS_TOKEN?.substring(0, 20)}...
          </p>
        </div>

        <p style={{ fontSize: '14px', opacity: 0.9 }}>
          If you see this, the map component is mounting but Mapbox isn't initializing.
        </p>
        <p style={{ fontSize: '12px', opacity: 0.7, marginTop: '10px' }}>
          Check browser console (F12) for detailed errors.
        </p>
      </div>
    </div>
  );
};

export default MapDiagnostic;
