import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';

interface GraphNode {
  id: string;
  name: string;
  type: 'task' | 'tool' | 'primitive' | 'phase' | 'category';
  group: string;
  usim_hash?: string;
  unicode?: string;
}

interface GraphLink {
  source: string;
  target: string;
  type: 'uses_tool' | 'has_primitive' | 'in_phase' | 'in_category';
}

interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

const GraphVisualization: React.FC = () => {
  const svgRef = useRef<SVGSVGElement>(null);
  const [graphData, setGraphData] = useState<GraphData | null>(null);
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [filter, setFilter] = useState<string>('all');

  useEffect(() => {
    // Fetch real tasks from Kali Tools API (serves 164 tasks from Supabase)
    fetch('http://localhost:18451/tasks')
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        const tasks = data.tasks || [];
        console.log(`✅ GraphVisualization: Loaded ${tasks.length} real tasks from Supabase`);
        buildGraphData(tasks);
      })
      .catch(err => {
        console.error('❌ GraphVisualization: Failed to fetch tasks:', err);
        setGraphData({ nodes: [], links: [] });
      });
  }, []);

  const buildGraphData = (tasks: any[]) => {
    const nodes: GraphNode[] = [];
    const links: GraphLink[] = [];
    const nodeIds = new Set<string>();

    // Add task nodes
    tasks.forEach(task => {
      const taskId = `task_${task.task_id}`;
      if (!nodeIds.has(taskId)) {
        nodes.push({
          id: taskId,
          name: task.task_name,
          type: 'task',
          group: task.hd4_phase || 'Unknown',
          usim_hash: task.usim_hash,
          unicode: task.unicode_compressed
        });
        nodeIds.add(taskId);
      }

      // Add phase node
      const phaseId = `phase_${task.hd4_phase}`;
      if (!nodeIds.has(phaseId)) {
        nodes.push({
          id: phaseId,
          name: task.hd4_phase || 'Unknown',
          type: 'phase',
          group: task.hd4_phase || 'Unknown'
        });
        nodeIds.add(phaseId);
      }
      links.push({ source: taskId, target: phaseId, type: 'in_phase' });

      // Add category node
      const categoryId = `category_${task.category}`;
      if (!nodeIds.has(categoryId)) {
        nodes.push({
          id: categoryId,
          name: task.category || 'Unknown',
          type: 'category',
          group: task.hd4_phase || 'Unknown'
        });
        nodeIds.add(categoryId);
      }
      links.push({ source: taskId, target: categoryId, type: 'in_category' });

      // Add tool nodes
      if (task.kali_tools && Array.isArray(task.kali_tools)) {
        task.kali_tools.forEach((tool: string) => {
          const toolId = `tool_${tool}`;
          if (!nodeIds.has(toolId)) {
            nodes.push({
              id: toolId,
              name: tool,
              type: 'tool',
              group: 'Tools'
            });
            nodeIds.add(toolId);
          }
          links.push({ source: taskId, target: toolId, type: 'uses_tool' });
        });
      }

      // Add primitive nodes
      if (task.primitives && Array.isArray(task.primitives)) {
        task.primitives.forEach((primitive: string) => {
          const primId = `primitive_${primitive}`;
          if (!nodeIds.has(primId)) {
            nodes.push({
              id: primId,
              name: primitive,
              type: 'primitive',
              group: 'Primitives'
            });
            nodeIds.add(primId);
          }
          links.push({ source: taskId, target: primId, type: 'has_primitive' });
        });
      }
    });

    setGraphData({ nodes, links });
  };

  useEffect(() => {
    if (!graphData || !svgRef.current) return;

    const width = svgRef.current.clientWidth;
    const height = svgRef.current.clientHeight;

    // Clear previous graph
    d3.select(svgRef.current).selectAll('*').remove();

    const svg = d3.select(svgRef.current)
      .attr('width', width)
      .attr('height', height);

    // Add zoom behavior
    const g = svg.append('g');
    
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        g.attr('transform', event.transform);
      });

    svg.call(zoom);

    // Color scale for node types
    const colorScale = d3.scaleOrdinal<string>()
      .domain(['task', 'tool', 'primitive', 'phase', 'category'])
      .range(['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6']);

    // Create force simulation
    const simulation = d3.forceSimulation(graphData.nodes as any)
      .force('link', d3.forceLink(graphData.links)
        .id((d: any) => d.id)
        .distance(100))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(30));

    // Create links
    const link = g.append('g')
      .selectAll('line')
      .data(graphData.links)
      .enter()
      .append('line')
      .attr('stroke', '#64748b')
      .attr('stroke-opacity', 0.6)
      .attr('stroke-width', 2);

    // Create nodes
    const node = g.append('g')
      .selectAll('g')
      .data(graphData.nodes)
      .enter()
      .append('g')
      .call(d3.drag<any, any>()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));

    // Add circles to nodes
    node.append('circle')
      .attr('r', (d: any) => d.type === 'phase' ? 25 : d.type === 'task' ? 20 : 15)
      .attr('fill', (d: any) => colorScale(d.type))
      .attr('stroke', '#1e293b')
      .attr('stroke-width', 2)
      .style('cursor', 'pointer')
      .on('click', (event, d: any) => {
        setSelectedNode(d);
      });

    // Add labels to nodes
    node.append('text')
      .text((d: any) => d.unicode || d.name.substring(0, 10))
      .attr('x', 0)
      .attr('y', 30)
      .attr('text-anchor', 'middle')
      .attr('fill', '#e2e8f0')
      .attr('font-size', '10px')
      .style('pointer-events', 'none');

    // Update positions on simulation tick
    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => d.source.x)
        .attr('y1', (d: any) => d.source.y)
        .attr('x2', (d: any) => d.target.x)
        .attr('y2', (d: any) => d.target.y);

      node.attr('transform', (d: any) => `translate(${d.x},${d.y})`);
    });

    function dragstarted(event: any) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      event.subject.fx = event.subject.x;
      event.subject.fy = event.subject.y;
    }

    function dragged(event: any) {
      event.subject.fx = event.x;
      event.subject.fy = event.y;
    }

    function dragended(event: any) {
      if (!event.active) simulation.alphaTarget(0);
      event.subject.fx = null;
      event.subject.fy = null;
    }

  }, [graphData]);

  return (
    <div className="w-full h-full bg-gray-900 relative">
      {/* Controls */}
      <div className="absolute top-4 left-4 z-10 bg-gray-800 p-4 rounded-lg shadow-lg">
        <h3 className="text-white font-bold mb-2">CTAS Task Graph</h3>
        <div className="flex flex-col gap-2">
          <button
            onClick={() => setFilter('all')}
            className={`px-3 py-1 rounded ${filter === 'all' ? 'bg-blue-600' : 'bg-gray-700'} text-white text-sm`}
          >
            All Tasks
          </button>
          <button
            onClick={() => setFilter('Hunt')}
            className={`px-3 py-1 rounded ${filter === 'Hunt' ? 'bg-red-600' : 'bg-gray-700'} text-white text-sm`}
          >
            Hunt Phase
          </button>
          <button
            onClick={() => setFilter('Detect')}
            className={`px-3 py-1 rounded ${filter === 'Detect' ? 'bg-yellow-600' : 'bg-gray-700'} text-white text-sm`}
          >
            Detect Phase
          </button>
        </div>
      </div>

      {/* Legend */}
      <div className="absolute top-4 right-4 z-10 bg-gray-800 p-4 rounded-lg shadow-lg">
        <h3 className="text-white font-bold mb-2">Legend</h3>
        <div className="flex flex-col gap-2 text-sm">
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-blue-500"></div>
            <span className="text-gray-300">Tasks</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-green-500"></div>
            <span className="text-gray-300">Tools</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-yellow-500"></div>
            <span className="text-gray-300">Primitives</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-red-500"></div>
            <span className="text-gray-300">HD4 Phases</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded-full bg-purple-500"></div>
            <span className="text-gray-300">Categories</span>
          </div>
        </div>
      </div>

      {/* Node Details Panel */}
      {selectedNode && (
        <div className="absolute bottom-4 left-4 z-10 bg-gray-800 p-4 rounded-lg shadow-lg max-w-md">
          <div className="flex justify-between items-start mb-2">
            <h3 className="text-white font-bold">{selectedNode.name}</h3>
            <button
              onClick={() => setSelectedNode(null)}
              className="text-gray-400 hover:text-white"
            >
              ✕
            </button>
          </div>
          <div className="text-sm text-gray-300 space-y-1">
            <p><span className="font-semibold">Type:</span> {selectedNode.type}</p>
            <p><span className="font-semibold">Group:</span> {selectedNode.group}</p>
            {selectedNode.usim_hash && (
              <p><span className="font-semibold">USIM:</span> {selectedNode.usim_hash.substring(0, 16)}...</p>
            )}
            {selectedNode.unicode && (
              <p><span className="font-semibold">Unicode:</span> {selectedNode.unicode}</p>
            )}
          </div>
        </div>
      )}

      {/* Graph SVG */}
      <svg ref={svgRef} className="w-full h-full"></svg>
    </div>
  );
};

export default GraphVisualization;

