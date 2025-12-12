import React, { useState, useEffect } from 'react';
import { AlertTriangle, Shield, Zap, Globe, Database, Activity, Map } from 'lucide-react';
import { InfoStream } from '../types';
import WorkingMap from '@/components/WorkingMap';

const InfoStreams: React.FC = () => {
  const [streams, setStreams] = useState<InfoStream[]>([]);
  const [selectedStream, setSelectedStream] = useState<InfoStream | null>(null);
  const [filter, setFilter] = useState('all');
  const [view, setView] = useState<'list' | 'map'>('list');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Fetch real OSINT streams from TAPS/API (2,309 USIMs)
    const fetchStreams = async () => {
      setIsLoading(true);
      try {
        const response = await fetch('http://localhost:18450/api/streams');
        if (response.ok) {
          const data = await response.json();
          console.log(`✅ InfoStreams: Loaded ${data.streams?.length || 0} real streams`);
          setStreams(data.streams || []);
        } else {
          console.warn('⚠️  InfoStreams: API not available, showing empty state');
          setStreams([]);
        }
      } catch (err) {
        console.error('❌ InfoStreams: Failed to fetch streams:', err);
        setStreams([]);
      } finally {
        setIsLoading(false);
      }
    };

    fetchStreams();
    
    // Refresh every 30 seconds for real-time updates
    const interval = setInterval(fetchStreams, 30000);
    return () => clearInterval(interval);
  }, []);

  const getPriorityIcon = (priority: string) => {
    switch (priority) {
      case 'critical':
        return <AlertTriangle className="w-4 h-4 text-red-500" />;
      case 'high':
        return <Shield className="w-4 h-4 text-orange-500" />;
      case 'medium':
        return <Zap className="w-4 h-4 text-yellow-500" />;
      default:
        return <Activity className="w-4 h-4 text-blue-500" />;
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'critical':
        return 'border-red-200 bg-red-50';
      case 'high':
        return 'border-orange-200 bg-orange-50';
      case 'medium':
        return 'border-yellow-200 bg-yellow-50';
      default:
        return 'border-blue-200 bg-blue-50';
    }
  };

  const filteredStreams = streams.filter(stream => {
    if (filter === 'all') return true;
    return stream.priority === filter;
  });

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Database className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">Information Streams</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-green-100 text-green-800 px-3 py-1 rounded text-sm font-semibold">
                Real-time Intelligence
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Real-time threat intelligence and security information streams from multiple sources.
          </p>

          {/* Filter Controls */}
          <div className="flex gap-4 mb-6">
            <button
              onClick={() => setFilter('all')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'all' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              All ({streams.length})
            </button>
            <button
              onClick={() => setFilter('critical')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'critical' 
                  ? 'bg-red-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Critical ({streams.filter(s => s.priority === 'critical').length})
            </button>
            <button
              onClick={() => setFilter('high')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'high' 
                  ? 'bg-orange-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              High ({streams.filter(s => s.priority === 'high').length})
            </button>
            <button
              onClick={() => setFilter('medium')}
              className={`px-4 py-2 rounded-md font-medium ${
                filter === 'medium' 
                  ? 'bg-yellow-600 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Medium ({streams.filter(s => s.priority === 'medium').length})
            </button>
          </div>

          {/* View Toggle */}
          <div className="flex items-center space-x-2 mb-6">
            <button
              onClick={() => setView('list')}
              className={`px-3 py-1 rounded text-sm flex items-center ${
                view === 'list' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
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
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              <Map size={14} className="mr-2" />
              Map View
            </button>
          </div>
        </div>

        {isLoading ? (
          /* Loading State */
          <div className="flex items-center justify-center h-64 bg-white rounded-lg shadow-xl">
            <div className="text-center">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
              <p className="text-gray-600">Loading intelligence streams...</p>
            </div>
          </div>
        ) : view === 'list' ? (
          /* Streams Grid */
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredStreams.length === 0 ? (
              <div className="col-span-3 text-center py-12 bg-white rounded-lg shadow-xl">
                <Database className="w-16 h-16 mx-auto mb-4 text-gray-400" />
                <h3 className="text-xl font-semibold text-gray-900 mb-2">No Streams Available</h3>
                <p className="text-gray-600">TAPS streaming service is not running. Start the API Gateway to see real-time intelligence.</p>
              </div>
            ) : null}
            {filteredStreams.map((stream) => (
              <div 
                key={stream.id} 
                className={`bg-white rounded-lg shadow-xl p-6 border-l-4 ${getPriorityColor(stream.priority)} cursor-pointer hover:shadow-2xl transition-shadow`}
                onClick={() => setSelectedStream(stream)}
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center gap-2">
                    {getPriorityIcon(stream.priority)}
                    <span className={`px-2 py-1 rounded text-xs font-semibold uppercase ${
                      stream.priority === 'critical' ? 'bg-red-100 text-red-800' :
                      stream.priority === 'high' ? 'bg-orange-100 text-orange-800' :
                      stream.priority === 'medium' ? 'bg-yellow-100 text-yellow-800' :
                      'bg-blue-100 text-blue-800'
                    }`}>
                      {stream.priority}
                    </span>
                  </div>
                  <span className="text-xs text-gray-500">
                    {new Date(stream.timestamp).toLocaleTimeString()}
                  </span>
                </div>
                
                <h3 className="text-lg font-semibold text-gray-900 mb-2">
                  {stream.title}
                </h3>
                
                <p className="text-gray-600 text-sm mb-4 line-clamp-3">
                  {stream.content}
                </p>
                
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Globe className="w-4 h-4 text-gray-400" />
                    <span className="text-sm text-gray-500">{stream.source}</span>
                  </div>
                  
                  <div className="flex gap-1">
                    {stream.tags.slice(0, 2).map((tag) => (
                      <span 
                        key={tag} 
                        className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs"
                      >
                        {tag}
                      </span>
                    ))}
                    {stream.tags.length > 2 && (
                      <span className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs">
                        +{stream.tags.length - 2}
                      </span>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          /* Map View */
          <div className="h-96 bg-white rounded-lg shadow-xl overflow-hidden">
            <WorkingMap />
          </div>
        )}

        {/* Stream Detail Modal */}
        {selectedStream && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    {getPriorityIcon(selectedStream.priority)}
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedStream.title}
                    </h2>
                  </div>
                  <button
                    onClick={() => setSelectedStream(null)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                
                <div className="space-y-4">
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Content</h3>
                    <p className="text-gray-900 mt-1">{selectedStream.content}</p>
                  </div>
                  
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Source</h3>
                      <p className="text-gray-900 mt-1">{selectedStream.source}</p>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Timestamp</h3>
                      <p className="text-gray-900 mt-1">
                        {new Date(selectedStream.timestamp).toLocaleString()}
                      </p>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Tags</h3>
                    <div className="flex flex-wrap gap-2 mt-1">
                      {selectedStream.tags.map((tag) => (
                        <span 
                          key={tag} 
                          className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default InfoStreams;