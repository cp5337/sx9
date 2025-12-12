import React, { useState } from 'react';
import { Search, AlertTriangle, Globe, Database } from 'lucide-react';
import { shodanApi } from '@/utils/shodanApi';
import { demoDataTracker } from '@/utils/demoDataTracker';
import Map, { Marker, Popup } from 'react-map-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

interface ShodanProps {
  selectedSectors: { id: string; name: string; shodanQuery?: string }[];
}

const Shodan: React.FC<ShodanProps> = ({ selectedSectors = [] }) => {
  const [results, setResults] = useState<any[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedResult, setSelectedResult] = useState<any | null>(null);
  const apiKey = import.meta.env.VITE_SHODAN_API_KEY;

  const handleSearch = async (sector: { name: string; shodanQuery?: string }) => {
    if (!sector.shodanQuery) return;
    
    setIsSearching(true);
    setError(null);
    
    const startTime = Date.now();
    try {
      const searchResults = await shodanApi.search({ query: sector.shodanQuery });
      setResults(prev => [...prev, ...searchResults]);
      
      // Track successful search
      demoDataTracker.trackSearch(
        'shodan',
        sector.shodanQuery,
        { sector: sector.name },
        searchResults.length,
        Date.now() - startTime
      );
    } catch (err) {
      setError('Failed to fetch results from Shodan');
      console.error('Shodan search error:', err);
      
      // Track failed search
      demoDataTracker.trackError('shodan', 'search', 'Failed to fetch results from Shodan', sector.shodanQuery);
    } finally {
      setIsSearching(false);
    }
  };

  const handleSearchAll = async () => {
    setResults([]);
    for (const sector of selectedSectors) {
      await handleSearch(sector);
    }
  };

  if (!apiKey) {
    return (
      <div className="p-4 bg-yellow-500/10 text-yellow-500 rounded-lg flex items-center">
        <AlertTriangle size={16} className="mr-2" />
        <span>Demo Mode - Using simulated Shodan results</span>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col bg-gray-100 dark:bg-gray-900">
      <div className="p-4">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-lg font-semibold flex items-center">
            <Globe className="mr-2" />
            Shodan Intelligence
          </h1>
          <button
            onClick={handleSearchAll}
            disabled={isSearching || selectedSectors.length === 0}
            className="bg-blue-500 text-white px-4 py-2 rounded flex items-center disabled:opacity-50"
          >
            {isSearching ? (
              'Searching...'
            ) : (
              <>
                <Search size={16} className="mr-2" />
                Search Selected Sectors
              </>
            )}
          </button>
        </div>

        {error && (
          <div className="bg-red-500/10 text-red-500 p-2 rounded flex items-center mb-4">
            <AlertTriangle size={16} className="mr-2" />
            <span>{error}</span>
          </div>
        )}
      </div>

      <div className="flex-1 relative">
        <Map
          initialViewState={{
            latitude: 20,
            longitude: 0,
            zoom: 1.5
          }}
          style={{ width: '100%', height: '100%' }}
          mapStyle="mapbox://styles/mapbox/dark-v11"
          mapboxAccessToken={apiKey}
        >
          {results.map((result, index) => (
            <Marker
              key={index}
              longitude={result.location.longitude}
              latitude={result.location.latitude}
              onClick={e => {
                e.originalEvent.stopPropagation();
                setSelectedResult(result);
              }}
            >
              <div className="w-4 h-4 bg-red-500 rounded-full border-2 border-white cursor-pointer" />
            </Marker>
          ))}

          {selectedResult && (
            <Popup
              longitude={selectedResult.location.longitude}
              latitude={selectedResult.location.latitude}
              anchor="bottom"
              onClose={() => setSelectedResult(null)}
            >
              <div className="p-2 max-w-xs">
                <h3 className="font-semibold text-sm flex items-center mb-2">
                  <Database size={12} className="mr-1" />
                  {selectedResult.ip_str}:{selectedResult.port}
                </h3>
                <p className="text-xs text-gray-600 mb-1">
                  {selectedResult.location.city_name}, {selectedResult.location.country_name}
                </p>
                {selectedResult.vulns?.length > 0 && (
                  <div className="mt-2">
                    <h4 className="text-xs font-semibold mb-1">Vulnerabilities:</h4>
                    <div className="flex flex-wrap gap-1">
                      {selectedResult.vulns.map((vuln: string) => (
                        <span key={vuln} className="px-1 py-0.5 text-xxs bg-red-100 text-red-800 rounded">
                          {vuln}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </Popup>
          )}
        </Map>
      </div>
    </div>
  );
};

export default Shodan;