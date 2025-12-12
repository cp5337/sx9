import React, { useState } from 'react';
import { 
  CTASStatCard, 
  CTASMetricCard, 
  CTASAlertCard, 
  CTASDataTable,
  CTASChat,
  CTASLineChart,
  CTASBadge,
  CTASProgress
} from '../index';
import { 
  Activity, 
  Shield, 
  AlertTriangle, 
  Globe, 
  Target, 
  Database,
  Server,
  Zap,
  MessageSquare,
  BarChart3,
  Users,
  Calendar,
  Clock,
  Video,
  Phone,
  Mic,
  MicOff,
  Camera,
  CameraOff,
  Bell,
  Settings,
  User,
  LogOut,
  Search,
  Filter,
  Plus,
  MoreHorizontal,
  Mail,
  Bot,
  Cpu,
  Network,
  Brain,
  Eye,
  Lock,
  Unlock,
  ArrowLeft,
  FileText,
  TrendingUp,
  TrendingDown,
  Map,
  DollarSign,
  Building,
  Radio,
  Satellite,
  Code,
  Key,
  Fingerprint,
  Shield as ShieldIcon,
  Zap as ZapIcon,
  Target as TargetIcon,
  ChevronDown,
  ChevronRight,
  MapPin,
  Layers,
  Compass,
  Navigation,
  Send,
  Paperclip,
  Download,
  Upload,
  Folder,
  File,
  Image,
  Video as VideoIcon,
  Mic as MicIcon,
  Phone as PhoneIcon
} from 'lucide-react';
import { demoDataProvider } from '../../../utils/demoDataProvider';

interface PersonaHomeProps {
  personaId: string;
  onBack: () => void;
}

const PersonaHome: React.FC<PersonaHomeProps> = ({ personaId, onBack }) => {
  const [activeTab, setActiveTab] = useState('hunt');
  const [expandedAccordions, setExpandedAccordions] = useState<string[]>(['active-tasks']);
  const [framework, setFramework] = useState<'hd4' | 'killchain'>('hd4');

  // Persona definitions with AOR data
  const personas = {
    natasha: {
      name: 'Natasha Volkov',
      role: 'AI/ML Technical Lead',
      icon: Brain,
      color: 'bg-gray-600',
      expertise: ['Neural Networks', 'AI Frameworks', 'Mathematical Models'],
      description: 'Core AI & Mathematical Frameworks Specialist',
      aor: {
        regions: ['Global AI Infrastructure', 'Neural Network Clusters', 'ML Model Repositories'],
        coordinates: [
          { lat: 40.7128, lng: -74.0060, name: 'NYC AI Hub', type: 'primary' },
          { lat: 37.7749, lng: -122.4194, name: 'SF ML Center', type: 'secondary' },
          { lat: 51.5074, lng: -0.1278, name: 'London AI Lab', type: 'secondary' },
          { lat: 35.6762, lng: 139.6503, name: 'Tokyo Neural Net', type: 'secondary' }
        ]
      },
      capabilities: [
        'Neural Lattice (NLX) Analysis',
        'Synaptic Convergent Hashing',
        'CTAS RepoPrompt System',
        'AI CLI Framework Development'
      ],
      metrics: {
        activeModels: 23,
        processingPower: 94,
        accuracy: 98.7,
        responseTime: 0.12
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Neural Pattern Analysis', status: 'in-progress', priority: 'high', progress: 75 },
          { id: 2, title: 'Model Optimization', status: 'pending', priority: 'medium', progress: 0 },
          { id: 3, title: 'Framework Update', status: 'completed', priority: 'low', progress: 100 }
        ],
        'ai-models': [
          { id: 1, name: 'GPT-4 Integration', status: 'active', accuracy: 98.7 },
          { id: 2, name: 'Claude Analysis', status: 'active', accuracy: 97.3 },
          { id: 3, name: 'Custom NLX Model', status: 'training', accuracy: 89.2 }
        ],
        'neural-networks': [
          { id: 1, name: 'Pattern Recognition', nodes: 1024, layers: 8, status: 'online' },
          { id: 2, name: 'Threat Detection', nodes: 512, layers: 6, status: 'online' },
          { id: 3, name: 'Behavioral Analysis', nodes: 2048, layers: 12, status: 'training' }
        ]
      }
    },
    omar: {
      name: 'Omar Al-Rashid',
      role: 'MENA Operations',
      icon: Eye,
      color: 'bg-gray-600',
      expertise: ['Cultural Intelligence', 'Arabic NLP', 'Regional Analysis'],
      description: 'MENA Operations & Cultural Intelligence Specialist',
      aor: {
        regions: ['Middle East', 'North Africa', 'Gulf States', 'Levant'],
        coordinates: [
          { lat: 24.7136, lng: 46.6753, name: 'Riyadh Hub', type: 'primary' },
          { lat: 25.2048, lng: 55.2708, name: 'Dubai Ops', type: 'secondary' },
          { lat: 30.0444, lng: 31.2357, name: 'Cairo Intel', type: 'secondary' },
          { lat: 33.3152, lng: 44.3661, name: 'Baghdad Station', type: 'secondary' },
          { lat: 31.9539, lng: 35.9106, name: 'Amman Base', type: 'secondary' }
        ]
      },
      capabilities: [
        'Geographic Threat Correlation',
        'Cultural Analysis Algorithms',
        'Regional Persona Generation',
        'Arabic/Farsi NLP Processing'
      ],
      metrics: {
        activeRegions: 8,
        culturalPatterns: 156,
        threatCorrelations: 42,
        responseTime: 0.18
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Regional Threat Assessment', status: 'in-progress', priority: 'critical', progress: 60 },
          { id: 2, title: 'Cultural Pattern Analysis', status: 'pending', priority: 'high', progress: 0 },
          { id: 3, title: 'Language Model Training', status: 'completed', priority: 'medium', progress: 100 }
        ],
        'regional-intel': [
          { id: 1, name: 'Saudi Arabia', threats: 12, status: 'monitoring' },
          { id: 2, name: 'UAE', threats: 8, status: 'active' },
          { id: 3, name: 'Egypt', threats: 15, status: 'critical' },
          { id: 4, name: 'Iraq', threats: 23, status: 'critical' }
        ],
        'cultural-analysis': [
          { id: 1, name: 'Arabic Dialects', patterns: 45, confidence: 94 },
          { id: 2, name: 'Social Networks', patterns: 67, confidence: 87 },
          { id: 3, name: 'Religious Groups', patterns: 34, confidence: 92 }
        ]
      }
    },
    chen: {
      name: 'Chen Wei',
      role: 'Chinese Operations',
      icon: Network,
      color: 'bg-gray-600',
      expertise: ['Economic Warfare', 'Supply Chain', 'Financial Analysis'],
      description: 'Chinese Operations & Economic Warfare Specialist',
      aor: {
        regions: ['China', 'Asia-Pacific', 'Global Supply Chains', 'Financial Networks'],
        coordinates: [
          { lat: 39.9042, lng: 116.4074, name: 'Beijing HQ', type: 'primary' },
          { lat: 31.2304, lng: 121.4737, name: 'Shanghai Finance', type: 'secondary' },
          { lat: 22.3193, lng: 114.1694, name: 'Hong Kong Trade', type: 'secondary' },
          { lat: 35.8617, lng: 104.1954, name: 'Lanzhou Supply', type: 'secondary' }
        ]
      },
      capabilities: [
        'Financial Flow Analysis',
        'Blockchain Tracking',
        'Economic Modeling',
        'Supply Chain Disruption'
      ],
      metrics: {
        activeNetworks: 12,
        financialFlows: 89,
        economicModels: 34,
        responseTime: 0.15
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Supply Chain Analysis', status: 'in-progress', priority: 'high', progress: 45 },
          { id: 2, title: 'Financial Flow Tracking', status: 'pending', priority: 'critical', progress: 0 },
          { id: 3, title: 'Economic Impact Assessment', status: 'completed', priority: 'medium', progress: 100 }
        ],
        'financial-networks': [
          { id: 1, name: 'Belt & Road', flows: 156, status: 'monitoring' },
          { id: 2, name: 'Digital Yuan', flows: 89, status: 'tracking' },
          { id: 3, name: 'Crypto Networks', flows: 234, status: 'active' }
        ],
        'supply-chains': [
          { id: 1, name: 'Semiconductors', nodes: 45, status: 'critical' },
          { id: 2, name: 'Rare Earths', nodes: 23, status: 'monitoring' },
          { id: 3, name: 'Manufacturing', nodes: 67, status: 'active' }
        ]
      }
    },
    hayes: {
      name: 'Commander Hayes',
      role: 'Kinetic Operations',
      icon: Shield,
      color: 'bg-gray-600',
      expertise: ['System Integration', 'EOD', 'Mobile Command'],
      description: 'Kinetic Operations & System Architecture Specialist',
      aor: {
        regions: ['Global Operations', 'Forward Bases', 'Mobile Units', 'Training Facilities'],
        coordinates: [
          { lat: 38.8977, lng: -77.0365, name: 'Pentagon Ops', type: 'primary' },
          { lat: 36.1699, lng: -115.1398, name: 'Nellis Training', type: 'secondary' },
          { lat: 32.7767, lng: -96.7970, name: 'Fort Hood', type: 'secondary' },
          { lat: 35.2271, lng: -80.8431, name: 'Fort Bragg', type: 'secondary' }
        ]
      },
      capabilities: [
        'Kinetic Effects Modeling',
        'IED/Drone Systems Analysis',
        'System Integration',
        'Mobile Command Operations'
      ],
      metrics: {
        activeOperations: 6,
        systemIntegrations: 18,
        kineticModels: 27,
        responseTime: 0.08
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Kinetic Effects Modeling', status: 'in-progress', priority: 'critical', progress: 85 },
          { id: 2, title: 'System Integration Test', status: 'pending', priority: 'high', progress: 0 },
          { id: 3, title: 'Mobile Command Setup', status: 'completed', priority: 'medium', progress: 100 }
        ],
        'kinetic-operations': [
          { id: 1, name: 'Alpha Team', status: 'deployed', location: 'Forward Base A' },
          { id: 2, name: 'Bravo Team', status: 'training', location: 'Training Facility' },
          { id: 3, name: 'Charlie Team', status: 'standby', location: 'Home Base' }
        ],
        'system-integrations': [
          { id: 1, name: 'Drone Systems', integration: 95, status: 'online' },
          { id: 2, name: 'IED Detection', integration: 87, status: 'online' },
          { id: 3, name: 'Mobile Command', integration: 92, status: 'online' }
        ]
      }
    },
    emily: {
      name: 'Emily Carter',
      role: 'Infrastructure Security',
      icon: Lock,
      color: 'bg-gray-600',
      expertise: ['Cloud Security', 'Digital Twins', 'Critical Infrastructure'],
      description: 'Tech Infrastructure & Cloud Security Specialist',
      aor: {
        regions: ['Global Infrastructure', 'Cloud Networks', 'Critical Systems', 'Data Centers'],
        coordinates: [
          { lat: 37.7749, lng: -122.4194, name: 'SF Cloud Hub', type: 'primary' },
          { lat: 40.7128, lng: -74.0060, name: 'NYC Data Center', type: 'secondary' },
          { lat: 51.5074, lng: -0.1278, name: 'London Infrastructure', type: 'secondary' },
          { lat: 35.6762, lng: 139.6503, name: 'Tokyo Network', type: 'secondary' }
        ]
      },
      capabilities: [
        'Cloud Deployment Architectures',
        'Digital Twins',
        'Infrastructure Protection',
        'Critical Systems Security'
      ],
      metrics: {
        activeSystems: 45,
        securityProtocols: 67,
        infrastructureNodes: 234,
        responseTime: 0.14
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Infrastructure Security Audit', status: 'in-progress', priority: 'high', progress: 65 },
          { id: 2, title: 'Digital Twin Deployment', status: 'pending', priority: 'medium', progress: 0 },
          { id: 3, title: 'Cloud Security Update', status: 'completed', priority: 'low', progress: 100 }
        ],
        'critical-infrastructure': [
          { id: 1, name: 'Power Grid', security: 94, status: 'protected' },
          { id: 2, name: 'Water Systems', security: 87, status: 'monitoring' },
          { id: 3, name: 'Transportation', security: 92, status: 'protected' },
          { id: 4, name: 'Communications', security: 96, status: 'protected' }
        ],
        'cloud-security': [
          { id: 1, name: 'AWS Infrastructure', nodes: 156, status: 'secure' },
          { id: 2, name: 'Azure Systems', nodes: 89, status: 'secure' },
          { id: 3, name: 'Google Cloud', nodes: 67, status: 'monitoring' }
        ]
      }
    },
    james: {
      name: 'James Mitchell',
      role: 'Financial Intelligence',
      icon: Cpu,
      color: 'bg-gray-600',
      expertise: ['Blockchain Analysis', 'Cross-border Tracking', 'Cryptocurrency'],
      description: 'Financial Intelligence & Blockchain Analysis Specialist',
      aor: {
        regions: ['Global Financial Networks', 'Cryptocurrency Exchanges', 'Blockchain Networks', 'Banking Systems'],
        coordinates: [
          { lat: 40.7128, lng: -74.0060, name: 'NYC Finance', type: 'primary' },
          { lat: 51.5074, lng: -0.1278, name: 'London Banking', type: 'secondary' },
          { lat: 35.6762, lng: 139.6503, name: 'Tokyo Markets', type: 'secondary' },
          { lat: 22.3193, lng: 114.1694, name: 'Hong Kong Finance', type: 'secondary' }
        ]
      },
      capabilities: [
        'Cross-border Financial Tracking',
        'Cryptocurrency Analysis',
        'Money Laundering Detection',
        'Financial Network Mapping'
      ],
      metrics: {
        activeTransactions: 156,
        blockchainChains: 8,
        financialNetworks: 23,
        responseTime: 0.22
      },
      specialtyTasks: {
        'active-tasks': [
          { id: 1, title: 'Blockchain Analysis', status: 'in-progress', priority: 'high', progress: 55 },
          { id: 2, title: 'Cross-border Tracking', status: 'pending', priority: 'critical', progress: 0 },
          { id: 3, title: 'Financial Network Mapping', status: 'completed', priority: 'medium', progress: 100 }
        ],
        'financial-networks': [
          { id: 1, name: 'SWIFT Network', transactions: 234, status: 'monitoring' },
          { id: 2, name: 'Cryptocurrency', transactions: 567, status: 'tracking' },
          { id: 3, name: 'Dark Web', transactions: 89, status: 'investigating' }
        ],
        'blockchain-analysis': [
          { id: 1, name: 'Bitcoin Network', nodes: 12345, status: 'tracking' },
          { id: 2, name: 'Ethereum', nodes: 8765, status: 'monitoring' },
          { id: 3, name: 'Privacy Coins', nodes: 2345, status: 'investigating' }
        ]
      }
    }
  };

  const persona = personas[personaId as keyof typeof personas];
  if (!persona) return null;

  // Get demo data
  const infoStreams = demoDataProvider.getInfoStreams();

  const toggleAccordion = (accordionId: string) => {
    setExpandedAccordions(prev => 
      prev.includes(accordionId) 
        ? prev.filter(id => id !== accordionId)
        : [...prev, accordionId]
    );
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'critical': return 'text-red-600';
      case 'high': return 'text-orange-600';
      case 'medium': return 'text-yellow-600';
      case 'low': return 'text-green-600';
      default: return 'text-gray-600';
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'in-progress': return 'bg-blue-100 text-blue-800';
      case 'pending': return 'bg-yellow-100 text-yellow-800';
      case 'completed': return 'bg-green-100 text-green-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="bg-gray-50 dark:bg-gray-900 min-h-screen">
      {/* Header */}
      <div className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-12">
            {/* Left */}
            <div className="flex items-center space-x-3">
              <button
                onClick={onBack}
                className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              >
                <ArrowLeft className="w-4 h-4" />
              </button>
              <div className="flex items-center space-x-2">
                <div className={`w-8 h-8 ${persona.color} rounded-full flex items-center justify-center`}>
                  <persona.icon className="w-5 h-5 text-white" />
                </div>
                <div>
                  <span className="text-sm font-medium text-gray-900 dark:text-white">{persona.name}</span>
                  <span className="text-xs text-gray-500 dark:text-gray-400 ml-2">{persona.role}</span>
                </div>
              </div>
            </div>

            {/* Center - Tabs */}
            <div className="flex items-center space-x-1">
              {framework === 'hd4' ? (
                <>
                  <button
                    onClick={() => setActiveTab('hunt')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'hunt'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Eye className="w-3 h-3" />
                    <span>Hunt</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('detect')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'detect'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Target className="w-3 h-3" />
                    <span>Detect</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('disrupt')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'disrupt'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Zap className="w-3 h-3" />
                    <span>Disrupt</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('disable')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'disable'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Shield className="w-3 h-3" />
                    <span>Disable</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('dominate')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'dominate'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Activity className="w-3 h-3" />
                    <span>Dominate</span>
                  </button>
                </>
              ) : (
                <>
                  <button
                    onClick={() => setActiveTab('reconnaissance')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'reconnaissance'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Eye className="w-3 h-3" />
                    <span>Reconnaissance</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('weaponization')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'weaponization'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Target className="w-3 h-3" />
                    <span>Weaponization</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('delivery')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'delivery'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Zap className="w-3 h-3" />
                    <span>Delivery</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('exploitation')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'exploitation'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Shield className="w-3 h-3" />
                    <span>Exploitation</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('installation')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'installation'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Server className="w-3 h-3" />
                    <span>Installation</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('command')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'command'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Cpu className="w-3 h-3" />
                    <span>Command</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('actions')}
                    className={`flex items-center space-x-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
                      activeTab === 'actions'
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                    }`}
                  >
                    <Activity className="w-3 h-3" />
                    <span>Actions</span>
                  </button>
                </>
              )}
            </div>

            {/* Right */}
            <div className="flex items-center space-x-2">
              <CTASBadge variant="success" size="sm">Online</CTASBadge>
              <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                <Settings className="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto p-4">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
          {/* Left Sidebar - Specialty Tasks */}
          <div className="lg:col-span-1 space-y-4">
            {/* Persona Info */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-3">
              <h3 className="font-medium text-gray-900 dark:text-white mb-2 text-sm">Specialist Profile</h3>
              <p className="text-xs text-gray-600 dark:text-gray-400 mb-3">{persona.description}</p>
              <div className="grid grid-cols-2 gap-2">
                {Object.entries(persona.metrics).map(([key, value]) => (
                  <div key={key} className="text-center">
                    <div className="text-sm font-medium text-gray-900 dark:text-white">
                      {typeof value === 'number' && value > 100 ? value.toLocaleString() : value}
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400 capitalize">
                      {key.replace(/([A-Z])/g, ' $1').trim()}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Framework Toggle */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-3">
              <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Operational Framework</h3>
              <div className="space-y-2">
                <button
                  onClick={() => setFramework('hd4')}
                  className={`w-full flex items-center justify-between p-2 text-left rounded transition-colors ${
                    framework === 'hd4' 
                      ? 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white' 
                      : 'hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-400'
                  }`}
                >
                  <span className="text-xs font-medium">HD4 Framework</span>
                  {framework === 'hd4' && <div className="w-2 h-2 bg-blue-500 rounded-full"></div>}
                </button>
                <button
                  onClick={() => setFramework('killchain')}
                  className={`w-full flex items-center justify-between p-2 text-left rounded transition-colors ${
                    framework === 'killchain' 
                      ? 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white' 
                      : 'hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-400'
                  }`}
                >
                  <span className="text-xs font-medium">Kill Chain</span>
                  {framework === 'killchain' && <div className="w-2 h-2 bg-red-500 rounded-full"></div>}
                </button>
              </div>
            </div>

            {/* Specialty Tasks Accordions */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-3">
              <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Specialty Tasks</h3>
              <div className="space-y-2">
                {Object.entries(persona.specialtyTasks).map(([sectionId, tasks]) => (
                  <div key={sectionId} className="border border-gray-200 dark:border-gray-600 rounded">
                    <button
                      onClick={() => toggleAccordion(sectionId)}
                      className="w-full flex items-center justify-between p-2 text-left hover:bg-gray-50 dark:hover:bg-gray-700"
                    >
                      <span className="text-xs font-medium text-gray-900 dark:text-white capitalize">
                        {sectionId.replace(/-/g, ' ')}
                      </span>
                      {expandedAccordions.includes(sectionId) ? (
                        <ChevronDown className="w-3 h-3 text-gray-500" />
                      ) : (
                        <ChevronRight className="w-3 h-3 text-gray-500" />
                      )}
                    </button>
                    {expandedAccordions.includes(sectionId) && (
                      <div className="p-2 border-t border-gray-200 dark:border-gray-600 space-y-2">
                        {tasks.map((task: any) => (
                          <div key={task.id} className="text-xs">
                            <div className="flex items-center justify-between">
                              <span className="text-gray-900 dark:text-white">{task.title || task.name}</span>
                              {task.priority && (
                                <span className={`text-xs ${getPriorityColor(task.priority)}`}>
                                  {task.priority}
                                </span>
                              )}
                            </div>
                            {task.status && (
                              <div className="flex items-center justify-between mt-1">
                                <span className={`px-1 py-0.5 rounded text-xs ${getStatusColor(task.status)}`}>
                                  {task.status}
                                </span>
                                {task.progress !== undefined && (
                                  <span className="text-xs text-gray-500">{task.progress}%</span>
                                )}
                              </div>
                            )}
                            {task.progress !== undefined && (
                              <CTASProgress value={task.progress} variant={task.progress >= 80 ? 'success' : task.progress >= 50 ? 'warning' : 'danger'} />
                            )}
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Main Content Area */}
          <div className="lg:col-span-3 space-y-4">
            {/* Reconnaissance Tab */}
            {activeTab === 'reconnaissance' && (
              <div className="space-y-4">
                {/* System Overview Cards */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
                  <CTASStatCard
                    title="Active Operations"
                    value={('activeOperations' in persona.metrics ? persona.metrics.activeOperations : 
                           'activeModels' in persona.metrics ? persona.metrics.activeModels :
                           'activeRegions' in persona.metrics ? persona.metrics.activeRegions :
                           'activeNetworks' in persona.metrics ? persona.metrics.activeNetworks :
                           'activeSystems' in persona.metrics ? persona.metrics.activeSystems :
                           'activeTransactions' in persona.metrics ? persona.metrics.activeTransactions : 0)}
                    icon={Activity}
                    status="success"
                  />
                  <CTASStatCard
                    title="Processing Power"
                    value={`${('processingPower' in persona.metrics ? persona.metrics.processingPower : 85)}%`}
                    icon={Zap}
                    status="warning"
                  />
                  <CTASStatCard
                    title="Response Time"
                    value={`${persona.metrics.responseTime}s`}
                    icon={Clock}
                    status="info"
                  />
                  <CTASStatCard
                    title="Accuracy"
                    value={`${('accuracy' in persona.metrics ? persona.metrics.accuracy : 95)}%`}
                    icon={Target}
                    status="success"
                  />
                </div>

                {/* Capabilities */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Core Capabilities</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                    {persona.capabilities.map((capability, index) => (
                      <div key={index} className="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-700 rounded">
                        <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                        <span className="text-sm text-gray-900 dark:text-white">{capability}</span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Recent Alerts */}
                <div className="space-y-3">
                  <h3 className="font-medium text-gray-900 dark:text-white text-sm">Recent Alerts</h3>
                  <div className="grid gap-3">
                    {infoStreams.slice(0, 2).map((stream, index) => (
                      <CTASAlertCard
                        key={stream.id}
                        type={stream.priority === 'critical' ? 'danger' : stream.priority === 'high' ? 'warning' : 'info'}
                        title={stream.title}
                        message={stream.content}
                        onClose={() => console.log(`Close alert ${stream.id}`)}
                      />
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* Weaponization Tab */}
            {activeTab === 'weaponization' && (
              <div className="space-y-4">
                {/* GIS Header */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="font-medium text-gray-900 dark:text-white text-sm">Area of Responsibility (AOR)</h3>
                    <div className="flex items-center space-x-2">
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Layers className="w-4 h-4" />
                      </button>
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Compass className="w-4 h-4" />
                      </button>
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Navigation className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
                    {persona.aor.regions.map((region, index) => (
                      <CTASBadge key={index} variant="info" size="sm">{region}</CTASBadge>
                    ))}
                  </div>
                </div>

                {/* GIS Map Placeholder */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <div className="aspect-video bg-gray-100 dark:bg-gray-700 rounded flex items-center justify-center">
                    <div className="text-center">
                      <Map className="w-12 h-12 text-gray-400 mx-auto mb-2" />
                      <p className="text-sm text-gray-500 dark:text-gray-400">GIS Map Display</p>
                      <p className="text-xs text-gray-400 dark:text-gray-500">Interactive map with {persona.aor.coordinates.length} operational nodes</p>
                    </div>
                  </div>
                </div>

                {/* Operational Nodes */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Operational Nodes</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {persona.aor.coordinates.map((node, index) => (
                      <div key={index} className="flex items-center space-x-3 p-2 bg-gray-50 dark:bg-gray-700 rounded">
                        <div className={`w-3 h-3 rounded-full ${node.type === 'primary' ? 'bg-red-500' : 'bg-blue-500'}`}></div>
                        <div>
                          <div className="text-sm font-medium text-gray-900 dark:text-white">{node.name}</div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">
                            {node.lat.toFixed(4)}, {node.lng.toFixed(4)}
                          </div>
                        </div>
                        <CTASBadge variant={node.type === 'primary' ? 'danger' : 'info'} size="sm">
                          {node.type}
                        </CTASBadge>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* Delivery Tab */}
            {activeTab === 'delivery' && (
              <div className="space-y-4">
                {/* Communication Header */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="font-medium text-gray-900 dark:text-white text-sm">Communication Center</h3>
                    <div className="flex items-center space-x-2">
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Mail className="w-4 h-4" />
                      </button>
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Paperclip className="w-4 h-4" />
                      </button>
                      <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                        <Send className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
                    <button className="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                      <MessageSquare className="w-4 h-4 text-gray-600" />
                      <span className="text-xs text-gray-900 dark:text-white">Chat</span>
                    </button>
                    <button className="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                      <Mail className="w-4 h-4 text-gray-600" />
                      <span className="text-xs text-gray-900 dark:text-white">Email</span>
                    </button>
                    <button className="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                      <Video className="w-4 h-4 text-gray-600" />
                      <span className="text-xs text-gray-900 dark:text-white">Video</span>
                    </button>
                    <button className="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                      <Phone className="w-4 h-4 text-gray-600" />
                      <span className="text-xs text-gray-900 dark:text-white">Voice</span>
                    </button>
                  </div>
                </div>

                {/* Chat Interface */}
                <CTASChat
                  title={`${persona.name} Communication`}
                  placeholder={`Direct communication with ${persona.name}...`}
                  className="h-96"
                />

                {/* File Transfer */}
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">File Transfer</h3>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                    <button className="flex flex-col items-center p-3 border border-gray-200 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-700">
                      <Upload className="w-6 h-6 text-gray-600 mb-1" />
                      <span className="text-xs text-gray-900 dark:text-white">Upload</span>
                    </button>
                    <button className="flex flex-col items-center p-3 border border-gray-200 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-700">
                      <Download className="w-6 h-6 text-gray-600 mb-1" />
                      <span className="text-xs text-gray-900 dark:text-white">Download</span>
                    </button>
                    <button className="flex flex-col items-center p-3 border border-gray-200 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-700">
                      <Folder className="w-6 h-6 text-gray-600 mb-1" />
                      <span className="text-xs text-gray-900 dark:text-white">Documents</span>
                    </button>
                    <button className="flex flex-col items-center p-3 border border-gray-200 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-700">
                      <Image className="w-6 h-6 text-gray-600 mb-1" />
                      <span className="text-xs text-gray-900 dark:text-white">Media</span>
                    </button>
                  </div>
                </div>
              </div>
            )}

            {/* Exploitation Tab */}
            {activeTab === 'exploitation' && (
              <div className="space-y-4">
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Exploitation Operations</h3>
                  <div className="space-y-3">
                    {[1, 2, 3].map((op) => (
                      <div key={op} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                        <div>
                          <div className="text-sm font-medium text-gray-900 dark:text-white">Exploitation {op}</div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">Status: Active</div>
                        </div>
                        <CTASBadge variant="success" size="sm">Running</CTASBadge>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* Installation Tab */}
            {activeTab === 'installation' && (
              <div className="space-y-4">
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Installation Status</h3>
                  <div className="space-y-3">
                    {[1, 2, 3].map((op) => (
                      <div key={op} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                        <div>
                          <div className="text-sm font-medium text-gray-900 dark:text-white">Installation {op}</div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">Status: Complete</div>
                        </div>
                        <CTASBadge variant="success" size="sm">Installed</CTASBadge>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* Command Tab */}
            {activeTab === 'command' && (
              <div className="space-y-4">
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Command & Control</h3>
                  <div className="space-y-3">
                    {[1, 2, 3].map((op) => (
                      <div key={op} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                        <div>
                          <div className="text-sm font-medium text-gray-900 dark:text-white">C2 Channel {op}</div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">Status: Active</div>
                        </div>
                        <CTASBadge variant="success" size="sm">Connected</CTASBadge>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* Actions Tab */}
            {activeTab === 'actions' && (
              <div className="space-y-4">
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
                  <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Actions on Objectives</h3>
                  <div className="space-y-3">
                    {[1, 2, 3].map((op) => (
                      <div key={op} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                        <div>
                          <div className="text-sm font-medium text-gray-900 dark:text-white">Objective {op}</div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">Status: In Progress</div>
                        </div>
                        <CTASBadge variant="warning" size="sm">Executing</CTASBadge>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default PersonaHome;
