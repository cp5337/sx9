import React, { useState, useEffect } from 'react';
import { AlertTriangle, Shield, Zap, Globe, Database, Activity } from 'lucide-react';
import { InfoStream } from '../types';
import { getDemoData } from '../utils/demoDataProvider';

const AnimatedInfoStreams: React.FC = () => {
  const [streams, setStreams] = useState<InfoStream[]>([]);
  const [currentStreamIndex, setCurrentStreamIndex] = useState(0);

  useEffect(() => {
    // Use centralized demo data provider
    const demoStreams = getDemoData<InfoStream[]>('infoStreams', []);
    setStreams(demoStreams);
  }, []);

  useEffect(() => {
    if (streams.length === 0) return;

    const interval = setInterval(() => {
      setCurrentStreamIndex((prev) => (prev + 1) % streams.length);
    }, 3000);

    return () => clearInterval(interval);
  }, [streams.length]);

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

  if (streams.length === 0) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <Database className="w-8 h-8 mx-auto mb-2 text-gray-400" />
          <p className="text-gray-500">Loading information streams...</p>
        </div>
      </div>
    );
  }

  const currentStream = streams[currentStreamIndex];
  
  if (!currentStream) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <Database className="w-8 h-8 mx-auto mb-2 text-gray-400" />
          <p className="text-gray-500">No streams available</p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-xl p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <Database className="w-6 h-6 text-blue-600" />
          <h2 className="text-xl font-bold text-gray-900">Live Information Streams</h2>
        </div>
        <div className="flex items-center gap-2">
          <div className="flex space-x-1">
            {streams.map((_, index) => (
              <div
                key={index}
                className={`w-2 h-2 rounded-full transition-colors ${
                  index === currentStreamIndex ? 'bg-blue-600' : 'bg-gray-300'
                }`}
              />
            ))}
          </div>
          <span className="text-sm text-gray-500">
            {currentStreamIndex + 1} of {streams.length}
          </span>
        </div>
      </div>

      <div className={`border-l-4 p-4 rounded-r-lg transition-all duration-500 ${getPriorityColor(currentStream.priority)}`}>
        <div className="flex items-start justify-between mb-3">
          <div className="flex items-center gap-2">
            {getPriorityIcon(currentStream.priority)}
            <span className={`px-2 py-1 rounded text-xs font-semibold uppercase ${
              currentStream.priority === 'critical' ? 'bg-red-100 text-red-800' :
              currentStream.priority === 'high' ? 'bg-orange-100 text-orange-800' :
              currentStream.priority === 'medium' ? 'bg-yellow-100 text-yellow-800' :
              'bg-blue-100 text-blue-800'
            }`}>
              {currentStream.priority}
            </span>
          </div>
          <span className="text-xs text-gray-500">
            {new Date(currentStream.timestamp).toLocaleTimeString()}
          </span>
        </div>
        
        <h3 className="text-lg font-semibold text-gray-900 mb-2">
          {currentStream.title}
        </h3>
        
        <p className="text-gray-600 text-sm mb-3">
          {currentStream.content}
        </p>
        
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Globe className="w-4 h-4 text-gray-400" />
            <span className="text-sm text-gray-500">{currentStream.source}</span>
          </div>
          
          <div className="flex gap-1">
            {currentStream.tags.slice(0, 2).map((tag) => (
              <span 
                key={tag} 
                className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs"
              >
                {tag}
              </span>
            ))}
            {currentStream.tags.length > 2 && (
              <span className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs">
                +{currentStream.tags.length - 2}
              </span>
            )}
          </div>
        </div>
      </div>

      <div className="mt-4 flex justify-between text-xs text-gray-500">
        <span>Auto-refresh every 3 seconds</span>
        <span>Last updated: {new Date().toLocaleTimeString()}</span>
      </div>
    </div>
  );
};

export default AnimatedInfoStreams;