import React, { useState, useEffect } from 'react';
import { Upload, Download, Search, Globe } from 'lucide-react';
import MapGL, { Source, Layer } from 'react-map-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

/**
 * GeospatialDataManager.tsx
 * Component for managing and visualizing geospatial data using GeoJSON
 * Author: Charlie Payne
 * Date: June 15, 2023
 * 
 * This component provides an interface for managing geospatial data in CTAS,
 * including data import, export, and visualization capabilities using GeoJSON format.
 * 
 * MVP:
 * - Basic GeoJSON data visualization
 * - Simple data import/export functionality
 * 
 * IOC:
 * - Integration with PostgreSQL/PostGIS for efficient geospatial queries
 * - Basic spatial analysis tools
 * 
 * Production:
 * - Advanced geospatial analytics and machine learning integration
 * - Real-time geospatial data processing and visualization
 * - Integration with external GIS services and data sources
 */

// import { queryGeospatialData } from '@/components/utils/postgresDriver';
interface GeoJSONFeature {
  type: 'Feature';
  geometry: {
    type: string;
    coordinates: number[] | number[][] | number[][][];
  };
  properties: {
    [key: string]: unknown;
  };
}

interface GeoJSONData {
  type: 'FeatureCollection';
  features: GeoJSONFeature[];
}

const GeospatialDataManager: React.FC = () => {
  const [geoJSONData, setGeoJSONData] = useState<GeoJSONData | null>(null);
  const [viewport, setViewport] = useState({
    latitude: 0,
    longitude: 0,
    zoom: 1
  });

  useEffect(() => {
    fetchGeospatialData();
  }, []);

  const fetchGeospatialData = async () => {
    try {
      // Mock geospatial data for now
      const mockData = [
        { geojson: '{"type":"Feature","geometry":{"type":"Point","coordinates":[-74.006,40.7128]},"properties":{"name":"New York"}}' },
        { geojson: '{"type":"Feature","geometry":{"type":"Point","coordinates":[-118.2437,34.0522]},"properties":{"name":"Los Angeles"}}' }
      ];
      const features = mockData.map((row: any) => JSON.parse(row.geojson));
      setGeoJSONData({
        type: 'FeatureCollection',
        features: features
      });
    } catch (error) {
      console.error('Error fetching geospatial data:', error);
    }
  };

  const importData = () => {
    // Implement GeoJSON data import functionality
    console.log('Importing GeoJSON data...');
  };

  const exportData = () => {
    // Implement GeoJSON data export functionality
    console.log('Exporting GeoJSON data...');
    if (geoJSONData) {
      const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(geoJSONData));
      const downloadAnchorNode = document.createElement('a');
      downloadAnchorNode.setAttribute("href", dataStr);
      downloadAnchorNode.setAttribute("download", "geospatial_data.geojson");
      document.body.appendChild(downloadAnchorNode);
      downloadAnchorNode.click();
      downloadAnchorNode.remove();
    }
  };

  return (
    <div className="bg-gray-900 text-white p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Globe className="mr-2" size={24} />
        Geospatial Data Manager
      </h2>

      <div className="flex space-x-4 mb-4">
        <button onClick={importData} className="flex items-center bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
          <Upload size={16} className="mr-2" />
          Import GeoJSON
        </button>
        <button onClick={exportData} className="flex items-center bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600">
          <Download size={16} className="mr-2" />
          Export GeoJSON
        </button>
      </div>

      <div className="mb-4 h-96">
        <MapGL
          {...viewport}
          style={{ width: '100%', height: '100%' }}
          mapStyle="mapbox://styles/mapbox/dark-v10"
          onMove={evt => setViewport(evt.viewState)}
          mapboxAccessToken={import.meta.env.VITE_MAPBOX_TOKEN || 'your-mapbox-token-here'}
        >
          {geoJSONData && (
            <Source type="geojson" data={geoJSONData as any}>
              <Layer
                id="data"
                type="fill"
                paint={{
                  'fill-color': '#088',
                  'fill-opacity': 0.8
                }}
              />
            </Source>
          )}
        </MapGL>
      </div>

      <div className="mb-4">
        <h3 className="text-lg font-semibold mb-2">GeoJSON Data Preview</h3>
        <pre className="bg-gray-800 p-4 rounded overflow-auto max-h-60">
          {JSON.stringify(geoJSONData, null, 2)}
        </pre>
      </div>
    </div>
  );
};

export default GeospatialDataManager;