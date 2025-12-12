import React, { useState, useEffect } from 'react';
import { Workflow, Play, Settings, BarChart3, Clock, CheckCircle, AlertCircle } from 'lucide-react';
import { N8NWorkflow } from '../types';
import { getDemoData } from '../utils/demoDataProvider';

const N8NWorkflows: React.FC = () => {
  const [workflows, setWorkflows] = useState<N8NWorkflow[]>([]);
  const [selectedPhase, setSelectedPhase] = useState('Hunt');
  const [selectedWorkflow, setSelectedWorkflow] = useState<N8NWorkflow | null>(null);

  useEffect(() => {
    // Use centralized demo data provider
    const demoWorkflows = getDemoData<N8NWorkflow[]>('n8nWorkflows', []);
    setWorkflows(demoWorkflows);
  }, []);

  const phases = ['Hunt', 'Detect', 'Disrupt', 'Disable', 'Dominate'];

  const getPhaseIcon = (phase: string) => {
    switch (phase) {
      case 'Hunt':
        return <BarChart3 className="w-4 h-4 text-blue-500" />;
      case 'Detect':
        return <AlertCircle className="w-4 h-4 text-yellow-500" />;
      case 'Disrupt':
        return <AlertCircle className="w-4 h-4 text-orange-500" />;
      case 'Disable':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      case 'Dominate':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      default:
        return <Workflow className="w-4 h-4 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800';
      case 'paused':
        return 'bg-yellow-100 text-yellow-800';
      case 'error':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const filteredWorkflows = workflows.filter(workflow => workflow.phase === selectedPhase);

  const executeWorkflow = (workflowId: string) => {
    console.log(`Executing workflow ${workflowId}`);
    // Production workflow execution logic would go here
  };

  const createWorkflow = () => {
    console.log('Creating new workflow');
    // Production workflow creation logic would go here
  };

  const editWorkflow = (workflowId: string) => {
    console.log(`Editing workflow ${workflowId}`);
    // Production workflow editing logic would go here
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Workflow className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">N8N Workflows</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-purple-100 text-purple-800 px-3 py-1 rounded text-sm font-semibold">
                Automation Engine
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Automated workflows for threat intelligence collection, analysis, and response.
          </p>

          {/* Phase Selection */}
          <div className="flex gap-4 mb-6">
            {phases.map((phase) => (
              <button
                key={phase}
                onClick={() => setSelectedPhase(phase)}
                className={`flex items-center gap-2 px-4 py-2 rounded-md font-medium transition-colors ${
                  selectedPhase === phase 
                    ? 'bg-blue-600 text-white' 
                    : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                }`}
              >
                {getPhaseIcon(phase)}
                {phase}
              </button>
            ))}
          </div>
        </div>

        {/* Workflows Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredWorkflows.map((workflow) => (
            <div 
              key={workflow.id} 
              className="bg-white rounded-lg shadow-xl p-6 border border-gray-200 hover:shadow-2xl transition-shadow cursor-pointer"
              onClick={() => setSelectedWorkflow(workflow)}
            >
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center gap-2">
                  {getPhaseIcon(workflow.phase)}
                  <span className={`px-2 py-1 rounded text-xs font-semibold ${getStatusColor(workflow.status)}`}>
                    {workflow.status}
                  </span>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      executeWorkflow(workflow.id);
                    }}
                    className="p-1 text-gray-400 hover:text-green-600 transition-colors"
                    title="Execute Workflow"
                  >
                    <Play className="w-4 h-4" />
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      editWorkflow(workflow.id);
                    }}
                    className="p-1 text-gray-400 hover:text-blue-600 transition-colors"
                    title="Edit Workflow"
                  >
                    <Settings className="w-4 h-4" />
                  </button>
                </div>
              </div>
              
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                {workflow.name}
              </h3>
              
              <p className="text-gray-600 text-sm mb-4 line-clamp-2">
                {workflow.description}
              </p>
              
              <div className="space-y-2">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-500">Nodes:</span>
                  <span className="font-medium">{workflow.nodes}</span>
                </div>
                
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-500">Success Rate:</span>
                  <span className="font-medium">{(workflow.successRate * 100).toFixed(1)}%</span>
                </div>
                
                <div className="flex items-center gap-2 text-sm text-gray-500">
                  <Clock className="w-4 h-4" />
                  <span>Last run: {new Date(workflow.lastRun).toLocaleDateString()}</span>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Workflow Detail Modal */}
        {selectedWorkflow && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    {getPhaseIcon(selectedWorkflow.phase)}
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedWorkflow.name}
                    </h2>
                  </div>
                  <button
                    onClick={() => setSelectedWorkflow(null)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                
                <div className="space-y-4">
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Description</h3>
                    <p className="text-gray-900 mt-1">{selectedWorkflow.description}</p>
                  </div>
                  
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Phase</h3>
                      <p className="text-gray-900 mt-1">{selectedWorkflow.phase}</p>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Status</h3>
                      <span className={`inline-block px-2 py-1 rounded text-xs font-semibold mt-1 ${getStatusColor(selectedWorkflow.status)}`}>
                        {selectedWorkflow.status}
                      </span>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Nodes</h3>
                      <p className="text-gray-900 mt-1">{selectedWorkflow.nodes}</p>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Success Rate</h3>
                      <p className="text-gray-900 mt-1">{(selectedWorkflow.successRate * 100).toFixed(1)}%</p>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500">Last Run</h3>
                    <p className="text-gray-900 mt-1">
                      {new Date(selectedWorkflow.lastRun).toLocaleString()}
                    </p>
                  </div>
                  
                  <div className="flex gap-4 pt-4">
                    <button
                      onClick={() => {
                        executeWorkflow(selectedWorkflow.id);
                        setSelectedWorkflow(null);
                      }}
                      className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors"
                    >
                      <Play className="w-4 h-4" />
                      Execute
                    </button>
                    <button
                      onClick={() => {
                        editWorkflow(selectedWorkflow.id);
                        setSelectedWorkflow(null);
                      }}
                      className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                    >
                      <Settings className="w-4 h-4" />
                      Edit
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default N8NWorkflows;