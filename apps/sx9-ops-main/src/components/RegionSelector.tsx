import React from 'react';
import { Globe } from 'lucide-react';


interface Region {
  id: string;
  name: string;
  selected: boolean;
  bounds: [[number, number], [number, number]]; // [[SW], [NE]]
}

interface RegionSelectorProps {
  regions: Region[];
  onRegionChange: (regionId: string) => void;
}

const RegionSelector: React.FC<RegionSelectorProps> = ({ regions, onRegionChange }) => {
  return (
    <div className="flex items-center space-x-2 overflow-x-auto py-2">
      {regions.map(region => (
        <button
          key={region.id}
          onClick={() => onRegionChange(region.id)}
          className={`flex items-center px-2 py-1 rounded-full text-xs whitespace-nowrap ${
            region.selected 
              ? 'bg-green-500 text-white' 
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
          }`}
        >
          <Globe className="w-3 h-3 mr-1" />
          {region.name}
        </button>
      ))}
    </div>
  );
};

export default RegionSelector;