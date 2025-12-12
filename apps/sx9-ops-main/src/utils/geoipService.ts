/**
 * GeoIP Service - Simple MaxMind GeoLite2 Integration
 * Uses the test database for now, can be upgraded to full GeoIP2 later
 */

export interface GeoIPResult {
  ip: string;
  country?: string;
  city?: string;
  latitude: number;
  longitude: number;
  accuracy_radius?: number;
  timezone?: string;
  postal_code?: string;
  continent?: string;
  threat_level?: 'low' | 'medium' | 'high' | 'critical';
}

/**
 * Simple GeoIP lookup using MaxMind test database
 * For production, this would call the enhanced-geolocation API
 */
export class GeoIPService {
  private static instance: GeoIPService;
  private cache: Map<string, GeoIPResult> = new Map();

  private constructor() {}

  public static getInstance(): GeoIPService {
    if (!GeoIPService.instance) {
      GeoIPService.instance = new GeoIPService();
    }
    return GeoIPService.instance;
  }

  /**
   * Lookup IP address geolocation
   * For now, returns mock data based on IP patterns
   * TODO: Integrate with ctas7-enhanced-geolocation API when it's running
   */
  public async lookup(ip: string): Promise<GeoIPResult> {
    // Check cache first
    if (this.cache.has(ip)) {
      return this.cache.get(ip)!;
    }

    // Mock data based on IP ranges (for demo purposes)
    const result = this.getMockGeoIP(ip);
    
    // Cache result
    this.cache.set(ip, result);
    
    return result;
  }

  /**
   * Lookup multiple IPs in batch
   */
  public async lookupBatch(ips: string[]): Promise<Map<string, GeoIPResult>> {
    const results = new Map<string, GeoIPResult>();
    
    for (const ip of ips) {
      const result = await this.lookup(ip);
      results.set(ip, result);
    }
    
    return results;
  }

  /**
   * Clear cache
   */
  public clearCache(): void {
    this.cache.clear();
  }

  /**
   * Mock GeoIP data generator (for testing without API)
   * Maps IP patterns to realistic locations
   */
  private getMockGeoIP(ip: string): GeoIPResult {
    const octets = ip.split('.').map(Number);
    
    // Determine location based on first octet
    const firstOctet = octets[0];
    
    // Known malicious IP ranges (simplified)
    const isMalicious = this.checkMaliciousIP(ip);
    
    // Generate location based on IP
    let country = 'Unknown';
    let city = 'Unknown';
    let lat = 0;
    let lng = 0;
    let continent = 'Unknown';
    
    if (firstOctet >= 1 && firstOctet <= 50) {
      // North America
      country = 'United States';
      city = 'New York';
      lat = 40.7128;
      lng = -74.0060;
      continent = 'North America';
    } else if (firstOctet >= 51 && firstOctet <= 100) {
      // Europe
      country = 'United Kingdom';
      city = 'London';
      lat = 51.5074;
      lng = -0.1278;
      continent = 'Europe';
    } else if (firstOctet >= 101 && firstOctet <= 150) {
      // Asia
      country = 'China';
      city = 'Beijing';
      lat = 39.9042;
      lng = 116.4074;
      continent = 'Asia';
    } else if (firstOctet >= 151 && firstOctet <= 200) {
      // Russia
      country = 'Russia';
      city = 'Moscow';
      lat = 55.7558;
      lng = 37.6173;
      continent = 'Europe';
    } else {
      // Other
      country = 'Brazil';
      city = 'São Paulo';
      lat = -23.5505;
      lng = -46.6333;
      continent = 'South America';
    }
    
    // Add some randomness for realism
    lat += (Math.random() - 0.5) * 0.1;
    lng += (Math.random() - 0.5) * 0.1;
    
    return {
      ip,
      country,
      city,
      latitude: lat,
      longitude: lng,
      accuracy_radius: 50000, // 50km radius
      continent,
      threat_level: isMalicious ? 'high' : 'low'
    };
  }

  /**
   * Check if IP is in known malicious ranges
   * This is a simplified check - production would use threat intel feeds
   */
  private checkMaliciousIP(ip: string): boolean {
    // Known malicious patterns (simplified)
    const maliciousPatterns = [
      /^203\.0\.113\./, // TEST-NET-3 (RFC 5737)
      /^192\.0\.2\./,   // TEST-NET-1
      /^198\.51\.100\./, // TEST-NET-2
      /^185\./, // Common VPN/proxy range
      /^45\./,  // Common hosting range
    ];
    
    return maliciousPatterns.some(pattern => pattern.test(ip));
  }
}

// Export singleton instance
export const geoipService = GeoIPService.getInstance();

