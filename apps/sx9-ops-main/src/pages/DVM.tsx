import React, { useState, useEffect } from 'react';
import { Cpu, Database, Network, Shield, Zap, Globe, Activity, AlertTriangle, CheckCircle, Clock, Brain, Target, Terminal } from 'lucide-react';
import KnowledgeGraph from '@/components/KnowledgeGraph';
import PineconeSVM from '@/components/PineconeSVM';
import NetworksControl from '@/components/NetworksControl';
import WSLEnvironment from '@/components/WSLEnvironment';

const DVM: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'knowledge' | 'networks' | 'wsl'>('knowledge');

  return (
    <div className="h-full bg-gray-100 dark:bg-gray-900 p-4 overflow-auto">
      {/* Header */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <Cpu className="w-8 h-8 text-blue-600" />
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
              Deception Vector Machine (DVM)
            </h1>
          </div>
          <div className="flex items-center gap-2">
            <span className="bg-blue-100 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 px-3 py-1 rounded text-sm font-semibold">
              AI-Powered Deception
            </span>
          </div>
        </div>
        
        <p className="text-gray-600 dark:text-gray-400 mb-4">
          Advanced deception and vector manipulation system using AI/ML for threat detection and response.
        </p>

        {/* System Overview Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Brain className="w-4 h-4 text-blue-500" />
              <span className="text-sm font-medium text-blue-800 dark:text-blue-200">AI Models</span>
            </div>
            <span className="text-2xl font-bold text-blue-900 dark:text-blue-100">12</span>
          </div>
          
          <div className="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Target className="w-4 h-4 text-green-500" />
              <span className="text-sm font-medium text-green-800 dark:text-green-200">Active Vectors</span>
            </div>
            <span className="text-2xl font-bold text-green-900 dark:text-green-100">47</span>
          </div>
          
          <div className="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Shield className="w-4 h-4 text-purple-500" />
              <span className="text-sm font-medium text-purple-800 dark:text-purple-200">Threats Blocked</span>
            </div>
            <span className="text-2xl font-bold text-purple-900 dark:text-purple-100">1,234</span>
          </div>
          
          <div className="bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Activity className="w-4 h-4 text-orange-500" />
              <span className="text-sm font-medium text-orange-800 dark:text-orange-200">Success Rate</span>
            </div>
            <span className="text-2xl font-bold text-orange-900 dark:text-orange-100">94.2%</span>
          </div>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
        <div className="flex space-x-2">
          <button
            onClick={() => setActiveTab('knowledge')}
            className={`px-4 py-2 rounded-md font-medium transition-colors ${
              activeTab === 'knowledge' 
                ? 'bg-blue-600 text-white' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
            }`}
          >
            <Brain className="w-4 h-4 inline mr-2" />
            Knowledge Graph
          </button>
          <button
            onClick={() => setActiveTab('networks')}
            className={`px-4 py-2 rounded-md font-medium transition-colors ${
              activeTab === 'networks' 
                ? 'bg-blue-600 text-white' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
            }`}
          >
            <Network className="w-4 h-4 inline mr-2" />
            Networks
          </button>
          <button
            onClick={() => setActiveTab('wsl')}
            className={`px-4 py-2 rounded-md font-medium transition-colors ${
              activeTab === 'wsl' 
                ? 'bg-blue-600 text-white' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
            }`}
          >
            <Terminal className="w-4 h-4 inline mr-2" />
            WSL Environment
          </button>
        </div>
      </div>

      {/* Content Area */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow">
        <div className="p-4">
          {activeTab === 'knowledge' && (
            <div className="space-y-4">
              <div>
                <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                  Knowledge Graph & AI Models
                </h2>
                <KnowledgeGraph />
              </div>
              <div className="mt-6">
                <h3 className="text-md font-semibold text-gray-900 dark:text-white mb-4">
                  Pinecone SVM Analysis
                </h3>
                <PineconeSVM />
              </div>
            </div>
          )}
          {activeTab === 'networks' && (
            <div>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                Network Control & Monitoring
              </h2>
              <NetworksControl />
            </div>
          )}
          {activeTab === 'wsl' && (
            <div>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                Windows Subsystem for Linux Environment
              </h2>
              <WSLEnvironment />
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default DVM;