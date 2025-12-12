import React, { useState } from 'react';
import {
  Grid, Layers, Shield, Database, Network, Settings, Map, BarChart3, Terminal,
  Target, Eye, Brain, Cpu, Server, Search, FileText, Package, CheckCircle2, RefreshCw
} from 'lucide-react';

// Component imports
import AdminDashboard from '@/components/AdminDashboard/AdminDashboard';
import RedTeamRunner from '@/components/RedTeam/RedTeamRunner';
import PhaseMapping from '@/components/RedTeam/PhaseMapping';
import HashComposer from '@/components/HashComposer/HashComposer';
import Cognigraph from '@/components/Cognigraph';
import EnhancedMap from '@/components/EnhancedMap';
import HD4Map from '@/components/HD4Map';
import CTASGraphAnimation from '@/components/CTASGraphAnimation';
import Shodan from '@/components/Shodan';
import TaskManagement from '@/components/TaskManagement';
import ThreatActorList from '@/components/ThreatActorList';
import HD4PhaseContent from '@/components/HD4PhaseContent';
import HD4TaskView from '@/components/HD4TaskView';
import KaliToolsIntegration from '@/components/KaliToolsIntegration';
import Playbooks from '@/components/Playbooks';
import MultiCLI from '@/components/MultiCLI';
import AnsiblePlaybooks from '@/components/AnsiblePlaybooks';
import AtomicTestRunner from '@/components/AtomicRedTeam/AtomicTestRunner';
import AttackNavigator from '@/components/AtomicRedTeam/AttackNavigator';
import { DataVisualizationToolbar, DatabaseConnectionPanel, FilterPanel, NetworkView } from '@/components/shared';

type Status = 'available' | 'loaded' | 'upgrade';
type Category = 'all' | 'toolbars' | 'infrastructure' | 'admin' | 'redteam' | 'analysis' | 'visualization' | 'maps' | 'intel' | 'ops';

interface Component {
  id: string;
  name: string;
  desc: string;
  category: Category;
  icon: React.ComponentType<any>;
  status: Status;
  component: React.ComponentType<any>;
}

const COMPONENTS: Component[] = [
  // Toolbars
  { id: 'data-toolbar', name: 'Data Visualization Toolbar', desc: '13 common visualization actions', category: 'toolbars', icon: Layers, status: 'available', component: () => <DataVisualizationToolbar variant="default" /> },
  { id: 'filter-panel', name: 'Filter Panel', desc: 'Advanced filtering with sectors/phases', category: 'toolbars', icon: Search, status: 'available', component: FilterPanel },
  { id: 'multi-cli', name: 'Multi CLI', desc: 'Multi-agent CLI with chat/terminal modes', category: 'toolbars', icon: Terminal, status: 'loaded', component: () => <MultiCLI currentHD4Phase="hunt" /> },

  // Infrastructure
  { id: 'db-panel', name: 'Database Connection Panel', desc: 'Unified database management', category: 'infrastructure', icon: Database, status: 'available', component: DatabaseConnectionPanel },

  // Admin
  { id: 'admin', name: 'Admin Dashboard', desc: 'System administration', category: 'admin', icon: Settings, status: 'upgrade', component: AdminDashboard },

  // Red Team
  { id: 'redteam', name: 'Red Team Runner', desc: 'Red team ops & testing', category: 'redteam', icon: Target, status: 'upgrade', component: RedTeamRunner },
  { id: 'phase-mapping', name: 'Phase Mapping', desc: 'HD4 to ATT&CK/Kill Chain mapping', category: 'redteam', icon: Network, status: 'available', component: PhaseMapping },
  { id: 'atomic-runner', name: 'Atomic Test Runner', desc: 'Atomic Red Team test execution', category: 'redteam', icon: Target, status: 'available', component: () => <AtomicTestRunner phase="Hunt" /> },
  { id: 'attack-nav', name: 'ATT&CK Navigator', desc: 'MITRE ATT&CK framework browser', category: 'redteam', icon: Shield, status: 'available', component: AttackNavigator },
  { id: 'ansible', name: 'Ansible Playbooks', desc: 'Infrastructure automation runner', category: 'ops', icon: Server, status: 'available', component: AnsiblePlaybooks },

  // Analysis
  { id: 'hash', name: 'Hash Composer', desc: 'Hash composition & analysis', category: 'analysis', icon: Terminal, status: 'upgrade', component: HashComposer },

  // Visualization
  { id: 'cognigraph', name: 'Cognigraph', desc: 'Cognitive graph visualization', category: 'visualization', icon: Brain, status: 'upgrade', component: Cognigraph },
  { id: 'ctas-graph', name: 'CTAS Graph Animation', desc: 'Animated entity graph view', category: 'visualization', icon: Network, status: 'loaded', component: CTASGraphAnimation },
  { id: 'network-view', name: 'Network View', desc: 'Network topology visualization', category: 'visualization', icon: Network, status: 'available', component: NetworkView },

  // Maps
  { id: 'map', name: 'Enhanced Map', desc: 'Mapbox GL integration', category: 'maps', icon: Map, status: 'loaded', component: EnhancedMap },
  { id: 'hd4-map', name: 'HD4 Map', desc: 'HD4 tactical operations map', category: 'maps', icon: Map, status: 'loaded', component: () => <HD4Map hd4Action="Hunt" /> },

  // Intel
  { id: 'shodan', name: 'Shodan Intelligence', desc: 'Shodan API integration', category: 'intel', icon: Eye, status: 'upgrade', component: Shodan },
  { id: 'threats', name: 'Threat Actor Intel', desc: 'Threat actor tracking', category: 'intel', icon: Shield, status: 'upgrade', component: ThreatActorList },

  // Ops
  { id: 'tasks', name: 'Task Management', desc: 'Task & workflow management', category: 'ops', icon: FileText, status: 'available', component: TaskManagement },
  { id: 'hd4', name: 'HD4 Phase Content', desc: 'HD4 operational framework', category: 'ops', icon: Cpu, status: 'upgrade', component: () => <HD4PhaseContent phase="Hunt" view="map" /> },
  { id: 'hd4-tasks', name: 'HD4 Task View', desc: 'HD4 task management panel', category: 'ops', icon: FileText, status: 'available', component: () => <HD4TaskView hd4Action="Hunt" /> },
  { id: 'kali', name: 'Kali Tools Integration', desc: 'Kali Linux tool launcher', category: 'ops', icon: Server, status: 'loaded', component: KaliToolsIntegration },
  { id: 'playbooks', name: 'Playbooks', desc: 'Security playbook runner', category: 'ops', icon: FileText, status: 'available', component: Playbooks },
];

const CATEGORIES: { id: Category; name: string; icon: React.ComponentType<any> }[] = [
  { id: 'all', name: 'All', icon: Grid },
  { id: 'toolbars', name: 'Toolbars', icon: Layers },
  { id: 'infrastructure', name: 'Infra', icon: Server },
  { id: 'admin', name: 'Admin', icon: Settings },
  { id: 'redteam', name: 'Red Team', icon: Target },
  { id: 'analysis', name: 'Analysis', icon: BarChart3 },
  { id: 'visualization', name: 'Visual', icon: Eye },
  { id: 'maps', name: 'Maps', icon: Map },
  { id: 'intel', name: 'Intel', icon: Shield },
  { id: 'ops', name: 'Ops', icon: Cpu },
];

export default function Gallery() {
  const [category, setCategory] = useState<Category>('all');
  const [selected, setSelected] = useState<string | null>(null);
  const [search, setSearch] = useState('');

  const filtered = COMPONENTS.filter(c =>
    (category === 'all' || c.category === category) &&
    (c.name.toLowerCase().includes(search.toLowerCase()) || c.desc.toLowerCase().includes(search.toLowerCase()))
  );

  const current = COMPONENTS.find(c => c.id === selected);

  const StatusBadge: React.FC<{ status: Status }> = ({ status }) => (
    <span className={`text-[10px] px-1.5 py-0.5 rounded font-medium ${
      status === 'available' ? 'bg-green-900/50 text-green-400' :
      status === 'loaded' ? 'bg-blue-900/50 text-blue-400' :
      'bg-yellow-900/50 text-yellow-400'
    }`}>
      {status.toUpperCase()}
    </span>
  );

  return (
    <div className="h-full bg-gray-900 text-gray-100 flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
        <div className="flex items-center gap-2">
          <Package size={18} className="text-blue-400" />
          <span className="font-semibold">Component Gallery</span>
          <span className="text-xs text-gray-500">{filtered.length} components</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="relative">
            <Search size={12} className="absolute left-2 top-1/2 -translate-y-1/2 text-gray-500" />
            <input
              value={search}
              onChange={e => setSearch(e.target.value)}
              placeholder="Search..."
              className="pl-7 pr-3 py-1 text-xs bg-gray-800 border border-gray-700 rounded w-48"
            />
          </div>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar */}
        <div className="w-44 border-r border-gray-700 p-2 space-y-1 overflow-y-auto">
          {CATEGORIES.map(cat => {
            const Icon = cat.icon;
            const count = cat.id === 'all' ? COMPONENTS.length : COMPONENTS.filter(c => c.category === cat.id).length;
            return (
              <button
                key={cat.id}
                onClick={() => { setCategory(cat.id); setSelected(null); }}
                className={`w-full flex items-center justify-between px-2 py-1.5 rounded text-xs transition-colors
                  ${category === cat.id ? 'bg-blue-600 text-white' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}`}
              >
                <div className="flex items-center gap-2">
                  <Icon size={12} />
                  <span>{cat.name}</span>
                </div>
                <span className="text-[10px] opacity-60">{count}</span>
              </button>
            );
          })}
        </div>

        {/* Main */}
        <div className="flex-1 overflow-y-auto p-4">
          {current ? (
            <div className="space-y-4">
              {/* Detail Header */}
              <div className="flex items-start justify-between pb-3 border-b border-gray-700">
                <div>
                  <div className="flex items-center gap-2 mb-1">
                    <current.icon size={16} className="text-gray-400" />
                    <span className="font-medium">{current.name}</span>
                    <StatusBadge status={current.status} />
                  </div>
                  <p className="text-xs text-gray-500">{current.desc}</p>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => setSelected(null)}
                    className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded"
                  >
                    Back
                  </button>
                  <button
                    disabled={current.status === 'loaded'}
                    className={`px-3 py-1 text-xs rounded flex items-center gap-1 ${
                      current.status === 'loaded' ? 'bg-green-900/50 text-green-400' :
                      'bg-blue-600 hover:bg-blue-500 text-white'
                    }`}
                  >
                    {current.status === 'loaded' ? <><CheckCircle2 size={12} /> Loaded</> : 'Load'}
                  </button>
                </div>
              </div>

              {/* Preview */}
              <div className="bg-gray-800 rounded border border-gray-700 p-3">
                <div className="text-xs text-gray-500 mb-2">Preview</div>
                <div className="bg-gray-900 rounded p-3 border border-gray-700 min-h-[200px]">
                  <current.component />
                </div>
              </div>
            </div>
          ) : (
            <div className="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
              {filtered.map(comp => {
                const Icon = comp.icon;
                return (
                  <div
                    key={comp.id}
                    onClick={() => setSelected(comp.id)}
                    className="bg-gray-800 rounded p-3 cursor-pointer hover:bg-gray-750 border border-gray-700 hover:border-gray-600 transition-colors"
                  >
                    <div className="flex items-start justify-between mb-2">
                      <div className="flex items-center gap-2">
                        <Icon size={14} className="text-gray-500" />
                        <span className="text-sm font-medium">{comp.name}</span>
                      </div>
                    </div>
                    <p className="text-xs text-gray-500 mb-2 line-clamp-2">{comp.desc}</p>
                    <StatusBadge status={comp.status} />
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
