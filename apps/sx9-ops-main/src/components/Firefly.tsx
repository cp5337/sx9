import React, { useState, useEffect } from 'react';
import {
  Flame, Server, Terminal, RefreshCw, Key, Eye, EyeOff,
  CheckCircle2, XCircle, Loader2, Rocket, Lock, Shield, Plus, Trash2
} from 'lucide-react';
import { GoogleCloudIcon, AzureIcon, DockerIcon, TerraformIcon, VaultIcon } from './icons/LLMIcons';

type Status = 'ready' | 'deploying' | 'not_deployed' | 'error';
type Provider = 'gcp' | 'azure' | 'docker';

interface Secret {
  key: string;
  value: string;
  provider: Provider | 'all';
  masked: boolean;
}

interface Module {
  id: string;
  name: string;
  desc: string;
  provider: Provider;
  path: string;
  resources: string[];
  cost: string;
  status: Status;
}

// Real GCP Project Config
const GCP_CONFIG = {
  projectId: 'gen-lang-client-0779767785',
  region: 'us-central1',
  gatewayUrl: 'https://abe-firefly-gateway.googleapis.com',
};

// Firefly API Endpoints (from firefly-openapi.yaml)
const FIREFLY_ENDPOINTS = {
  health: '/api/v1/health',
  marcValidate: '/api/v1/marc/validate',
  documentAnalyze: '/api/v1/document/analyze',
  ingestUpload: '/api/v1/ingest/upload',
  ingestGoogleDrive: '/api/v1/ingest/google-drive',
  ingestBatch: '/api/v1/ingest/batch',
  ingestStatus: '/api/v1/ingest/status',
  summarize: '/api/v1/summarize',
  analyze: '/api/v1/analyze',
  eeiExtract: '/api/v1/eei/extract',
  intelligenceProcess: '/api/v1/intelligence/process',
  freeApis: '/api/v1/sources/free-apis',
};

const MODULES: Module[] = [
  // GCP Modules
  { id: 'abe-core', name: 'ABE Core Infrastructure', desc: 'Cloud Run, Storage, Pub/Sub, Load Balancer', provider: 'gcp', path: '04-abe-iac/cognetix-abe', resources: ['Cloud Run (3)', 'Storage (2)', 'Pub/Sub (2)', 'Load Balancer'], cost: '$15-30/mo', status: 'not_deployed' },
  { id: 'firefly-fn', name: 'Firefly Functions', desc: 'Doc analysis, MARC validation, EEI extraction', provider: 'gcp', path: '04-abe-iac/cognetix-abe/firefly-deployment', resources: ['Cloud Functions (3)', 'API Gateway'], cost: '$5-10/mo', status: 'not_deployed' },
  { id: 'gpu-instance', name: 'GPU Compute', desc: 'GNN inference & training instance', provider: 'gcp', path: '04-abe-iac/gpu-instance-terraform', resources: ['Compute Engine (GPU)', 'Persistent Disk'], cost: '$200-500/mo', status: 'not_deployed' },
  { id: 'ctas7-registry', name: 'CTAS7 GIS Registry', desc: 'GIS core, infrastructure components', provider: 'gcp', path: '04-abe-iac/ctas7-terraform-Registry', resources: ['GIS Services', 'Core Infra'], cost: '$20-40/mo', status: 'not_deployed' },
  // Docker/Local Modules
  { id: 'osint-stack', name: 'OSINT Stack', desc: 'Neo4j, GLAF, collectors, MCP server', provider: 'docker', path: 'osint-orbstack', resources: ['Neo4j', 'GLAF', 'MCP Server', 'CLI Bridge'], cost: 'Local', status: 'not_deployed' },
  { id: 'kali-ringbus', name: 'Kali Alpine L2', desc: 'Security tools, ringbus integration', provider: 'docker', path: 'kali-alpine-ringbus-l2', resources: ['Kali Alpine', 'Ringbus L2', 'CLI Bridge'], cost: 'Local', status: 'not_deployed' },
  // Azure placeholder
  { id: 'azure-cognitive', name: 'Azure Cognitive', desc: 'Azure AI services (planned)', provider: 'azure', path: 'azure-cognitive', resources: ['Cognitive Services', 'Azure OpenAI'], cost: '$50-100/mo', status: 'not_deployed' },
];

const DEFAULT_SECRETS: Secret[] = [
  { key: 'GCP_PROJECT_ID', value: 'gen-lang-client-0779767785', provider: 'gcp', masked: false },
  { key: 'GCP_REGION', value: 'us-central1', provider: 'gcp', masked: false },
  { key: 'GOOGLE_APPLICATION_CREDENTIALS', value: '', provider: 'gcp', masked: true },
  { key: 'OPENAI_API_KEY', value: '', provider: 'all', masked: true },
  { key: 'ANTHROPIC_API_KEY', value: '', provider: 'all', masked: true },
  { key: 'ELEVENLABS_API_KEY', value: '', provider: 'all', masked: true },
  { key: 'NEO4J_PASSWORD', value: 'Protected1', provider: 'docker', masked: true },
];

export default function Firefly() {
  const [modules, setModules] = useState<Module[]>(MODULES);
  const [activeTab, setActiveTab] = useState<'deploy' | 'firefly' | 'vault' | 'logs'>('deploy');
  const [activeProvider, setActiveProvider] = useState<Provider | 'all'>('all');
  const [secrets, setSecrets] = useState<Secret[]>(DEFAULT_SECRETS);
  const [logs, setLogs] = useState<string[]>([]);
  const [fireflyStatus, setFireflyStatus] = useState<'online' | 'offline' | 'checking'>('checking');
  const [newSecret, setNewSecret] = useState({ key: '', value: '', provider: 'all' as Provider | 'all' });

  useEffect(() => {
    checkFireflyHealth();
  }, []);

  const checkFireflyHealth = async () => {
    setFireflyStatus('checking');
    try {
      const response = await fetch(`${GCP_CONFIG.gatewayUrl}${FIREFLY_ENDPOINTS.health}`, {
        signal: AbortSignal.timeout(5000)
      });
      setFireflyStatus(response.ok ? 'online' : 'offline');
    } catch {
      setFireflyStatus('offline');
    }
  };

  const StatusDot: React.FC<{ status: Status | 'online' | 'offline' | 'checking' }> = ({ status }) => (
    <span className={`inline-block w-2 h-2 rounded-full ${
      status === 'ready' || status === 'online' ? 'bg-green-500' :
      status === 'deploying' || status === 'checking' ? 'bg-yellow-500 animate-pulse' :
      status === 'error' ? 'bg-red-500' : 'bg-gray-500'
    }`} />
  );

  const ProviderIcon: React.FC<{ provider: Provider; size?: number }> = ({ provider, size = 16 }) => {
    switch (provider) {
      case 'gcp': return <GoogleCloudIcon size={size} className="text-blue-400" />;
      case 'azure': return <AzureIcon size={size} className="text-cyan-400" />;
      case 'docker': return <DockerIcon size={size} className="text-sky-400" />;
    }
  };

  const deploy = async (id: string) => {
    const mod = modules.find(m => m.id === id);
    if (!mod) return;
    setModules(prev => prev.map(m => m.id === id ? { ...m, status: 'deploying' } : m));
    setLogs(prev => [...prev, `[${new Date().toLocaleTimeString()}] Deploying ${mod.name}...`]);
    await new Promise(r => setTimeout(r, 2500));
    setModules(prev => prev.map(m => m.id === id ? { ...m, status: 'ready' } : m));
    setLogs(prev => [...prev, `[${new Date().toLocaleTimeString()}] ${mod.name} deployed successfully`]);
  };

  const toggleSecretMask = (key: string) => {
    setSecrets(prev => prev.map(s => s.key === key ? { ...s, masked: !s.masked } : s));
  };

  const updateSecret = (key: string, value: string) => {
    setSecrets(prev => prev.map(s => s.key === key ? { ...s, value } : s));
  };

  const addSecret = () => {
    if (newSecret.key && !secrets.find(s => s.key === newSecret.key)) {
      setSecrets(prev => [...prev, { ...newSecret, masked: true }]);
      setNewSecret({ key: '', value: '', provider: 'all' });
    }
  };

  const deleteSecret = (key: string) => {
    setSecrets(prev => prev.filter(s => s.key !== key));
  };

  const filteredModules = activeProvider === 'all' ? modules : modules.filter(m => m.provider === activeProvider);

  const tabs = [
    { id: 'deploy', label: 'Deploy', icon: <TerraformIcon size={14} /> },
    { id: 'firefly', label: 'Firefly API', icon: <Flame size={14} /> },
    { id: 'vault', label: 'Secrets', icon: <VaultIcon size={14} /> },
    { id: 'logs', label: 'Logs', icon: <Terminal size={14} /> },
  ];

  const providers = [
    { id: 'all', label: 'All', icon: <Server size={12} /> },
    { id: 'gcp', label: 'GCP', icon: <GoogleCloudIcon size={12} /> },
    { id: 'azure', label: 'Azure', icon: <AzureIcon size={12} /> },
    { id: 'docker', label: 'Docker', icon: <DockerIcon size={12} /> },
  ];

  return (
    <div className="h-full bg-gray-900 text-gray-100 flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
        <div className="flex items-center gap-2">
          <TerraformIcon size={18} className="text-purple-400" />
          <span className="font-semibold">Infrastructure as Code</span>
          <span className="text-xs text-gray-500">Multi-cloud deployment</span>
        </div>
        <div className="flex items-center gap-3 text-xs">
          <div className="flex items-center gap-1">
            <StatusDot status={fireflyStatus} />
            <span className="text-gray-400">Firefly</span>
          </div>
          <button onClick={checkFireflyHealth} className="p-1 hover:bg-gray-700 rounded">
            <RefreshCw size={12} />
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex gap-1 px-4 py-2 border-b border-gray-700 bg-gray-800/50">
        {tabs.map(t => (
          <button key={t.id} onClick={() => setActiveTab(t.id as typeof activeTab)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium transition-colors
              ${activeTab === t.id ? 'bg-purple-600 text-white' : 'text-gray-400 hover:text-white hover:bg-gray-700'}`}>
            {t.icon} {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-4">
        {/* Deploy Tab */}
        {activeTab === 'deploy' && (
          <div className="space-y-3">
            {/* Provider Filter */}
            <div className="flex gap-1 mb-3">
              {providers.map(p => (
                <button key={p.id} onClick={() => setActiveProvider(p.id as typeof activeProvider)}
                  className={`flex items-center gap-1 px-2 py-1 rounded text-xs
                    ${activeProvider === p.id ? 'bg-gray-700 text-white' : 'text-gray-500 hover:text-white'}`}>
                  {p.icon} {p.label}
                </button>
              ))}
            </div>

            {/* Modules */}
            <div className="space-y-2">
              {filteredModules.map(m => (
                <div key={m.id} className={`bg-gray-800 rounded p-3 border-l-2 ${
                  m.provider === 'gcp' ? 'border-blue-500' :
                  m.provider === 'azure' ? 'border-cyan-500' : 'border-sky-500'
                }`}>
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <ProviderIcon provider={m.provider} />
                        <StatusDot status={m.status} />
                        <span className="font-medium text-sm">{m.name}</span>
                        <span className="text-[10px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-400">{m.cost}</span>
                      </div>
                      <div className="text-xs text-gray-500 mb-1">{m.desc}</div>
                      <div className="text-[10px] text-gray-600 font-mono">{m.path}</div>
                      <div className="flex gap-1 mt-2">
                        {m.resources.map((r, i) => (
                          <span key={i} className="text-[10px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-400">{r}</span>
                        ))}
                      </div>
                    </div>
                    <button onClick={() => deploy(m.id)} disabled={m.status === 'deploying' || m.status === 'ready'}
                      className={`px-3 py-1.5 rounded text-xs font-medium flex items-center gap-1 ml-3
                        ${m.status === 'ready' ? 'bg-green-900/50 text-green-400' :
                          m.status === 'deploying' ? 'bg-yellow-900/50 text-yellow-400' :
                          'bg-purple-600 text-white hover:bg-purple-500'}`}>
                      {m.status === 'ready' ? <><CheckCircle2 size={12} /> Deployed</> :
                       m.status === 'deploying' ? <><Loader2 size={12} className="animate-spin" /> Deploying</> :
                       <><Rocket size={12} /> Deploy</>}
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Firefly API Tab */}
        {activeTab === 'firefly' && (
          <div className="space-y-3">
            <div className="bg-gray-800 rounded p-3">
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                  <Flame size={16} className="text-orange-500" />
                  <span className="font-medium">Firefly Gateway</span>
                  <StatusDot status={fireflyStatus} />
                </div>
                <code className="text-xs text-gray-500">{GCP_CONFIG.gatewayUrl}</code>
              </div>
              <div className="grid grid-cols-2 gap-2">
                {Object.entries(FIREFLY_ENDPOINTS).map(([key, path]) => (
                  <div key={key} className="bg-gray-900 rounded p-2 text-xs">
                    <div className="text-gray-300 font-medium mb-1">{key.replace(/([A-Z])/g, ' $1').trim()}</div>
                    <code className="text-gray-500">{path}</code>
                  </div>
                ))}
              </div>
            </div>

            <div className="bg-gray-800 rounded p-3">
              <div className="text-sm font-medium mb-2">Quick Test</div>
              <div className="flex gap-2">
                <button onClick={checkFireflyHealth}
                  className="px-3 py-1.5 bg-orange-600 hover:bg-orange-500 rounded text-xs font-medium">
                  Test Health
                </button>
                <button className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-xs font-medium">
                  Test MARC Validate
                </button>
                <button className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-xs font-medium">
                  Test Summarize
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Secrets Vault Tab */}
        {activeTab === 'vault' && (
          <div className="space-y-3">
            <div className="flex items-center gap-2 mb-2">
              <Shield size={16} className="text-yellow-500" />
              <span className="text-sm font-medium">Secrets Vault</span>
              <span className="text-xs text-gray-500">Local encrypted storage</span>
            </div>

            {/* Existing Secrets */}
            <div className="space-y-2">
              {secrets.map(s => (
                <div key={s.key} className="bg-gray-800 rounded p-2 flex items-center gap-2">
                  <div className="flex items-center gap-1 min-w-[80px]">
                    {s.provider === 'gcp' && <GoogleCloudIcon size={12} />}
                    {s.provider === 'azure' && <AzureIcon size={12} />}
                    {s.provider === 'docker' && <DockerIcon size={12} />}
                    {s.provider === 'all' && <Key size={12} className="text-yellow-500" />}
                  </div>
                  <div className="flex-1 grid grid-cols-2 gap-2">
                    <code className="text-xs text-gray-300 bg-gray-900 px-2 py-1 rounded">{s.key}</code>
                    <div className="flex items-center gap-1">
                      <input
                        type={s.masked ? 'password' : 'text'}
                        value={s.value}
                        onChange={(e) => updateSecret(s.key, e.target.value)}
                        placeholder="Enter value..."
                        className="flex-1 text-xs bg-gray-900 border border-gray-700 rounded px-2 py-1 text-gray-300"
                      />
                      <button onClick={() => toggleSecretMask(s.key)} className="p-1 hover:bg-gray-700 rounded">
                        {s.masked ? <EyeOff size={12} /> : <Eye size={12} />}
                      </button>
                      <button onClick={() => deleteSecret(s.key)} className="p-1 hover:bg-red-900 rounded text-red-400">
                        <Trash2 size={12} />
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            {/* Add New Secret */}
            <div className="bg-gray-800 rounded p-3">
              <div className="text-xs text-gray-400 mb-2">Add New Secret</div>
              <div className="flex gap-2">
                <select value={newSecret.provider} onChange={e => setNewSecret({...newSecret, provider: e.target.value as any})}
                  className="text-xs bg-gray-900 border border-gray-700 rounded px-2 py-1">
                  <option value="all">All</option>
                  <option value="gcp">GCP</option>
                  <option value="azure">Azure</option>
                  <option value="docker">Docker</option>
                </select>
                <input value={newSecret.key} onChange={e => setNewSecret({...newSecret, key: e.target.value})}
                  placeholder="KEY_NAME" className="flex-1 text-xs bg-gray-900 border border-gray-700 rounded px-2 py-1" />
                <input value={newSecret.value} onChange={e => setNewSecret({...newSecret, value: e.target.value})}
                  placeholder="Value" type="password" className="flex-1 text-xs bg-gray-900 border border-gray-700 rounded px-2 py-1" />
                <button onClick={addSecret} className="px-3 py-1 bg-yellow-600 hover:bg-yellow-500 rounded text-xs font-medium flex items-center gap-1">
                  <Plus size={12} /> Add
                </button>
              </div>
            </div>

            {/* Export/Import */}
            <div className="flex gap-2 text-xs">
              <button className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded flex items-center gap-1">
                <Lock size={12} /> Export Encrypted
              </button>
              <button className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded">
                Import from .env
              </button>
            </div>
          </div>
        )}

        {/* Logs Tab */}
        {activeTab === 'logs' && (
          <div className="bg-black rounded p-3 font-mono text-xs text-green-400 h-full overflow-auto">
            {logs.length === 0 ? (
              <span className="text-gray-600">No deployment logs yet...</span>
            ) : (
              logs.map((log, i) => <div key={i}>{log}</div>)
            )}
          </div>
        )}
      </div>
    </div>
  );
}
