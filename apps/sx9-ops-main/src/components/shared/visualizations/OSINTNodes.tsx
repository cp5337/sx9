import React, { useEffect, useState } from 'react';
import { Eye, AlertCircle, CheckCircle2, Loader2, Database } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';

interface OSINTNode {
  id: string;
  label: string;
  type: 'threat_actor' | 'campaign' | 'ioc' | 'vulnerability' | 'asset';
  properties: Record<string, any>;
  position?: { x: number; y: number };
}

interface OSINTEdge {
  id: string;
  source: string;
  target: string;
  type: string;
  properties: Record<string, any>;
}

interface OSINTNodesProps {
  onNodeClick?: (nodeId: string) => void;
  className?: string;
  useNeo4j?: boolean; // Use Neo4j instead of GLAF
}

/**
 * OSINTNodes - Visualizes OSINT (Open Source Intelligence) nodes
 * Based on cloned ground stations converted to WASM microkernels
 * Supports both Neo4j (port 7687) and GLAF (port 18050) backends
 */
export const OSINTNodes: React.FC<OSINTNodesProps> = ({ 
  onNodeClick, 
  className = '',
  useNeo4j = true // Default to Neo4j while GLAF is being configured
}) => {
  const [nodes, setNodes] = useState<OSINTNode[]>([]);
  const [edges, setEdges] = useState<OSINTEdge[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [connectionStatus, setConnectionStatus] = useState<'connected' | 'disconnected'>('disconnected');
  const canvasRef = React.useRef<HTMLCanvasElement>(null);

  // Load OSINT nodes from Neo4j or GLAF
  useEffect(() => {
    const loadOSINTNodes = async () => {
      try {
        setLoading(true);
        setError(null);

        if (useNeo4j) {
          // Use Neo4j (port 7687)
          await loadFromNeo4j();
        } else {
          // Use GLAF (port 18050) - fallback
          await loadFromGLAF();
        }
      } catch (err) {
        console.error('[OSINTNodes] Error loading nodes:', err);
        setError(err instanceof Error ? err.message : 'Failed to load OSINT nodes');
        setLoading(false);
      }
    };

    loadOSINTNodes();
  }, [useNeo4j]);

  const loadFromNeo4j = async () => {
    try {
      // Neo4j HTTP API endpoint (using Neo4j's HTTP API on port 7474)
      const neo4jUrl = import.meta.env.VITE_NEO4J_URI || 'http://localhost:7474';
      const neo4jUser = import.meta.env.VITE_NEO4J_USER || 'neo4j';
      const neo4jPassword = import.meta.env.VITE_NEO4J_PASSWORD || 'ctas7_graph';

      // Cypher query to get OSINT-related nodes
      const cypherQuery = `
        MATCH (n)
        WHERE n:ThreatActor OR n:Campaign OR n:IOC OR n:Vulnerability OR n:Asset
        OPTIONAL MATCH (n)-[r]->(m)
        RETURN n, r, m
        LIMIT 100
      `;

      // Use Neo4j HTTP API
      const response = await fetch(`${neo4jUrl}/db/neo4j/tx/commit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Basic ${btoa(`${neo4jUser}:${neo4jPassword}`)}`
        },
        body: JSON.stringify({
          statements: [{
            statement: cypherQuery,
            resultDataContents: ['graph']
          }]
        })
      });

      if (!response.ok) {
        // Fallback: try Bolt protocol via HTTP proxy or use mock data
        console.warn('[OSINTNodes] Neo4j HTTP API failed, using mock data');
        setMockData();
        setConnectionStatus('disconnected');
        setLoading(false);
        return;
      }

      const data = await response.json();
      
      if (data.errors && data.errors.length > 0) {
        throw new Error(data.errors[0].message);
      }

      // Parse Neo4j graph format
      const graphData = data.results[0]?.data || [];
      const parsedNodes: OSINTNode[] = [];
      const parsedEdges: OSINTEdge[] = [];
      const nodeMap = new Map<string, OSINTNode>();

      graphData.forEach((row: any) => {
        // Process nodes
        if (row.graph?.nodes) {
          row.graph.nodes.forEach((node: any) => {
            if (!nodeMap.has(node.id)) {
              const labels = node.labels || [];
              const nodeType = labels.find((l: string) => 
                ['ThreatActor', 'Campaign', 'IOC', 'Vulnerability', 'Asset'].includes(l)
              )?.toLowerCase().replace(/([A-Z])/g, '_$1').toLowerCase() || 'asset';

              parsedNodes.push({
                id: node.id.toString(),
                label: node.properties?.name || node.properties?.id || `Node ${node.id}`,
                type: nodeType as OSINTNode['type'],
                properties: node.properties || {},
                position: {
                  x: Math.random() * 600 + 100,
                  y: Math.random() * 400 + 100
                }
              });
              nodeMap.set(node.id.toString(), parsedNodes[parsedNodes.length - 1]);
            }
          });
        }

        // Process relationships
        if (row.graph?.relationships) {
          row.graph.relationships.forEach((rel: any) => {
            parsedEdges.push({
              id: rel.id.toString(),
              source: rel.startNode.toString(),
              target: rel.endNode.toString(),
              type: rel.type,
              properties: rel.properties || {}
            });
          });
        }
      });

      setNodes(parsedNodes);
      setEdges(parsedEdges);
      setConnectionStatus('connected');
      setLoading(false);
    } catch (err) {
      console.warn('[OSINTNodes] Neo4j connection failed, using mock data:', err);
      setMockData();
      setConnectionStatus('disconnected');
      setLoading(false);
    }
  };

  const loadFromGLAF = async () => {
    try {
      // GLAF API endpoint (port 18050)
      const glafUrl = 'http://localhost:18050';
      
      const response = await fetch(`${glafUrl}/api/graph/nodes?limit=100&label=OSINT`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        }
      });

      if (!response.ok) {
        throw new Error(`GLAF API error: ${response.statusText}`);
      }

      const data = await response.json();
      
      // Parse GLAF format
      const parsedNodes: OSINTNode[] = (data.nodes || []).map((node: any, index: number) => ({
        id: node.id || `node-${index}`,
        label: node.name || node.label || `Node ${index}`,
        type: node.type || 'asset',
        properties: node.properties || {},
        position: {
          x: Math.random() * 600 + 100,
          y: Math.random() * 400 + 100
        }
      }));

      const parsedEdges: OSINTEdge[] = (data.edges || []).map((edge: any, index: number) => ({
        id: edge.id || `edge-${index}`,
        source: edge.source,
        target: edge.target,
        type: edge.type || 'RELATED_TO',
        properties: edge.properties || {}
      }));

      setNodes(parsedNodes);
      setEdges(parsedEdges);
      setConnectionStatus('connected');
      setLoading(false);
    } catch (err) {
      console.warn('[OSINTNodes] GLAF connection failed, using mock data:', err);
      setMockData();
      setConnectionStatus('disconnected');
      setLoading(false);
    }
  };

  const setMockData = () => {
    // Mock data for demo
    const mockNodes: OSINTNode[] = [
      { id: '1', label: 'APT28', type: 'threat_actor', properties: {}, position: { x: 200, y: 150 } },
      { id: '2', label: 'SolarWinds Campaign', type: 'campaign', properties: {}, position: { x: 400, y: 150 } },
      { id: '3', label: 'CVE-2021-44228', type: 'vulnerability', properties: {}, position: { x: 300, y: 300 } },
      { id: '4', label: '192.168.1.100', type: 'ioc', properties: {}, position: { x: 500, y: 300 } }
    ];

    const mockEdges: OSINTEdge[] = [
      { id: 'e1', source: '1', target: '2', type: 'ORCHESTRATES', properties: {} },
      { id: 'e2', source: '2', target: '3', type: 'EXPLOITS', properties: {} },
      { id: 'e3', source: '2', target: '4', type: 'USES', properties: {} }
    ];

    setNodes(mockNodes);
    setEdges(mockEdges);
  };

  // Render graph on canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || loading) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw edges
    edges.forEach(edge => {
      const sourceNode = nodes.find(n => n.id === edge.source);
      const targetNode = nodes.find(n => n.id === edge.target);
      
      if (!sourceNode || !targetNode || !sourceNode.position || !targetNode.position) return;

      ctx.strokeStyle = '#3b82f6';
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      ctx.moveTo(sourceNode.position.x, sourceNode.position.y);
      ctx.lineTo(targetNode.position.x, targetNode.position.y);
      ctx.stroke();
    });

    // Draw nodes
    nodes.forEach(node => {
      if (!node.position) return;

      const x = node.position.x;
      const y = node.position.y;

      const colors: Record<string, string> = {
        'threat_actor': '#ef4444',
        'campaign': '#f59e0b',
        'ioc': '#10b981',
        'vulnerability': '#8b5cf6',
        'asset': '#3b82f6'
      };

      ctx.fillStyle = colors[node.type] || '#6b7280';
      ctx.beginPath();
      ctx.arc(x, y, 12, 0, Math.PI * 2);
      ctx.fill();

      ctx.strokeStyle = '#1f2937';
      ctx.lineWidth = 2;
      ctx.stroke();

      ctx.fillStyle = '#e5e7eb';
      ctx.font = '11px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(node.label, x, y + 25);
    });
  }, [nodes, edges, loading]);

  return (
    <Card className={`p-4 bg-gray-900 border-gray-800 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Eye className="h-5 w-5 text-gray-400" />
          <h3 className="text-sm font-mono font-bold uppercase tracking-wide text-gray-200">
            OSINT Nodes
          </h3>
        </div>
        <div className="flex items-center gap-2">
          <Badge className={`text-xs font-mono ${
            connectionStatus === 'connected' 
              ? 'bg-green-500/20 text-green-400 border-green-500/50' 
              : 'bg-gray-500/20 text-gray-400 border-gray-500/50'
          }`}>
            {useNeo4j ? 'Neo4j' : 'GLAF'}
          </Badge>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => {
              // Toggle between Neo4j and GLAF
              const newUseNeo4j = !useNeo4j;
              setLoading(true);
              setTimeout(() => {
                if (newUseNeo4j) {
                  loadFromNeo4j();
                } else {
                  loadFromGLAF();
                }
              }, 100);
            }}
            className="h-6 text-xs"
          >
            Switch
          </Button>
        </div>
      </div>

      {error && (
        <div className="mb-4 p-2 bg-red-500/10 border border-red-500/50 rounded text-xs text-red-400 font-mono">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center h-96 text-gray-500">
          <Loader2 className="h-6 w-6 animate-spin mr-2" />
          <span className="text-xs font-mono">
            Loading OSINT nodes from {useNeo4j ? 'Neo4j' : 'GLAF'}...
          </span>
        </div>
      ) : (
        <div className="relative bg-gray-950 rounded border border-gray-800" style={{ height: '400px' }}>
          <canvas
            ref={canvasRef}
            className="w-full h-full"
            style={{ cursor: 'pointer' }}
            onClick={(e) => {
              const rect = canvasRef.current?.getBoundingClientRect();
              if (rect && onNodeClick) {
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;
                const clickedNode = nodes.find(n => {
                  if (!n.position) return false;
                  const distance = Math.sqrt(
                    Math.pow(x - n.position.x, 2) + Math.pow(y - n.position.y, 2)
                  );
                  return distance < 15;
                });
                if (clickedNode) {
                  onNodeClick(clickedNode.id);
                }
              }
            }}
          />
          
          {nodes.length === 0 && !loading && (
            <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-xs font-mono">
              No OSINT nodes found
            </div>
          )}
        </div>
      )}

      <div className="mt-4 flex flex-wrap gap-2 text-xs text-gray-500 font-mono">
        <span>Nodes: {nodes.length}</span>
        <span>•</span>
        <span>Relationships: {edges.length}</span>
        <span>•</span>
        <span>Backend: {useNeo4j ? 'Neo4j (7687)' : 'GLAF (18050)'}</span>
      </div>
    </Card>
  );
};

export default OSINTNodes;

