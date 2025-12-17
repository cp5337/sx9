import { useState, useRef, useEffect } from "react";
import { Send, X } from "lucide-react";

interface Message {
  role: "user" | "agent";
  content: string;
  timestamp: number;
}

interface AgentChatPanelProps {
  agentId: string;
  agentName: string;
  messages: Message[];
  onSendMessage: (message: string) => void;
  onClose: () => void;
}

export function AgentChatPanel({
  agentId,
  agentName,
  messages,
  onSendMessage,
  onClose,
}: AgentChatPanelProps) {
  const [input, setInput] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (input.trim()) {
      onSendMessage(input);
      setInput("");
    }
  };

  return (
    <div className="flex flex-col h-full border-r border-gray-700 bg-gray-900">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 bg-gray-800 border-b border-gray-700">
        <span className="text-xs font-semibold text-white">{agentName}</span>
        <button
          onClick={onClose}
          className="text-gray-400 hover:text-white transition-colors"
          title="Close chat"
        >
          <X className="w-3 h-3" />
        </button>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-3 space-y-2">
        {messages.length === 0 ? (
          <div className="text-xs text-gray-500 text-center mt-4">
            Start a conversation with {agentName}
          </div>
        ) : (
          messages.map((msg, idx) => (
            <div
              key={idx}
              className={`text-xs ${
                msg.role === "user"
                  ? "text-blue-400 text-right"
                  : "text-gray-300 text-left"
              }`}
            >
              <div
                className={`inline-block px-2 py-1 rounded ${
                  msg.role === "user" ? "bg-blue-900/50" : "bg-gray-800"
                }`}
              >
                {msg.content}
              </div>
            </div>
          ))
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <form onSubmit={handleSubmit} className="p-2 border-t border-gray-700">
        <div className="flex gap-1">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder={`Ask ${agentName}...`}
            className="flex-1 px-2 py-1 text-xs bg-gray-800 border border-gray-700 rounded text-gray-200 placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
          />
          <button
            type="submit"
            disabled={!input.trim()}
            className="px-2 py-1 bg-blue-600 text-white text-xs rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Send className="w-3 h-3" />
          </button>
        </div>
      </form>
    </div>
  );
}
