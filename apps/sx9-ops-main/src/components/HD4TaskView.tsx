import React, { useState, useEffect } from 'react';
import { ChevronDown, ChevronRight, GripHorizontal, CircleDot, Terminal, Play, Eye, Layers, RefreshCw } from 'lucide-react';
import { supabase } from '../utils/supabaseClient';

interface HD4TaskViewProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  showCLI?: boolean;
  onToggleCLI?: () => void;
}

interface Task {
  task_id: string;
  task_name: string;
  description: string;
  category: string;
  hd4_phase: string;
  kali_tools: string[];
  primitives: string[];
}

const HD4TaskView: React.FC<HD4TaskViewProps> = ({ hd4Action, showCLI: externalShowCLI, onToggleCLI }) => {
  const [expandedCategories, setExpandedCategories] = useState<string[]>([]);
  const [internalShowCLI, setInternalShowCLI] = useState(false);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Use external CLI state if provided, otherwise use internal state
  const showCLI = externalShowCLI !== undefined ? externalShowCLI : internalShowCLI;
  const handleToggleCLI = onToggleCLI || (() => setInternalShowCLI(!internalShowCLI));

  useEffect(() => {
    loadTasksFromSupabase();
  }, [hd4Action]);

  const loadTasksFromSupabase = async () => {
    try {
      setLoading(true);
      
      // Try multiple phase name formats to match database
      const phaseVariations = [
        hd4Action,
        `${hd4Action} Phase`,
        hd4Action.toLowerCase(),
        `${hd4Action.toLowerCase()} phase`,
      ];
      
      // Try each variation
      let data: Task[] = [];
      let lastError = null;
      
      for (const phaseName of phaseVariations) {
        const { data: phaseData, error: fetchError } = await supabase
          .from('ctas_tasks')
          .select('*')
          .eq('hd4_phase', phaseName)
          .order('task_id', { ascending: true });
        
        if (!fetchError && phaseData && phaseData.length > 0) {
          data = phaseData;
          console.log(`✅ HD4TaskView: Loaded ${data.length} tasks for ${hd4Action} phase (matched: ${phaseName})`);
          break;
        }
        lastError = fetchError;
      }
      
      // If no matches, try case-insensitive search
      if (data.length === 0) {
        const { data: allTasks, error: allError } = await supabase
          .from('ctas_tasks')
          .select('*')
          .order('task_id', { ascending: true });
        
        if (!allError && allTasks) {
          // Filter client-side for case-insensitive match
          data = allTasks.filter(task => 
            task.hd4_phase?.toLowerCase().includes(hd4Action.toLowerCase())
          );
          console.log(`✅ HD4TaskView: Loaded ${data.length} tasks for ${hd4Action} phase (client-side filter)`);
        }
      }

      if (data.length === 0 && lastError) {
        throw lastError;
      }

      setTasks(data);
      setError(null);
    } catch (err) {
      console.error('❌ HD4TaskView: Error loading tasks from Supabase:', err);
      setError(err instanceof Error ? err.message : 'Failed to load tasks');
      setTasks([]);
    } finally {
      setLoading(false);
    }
  };

  const toggleCategory = (category: string) => {
    setExpandedCategories(prev =>
      prev.includes(category)
        ? prev.filter(c => c !== category)
        : [...prev, category]
    );
  };

  // Group tasks by category
  const tasksByCategory = tasks.reduce((acc, task) => {
    if (!acc[task.category]) {
      acc[task.category] = [];
    }
    acc[task.category].push(task);
    return acc;
  }, {} as Record<string, Task[]>);

  const handleExecuteTask = (taskId: string) => {
    console.log(`🚀 Executing task: ${taskId}`);
    // TODO: Integrate with Plasma execution engine
  };

  const handleViewEscalationLadder = (taskId: string) => {
    console.log(`🪜 Viewing escalation ladder for task: ${taskId}`);
    // TODO: Show escalation ladder (Script → Microkernel → WASM → Binary → Container → VM)
  };

  return (
    <div className="bg-gray-800 rounded-lg shadow-lg h-full flex flex-col overflow-hidden">
      <div className={`p-4 flex flex-col ${showCLI ? 'flex-1 overflow-y-auto' : 'h-full'}`}>
        <div className="flex justify-between items-center mb-4 flex-shrink-0">
          <div className="flex items-center gap-2">
            <GripHorizontal size={12} className="text-gray-500" />
            <CircleDot size={12} className="text-blue-500" />
            <h2 className="text-xs font-semibold">{hd4Action} Tasks</h2>
            {loading && <span className="text-xxs text-gray-500">(Loading...)</span>}
            {!loading && <span className="text-xxs text-gray-500">({tasks.length} tasks)</span>}
          </div>
          <div className="flex gap-2">
            <button
              onClick={loadTasksFromSupabase}
              className="flex items-center text-xs bg-gray-700 px-2 py-1 rounded hover:bg-gray-600"
              title="Refresh tasks from Supabase"
            >
              <RefreshCw size={10} className="mr-1" />
              Refresh
            </button>
            <button
              onClick={handleToggleCLI}
              className={`flex items-center text-xs px-2 py-1 rounded transition-colors ${
                showCLI ? 'bg-blue-600 text-white' : 'bg-gray-700 hover:bg-gray-600'
              }`}
            >
              <Terminal size={10} className="mr-1" />
              {showCLI ? 'Hide CLI' : 'Show CLI'}
            </button>
          </div>
        </div>


        {error && (
          <div className="mb-4 bg-red-900/20 border border-red-700 rounded-lg p-3 text-xs text-red-400 flex-shrink-0">
            ❌ Error loading tasks: {error}
          </div>
        )}

        <div className="space-y-2 overflow-y-auto flex-1">
        {loading ? (
          <div className="flex items-center justify-center py-12">
            <div className="text-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto mb-2"></div>
              <p className="text-xs text-gray-400">Loading tasks from Supabase...</p>
            </div>
          </div>
        ) : tasks.length === 0 ? (
          <div className="text-center py-12">
            <CircleDot size={32} className="mx-auto mb-2 text-gray-600" />
            <p className="text-xs text-gray-400">No tasks found for {hd4Action} phase</p>
          </div>
        ) : (
          Object.entries(tasksByCategory).map(([category, categoryTasks]) => (
            <div key={category} className="border border-gray-700 rounded-lg">
              <div
                className="flex justify-between items-center p-2 cursor-pointer hover:bg-gray-700"
                onClick={() => toggleCategory(category)}
              >
                <div className="flex items-center">
                  <CircleDot size={10} className="text-blue-500 mr-2" />
                  <span className="text-xs font-medium">{category}</span>
                  <span className="ml-2 text-xxs text-gray-500">({categoryTasks.length})</span>
                </div>
                {expandedCategories.includes(category) ? (
                  <ChevronDown size={12} />
                ) : (
                  <ChevronRight size={12} />
                )}
              </div>
              {expandedCategories.includes(category) && (
                <div className="border-t border-gray-700 p-2 space-y-2">
                  {categoryTasks.map(task => (
                    <div key={task.task_id} className="bg-gray-700 p-2 rounded text-xs">
                      <div className="flex justify-between items-center mb-1">
                        <span className="font-medium">{task.task_name}</span>
                        <span className="text-blue-400">ID: {task.task_id}</span>
                      </div>
                      <p className="text-gray-400 mb-2">{task.description}</p>
                      
                      {/* Kali Tools */}
                      {task.kali_tools && task.kali_tools.length > 0 && (
                        <div className="mb-2">
                          <span className="text-xxs text-gray-500">Kali Tools: </span>
                          <div className="flex flex-wrap gap-1 mt-1">
                            {task.kali_tools.map((tool, idx) => (
                              <span key={idx} className="px-1.5 py-0.5 bg-green-900 text-green-300 rounded text-xxs">
                                {tool}
                              </span>
                            ))}
                          </div>
                        </div>
                      )}

                      {/* Primitives */}
                      {task.primitives && task.primitives.length > 0 && (
                        <div className="mb-2">
                          <span className="text-xxs text-gray-500">Primitives: </span>
                          <div className="flex flex-wrap gap-1 mt-1">
                            {task.primitives.map((primitive, idx) => (
                              <span key={idx} className="px-1.5 py-0.5 bg-purple-900 text-purple-300 rounded text-xxs">
                                {primitive}
                              </span>
                            ))}
                          </div>
                        </div>
                      )}

                      {/* Action Buttons */}
                      <div className="flex gap-2 mt-2">
                        <button
                          onClick={() => handleExecuteTask(task.task_id)}
                          className="flex items-center gap-1 px-2 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded text-xxs"
                        >
                          <Play size={10} />
                          Execute
                        </button>
                        <button
                          onClick={() => handleViewEscalationLadder(task.task_id)}
                          className="flex items-center gap-1 px-2 py-1 bg-purple-600 hover:bg-purple-700 text-white rounded text-xxs"
                        >
                          <Layers size={10} />
                          Escalation Ladder
                        </button>
                        <button
                          className="flex items-center gap-1 px-2 py-1 bg-gray-600 hover:bg-gray-500 text-white rounded text-xxs"
                        >
                          <Eye size={10} />
                          Details
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          ))
        )}
        </div>
      </div>
    </div>
  );
};

export default HD4TaskView;