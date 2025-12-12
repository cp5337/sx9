import { DemoDataUsageEvent } from '@/types';

// Mapbox Configuration
const MAPBOX_CONFIG = {
  ACCESS_TOKEN: import.meta.env.VITE_MAPBOX_ACCESS_TOKEN,
  STYLE_URL: 'mapbox://styles/mapbox/dark-v11',
  DEMO_MODE: import.meta.env.VITE_DEMO_MODE === 'true'
};

// Layer Types for CTAS
export interface CTASLayer {
  id: string;
  type: 'people' | 'assets' | 'locations' | 'events' | 'network' | 'threats' | 'infrastructure' | 'heatmap';
  name: string;
  visible: boolean;
  opacity: number;
  data: any[];
  color?: string;
  size?: number;
  icon?: string;
}

// Mapbox Service for CTAS
export class MapboxService {
  private static instance: MapboxService;
  private map: any = null;
  private layers: Map<string, CTASLayer> = new Map();
  private isInitialized: boolean = false;

  private constructor() {}

  public static getInstance(): MapboxService {
    if (!MapboxService.instance) {
      MapboxService.instance = new MapboxService();
    }
    return MapboxService.instance;
  }

  // Initialize Mapbox
  public async initialize(containerId: string): Promise<void> {
    if (this.isInitialized) return;

    console.log('MapboxService: Initializing with config:', {
      accessToken: MAPBOX_CONFIG.ACCESS_TOKEN ? 'SET' : 'NOT SET',
      demoMode: MAPBOX_CONFIG.DEMO_MODE,
      containerId
    });

    // Check if container exists
    const container = document.getElementById(containerId);
    if (!container) {
      throw new Error(`Container with id '${containerId}' not found`);
    }

    if (!MAPBOX_CONFIG.ACCESS_TOKEN && !MAPBOX_CONFIG.DEMO_MODE) {
      console.warn('MapboxService: No access token, using demo mode');
      this.initializeDemoMode(containerId);
      return;
    }

    try {
      // Dynamically import mapbox-gl
      console.log('MapboxService: Importing mapbox-gl...');
      const mapboxgl = await import('mapbox-gl');
      console.log('MapboxService: mapbox-gl imported successfully');
      
      // Verify token exists
      if (!MAPBOX_CONFIG.ACCESS_TOKEN) {
        throw new Error('No Mapbox access token found in environment');
      }
      
      console.log('MapboxService: Token length:', MAPBOX_CONFIG.ACCESS_TOKEN.length);
      console.log('MapboxService: Token preview:', MAPBOX_CONFIG.ACCESS_TOKEN.substring(0, 20) + '...');
      
      // Set access token globally BEFORE creating map (required by Mapbox GL JS)
      mapboxgl.default.accessToken = MAPBOX_CONFIG.ACCESS_TOKEN;
      console.log('MapboxService: Access token set on mapboxgl.default.accessToken');
      
      // Ensure container has proper dimensions
      if (container.offsetHeight === 0 || container.offsetWidth === 0) {
        console.warn('MapboxService: Container has zero dimensions, setting defaults');
        container.style.height = '400px';
        container.style.width = '100%';
      }
      
      this.map = new mapboxgl.default.Map({
        container: containerId,
        style: MAPBOX_CONFIG.STYLE_URL,
        center: [-74.006, 40.7128], // New York
        zoom: 10,
        attributionControl: false, // We'll add our own attribution
        accessToken: MAPBOX_CONFIG.ACCESS_TOKEN // Also pass directly to Map constructor
      });

      console.log('MapboxService: Map instance created');

      this.map.on('load', () => {
        console.log('MapboxService: Map loaded successfully');
        this.isInitialized = true;
        this.initializeDefaultLayers();
      });

      this.map.on('error', (e: any) => {
        console.error('MapboxService: Map error event:', e);
        const errorMessage = e.error?.message || 'Unknown map error';
        
        // Check for token-related errors
        if (errorMessage.includes('token') || errorMessage.includes('access')) {
          console.error('MapboxService: Token error detected:', errorMessage);
          console.error('MapboxService: Current token:', MAPBOX_CONFIG.ACCESS_TOKEN ? `${MAPBOX_CONFIG.ACCESS_TOKEN.substring(0, 20)}...` : 'NOT SET');
          throw new Error(`Invalid Mapbox access token: ${errorMessage}. Check VITE_MAPBOX_ACCESS_TOKEN in .env`);
        }
        
        throw new Error(`Mapbox error: ${errorMessage}`);
        
        // Add attribution
        const attribution = document.createElement('div');
        attribution.className = 'mapboxgl-ctrl-attrib';
        attribution.innerHTML = '© <a href="https://www.mapbox.com/">Mapbox</a>';
        attribution.style.cssText = `
          position: absolute;
          bottom: 0;
          left: 0;
          background: rgba(0,0,0,0.7);
          color: white;
          padding: 4px 8px;
          font-size: 10px;
          border-radius: 0 4px 0 0;
          z-index: 1;
        `;
        container.appendChild(attribution);
      });

      this.map.on('error', (error: any) => {
        console.error('Mapbox error:', error);
        // Fallback to demo mode on error
        this.initializeDemoMode(containerId);
      });

    } catch (error) {
      console.error('Failed to initialize Mapbox:', error);
      // Fallback to demo mode
      this.initializeDemoMode(containerId);
    }
  }

  // Demo mode initialization
  private initializeDemoMode(containerId: string): void {
    const container = document.getElementById(containerId);
    if (container) {
      container.innerHTML = `
        <div style="
          width: 100%; 
          height: 100%; 
          background: linear-gradient(45deg, #1a1a1a, #2d2d2d);
          display: flex;
          align-items: center;
          justify-content: center;
          color: white;
          font-family: monospace;
        ">
          <div style="text-align: center;">
            <h3>Mapbox Demo Mode</h3>
            <p>Add VITE_MAPBOX_ACCESS_TOKEN to your .env file</p>
            <p>to enable interactive maps</p>
          </div>
        </div>
      `;
    }
  }

  // Initialize default layers
  private initializeDefaultLayers(): void {
    // People layer (operators, targets, contacts)
    this.addLayer({
      id: 'people',
      type: 'people',
      name: 'People',
      visible: true,
      opacity: 0.9,
      data: [],
      color: '#3b82f6', // Blue
      size: 8,
      icon: 'person'
    });

    // Assets layer (deployed systems, sensors, equipment)
    this.addLayer({
      id: 'assets',
      type: 'assets',
      name: 'Assets',
      visible: true,
      opacity: 0.8,
      data: [],
      color: '#10b981', // Green
      size: 7,
      icon: 'asset'
    });

    // Locations layer (facilities, POIs, safe houses)
    this.addLayer({
      id: 'locations',
      type: 'locations',
      name: 'Locations',
      visible: true,
      opacity: 0.7,
      data: [],
      color: '#8b5cf6', // Purple
      size: 6,
      icon: 'location'
    });

    // Events layer (incidents, activities, alerts)
    this.addLayer({
      id: 'events',
      type: 'events',
      name: 'Events',
      visible: true,
      opacity: 0.6,
      data: [],
      color: '#f59e0b', // Amber
      size: 5,
      icon: 'event'
    });

    // Network layer (connections, communications, relationships)
    this.addLayer({
      id: 'network',
      type: 'network',
      name: 'Network',
      visible: false,
      opacity: 0.5,
      data: [],
      color: '#06b6d4', // Cyan
      size: 2,
      icon: 'network'
    });

    // Threats layer (threat actors, IOCs, malicious activity)
    this.addLayer({
      id: 'threats',
      type: 'threats',
      name: 'Threats',
      visible: true,
      opacity: 0.9,
      data: [],
      color: '#ef4444', // Red
      size: 9,
      icon: 'threat'
    });

    // Infrastructure layer (servers, routers, critical systems)
    this.addLayer({
      id: 'infrastructure',
      type: 'infrastructure',
      name: 'Infrastructure',
      visible: false,
      opacity: 0.7,
      data: [],
      color: '#64748b', // Slate
      size: 6,
      icon: 'infrastructure'
    });
  }

  // Add a new layer
  public addLayer(layer: CTASLayer): void {
    if (!this.map || !this.isInitialized) {
      this.layers.set(layer.id, layer);
      return;
    }

    this.layers.set(layer.id, layer);

    // Add source
    this.map.addSource(layer.id, {
      type: 'geojson',
      data: {
        type: 'FeatureCollection',
        features: this.convertToGeoJSON(layer.data, layer.type)
      }
    });

    // Add layer based on type
    switch (layer.type) {
      case 'people':
      case 'assets':
      case 'locations':
      case 'threats':
      case 'infrastructure':
        this.addCircleLayer(layer);
        break;
      case 'events':
        this.addEventLayer(layer);
        break;
      case 'heatmap':
        this.addHeatmapLayer(layer);
        break;
      case 'network':
        this.addNetworkLayer(layer);
        break;
    }
  }

  // Add circle layer for points
  private addCircleLayer(layer: CTASLayer): void {
    this.map.addLayer({
      id: layer.id,
      type: 'circle',
      source: layer.id,
      paint: {
        'circle-radius': layer.size || 6,
        'circle-color': layer.color || '#ffffff',
        'circle-opacity': layer.opacity,
        'circle-stroke-width': 2,
        'circle-stroke-color': '#000000'
      },
      filter: ['==', ['get', 'visible'], true]
    });

    // Add labels
    this.map.addLayer({
      id: `${layer.id}-labels`,
      type: 'symbol',
      source: layer.id,
      layout: {
        'text-field': ['get', 'name'],
        'text-font': ['Open Sans Regular'],
        'text-size': 12,
        'text-offset': [0, 1.5],
        'text-anchor': 'top'
      },
      paint: {
        'text-color': '#ffffff',
        'text-halo-color': '#000000',
        'text-halo-width': 1
      },
      filter: ['==', ['get', 'visible'], true]
    });
  }

  // Add event layer with animations
  private addEventLayer(layer: CTASLayer): void {
    this.map.addLayer({
      id: layer.id,
      type: 'circle',
      source: layer.id,
      paint: {
        'circle-radius': [
          'interpolate',
          ['linear'],
          ['get', 'timestamp'],
          Date.now() - 24 * 60 * 60 * 1000, layer.size || 4,
          Date.now(), (layer.size || 4) * 2
        ],
        'circle-color': layer.color || '#ffff44',
        'circle-opacity': layer.opacity,
        'circle-stroke-width': 1,
        'circle-stroke-color': '#ffffff'
      }
    });
  }

  // Add heatmap layer
  private addHeatmapLayer(layer: CTASLayer): void {
    this.map.addLayer({
      id: layer.id,
      type: 'heatmap',
      source: layer.id,
      paint: {
        'heatmap-weight': ['get', 'weight'],
        'heatmap-intensity': 1,
        'heatmap-color': [
          'interpolate',
          ['linear'],
          ['heatmap-density'],
          0, 'rgba(0, 0, 255, 0)',
          0.5, 'rgba(0, 255, 0, 0.5)',
          1, 'rgba(255, 0, 0, 0.8)'
        ],
        'heatmap-radius': 30,
        'heatmap-opacity': layer.opacity
      }
    });
  }

  // Add network layer with connections
  private addNetworkLayer(layer: CTASLayer): void {
    // Add connections as lines
    this.map.addLayer({
      id: `${layer.id}-connections`,
      type: 'line',
      source: layer.id,
      paint: {
        'line-color': layer.color || '#ffffff',
        'line-width': 2,
        'line-opacity': layer.opacity
      }
    });

    // Add nodes as circles
    this.map.addLayer({
      id: `${layer.id}-nodes`,
      type: 'circle',
      source: layer.id,
      paint: {
        'circle-radius': layer.size || 6,
        'circle-color': layer.color || '#ffffff',
        'circle-opacity': layer.opacity,
        'circle-stroke-width': 2,
        'circle-stroke-color': '#000000'
      }
    });
  }

  // Update layer data
  public updateLayerData(layerId: string, data: any[]): void {
    const layer = this.layers.get(layerId);
    if (!layer) return;

    layer.data = data;
    this.layers.set(layerId, layer);

    if (this.map && this.isInitialized) {
      const source = this.map.getSource(layerId);
      if (source) {
        source.setData({
          type: 'FeatureCollection',
          features: this.convertToGeoJSON(data, layer.type)
        });
      }
    }
  }

  // Toggle layer visibility
  public toggleLayer(layerId: string): void {
    const layer = this.layers.get(layerId);
    if (!layer) return;

    layer.visible = !layer.visible;
    this.layers.set(layerId, layer);

    if (this.map && this.isInitialized) {
      this.map.setLayoutProperty(layerId, 'visibility', layer.visible ? 'visible' : 'none');
      this.map.setLayoutProperty(`${layerId}-labels`, 'visibility', layer.visible ? 'visible' : 'none');
    }
  }

  // Set layer opacity
  public setLayerOpacity(layerId: string, opacity: number): void {
    const layer = this.layers.get(layerId);
    if (!layer) return;

    layer.opacity = opacity;
    this.layers.set(layerId, layer);

    if (this.map && this.isInitialized) {
      this.map.setPaintProperty(layerId, 'circle-opacity', opacity);
    }
  }

  // Get all layers
  public getLayers(): CTASLayer[] {
    return Array.from(this.layers.values());
  }

  // Convert data to GeoJSON format
  private convertToGeoJSON(data: any[], type: string): any[] {
    return data.map(item => ({
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: item.coordinates || [item.longitude || 0, item.latitude || 0]
      },
      properties: {
        id: item.id,
        name: item.name,
        type: type,
        visible: true,
        ...item.properties
      }
    }));
  }

  // Demo data generators
  private getDemoThreatActors(): any[] {
    return [
      {
        id: 'apt29',
        name: 'APT29',
        coordinates: [-74.006, 40.7128],
        properties: {
          country: 'Russia',
          capabilities: ['Spear Phishing', 'RAT Deployment'],
          threat_level: 'high'
        }
      },
      {
        id: 'lazarus',
        name: 'Lazarus Group',
        coordinates: [127.7669, 35.9078],
        properties: {
          country: 'North Korea',
          capabilities: ['Banking Trojans', 'Supply Chain'],
          threat_level: 'critical'
        }
      },
      {
        id: 'apt41',
        name: 'APT41',
        coordinates: [116.4074, 39.9042],
        properties: {
          country: 'China',
          capabilities: ['Cyber Espionage', 'Ransomware'],
          threat_level: 'high'
        }
      }
    ];
  }

  private getDemoInfrastructure(): any[] {
    return [
      {
        id: 'c2-1',
        name: 'C2 Server 1',
        coordinates: [-118.2437, 34.0522],
        properties: {
          type: 'command-control',
          malware: 'Cobalt Strike',
          status: 'active'
        }
      },
      {
        id: 'c2-2',
        name: 'C2 Server 2',
        coordinates: [-87.6298, 41.8781],
        properties: {
          type: 'command-control',
          malware: 'Metasploit',
          status: 'active'
        }
      },
      {
        id: 'proxy-1',
        name: 'Proxy Server',
        coordinates: [-80.1918, 25.7617],
        properties: {
          type: 'proxy',
          purpose: 'traffic-obfuscation',
          status: 'active'
        }
      }
    ];
  }

  private getDemoTargets(): any[] {
    return [
      {
        id: 'target-1',
        name: 'Financial Institution',
        coordinates: [-74.006, 40.7128],
        properties: {
          sector: 'finance',
          compromised: true,
          ioc_count: 15
        }
      },
      {
        id: 'target-2',
        name: 'Government Agency',
        coordinates: [-77.0369, 38.9072],
        properties: {
          sector: 'government',
          compromised: false,
          ioc_count: 8
        }
      },
      {
        id: 'target-3',
        name: 'Tech Company',
        coordinates: [-122.4194, 37.7749],
        properties: {
          sector: 'technology',
          compromised: true,
          ioc_count: 23
        }
      }
    ];
  }

  private getDemoEvents(): any[] {
    const now = Date.now();
    return [
      {
        id: 'event-1',
        name: 'Malware Detection',
        coordinates: [-74.006, 40.7128],
        properties: {
          type: 'malware',
          severity: 'high',
          timestamp: now - 3600000
        }
      },
      {
        id: 'event-2',
        name: 'Phishing Attempt',
        coordinates: [-118.2437, 34.0522],
        properties: {
          type: 'phishing',
          severity: 'medium',
          timestamp: now - 7200000
        }
      },
      {
        id: 'event-3',
        name: 'Data Exfiltration',
        coordinates: [-87.6298, 41.8781],
        properties: {
          type: 'data-theft',
          severity: 'critical',
          timestamp: now - 10800000
        }
      }
    ];
  }

  // Add demo data usage events as a heatmap
  public addDemoDataHeatmap(events: DemoDataUsageEvent[]): void {
    const heatmapData = events
      .filter(event => event.timestamp)
      .map(event => ({
        id: event.id,
        coordinates: this.getRandomCoordinates(),
        properties: {
          weight: 1,
          timestamp: new Date(event.timestamp).getTime(),
          dataType: event.dataType,
          action: event.action
        }
      }));

    this.addLayer({
      id: 'demo-data-heatmap',
      type: 'heatmap',
      name: 'Demo Data Usage',
      visible: true,
      opacity: 0.7,
      data: heatmapData
    });
  }

  // Generate random coordinates for demo data
  private getRandomCoordinates(): [number, number] {
    return [
      -180 + Math.random() * 360, // longitude
      -90 + Math.random() * 180   // latitude
    ];
  }

  // Resize map
  public resize(): void {
    if (this.map && this.isInitialized) {
      try {
        this.map.resize();
        console.log('MapboxService: Map resized successfully');
      } catch (error) {
        console.error('MapboxService: Error resizing map:', error);
      }
    }
  }

  // Zoom controls
  public zoomIn(): void {
    if (this.map && this.isInitialized) {
      this.map.zoomIn();
    }
  }

  public zoomOut(): void {
    if (this.map && this.isInitialized) {
      this.map.zoomOut();
    }
  }

  public resetView(): void {
    if (this.map && this.isInitialized) {
      this.map.flyTo({
        center: [-98.5795, 39.8283], // Center of US
        zoom: 4,
        duration: 1000
      });
    }
  }

  // Cleanup
  public destroy(): void {
    if (this.map) {
      this.map.remove();
      this.map = null;
    }
    this.layers.clear();
    this.isInitialized = false;
  }
}

// Export singleton instance
export const mapboxService = MapboxService.getInstance();
