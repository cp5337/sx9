import React, { useState, useRef, useEffect } from 'react';
import { Terminal, Plus, X, Minimize2, Maximize2, Filter, Copy, Clipboard, Activity } from 'lucide-react';
import { GPTIcon, GrokIcon, GeminiIcon, KaliIcon, ClaudeIcon } from '@/components/icons/LLMIcons';
import { assembleSystemContext, formatSystemContextForPrompt } from '@/lib/services/system-context-provider';

type AgentType = 'natasha' | 'marcus' | 'elena' | 'cove' | 'kali';
type HD4Phase = 'hunt' | 'detect' | 'disrupt' | 'disable' | 'dominate' | 'all';

interface Agent {
  id: AgentType;
  name: string;
  llm: string;
  icon: React.ReactNode;
  color: string;
}

const AGENTS: Record<AgentType, Agent> = {
  natasha: { id: 'natasha', name: 'Natasha', llm: 'GPT-4', icon: <GPTIcon />, color: 'text-emerald-400' },
  marcus: { id: 'marcus', name: 'Marcus', llm: 'Gemini 2M', icon: <GeminiIcon />, color: 'text-blue-400' },
  elena: { id: 'elena', name: 'Elena', llm: 'Grok', icon: <GrokIcon />, color: 'text-purple-400' },
  cove: { id: 'cove', name: 'Cove', llm: 'Claude', icon: <ClaudeIcon />, color: 'text-orange-400' },
  kali: { id: 'kali', name: 'Kali ISO', llm: 'Terminal', icon: <KaliIcon />, color: 'text-green-400' }
};

interface CLISession {
  id: string;
  agent: AgentType;
  hd4Filter: HD4Phase;
  title: string;
  messages: Array<{
    id: string;
    type: 'user' | 'assistant' | 'system' | 'clipboard';
    content: string;
    timestamp: Date;
    hd4Phase?: HD4Phase;
  }>;
  input: string;
  isProcessing: boolean;
  backgroundTasks: Array<{
    id: string;
    name: string;
    status: 'running' | 'completed' | 'failed';
  }>;
}

interface MultiCLIProps {
  className?: string;
  currentHD4Phase?: HD4Phase;
  defaultAgent?: AgentType;
  showChat?: boolean;
  showTerminal?: boolean;
  terminalMode?: boolean;
}

const MultiCLI: React.FC<MultiCLIProps> = ({ 
  className = '', 
  currentHD4Phase = 'all',
  defaultAgent = 'natasha',
  showChat = true,
  showTerminal = true,
  terminalMode = false
}) => {
  const [sessions, setSessions] = useState<CLISession[]>([
    {
      id: '1',
      agent: defaultAgent,
      hd4Filter: currentHD4Phase,
      title: `${AGENTS[defaultAgent].name} (${AGENTS[defaultAgent].llm})`,
      messages: [],
      input: '',
      isProcessing: false,
      backgroundTasks: []
    }
  ]);
  const [activeSessionId, setActiveSessionId] = useState('1');
  const [isMinimized, setIsMinimized] = useState(false);
  const [atomicClipboard, setAtomicClipboard] = useState<string[]>([]);
  const [showAgentSelector, setShowAgentSelector] = useState(false);
  const [showHD4Filter, setShowHD4Filter] = useState(false);
  const [draggedAgent, setDraggedAgent] = useState<AgentType | null>(null);
  const [isDragOver, setIsDragOver] = useState(false);
  const chatRefs = useRef<{ [key: string]: HTMLDivElement | null }>({});

  const activeSession = sessions.find(s => s.id === activeSessionId);

  // Load atomic clipboard on mount
  useEffect(() => {
    loadAtomicClipboard();
    const interval = setInterval(loadAtomicClipboard, 5000); // Refresh every 5s
    return () => clearInterval(interval);
  }, []);

  const loadAtomicClipboard = async () => {
    try {
      const response = await fetch('http://localhost:15001/api/atomic-clipboard');
      if (response.ok) {
        const data = await response.json();
        setAtomicClipboard(data.items || []);
      }
    } catch (error) {
      console.log('Atomic clipboard service not available');
    }
  };

  const addToAtomicClipboard = async (text: string) => {
    try {
      await fetch('http://localhost:15001/api/atomic-clipboard', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, source: activeSession?.agent || 'system' })
      });
      loadAtomicClipboard();
    } catch (error) {
      console.error('Failed to add to atomic clipboard:', error);
    }
  };

  const addNewSession = (agent: AgentType) => {
    const newId = Date.now().toString();
    const agentInfo = AGENTS[agent];
    const newSession: CLISession = {
      id: newId,
      agent,
      hd4Filter: currentHD4Phase,
      title: `${agentInfo.name} (${agentInfo.llm})`,
      messages: [{
        id: `${newId}-welcome`,
        type: 'system',
        content: `${agentInfo.name} CLI initialized. ${agent === 'kali' ? 'Terminal ready for commands.' : 'Ask me anything about ' + currentHD4Phase.toUpperCase() + ' operations.'}`,
        timestamp: new Date()
      }],
      input: '',
      isProcessing: false,
      backgroundTasks: []
    };
    setSessions([...sessions, newSession]);
    setActiveSessionId(newId);
    setShowAgentSelector(false);
  };

  const closeSession = (id: string) => {
    const newSessions = sessions.filter(s => s.id !== id);
    if (newSessions.length === 0) {
      // Always keep at least one session
      addNewSession('natasha');
    } else {
      setSessions(newSessions);
      if (activeSessionId === id) {
        setActiveSessionId(newSessions[0].id);
      }
    }
  };

  const updateSessionInput = (id: string, input: string) => {
    setSessions(sessions.map(s => s.id === id ? { ...s, input } : s));
  };

  const updateHD4Filter = (sessionId: string, filter: HD4Phase) => {
    setSessions(sessions.map(s => 
      s.id === sessionId ? { ...s, hd4Filter: filter } : s
    ));
    setShowHD4Filter(false);
  };

  const sendMessage = async (sessionId: string) => {
    const session = sessions.find(s => s.id === sessionId);
    if (!session || !session.input.trim() || session.isProcessing) return;

    const userMessage = {
      id: Date.now().toString(),
      type: 'user' as const,
      content: session.input,
      timestamp: new Date(),
      hd4Phase: session.hd4Filter
    };

    // Add user message and set processing
    setSessions(sessions.map(s => 
      s.id === sessionId 
        ? { ...s, messages: [...s.messages, userMessage], input: '', isProcessing: true }
        : s
    ));

    // Add to atomic clipboard for cross-session context
    await addToAtomicClipboard(session.input);

    // Check if input is Atomic Cliport command
    const isCliportCommand = session.input.trim().startsWith('cliport:') || 
                            session.input.trim().startsWith('atomic:') ||
                            session.input.toLowerCase().includes('pick') ||
                            session.input.toLowerCase().includes('place') ||
                            session.input.toLowerCase().includes('grasp');

    // Call appropriate backend based on agent type
    try {
      const agentInfo = AGENTS[session.agent];
      let responseContent = '';

      if (isCliportCommand && !terminalMode) {
        // Process through Atomic Cliport with Thalamic Filter + DistilBERT
        const { processCliportCommand } = await import('../lib/services/atomic-cliport');
        
        const cliportCmd = {
          id: Date.now().toString(),
          type: session.input.includes('pick') ? 'pick' : 
                session.input.includes('place') ? 'place' :
                session.input.includes('grasp') ? 'grasp' : 'visual_query',
          description: session.input,
          metadata: {
            agent_id: session.agent,
            hd4_phase: session.hd4Filter,
          },
        };

        const cliportResult = await processCliportCommand(cliportCmd);
        
        responseContent = `[Atomic Cliport] ${cliportResult.result}\n` +
                         `Confidence: ${(cliportResult.confidence * 100).toFixed(1)}%\n` +
                         `Processing: ${cliportResult.processing_ms}ms\n` +
                         `Thalamic Gate: ${cliportResult.thalamic_output?.gate_decision || 'N/A'}\n` +
                         `Pathway: ${cliportResult.thalamic_output?.pathway || 'N/A'}\n` +
                         `Priority: ${cliportResult.thalamic_output?.priority || 'N/A'}`;
      } else if (session.agent === 'kali' || terminalMode) {
        // Kali ISO terminal execution
        const response = await fetch('http://localhost:18080/kali/execute', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ command: session.input, hd4Phase: session.hd4Filter })
        });
        const data = await response.json();
        responseContent = data.output || 'Command executed';
      } else {
        // Agent LLM execution with full system context
        // Assemble system context so LLM sees the entire system
        const systemContext = await assembleSystemContext(session.hd4Filter);
        const systemContextPrompt = formatSystemContextForPrompt(systemContext);
        
        const response = await fetch(`http://localhost:15180/agent/${session.agent}/chat`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ 
            message: session.input, 
            hd4Phase: session.hd4Filter,
            atomicClipboard: atomicClipboard.slice(-5), // Last 5 items for context
            systemContext: systemContextPrompt, // Full system context for LLM
            systemContextJson: systemContext // Also send as JSON for structured access
          })
        });
        const data = await response.json();
        responseContent = data.response || `${agentInfo.name} is processing your request...`;
      }

      const responseMessage = {
        id: (Date.now() + 1).toString(),
        type: 'assistant' as const,
        content: responseContent,
        timestamp: new Date(),
        hd4Phase: session.hd4Filter
      };

      setSessions(prev => prev.map(s => 
        s.id === sessionId 
          ? { ...s, messages: [...s.messages, responseMessage], isProcessing: false }
          : s
      ));
    } catch (error) {
      // Fallback to simulated response
      const responseMessage = {
        id: (Date.now() + 1).toString(),
        type: 'system' as const,
        content: `⚠️ ${AGENTS[session.agent].name} service not available. Simulated response: Processing "${session.input}" for ${session.hd4Filter.toUpperCase()} phase.`,
        timestamp: new Date()
      };

      setSessions(prev => prev.map(s => 
        s.id === sessionId 
          ? { ...s, messages: [...s.messages, responseMessage], isProcessing: false }
          : s
      ));
    }

    // Auto-scroll to bottom
    setTimeout(() => {
      if (chatRefs.current[sessionId]) {
        chatRefs.current[sessionId]!.scrollTop = chatRefs.current[sessionId]!.scrollHeight;
      }
    }, 100);
  };

  if (isMinimized) {
    return (
      <div className={`bg-gray-900 border-t border-gray-700 ${className}`}>
        <div className="flex items-center justify-between px-3 py-2">
          <div className="flex items-center gap-2 text-xs text-gray-400">
            <Activity size={14} />
            <span>{sessions.length} Agent{sessions.length > 1 ? 's' : ''} Active</span>
            {atomicClipboard.length > 0 && (
              <>
                <span className="text-gray-600">•</span>
                <Clipboard size={12} />
                <span>{atomicClipboard.length}</span>
              </>
            )}
          </div>
          <button
            onClick={() => setIsMinimized(false)}
            className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
            title="Maximize"
          >
            <Maximize2 size={14} />
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-gray-900 border-t border-gray-700 flex flex-col ${className}`}>
      {/* Tab Bar */}
      <div className="flex items-center bg-gray-800 border-b border-gray-700">
        {/* Session Tabs */}
        <div className="flex-1 flex overflow-x-auto">
          {sessions.map((session) => {
            const agent = AGENTS[session.agent];
            const hasBackgroundTasks = session.backgroundTasks.some(t => t.status === 'running');
            return (
              <div
                key={session.id}
                draggable={session.agent === 'kali'}
                onDragStart={(e) => {
                  setDraggedAgent(session.agent);
                  e.dataTransfer.effectAllowed = 'copy';
                  e.dataTransfer.setData('text/plain', session.agent);
                }}
                onDragEnd={() => {
                  setDraggedAgent(null);
                }}
                className={`flex items-center gap-2 px-3 py-1.5 border-r border-gray-700 cursor-pointer transition-colors ${
                  activeSessionId === session.id
                    ? 'bg-gray-900 text-white'
                    : 'bg-gray-800 text-gray-400 hover:bg-gray-750'
                } ${session.agent === 'kali' ? 'cursor-move' : ''}`}
                onClick={() => setActiveSessionId(session.id)}
                title={session.agent === 'kali' ? 'Drag to chat area to add new session' : ''}
              >
                <span className={agent.color}>
                  {agent.icon}
                </span>
                <span className="text-xs">{agent.name}</span>
                {session.hd4Filter !== 'all' && (
                  <span className="text-xxs px-1 py-0.5 bg-blue-900/50 text-blue-300 rounded">
                    {session.hd4Filter.substring(0, 3).toUpperCase()}
                  </span>
                )}
                {hasBackgroundTasks && (
                  <Activity size={10} className="text-yellow-400 animate-pulse" />
                )}
                {sessions.length > 1 && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      closeSession(session.id);
                    }}
                    className="p-0.5 hover:bg-gray-700 rounded"
                  >
                    <X size={10} />
                  </button>
                )}
              </div>
            );
          })}
        </div>

        {/* Controls */}
        <div className="flex items-center gap-1 px-2 border-l border-gray-700 relative">
          <button
            onClick={() => setShowAgentSelector(!showAgentSelector)}
            className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
            title="Add Agent"
          >
            <Plus size={14} />
          </button>
          {activeSession && (
            <button
              onClick={() => setShowHD4Filter(!showHD4Filter)}
              className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
              title="HD4 Filter"
            >
              <Filter size={14} />
            </button>
          )}
          {atomicClipboard.length > 0 && (
            <button
              className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
              title={`Atomic Clipboard (${atomicClipboard.length} items)`}
            >
              <Clipboard size={14} />
            </button>
          )}
          <button
            onClick={() => setIsMinimized(true)}
            className="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white"
            title="Minimize"
          >
            <Minimize2 size={14} />
          </button>

          {/* Agent Selector Dropdown */}
          {showAgentSelector && (
            <div className="absolute top-full right-0 mt-1 bg-gray-800 border border-gray-700 rounded shadow-lg z-50 min-w-[200px]">
              <div className="p-2 border-b border-gray-700 text-xs text-gray-400 font-semibold">
                Select Agent
              </div>
              {Object.values(AGENTS).map((agent) => (
                <button
                  key={agent.id}
                  onClick={() => addNewSession(agent.id)}
                  className="w-full flex items-center gap-2 px-3 py-2 text-xs hover:bg-gray-700 transition-colors"
                >
                  <span className={agent.color}>{agent.icon}</span>
                  <div className="flex-1 text-left">
                    <div className="text-white">{agent.name}</div>
                    <div className="text-xxs text-gray-400">{agent.llm}</div>
                  </div>
                </button>
              ))}
            </div>
          )}

          {/* HD4 Filter Dropdown */}
          {showHD4Filter && activeSession && (
            <div className="absolute top-full right-0 mt-1 bg-gray-800 border border-gray-700 rounded shadow-lg z-50 min-w-[150px]">
              <div className="p-2 border-b border-gray-700 text-xs text-gray-400 font-semibold">
                HD4 Phase Filter
              </div>
              {(['all', 'hunt', 'detect', 'disrupt', 'disable', 'dominate'] as HD4Phase[]).map((phase) => (
                <button
                  key={phase}
                  onClick={() => updateHD4Filter(activeSession.id, phase)}
                  className={`w-full px-3 py-1.5 text-xs text-left hover:bg-gray-700 transition-colors ${
                    activeSession.hd4Filter === phase ? 'bg-blue-900/50 text-blue-300' : 'text-gray-300'
                  }`}
                >
                  {phase.toUpperCase()}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Active Session Content */}
      {activeSession && (
        <div className="flex-1 flex flex-col min-h-0">
          {/* Session Info Bar */}
          <div className="flex items-center justify-between px-3 py-1 bg-gray-850 border-b border-gray-700 text-xxs">
            <div className="flex items-center gap-2">
              <span className={AGENTS[activeSession.agent].color}>
                {AGENTS[activeSession.agent].icon}
              </span>
              <span className="text-gray-400">
                {AGENTS[activeSession.agent].name} • {AGENTS[activeSession.agent].llm}
              </span>
              {activeSession.hd4Filter !== 'all' && (
                <>
                  <span className="text-gray-600">•</span>
                  <Filter size={10} className="text-blue-400" />
                  <span className="text-blue-300">{activeSession.hd4Filter.toUpperCase()}</span>
                </>
              )}
            </div>
            <div className="flex items-center gap-2">
              {activeSession.backgroundTasks.length > 0 && (
                <span className="text-yellow-400">
                  {activeSession.backgroundTasks.filter(t => t.status === 'running').length} tasks running
                </span>
              )}
            </div>
          </div>

          {/* Messages */}
          <div 
            ref={(el) => chatRefs.current[activeSession.id] = el}
            className={`flex-1 overflow-y-auto p-3 space-y-2 relative ${
              isDragOver ? 'bg-blue-900/20 border-2 border-dashed border-blue-500' : ''
            }`}
            onDragOver={(e) => {
              e.preventDefault();
              e.stopPropagation();
              if (draggedAgent === 'kali') {
                e.dataTransfer.dropEffect = 'copy';
                setIsDragOver(true);
              }
            }}
            onDragLeave={(e) => {
              e.preventDefault();
              e.stopPropagation();
              setIsDragOver(false);
            }}
            onDrop={(e) => {
              e.preventDefault();
              e.stopPropagation();
              setIsDragOver(false);
              
              const agentType = e.dataTransfer.getData('text/plain') as AgentType;
              if (agentType === 'kali') {
                // Add new Kali session
                addNewSession('kali');
              }
            }}
          >
            {isDragOver && (
              <div className="absolute inset-0 flex items-center justify-center bg-blue-900/30 z-10 pointer-events-none">
                <div className="text-center">
                  <div className="text-blue-400 text-lg mb-2">🐉</div>
                  <div className="text-blue-300 text-sm font-semibold">Drop to add Kali session</div>
                </div>
              </div>
            )}
            {activeSession.messages.length === 0 ? (
              <div className="text-center text-gray-500 text-xs py-8">
                {activeSession.agent === 'kali' 
                  ? '🐉 Kali ISO Terminal ready. Enter commands...'
                  : `🤖 ${AGENTS[activeSession.agent].name} ready. Ask anything about ${activeSession.hd4Filter.toUpperCase()} operations...`}
              </div>
            ) : (
              activeSession.messages.map((message) => {
                const isKali = activeSession.agent === 'kali';
                return (
                  <div
                    key={message.id}
                    className={`flex ${message.type === 'user' ? 'justify-end' : 'justify-start'}`}
                  >
                    <div
                      className={`max-w-[80%] px-3 py-1.5 rounded text-xs ${
                        message.type === 'user'
                          ? 'bg-blue-600 text-white'
                          : message.type === 'system'
                          ? 'bg-yellow-900/30 text-yellow-300 border border-yellow-700/50'
                          : message.type === 'clipboard'
                          ? 'bg-purple-900/30 text-purple-300 border border-purple-700/50'
                          : isKali
                          ? 'bg-gray-800 text-green-400 font-mono border border-green-900/50'
                          : 'bg-gray-800 text-gray-300'
                      }`}
                    >
                      <div className="whitespace-pre-wrap break-words">{message.content}</div>
                      <div className="text-xxs opacity-70 mt-1 flex items-center justify-between">
                        <span>{message.timestamp.toLocaleTimeString()}</span>
                        {message.hd4Phase && message.hd4Phase !== 'all' && (
                          <span className="ml-2 px-1 py-0.5 bg-blue-900/50 text-blue-300 rounded">
                            {message.hd4Phase.substring(0, 3).toUpperCase()}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })
            )}
            {activeSession.isProcessing && (
              <div className="flex justify-start">
                <div className="bg-gray-800 px-3 py-2 rounded text-xs text-gray-400">
                  <div className="flex items-center gap-2">
                    <div className="animate-spin rounded-full h-3 w-3 border-b-2 border-blue-500"></div>
                    <span>{AGENTS[activeSession.agent].name} is thinking...</span>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Input */}
          <div className="border-t border-gray-700 p-2 flex gap-2">
            <input
              type="text"
              value={activeSession.input}
              onChange={(e) => updateSessionInput(activeSession.id, e.target.value)}
              onKeyPress={(e) => {
                if (e.key === 'Enter' && !activeSession.isProcessing) {
                  sendMessage(activeSession.id);
                }
              }}
              placeholder={
                activeSession.agent === 'kali' 
                  ? `$ Enter Kali command (${activeSession.hd4Filter})...` 
                  : `Ask ${AGENTS[activeSession.agent].name} about ${activeSession.hd4Filter}...`
              }
              disabled={activeSession.isProcessing}
              className={`flex-1 px-3 py-1.5 text-xs bg-gray-800 border border-gray-700 rounded focus:outline-none focus:border-blue-600 disabled:opacity-50 ${
                activeSession.agent === 'kali' ? 'font-mono text-green-400' : 'text-white'
              }`}
            />
            <button
              onClick={() => sendMessage(activeSession.id)}
              disabled={!activeSession.input.trim() || activeSession.isProcessing}
              className="px-3 py-1.5 bg-blue-600 text-white rounded text-xs hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed transition-colors"
            >
              {activeSession.isProcessing ? '...' : 'Send'}
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default MultiCLI;

