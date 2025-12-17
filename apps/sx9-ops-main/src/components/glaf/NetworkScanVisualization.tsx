/**
 * Network Scan Visualization Component
 *
 * Real-time network graph visualization from nmap/rustscan results
 * Uses Cytoscape.js for interactive network mapping
 */

import { useEffect, useRef, useState } from "react";
import cytoscape, { Core } from "cytoscape";
import dagre from "cytoscape-dagre";
import {
  parseNmapXML,
  parseRustScanJSON,
  buildNetworkGraph,
  mergeScanResults,
  type ScanHost,
  type NetworkGraph,
} from "@/lib/glaf/networkScanParser";
import { Play, RefreshCw, Download, ZoomIn, ZoomOut, Maximize2 } from "lucide-react";

cytoscape.use(dagre);

interface NetworkScanVisualizationProps {
  scanData?: string;
  scanType?: "nmap-xml" | "rustscan-json";
  autoRefresh?: boolean;
  refreshInterval?: number;
}

export default function NetworkScanVisualization({
  scanData,
  scanType = "nmap-xml",
  autoRefresh = false,
  refreshInterval = 30000,
}: NetworkScanVisualizationProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const cyRef = useRef<Core | null>(null);
  const [hosts, setHosts] = useState<ScanHost[]>([]);
  const [graph, setGraph] = useState<NetworkGraph>({ nodes: [], edges: [] });
  const [selectedNode, setSelectedNode] = useState<string | null>(null);

  // Parse scan data
  useEffect(() => {
    if (!scanData) return;

    try {
      const parsedHosts =
        scanType === "nmap-xml" ? parseNmapXML(scanData) : parseRustScanJSON(scanData);

      setHosts(prev => mergeScanResults(prev, parsedHosts));
    } catch (err) {
      console.error("Failed to parse scan data:", err);
    }
  }, [scanData, scanType]);

  // Build graph from hosts
  useEffect(() => {
    if (hosts.length === 0) return;
    const networkGraph = buildNetworkGraph(hosts);
    setGraph(networkGraph);
  }, [hosts]);

  // Initialize Cytoscape
  useEffect(() => {
    if (!containerRef.current || graph.nodes.length === 0) return;

    const cy = cytoscape({
      container: containerRef.current,

      elements: [
        ...graph.nodes.map(node => ({
          data: {
            id: node.id,
            label: node.label,
            type: node.type,
            ...node.data,
          },
        })),
        ...graph.edges.map(edge => ({
          data: {
            id: edge.id,
            source: edge.source,
            target: edge.target,
            label: edge.label,
            type: edge.type,
          },
        })),
      ],

      style: [
        // Host nodes
        {
          selector: 'node[type="host"]',
          style: {
            "background-color": (ele: any) => {
              const risk = ele.data("risk");
              return risk === "critical"
                ? "#ef4444"
                : risk === "high"
                  ? "#f97316"
                  : risk === "medium"
                    ? "#eab308"
                    : "#22c55e";
            },
            label: "data(label)",
            "text-valign": "center",
            "text-halign": "center",
            color: "#fff",
            "font-size": "12px",
            "font-weight": "bold",
            width: "60px",
            height: "60px",
            "border-width": 2,
            "border-color": "#1e293b",
            "text-wrap": "wrap",
            "text-max-width": "50px",
          },
        },

        // Port nodes
        {
          selector: 'node[type="port"]',
          style: {
            "background-color": "#3b82f6",
            label: "data(label)",
            "text-valign": "center",
            color: "#fff",
            "font-size": "10px",
            width: "40px",
            height: "40px",
            shape: "rectangle",
          },
        },

        // Service nodes
        {
          selector: 'node[type="service"]',
          style: {
            "background-color": "#8b5cf6",
            label: "data(label)",
            "text-valign": "center",
            color: "#fff",
            "font-size": "11px",
            width: "50px",
            height: "50px",
            shape: "diamond",
          },
        },

        // Edges
        {
          selector: "edge",
          style: {
            width: 2,
            "line-color": "#64748b",
            "target-arrow-color": "#64748b",
            "target-arrow-shape": "triangle",
            "curve-style": "bezier",
            label: "data(label)",
            "font-size": "9px",
            color: "#94a3b8",
            "text-rotation": "autorotate",
          },
        },

        // Selected node
        {
          selector: ":selected",
          style: {
            "border-width": 4,
            "border-color": "#0ea5e9",
            "overlay-color": "#0ea5e9",
            "overlay-opacity": 0.2,
          },
        },
      ],

      layout: {
        name: "dagre",
        rankDir: "TB",
        nodeSep: 50,
        rankSep: 100,
        padding: 30,
      },

      minZoom: 0.3,
      maxZoom: 3,
      wheelSensitivity: 0.2,
    });

    // Event handlers
    cy.on("tap", "node", evt => {
      const node = evt.target;
      setSelectedNode(node.id());
      console.log("Selected node:", node.data());
    });

    cy.on("tap", evt => {
      if (evt.target === cy) {
        setSelectedNode(null);
      }
    });

    cyRef.current = cy;

    return () => {
      cy.destroy();
    };
  }, [graph]);

  // Auto-refresh
  useEffect(() => {
    if (!autoRefresh) return;

    const interval = setInterval(() => {
      // Trigger re-scan (would call backend API)
      console.log("Auto-refreshing network scan...");
    }, refreshInterval);

    return () => clearInterval(interval);
  }, [autoRefresh, refreshInterval]);

  // Control functions
  const handleZoomIn = () => {
    cyRef.current?.zoom(cyRef.current.zoom() * 1.2);
  };

  const handleZoomOut = () => {
    cyRef.current?.zoom(cyRef.current.zoom() * 0.8);
  };

  const handleFit = () => {
    cyRef.current?.fit(undefined, 50);
  };

  const handleReset = () => {
    cyRef.current?.reset();
  };

  const handleExport = () => {
    if (!cyRef.current) return;
    const png = cyRef.current.png({ full: true, scale: 2 });
    const link = document.createElement("a");
    link.href = png;
    link.download = `network-scan-${Date.now()}.png`;
    link.click();
  };

  return (
    <div className="relative w-full h-full bg-gray-900">
      {/* Controls */}
      <div className="absolute top-4 right-4 z-10 flex flex-col gap-2">
        <button
          onClick={handleZoomIn}
          className="p-2 bg-gray-800 hover:bg-gray-700 text-white rounded shadow-lg"
          title="Zoom In"
        >
          <ZoomIn size={20} />
        </button>
        <button
          onClick={handleZoomOut}
          className="p-2 bg-gray-800 hover:bg-gray-700 text-white rounded shadow-lg"
          title="Zoom Out"
        >
          <ZoomOut size={20} />
        </button>
        <button
          onClick={handleFit}
          className="p-2 bg-gray-800 hover:bg-gray-700 text-white rounded shadow-lg"
          title="Fit to Screen"
        >
          <Maximize2 size={20} />
        </button>
        <button
          onClick={handleReset}
          className="p-2 bg-gray-800 hover:bg-gray-700 text-white rounded shadow-lg"
          title="Reset View"
        >
          <RefreshCw size={20} />
        </button>
        <button
          onClick={handleExport}
          className="p-2 bg-gray-800 hover:bg-gray-700 text-white rounded shadow-lg"
          title="Export PNG"
        >
          <Download size={20} />
        </button>
      </div>

      {/* Stats */}
      <div className="absolute top-4 left-4 z-10 bg-gray-800/90 text-white p-4 rounded shadow-lg">
        <div className="text-sm space-y-1">
          <div className="font-bold text-blue-400">Network Scan</div>
          <div>
            Hosts: <span className="font-mono">{hosts.length}</span>
          </div>
          <div>
            Nodes: <span className="font-mono">{graph.nodes.length}</span>
          </div>
          <div>
            Edges: <span className="font-mono">{graph.edges.length}</span>
          </div>
        </div>
      </div>

      {/* Selected Node Info */}
      {selectedNode && (
        <div className="absolute bottom-4 left-4 z-10 bg-gray-800/90 text-white p-4 rounded shadow-lg max-w-sm">
          <div className="text-sm space-y-1">
            <div className="font-bold text-blue-400">Selected Node</div>
            <div className="font-mono text-xs break-all">{selectedNode}</div>
            {/* Add more node details here */}
          </div>
        </div>
      )}

      {/* Legend */}
      <div className="absolute bottom-4 right-4 z-10 bg-gray-800/90 text-white p-3 rounded shadow-lg">
        <div className="text-xs space-y-2">
          <div className="font-bold text-blue-400 mb-2">Legend</div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-red-500"></div>
            <span>Critical Risk</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-orange-500"></div>
            <span>High Risk</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-yellow-500"></div>
            <span>Medium Risk</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-green-500"></div>
            <span>Low Risk</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-blue-500"></div>
            <span>Port</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-purple-500 rotate-45"></div>
            <span>Service</span>
          </div>
        </div>
      </div>

      {/* Graph Container */}
      <div ref={containerRef} className="w-full h-full" />
    </div>
  );
}
