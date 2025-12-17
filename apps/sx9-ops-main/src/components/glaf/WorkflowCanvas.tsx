import { useEffect, useRef } from 'react';
import cytoscape from 'cytoscape';
import dagre from 'cytoscape-dagre';
import { ZoomIn, ZoomOut, Maximize, LayoutGrid } from 'lucide-react';
import { Workflow, CytoscapeNode, CytoscapeEdge } from '../types/workflow.types';
import { NODE_CATEGORIES } from '../lib/workflow/nodeTypes';

cytoscape.use(dagre);

interface WorkflowCanvasProps {
  workflow: Workflow | null;
  selectedNodeId: string | null;
  onNodeSelect: (nodeId: string | null) => void;
  onNodePositionChange: (nodeId: string, x: number, y: number) => void;
  executionStatus?: Record<string, import('../types/workflow.types').ExecutionStatus | import('../types/workflow.types').LogStatus>;
}

export default function WorkflowCanvas({
  workflow,
  selectedNodeId,
  onNodeSelect,
  onNodePositionChange,
  executionStatus = {}
}: WorkflowCanvasProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const cyRef = useRef<cytoscape.Core | null>(null);
  // ready state not used

  useEffect(() => {
    if (!containerRef.current || !workflow) return;

    const elements: (CytoscapeNode | CytoscapeEdge)[] = [
      ...workflow.definition.nodes.map(node => ({
        data: {
          id: node.id,
          label: node.label,
          type: node.node_type,
          category: node.category,
          config: node.node_config,
          status: executionStatus[node.id]
        },
        position: {
          x: node.position_x || 0,
          y: node.position_y || 0
        },
        classes: `node-${node.category} ${executionStatus[node.id] || ''}`
      })),
      ...workflow.definition.edges.map(edge => ({
        data: {
          id: edge.id,
          source: edge.source_node_id,
          target: edge.target_node_id,
          label: edge.label,
          sourcePort: edge.source_port,
          targetPort: edge.target_port,
          isConditional: edge.is_conditional
        },
        classes: edge.is_conditional ? 'conditional' : ''
      }))
    ];

    const cy = cytoscape({
      container: containerRef.current,
      elements: elements as any,
      style: [
        {
          selector: 'node',
          style: {
            'background-color': '#1a2332',
            'border-width': 2,
            'border-color': '#2a3447',
            'label': 'data(label)',
            'text-valign': 'center',
            'text-halign': 'center',
            'color': '#e5e7eb',
            'font-size': '12px',
            'width': '120px',
            'height': '60px',
            'shape': 'roundrectangle',
            'text-wrap': 'wrap',
            'text-max-width': '100px'
          }
        },
        {
          selector: 'node.node-trigger',
          style: {
            'background-color': NODE_CATEGORIES.trigger.color,
            'border-color': NODE_CATEGORIES.trigger.color
          }
        },
        {
          selector: 'node.node-data_source',
          style: {
            'background-color': NODE_CATEGORIES.data_source.color,
            'border-color': NODE_CATEGORIES.data_source.color
          }
        },
        {
          selector: 'node.node-transformation',
          style: {
            'background-color': NODE_CATEGORIES.transformation.color,
            'border-color': NODE_CATEGORIES.transformation.color
          }
        },
        {
          selector: 'node.node-ai_ml',
          style: {
            'background-color': NODE_CATEGORIES.ai_ml.color,
            'border-color': NODE_CATEGORIES.ai_ml.color
          }
        },
        {
          selector: 'node.node-action',
          style: {
            'background-color': NODE_CATEGORIES.action.color,
            'border-color': NODE_CATEGORIES.action.color
          }
        },
        {
          selector: 'node.node-control_flow',
          style: {
            'background-color': NODE_CATEGORIES.control_flow.color,
            'border-color': NODE_CATEGORIES.control_flow.color
          }
        },
        {
          selector: 'node.node-output',
          style: {
            'background-color': NODE_CATEGORIES.output.color,
            'border-color': NODE_CATEGORIES.output.color
          }
        },
        {
          selector: 'node:selected',
          style: {
            'border-width': 3,
            'border-color': '#3b82f6',
            'overlay-color': '#3b82f6',
            'overlay-opacity': 0.2,
            'overlay-padding': 8
          }
        },
        {
          selector: 'node.running',
          style: {
            'border-color': '#3b82f6',
            'border-width': 3
          }
        },
        {
          selector: 'node.completed',
          style: {
            'border-color': '#10b981',
            'border-width': 3
          }
        },
        {
          selector: 'node.failed',
          style: {
            'border-color': '#ef4444',
            'border-width': 3
          }
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': '#2a3447',
            'target-arrow-color': '#2a3447',
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier',
            'label': 'data(label)',
            'font-size': '10px',
            'color': '#9ca3af',
            'text-background-color': '#0f1419',
            'text-background-opacity': 0.8,
            'text-background-padding': '2px'
          }
        },
        {
          selector: 'edge.conditional',
          style: {
            'line-style': 'dashed',
            'line-color': '#06b6d4'
          }
        },
        {
          selector: 'edge:selected',
          style: {
            'line-color': '#3b82f6',
            'target-arrow-color': '#3b82f6',
            'width': 3
          }
        }
      ],
      layout: {
        name: 'preset'
      },
      minZoom: 0.2,
      maxZoom: 3,
      wheelSensitivity: 0.2
    });

    cy.on('tap', 'node', (evt) => {
      const node = evt.target;
      onNodeSelect(node.id());
    });

    cy.on('tap', (evt) => {
      if (evt.target === cy) {
        onNodeSelect(null);
      }
    });

    cy.on('dragfree', 'node', (evt) => {
      const node = evt.target;
      const pos = node.position();
      onNodePositionChange(node.id(), Math.round(pos.x), Math.round(pos.y));
    });

    cyRef.current = cy;

    return () => {
      cy.destroy();
    };
  }, [workflow?.id]);

  useEffect(() => {
    if (!cyRef.current) return;

    cyRef.current.nodes().removeClass('selected');
    if (selectedNodeId) {
      const node = cyRef.current.getElementById(selectedNodeId);
      node.addClass('selected');
      cyRef.current.center(node);
    }
  }, [selectedNodeId]);

  useEffect(() => {
    if (!cyRef.current) return;

    Object.entries(executionStatus).forEach(([nodeId, status]) => {
      const node = cyRef.current!.getElementById(nodeId);
      node.removeClass('running completed failed');
      node.addClass(status);
    });
  }, [executionStatus]);

  const handleZoomIn = () => {
    if (cyRef.current) {
      cyRef.current.zoom(cyRef.current.zoom() * 1.2);
      cyRef.current.center();
    }
  };

  const handleZoomOut = () => {
    if (cyRef.current) {
      cyRef.current.zoom(cyRef.current.zoom() / 1.2);
      cyRef.current.center();
    }
  };

  const handleFit = () => {
    if (cyRef.current) {
      cyRef.current.fit(undefined, 50);
    }
  };

  const handleLayout = () => {
    if (cyRef.current) {
      cyRef.current.layout({
        name: 'dagre',
        rankDir: 'TB',
        nodeSep: 50,
        rankSep: 100,
        padding: 20
      } as any).run();

      setTimeout(() => {
        cyRef.current?.nodes().forEach(node => {
          const pos = node.position();
          onNodePositionChange(node.id(), Math.round(pos.x), Math.round(pos.y));
        });
      }, 100);
    }
  };

  if (!workflow) {
    return (
      <div className="flex-1 flex items-center justify-center bg-dark-bg text-dark-text-secondary">
        Select or create a workflow to begin
      </div>
    );
  }

  return (
    <div className="flex-1 relative bg-dark-bg">
      <div ref={containerRef} className="absolute inset-0" />

      <div className="absolute top-4 right-4 flex flex-col gap-2">
        <button
          onClick={handleZoomIn}
          className="p-2 bg-dark-surface border border-dark-border rounded hover:bg-dark-elevated transition-colors"
          title="Zoom In"
        >
          <ZoomIn size={18} className="text-dark-text-primary" />
        </button>
        <button
          onClick={handleZoomOut}
          className="p-2 bg-dark-surface border border-dark-border rounded hover:bg-dark-elevated transition-colors"
          title="Zoom Out"
        >
          <ZoomOut size={18} className="text-dark-text-primary" />
        </button>
        <button
          onClick={handleFit}
          className="p-2 bg-dark-surface border border-dark-border rounded hover:bg-dark-elevated transition-colors"
          title="Fit to Screen"
        >
          <Maximize size={18} className="text-dark-text-primary" />
        </button>
        <button
          onClick={handleLayout}
          className="p-2 bg-dark-surface border border-dark-border rounded hover:bg-dark-elevated transition-colors"
          title="Auto Layout"
        >
          <LayoutGrid size={18} className="text-dark-text-primary" />
        </button>
      </div>

      {workflow.definition.nodes.length === 0 && (
        <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
          <div className="text-center text-dark-text-secondary">
            <p className="text-lg mb-2">Empty workflow</p>
            <p className="text-sm">Drag nodes from the library to get started</p>
          </div>
        </div>
      )}
    </div>
  );
}
