import React, { useState } from 'react';
import { Search, AlertTriangle, Shield } from 'lucide-react';
import SectorSelector from './SectorSelector';
import Shodan from './Shodan';

interface Sector {
  id: string;
  name: string;
  selected: boolean;
  description: string;
  shodanQuery?: string;
}

const CriticalInfrastructureDashboard: React.FC = () => {
  const [sectors, setSectors] = useState<Sector[]>([
    { id: 'none', name: 'Not Specific', selected: false, description: 'General search without sector-specific filters' },
    { id: 'chemical', name: 'Chemical', selected: false, description: 'Chemical manufacturing and storage facilities', shodanQuery: 'org:"chemical" product:"SCADA" country:US' },
    { id: 'commercial', name: 'Commercial Facilities', selected: false, description: 'Public venues and shopping centers', shodanQuery: 'org:"mall" OR org:"arena" OR org:"stadium" country:US' },
    { id: 'communications', name: 'Communications', selected: false, description: 'Telecommunications and broadcasting', shodanQuery: 'org:"telecom" OR org:"isp" OR port:5060 country:US' },
    { id: 'manufacturing', name: 'Critical Manufacturing', selected: false, description: 'Primary metals and machinery', shodanQuery: 'port:502 org:"manufacturing" country:US' },
    { id: 'dams', name: 'Dams', selected: false, description: 'Dam infrastructure and hydropower', shodanQuery: 'org:"water" product:"SCADA" dam country:US' },
    { id: 'defense', name: 'Defense Industrial Base', selected: false, description: 'Defense and aerospace industry', shodanQuery: 'org:"defense" OR org:"aerospace" country:US' },
    { id: 'emergency', name: 'Emergency Services', selected: false, description: 'Law enforcement and emergency response', shodanQuery: 'org:"emergency" OR org:"police" OR org:"fire department" country:US' },
    { id: 'energy', name: 'Energy', selected: false, description: 'Electricity, oil, and natural gas', shodanQuery: 'port:502 product:"Siemens" OR product:"Schneider" country:US' },
    { id: 'financial', name: 'Financial Services', selected: false, description: 'Banking and financial institutions', shodanQuery: 'ssl:"bank" org:"financial" country:US' },
    { id: 'food-agriculture', name: 'Food and Agriculture', selected: false, description: 'Food production and agriculture', shodanQuery: 'org:"agriculture" OR org:"farm" product:"SCADA" country:US' },
    { id: 'government', name: 'Government Facilities', selected: false, description: 'Government buildings and facilities', shodanQuery: 'org:".gov" country:US' },
    { id: 'healthcare', name: 'Healthcare and Public Health', selected: false, description: 'Healthcare systems and services', shodanQuery: 'org:"hospital" OR org:"healthcare" ssl:medical country:US' },
    { id: 'it', name: 'Information Technology', selected: false, description: 'IT infrastructure and services', shodanQuery: 'org:"datacenter" port:443 country:US' },
    { id: 'nuclear', name: 'Nuclear Reactors and Materials', selected: false, description: 'Nuclear power and materials', shodanQuery: 'org:"nuclear" product:"SCADA" country:US' },
    { id: 'transportation', name: 'Transportation Systems', selected: false, description: 'Air, rail, and road systems', shodanQuery: 'org:"transit" OR org:"transportation" OR org:"railway" country:US' },
    { id: 'water', name: 'Water and Wastewater', selected: false, description: 'Water infrastructure systems', shodanQuery: 'org:"water" product:"SCADA" NOT dam country:US' }
  ]);

  const handleSectorToggle = (sectorId: string) => {
    setSectors(prevSectors => 
      prevSectors.map(sector => {
        if (sectorId === 'none') {
          // If "Not Specific" is selected, deselect all others
          return {
            ...sector,
            selected: sector.id === 'none' ? !sector.selected : false
          };
        } else {
          // If any other sector is selected, deselect "Not Specific"
          if (sector.id === sectorId) {
            return { ...sector, selected: !sector.selected };
          } else if (sector.id === 'none') {
            return { ...sector, selected: false };
          }
          return sector;
        }
      })
    );
  };

  const selectedSectors = sectors.filter(sector => sector.selected && sector.id !== 'none');

  return (
    <div className="h-full flex flex-col bg-gray-100 dark:bg-gray-900">
      <div className="p-4 bg-gray-800">
        <h1 className="text-xl font-bold text-white flex items-center mb-4">
          <Shield className="mr-2" />
          Critical Infrastructure Sectors
        </h1>
        
        <div className="mb-4">
          <SectorSelector sectors={sectors} onSectorToggle={handleSectorToggle} />
        </div>

        {sectors.some(s => s.selected) ? (
          <div className="flex items-center text-green-500 text-sm">
            <Search className="w-4 h-4 mr-2" />
            {sectors.find(s => s.id === 'none')?.selected 
              ? 'Performing general infrastructure search'
              : `Searching ${selectedSectors.map(s => s.name).join(', ')} sectors`
            }
          </div>
        ) : (
          <div className="flex items-center text-yellow-500 text-sm">
            <AlertTriangle className="w-4 h-4 mr-2" />
            Please select at least one sector to begin searching
          </div>
        )}
      </div>

      <div className="flex-1">
        <Shodan selectedSectors={selectedSectors} />
      </div>
    </div>
  );
};

export default CriticalInfrastructureDashboard;