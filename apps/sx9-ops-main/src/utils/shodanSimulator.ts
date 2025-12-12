import { v4 as uuidv4 } from 'uuid';

interface ShodanResult {
  id: string;
  ip: string;
  port: number;
  org: string;
  location: {
    country: string;
    city: string;
    coordinates: [number, number];
  };
  vulns: string[];
  lastUpdate: string;
  tags: string[];
}

// Simulated Shodan data for critical infrastructure sectors
const demoResults: Record<string, ShodanResult[]> = {
  energy: [
    {
      id: uuidv4(),
      ip: '192.168.1.1',
      port: 502,
      org: 'Texas Power Grid',
      location: {
        country: 'United States',
        city: 'Houston',
        coordinates: [29.7604, -95.3698]
      },
      vulns: ['CVE-2021-44228', 'CVE-2022-22965'],
      lastUpdate: new Date().toISOString(),
      tags: ['scada', 'modbus', 'energy']
    }
  ],
  financial: [
    {
      id: uuidv4(),
      ip: '10.0.0.1',
      port: 443,
      org: 'Major Financial Institution',
      location: {
        country: 'United States',
        city: 'New York',
        coordinates: [40.7128, -74.0060]
      },
      vulns: ['CVE-2021-3449', 'CVE-2021-3450'],
      lastUpdate: new Date().toISOString(),
      tags: ['banking', 'ssl', 'https']
    }
  ],
  healthcare: [
    {
      id: uuidv4(),
      ip: '172.16.0.1',
      port: 8080,
      org: 'Regional Medical Center',
      location: {
        country: 'United States',
        city: 'Boston',
        coordinates: [42.3601, -71.0589]
      },
      vulns: ['CVE-2021-21972', 'CVE-2021-21973'],
      lastUpdate: new Date().toISOString(),
      tags: ['medical', 'iot', 'healthcare']
    }
  ]
};

export const simulateShodanSearch = async (
  sector: string,
  apiKey?: string
): Promise<ShodanResult[]> => {
  // Simulate network delay
  await new Promise(resolve => setTimeout(resolve, 1000));

  // Check if API key is provided (in real implementation, validate the key)
  if (!apiKey) {
    console.log('No API key provided, using demo data');
  }

  // Return demo data for the sector
  return demoResults[sector.toLowerCase()] || [];
};