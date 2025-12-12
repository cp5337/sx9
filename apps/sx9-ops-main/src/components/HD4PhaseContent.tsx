import React, { useState, useEffect } from 'react';
import { Book, Target, Code, Link, Shield, ChevronDown, ChevronUp, Terminal } from 'lucide-react';
import HD4Map from './HD4Map';
import CTASGraphAnimation from './CTASGraphAnimation';
import HD4TaskView from './HD4TaskView';
import KaliToolsIntegration from './KaliToolsIntegration';
import Playbooks from './Playbooks';
import RedTeamRunner from './RedTeam/RedTeamRunner';
import PhaseMapping from './RedTeam/PhaseMapping';
import MultiCLI from './MultiCLI';

interface HD4PhaseContentProps {
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

type AgentType = 'natasha' | 'marcus' | 'elena' | 'cove' | 'kali';

const HD4PhaseContent: React.FC<HD4PhaseContentProps> = ({ phase, view }) => {
  const [activeTab, setActiveTab] = useState<'overview' | 'playbooks' | 'kali' | 'redteam' | 'mapping' | 'tasks'>('overview');
  const [activeAgent, setActiveAgent] = useState<AgentType>('natasha'); // Active agent tab
  const [agentView, setAgentView] = useState<'chat' | 'terminal' | 'split'>('split'); // View mode for agent tabs
  const [mapHeight, setMapHeight] = useState(60); // percentage
  const [isResizing, setIsResizing] = useState(false);
  const [showCLI, setShowCLI] = useState(false); // Persist CLI across horizon tabs

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;
      
      const container = document.getElementById('hd4-overview-container');
      if (!container) return;
      
      const containerRect = container.getBoundingClientRect();
      const relativeY = e.clientY - containerRect.top;
      const newHeightPercent = (relativeY / containerRect.height) * 100;
      
      // Clamp between 30% and 80%
      if (newHeightPercent >= 30 && newHeightPercent <= 80) {
        setMapHeight(newHeightPercent);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
    };

    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isResizing]);

  return (
    <div className="h-full flex flex-col bg-gray-900 text-gray-300">
      <div className="flex-none">
        <div className="flex border-b border-gray-700 items-center">
          <button
            onClick={() => setActiveTab('overview')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'overview'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Target size={12} className="mr-2" />
            Overview
          </button>
          <button
            onClick={() => setActiveTab('kali')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'kali'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Code size={12} className="mr-2" />
            Kali Tools
          </button>
          <button
            onClick={() => setActiveTab('playbooks')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'playbooks'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Book size={12} className="mr-2" />
            Playbooks
          </button>
          <button
            onClick={() => setActiveTab('redteam')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'redteam'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Shield size={12} className="mr-2" />
            Red Team
          </button>
          <button
            onClick={() => setActiveTab('mapping')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'mapping'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Link size={12} className="mr-2" />
            Phase Mapping
          </button>
          <button
            onClick={() => setActiveTab('tasks')}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px ${
              activeTab === 'tasks'
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
          >
            <Code size={12} className="mr-2" />
            Tasks
          </button>
          <div className="flex-1" />
          <button
            onClick={() => setShowCLI(!showCLI)}
            className={`flex items-center px-3 py-1.5 text-xs font-medium border-b-2 -mb-px transition-colors ${
              showCLI
                ? 'border-blue-500 text-blue-500'
                : 'border-transparent text-gray-500 hover:text-gray-400 hover:border-gray-400'
            }`}
            title={showCLI ? 'Hide CLI' : 'Show CLI'}
          >
            <Terminal size={12} className="mr-2" />
            {showCLI ? 'Hide CLI' : 'Show CLI'}
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-hidden">
        {activeTab === 'overview' && (
          <div id="hd4-overview-container" className="h-full flex flex-col relative">
            {/* Map/Graph Section - Full bleed */}
            <div 
              className="relative overflow-hidden"
              style={{ height: `${mapHeight}%`, minHeight: '200px' }}
            >
              {view === 'map' ? <HD4Map hd4Action={phase} /> : <CTASGraphAnimation />}
            </div>

            {/* Resize Handle with Chevrons */}
            <div
              onMouseDown={handleMouseDown}
              className={`relative flex items-center justify-center cursor-ns-resize group transition-all ${
                isResizing ? 'bg-blue-500/50 h-8' : 'bg-gray-700/50 hover:bg-blue-500/30 h-6'
              }`}
              style={{ zIndex: 20 }}
            >
              <div className="flex flex-col items-center">
                <ChevronUp 
                  size={16} 
                  className={`${isResizing ? 'text-white' : 'text-gray-400 group-hover:text-blue-400'} transition-colors`}
                />
                <div className="flex gap-1">
                  <div className={`w-1 h-1 rounded-full ${isResizing ? 'bg-white' : 'bg-gray-500 group-hover:bg-blue-400'}`} />
                  <div className={`w-1 h-1 rounded-full ${isResizing ? 'bg-white' : 'bg-gray-500 group-hover:bg-blue-400'}`} />
                  <div className={`w-1 h-1 rounded-full ${isResizing ? 'bg-white' : 'bg-gray-500 group-hover:bg-blue-400'}`} />
                </div>
                <ChevronDown 
                  size={16} 
                  className={`${isResizing ? 'text-white' : 'text-gray-400 group-hover:text-blue-400'} transition-colors`}
                />
              </div>
              <div className="absolute right-4 text-xs text-gray-400 group-hover:text-blue-400">
                Drag to resize • Map: {Math.round(mapHeight)}% • Tasks: {Math.round(100 - mapHeight)}%
              </div>
            </div>

            {/* Bottom Panel - Agent Tabs with Chat + Terminal OR CLI if enabled */}
            {showCLI ? (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent={activeAgent}
                />
              </div>
            ) : (
              <div 
                className="flex-1 overflow-hidden flex flex-col"
                style={{ minHeight: '200px' }}
              >
              {/* Agent Tabs */}
              <div className="flex-none flex items-center bg-gray-800 border-b border-gray-700 px-2 py-1">
                <div className="flex gap-1">
                  {(['natasha', 'marcus', 'elena', 'cove', 'kali'] as AgentType[]).map((agent) => (
                    <button
                      key={agent}
                      onClick={() => setActiveAgent(agent)}
                      className={`px-3 py-1 text-xs rounded transition-colors ${
                        activeAgent === agent
                          ? 'bg-blue-600 text-white'
                          : 'bg-gray-700 text-gray-400 hover:bg-gray-600 hover:text-white'
                      }`}
                    >
                      {agent.charAt(0).toUpperCase() + agent.slice(1)}
                    </button>
                  ))}
                </div>
                <div className="flex-1" />
                <div className="flex gap-1">
                  <button
                    onClick={() => setAgentView('chat')}
                    className={`px-2 py-1 text-xs rounded ${
                      agentView === 'chat' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-400'
                    }`}
                    title="Chat only"
                  >
                    💬
                  </button>
                  <button
                    onClick={() => setAgentView('split')}
                    className={`px-2 py-1 text-xs rounded ${
                      agentView === 'split' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-400'
                    }`}
                    title="Split view"
                  >
                    ⚡
                  </button>
                  <button
                    onClick={() => setAgentView('terminal')}
                    className={`px-2 py-1 text-xs rounded ${
                      agentView === 'terminal' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-400'
                    }`}
                    title="Terminal only"
                  >
                    {'>_'}
                  </button>
                </div>
                <div className="text-xxs text-gray-500 ml-2">
                  {activeAgent} • {agentView === 'split' ? 'Chat + Terminal' : agentView === 'chat' ? 'Chat' : 'Terminal'}
                </div>
              </div>

              {/* Agent Content - Chat + Terminal Split */}
              <div className="flex-1 overflow-hidden flex">
                {(agentView === 'chat' || agentView === 'split') && (
                  <div 
                    className={agentView === 'split' ? 'w-1/2 border-r border-gray-700' : 'w-full'}
                    onDragOver={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                    }}
                    onDrop={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                    }}
                  >
                    <MultiCLI 
                      currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                      className="h-full"
                      defaultAgent={activeAgent}
                      showTerminal={false}
                    />
                  </div>
                )}
                {(agentView === 'terminal' || agentView === 'split') && (
                  <div className={agentView === 'split' ? 'w-1/2' : 'w-full'}>
                    <MultiCLI 
                      currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                      className="h-full"
                      defaultAgent={activeAgent}
                      showChat={false}
                      terminalMode={true}
                    />
                  </div>
                )}
              </div>
              </div>
            )}
          </div>
        )}

        {activeTab === 'kali' && (
          <div className="h-full flex flex-col overflow-hidden">
            <div className="flex-1 overflow-y-auto">
              <KaliToolsIntegration />
            </div>
            {showCLI && (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent="kali"
                />
              </div>
            )}
          </div>
        )}

        {activeTab === 'playbooks' && (
          <div className="h-full flex flex-col overflow-hidden">
            <div className="flex-1 overflow-y-auto">
              <Playbooks />
            </div>
            {showCLI && (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent="natasha"
                />
              </div>
            )}
          </div>
        )}

        {activeTab === 'redteam' && (
          <div className="h-full flex flex-col overflow-hidden">
            <div className="flex-1 overflow-y-auto">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                <div>
                  <h2 className="text-sm font-semibold mb-4 flex items-center">
                    <Target size={14} className="mr-2" />
                    Red Team Runner
                  </h2>
                  <RedTeamRunner phase={phase} />
                </div>
              </div>
            </div>
            {showCLI && (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent="kali"
                />
              </div>
            )}
          </div>
        )}

        {activeTab === 'mapping' && (
          <div className="h-full flex flex-col overflow-hidden">
            <div className="flex-1 overflow-y-auto">
              <PhaseMapping />
            </div>
            {showCLI && (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent="natasha"
                />
              </div>
            )}
          </div>
        )}

        {activeTab === 'tasks' && (
          <div className="h-full flex flex-col overflow-hidden">
            <HD4TaskView 
              hd4Action={phase} 
              showCLI={showCLI}
              onToggleCLI={() => setShowCLI(!showCLI)}
            />
            {showCLI && (
              <div className="flex-none border-t border-gray-700" style={{ height: '40%', minHeight: '300px' }}>
                <MultiCLI 
                  currentHD4Phase={phase.toLowerCase() as 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate'} 
                  className="h-full"
                  defaultAgent="kali"
                />
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default HD4PhaseContent;