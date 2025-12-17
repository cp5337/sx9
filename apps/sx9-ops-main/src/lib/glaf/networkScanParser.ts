/**
 * Network Scan Parser - Nmap & RustScan
 *
 * Parses nmap XML and RustScan JSON output into graph nodes/edges
 * for Cytoscape visualization
 */

export interface ScanHost {
  ip: string;
  hostname?: string;
  mac?: string;
  vendor?: string;
  os?: string;
  status: "up" | "down";
  ports: ScanPort[];
  lastSeen: number;
}

export interface ScanPort {
  port: number;
  protocol: "tcp" | "udp";
  state: "open" | "closed" | "filtered";
  service?: string;
  version?: string;
  product?: string;
}

export interface NetworkGraph {
  nodes: NetworkNode[];
  edges: NetworkEdge[];
}

export interface NetworkNode {
  id: string;
  type: "host" | "port" | "service" | "subnet";
  label: string;
  data: {
    ip?: string;
    hostname?: string;
    port?: number;
    service?: string;
    status?: string;
    os?: string;
    vendor?: string;
    risk?: "critical" | "high" | "medium" | "low";
  };
}

export interface NetworkEdge {
  id: string;
  source: string;
  target: string;
  type: "hosts" | "runs" | "connects_to";
  label?: string;
}

/**
 * Parse Nmap XML output
 */
export function parseNmapXML(xml: string): ScanHost[] {
  const parser = new DOMParser();
  const doc = parser.parseFromString(xml, "text/xml");
  const hosts: ScanHost[] = [];

  const hostElements = doc.querySelectorAll("host");

  hostElements.forEach(hostEl => {
    const status = hostEl.querySelector("status")?.getAttribute("state") as "up" | "down";
    const addressEl = hostEl.querySelector('address[addrtype="ipv4"]');
    const macEl = hostEl.querySelector('address[addrtype="mac"]');
    const hostnameEl = hostEl.querySelector("hostname");
    const osEl = hostEl.querySelector("osmatch");

    const ip = addressEl?.getAttribute("addr") || "";
    const mac = macEl?.getAttribute("addr");
    const vendor = macEl?.getAttribute("vendor");
    const hostname = hostnameEl?.getAttribute("name");
    const os = osEl?.getAttribute("name");

    const ports: ScanPort[] = [];
    const portElements = hostEl.querySelectorAll("port");

    portElements.forEach(portEl => {
      const portNum = parseInt(portEl.getAttribute("portid") || "0");
      const protocol = portEl.getAttribute("protocol") as "tcp" | "udp";
      const stateEl = portEl.querySelector("state");
      const serviceEl = portEl.querySelector("service");

      ports.push({
        port: portNum,
        protocol,
        state: stateEl?.getAttribute("state") as "open" | "closed" | "filtered",
        service: serviceEl?.getAttribute("name"),
        version: serviceEl?.getAttribute("version"),
        product: serviceEl?.getAttribute("product"),
      });
    });

    hosts.push({
      ip,
      hostname,
      mac,
      vendor,
      os,
      status,
      ports,
      lastSeen: Date.now(),
    });
  });

  return hosts;
}

/**
 * Parse RustScan JSON output
 */
export function parseRustScanJSON(json: string): ScanHost[] {
  const data = JSON.parse(json);
  const hosts: ScanHost[] = [];

  // RustScan format: { "ip": "192.168.1.1", "ports": [22, 80, 443] }
  if (Array.isArray(data)) {
    data.forEach(item => {
      const ports: ScanPort[] = item.ports.map((port: number) => ({
        port,
        protocol: "tcp" as const,
        state: "open" as const,
        service: getServiceName(port),
      }));

      hosts.push({
        ip: item.ip,
        status: "up",
        ports,
        lastSeen: Date.now(),
      });
    });
  } else if (data.ip) {
    // Single host format
    const ports: ScanPort[] = data.ports.map((port: number) => ({
      port,
      protocol: "tcp" as const,
      state: "open" as const,
      service: getServiceName(port),
    }));

    hosts.push({
      ip: data.ip,
      status: "up",
      ports,
      lastSeen: Date.now(),
    });
  }

  return hosts;
}

/**
 * Convert scan results to Cytoscape graph
 */
export function buildNetworkGraph(hosts: ScanHost[]): NetworkGraph {
  const nodes: NetworkNode[] = [];
  const edges: NetworkEdge[] = [];

  hosts.forEach(host => {
    // Create host node
    const hostId = `host-${host.ip}`;
    nodes.push({
      id: hostId,
      type: "host",
      label: host.hostname || host.ip,
      data: {
        ip: host.ip,
        hostname: host.hostname,
        status: host.status,
        os: host.os,
        vendor: host.vendor,
        risk: calculateRisk(host),
      },
    });

    // Create port/service nodes
    host.ports.forEach(port => {
      if (port.state === "open") {
        const portId = `${hostId}-port-${port.port}`;
        nodes.push({
          id: portId,
          type: "port",
          label: `${port.port}/${port.protocol}`,
          data: {
            port: port.port,
            service: port.service,
            status: port.state,
          },
        });

        // Edge: host -> port
        edges.push({
          id: `${hostId}-to-${portId}`,
          source: hostId,
          target: portId,
          type: "hosts",
          label: port.service,
        });

        // Create service node if service identified
        if (port.service) {
          const serviceId = `service-${port.service}`;

          // Only add service node once
          if (!nodes.find(n => n.id === serviceId)) {
            nodes.push({
              id: serviceId,
              type: "service",
              label: port.service.toUpperCase(),
              data: {
                service: port.service,
              },
            });
          }

          // Edge: port -> service
          edges.push({
            id: `${portId}-to-${serviceId}`,
            source: portId,
            target: serviceId,
            type: "runs",
          });
        }
      }
    });
  });

  return { nodes, edges };
}

/**
 * Calculate risk level based on open ports and services
 */
function calculateRisk(host: ScanHost): "critical" | "high" | "medium" | "low" {
  const criticalPorts = [21, 23, 445, 3389, 5900]; // FTP, Telnet, SMB, RDP, VNC
  const highRiskPorts = [22, 3306, 5432, 27017]; // SSH, MySQL, PostgreSQL, MongoDB

  const openPorts = host.ports.filter(p => p.state === "open");

  if (openPorts.some(p => criticalPorts.includes(p.port))) {
    return "critical";
  }

  if (openPorts.some(p => highRiskPorts.includes(p.port))) {
    return "high";
  }

  if (openPorts.length > 10) {
    return "medium";
  }

  return "low";
}

/**
 * Get common service name from port number
 */
function getServiceName(port: number): string {
  const commonPorts: Record<number, string> = {
    21: "ftp",
    22: "ssh",
    23: "telnet",
    25: "smtp",
    53: "dns",
    80: "http",
    110: "pop3",
    143: "imap",
    443: "https",
    445: "smb",
    3306: "mysql",
    3389: "rdp",
    5432: "postgresql",
    5900: "vnc",
    8080: "http-proxy",
    27017: "mongodb",
  };

  return commonPorts[port] || "unknown";
}

/**
 * Merge multiple scans (incremental updates)
 */
export function mergeScanResults(existing: ScanHost[], newScan: ScanHost[]): ScanHost[] {
  const merged = new Map<string, ScanHost>();

  // Add existing hosts
  existing.forEach(host => merged.set(host.ip, host));

  // Update with new scan results
  newScan.forEach(host => {
    const existingHost = merged.get(host.ip);

    if (existingHost) {
      // Merge ports
      const portMap = new Map<number, ScanPort>();
      existingHost.ports.forEach(p => portMap.set(p.port, p));
      host.ports.forEach(p => portMap.set(p.port, p));

      merged.set(host.ip, {
        ...existingHost,
        ...host,
        ports: Array.from(portMap.values()),
        lastSeen: Date.now(),
      });
    } else {
      merged.set(host.ip, host);
    }
  });

  return Array.from(merged.values());
}
