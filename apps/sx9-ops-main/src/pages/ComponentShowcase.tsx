import React, { useState } from 'react';
import { 
  Grid, 
  Layers, 
  Shield, 
  Database, 
  Network, 
  Code, 
  Settings, 
  Users, 
  Map, 
  BarChart3,
  Terminal,
  Zap,
  Target,
  Eye,
  Brain,
  Globe,
  Cpu,
  Server,
  Lock,
  Search,
  FileText,
  Play,
  GitBranch,
  Hash,
  Palette
} from 'lucide-react';

// Import all components with aliases to avoid conflicts
import AdminDashboard from '@/components/AdminDashboard/AdminDashboard';
import SystemOverview from '@/components/AdminDashboard/SystemOverview';
import Localization from '@/components/AdminDashboard/Localization';
import PackageStatus from '@/components/AdminDashboard/PackageStatus';
import CodeViewer from '@/components/AdminDashboard/CodeViewer';
import ProjectExport from '@/components/AdminDashboard/ProjectExport';
import ThreatFeeds from '@/components/AdminDashboard/ThreatFeeds';
import APIStore from '@/components/AdminDashboard/APIStore';

import RedTeamRunner from '@/components/RedTeam/RedTeamRunner';
import PhaseMapping from '@/components/RedTeam/PhaseMapping';
import RecordList from '@/components/RedTeam/RecordList';
import TestList from '@/components/RedTeam/TestList';
import PhaseSelector from '@/components/RedTeam/PhaseSelector';

import AtomicTestRunner from '@/components/AtomicRedTeam/AtomicTestRunner';
import AtomicNDEXMapping from '@/components/AtomicRedTeam/AtomicNDEXMapping';
import AttackNavigator from '@/components/AtomicRedTeam/AttackNavigator';

import HashComposer from '@/components/HashComposer/HashComposer';
import { HashDisplay } from '@/components/HashComposer/HashDisplay';
import { HashGrid } from '@/components/HashComposer/HashGrid';
import { DomainSelector } from '@/components/HashComposer/DomainSelector';
import { LayerSelector } from '@/components/HashComposer/LayerSelector';
import { ExportPanel } from '@/components/HashComposer/ExportPanel';

import CTASCLI from '@/components/CTASCLI';
import KaliCLI from '@/components/KaliCLI';
import NetworkCLI from '@/components/NetworkCLI';
import WSLEnvironment from '@/components/WSLEnvironment';
import PythonIntegration from '@/components/PythonIntegration';

import Cognigraph from '@/components/Cognigraph';
import KnowledgeGraph from '@/components/KnowledgeGraph';
import Neo4jKnowledgeGraph from '@/components/Neo4jKnowledgeGraph';
import GraphVisualizer from '@/components/GraphVisualizer';
import CTASGraph from '@/components/CTASGraph';
import CTASGraphAnimation from '@/components/CTASGraphAnimation';
import TransformsGraph from '@/components/TransformsGraph';
import HD4Graph from '@/components/HD4Graph';

import EnhancedMap from '@/components/EnhancedMap';
import SimpleMap from '@/components/SimpleMap';
import GISMap from '@/components/GISMap';
import HD4MapComponent from '@/components/HD4Map';
import NetworkMap from '@/components/NetworkMap';
import MapView from '@/components/MapView';
import GeospatialDataManager from '@/components/GeospatialDataManager';

import Shodan from '@/components/Shodan';
import OSINTModule from '@/components/OSINTModule';
import PhishingModule from '@/components/PhishingModule';
import KaliToolsIntegration from '@/components/KaliToolsIntegration';
import AnsiblePlaybooks from '@/components/AnsiblePlaybooks';
import CTASPlaybooks from '@/components/CTASPlaybooks';
import HD4Playbooks from '@/components/HD4Playbooks';
import Playbooks from '@/components/Playbooks';

import TaskManagement from '@/components/TaskManagement';
import TaskList from '@/components/TaskList';
import CTASTaskList from '@/components/CTASTaskList';
import HD4TaskView from '@/components/HD4TaskView';

import ThreatActorList from '@/components/ThreatActorList';
import TopAPTs from '@/components/TopAPTs';
import CriticalInfrastructureDashboard from '@/components/CriticalInfrastructureDashboard';
import CriticalInfrastructureSectors from '@/components/CriticalInfrastructureSectors';

import RedisManager from '@/components/RedisManager';
import NetworksControl from '@/components/NetworksControl';
import IoTControl from '@/components/IoTControl';
import StreamProcessor from '@/components/StreamProcessor';
import AnimatedInfoStreams from '@/components/AnimatedInfoStreams';

import HD4PhaseContent from '@/components/HD4PhaseContent';
import HD4OperationalSpectrum from '@/components/HD4OperationalSpectrum';

import N8NWorkflows from '@/components/N8NWorkflows';
import SearchInterface from '@/components/SearchInterface';
import ControlPanel from '@/components/ControlPanel';
import AIIntegration from '@/components/AIIntegration';
import VirtualRavenDashboard from '@/components/VirtualRavenDashboard';

import OperationsOverview from '@/components/OperationsOverview';
import OperationsSummary from '@/components/OperationsSummary';
import SystemArchitectureDiagram from '@/components/SystemArchitectureDiagram';
import CTASSystemDiagram from '@/components/CTASSystemDiagram';

import PersonasList from '@/components/PersonasList';
import NISTvsHD4Comparison from '@/components/NISTvsHD4Comparison';
import SpreadsheetDisplay from '@/components/SpreadsheetDisplay';
import GridView from '@/components/GridView';
import ViewToggle from '@/components/ViewToggle';
import DocsScreen from '@/components/DocsScreen';

import ScriptScrapers from '@/components/ScriptScrapers';
import PineconeSVM from '@/components/PineconeSVM';
import RegionSelector from '@/components/RegionSelector';
import SectorSelector from '@/components/SectorSelector';
import ClientSelector from '@/components/ClientSelector';

interface ComponentCategory {
  id: string;
  name: string;
  icon: React.ComponentType<any>;
  description: string;
  components: {
    name: string;
    component: React.ComponentType<any>;
    description: string;
  }[];
}

const ComponentShowcase: React.FC = () => {
  const [selectedCategory, setSelectedCategory] = useState<string>('admin');
  const [selectedComponent, setSelectedComponent] = useState<string | null>(null);

  const categories: ComponentCategory[] = [
    {
      id: 'admin',
      name: 'Admin Dashboard',
      icon: Settings,
      description: 'System administration and management components',
      components: [
        { name: 'AdminDashboard', component: AdminDashboard, description: 'Main admin dashboard' },
        { name: 'SystemOverview', component: SystemOverview, description: 'System overview panel' },
        { name: 'Localization', component: Localization, description: 'Localization settings' },
        { name: 'PackageStatus', component: PackageStatus, description: 'Package status monitoring' },
        { name: 'CodeViewer', component: CodeViewer, description: 'Code viewing component' },
        { name: 'ProjectExport', component: ProjectExport, description: 'Project export functionality' },
        { name: 'ThreatFeeds', component: ThreatFeeds, description: 'Threat feed management' },
        { name: 'APIStore', component: APIStore, description: 'API store interface' }
      ]
    },
    {
      id: 'redteam',
      name: 'Red Team',
      icon: Target,
      description: 'Red team operations and testing components',
      components: [
        { name: 'RedTeamRunner', component: RedTeamRunner, description: 'Red team test runner' },
        { name: 'PhaseMapping', component: PhaseMapping, description: 'Phase mapping interface' },
        { name: 'RecordList', component: RecordList, description: 'Record listing component' },
        { name: 'TestList', component: TestList, description: 'Test list management' },
        { name: 'PhaseSelector', component: PhaseSelector, description: 'Phase selection interface' }
      ]
    },
    {
      id: 'atomic',
      name: 'Atomic Red Team',
      icon: Zap,
      description: 'Atomic Red Team testing components',
      components: [
        { name: 'AtomicTestRunner', component: AtomicTestRunner, description: 'Atomic test runner' },
        { name: 'AtomicNDEXMapping', component: AtomicNDEXMapping, description: 'NDEX mapping interface' },
        { name: 'AttackNavigator', component: AttackNavigator, description: 'Attack navigation tool' }
      ]
    },
    {
      id: 'hash',
      name: 'Hash Composer',
      icon: Hash,
      description: 'Hash composition and analysis tools',
      components: [
        { name: 'HashComposer', component: HashComposer, description: 'Main hash composer' },
        { name: 'HashDisplay', component: HashDisplay, description: 'Hash display component' },
        { name: 'HashGrid', component: HashGrid, description: 'Hash grid interface' },
        { name: 'DomainSelector', component: DomainSelector, description: 'Domain selection' },
        { name: 'LayerSelector', component: LayerSelector, description: 'Layer selection' },
        { name: 'ExportPanel', component: ExportPanel, description: 'Export functionality' }
      ]
    },
    {
      id: 'cli',
      name: 'CLI Tools',
      icon: Terminal,
      description: 'Command line interface components',
      components: [
        { name: 'CTASCLI', component: CTASCLI, description: 'Main CTAS CLI' },
        { name: 'KaliCLI', component: KaliCLI, description: 'Kali Linux CLI' },
        { name: 'NetworkCLI', component: NetworkCLI, description: 'Network CLI tools' },
        { name: 'WSLEnvironment', component: WSLEnvironment, description: 'WSL environment' },
        { name: 'PythonIntegration', component: PythonIntegration, description: 'Python integration' }
      ]
    },
    {
      id: 'graphs',
      name: 'Graphs & Visualization',
      icon: BarChart3,
      description: 'Graph and data visualization components',
      components: [
        { name: 'Cognigraph', component: Cognigraph, description: 'Cognitive graph interface' },
        { name: 'KnowledgeGraph', component: KnowledgeGraph, description: 'Knowledge graph' },
        { name: 'Neo4jKnowledgeGraph', component: Neo4jKnowledgeGraph, description: 'Neo4j knowledge graph' },
        { name: 'GraphVisualizer', component: GraphVisualizer, description: 'Graph visualization' },
        { name: 'CTASGraph', component: CTASGraph, description: 'CTAS graph component' },
        { name: 'CTASGraphAnimation', component: CTASGraphAnimation, description: 'Animated CTAS graph' },
        { name: 'TransformsGraph', component: TransformsGraph, description: 'Transforms graph' },
        { name: 'HD4Graph', component: HD4Graph, description: 'HD4 graph component' }
      ]
    },
    {
      id: 'maps',
      name: 'Maps & Geospatial',
      icon: Map,
      description: 'Mapping and geospatial components',
      components: [
        { name: 'EnhancedMap', component: EnhancedMap, description: 'Enhanced map with Mapbox' },
        { name: 'SimpleMap', component: SimpleMap, description: 'Simple CSS-based map' },
        { name: 'GISMap', component: GISMap, description: 'GIS mapping component' },
        { name: 'HD4Map', component: HD4MapComponent, description: 'HD4 map component' },
        { name: 'NetworkMap', component: NetworkMap, description: 'Network mapping' },
        { name: 'MapView', component: MapView, description: 'Map view component' },
        { name: 'GeospatialDataManager', component: GeospatialDataManager, description: 'Geospatial data management' }
      ]
    },
    {
      id: 'intel',
      name: 'Intelligence',
      icon: Eye,
      description: 'Intelligence gathering and analysis',
      components: [
        { name: 'Shodan', component: Shodan, description: 'Shodan intelligence' },
        { name: 'OSINTModule', component: OSINTModule, description: 'OSINT module' },
        { name: 'PhishingModule', component: PhishingModule, description: 'Phishing module' },
        { name: 'KaliToolsIntegration', component: KaliToolsIntegration, description: 'Kali tools integration' },
        { name: 'AnsiblePlaybooks', component: AnsiblePlaybooks, description: 'Ansible playbooks' },
        { name: 'CTASPlaybooks', component: CTASPlaybooks, description: 'CTAS playbooks' },
        { name: 'HD4Playbooks', component: HD4Playbooks, description: 'HD4 playbooks' },
        { name: 'Playbooks', component: Playbooks, description: 'General playbooks' }
      ]
    },
    {
      id: 'tasks',
      name: 'Task Management',
      icon: FileText,
      description: 'Task and workflow management',
      components: [
        { name: 'TaskManagement', component: TaskManagement, description: 'Task management interface' },
        { name: 'TaskList', component: TaskList, description: 'Task list component' },
        { name: 'CTASTaskList', component: CTASTaskList, description: 'CTAS task list' },
        { name: 'HD4TaskView', component: HD4TaskView, description: 'HD4 task view' }
      ]
    },
    {
      id: 'threats',
      name: 'Threat Intelligence',
      icon: Shield,
      description: 'Threat intelligence and actor tracking',
      components: [
        { name: 'ThreatActorList', component: ThreatActorList, description: 'Threat actor listing' },
        { name: 'TopAPTs', component: TopAPTs, description: 'Top APT groups' },
        { name: 'CriticalInfrastructureDashboard', component: CriticalInfrastructureDashboard, description: 'Critical infrastructure dashboard' },
        { name: 'CriticalInfrastructureSectors', component: CriticalInfrastructureSectors, description: 'Critical infrastructure sectors' }
      ]
    },
    {
      id: 'infrastructure',
      name: 'Infrastructure',
      icon: Server,
      description: 'Infrastructure management components',
      components: [
        { name: 'RedisManager', component: RedisManager, description: 'Redis management' },
        { name: 'NetworksControl', component: NetworksControl, description: 'Network control' },
        { name: 'IoTControl', component: IoTControl, description: 'IoT control interface' },
        { name: 'StreamProcessor', component: StreamProcessor, description: 'Stream processing' },
        { name: 'AnimatedInfoStreams', component: AnimatedInfoStreams, description: 'Animated info streams' }
      ]
    },
    {
      id: 'hd4',
      name: 'HD4 Framework',
      icon: Brain,
      description: 'HD4 framework components',
      components: [
        { name: 'HD4PhaseContent', component: HD4PhaseContent, description: 'HD4 phase content' },
        { name: 'HD4OperationalSpectrum', component: HD4OperationalSpectrum, description: 'HD4 operational spectrum' }
      ]
    },
    {
      id: 'workflows',
      name: 'Workflows & Automation',
      icon: GitBranch,
      description: 'Workflow and automation components',
      components: [
        { name: 'N8NWorkflows', component: N8NWorkflows, description: 'N8N workflow integration' },
        { name: 'SearchInterface', component: SearchInterface, description: 'Search interface' },
        { name: 'ControlPanel', component: ControlPanel, description: 'Control panel' },
        { name: 'AIIntegration', component: AIIntegration, description: 'AI integration' },
        { name: 'VirtualRavenDashboard', component: VirtualRavenDashboard, description: 'Virtual Raven dashboard' }
      ]
    },
    {
      id: 'operations',
      name: 'Operations',
      icon: Cpu,
      description: 'Operational components',
      components: [
        { name: 'OperationsOverview', component: OperationsOverview, description: 'Operations overview' },
        { name: 'OperationsSummary', component: OperationsSummary, description: 'Operations summary' },
        { name: 'SystemArchitectureDiagram', component: SystemArchitectureDiagram, description: 'System architecture diagram' },
        { name: 'CTASSystemDiagram', component: CTASSystemDiagram, description: 'CTAS system diagram' }
      ]
    },
    {
      id: 'ui',
      name: 'UI Components',
      icon: Palette,
      description: 'User interface components',
      components: [
        { name: 'PersonasList', component: PersonasList, description: 'Personas listing' },
        { name: 'NISTvsHD4Comparison', component: NISTvsHD4Comparison, description: 'NIST vs HD4 comparison' },
        { name: 'SpreadsheetDisplay', component: SpreadsheetDisplay, description: 'Spreadsheet display' },
        { name: 'GridView', component: GridView, description: 'Grid view component' },
        { name: 'ViewToggle', component: ViewToggle, description: 'View toggle component' },
        { name: 'DocsScreen', component: DocsScreen, description: 'Documentation screen' }
      ]
    },
    {
      id: 'utilities',
      name: 'Utilities',
      icon: Code,
      description: 'Utility and helper components',
      components: [
        { name: 'ScriptScrapers', component: ScriptScrapers, description: 'Script scraping tools' },
        { name: 'PineconeSVM', component: PineconeSVM, description: 'Pinecone SVM integration' },
        { name: 'RegionSelector', component: RegionSelector, description: 'Region selection' },
        { name: 'SectorSelector', component: SectorSelector, description: 'Sector selection' },
        { name: 'ClientSelector', component: ClientSelector, description: 'Client selection' }
      ]
    }
  ];

  const currentCategory = categories.find(cat => cat.id === selectedCategory);
  const currentComponent = currentCategory?.components.find(comp => comp.name === selectedComponent);

  return (
    <div className="h-full w-full bg-gray-900 text-white">
      <div className="h-full flex">
        {/* Sidebar */}
        <div className="w-80 bg-gray-800 border-r border-gray-700 overflow-y-auto">
          <div className="p-4 border-b border-gray-700">
            <h1 className="text-xl font-bold">Component Showcase</h1>
            <p className="text-sm text-gray-400 mt-1">
              Explore all CTAS components
            </p>
          </div>
          
          <div className="p-4">
            <div className="space-y-2">
              {categories.map((category) => (
                <div key={category.id}>
                  <button
                    onClick={() => {
                      setSelectedCategory(category.id);
                      setSelectedComponent(null);
                    }}
                    className={`w-full text-left p-3 rounded-lg transition-colors ${
                      selectedCategory === category.id
                        ? 'bg-blue-600 text-white'
                        : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                    }`}
                  >
                    <div className="flex items-center space-x-3">
                      <category.icon className="w-5 h-5" />
                      <div>
                        <div className="font-medium">{category.name}</div>
                        <div className="text-xs opacity-75">{category.description}</div>
                      </div>
                    </div>
                  </button>
                  
                  {selectedCategory === category.id && (
                    <div className="mt-2 ml-4 space-y-1">
                      {category.components.map((component) => (
                        <button
                          key={component.name}
                          onClick={() => setSelectedComponent(component.name)}
                          className={`w-full text-left p-2 rounded text-sm transition-colors ${
                            selectedComponent === component.name
                              ? 'bg-blue-500 text-white'
                              : 'text-gray-400 hover:text-white hover:bg-gray-600'
                          }`}
                        >
                          {component.name}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Main Content */}
        <div className="flex-1 flex flex-col">
          {currentCategory && (
            <div className="p-6">
              <div className="mb-6">
                <h2 className="text-2xl font-bold mb-2">{currentCategory.name}</h2>
                <p className="text-gray-400">{currentCategory.description}</p>
              </div>

              {currentComponent ? (
                <div className="bg-gray-800 rounded-lg p-6">
                  <div className="mb-4">
                    <h3 className="text-xl font-semibold mb-2">{currentComponent.name}</h3>
                    <p className="text-gray-400">{currentComponent.description}</p>
                  </div>
                  
                  <div className="border border-gray-700 rounded-lg p-4 bg-gray-900">
                    <currentComponent.component />
                  </div>
                </div>
              ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {currentCategory.components.map((component) => (
                    <div
                      key={component.name}
                      className="bg-gray-800 rounded-lg p-4 cursor-pointer hover:bg-gray-700 transition-colors"
                      onClick={() => setSelectedComponent(component.name)}
                    >
                      <h3 className="font-semibold mb-2">{component.name}</h3>
                      <p className="text-sm text-gray-400">{component.description}</p>
                      <div className="mt-3 text-xs text-blue-400">Click to view</div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ComponentShowcase;
