// Shodan API utility functions

export interface ShodanResult {
  ip: string;
  port: number;
  service: string;
  banner: string;
  timestamp: string;
  location: {
    country: string;
    city: string;
    latitude: number;
    longitude: number;
  };
  tags: string[];
  vulns: string[];
}

export interface ShodanSearchParams {
  query: string;
  facets?: string;
  minify?: boolean;
}

export const shodanApi = {
  search: async (params: ShodanSearchParams): Promise<ShodanResult[]> => {
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 500));
    
    const mockResults: ShodanResult[] = [
      {
        ip: '192.168.1.100',
        port: 22,
        service: 'ssh',
        banner: 'SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5',
        timestamp: new Date().toISOString(),
        location: {
          country: 'United States',
          city: 'New York',
          latitude: 40.7128,
          longitude: -74.0060
        },
        tags: ['ssh', 'linux', 'ubuntu'],
        vulns: ['CVE-2021-28041']
      },
      {
        ip: '10.0.0.50',
        port: 80,
        service: 'http',
        banner: 'Apache/2.4.41 (Ubuntu)',
        timestamp: new Date().toISOString(),
        location: {
          country: 'United States',
          city: 'Los Angeles',
          latitude: 34.0522,
          longitude: -118.2437
        },
        tags: ['http', 'apache', 'web'],
        vulns: []
      }
    ];

    return mockResults;
  },

  host: async (ip: string): Promise<ShodanResult | null> => {
    await new Promise(resolve => setTimeout(resolve, 300));
    
    return {
      ip,
      port: 22,
      service: 'ssh',
      banner: 'SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5',
      timestamp: new Date().toISOString(),
      location: {
        country: 'United States',
        city: 'New York',
        latitude: 40.7128,
        longitude: -74.0060
      },
      tags: ['ssh', 'linux', 'ubuntu'],
      vulns: ['CVE-2021-28041']
    };
  },

  getApiInfo: async (): Promise<{ query_credits: number; scan_credits: number; usage_limits: any }> => {
    await new Promise(resolve => setTimeout(resolve, 100));
    
    return {
      query_credits: 100,
      scan_credits: 50,
      usage_limits: {
        queries_per_second: 1,
        queries_per_day: 1000
      }
    };
  }
};