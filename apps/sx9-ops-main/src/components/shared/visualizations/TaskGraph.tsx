import React, { useEffect, useState } from 'react';
import { GitBranch, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { supabase } from '@/utils/supabaseClient';

interface TaskNode {
  id: string;
  hash_id: string;
  task_name: string;
  hd4_phase: string;
  primitive_type: string;
  position?: { x: number; y: number };
  predecessors: string[];
  successors: string[];
}

interface TaskEdge {
  id: string;
  source: string;
  target: string;
  type: 'predecessor' | 'successor';
}

interface TaskGraphProps {
  hd4Phase?: string;
  onTaskClick?: (taskId: string) => void;
  className?: string;
}

export const TaskGraph: React.FC<TaskGraphProps> = ({ 
  hd4Phase, 
  onTaskClick, 
  className = '' 
}) => {
  const [tasks, setTasks] = useState<TaskNode[]>([]);
  const [edges, setEdges] = useState<TaskEdge[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const canvasRef = React.useRef<HTMLCanvasElement>(null);

  // Load tasks from Supabase
  useEffect(() => {
    const loadTasks = async () => {
      try {
        setLoading(true);
        setError(null);

        let query = supabase
          .from('ctas_tasks')
          .select('*')
          .order('task_seq', { ascending: true });

        // Filter by HD4 phase if provided
        if (hd4Phase && hd4Phase !== 'All') {
          query = query.eq('hd4_phase', hd4Phase);
        }

        const { data, error: queryError } = await query;

        if (queryError) {
          throw queryError;
        }

        if (!data || data.length === 0) {
          setTasks([]);
          setEdges([]);
          setLoading(false);
          return;
        }

        // Convert to TaskNode format
        const taskNodes: TaskNode[] = data.map((task: any, index: number) => {
          // Simple layout: arrange in a grid
          const cols = Math.ceil(Math.sqrt(data.length));
          const row = Math.floor(index / cols);
          const col = index % cols;
          
          return {
            id: task.id || task.hash_id,
            hash_id: task.hash_id || task.task_id,
            task_name: task.task_name,
            hd4_phase: task.hd4_phase,
            primitive_type: task.primitive_type,
            position: {
              x: 100 + col * 200,
              y: 100 + row * 150
            },
            predecessors: Array.isArray(task.predecessors) 
              ? task.predecessors 
              : (task.predecessors ? JSON.parse(task.predecessors) : []),
            successors: Array.isArray(task.successors)
              ? task.successors
              : (task.successors ? JSON.parse(task.successors) : [])
          };
        });

        setTasks(taskNodes);

        // Build edges from relationships
        const taskEdges: TaskEdge[] = [];
        taskNodes.forEach(task => {
          // Add predecessor edges
          task.predecessors.forEach((predId: string) => {
            const predTask = taskNodes.find(t => 
              t.hash_id === predId || t.id === predId
            );
            if (predTask) {
              taskEdges.push({
                id: `${predTask.id}-${task.id}`,
                source: predTask.id,
                target: task.id,
                type: 'predecessor'
              });
            }
          });

          // Add successor edges
          task.successors.forEach((succId: string) => {
            const succTask = taskNodes.find(t => 
              t.hash_id === succId || t.id === succId
            );
            if (succTask) {
              taskEdges.push({
                id: `${task.id}-${succTask.id}`,
                source: task.id,
                target: succTask.id,
                type: 'successor'
              });
            }
          });
        });

        setEdges(taskEdges);
        setLoading(false);
      } catch (err) {
        console.error('[TaskGraph] Error loading tasks:', err);
        setError(err instanceof Error ? err.message : 'Failed to load tasks');
        setLoading(false);
      }
    };

    loadTasks();
  }, [hd4Phase]);

  // Render graph on canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || loading) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw edges
    edges.forEach(edge => {
      const sourceTask = tasks.find(t => t.id === edge.source);
      const targetTask = tasks.find(t => t.id === edge.target);
      
      if (!sourceTask || !targetTask || !sourceTask.position || !targetTask.position) return;

      ctx.strokeStyle = edge.type === 'predecessor' ? '#3b82f6' : '#10b981';
      ctx.lineWidth = 1.5;
      ctx.setLineDash(edge.type === 'predecessor' ? [] : [5, 5]);
      ctx.beginPath();
      ctx.moveTo(sourceTask.position.x, sourceTask.position.y);
      ctx.lineTo(targetTask.position.x, targetTask.position.y);
      ctx.stroke();
      ctx.setLineDash([]);
    });

    // Draw nodes
    tasks.forEach(task => {
      if (!task.position) return;

      const x = task.position.x;
      const y = task.position.y;

      // Node color based on HD4 phase
      const phaseColors: Record<string, string> = {
        'Hunt': '#3b82f6',
        'Detect': '#10b981',
        'Disrupt': '#f59e0b',
        'Disable': '#ef4444',
        'Dominate': '#8b5cf6'
      };

      ctx.fillStyle = phaseColors[task.hd4_phase] || '#6b7280';
      ctx.beginPath();
      ctx.arc(x, y, 10, 0, Math.PI * 2);
      ctx.fill();

      // Node border
      ctx.strokeStyle = '#1f2937';
      ctx.lineWidth = 2;
      ctx.stroke();

      // Task name label
      ctx.fillStyle = '#e5e7eb';
      ctx.font = '11px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(
        task.task_name.length > 20 
          ? task.task_name.substring(0, 20) + '...' 
          : task.task_name,
        x,
        y + 25
      );
    });
  }, [tasks, edges, loading]);

  const getPhaseColor = (phase: string) => {
    const colors: Record<string, string> = {
      'Hunt': 'bg-blue-500/20 text-blue-400 border-blue-500/50',
      'Detect': 'bg-green-500/20 text-green-400 border-green-500/50',
      'Disrupt': 'bg-yellow-500/20 text-yellow-400 border-yellow-500/50',
      'Disable': 'bg-red-500/20 text-red-400 border-red-500/50',
      'Dominate': 'bg-purple-500/20 text-purple-400 border-purple-500/50'
    };
    return colors[phase] || 'bg-gray-500/20 text-gray-400 border-gray-500/50';
  };

  return (
    <Card className={`p-4 bg-gray-900 border-gray-800 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <GitBranch className="h-5 w-5 text-gray-400" />
          <h3 className="text-sm font-mono font-bold uppercase tracking-wide text-gray-200">
            Task Dependency Graph
          </h3>
        </div>
        {hd4Phase && (
          <Badge className={`text-xs font-mono ${getPhaseColor(hd4Phase)}`}>
            {hd4Phase}
          </Badge>
        )}
      </div>

      {error && (
        <div className="mb-4 p-2 bg-red-500/10 border border-red-500/50 rounded text-xs text-red-400 font-mono">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center h-96 text-gray-500">
          <Loader2 className="h-6 w-6 animate-spin mr-2" />
          <span className="text-xs font-mono">Loading tasks from Supabase...</span>
        </div>
      ) : (
        <div className="relative bg-gray-950 rounded border border-gray-800" style={{ height: '400px', overflow: 'auto' }}>
          <canvas
            ref={canvasRef}
            className="w-full h-full"
            style={{ cursor: 'pointer' }}
            onClick={(e) => {
              const rect = canvasRef.current?.getBoundingClientRect();
              if (rect && onTaskClick) {
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;
                // Find closest task node
                const clickedTask = tasks.find(t => {
                  if (!t.position) return false;
                  const distance = Math.sqrt(
                    Math.pow(x - t.position.x, 2) + Math.pow(y - t.position.y, 2)
                  );
                  return distance < 15;
                });
                if (clickedTask) {
                  onTaskClick(clickedTask.id);
                }
              }
            }}
          />
          
          {tasks.length === 0 && !loading && (
            <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-xs font-mono">
              No tasks found
            </div>
          )}
        </div>
      )}

      <div className="mt-4 flex flex-wrap gap-2 text-xs text-gray-500 font-mono">
        <span>Tasks: {tasks.length}</span>
        <span>•</span>
        <span>Dependencies: {edges.length}</span>
        {hd4Phase && <><span>•</span><span>Phase: {hd4Phase}</span></>}
      </div>
    </Card>
  );
};

export default TaskGraph;

