import React, { useState, useEffect, useCallback } from 'react';
import { 
  Database, Play, Pause, RefreshCw, CheckCircle, Clock, 
  AlertCircle, Loader2, Download, Upload, Zap, Shield,
  Target, FileCode, Bug, Radio, Terminal, Globe
} from 'lucide-react';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

interface ThreatSource {
  id: string;
  name: string;
  icon: string;
  category: 'attack' | 'emulation' | 'detection' | 'tools' | 'lotl' | 'osint';
  status: 'pending' | 'running' | 'complete' | 'error';
  itemCount: number;
  expectedCount: number;
  lastUpdated: string | null;
  progress: number;
}

interface PipelineStats {
  totalSources: number;
  totalTools: number;
  totalTechniques: number;
  totalMappings: number;
  completedSources: number;
  failedSources: number;
}

interface PipelineState {
  status: 'idle' | 'running' | 'paused' | 'complete' | 'error';
  currentSource: string | null;
  overallProgress: number;
  startTime: string | null;
  estimatedTimeRemaining: string | null;
}

// ═══════════════════════════════════════════════════════════════════════════
// Initial Data
// ═══════════════════════════════════════════════════════════════════════════

const THREAT_SOURCES: ThreatSource[] = [
  // ATT&CK Suite
  { id: 'mitre-attack', name: 'MITRE ATT&CK', icon: 'target', category: 'attack', status: 'pending', itemCount: 0, expectedCount: 700, lastUpdated: null, progress: 0 },
  { id: 'mitre-defend', name: 'MITRE D3FEND', icon: 'shield', category: 'attack', status: 'pending', itemCount: 0, expectedCount: 150, lastUpdated: null, progress: 0 },
  { id: 'mitre-car', name: 'MITRE CAR', icon: 'target', category: 'attack', status: 'pending', itemCount: 0, expectedCount: 425, lastUpdated: null, progress: 0 },
  { id: 'mitre-atlas', name: 'MITRE ATLAS', icon: 'target', category: 'attack', status: 'pending', itemCount: 0, expectedCount: 97, lastUpdated: null, progress: 0 },
  
  // Adversary Emulation
  { id: 'atomic-red-team', name: 'Atomic Red Team', icon: 'zap', category: 'emulation', status: 'pending', itemCount: 0, expectedCount: 1200, lastUpdated: null, progress: 0 },
  { id: 'caldera', name: 'Caldera', icon: 'zap', category: 'emulation', status: 'pending', itemCount: 0, expectedCount: 500, lastUpdated: null, progress: 0 },
  
  // Detection Rules
  { id: 'nuclei', name: 'Nuclei Templates', icon: 'bug', category: 'detection', status: 'pending', itemCount: 0, expectedCount: 8000, lastUpdated: null, progress: 0 },
  { id: 'sigma', name: 'Sigma Rules', icon: 'radio', category: 'detection', status: 'pending', itemCount: 0, expectedCount: 2500, lastUpdated: null, progress: 0 },
  { id: 'yara', name: 'YARA Rules', icon: 'filecode', category: 'detection', status: 'pending', itemCount: 0, expectedCount: 1000, lastUpdated: null, progress: 0 },
  { id: 'wazuh', name: 'Wazuh Rules', icon: 'shield', category: 'detection', status: 'pending', itemCount: 0, expectedCount: 500, lastUpdated: null, progress: 0 },
  
  // Tools
  { id: 'kali-tools', name: 'Kali Linux Tools', icon: 'terminal', category: 'tools', status: 'pending', itemCount: 0, expectedCount: 600, lastUpdated: null, progress: 0 },
  { id: 'nmap-scripts', name: 'Nmap Scripts', icon: 'globe', category: 'tools', status: 'pending', itemCount: 0, expectedCount: 600, lastUpdated: null, progress: 0 },
  
  // Living Off The Land
  { id: 'lolbas', name: 'LOLBAS', icon: 'terminal', category: 'lotl', status: 'pending', itemCount: 0, expectedCount: 200, lastUpdated: null, progress: 0 },
  { id: 'gtfobins', name: 'GTFOBins', icon: 'terminal', category: 'lotl', status: 'pending', itemCount: 0, expectedCount: 300, lastUpdated: null, progress: 0 },
  { id: 'loldrivers', name: 'LOLDrivers', icon: 'terminal', category: 'lotl', status: 'pending', itemCount: 0, expectedCount: 100, lastUpdated: null, progress: 0 },
  { id: 'hijacklibs', name: 'HijackLibs', icon: 'terminal', category: 'lotl', status: 'pending', itemCount: 0, expectedCount: 150, lastUpdated: null, progress: 0 },
  { id: 'wadcoms', name: 'WADComs', icon: 'terminal', category: 'lotl', status: 'pending', itemCount: 0, expectedCount: 100, lastUpdated: null, progress: 0 },
  
  // OSINT
  { id: 'osint-framework', name: 'OSINT Framework', icon: 'globe', category: 'osint', status: 'pending', itemCount: 0, expectedCount: 500, lastUpdated: null, progress: 0 },
  { id: 'awesome-osint', name: 'Awesome OSINT', icon: 'globe', category: 'osint', status: 'pending', itemCount: 0, expectedCount: 300, lastUpdated: null, progress: 0 },
  { id: 'sherlock', name: 'Sherlock', icon: 'globe', category: 'osint', status: 'pending', itemCount: 0, expectedCount: 400, lastUpdated: null, progress: 0 },
];

const CATEGORY_LABELS: Record<string, string> = {
  attack: 'ATT&CK Framework',
  emulation: 'Adversary Emulation',
  detection: 'Detection Rules',
  tools: 'Security Tools',
  lotl: 'Living Off The Land',
  osint: 'OSINT Sources'
};

const CATEGORY_COLORS: Record<string, string> = {
  attack: 'text-red-400 bg-red-500/10',
  emulation: 'text-orange-400 bg-orange-500/10',
  detection: 'text-blue-400 bg-blue-500/10',
  tools: 'text-green-400 bg-green-500/10',
  lotl: 'text-purple-400 bg-purple-500/10',
  osint: 'text-cyan-400 bg-cyan-500/10'
};

// ═══════════════════════════════════════════════════════════════════════════
// API Functions (Firefly IAC triggers these)
// ═══════════════════════════════════════════════════════════════════════════

const API_BASE = '/api/firefly/threat-intel';

async function startPipeline(): Promise<{ jobId: string }> {
  const response = await fetch(`${API_BASE}/start`, { method: 'POST' });
  return response.json();
}

async function pausePipeline(jobId: string): Promise<void> {
  await fetch(`${API_BASE}/pause/${jobId}`, { method: 'POST' });
}

async function getPipelineStatus(jobId: string): Promise<{
  state: PipelineState;
  sources: ThreatSource[];
  stats: PipelineStats;
}> {
  const response = await fetch(`${API_BASE}/status/${jobId}`);
  return response.json();
}

async function getStoredStats(): Promise<PipelineStats> {
  const response = await fetch(`${API_BASE}/stats`);
  return response.json();
}

// ═══════════════════════════════════════════════════════════════════════════
// Components
// ═══════════════════════════════════════════════════════════════════════════

const IconMap: Record<string, React.FC<{ className?: string }>> = {
  target: Target,
  shield: Shield,
  zap: Zap,
  bug: Bug,
  radio: Radio,
  terminal: Terminal,
  globe: Globe,
  filecode: FileCode,
  database: Database,
};

function StatCard({ label, value, icon: Icon, status }: { 
  label: string; 
  value: number | string; 
  icon: React.FC<{ className?: string }>;
  status?: 'success' | 'warning' | 'error' | 'loading';
}) {
  const statusColors = {
    success: 'border-green-500/30 bg-green-500/5',
    warning: 'border-yellow-500/30 bg-yellow-500/5',
    error: 'border-red-500/30 bg-red-500/5',
    loading: 'border-blue-500/30 bg-blue-500/5',
  };
  
  return (
    <div className={`rounded-lg border p-4 ${status ? statusColors[status] : 'border-gray-700 bg-gray-800/50'}`}>
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-400">{label}</p>
          <p className="text-2xl font-bold text-white mt-1">
            {typeof value === 'number' ? value.toLocaleString() : value}
          </p>
        </div>
        <Icon className="w-8 h-8 text-gray-500" />
      </div>
    </div>
  );
}

function SourceRow({ source }: { source: ThreatSource }) {
  const IconComponent = IconMap[source.icon] || Database;
  
  const statusIcon = {
    pending: <Clock className="w-4 h-4 text-gray-400" />,
    running: <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />,
    complete: <CheckCircle className="w-4 h-4 text-green-400" />,
    error: <AlertCircle className="w-4 h-4 text-red-400" />,
  }[source.status];
  
  const statusText = {
    pending: 'Pending',
    running: 'Running...',
    complete: 'Complete',
    error: 'Error',
  }[source.status];
  
  return (
    <div className="flex items-center justify-between py-3 px-4 hover:bg-gray-800/50 rounded-lg transition-colors">
      <div className="flex items-center gap-3">
        <div className={`p-2 rounded-lg ${CATEGORY_COLORS[source.category]}`}>
          <IconComponent className="w-4 h-4" />
        </div>
        <div>
          <p className="text-sm font-medium text-white">{source.name}</p>
          <p className="text-xs text-gray-500">{CATEGORY_LABELS[source.category]}</p>
        </div>
      </div>
      
      <div className="flex items-center gap-6">
        {source.status === 'running' && (
          <div className="w-32">
            <div className="h-1.5 bg-gray-700 rounded-full overflow-hidden">
              <div 
                className="h-full bg-blue-500 rounded-full transition-all duration-300"
                style={{ width: `${source.progress}%` }}
              />
            </div>
            <p className="text-xs text-gray-500 mt-1 text-center">{source.progress}%</p>
          </div>
        )}
        
        <div className="w-20 text-right">
          <p className="text-sm font-mono text-white">
            {source.itemCount > 0 ? source.itemCount.toLocaleString() : '-'}
          </p>
          <p className="text-xs text-gray-500">
            / {source.expectedCount.toLocaleString()}
          </p>
        </div>
        
        <div className="flex items-center gap-2 w-24">
          {statusIcon}
          <span className={`text-sm ${
            source.status === 'complete' ? 'text-green-400' :
            source.status === 'running' ? 'text-blue-400' :
            source.status === 'error' ? 'text-red-400' :
            'text-gray-400'
          }`}>
            {statusText}
          </span>
        </div>
        
        <div className="w-24 text-right">
          <p className="text-xs text-gray-500">
            {source.lastUpdated || '-'}
          </p>
        </div>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Main Dashboard Component
// ═══════════════════════════════════════════════════════════════════════════

export default function ThreatIntelPipeline() {
  const [sources, setSources] = useState<ThreatSource[]>(THREAT_SOURCES);
  const [pipelineState, setPipelineState] = useState<PipelineState>({
    status: 'idle',
    currentSource: null,
    overallProgress: 0,
    startTime: null,
    estimatedTimeRemaining: null
  });
  const [stats, setStats] = useState<PipelineStats>({
    totalSources: 20,
    totalTools: 0,
    totalTechniques: 0,
    totalMappings: 0,
    completedSources: 0,
    failedSources: 0
  });
  const [jobId, setJobId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  
  // Load initial stats
  useEffect(() => {
    getStoredStats()
      .then(setStats)
      .catch(() => {/* Use defaults */});
  }, []);
  
  // Poll for status when running
  useEffect(() => {
    if (!jobId || pipelineState.status !== 'running') return;
    
    const interval = setInterval(async () => {
      try {
        const { state, sources: updatedSources, stats: updatedStats } = 
          await getPipelineStatus(jobId);
        setPipelineState(state);
        setSources(updatedSources);
        setStats(updatedStats);
        
        if (state.status === 'complete' || state.status === 'error') {
          clearInterval(interval);
        }
      } catch (err) {
        console.error('Failed to get status:', err);
      }
    }, 2000);
    
    return () => clearInterval(interval);
  }, [jobId, pipelineState.status]);
  
  const handleStart = async () => {
    try {
      setError(null);
      const { jobId: newJobId } = await startPipeline();
      setJobId(newJobId);
      setPipelineState(prev => ({ ...prev, status: 'running', startTime: new Date().toISOString() }));
    } catch (err) {
      setError('Failed to start pipeline. Is Firefly IAC running?');
    }
  };
  
  const handlePause = async () => {
    if (!jobId) return;
    try {
      await pausePipeline(jobId);
      setPipelineState(prev => ({ ...prev, status: 'paused' }));
    } catch (err) {
      setError('Failed to pause pipeline');
    }
  };
  
  // Group sources by category
  const groupedSources = sources.reduce((acc, source) => {
    if (!acc[source.category]) acc[source.category] = [];
    acc[source.category].push(source);
    return acc;
  }, {} as Record<string, ThreatSource[]>);
  
  const completedCount = sources.filter(s => s.status === 'complete').length;
  const runningSource = sources.find(s => s.status === 'running');
  
  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-3">
            <Database className="w-8 h-8 text-blue-400" />
            Threat Intelligence Pipeline
          </h1>
          <p className="text-gray-400 mt-1">
            Download, normalize, and load threat data from 20+ sources
          </p>
        </div>
        
        <div className="flex items-center gap-3">
          {pipelineState.status === 'idle' || pipelineState.status === 'complete' ? (
            <button
              onClick={handleStart}
              className="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-500 rounded-lg font-medium transition-colors"
            >
              <Play className="w-5 h-5" />
              Start Pipeline
            </button>
          ) : pipelineState.status === 'running' ? (
            <button
              onClick={handlePause}
              className="flex items-center gap-2 px-6 py-3 bg-yellow-600 hover:bg-yellow-500 rounded-lg font-medium transition-colors"
            >
              <Pause className="w-5 h-5" />
              Pause
            </button>
          ) : (
            <button
              onClick={handleStart}
              className="flex items-center gap-2 px-6 py-3 bg-green-600 hover:bg-green-500 rounded-lg font-medium transition-colors"
            >
              <RefreshCw className="w-5 h-5" />
              Resume
            </button>
          )}
        </div>
      </div>
      
      {/* Error Banner */}
      {error && (
        <div className="mb-6 p-4 bg-red-500/10 border border-red-500/30 rounded-lg flex items-center gap-3">
          <AlertCircle className="w-5 h-5 text-red-400" />
          <p className="text-red-400">{error}</p>
        </div>
      )}
      
      {/* Stats Cards */}
      <div className="grid grid-cols-4 gap-4 mb-8">
        <StatCard 
          label="Total Sources" 
          value={stats.totalSources} 
          icon={Database}
          status={pipelineState.status === 'running' ? 'loading' : undefined}
        />
        <StatCard 
          label="Tools Loaded" 
          value={stats.totalTools} 
          icon={Terminal}
          status={stats.totalTools > 0 ? 'success' : undefined}
        />
        <StatCard 
          label="Techniques" 
          value={stats.totalTechniques} 
          icon={Target}
          status={stats.totalTechniques > 0 ? 'success' : undefined}
        />
        <StatCard 
          label="Mappings" 
          value={stats.totalMappings} 
          icon={Zap}
          status={stats.totalMappings > 0 ? 'success' : undefined}
        />
      </div>
      
      {/* Progress Bar */}
      {pipelineState.status === 'running' && (
        <div className="mb-8 p-4 bg-gray-800 rounded-lg border border-gray-700">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2">
              <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />
              <span className="text-sm text-gray-300">
                Processing: <strong>{runningSource?.name || 'Initializing...'}</strong>
              </span>
            </div>
            <span className="text-sm text-gray-400">
              {completedCount} / {sources.length} sources • 
              {pipelineState.estimatedTimeRemaining && ` ~${pipelineState.estimatedTimeRemaining} remaining`}
            </span>
          </div>
          <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
            <div 
              className="h-full bg-gradient-to-r from-blue-500 to-cyan-400 rounded-full transition-all duration-500"
              style={{ width: `${(completedCount / sources.length) * 100}%` }}
            />
          </div>
        </div>
      )}
      
      {/* Complete Banner */}
      {pipelineState.status === 'complete' && (
        <div className="mb-8 p-4 bg-green-500/10 border border-green-500/30 rounded-lg flex items-center justify-between">
          <div className="flex items-center gap-3">
            <CheckCircle className="w-6 h-6 text-green-400" />
            <div>
              <p className="font-medium text-green-400">Pipeline Complete!</p>
              <p className="text-sm text-gray-400">
                {stats.totalTools.toLocaleString()} tools loaded with {stats.totalMappings.toLocaleString()} technique mappings
              </p>
            </div>
          </div>
          <button
            onClick={() => window.location.href = '/kali'}
            className="px-4 py-2 bg-green-600 hover:bg-green-500 rounded-lg text-sm font-medium transition-colors"
          >
            Go to Kali Tools →
          </button>
        </div>
      )}
      
      {/* Sources List */}
      <div className="bg-gray-800/50 rounded-lg border border-gray-700">
        <div className="px-4 py-3 border-b border-gray-700 flex items-center justify-between">
          <h2 className="font-medium">Data Sources</h2>
          <div className="flex items-center gap-4 text-sm text-gray-400">
            <span className="flex items-center gap-1">
              <CheckCircle className="w-4 h-4 text-green-400" />
              {completedCount} complete
            </span>
            <span className="flex items-center gap-1">
              <Clock className="w-4 h-4 text-gray-400" />
              {sources.filter(s => s.status === 'pending').length} pending
            </span>
          </div>
        </div>
        
        <div className="divide-y divide-gray-700/50">
          {Object.entries(groupedSources).map(([category, categorySources]) => (
            <div key={category}>
              <div className="px-4 py-2 bg-gray-800/30">
                <span className={`text-xs font-medium uppercase tracking-wider ${CATEGORY_COLORS[category].split(' ')[0]}`}>
                  {CATEGORY_LABELS[category]}
                </span>
              </div>
              {categorySources.map(source => (
                <SourceRow key={source.id} source={source} />
              ))}
            </div>
          ))}
        </div>
      </div>
      
      {/* Footer Actions */}
      <div className="mt-6 flex items-center justify-between text-sm text-gray-500">
        <div className="flex items-center gap-4">
          <button className="hover:text-gray-300 transition-colors flex items-center gap-1">
            <Download className="w-4 h-4" />
            Export to JSON
          </button>
          <button className="hover:text-gray-300 transition-colors flex items-center gap-1">
            <Upload className="w-4 h-4" />
            Upload to Supabase
          </button>
        </div>
        <p>
          Last full sync: Never
        </p>
      </div>
    </div>
  );
}
