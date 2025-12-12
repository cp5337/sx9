import React, { useState } from 'react';
import { Search, AlertCircle } from 'lucide-react';
import { searchSimulator } from '@/utils/searchSimulator';

const SearchInterface: React.FC = () => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;
    
    setIsSearching(true);
    try {
      const searchResults = await searchSimulator.search(query);
      setResults(searchResults);
    } catch (error) {
      console.error('Search error:', error);
    } finally {
      setIsSearching(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center space-x-2">
        <div className="relative flex-1">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
            placeholder="Search threat intelligence..."
            className="w-full pl-10 pr-4 py-2 rounded-lg border bg-white dark:bg-gray-800"
          />
          <Search className="absolute left-3 top-2.5 h-5 w-5 text-gray-400" />
        </div>
        <button
          onClick={handleSearch}
          disabled={isSearching}
          className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50"
        >
          {isSearching ? 'Searching...' : 'Search'}
        </button>
      </div>

      <div className="bg-yellow-500/10 text-yellow-500 p-2 rounded flex items-center">
        <AlertCircle className="w-4 h-4 mr-2" />
        <span className="text-sm">Demo Mode - Using simulated search results</span>
      </div>

      <div className="space-y-4">
        {results.map((result) => (
          <div key={result.id} className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
            <h3 className="font-semibold mb-2">{result.title}</h3>
            <p className="text-sm text-gray-600 dark:text-gray-300 mb-2">
              {result.description}
            </p>
            <div className="flex items-center justify-between text-xs text-gray-500">
              <span>Source: {result.source}</span>
              <span>Confidence: {(result.confidence * 100).toFixed(1)}%</span>
              <span>{new Date(result.timestamp).toLocaleString()}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default SearchInterface;