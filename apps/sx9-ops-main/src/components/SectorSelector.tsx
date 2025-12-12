import React from 'react';
import { Info, Shield } from 'lucide-react';

interface Sector {
  id: string;
  name: string;
  selected: boolean;
  description: string;
  shodanQuery?: string;
}

interface SectorSelectorProps {
  sectors: Sector[];
  onSectorToggle: (sectorId: string) => void;
}

const SectorSelector: React.FC<SectorSelectorProps> = ({ sectors, onSectorToggle }) => {
  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
      {sectors.map(sector => (
        <button
          key={sector.id}
          onClick={() => onSectorToggle(sector.id)}
          className={`
            p-2 rounded-lg text-left transition-colors duration-200
            ${sector.selected 
              ? 'bg-blue-500 text-white' 
              : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }
            ${sector.id === 'none' ? 'col-span-full' : ''}
          `}
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <Shield className="w-4 h-4 mr-2" />
              <span className="text-sm font-medium">{sector.name}</span>
            </div>
            <div className="group relative">
              <Info className="w-4 h-4 text-gray-400" />
              <div className="absolute right-0 top-6 w-64 p-2 bg-gray-800 rounded shadow-lg 
                            text-xs text-gray-300 hidden group-hover:block z-10">
                {sector.description}
                {sector.shodanQuery && (
                  <div className="mt-1 text-gray-400">
                    Query: {sector.shodanQuery}
                  </div>
                )}
              </div>
            </div>
          </div>
        </button>
      ))}
    </div>
  );
};

export default SectorSelector;