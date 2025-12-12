import React, { useState, useEffect } from 'react';
import { Search, MapPin, Shield, AlertTriangle, Database, Network, Zap, Globe, Activity, CheckCircle, Clock, Map } from 'lucide-react';
import { shodanApi } from '@/utils/shodanApi';
import EnhancedMap from '@/components/EnhancedMap';

// Add the missing function
const simulateShodanSearch = async (sector: string, apiKey: string) => {
  // Simulate API call delay
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  return [
    {
      ip: '192.168.1.100',
      port: 80,
      hostname: 'example.com',
      location: { country: 'US', city: 'New York' },
      vulns: ['CVE-2021-1234', 'CVE-2021-5678'],
      tags: ['web', 'http', 'nginx']
    }
  ];
};

interface ShodanProps {
  selectedSectors?: string[];
}

const Shodan: React.FC<ShodanProps> = ({ selectedSectors = [] }) => {
  const [results, setResults] = useState<any[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [apiKey, setApiKey] = useState(import.meta.env.VITE_SHODAN_API_KEY || '');
  const [view, setView] = useState<'list' | 'map'>('list');

  const handleSearch = async (sector: string) => {
    setIsSearching(true);
    setError(null);
    
    try {
      const searchResults = await simulateShodanSearch(sector, apiKey);
      setResults(prev => [...prev, ...searchResults]);
    } catch (err) {
      setError('Failed to fetch results from Shodan');
      console.error('Shodan search error:', err);
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

  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-lg font-semibold flex items-center">
            <Globe className="mr-2" size={20} />
            Shodan Intelligence
          </h1>
          <div className="flex items-center space-x-2">
            <input
              type="password"
              placeholder="Shodan API Key"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              className="px-3 py-1 text-sm rounded border bg-white dark:bg-gray-800"
            />
            <button
              onClick={handleSearchAll}
              disabled={isSearching || selectedSectors.length === 0}
              className="bg-blue-500 text-white px-4 py-1 rounded text-sm flex items-center disabled:opacity-50"
            >
              {isSearching ? (
                <>Searching...</>
              ) : (
                <>
                  <Search size={14} className="mr-2" />
                  Search All Sectors
                </>
              )}
            </button>
          </div>
        </div>

        {/* View Toggle */}
        <div className="flex items-center space-x-2 mb-4">
          <button
            onClick={() => setView('list')}
            className={`px-3 py-1 rounded text-sm flex items-center ${
              view === 'list' 
                ? 'bg-blue-500 text-white' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
            }`}
          >
            <Database size={14} className="mr-2" />
            List View
          </button>
          <button
            onClick={() => setView('map')}
            className={`px-3 py-1 rounded text-sm flex items-center ${
              view === 'map' 
                ? 'bg-blue-500 text-white' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
            }`}
          >
            <Map size={14} className="mr-2" />
            Map View
          </button>
        </div>

        {!apiKey && (
          <div className="bg-yellow-500/10 text-yellow-500 p-2 rounded flex items-center mb-4">
            <AlertTriangle size={16} className="mr-2" />
            <span className="text-sm">Demo Mode - Using simulated Shodan results</span>
          </div>
        )}

        {error && (
          <div className="bg-red-500/10 text-red-500 p-2 rounded flex items-center mb-4">
            <AlertTriangle size={16} className="mr-2" />
            <span className="text-sm">{error}</span>
          </div>
        )}

        {view === 'list' ? (
          <>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
              {selectedSectors.map(sector => (
                <div key={sector} className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
                  <div className="flex justify-between items-center mb-2">
                    <h2 className="font-semibold flex items-center">
                      <Shield size={16} className="mr-2" />
                      {sector}
                    </h2>
                    <button
                      onClick={() => handleSearch(sector)}
                      disabled={isSearching}
                      className="text-blue-500 hover:text-blue-600"
                    >
                      <Search size={16} />
                    </button>
                  </div>
                  <div className="text-sm text-gray-500">
                    Results: {results.filter(r => r.tags.includes(sector.toLowerCase())).length}
                  </div>
                </div>
              ))}
            </div>

            <div className="space-y-4">
              {results.map((result) => (
                <div key={result.id} className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
                  <div className="flex justify-between items-center mb-2">
                    <h3 className="font-semibold flex items-center">
                      <Database size={16} className="mr-2" />
                      {result.org}
                    </h3>
                    <span className="text-sm text-gray-500">
                      {result.location.city}, {result.location.country}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 gap-4 mb-2 text-sm">
                    <div>
                      <span className="font-medium">IP:</span> {result.ip}
                    </div>
                    <div>
                      <span className="font-medium">Port:</span> {result.port}
                    </div>
                  </div>
                  {result.vulns.length > 0 && (
                    <div className="mt-2">
                      <h4 className="text-sm font-medium mb-1">Vulnerabilities:</h4>
                      <div className="flex flex-wrap gap-2">
                        {result.vulns.map((vuln: string) => (
                          <span
                            key={vuln}
                            className="px-2 py-1 text-xs bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded"
                          >
                            {vuln}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                  <div className="mt-2 flex flex-wrap gap-2">
                    {result.tags.map((tag: string) => (
                      <span
                        key={tag}
                        className="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 rounded"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </>
        ) : (
          <div className="h-96 bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
            <EnhancedMap 
              showLayerControls={true}
              showDemoData={true}
              className="h-full w-full"
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default Shodan;