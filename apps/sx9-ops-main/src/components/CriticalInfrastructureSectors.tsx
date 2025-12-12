import React from 'react';

interface Sector {
  id: string;
  name: string;
  value: number;
}

interface CriticalInfrastructureSectorsProps {
  sectors: Sector[];
  onSectorChange: (sectorId: string, value: number) => void;
}

const CriticalInfrastructureSectors: React.FC<CriticalInfrastructureSectorsProps> = ({
  sectors,
  onSectorChange,
}) => {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
      {sectors.map((sector) => (
        <div key={sector.id} className="flex flex-col">
          <label htmlFor={sector.id} className="text-sm mb-1">{sector.name}</label>
          <div className="flex items-center">
            <input
              type="range"
              id={sector.id}
              min="0"
              max="100"
              value={sector.value}
              onChange={(e) => onSectorChange(sector.id, parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <span className="ml-2 text-sm w-8">{sector.value}%</span>
          </div>
        </div>
      ))}
    </div>
  );
};

export default CriticalInfrastructureSectors;