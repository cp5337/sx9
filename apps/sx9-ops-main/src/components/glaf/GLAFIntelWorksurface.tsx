import React, { useState, useEffect, useCallback } from "react";
import {
  Zap,
  Database,
  Activity,
  Send,
  CheckCircle,
  XCircle,
  X,
} from "lucide-react";
// MultiPromptSystem and DraggableDivider imports removed

interface GraphNode {
  id: string;
  label: string;
  type: string;
  data: Record<string, any>;
}

interface GLAFTask {
  id: string;
  nodeId: string;
  status: "pending" | "processing" | "completed" | "failed";
  result?: any;
  error?: string;
  timestamp: number;
}

interface ContainerStatus {
  running: boolean;
  taskId?: string;
  uptime?: number;
  metrics?: {
    tasksProcessed: number;
    queriesTotal: number;
    sledisSize: number;
  };
}

export function GLAFIntelWorksurface() {
  const [selectedNodes, setSelectedNodes] = useState<GraphNode[]>([]);
  const [tasks, setTasks] = useState<GLAFTask[]>([]);
  const [containerStatus, setContainerStatus] = useState<ContainerStatus>({
    running: false,
  });
  const [autoIngest, setAutoIngest] = useState(false);

  // Poll container status
  useEffect(() => {
    const checkStatus = async () => {
      try {
        const response = await fetch("http://localhost:3000/healthz");
        if (response.ok) {
          const metrics = await fetch("http://localhost:3000/metrics").then(
            (r) => r.json()
          );
          setContainerStatus({
            running: true,
            taskId: metrics.task_id,
            uptime: metrics.uptime_seconds,
            metrics: {
              tasksProcessed: metrics.tasks_processed,
              queriesTotal: metrics.glaf_queries_total,
              sledisSize: metrics.sledis_size_bytes,
            },
          });
        } else {
          setContainerStatus({ running: false });
        }
      } catch (error) {
        setContainerStatus({ running: false });
      }
    };

    checkStatus();
    const interval = setInterval(checkStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  // Listen for graph node selections from Cytoscape
  useEffect(() => {
    const handleNodeSelect = (event: CustomEvent) => {
      const node = event.detail as GraphNode;
      setSelectedNodes((prev) => {
        const exists = prev.find((n) => n.id === node.id);
        if (exists) return prev;
        return [...prev, node];
      });

      // Auto-ingest if enabled
      if (autoIngest) {
        sendToGLAF(node);
      }
    };

    window.addEventListener(
      "graph:node:selected",
      handleNodeSelect as EventListener
    );
    return () =>
      window.removeEventListener(
        "graph:node:selected",
        handleNodeSelect as EventListener
      );
  }, [autoIngest]);

  const sendToGLAF = useCallback(async (node: GraphNode) => {
    const taskId = `task-${Date.now()}-${node.id}`;

    const newTask: GLAFTask = {
      id: taskId,
      nodeId: node.id,
      status: "pending",
      timestamp: Date.now(),
    };

    setTasks((prev) => [newTask, ...prev]);

    try {
      // Send to NATS (which GLAF-INTEL container subscribes to)
      const response = await fetch("/api/glaf-intel/ingest", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          taskId,
          nodeId: node.id,
          nodeType: node.type,
          data: node.data,
          crystal: {
            precision: 0.8,
            speed: 0.7,
            depth: 0.6,
            noise: 0.5,
          },
        }),
      });

      if (response.ok) {
        setTasks((prev) =>
          prev.map((t) =>
            t.id === taskId ? { ...t, status: "processing" } : t
          )
        );

        // Poll for result
        pollTaskResult(taskId);
      } else {
        throw new Error("Failed to send to GLAF-INTEL");
      }
    } catch (error) {
      setTasks((prev) =>
        prev.map((t) =>
          t.id === taskId
            ? {
                ...t,
                status: "failed",
                error: error instanceof Error ? error.message : "Unknown error",
              }
            : t
        )
      );
    }
  }, []);

  const pollTaskResult = async (taskId: string) => {
    const maxAttempts = 60; // 60 seconds timeout
    let attempts = 0;

    const poll = async () => {
      try {
        const response = await fetch(`/api/glaf-intel/task/${taskId}`);
        if (response.ok) {
          const result = await response.json();

          if (result.status === "completed") {
            setTasks((prev) =>
              prev.map((t) =>
                t.id === taskId
                  ? { ...t, status: "completed", result: result.data }
                  : t
              )
            );
            return;
          } else if (result.status === "failed") {
            setTasks((prev) =>
              prev.map((t) =>
                t.id === taskId
                  ? { ...t, status: "failed", error: result.error }
                  : t
              )
            );
            return;
          }
        }

        attempts++;
        if (attempts < maxAttempts) {
          setTimeout(poll, 1000);
        } else {
          setTasks((prev) =>
            prev.map((t) =>
              t.id === taskId ? { ...t, status: "failed", error: "Timeout" } : t
            )
          );
        }
      } catch (error) {
        console.error("Poll error:", error);
      }
    };

    poll();
  };

  const removeNode = (nodeId: string) => {
    setSelectedNodes((prev) => prev.filter((n) => n.id !== nodeId));
  };

  const clearCompleted = () => {
    setTasks((prev) =>
      prev.filter((t) => t.status === "pending" || t.status === "processing")
    );
  };

  return (
    <div className="flex flex-col h-full bg-gray-50 dark:bg-gray-900">
      {/* Upper: Worksurface Content */}
      <div
        className="flex-1 flex flex-col gap-4 p-4 overflow-y-auto"
        style={{ minHeight: "200px" }}
      >
        {/* Container Status */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-200 dark:border-gray-700">
          <div className="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white">
                <Zap className="w-4 h-4 text-blue-500" />
                <span>GLAF-INTEL Container</span>
              </div>
              <span
                className={`px-2 py-1 text-xs rounded ${
                  containerStatus.running
                    ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                    : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
                }`}
              >
                {containerStatus.running ? "Running" : "Offline"}
              </span>
            </div>
          </div>
          {containerStatus.running && containerStatus.metrics && (
            <div className="p-4">
              <div className="grid grid-cols-3 gap-4 text-xs">
                <div>
                  <div className="text-gray-500 dark:text-gray-400">Tasks</div>
                  <div className="font-mono font-semibold text-gray-900 dark:text-white">
                    {containerStatus.metrics.tasksProcessed}
                  </div>
                </div>
                <div>
                  <div className="text-gray-500 dark:text-gray-400">
                    Queries
                  </div>
                  <div className="font-mono font-semibold text-gray-900 dark:text-white">
                    {containerStatus.metrics.queriesTotal}
                  </div>
                </div>
                <div>
                  <div className="text-gray-500 dark:text-gray-400">
                    Storage
                  </div>
                  <div className="font-mono font-semibold text-gray-900 dark:text-white">
                    {(containerStatus.metrics.sledisSize / 1024 / 1024).toFixed(
                      1
                    )}{" "}
                    MB
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Selected Nodes Queue */}
        <div className="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden">
          <div className="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white">
                <Database className="w-4 h-4 text-purple-500" />
                <span>Graph Data Queue ({selectedNodes.length})</span>
              </div>
              <div className="flex gap-2">
                <button
                  className={`px-3 py-1 text-xs rounded transition-colors ${
                    autoIngest
                      ? "bg-blue-500 text-white"
                      : "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300"
                  }`}
                  onClick={() => setAutoIngest(!autoIngest)}
                >
                  Auto-Ingest {autoIngest ? "ON" : "OFF"}
                </button>
                <button
                  className="px-3 py-1 text-xs rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors disabled:opacity-50"
                  onClick={() => setSelectedNodes([])}
                  disabled={selectedNodes.length === 0}
                >
                  Clear
                </button>
              </div>
            </div>
          </div>
          <div className="flex-1 overflow-y-auto p-4">
            {selectedNodes.length === 0 ? (
              <div className="text-center text-gray-500 dark:text-gray-400 text-sm py-8">
                Select nodes from the graph to queue for GLAF-INTEL processing
              </div>
            ) : (
              <div className="space-y-2">
                {selectedNodes.map((node) => (
                  <div
                    key={node.id}
                    className="flex items-center justify-between p-3 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-750 transition-colors"
                  >
                    <div className="flex-1 min-w-0">
                      <div className="font-medium text-sm truncate text-gray-900 dark:text-white">
                        {node.label}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">
                        {node.type}
                      </div>
                    </div>
                    <div className="flex gap-2">
                      <button
                        className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors disabled:opacity-50"
                        onClick={() => sendToGLAF(node)}
                        disabled={!containerStatus.running}
                        title="Send to GLAF-INTEL"
                      >
                        <Send className="w-3 h-3" />
                      </button>
                      <button
                        className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors"
                        onClick={() => removeNode(node.id)}
                        title="Remove from queue"
                      >
                        <X className="w-3 h-3" />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Task Results */}
        <div className="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden">
          <div className="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white">
                <Activity className="w-4 h-4 text-green-500" />
                <span>Processing Results ({tasks.length})</span>
              </div>
              <button
                className="px-3 py-1 text-xs rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                onClick={clearCompleted}
              >
                Clear Completed
              </button>
            </div>
          </div>
          <div className="flex-1 overflow-y-auto p-4">
            {tasks.length === 0 ? (
              <div className="text-center text-gray-500 dark:text-gray-400 text-sm py-8">
                No tasks yet
              </div>
            ) : (
              <div className="space-y-2">
                {tasks.map((task) => (
                  <div
                    key={task.id}
                    className="p-3 border border-gray-200 dark:border-gray-700 rounded-lg"
                  >
                    <div className="flex items-start justify-between mb-2">
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-sm truncate text-gray-900 dark:text-white">
                          Node: {task.nodeId}
                        </div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">
                          {new Date(task.timestamp).toLocaleTimeString()}
                        </div>
                      </div>
                      <span
                        className={`ml-2 px-2 py-1 text-xs rounded flex items-center gap-1 ${
                          task.status === "completed"
                            ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                            : task.status === "failed"
                            ? "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
                            : task.status === "processing"
                            ? "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200"
                            : "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200"
                        }`}
                      >
                        {task.status === "completed" && (
                          <CheckCircle className="w-3 h-3" />
                        )}
                        {task.status === "failed" && (
                          <XCircle className="w-3 h-3" />
                        )}
                        {task.status}
                      </span>
                    </div>

                    {task.result && (
                      <div className="mt-2 p-2 bg-gray-100 dark:bg-gray-900 rounded text-xs font-mono overflow-x-auto text-gray-900 dark:text-gray-100">
                        {JSON.stringify(task.result, null, 2)}
                      </div>
                    )}

                    {task.error && (
                      <div className="mt-2 p-2 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded text-xs">
                        {task.error}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
