import { useEffect, useState } from 'react';
import { supabase } from '../lib/supabase';
import { Workflow, WorkflowStatus } from '../types/workflow.types';
import { Plus, Play, Copy, Trash2, Clock, CheckCircle, XCircle, Calendar } from 'lucide-react';

interface WorkflowListProps {
  onSelectWorkflow: (workflow: Workflow) => void;
  onCreateWorkflow: () => void;
  selectedWorkflowId: string | null;
}

export default function WorkflowList({
  onSelectWorkflow,
  onCreateWorkflow,
  selectedWorkflowId
}: WorkflowListProps) {
  const [workflows, setWorkflows] = useState<Workflow[]>([]);
  const [loading, setLoading] = useState(true);
  const [filterStatus, setFilterStatus] = useState<WorkflowStatus | 'all'>('all');

  useEffect(() => {
    fetchWorkflows();

    const channel = supabase
      .channel('workflows-changes')
      .on('postgres_changes', { event: '*', schema: 'public', table: 'workflows' }, () => {
        fetchWorkflows();
      })
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  }, []);

  const fetchWorkflows = async () => {
    setLoading(true);
    let query = supabase
      .from('workflows')
      .select('*')
      .order('updated_at', { ascending: false });

    if (filterStatus !== 'all') {
      query = query.eq('status', filterStatus);
    }

    const { data, error } = await query;

    if (error) {
      console.error('Error fetching workflows:', error);
    } else {
      setWorkflows(data as Workflow[]);
    }
    setLoading(false);
  };

  const handleDuplicateWorkflow = async (workflow: Workflow, e: React.MouseEvent) => {
    e.stopPropagation();

    const { data, error } = await supabase
      .from('workflows')
      .insert({
        name: `${workflow.name} (Copy)`,
        description: workflow.description,
        definition: workflow.definition,
        status: 'draft',
        trigger_type: workflow.trigger_type,
        trigger_config: workflow.trigger_config
      })
      .select()
      .single();

    if (error) {
      console.error('Error duplicating workflow:', error);
    } else {
      await fetchWorkflows();
      onSelectWorkflow(data as Workflow);
    }
  };

  const handleDeleteWorkflow = async (workflowId: string, e: React.MouseEvent) => {
    e.stopPropagation();

    if (!confirm('Are you sure you want to delete this workflow?')) return;

    const { error } = await supabase
      .from('workflows')
      .delete()
      .eq('id', workflowId);

    if (error) {
      console.error('Error deleting workflow:', error);
    } else {
      fetchWorkflows();
    }
  };

  const handleExecuteWorkflow = async (workflowId: string, e: React.MouseEvent) => {
    e.stopPropagation();

    const { workflowExecutor } = await import('../lib/workflow/executor');

    try {
      await workflowExecutor.executeWorkflow(workflowId, {}, 'manual');
    } catch (error) {
      console.error('Error executing workflow:', error);
    }
  };

  useEffect(() => {
    fetchWorkflows();
  }, [filterStatus]);

  if (loading) {
    return (
      <div className="flex-1 flex items-center justify-center text-dark-text-secondary">
        Loading workflows...
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-dark-surface">
      <div className="p-4 border-b border-dark-border">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-dark-text-primary">Workflows</h2>
          <button
            onClick={onCreateWorkflow}
            className="flex items-center gap-2 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded transition-colors"
          >
            <Plus size={16} />
            New Workflow
          </button>
        </div>

        <div className="flex gap-2">
          {(['all', 'draft', 'active', 'archived'] as const).map(status => (
            <button
              key={status}
              onClick={() => setFilterStatus(status)}
              className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
                filterStatus === status
                  ? 'bg-blue-600 text-white'
                  : 'bg-dark-elevated text-dark-text-secondary hover:text-dark-text-primary'
              }`}
            >
              {status.charAt(0).toUpperCase() + status.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {workflows.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center px-4">
            <p className="text-dark-text-secondary mb-4">No workflows found</p>
            <button
              onClick={onCreateWorkflow}
              className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded transition-colors"
            >
              <Plus size={16} />
              Create Your First Workflow
            </button>
          </div>
        ) : (
          <div className="p-4 space-y-2">
            {workflows.map(workflow => (
              <WorkflowCard
                key={workflow.id}
                workflow={workflow}
                isSelected={selectedWorkflowId === workflow.id}
                onSelect={() => onSelectWorkflow(workflow)}
                onDuplicate={(e) => handleDuplicateWorkflow(workflow, e)}
                onDelete={(e) => handleDeleteWorkflow(workflow.id, e)}
                onExecute={(e) => handleExecuteWorkflow(workflow.id, e)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

interface WorkflowCardProps {
  workflow: Workflow;
  isSelected: boolean;
  onSelect: () => void;
  onDuplicate: (e: React.MouseEvent) => void;
  onDelete: (e: React.MouseEvent) => void;
  onExecute: (e: React.MouseEvent) => void;
}

function WorkflowCard({
  workflow,
  isSelected,
  onSelect,
  onDuplicate,
  onDelete,
  onExecute
}: WorkflowCardProps) {
  const getStatusIcon = () => {
    switch (workflow.status) {
      case 'active':
        return <CheckCircle size={14} className="text-green-500" />;
      case 'draft':
        return <Clock size={14} className="text-yellow-500" />;
      case 'archived':
        return <XCircle size={14} className="text-gray-500" />;
      default:
        return null;
    }
  };

  const successRate = workflow.total_executions > 0
    ? Math.round((workflow.successful_executions / workflow.total_executions) * 100)
    : 0;

  return (
    <button
      onClick={onSelect}
      className={`w-full p-4 rounded border transition-all text-left ${
        isSelected
          ? 'bg-dark-elevated border-blue-500'
          : 'bg-dark-bg border-dark-border hover:border-blue-500/50 hover:bg-dark-elevated'
      }`}
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            {getStatusIcon()}
            <h3 className="text-sm font-medium text-dark-text-primary truncate">
              {workflow.name}
            </h3>
          </div>
          {workflow.description && (
            <p className="text-xs text-dark-text-secondary line-clamp-2">
              {workflow.description}
            </p>
          )}
        </div>
        <div className="flex gap-1 ml-2">
          {workflow.status === 'active' && (
            <button
              onClick={onExecute}
              className="p-1.5 hover:bg-green-500/20 rounded transition-colors"
              title="Execute Workflow"
            >
              <Play size={14} className="text-green-500" />
            </button>
          )}
          <button
            onClick={onDuplicate}
            className="p-1.5 hover:bg-blue-500/20 rounded transition-colors"
            title="Duplicate"
          >
            <Copy size={14} className="text-blue-400" />
          </button>
          <button
            onClick={onDelete}
            className="p-1.5 hover:bg-red-500/20 rounded transition-colors"
            title="Delete"
          >
            <Trash2 size={14} className="text-red-400" />
          </button>
        </div>
      </div>

      <div className="flex items-center gap-4 text-2xs text-dark-text-secondary">
        <div className="flex items-center gap-1">
          <Calendar size={12} />
          <span>{new Date(workflow.updated_at).toLocaleDateString()}</span>
        </div>
        <div>
          {workflow.definition.nodes.length} nodes
        </div>
        {workflow.total_executions > 0 && (
          <div className="flex items-center gap-1">
            <span>{successRate}% success</span>
            <span>({workflow.total_executions} runs)</span>
          </div>
        )}
      </div>

      {workflow.tags && workflow.tags.length > 0 && (
        <div className="flex gap-1 mt-2 flex-wrap">
          {workflow.tags.slice(0, 3).map((tag, index) => (
            <span
              key={index}
              className="px-2 py-0.5 text-2xs bg-blue-500/10 text-blue-400 rounded"
            >
              {tag}
            </span>
          ))}
        </div>
      )}
    </button>
  );
}
