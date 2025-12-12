import React, { useState } from 'react';
import { Activity, Settings, Plus, RefreshCw, Database, AlertCircle, ChevronLeft, ChevronRight, Zap, Target, TrendingUp } from 'lucide-react';
import { AgentPanel } from '../components/plasma/agent-panel';
import { AgentDetailModal } from '../components/plasma/agent-detail-modal';
import { AgentDeploymentModal } from '../components/plasma/agent-deployment-modal';
import { WazuhConfigModal } from '../components/plasma/wazuh-config-modal';
import { useWazuhAgents } from '../hooks/use-wazuh-agents';
import { usePlasmaStream } from '../hooks/use-plasma-stream';
import { useCDNStats } from '../hooks/use-cdn-stats';
import type { WazuhAgent, Threat } from '../types/plasma';

const Plasma: React.FC = () => {
  const [selectedAgent, setSelectedAgent] = useState<WazuhAgent | null>(null);
  const [showAgentDetail, setShowAgentDetail] = useState(false);
  const [showDeployment, setShowDeployment] = useState(false);
  const [showWazuhConfig, setShowWazuhConfig] = useState(false);
  const [showLeftPanel, setShowLeftPanel] = useState(true);
  const [showRightPanel, setShowRightPanel] = useState(true);

  const { agents, managers, loading, refreshAgents, addManager, removeManager, restartAgent, deleteAgent } =
    useWazuhAgents();

  const { threats, connected, useMockData } = usePlasmaStream(true);
  const { stats: cdnStats, connected: cdnConnected } = useCDNStats(true);

  const handleAgentClick = (agent: WazuhAgent) => {
    setSelectedAgent(agent);
    setShowAgentDetail(true);
  };

  const getThreatBadge = (level: Threat["level"]) => {
    const badges = {
      critical: "bg-red-950/30 text-red-400 border-red-900/50",
      high: "bg-amber-950/30 text-amber-400 border-amber-900/50",
      medium: "bg-blue-950/30 text-blue-400 border-blue-900/50",
      low: "border-slate-700 text-slate-400"
    };
    return (
      <span className={`px-2 py-1 rounded text-xs font-semibold border ${badges[level]}`}>
        {level.toUpperCase()}
      </span>
    );
  };

  const criticalCount = threats.filter((t) => t.level === "critical").length;
  const highCount = threats.filter((t) => t.level === "high").length;
  const activeAgents = agents.filter((a) => a.status === "active").length;

  return (
    <div className="h-screen flex flex-col bg-gray-900">
      {/* Header */}
      <header className="h-14 border-b border-gray-800 bg-gray-900/50 flex items-center justify-between px-4">
        <div className="flex items-center gap-3">
          <Activity className="w-5 h-5 text-slate-400" />
          <h1 className="text-lg font-mono font-bold tracking-wider text-gray-200">PLASMA</h1>
          <span className={`px-2 py-1 rounded text-xs font-mono border ${
            connected ? 'border-green-700 text-green-400' : 'border-red-700 text-red-400'
          }`}>
            {connected ? "CONNECTED" : "DISCONNECTED"}
          </span>
          {useMockData && (
            <span className="px-2 py-1 rounded text-xs font-mono text-amber-500 border border-amber-500/50 flex items-center gap-1">
              <AlertCircle className="w-3 h-3" />
              MOCK MODE
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <div className="flex items-center gap-4 mr-4 text-xs font-mono">
            <div className="flex items-center gap-2">
              <span className="text-slate-400">AGENTS:</span>
              <span className="text-green-500">
                {activeAgents}/{agents.length}
              </span>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-slate-400">THREATS:</span>
              <span className="text-red-500">{criticalCount}</span>
              <span className="text-slate-400">/</span>
              <span className="text-amber-500">{highCount}</span>
            </div>
          </div>

          <button
            onClick={refreshAgents}
            disabled={loading}
            className="px-3 py-1.5 bg-transparent border border-gray-700 hover:border-gray-600 text-gray-200 rounded text-sm flex items-center gap-2 transition-colors"
          >
            <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
            REFRESH
          </button>

          <button
            onClick={() => setShowDeployment(true)}
            className="px-3 py-1.5 bg-transparent border border-gray-700 hover:border-gray-600 text-gray-200 rounded text-sm flex items-center gap-2 transition-colors"
          >
            <Plus className="w-4 h-4" />
            DEPLOY
          </button>

          <button
            onClick={() => setShowWazuhConfig(true)}
            className="px-3 py-1.5 bg-transparent border border-gray-700 hover:border-gray-600 text-gray-200 rounded text-sm flex items-center gap-2 transition-colors"
          >
            <Database className="w-4 h-4" />
            MANAGERS
          </button>

          <button
            onClick={() => setShowWazuhConfig(true)}
            className="px-3 py-1.5 bg-transparent border border-gray-700 hover:border-gray-600 text-gray-200 rounded text-sm flex items-center gap-2 transition-colors"
          >
            <Settings className="w-4 h-4" />
            CONFIG
          </button>
        </div>
      </header>

      {/* Main Layout */}
      <div className="flex-1 flex overflow-hidden">
        {/* Agent Panel */}
        {showLeftPanel ? (
          <div className="w-80 border-r border-gray-800 relative">
            <button
              onClick={() => setShowLeftPanel(false)}
              className="absolute top-2 right-2 z-10 p-1 bg-gray-800 hover:bg-gray-700 rounded border border-gray-700 text-gray-400 transition-colors"
              title="Collapse Agents Panel"
            >
              <ChevronLeft className="w-4 h-4" />
            </button>
            <AgentPanel
              agents={agents}
              onAgentClick={handleAgentClick}
              onRestartAgent={restartAgent}
              onDeleteAgent={deleteAgent}
            />
          </div>
        ) : (
          <div className="w-12 border-r border-gray-800 flex flex-col items-center py-2 gap-4 bg-gray-900/30">
            <button
              onClick={() => setShowLeftPanel(true)}
              className="p-2 bg-gray-800 hover:bg-gray-700 rounded border border-gray-700 text-gray-400 transition-colors"
              title="Expand Agents Panel"
            >
              <ChevronRight className="w-4 h-4" />
            </button>
            <button
              onClick={() => setShowLeftPanel(true)}
              className="p-2 hover:bg-gray-800 rounded transition-colors group"
              title="Wazuh Agents"
            >
              <Activity className="w-5 h-5 text-slate-500 group-hover:text-green-400" />
            </button>
          </div>
        )}

        {/* Threat Stream */}
        <div className="flex-1 flex flex-col">
          <div className="p-4 border-b border-gray-800 bg-gray-900/30">
            <h2 className="text-sm font-mono font-semibold tracking-wider text-gray-200">THREAT STREAM</h2>
            <div className="text-xs text-slate-400 mt-1">
              {useMockData ? "Mock data (AXON unavailable)" : "Real-time threat intelligence from AXON"}
            </div>
          </div>
          <div className="flex-1 overflow-y-auto">
            <div className="p-2 space-y-2">
              {threats.map((threat) => (
                <div
                  key={threat.id}
                  className="p-3 rounded-md border border-gray-800 bg-gray-900/50 hover:bg-gray-800/50 transition-colors cursor-pointer"
                >
                  <div className="flex items-start justify-between mb-2">
                    {getThreatBadge(threat.level)}
                    <span className="text-xs text-slate-400 font-mono" suppressHydrationWarning>
                      {new Date(threat.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                  <div className="text-sm text-gray-200 mb-2">{threat.description}</div>
                  <div className="flex items-center gap-4 text-xs font-mono text-slate-400">
                    <span>
                      <span className="text-slate-500">Agent:</span> {threat.agentId}
                    </span>
                    <span>
                      <span className="text-slate-500">Rule:</span> {threat.ruleId}
                    </span>
                    {threat.mitreTactic && (
                      <span>
                        <span className="text-slate-500">MITRE:</span> {threat.mitreTactic}
                      </span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* HFT Analytics Panel */}
        {showRightPanel ? (
          <div className="w-80 border-l border-gray-800 bg-gray-900/30 relative">
            <button
              onClick={() => setShowRightPanel(false)}
              className="absolute top-2 left-2 z-10 p-1 bg-gray-800 hover:bg-gray-700 rounded border border-gray-700 text-gray-400"
            >
              <ChevronRight className="w-4 h-4" />
            </button>
            <div className="p-4 border-b border-gray-800">
              <h2 className="text-sm font-mono font-semibold tracking-wider text-gray-200">HFT ANALYTICS</h2>
              <div className="text-xs text-slate-400 mt-1">High-Frequency Threat Intelligence</div>
            </div>
            <div className="p-4 space-y-4">
              <div className="bg-gray-800/50 rounded-lg p-3 border border-gray-700">
                <div className="text-xs text-slate-400 mb-1">Threat Velocity</div>
                <div className="text-2xl font-mono font-bold text-gray-200">
                  {cdnStats?.threats.total || threats.length}
                </div>
                <div className="text-xs text-green-500 mt-1">
                  {cdnConnected ? 'Live from CDN' : 'Mock data'}
                </div>
              </div>
              <div className="bg-gray-800/50 rounded-lg p-3 border border-gray-700">
                <div className="text-xs text-slate-400 mb-1">Detection Latency</div>
                <div className="text-2xl font-mono font-bold text-gray-200">
                  {cdnStats?.events.rate ? `${Math.round(1000 / cdnStats.events.rate)}ms` : '47ms'}
                </div>
                <div className="text-xs text-green-500 mt-1">
                  {cdnConnected ? `${cdnStats?.events.rate.toFixed(1)} events/s` : 'Mock data'}
                </div>
              </div>
              <div className="bg-gray-800/50 rounded-lg p-3 border border-gray-700">
                <div className="text-xs text-slate-400 mb-1">MITRE Coverage</div>
                <div className="text-2xl font-mono font-bold text-gray-200">
                  {cdnStats ? `${Math.round((cdnStats.hd4.detect / 188) * 100)}%` : '87%'}
                </div>
                <div className="text-xs text-slate-400 mt-1">
                  {cdnStats ? `${cdnStats.hd4.detect}/188 techniques` : '164/188 techniques'}
                </div>
              </div>
              <div className="bg-gray-800/50 rounded-lg p-3 border border-gray-700">
                <div className="text-xs text-slate-400 mb-1">Tool Executions</div>
                <div className="text-2xl font-mono font-bold text-gray-200">
                  {cdnStats?.tools.executions || 0}
                </div>
                <div className="text-xs text-slate-400 mt-1">
                  {cdnStats?.tools.active || 0} active
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="w-12 border-l border-gray-800 flex flex-col items-center py-2 gap-3 bg-gray-900/30">
            <button
              onClick={() => setShowRightPanel(true)}
              className="p-2 bg-gray-800 hover:bg-gray-700 rounded border border-gray-700 text-gray-400 transition-colors"
              title="Expand HFT Analytics"
            >
              <ChevronLeft className="w-4 h-4" />
            </button>
            <div className="flex flex-col items-center gap-4 mt-4">
              <button
                onClick={() => setShowRightPanel(true)}
                className="p-2 hover:bg-gray-800 rounded transition-colors group"
                title="Threat Velocity"
              >
                <TrendingUp className="w-5 h-5 text-slate-500 group-hover:text-green-400" />
              </button>
              <button
                onClick={() => setShowRightPanel(true)}
                className="p-2 hover:bg-gray-800 rounded transition-colors group"
                title="Detection Latency"
              >
                <Zap className="w-5 h-5 text-slate-500 group-hover:text-blue-400" />
              </button>
              <button
                onClick={() => setShowRightPanel(true)}
                className="p-2 hover:bg-gray-800 rounded transition-colors group"
                title="MITRE Coverage"
              >
                <Target className="w-5 h-5 text-slate-500 group-hover:text-amber-400" />
              </button>
              <button
                onClick={() => setShowRightPanel(true)}
                className="p-2 hover:bg-gray-800 rounded transition-colors group"
                title="HFT Analytics"
              >
                <Activity className="w-5 h-5 text-slate-500 group-hover:text-purple-400" />
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Modals */}
      {selectedAgent && (
        <AgentDetailModal
          agent={selectedAgent}
          isOpen={showAgentDetail}
          onClose={() => {
            setShowAgentDetail(false);
            setSelectedAgent(null);
          }}
        />
      )}

      <AgentDeploymentModal
        isOpen={showDeployment}
        onClose={() => setShowDeployment(false)}
        managers={managers}
      />

      <WazuhConfigModal
        isOpen={showWazuhConfig}
        onClose={() => setShowWazuhConfig(false)}
        managers={managers}
        onAddManager={addManager}
        onRemoveManager={removeManager}
      />
    </div>
  );
};

export default Plasma;
