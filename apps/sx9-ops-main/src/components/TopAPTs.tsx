import React from 'react';
import { Marker, Popup } from 'react-map-gl';
import { Target, AlertTriangle } from 'lucide-react';

interface APT {
  id: string;
  name: string;
  location: {
    lat: number;
    lon: number;
  };
  description: string;
  knownFor: string;
  targets: {
    name: string;
    location: [number, number];
    sector: string;
  }[];
}

const topAPTs: APT[] = [
  {
    id: 'apt1',
    name: 'APT28 (Fancy Bear)',
    location: { lat: 55.7558, lon: 37.6173 },
    description: 'Russian military intelligence agency GRU.',
    knownFor: "Targeting NATO, European governments, and U.S. political entities",
    targets: [
      { name: "Washington D.C., USA", location: [-77.0369, 38.9072], sector: "Government" },
      { name: "Brussels, Belgium", location: [4.3517, 50.8503], sector: "Government (NATO)" },
      { name: "Berlin, Germany", location: [13.4050, 52.5200], sector: "Government" }
    ]
  },
  {
    id: 'apt2',
    name: 'APT29 (Cozy Bear)',
    location: { lat: 55.7558, lon: 37.6173 },
    description: 'Russian intelligence agency SVR.',
    knownFor: "Targeting governments, think tanks, and healthcare organizations",
    targets: [
      { name: "Washington D.C., USA", location: [-77.0369, 38.9072], sector: "Government" },
      { name: "Boston, Massachusetts, USA", location: [-71.0589, 42.3601], sector: "Healthcare" },
      { name: "London, UK", location: [-0.1276, 51.5074], sector: "Government and Think Tanks" }
    ]
  },
  {
    id: 'apt3',
    name: 'APT1',
    location: { lat: 31.2304, lon: 121.4737 },
    description: 'Chinese PLA Unit 61398 (People\'s Liberation Army).',
    knownFor: "Targeting U.S. defense contractors and industrial sectors",
    targets: [
      { name: "San Francisco, California, USA", location: [-122.4194, 37.7749], sector: "Technology" },
      { name: "Houston, Texas, USA", location: [-95.3698, 29.7604], sector: "Energy" },
      { name: "New York City, USA", location: [-74.0060, 40.7128], sector: "Financial Institutions" }
    ]
  },
  {
    id: 'apt5',
    name: 'Lazarus Group',
    location: { lat: 39.0392, lon: 125.7625 },
    description: 'North Korean government-sponsored group.',
    knownFor: "Targeting financial institutions and media companies",
    targets: [
      { name: "New York City, USA", location: [-74.0060, 40.7128], sector: "Financial Institutions" },
      { name: "Seoul, South Korea", location: [126.9780, 37.5665], sector: "Media and Technology" },
      { name: "Zurich, Switzerland", location: [8.5417, 47.3769], sector: "Financial Institutions" }
    ]
  },
  {
    id: 'apt6',
    name: 'Charming Kitten (APT35)',
    location: { lat: 35.6892, lon: 51.3890 },
    description: 'Iranian cyber espionage group linked to the IRGC.',
    knownFor: "Targeting journalists, academia, and government entities",
    targets: [
      { name: "Oxford, UK", location: [-1.2577, 51.7520], sector: "Healthcare and Academia" },
      { name: "Tehran, Iran", location: [51.3890, 35.6892], sector: "Government" },
      { name: "London, UK", location: [-0.1276, 51.5074], sector: "Government and Academia" }
    ]
  },
];

const TopAPTs: React.FC = () => {
  return (
    <>
      {topAPTs.map((apt) => (
        <React.Fragment key={apt.id}>
          <Marker longitude={apt.location.lon} latitude={apt.location.lat} anchor="bottom">
            <div className="relative group">
              <AlertTriangle size={16} className="text-red-500" />
              <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 bg-black text-white p-1 rounded text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                <strong>{apt.name}</strong>: {apt.description}
              </div>
            </div>
          </Marker>
          {apt.targets.map((target, index) => (
            <Marker key={`${apt.id}-target-${index}`} longitude={target.location[0]} latitude={target.location[1]} anchor="bottom">
              <div className="relative group">
                <Target size={12} className="text-yellow-500" />
                <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 bg-black text-white p-1 rounded text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <strong>{target.name}</strong>: {target.sector}
                </div>
              </div>
            </Marker>
          ))}
        </React.Fragment>
      ))}
    </>
  );
};

export default TopAPTs;