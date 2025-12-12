export interface PortInfo {
  port: number;
  state: 'open' | 'closed' | 'filtered';
  service: string;
}

export interface ServiceInfo {
  name: string;
  version: string;
  port: number;
  protocol: string;
  banner?: string;
}

export interface OSInfo {
  name: string;
  version: string;
  accuracy: number;
}

export interface ScanResult {
  target: string;
  timestamp: string;
  ports: PortInfo[];
  services: ServiceInfo[];
  vulnerabilities: string[];
  osInfo: OSInfo;
}

export interface Contact {
  email?: string;
  phone?: string;
  name?: string;
  organization?: string;
}

export interface DomainInfo {
  registrar: string;
  creationDate: string;
  expiryDate: string;
  nameservers: string[];
  status: string;
  contacts: {
    admin: Contact | null;
    technical: Contact | null;
  };
}

export interface SocialMediaProfile {
  platform: string;
  username: string;
  url: string;
  followers?: number;
  lastActive?: string;
  verified?: boolean;
}

export interface OsintResult {
  target: string;
  timestamp: string;
  domainInfo: DomainInfo;
  socialProfiles: SocialMediaProfile[];
  digitalFootprint: unknown[];
}

export interface HuntOperation {
  id: string;
  target: string;
  startTime: string;
  endTime?: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  type: 'network-scan' | 'osint' | 'vulnerability-assessment';
  results: ScanResult | OsintResult | null;
  error?: string;
}