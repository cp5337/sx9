import { useState } from 'react';
import { supabase } from '../lib/supabase';
import { Workflow, NodeTypeDefinition, WorkflowNode } from '../types/workflow.types';
import { workflowExecutor } from '../lib/workflow/executor';
import WorkflowList from './WorkflowList';
import WorkflowCanvas from './WorkflowCanvas';
import WorkflowNodeLibrary from './WorkflowNodeLibrary';
import WorkflowExecutionHistory from './WorkflowExecutionHistory';
import { Play, Save, Settings, History, Layers, FileCode } from 'lucide-react';

export default function WorkflowBuilder() {
  const [selectedWorkflow, setSelectedWorkflow] = useState<Workflow | null>(null);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'canvas' | 'history' | 'settings'>('canvas');
  const [isExecuting, setIsExecuting] = useState(false);
  const [showNodeLibrary, setShowNodeLibrary] = useState(false);

  const handleCreateWorkflow = async () => {
    const { data, error } = await supabase
      .from('workflows')
      .insert({
        name: `New Workflow ${new Date().toLocaleTimeString()}`,
        description: 'A new workflow',
        trigger_type: 'manual',
        status: 'draft',
        definition: { nodes: [], edges: [] }
      })
      .select()
      .single();

    if (error) {
      console.error('Error creating workflow:', error);
    } else {
      setSelectedWorkflow(data as Workflow);
    }
  };

  const handleSelectWorkflow = (workflow: Workflow) => {
    setSelectedWorkflow(workflow);
    setSelectedNodeId(null);
    setActiveTab('canvas');
  };

  const handleAddNode = async (nodeType: NodeTypeDefinition) => {
    if (!selectedWorkflow) return;

    const nodeKey = `${nodeType.id}_${Date.now()}`;
    const newNode: WorkflowNode = {
      id: crypto.randomUUID(),
      workflow_id: selectedWorkflow.id,
      node_key: nodeKey,
      node_type: nodeType.id,
      label: nodeType.name,
      category: nodeType.category,
      node_config: nodeType.defaultConfig || {},
      input_schema: { type: 'object', properties: {} },
      output_schema: { type: 'object', properties: {} },
      position_x: Math.random() * 400 + 200,
      position_y: Math.random() * 300 + 100,
      timeout_seconds: 60,
      retry_enabled: true,
      max_retries: 2,
      created_at: new Date().toISOString()
    };

    const updatedDefinition = {
      nodes: [...selectedWorkflow.definition.nodes, newNode],
      edges: selectedWorkflow.definition.edges
    };

    const { data, error } = await supabase
      .from('workflows')
      .update({ definition: updatedDefinition })
      .eq('id', selectedWorkflow.id)
      .select()
      .single();

    if (error) {
      console.error('Error adding node:', error);
    } else {
      setSelectedWorkflow(data as Workflow);
      setShowNodeLibrary(false);
    }
  };

  const handleNodePositionChange = async (nodeId: string, x: number, y: number) => {
    if (!selectedWorkflow) return;

    const updatedNodes = selectedWorkflow.definition.nodes.map(node =>
      node.id === nodeId ? { ...node, position_x: x, position_y: y } : node
    );

    const updatedDefinition = {
      nodes: updatedNodes,
      edges: selectedWorkflow.definition.edges
    };

    await supabase
      .from('workflows')
      .update({ definition: updatedDefinition })
      .eq('id', selectedWorkflow.id);

    setSelectedWorkflow({
      ...selectedWorkflow,
      definition: updatedDefinition
    });
  };

  const handleExecuteWorkflow = async () => {
    if (!selectedWorkflow || isExecuting) return;

    setIsExecuting(true);
    try {
      await workflowExecutor.executeWorkflow(selectedWorkflow.id, {}, 'manual');
      setActiveTab('history');
    } catch (error) {
      console.error('Execution error:', error);
      alert(`Workflow execution failed: ${(error as Error).message}`);
    } finally {
      setIsExecuting(false);
    }
  };

  const handleSaveWorkflow = async () => {
    if (!selectedWorkflow) return;

    const { error } = await supabase
      .from('workflows')
      .update({
        updated_at: new Date().toISOString()
      })
      .eq('id', selectedWorkflow.id);

    if (error) {
      console.error('Error saving workflow:', error);
    } else {
      alert('Workflow saved successfully');
    }
  };

  return (
    <div className="flex h-full bg-dark-bg">
      <div className="w-80 border-r border-dark-border">
        <WorkflowList
          onSelectWorkflow={handleSelectWorkflow}
          onCreateWorkflow={handleCreateWorkflow}
          selectedWorkflowId={selectedWorkflow?.id || null}
        />
      </div>

      <div className="flex-1 flex flex-col">
        {selectedWorkflow ? (
          <>
            <div className="flex items-center justify-between px-4 py-3 border-b border-dark-border bg-dark-surface">
              <div className="flex-1">
                <h1 className="text-lg font-semibold text-dark-text-primary">
                  {selectedWorkflow.name}
                </h1>
                {selectedWorkflow.description && (
                  <p className="text-sm text-dark-text-secondary">
                    {selectedWorkflow.description}
                  </p>
                )}
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setShowNodeLibrary(!showNodeLibrary)}
                  className={`flex items-center gap-2 px-3 py-1.5 rounded transition-colors ${
                    showNodeLibrary
                      ? 'bg-blue-600 text-white'
                      : 'bg-dark-elevated text-dark-text-primary hover:bg-dark-elevated/80'
                  }`}
                >
                  <Layers size={16} />
                  Nodes
                </button>
                <button
                  onClick={handleSaveWorkflow}
                  className="flex items-center gap-2 px-3 py-1.5 bg-dark-elevated text-dark-text-primary hover:bg-dark-elevated/80 rounded transition-colors"
                >
                  <Save size={16} />
                  Save
                </button>
                <button
                  onClick={handleExecuteWorkflow}
                  disabled={isExecuting}
                  className="flex items-center gap-2 px-3 py-1.5 bg-green-600 hover:bg-green-700 text-white rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Play size={16} />
                  {isExecuting ? 'Running...' : 'Run'}
                </button>
              </div>
            </div>

            <div className="flex items-center px-4 py-2 border-b border-dark-border bg-dark-surface">
              <button
                onClick={() => setActiveTab('canvas')}
                className={`flex items-center gap-2 px-3 py-1.5 text-sm rounded transition-colors ${
                  activeTab === 'canvas'
                    ? 'bg-dark-elevated text-dark-text-primary'
                    : 'text-dark-text-secondary hover:text-dark-text-primary'
                }`}
              >
                <FileCode size={16} />
                Canvas
              </button>
              <button
                onClick={() => setActiveTab('history')}
                className={`flex items-center gap-2 px-3 py-1.5 text-sm rounded transition-colors ${
                  activeTab === 'history'
                    ? 'bg-dark-elevated text-dark-text-primary'
                    : 'text-dark-text-secondary hover:text-dark-text-primary'
                }`}
              >
                <History size={16} />
                Executions
              </button>
              <button
                onClick={() => setActiveTab('settings')}
                className={`flex items-center gap-2 px-3 py-1.5 text-sm rounded transition-colors ${
                  activeTab === 'settings'
                    ? 'bg-dark-elevated text-dark-text-primary'
                    : 'text-dark-text-secondary hover:text-dark-text-primary'
                }`}
              >
                <Settings size={16} />
                Settings
              </button>
            </div>

            <div className="flex-1 flex overflow-hidden">
              {showNodeLibrary && (
                <div className="w-80 border-r border-dark-border">
                  <WorkflowNodeLibrary onNodeSelect={handleAddNode} />
                </div>
              )}

              <div className="flex-1">
                {activeTab === 'canvas' && (
                  <WorkflowCanvas
                    workflow={selectedWorkflow}
                    selectedNodeId={selectedNodeId}
                    onNodeSelect={setSelectedNodeId}
                    onNodePositionChange={handleNodePositionChange}
                  />
                )}
                {activeTab === 'history' && (
                  <WorkflowExecutionHistory workflowId={selectedWorkflow.id} />
                )}
                {activeTab === 'settings' && (
                  <div className="p-4 text-dark-text-secondary">
                    Workflow settings coming soon...
                  </div>
                )}
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-dark-text-secondary">
            <div className="text-center">
              <FileCode size={64} className="mx-auto mb-4 opacity-20" />
              <p className="text-lg mb-2">No workflow selected</p>
              <p className="text-sm">Select a workflow from the list or create a new one</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
