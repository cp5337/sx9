import React, { useState } from 'react';

const SimpleMap: React.FC = () => {
  const [selectedRegion, setSelectedRegion] = useState<string>('');

  const regions = [
    { id: 'north-america', name: 'North America', color: 'bg-blue-600', coords: 'top-1/4 left-1/4 w-1/3 h-1/3' },
    { id: 'europe', name: 'Europe', color: 'bg-green-600', coords: 'top-1/3 right-1/4 w-1/4 h-1/4' },
    { id: 'asia', name: 'Asia', color: 'bg-yellow-600', coords: 'top-1/4 right-1/6 w-1/3 h-1/3' },
    { id: 'africa', name: 'Africa', color: 'bg-orange-600', coords: 'bottom-1/4 left-1/3 w-1/3 h-1/4' },
    { id: 'south-america', name: 'South America', color: 'bg-purple-600', coords: 'bottom-1/6 left-1/4 w-1/4 h-1/3' },
    { id: 'australia', name: 'Australia', color: 'bg-red-600', coords: 'bottom-1/4 right-1/4 w-1/6 h-1/6' }
  ];

  const threatData = [
    { id: 1, name: 'APT29', region: 'europe', type: 'threat-actor', lat: 55.7558, lng: 37.6176, severity: 'high' },
    { id: 2, name: 'Lazarus Group', region: 'asia', type: 'threat-actor', lat: 39.9042, lng: 116.4074, severity: 'high' },
    { id: 3, name: 'C2 Server', region: 'north-america', type: 'infrastructure', lat: 40.7128, lng: -74.0060, severity: 'medium' },
    { id: 4, name: 'Target Network', region: 'europe', type: 'target', lat: 51.5074, lng: -0.1278, severity: 'low' },
    { id: 5, name: 'Malware Distribution', region: 'south-america', type: 'event', lat: -23.5505, lng: -46.6333, severity: 'medium' }
  ];

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'high': return 'bg-red-500';
      case 'medium': return 'bg-yellow-500';
      case 'low': return 'bg-green-500';
      default: return 'bg-gray-500';
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'threat-actor': return '👤';
      case 'infrastructure': return '🏢';
      case 'target': return '🎯';
      case 'event': return '⚠️';
      default: return '📍';
    }
  };

  return (
    <div className="h-full w-full bg-gray-900 rounded-lg overflow-hidden relative">
      {/* World Map Background */}
      <div className="absolute inset-0 bg-gradient-to-br from-blue-900 via-blue-800 to-blue-900">
        {/* Continents */}
        {regions.map((region) => (
          <div
            key={region.id}
            className={`absolute ${region.coords} ${region.color} rounded-full opacity-30 cursor-pointer transition-all duration-200 hover:opacity-50 ${
              selectedRegion === region.id ? 'opacity-70 ring-2 ring-white' : ''
            }`}
            onClick={() => setSelectedRegion(selectedRegion === region.id ? '' : region.id)}
          />
        ))}
        
        {/* Grid Lines */}
        <div className="absolute inset-0 opacity-20">
          {Array.from({ length: 10 }, (_, i) => (
            <div
              key={`h-${i}`}
              className="absolute w-full border-t border-white"
              style={{ top: `${i * 10}%` }}
            />
          ))}
          {Array.from({ length: 20 }, (_, i) => (
            <div
              key={`v-${i}`}
              className="absolute h-full border-l border-white"
              style={{ left: `${i * 5}%` }}
            />
          ))}
        </div>
      </div>

      {/* Threat Data Points */}
      {threatData.map((threat) => (
        <div
          key={threat.id}
          className={`absolute w-4 h-4 ${getSeverityColor(threat.severity)} rounded-full animate-pulse cursor-pointer transition-all duration-200 hover:scale-125`}
          style={{
            top: `${((90 - threat.lat) / 180) * 100}%`,
            left: `${((threat.lng + 180) / 360) * 100}%`,
            transform: 'translate(-50%, -50%)'
          }}
          title={`${threat.name} (${threat.type})`}
        >
          <div className="absolute -top-8 -left-8 bg-gray-800 text-white text-xs px-2 py-1 rounded whitespace-nowrap opacity-0 hover:opacity-100 transition-opacity">
            <div className="flex items-center space-x-1">
              <span>{getTypeIcon(threat.type)}</span>
              <span>{threat.name}</span>
            </div>
            <div className="text-xs opacity-75">{threat.type}</div>
          </div>
        </div>
      ))}

      {/* Legend */}
      <div className="absolute bottom-4 left-4 bg-gray-800 bg-opacity-75 rounded-lg p-3 text-white">
        <div className="text-sm font-semibold mb-2">Legend</div>
        <div className="space-y-1 text-xs">
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-red-500 rounded-full"></div>
            <span>High Severity</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
            <span>Medium Severity</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span>Low Severity</span>
          </div>
        </div>
      </div>

      {/* Controls */}
      <div className="absolute top-4 right-4 bg-gray-800 bg-opacity-75 rounded-lg p-3 text-white">
        <div className="text-sm font-semibold mb-2">Controls</div>
        <div className="space-y-1 text-xs">
          <div>Click regions to highlight</div>
          <div>Hover over points for details</div>
          <div>Points: {threatData.length}</div>
        </div>
      </div>

      {/* Selected Region Info */}
      {selectedRegion && (
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-gray-800 bg-opacity-90 text-white p-4 rounded-lg">
          <h3 className="text-lg font-semibold mb-2">
            {regions.find(r => r.id === selectedRegion)?.name}
          </h3>
          <div className="text-sm">
            <div className="mb-2">
              <strong>Threats:</strong> {threatData.filter(t => t.region === selectedRegion).length}
            </div>
            <div className="space-y-1">
              {threatData
                .filter(t => t.region === selectedRegion)
                .map(threat => (
                  <div key={threat.id} className="flex items-center space-x-2">
                    <span>{getTypeIcon(threat.type)}</span>
                    <span>{threat.name}</span>
                    <span className={`px-1 py-0.5 rounded text-xs ${getSeverityColor(threat.severity)}`}>
                      {threat.severity}
                    </span>
                  </div>
                ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default SimpleMap;
