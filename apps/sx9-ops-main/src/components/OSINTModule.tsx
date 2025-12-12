import React, { useState } from 'react';
import { Search, Globe, Twitter, Linkedin, Github, Mail, Phone, MapPin } from 'lucide-react';
import { OSINTResult } from '../types';
import { getDemoData } from '../utils/demoDataProvider';

const OSINTModule: React.FC = () => {
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<OSINTResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [selectedPlatform, setSelectedPlatform] = useState('all');

  const platforms = [
    { id: 'all', name: 'All Platforms', icon: <Globe className="w-4 h-4" /> },
    { id: 'twitter', name: 'Twitter', icon: <Twitter className="w-4 h-4" /> },
    { id: 'linkedin', name: 'LinkedIn', icon: <Linkedin className="w-4 h-4" /> },
    { id: 'github', name: 'GitHub', icon: <Github className="w-4 h-4" /> },
    { id: 'email', name: 'Email', icon: <Mail className="w-4 h-4" /> },
    { id: 'phone', name: 'Phone', icon: <Phone className="w-4 h-4" /> },
    { id: 'location', name: 'Location', icon: <MapPin className="w-4 h-4" /> }
  ];

  const performOSINTSearch = async (query: string, platform: string) => {
    setIsSearching(true);
    
    try {
      // Production OSINT search implementation
      // This would integrate with real OSINT APIs and services
      const results = await searchOSINTServices(query, platform);
      setSearchResults(results);
    } catch (error) {
      // Fallback to demo data if production fails
      if (import.meta.env.VITE_DEMO_MODE === 'true') {
        const demoResults = getDemoData<OSINTResult[]>('osintResults', []);
        setSearchResults(demoResults.filter(result => 
          platform === 'all' || result.platform === platform
        ));
      } else {
        console.error('OSINT search failed:', error);
        setSearchResults([]);
      }
    } finally {
      setIsSearching(false);
    }
  };

  const searchOSINTServices = async (query: string, platform: string): Promise<OSINTResult[]> => {
    // Production implementation would call real OSINT APIs
    // For now, this throws an error to trigger demo fallback
    throw new Error('OSINT services not configured');
  };

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      performOSINTSearch(searchQuery, selectedPlatform);
    }
  };

  const getPlatformIcon = (platform: string) => {
    switch (platform) {
      case 'twitter':
        return <Twitter className="w-4 h-4 text-blue-400" />;
      case 'linkedin':
        return <Linkedin className="w-4 h-4 text-blue-600" />;
      case 'github':
        return <Github className="w-4 h-4 text-gray-800" />;
      case 'email':
        return <Mail className="w-4 h-4 text-red-500" />;
      case 'phone':
        return <Phone className="w-4 h-4 text-green-500" />;
      case 'location':
        return <MapPin className="w-4 h-4 text-purple-500" />;
      default:
        return <Globe className="w-4 h-4 text-gray-500" />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Search className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">OSINT Intelligence</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-orange-100 text-orange-800 px-3 py-1 rounded text-sm font-semibold">
                Open Source Intelligence
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Comprehensive open-source intelligence gathering across multiple platforms and sources.
          </p>

          {/* Search Form */}
          <form onSubmit={handleSearch} className="space-y-4">
            <div className="flex gap-4">
              <div className="flex-1">
                <input
                  type="text"
                  placeholder="Enter search query"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              <button
                type="submit"
                disabled={isSearching || !searchQuery.trim()}
                className="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
              >
                {isSearching ? 'Searching...' : 'Search'}
              </button>
            </div>

            {/* Platform Selection */}
            <div className="flex gap-2 flex-wrap">
              {platforms.map((platform) => (
                <button
                  key={platform.id}
                  type="button"
                  onClick={() => setSelectedPlatform(platform.id)}
                  className={`flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                    selectedPlatform === platform.id
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                  }`}
                >
                  {platform.icon}
                  {platform.name}
                </button>
              ))}
            </div>
          </form>
        </div>

        {/* Search Results */}
        {searchResults.length > 0 && (
          <div className="bg-white rounded-lg shadow-xl p-6">
            <h2 className="text-2xl font-semibold text-gray-800 mb-6">
              Search Results ({searchResults.length})
            </h2>
            
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {searchResults.map((result) => (
                <div key={result.id} className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-2">
                      {getPlatformIcon(result.platform)}
                      <span className="text-sm font-medium text-gray-600 capitalize">
                        {result.platform}
                      </span>
                    </div>
                    {result.verified && (
                      <span className="px-2 py-1 bg-green-100 text-green-800 rounded text-xs font-semibold">
                        Verified
                      </span>
                    )}
                  </div>
                  
                  <h3 className="font-semibold text-gray-900 mb-2">
                    {result.username}
                  </h3>
                  
                  <div className="space-y-2 text-sm text-gray-600">
                    <div className="flex items-center gap-2">
                      <Globe className="w-4 h-4" />
                      <a href={result.url} target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:underline">
                        View Profile
                      </a>
                    </div>
                    
                    {result.followers > 0 && (
                      <div>
                        <span className="font-medium">Followers:</span> {result.followers.toLocaleString()}
                      </div>
                    )}
                    
                    <div>
                      <span className="font-medium">Last Active:</span> {new Date(result.lastActive).toLocaleDateString()}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* No Results */}
        {searchQuery && !isSearching && searchResults.length === 0 && (
          <div className="bg-white rounded-lg shadow-xl p-6 text-center">
            <Search className="w-12 h-12 mx-auto mb-4 text-gray-400" />
            <h3 className="text-lg font-semibold text-gray-900 mb-2">No Results Found</h3>
            <p className="text-gray-600">
              No OSINT results found for "{searchQuery}" on {selectedPlatform === 'all' ? 'all platforms' : selectedPlatform}.
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default OSINTModule;