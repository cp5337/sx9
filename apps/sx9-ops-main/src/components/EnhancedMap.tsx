import React, { useState, useEffect, useRef } from 'react';
import { 
  Layers, 
  Eye, 
  EyeOff, 
  Settings, 
  Download, 
  RefreshCw, 
  ZoomIn, 
  ZoomOut,
  Target,
  Globe,
  Activity,
  AlertTriangle,
  Database,
  Map as MapIcon
} from 'lucide-react';
import { mapboxService, CTASLayer } from '@/utils/mapboxService';
import { demoDataTracker } from '@/utils/demoDataTracker';
import FallbackMap from './FallbackMap';
import SimpleMap from './SimpleMap';
import 'mapbox-gl/dist/mapbox-gl.css';

interface EnhancedMapProps {
  className?: string;
  showLayerControls?: boolean;
  showDemoData?: boolean;
  height?: string;
  width?: string;
  resizable?: boolean;
}

const EnhancedMap: React.FC<EnhancedMapProps> = ({ 
  className = '', 
  showLayerControls = true,
  showDemoData = false,
  height = '100%',
  width = '100%',
  resizable = false
}) => {
  const mapContainerRef = useRef<HTMLDivElement>(null);
  const [containerId] = useState(() => `enhanced-map-container-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`);
  const [layers, setLayers] = useState<CTASLayer[]>([]);
  const [isInitialized, setIsInitialized] = useState(false);
  const [showControls, setShowControls] = useState(false);
  const [selectedLayer, setSelectedLayer] = useState<string | null>(null);
  const [isResizing, setIsResizing] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Initialize map
  useEffect(() => {
    if (mapContainerRef.current && !isInitialized) {
      const initMap = async () => {
        try {
          console.log('EnhancedMap: Starting map initialization...', containerId);
          setIsLoading(true);
          setError(null);
          await mapboxService.initialize(containerId);
          console.log('EnhancedMap: Map initialized successfully');
          setIsInitialized(true);
          setIsLoading(false);
          setLayers(mapboxService.getLayers());
        } catch (error) {
          console.error('EnhancedMap: Failed to initialize map:', error);
          setError(error instanceof Error ? error.message : 'Failed to load map');
          setIsLoading(false);
          // Show fallback content
          if (mapContainerRef.current) {
            mapContainerRef.current.innerHTML = `
              <div style="
                width: 100%; 
                height: 100%; 
                background: linear-gradient(45deg, #1a1a1a, #2d2d2d);
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                font-family: monospace;
                border-radius: 8px;
              ">
                <div style="text-align: center;">
                  <h3>Map Error</h3>
                  <p>${error instanceof Error ? error.message : 'Failed to load map'}</p>
                  <p style="font-size: 12px; margin-top: 10px; opacity: 0.7;">
                    Check console for details
                  </p>
                </div>
              </div>
            `;
          }
        }
      };

      // Add a small delay to ensure DOM is ready
      setTimeout(initMap, 100);
    }

    return () => {
      // Cleanup
      try {
        mapboxService.destroy();
      } catch (error) {
        console.error('EnhancedMap: Error during cleanup:', error);
      }
    };
  }, [isInitialized]);

  // Resize map when container size changes
  useEffect(() => {
    if (!isInitialized || !mapContainerRef.current) return;

    const resizeObserver = new ResizeObserver(() => {
      try {
        // Small delay to ensure container has finished resizing
        setTimeout(() => {
          mapboxService.resize();
          console.log('EnhancedMap: Map resized to fit container');
        }, 50);
      } catch (error) {
        console.error('EnhancedMap: Error resizing map:', error);
      }
    });

    resizeObserver.observe(mapContainerRef.current);

    return () => {
      resizeObserver.disconnect();
    };
  }, [isInitialized]);

  // Add demo data heatmap if enabled
  useEffect(() => {
    if (showDemoData && isInitialized) {
      const events = demoDataTracker.getEvents();
      if (events.length > 0) {
        mapboxService.addDemoDataHeatmap(events);
        setLayers(mapboxService.getLayers());
      }
    }
  }, [showDemoData, isInitialized]);

  // Toggle layer visibility
  const toggleLayer = (layerId: string) => {
    mapboxService.toggleLayer(layerId);
    setLayers(mapboxService.getLayers());
  };

  // Set layer opacity
  const setLayerOpacity = (layerId: string, opacity: number) => {
    mapboxService.setLayerOpacity(layerId, opacity);
    setLayers(mapboxService.getLayers());
  };

  // Export map data
  const exportMapData = () => {
    const data = {
      layers: layers,
      timestamp: new Date().toISOString(),
      demoData: showDemoData ? demoDataTracker.getEvents() : []
    };

    const dataStr = JSON.stringify(data, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `ctas-map-data-${new Date().toISOString().split('T')[0]}.json`;
    link.click();
    URL.revokeObjectURL(url);
  };

  // Get layer icon
  const getLayerIcon = (type: string) => {
    switch (type) {
      case 'threat-actors': return <AlertTriangle className="w-4 h-4" />;
      case 'infrastructure': return <Globe className="w-4 h-4" />;
      case 'targets': return <Target className="w-4 h-4" />;
      case 'events': return <Activity className="w-4 h-4" />;
      case 'heatmap': return <Database className="w-4 h-4" />;
      case 'network': return <MapIcon className="w-4 h-4" />;
      default: return <Layers className="w-4 h-4" />;
    }
  };

  // Get layer color
  const getLayerColor = (layer: CTASLayer) => {
    return layer.color || '#ffffff';
  };

  return (
    <div 
      className={`relative ${className}`}
      style={{ 
        height: height, 
        width: width,
        minHeight: '200px'
      }}
    >
      {/* Map Container - Full bleed, no padding */}
      <div 
        id={containerId}
        ref={mapContainerRef}
        className="absolute inset-0 w-full h-full"
        style={{ 
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0
        }}
      />

      {/* Loading Overlay */}
      {isLoading && (
        <div className="absolute inset-0 bg-gray-900/75 flex items-center justify-center z-30">
          <div className="text-center text-white">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-white mx-auto mb-2"></div>
            <p className="text-sm">Loading Map...</p>
          </div>
        </div>
      )}

      {/* Error Overlay */}
      {error && !isLoading && (
        <div className="absolute inset-0 z-30">
          <SimpleMap />
          <div className="absolute top-4 left-4 bg-red-900/90 text-white p-3 rounded-lg">
            <h3 className="text-sm font-semibold mb-1">Mapbox Error</h3>
            <p className="text-xs mb-2">{error}</p>
            <button 
              onClick={() => {
                setError(null);
                setIsInitialized(false);
                setIsLoading(true);
                // Reinitialize map
                setTimeout(() => {
                  mapboxService.initialize(containerId)
                    .then(() => {
                      setIsInitialized(true);
                      setIsLoading(false);
                      setLayers(mapboxService.getLayers());
                    })
                    .catch((err) => {
                      setError(err.message);
                      setIsLoading(false);
                    });
                }, 100);
              }}
              className="px-2 py-1 bg-white text-red-900 rounded text-xs hover:bg-gray-100"
            >
              Retry Mapbox
            </button>
          </div>
        </div>
      )}

          {/* Layer Controls - REMOVED (useless with 0 layers) */}
          {false && showLayerControls && (
            <div className="absolute top-2 right-2 z-30">
          <div className="bg-gray-900/90 backdrop-blur-sm rounded-lg shadow-lg border border-gray-700">
            {/* Controls Header */}
            <div className="flex items-center justify-between p-3 border-b border-gray-700">
              <div className="flex items-center">
                <Layers className="w-4 h-4 mr-2 text-gray-300" />
                <span className="text-sm font-medium text-gray-300">Layers</span>
              </div>
              <div className="flex items-center space-x-2">
                <button
                  onClick={() => setShowControls(!showControls)}
                  className="p-1 text-gray-400 hover:text-gray-300"
                  title={showControls ? 'Hide Controls' : 'Show Controls'}
                >
                  <Settings className="w-4 h-4" />
                </button>
                <button
                  onClick={exportMapData}
                  className="p-1 text-gray-400 hover:text-gray-300"
                  title="Export Map Data"
                >
                  <Download className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Layer List */}
            {showControls && (
              <div className="p-3 max-h-96 overflow-y-auto">
                <div className="space-y-3">
                  {layers.map((layer) => (
                    <div key={layer.id} className="space-y-2">
                      {/* Layer Header */}
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-2">
                          <button
                            onClick={() => toggleLayer(layer.id)}
                            className="p-1 text-gray-400 hover:text-gray-300"
                            title={layer.visible ? 'Hide Layer' : 'Show Layer'}
                          >
                            {layer.visible ? <Eye className="w-4 h-4" /> : <EyeOff className="w-4 h-4" />}
                          </button>
                          <div 
                            className="w-3 h-3 rounded-full"
                            style={{ backgroundColor: getLayerColor(layer) }}
                          />
                          <span className="text-xs text-gray-300">{layer.name}</span>
                        </div>
                        <div className="flex items-center space-x-1">
                          <span className="text-xs text-gray-500">
                            {layer.data.length}
                          </span>
                        </div>
                      </div>

                      {/* Layer Controls */}
                      {layer.visible && (
                        <div className="ml-6 space-y-2">
                          {/* Opacity Control */}
                          <div className="flex items-center space-x-2">
                            <span className="text-xs text-gray-500 w-12">Opacity:</span>
                            <input
                              type="range"
                              min="0"
                              max="1"
                              step="0.1"
                              value={layer.opacity}
                              onChange={(e) => setLayerOpacity(layer.id, parseFloat(e.target.value))}
                              className="flex-1 h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                            />
                            <span className="text-xs text-gray-500 w-8">
                              {Math.round(layer.opacity * 100)}%
                            </span>
                          </div>

                          {/* Layer Info */}
                          <div className="text-xs text-gray-500 space-y-1">
                            <div>Type: {layer.type}</div>
                            <div>Points: {layer.data.length}</div>
                            {layer.size && <div>Size: {layer.size}px</div>}
                          </div>
                        </div>
                      )}
                    </div>
                  ))}
                </div>

                {/* Demo Data Toggle */}
                {showDemoData && (
                  <div className="mt-4 pt-3 border-t border-gray-700">
                    <div className="flex items-center justify-between">
                      <span className="text-xs text-gray-300">Demo Data Heatmap</span>
                      <button
                        onClick={() => {
                          const events = demoDataTracker.getEvents();
                          if (events.length > 0) {
                            mapboxService.addDemoDataHeatmap(events);
                            setLayers(mapboxService.getLayers());
                          }
                        }}
                        className="p-1 text-gray-400 hover:text-gray-300"
                        title="Refresh Demo Data"
                      >
                        <RefreshCw className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      {/* Map Controls - Simple zoom buttons */}
      <div className="absolute bottom-4 left-4 z-30">
        <div className="bg-gray-900/90 backdrop-blur-sm rounded shadow-lg border border-gray-700">
          <button
            onClick={() => mapboxService.zoomIn()}
            className="block px-3 py-2 text-gray-300 hover:text-white hover:bg-blue-600 transition-colors border-b border-gray-700"
            title="Zoom In"
          >
            <span className="text-lg font-bold">+</span>
          </button>
          <button
            onClick={() => mapboxService.zoomOut()}
            className="block px-3 py-2 text-gray-300 hover:text-white hover:bg-blue-600 transition-colors"
            title="Zoom Out"
          >
            <span className="text-lg font-bold">−</span>
          </button>
        </div>
      </div>


      {/* Resize Handle */}
      {resizable && (
        <div 
          className="absolute bottom-0 right-0 w-4 h-4 cursor-se-resize z-20"
          style={{
            background: 'linear-gradient(135deg, transparent 50%, #6b7280 50%)',
            borderRadius: '0 0 8px 0'
          }}
          onMouseDown={(e) => {
            e.preventDefault();
            setIsResizing(true);
            
            const startX = e.clientX;
            const startY = e.clientY;
            const startWidth = mapContainerRef.current?.parentElement?.offsetWidth || 0;
            const startHeight = mapContainerRef.current?.parentElement?.offsetHeight || 0;
            
            const handleMouseMove = (moveEvent: MouseEvent) => {
              const deltaX = moveEvent.clientX - startX;
              const deltaY = moveEvent.clientY - startY;
              
              if (mapContainerRef.current?.parentElement) {
                const newWidth = Math.max(400, startWidth + deltaX);
                const newHeight = Math.max(300, startHeight + deltaY);
                
                mapContainerRef.current.parentElement.style.width = `${newWidth}px`;
                mapContainerRef.current.parentElement.style.height = `${newHeight}px`;
                
                // Trigger map resize
                mapboxService.resize();
              }
            };
            
            const handleMouseUp = () => {
              setIsResizing(false);
              document.removeEventListener('mousemove', handleMouseMove);
              document.removeEventListener('mouseup', handleMouseUp);
            };
            
            document.addEventListener('mousemove', handleMouseMove);
            document.addEventListener('mouseup', handleMouseUp);
          }}
        />
      )}

      {/* Custom CSS for slider */}
      <style>{`
        .slider::-webkit-slider-thumb {
          appearance: none;
          height: 12px;
          width: 12px;
          border-radius: 50%;
          background: #3b82f6;
          cursor: pointer;
        }
        .slider::-moz-range-thumb {
          height: 12px;
          width: 12px;
          border-radius: 50%;
          background: #3b82f6;
          cursor: pointer;
          border: none;
        }
      `}</style>
    </div>
  );
};

export default EnhancedMap;
