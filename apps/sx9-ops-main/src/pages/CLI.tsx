import React, { useState } from 'react';
import {
  Brain, Mic, Volume2, Cpu, Server, Activity, RefreshCw,
  Database, Network, Settings, ExternalLink, Box
} from 'lucide-react';

// Port configurations
const PORTS = {
  ELITE_AGENTS: { GROK: 50051, NATASHA: 50052, COVE: 50053, ALTAIR: 50054, CLAUDE: 50055, ZOE: 50056, GPT: 50057, ELENA: 50058 },
  MEMORY_MESH: { CONTEXT: 19011, CLIPBOARD: 19012, THALAMIC: 19013, SLEDIS: 19014, VOICE: 19015, SHUTTLE: 19016 },
  INFERENCE: { OLLAMA: 11434, HUGGINGFACE: 8080, THALAMIC: 18116, PHI3: 18118, CHROMADB: 8000, EMBEDDING: 18117 },
} as const;

type Status = 'online' | 'offline' | 'loading';

interface Agent { id: string; name: string; llm: string; port: number; role: 'ops' | 'dev'; voice?: string; }
interface InferenceProvider { id: string; name: string; endpoint: string; models: string[]; status: Status; type: 'local' | 'api'; }

const AgentVoiceRegistry: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'agents' | 'inference' | 'voice' | 'mesh'>('agents');
  const [refreshing, setRefreshing] = useState(false);

  const agents: Agent[] = [
    { id: 'grok', name: 'Grok', llm: 'xAI Grok-2', port: PORTS.ELITE_AGENTS.GROK, role: 'ops', voice: 'adam' },
    { id: 'natasha', name: 'Natasha', llm: 'GPT-4 Turbo', port: PORTS.ELITE_AGENTS.NATASHA, role: 'ops', voice: 'rachel' },
    { id: 'cove', name: 'Cove', llm: 'Claude Opus 4', port: PORTS.ELITE_AGENTS.COVE, role: 'dev', voice: 'josh' },
    { id: 'altair', name: 'Altair', llm: 'Gemini 2.0', port: PORTS.ELITE_AGENTS.ALTAIR, role: 'ops', voice: 'antoni' },
    { id: 'claude', name: 'Claude', llm: 'Claude Sonnet 4', port: PORTS.ELITE_AGENTS.CLAUDE, role: 'dev', voice: 'arnold' },
    { id: 'zoe', name: 'Zoe', llm: 'Ollama/PHI-3', port: PORTS.ELITE_AGENTS.ZOE, role: 'ops', voice: 'bella' },
    { id: 'marcus', name: 'Marcus', llm: 'GPT-4o', port: PORTS.ELITE_AGENTS.GPT, role: 'dev', voice: 'sam' },
    { id: 'elena', name: 'Elena', llm: 'Grok-2 Vision', port: PORTS.ELITE_AGENTS.ELENA, role: 'ops', voice: 'elli' },
  ];

  const inferenceProviders: InferenceProvider[] = [
    {
      id: 'ollama', name: 'Ollama', endpoint: `http://localhost:${PORTS.INFERENCE.OLLAMA}`, type: 'local', status: 'online',
      models: ['llama3.2', 'phi3', 'mistral', 'codellama', 'qwen2.5', 'deepseek-r1']
    },
    {
      id: 'huggingface', name: 'Hugging Face TGI', endpoint: `http://localhost:${PORTS.INFERENCE.HUGGINGFACE}`, type: 'local', status: 'offline',
      models: ['mistralai/Mistral-7B', 'meta-llama/Llama-3-8B', 'microsoft/phi-3-mini']
    },
    {
      id: 'thalamic', name: 'Thalamic Filter', endpoint: `http://localhost:${PORTS.INFERENCE.THALAMIC}`, type: 'local', status: 'online',
      models: ['distilbert-lora (NLM→Lisp)']
    },
    {
      id: 'chromadb', name: 'ChromaDB + Embeddings', endpoint: `http://localhost:${PORTS.INFERENCE.CHROMADB}`, type: 'local', status: 'online',
      models: ['all-MiniLM-L6-v2 (384-dim)']
    },
  ];

  const voiceConfigs = [
    { provider: 'ElevenLabs', type: 'TTS+STT', endpoint: 'api.elevenlabs.io', status: 'online' as Status, voices: ['rachel', 'adam', 'bella', 'josh', 'arnold', 'elli', 'sam', 'antoni'] },
    { provider: 'OpenAI Whisper', type: 'STT', endpoint: 'api.openai.com', status: 'online' as Status, voices: [] },
    { provider: 'Piper (Local)', type: 'TTS', endpoint: 'localhost:18121', status: 'offline' as Status, voices: ['lessac'] },
  ];

  const meshServices = [
    { name: 'Context Mesh', port: PORTS.MEMORY_MESH.CONTEXT, desc: 'Cross-agent context' },
    { name: 'Atomic Clipboard', port: PORTS.MEMORY_MESH.CLIPBOARD, desc: 'Shared memory buffer' },
    { name: 'Thalamic Filter', port: PORTS.MEMORY_MESH.THALAMIC, desc: 'DistilBERT NLM→Lisp' },
    { name: 'Sledis Cache', port: PORTS.MEMORY_MESH.SLEDIS, desc: 'KV cache layer' },
    { name: 'Voice Gateway', port: PORTS.MEMORY_MESH.VOICE, desc: 'ElevenLabs routing' },
    { name: 'Shuttle Sync', port: PORTS.MEMORY_MESH.SHUTTLE, desc: 'State sync bus' },
  ];

  const StatusDot: React.FC<{ status: Status }> = ({ status }) => (
    <span className={`inline-block w-2 h-2 rounded-full ${status === 'online' ? 'bg-green-500' : status === 'loading' ? 'bg-yellow-500' : 'bg-gray-400'}`} />
  );

  const tabs = [
    { id: 'agents', label: 'Agents', icon: <Cpu size={14} /> },
    { id: 'inference', label: 'Inference', icon: <Box size={14} /> },
    { id: 'voice', label: 'Voice', icon: <Mic size={14} /> },
    { id: 'mesh', label: 'Mesh', icon: <Network size={14} /> },
  ];

  return (
    <div className="h-full bg-gray-900 text-gray-100 flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
        <div className="flex items-center gap-2">
          <Brain size={18} className="text-blue-400" />
          <span className="font-semibold">Agent & Voice Registry</span>
        </div>
        <button onClick={() => { setRefreshing(true); setTimeout(() => setRefreshing(false), 500); }}
          className={`p-1.5 rounded hover:bg-gray-700 ${refreshing ? 'animate-spin' : ''}`}>
          <RefreshCw size={14} />
        </button>
      </div>

      {/* Tabs */}
      <div className="flex gap-1 px-4 py-2 border-b border-gray-700 bg-gray-800/50">
        {tabs.map(t => (
          <button key={t.id} onClick={() => setActiveTab(t.id as typeof activeTab)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium transition-colors
              ${activeTab === t.id ? 'bg-blue-600 text-white' : 'text-gray-400 hover:text-white hover:bg-gray-700'}`}>
            {t.icon} {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-4">
        {/* Agents Tab */}
        {activeTab === 'agents' && (
          <div className="space-y-3">
            <div className="text-xs text-gray-400 mb-2">8 Elite Agents (ports 50051-50058) | Dual-role: OPS / DEV</div>
            <div className="grid grid-cols-2 lg:grid-cols-4 gap-2">
              {agents.map(a => (
                <div key={a.id} className="bg-gray-800 rounded p-3 border-l-2 border-blue-500">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-medium text-sm">{a.name}</span>
                    <span className={`text-[10px] px-1.5 py-0.5 rounded font-bold ${a.role === 'ops' ? 'bg-red-900/50 text-red-300' : 'bg-blue-900/50 text-blue-300'}`}>
                      {a.role.toUpperCase()}
                    </span>
                  </div>
                  <div className="text-xs text-gray-400 space-y-0.5">
                    <div className="flex justify-between"><span>LLM</span><span className="text-gray-300">{a.llm}</span></div>
                    <div className="flex justify-between"><span>Port</span><span className="font-mono text-gray-300">{a.port}</span></div>
                    {a.voice && <div className="flex justify-between"><span>Voice</span><span className="text-gray-300">{a.voice}</span></div>}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Inference Tab */}
        {activeTab === 'inference' && (
          <div className="space-y-3">
            <div className="text-xs text-gray-400 mb-2">Local inference with Ollama & Hugging Face TGI</div>

            {inferenceProviders.map(p => (
              <div key={p.id} className="bg-gray-800 rounded p-3">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <StatusDot status={p.status} />
                    <span className="font-medium text-sm">{p.name}</span>
                    <span className="text-[10px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-300">{p.type.toUpperCase()}</span>
                  </div>
                  <code className="text-xs text-gray-500">{p.endpoint}</code>
                </div>
                <div className="flex flex-wrap gap-1">
                  {p.models.map((m, i) => (
                    <span key={i} className="text-[10px] px-2 py-0.5 rounded bg-gray-700 text-gray-300">{m}</span>
                  ))}
                </div>
                {p.id === 'ollama' && (
                  <div className="mt-2 p-2 bg-gray-900 rounded text-xs text-gray-400">
                    <code>ollama run llama3.2</code> | <code>ollama run phi3</code> | <code>ollama list</code>
                  </div>
                )}
                {p.id === 'huggingface' && (
                  <div className="mt-2 p-2 bg-gray-900 rounded text-xs text-gray-400">
                    <code>docker run -p 8080:80 ghcr.io/huggingface/text-generation-inference</code>
                  </div>
                )}
              </div>
            ))}

            <div className="bg-gray-800 rounded p-3 mt-4">
              <div className="font-medium text-sm mb-2 flex items-center gap-2">
                <Settings size={14} /> Quick Setup
              </div>
              <div className="grid grid-cols-2 gap-2 text-xs">
                <div className="bg-gray-900 p-2 rounded">
                  <div className="text-gray-300 font-medium mb-1">Ollama</div>
                  <code className="text-gray-500 block">brew install ollama</code>
                  <code className="text-gray-500 block">ollama serve</code>
                </div>
                <div className="bg-gray-900 p-2 rounded">
                  <div className="text-gray-300 font-medium mb-1">HuggingFace</div>
                  <code className="text-gray-500 block">pip install text-generation</code>
                  <code className="text-gray-500 block">text-generation-server</code>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Voice Tab */}
        {activeTab === 'voice' && (
          <div className="space-y-3">
            <div className="text-xs text-gray-400 mb-2">ElevenLabs primary | Whisper STT | Local Piper fallback</div>

            {voiceConfigs.map((v, i) => (
              <div key={i} className="bg-gray-800 rounded p-3">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <StatusDot status={v.status} />
                    <span className="font-medium text-sm">{v.provider}</span>
                    <span className="text-[10px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-300">{v.type}</span>
                  </div>
                  <code className="text-xs text-gray-500">{v.endpoint}</code>
                </div>
                {v.voices.length > 0 && (
                  <div className="flex flex-wrap gap-1">
                    {v.voices.map((voice, j) => (
                      <span key={j} className="text-[10px] px-2 py-0.5 rounded bg-gray-700 text-gray-300">{voice}</span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {/* Mesh Tab */}
        {activeTab === 'mesh' && (
          <div className="space-y-3">
            <div className="text-xs text-gray-400 mb-2">Memory Mesh (ports 19011-19016) | Cross-agent state sync</div>

            <div className="grid grid-cols-2 lg:grid-cols-3 gap-2">
              {meshServices.map(s => (
                <div key={s.port} className="bg-gray-800 rounded p-2">
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-sm font-medium">{s.name}</span>
                    <span className="font-mono text-xs text-gray-500">:{s.port}</span>
                  </div>
                  <div className="text-xs text-gray-400">{s.desc}</div>
                </div>
              ))}
            </div>

            <div className="bg-gray-800 rounded p-3 mt-2">
              <div className="font-mono text-[10px] text-gray-500 leading-relaxed whitespace-pre">
{`┌─────────────────────────────────────────────┐
│          MEMORY MESH (19011-19016)          │
├─────────────────────────────────────────────┤
│  Context ─── Clipboard ─── Thalamic         │
│  :19011      :19012        :19013           │
│      └──────────┬──────────┘                │
│  Sledis ─── Voice GW ─── Shuttle            │
│  :19014     :19015        :19016            │
│                 │                           │
│           [ElevenLabs]                      │
└─────────────────────────────────────────────┘`}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default AgentVoiceRegistry;
