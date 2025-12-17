import React, { useState, useRef } from "react";
import { Terminal, Plus, X, Activity } from "lucide-react";
import { Bot } from "lucide-react";

// Using the same icons approach or falling back to Bot if missing
const GPTIcon = () => <Bot className="text-emerald-400" />;
const GeminiIcon = () => <Bot className="text-blue-400" />;
const GrokIcon = () => <Bot className="text-purple-400" />;
const ClaudeIcon = () => <Bot className="text-orange-400" />;
const KaliIcon = () => <Terminal className="text-green-400" />;

type AgentType = "natasha" | "marcus" | "elena" | "cove" | "kali";
type HD4Phase = "hunt" | "detect" | "disrupt" | "disable" | "dominate" | "all";

interface Agent {
  id: AgentType;
  name: string;
  llm: string;
  icon: React.ReactNode;
  color: string;
}

const AGENTS: Record<AgentType, Agent> = {
  natasha: {
    id: "natasha",
    name: "Natasha",
    llm: "GPT-4",
    icon: <GPTIcon />,
    color: "text-emerald-400",
  },
  marcus: {
    id: "marcus",
    name: "Marcus",
    llm: "Gemini 2M",
    icon: <GeminiIcon />,
    color: "text-blue-400",
  },
  elena: {
    id: "elena",
    name: "Elena",
    llm: "Grok",
    icon: <GrokIcon />,
    color: "text-purple-400",
  },
  cove: {
    id: "cove",
    name: "Cove",
    llm: "Claude",
    icon: <ClaudeIcon />,
    color: "text-orange-400",
  },
  kali: {
    id: "kali",
    name: "Kali ISO",
    llm: "Terminal",
    icon: <KaliIcon />,
    color: "text-green-400",
  },
};

interface CLISession {
  id: string;
  agent: AgentType;
  hd4Filter: HD4Phase;
  title: string;
  messages: Array<{
    id: string;
    type: "user" | "assistant" | "system" | "clipboard";
    content: string;
    timestamp: Date;
    hd4Phase?: HD4Phase;
  }>;
  input: string;
  isProcessing: boolean;
  backgroundTasks: Array<{
    id: string;
    name: string;
    status: "running" | "completed" | "failed";
  }>;
}

interface MultiWindowChatProps {
  className?: string;
  currentHD4Phase?: HD4Phase;
  defaultAgent?: AgentType;
  onClose?: () => void;
}

const MultiWindowChat: React.FC<MultiWindowChatProps> = ({
  className = "",
  currentHD4Phase = "all",
  defaultAgent = "natasha",
  onClose,
}) => {
  const [sessions, setSessions] = useState<CLISession[]>([
    {
      id: "1",
      agent: defaultAgent,
      hd4Filter: currentHD4Phase,
      title: `${AGENTS[defaultAgent].name} (${AGENTS[defaultAgent].llm})`,
      messages: [
        {
          id: "1-welcome",
          type: "system",
          content: `${AGENTS[defaultAgent].name} initialized in split-view mode.`,
          timestamp: new Date(),
        },
      ],
      input: "",
      isProcessing: false,
      backgroundTasks: [],
    },
  ]);

  const [showAgentSelector, setShowAgentSelector] = useState(false);
  const scrollRefs = useRef<{ [key: string]: HTMLDivElement | null }>({});

  const addNewSession = (agent: AgentType) => {
    const newId = Date.now().toString();
    const agentInfo = AGENTS[agent];
    const newSession: CLISession = {
      id: newId,
      agent,
      hd4Filter: currentHD4Phase,
      title: `${agentInfo.name} (${agentInfo.llm})`,
      messages: [
        {
          id: `${newId}-welcome`,
          type: "system",
          content: `${agentInfo.name} joined the workspace.`,
          timestamp: new Date(),
        },
      ],
      input: "",
      isProcessing: false,
      backgroundTasks: [],
    };
    setSessions([...sessions, newSession]);
    setShowAgentSelector(false);
  };

  const closeSession = (id: string) => {
    if (sessions.length <= 1) return; // Prevent closing the last session
    setSessions(sessions.filter((s) => s.id !== id));
  };

  const sendMessage = async (sessionId: string) => {
    const session = sessions.find((s) => s.id === sessionId);
    if (!session || !session.input.trim() || session.isProcessing) return;

    const userMessage = {
      id: Date.now().toString(),
      type: "user" as const,
      content: session.input,
      timestamp: new Date(),
      hd4Phase: session.hd4Filter,
    };

    // Update session with user message
    setSessions((prev) =>
      prev.map((s) =>
        s.id === sessionId
          ? {
              ...s,
              messages: [...s.messages, userMessage],
              input: "",
              isProcessing: true,
            }
          : s
      )
    );

    // Simulate response
    setTimeout(() => {
      const responseMessage = {
        id: (Date.now() + 1).toString(),
        type: "assistant" as const,
        content: `[${AGENTS[session.agent].name}] Processed: ${session.input}`,
        timestamp: new Date(),
        hd4Phase: session.hd4Filter,
      };

      setSessions((prev) =>
        prev.map((s) =>
          s.id === sessionId
            ? {
                ...s,
                messages: [...s.messages, responseMessage],
                isProcessing: false,
              }
            : s
        )
      );

      // Auto-scroll
      if (scrollRefs.current[sessionId]) {
        scrollRefs.current[sessionId]!.scrollTop =
          scrollRefs.current[sessionId]!.scrollHeight;
      }
    }, 1000);
  };

  const updateSessionInput = (id: string, val: string) => {
    setSessions(sessions.map((s) => (s.id === id ? { ...s, input: val } : s)));
  };

  return (
    <div
      className={`flex flex-col h-full bg-gray-900 border-t border-gray-700 ${className}`}
    >
      {/* Header / Controls */}
      <div className="flex items-center justify-between px-2 py-1 bg-gray-800 border-b border-gray-700">
        <div className="flex items-center gap-2 text-xs text-gray-400">
          <Activity size={14} />
          <span>{sessions.length} Active Contexts</span>
        </div>
        <div className="flex items-center gap-2 relative">
          <button
            onClick={() => setShowAgentSelector(!showAgentSelector)}
            className="flex items-center gap-1 px-2 py-1 bg-blue-600 hover:bg-blue-500 text-white rounded text-xs transition-colors"
          >
            <Plus size={12} />
            Add Context
          </button>

          {onClose && (
            <button
              onClick={onClose}
              className="flex items-center justify-center p-1 hover:bg-red-500/20 text-gray-400 hover:text-red-400 rounded transition-colors"
              title="Close Chat Panel"
            >
              <X size={16} />
            </button>
          )}

          {showAgentSelector && (
            <div className="absolute top-full right-0 mt-1 w-48 bg-gray-800 border border-gray-700 rounded shadow-xl z-50">
              <div className="p-2 border-b border-gray-700 text-xs text-gray-400 font-semibold">
                Deploy Agent
              </div>
              {Object.values(AGENTS).map((agent) => (
                <button
                  key={agent.id}
                  onClick={() => addNewSession(agent.id)}
                  className="w-full flex items-center gap-2 px-3 py-2 text-xs hover:bg-gray-700 transition-colors text-left"
                >
                  <span className={agent.color}>{agent.icon}</span>
                  <span className="text-gray-200">{agent.name}</span>
                </button>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Tiled Content - Robust Flex Layout */}
      <div className="flex-1 p-2 gap-3 flex flex-row overflow-x-auto">
        {sessions.map((session) => (
          <div
            key={session.id}
            className="flex flex-col bg-gray-850 border border-gray-700 rounded-lg overflow-hidden shadow-lg flex-shrink-0 w-1/2 min-w-[400px]"
          >
            {/* Session Header */}
            <div className="flex items-center justify-between px-3 py-2 bg-gray-800 border-b border-gray-700">
              <div className="flex items-center gap-2">
                <span className={AGENTS[session.agent].color}>
                  {AGENTS[session.agent].icon}
                </span>
                <span className="text-xs font-semibold text-gray-200">
                  {session.title}
                </span>
              </div>
              {sessions.length > 1 && (
                <button
                  onClick={() => closeSession(session.id)}
                  className="text-gray-500 hover:text-white"
                >
                  <X size={12} />
                </button>
              )}
            </div>

            {/* Messages Area */}
            <div
              className="flex-1 overflow-y-auto p-3 space-y-2 bg-gray-900/50"
              ref={(el) => (scrollRefs.current[session.id] = el)}
            >
              {session.messages.map((msg) => (
                <div
                  key={msg.id}
                  className={`flex ${
                    msg.type === "user" ? "justify-end" : "justify-start"
                  }`}
                >
                  <div
                    className={`max-w-[85%] px-2 py-1.5 rounded text-xs ${
                      msg.type === "user"
                        ? "bg-blue-600/20 text-blue-200 border border-blue-500/30"
                        : msg.type === "system"
                        ? "bg-yellow-900/20 text-yellow-300/80 border border-yellow-700/30 italic"
                        : "bg-gray-800 text-gray-300 border border-gray-700"
                    }`}
                  >
                    {msg.content}
                  </div>
                </div>
              ))}
              {session.isProcessing && (
                <div className="flex justify-start">
                  <div className="px-2 py-1 bg-gray-800 rounded text-xs text-gray-400 animate-pulse">
                    ...
                  </div>
                </div>
              )}
            </div>

            {/* Input Area */}
            <div className="p-2 border-t border-gray-700 bg-gray-800">
              <div className="flex gap-2">
                <input
                  className="flex-1 bg-gray-900 border border-gray-600 rounded px-2 py-1 text-xs text-white focus:border-blue-500 focus:outline-none"
                  placeholder="Command..."
                  value={session.input}
                  onChange={(e) =>
                    updateSessionInput(session.id, e.target.value)
                  }
                  onKeyPress={(e) =>
                    e.key === "Enter" && sendMessage(session.id)
                  }
                />
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default MultiWindowChat;
