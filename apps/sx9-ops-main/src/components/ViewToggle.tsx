import React from 'react';
import { Map, Grid, Network } from 'lucide-react';

interface ViewToggleProps {
  view: 'map' | 'grid' | 'graph';
  setView: (view: 'map' | 'grid' | 'graph') => void;
}

const ViewToggle: React.FC<ViewToggleProps> = ({ view, setView }) => {
  return (
    <div className="flex space-x-2 mb-4">
      <button
        onClick={() => setView('map')}
        className={`p-2 rounded ${
          view === 'map' ? 'bg-blue-500 text-white' : 'bg-gray-200 dark:bg-gray-700'
        }`}
      >
        <Map size={20} />
      </button>
      <button
        onClick={() => setView('grid')}
        className={`p-2 rounded ${
          view === 'grid' ? 'bg-blue-500 text-white' : 'bg-gray-200 dark:bg-gray-700'
        }`}
      >
        <Grid size={20} />
      </button>
      <button
        onClick={() => setView('graph')}
        className={`p-2 rounded ${
          view === 'graph' ? 'bg-blue-500 text-white' : 'bg-gray-200 dark:bg-gray-700'
        }`}
      >
        <Network size={20} />
      </button>
    </div>
  );
};

export default ViewToggle;