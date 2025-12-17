import { useEffect, useState } from 'react';
import { supabase } from '../lib/supabase';
import { WorkflowExecution, WorkflowExecutionLog } from '../types/workflow.types';
import { CheckCircle, XCircle, Clock, AlertCircle, ChevronDown, ChevronRight, Play } from 'lucide-react';

interface WorkflowExecutionHistoryProps {
  workflowId: string;
}

export default function WorkflowExecutionHistory({ workflowId }: WorkflowExecutionHistoryProps) {
  const [executions, setExecutions] = useState<WorkflowExecution[]>([]);
  const [expandedExecution, setExpandedExecution] = useState<string | null>(null);
  const [executionLogs, setExecutionLogs] = useState<Record<string, WorkflowExecutionLog[]>>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchExecutions();

    const channel = supabase
      .channel('execution-changes')
      .on('postgres_changes', {
        event: '*',
        schema: 'public',
        table: 'workflow_executions',
        filter: `workflow_id=eq.${workflowId}`
      }, () => {
        fetchExecutions();
      })
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, [workflowId]);

  const fetchExecutions = async () => {
    setLoading(true);
    const { data, error } = await supabase
      .from('workflow_executions')
      .select('*')
      .eq('workflow_id', workflowId)
      .order('created_at', { ascending: false })
      .limit(20);

    if (error) {
      console.error('Error fetching executions:', error);
    } else {
      setExecutions(data as WorkflowExecution[]);
    }
    setLoading(false);
  };

  const fetchExecutionLogs = async (executionId: string) => {
    if (executionLogs[executionId]) return;

    const { data, error } = await supabase
      .from('workflow_execution_logs')
      .select('*')
      .eq('execution_id', executionId)
      .order('started_at', { ascending: true });

    if (error) {
      console.error('Error fetching logs:', error);
    } else {
      setExecutionLogs(prev => ({
        ...prev,
        [executionId]: data as WorkflowExecutionLog[]
      }));
    }
  };

  const toggleExecution = (executionId: string) => {
    if (expandedExecution === executionId) {
      setExpandedExecution(null);
    } else {
      setExpandedExecution(executionId);
      fetchExecutionLogs(executionId);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full text-dark-text-secondary">
        Loading execution history...
      </div>
    );
  }

  if (executions.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-full text-center px-4">
        <Play size={48} className="text-dark-text-secondary mb-4 opacity-50" />
        <p className="text-dark-text-secondary mb-2">No executions yet</p>
        <p className="text-xs text-dark-text-secondary">
          Run this workflow to see execution history
        </p>
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="p-4 space-y-2">
        {executions.map(execution => (
          <ExecutionCard
            key={execution.id}
            execution={execution}
            isExpanded={expandedExecution === execution.id}
            logs={executionLogs[execution.id]}
            onToggle={() => toggleExecution(execution.id)}
          />
        ))}
      </div>
    </div>
  );
}

interface ExecutionCardProps {
  execution: WorkflowExecution;
  isExpanded: boolean;
  logs?: WorkflowExecutionLog[];
  onToggle: () => void;
}

function ExecutionCard({ execution, isExpanded, logs, onToggle }: ExecutionCardProps) {
  const getStatusIcon = () => {
    switch (execution.status) {
      case 'completed':
        return <CheckCircle size={16} className="text-green-500" />;
      case 'failed':
        return <XCircle size={16} className="text-red-500" />;
      case 'running':
        return <Clock size={16} className="text-blue-500 animate-pulse" />;
      case 'timeout':
        return <AlertCircle size={16} className="text-yellow-500" />;
      default:
        return <Clock size={16} className="text-gray-500" />;
    }
  };

  const getStatusColor = () => {
    switch (execution.status) {
      case 'completed':
        return 'border-green-500/30 bg-green-500/5';
      case 'failed':
        return 'border-red-500/30 bg-red-500/5';
      case 'running':
        return 'border-blue-500/30 bg-blue-500/5';
      case 'timeout':
        return 'border-yellow-500/30 bg-yellow-500/5';
      default:
        return 'border-dark-border';
    }
  };

  const formatDuration = (ms?: number) => {
    if (!ms) return '--';
    if (ms < 1000) return `${ms}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
    return `${(ms / 60000).toFixed(1)}m`;
  };

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h ago`;
    return date.toLocaleDateString();
  };

  return (
    <div className={`border rounded transition-colors ${getStatusColor()}`}>
      <button
        onClick={onToggle}
        className="w-full p-4 text-left hover:bg-dark-elevated/50 transition-colors"
      >
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-3 flex-1">
            <div className="mt-0.5">
              {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
            </div>
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                {getStatusIcon()}
                <span className="text-sm font-medium text-dark-text-primary">
                  {execution.status.charAt(0).toUpperCase() + execution.status.slice(1)}
                </span>
                <span className="text-2xs text-dark-text-secondary">
                  {formatTimestamp(execution.created_at)}
                </span>
              </div>
              <div className="flex items-center gap-4 text-xs text-dark-text-secondary">
                <div>Duration: {formatDuration(execution.duration_ms)}</div>
                <div>
                  Nodes: {execution.execution_metrics.nodes_executed}
                  {' '}({execution.execution_metrics.nodes_succeeded} ✓ / {execution.execution_metrics.nodes_failed} ✗)
                </div>
                {execution.triggered_by && (
                  <div>Trigger: {execution.triggered_by}</div>
                )}
              </div>
            </div>
          </div>
        </div>
      </button>

      {isExpanded && logs && (
        <div className="border-t border-dark-border bg-dark-bg/50">
          <div className="p-4 space-y-2">
            {logs.map(log => (
              <LogEntry key={log.id} log={log} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

interface LogEntryProps {
  log: WorkflowExecutionLog;
}

function LogEntry({ log }: LogEntryProps) {
  const getStatusIcon = () => {
    switch (log.status) {
      case 'completed':
        return <CheckCircle size={14} className="text-green-500" />;
      case 'failed':
        return <XCircle size={14} className="text-red-500" />;
      case 'running':
        return <Clock size={14} className="text-blue-500" />;
      case 'skipped':
        return <AlertCircle size={14} className="text-gray-500" />;
      default:
        return <Clock size={14} className="text-gray-400" />;
    }
  };

  return (
    <div className="flex items-start gap-3 p-3 bg-dark-surface rounded text-xs">
      {getStatusIcon()}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <span className="font-medium text-dark-text-primary">{log.node_key}</span>
          <span className="text-dark-text-secondary">({log.node_type})</span>
          {log.latency_ms !== null && log.latency_ms !== undefined && (
            <span className="text-dark-text-secondary">
              {log.latency_ms}ms
            </span>
          )}
        </div>
        {log.message && (
          <p className="text-dark-text-secondary">{log.message}</p>
        )}
        {log.error_data && (
          <div className="mt-1 p-2 bg-red-500/10 border border-red-500/30 rounded">
            <p className="text-red-400 font-mono text-2xs">{log.error_data.message}</p>
          </div>
        )}
        {log.output_data && Object.keys(log.output_data).length > 0 && (
          <details className="mt-1">
            <summary className="cursor-pointer text-dark-text-secondary hover:text-dark-text-primary">
              Output data
            </summary>
            <pre className="mt-1 p-2 bg-dark-bg rounded overflow-x-auto text-2xs">
              {JSON.stringify(log.output_data, null, 2)}
            </pre>
          </details>
        )}
      </div>
    </div>
  );
}
