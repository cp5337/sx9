import React, { useState, useEffect } from "react";
import {
  ChevronDown,
  ChevronRight,
  Target,
  AlertTriangle,
  Shield,
  Zap,
  Globe,
  Database,
} from "lucide-react";
import { supabase } from "../utils/supabaseClient";
import { CTAS7_API_ENDPOINTS } from "@/services/ctas7-api-integration";

interface Task {
  id: string;
  task_id: string;
  task_seq: number;
  task_name: string;
  description: string;
  category: string;
  hd4_phase: string;
  primitive_type: string;
  predecessors: string[];
  successors: string[];
  probability: number;
  threat_level: number;
  detection_risk: number;
  hash_id: string;
  trivariate_hash?: string | null;
  semantic_hash?: string | null;
  unicode_address?: string | null;
  escalation_unicode?: string | null;
  interview_version?: string | null;
  // HD4 & Node Interview Attributes
  ptcc_primitives?: any[] | null;
  ptcc_tool_chain?: any[] | null;
  mitre_attack_techniques?: any[] | null;
  kali_tools_required?: any[] | null;

  // Operational Metadata
  linear_issue_key?: string | null;
  status?: string;
  priority?: string;
  operation_type?: string;
  assigned_to?: string;
}

interface PrimitiveDisplay {
  name: string;
  unicode?: string;
}

const Tasks: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedCategories, setExpandedCategories] = useState<string[]>([]);
  const [expandedPhases, setExpandedPhases] = useState<string[]>(["Hunt Phase"]);

  useEffect(() => {
    loadTasksFromSupabase();
  }, []);

  const loadTasksFromSupabase = async () => {
    try {
      setLoading(true);

      // Try Supabase client first
      const { data, error: fetchError } = await supabase
        .from("ctas_tasks")
        .select("*")
        .order("task_id", { ascending: true });

      if (fetchError) {
        // Fallback: Direct PostgREST API call
        console.warn("Supabase client failed, trying direct PostgREST:", fetchError);
        const postgrestUrl = import.meta.env.VITE_SUPABASE_URL || CTAS7_API_ENDPOINTS.supabase;
        const response = await fetch(`${postgrestUrl}/ctas_tasks?order=task_id.asc`, {
          headers: {
            Accept: "application/json",
            apikey: import.meta.env.VITE_SUPABASE_ANON_KEY || "",
          },
        });

        if (!response.ok) {
          throw new Error(`PostgREST error: ${response.status} ${response.statusText}`);
        }

        const directData = await response.json();
        setTasks(directData || []);
        setError(null);
        return;
      }

      setTasks(data || []);
      setError(null);
    } catch (err) {
      console.error("Error loading tasks from Supabase:", err);
      setError(err instanceof Error ? err.message : "Failed to load tasks");
    } finally {
      setLoading(false);
    }
  };

  const togglePhase = (phase: string) => {
    setExpandedPhases(prev =>
      prev.includes(phase) ? prev.filter(p => p !== phase) : [...prev, phase]
    );
  };

  const toggleCategory = (category: string) => {
    setExpandedCategories(prev =>
      prev.includes(category) ? prev.filter(c => c !== category) : [...prev, category]
    );
  };

  const getHD4Icon = (phase: string) => {
    switch (phase) {
      case "Hunt":
        return <Target className="w-4 h-4 text-blue-500" />;
      case "Detect":
        return <Shield className="w-4 h-4 text-green-500" />;
      case "Disable":
        return <Zap className="w-4 h-4 text-yellow-500" />;
      case "Disrupt":
        return <AlertTriangle className="w-4 h-4 text-red-500" />;
      case "Dominate":
        return <Globe className="w-4 h-4 text-purple-500" />;
      default:
        return null;
    }
  };

  const getPhaseColor = (phase: string) => {
    switch (phase) {
      case "Hunt":
        return "bg-blue-900/30 border-blue-500";
      case "Detect":
        return "bg-green-900/30 border-green-500";
      case "Disable":
        return "bg-yellow-900/30 border-yellow-500";
      case "Disrupt":
        return "bg-red-900/30 border-red-500";
      case "Dominate":
        return "bg-purple-900/30 border-purple-500";
      default:
        return "bg-gray-900/30 border-gray-500";
    }
  };

  const getPrimitiveColor = (primitive: string) => {
    switch (primitive) {
      case "Concept":
        return "bg-cyan-600 text-white";
      case "Actor":
        return "bg-orange-600 text-white";
      case "Object":
        return "bg-green-600 text-white";
      case "Event":
        return "bg-pink-600 text-white";
      case "Attribute":
        return "bg-indigo-600 text-white";
      default:
        return "bg-gray-600 text-white";
    }
  };

  // Group tasks by phase, then by category
  const tasksByPhase = tasks.reduce(
    (acc, task) => {
      const phase = task.hd4_phase || "Unknown";
      if (!acc[phase]) {
        acc[phase] = {};
      }
      const category = task.category || "Uncategorized";
      if (!acc[phase][category]) {
        acc[phase][category] = [];
      }
      acc[phase][category].push(task);
      return acc;
    },
    {} as Record<string, Record<string, Task[]>>
  );

  const phases = ["Hunt", "Detect", "Disrupt", "Disable", "Dominate"];

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-900">
        <div className="text-center">
          <Database className="w-12 h-12 text-blue-500 animate-pulse mx-auto mb-4" />
          <p className="text-gray-300">Loading tasks from Supabase...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-900">
        <div className="text-center">
          <AlertTriangle className="w-12 h-12 text-red-500 mx-auto mb-4" />
          <p className="text-red-400 mb-2">Error loading tasks</p>
          <p className="text-gray-400 text-sm">{error}</p>
          <button
            onClick={loadTasksFromSupabase}
            className="mt-4 px-4 py-2 bg-blue-600 text-white hover:bg-blue-700"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-gray-900 min-h-screen">
      {/* Minimal Title Bar */}
      <div className="flex justify-between items-center border-b border-gray-700 bg-gray-800/50 px-2 py-1">
        <span className="text-xs text-gray-400">{tasks.length} tasks</span>
        <button
          onClick={loadTasksFromSupabase}
          className="flex items-center gap-1 px-1.5 py-0.5 bg-blue-600 text-white text-xs"
        >
          <Database size={11} />
          Refresh
        </button>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm border-collapse bg-gray-800">
          <thead>
            <tr className="bg-gray-700">
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                Category / Task ID
              </th>
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                Task Name
              </th>
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                Description
              </th>
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                Primitive
              </th>
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                Primitives
              </th>
              <th className="p-1.5 border border-gray-600 text-left text-gray-300 text-xs">
                HD4 Phase
              </th>
            </tr>
          </thead>
          <tbody>
            {phases.map(phase => {
              const phaseData = tasksByPhase[phase];
              if (!phaseData) return null;

              const categories = Object.keys(phaseData);
              const isPhaseExpanded = expandedPhases.includes(`${phase} Phase`);

              return (
                <React.Fragment key={phase}>
                  {/* Phase Header */}
                  <tr className="bg-gray-800">
                    <td colSpan={5} className="p-1 border border-gray-600">
                      <button
                        onClick={() => togglePhase(`${phase} Phase`)}
                        className="flex items-center w-full text-left font-bold text-white px-1 py-0.5"
                      >
                        {isPhaseExpanded ? (
                          <ChevronDown size={14} className="mr-1" />
                        ) : (
                          <ChevronRight size={14} className="mr-1" />
                        )}
                        <span>{phase} Phase</span>
                        <span className="ml-2 text-xs text-gray-400">
                          ({categories.reduce((sum, cat) => sum + phaseData[cat].length, 0)} tasks)
                        </span>
                      </button>
                    </td>
                  </tr>

                  {/* Categories within Phase */}
                  {isPhaseExpanded &&
                    categories.map(category => {
                      const categoryTasks = phaseData[category];
                      const isCategoryExpanded = expandedCategories.includes(
                        `${phase}-${category}`
                      );

                      return (
                        <React.Fragment key={`${phase}-${category}`}>
                          {/* Category Header */}
                          <tr className="bg-gray-800">
                            <td colSpan={5} className="p-1 border border-gray-600 pl-6">
                              <button
                                onClick={() => toggleCategory(`${phase}-${category}`)}
                                className="flex items-center w-full text-left font-semibold text-gray-300 px-1 py-0.5"
                              >
                                {isCategoryExpanded ? (
                                  <ChevronDown size={12} className="mr-1" />
                                ) : (
                                  <ChevronRight size={12} className="mr-1" />
                                )}
                                <span>{category}</span>
                                <span className="ml-2 text-xs text-gray-500">
                                  ({categoryTasks.length} tasks)
                                </span>
                              </button>
                            </td>
                          </tr>

                          {/* Tasks within Category */}
                          {isCategoryExpanded &&
                            categoryTasks.map(task => (
                              <tr key={task.id} className="text-gray-300">
                                <td className="p-1 border border-gray-600 pl-10 text-xs font-mono">
                                  {task.task_id}
                                </td>
                                <td className="p-1 border border-gray-600 text-xs">
                                  {task.task_name}
                                  {task.primitive_type}
                                </td>
                                <td className="p-1 border border-gray-600 text-xs text-gray-400">
                                  {task.description}
                                </td>
                                <td className="p-1 border border-gray-600 text-xs font-mono text-cyan-400">
                                  {task.primitive_type}
                                </td>
                                <td className="p-1 border border-gray-600 text-xs text-gray-400">
                                  {task.hd4_phase}
                                </td>
                              </tr>
                            ))}
                        </React.Fragment>
                      );
                    })}
                </React.Fragment>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default Tasks;
