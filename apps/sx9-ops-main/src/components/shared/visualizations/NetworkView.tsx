import React, { useEffect, useState, useRef } from 'react';
import { Network, AlertCircle, CheckCircle2 } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface NetworkNode {
  id: string;
  label: string;
  type: 'gateway' | 'service' | 'database' | 'agent';
  status: 'connected' | 'disconnected' | 'error';
  position?: { x: number; y: number };
}

interface NetworkEdge {
  id: string;
  source: string;
  target: string;
  type: 'data' | 'control' | 'websocket';
}

interface NetworkViewProps {
  onNodeClick?: (nodeId: string) => void;
  className?: string;
}

export const NetworkView: React.FC<NetworkViewProps> = ({ onNodeClick, className = '' }) => {
  const [nodes, setNodes] = useState<NetworkNode[]>([]);
  const [edges, setEdges] = useState<NetworkEdge[]>([]);
  const [connectionStatus, setConnectionStatus] = useState<'connecting' | 'connected' | 'disconnected'>('disconnected');
  const [error, setError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // Connect to SX9 Gateway WebSocket (port 18600)
  useEffect(() => {
    const connectWebSocket = () => {
      try {
        setConnectionStatus('connecting');
        const ws = new WebSocket('ws://localhost:18600');
        
        ws.onopen = () => {
          setConnectionStatus('connected');
          setError(null);
          console.log('[NetworkView] Connected to SX9 Gateway');
          
          // Request initial network topology
          ws.send(JSON.stringify({
            type: 'get_network_topology',
            timestamp: Date.now()
          }));
        };

        ws.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            
            if (data.type === 'network_topology') {
              // Update nodes and edges from gateway
              if (data.nodes) {
                setNodes(data.nodes.map((n: any) => ({
                  id: n.id || n.hash_id,
                  label: n.name || n.label || n.id,
                  type: n.type || 'service',
                  status: n.status || 'connected',
                  position: n.position
                })));
              }
              
              if (data.edges) {
                setEdges(data.edges.map((e: any) => ({
                  id: e.id || `${e.source}-${e.target}`,
                  source: e.source,
                  target: e.target,
                  type: e.type || 'data'
                })));
              }
            } else if (data.type === 'node_update') {
              // Update single node
              setNodes(prev => prev.map(n => 
                n.id === data.node.id ? { ...n, ...data.node } : n
              ));
            } else if (data.type === 'edge_update') {
              // Update single edge
              setEdges(prev => prev.map(e => 
                e.id === data.edge.id ? { ...e, ...data.edge } : e
              ));
            }
          } catch (err) {
            console.error('[NetworkView] Error parsing WebSocket message:', err);
          }
        };

        ws.onerror = (err) => {
          console.error('[NetworkView] WebSocket error:', err);
          setError('Failed to connect to SX9 Gateway');
          setConnectionStatus('disconnected');
        };

        ws.onclose = () => {
          setConnectionStatus('disconnected');
          // Attempt to reconnect after 3 seconds
          setTimeout(connectWebSocket, 3000);
        };

        wsRef.current = ws;
      } catch (err) {
        setError('WebSocket connection failed');
        setConnectionStatus('disconnected');
      }
    };

    connectWebSocket();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, []);

  // Simple canvas rendering (can be replaced with React Flow or Cytoscape.js)
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw edges
    edges.forEach(edge => {
      const sourceNode = nodes.find(n => n.id === edge.source);
      const targetNode = nodes.find(n => n.id === edge.target);
      
      if (!sourceNode || !targetNode) return;

      const x1 = sourceNode.position?.x || canvas.width / 2;
      const y1 = sourceNode.position?.y || canvas.height / 2;
      const x2 = targetNode.position?.x || canvas.width / 2;
      const y2 = targetNode.position?.y || canvas.height / 2;

      ctx.strokeStyle = edge.type === 'websocket' ? '#3b82f6' : '#6b7280';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);
      ctx.stroke();
    });

    // Draw nodes
    nodes.forEach(node => {
      const x = node.position?.x || canvas.width / 2;
      const y = node.position?.y || canvas.height / 2;

      // Node color based on status
      const colors = {
        connected: '#10b981',
        disconnected: '#6b7280',
        error: '#ef4444'
      };

      ctx.fillStyle = colors[node.status] || colors.disconnected;
      ctx.beginPath();
      ctx.arc(x, y, 8, 0, Math.PI * 2);
      ctx.fill();

      // Node label
      ctx.fillStyle = '#e5e7eb';
      ctx.font = '12px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(node.label, x, y + 20);
    });
  }, [nodes, edges]);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'connected': return 'bg-green-500/20 text-green-400 border-green-500/50';
      case 'connecting': return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/50';
      case 'disconnected': return 'bg-gray-500/20 text-gray-400 border-gray-500/50';
      default: return 'bg-red-500/20 text-red-400 border-red-500/50';
    }
  };

  return (
    <Card className={`p-4 bg-gray-900 border-gray-800 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Network className="h-5 w-5 text-gray-400" />
          <h3 className="text-sm font-mono font-bold uppercase tracking-wide text-gray-200">
            Network Topology
          </h3>
        </div>
        <Badge className={`text-xs font-mono ${getStatusColor(connectionStatus)}`}>
          {connectionStatus === 'connected' && <CheckCircle2 className="h-3 w-3 mr-1" />}
          {connectionStatus === 'connecting' && <AlertCircle className="h-3 w-3 mr-1" />}
          {connectionStatus.toUpperCase()}
        </Badge>
      </div>

      {error && (
        <div className="mb-4 p-2 bg-red-500/10 border border-red-500/50 rounded text-xs text-red-400 font-mono">
          {error}
        </div>
      )}

      <div className="relative bg-gray-950 rounded border border-gray-800" style={{ height: '400px' }}>
        <canvas
          ref={canvasRef}
          className="w-full h-full"
          style={{ cursor: 'pointer' }}
          onClick={(e) => {
            // Simple click detection (can be improved)
            const rect = canvasRef.current?.getBoundingClientRect();
            if (rect && onNodeClick) {
              const x = e.clientX - rect.left;
              const y = e.clientY - rect.top;
              // Find closest node
              const clickedNode = nodes.find(n => {
                const nodeX = n.position?.x || 0;
                const nodeY = n.position?.y || 0;
                const distance = Math.sqrt(Math.pow(x - nodeX, 2) + Math.pow(y - nodeY, 2));
                return distance < 20;
              });
              if (clickedNode) {
                onNodeClick(clickedNode.id);
              }
            }
          }}
        />
        
        {nodes.length === 0 && (
          <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-xs font-mono">
            {connectionStatus === 'connecting' 
              ? 'Connecting to SX9 Gateway...' 
              : 'No network nodes available'}
          </div>
        )}
      </div>

      <div className="mt-4 flex flex-wrap gap-2 text-xs text-gray-500 font-mono">
        <span>Nodes: {nodes.length}</span>
        <span>•</span>
        <span>Edges: {edges.length}</span>
        <span>•</span>
        <span>Gateway: ws://localhost:18600</span>
      </div>
    </Card>
  );
};

export default NetworkView;



