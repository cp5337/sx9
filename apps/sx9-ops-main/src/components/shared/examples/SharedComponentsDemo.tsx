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
  Unlock
} from 'lucide-react';
import { demoDataProvider } from '../../../utils/demoDataProvider';

const SharedComponentsDemo: React.FC = () => {
  const [activePersona, setActivePersona] = useState('natasha');
  const [isInMeeting, setIsInMeeting] = useState(false);

  // Get demo data
  const infoStreams = demoDataProvider.getInfoStreams();
  const shodanResults = demoDataProvider.getShodanResults();

  // AI-MCP Personas (Office)
  const personas = [
    { 
      id: 'natasha', 
      name: 'Natasha Volkov', 
      role: 'AI/ML Technical Lead', 
      status: 'online', 
      avatar: 'NV',
      icon: Brain,
      color: 'bg-gray-600',
      expertise: ['Neural Networks', 'AI Frameworks', 'Mathematical Models'],
      load: 85
    },
    { 
      id: 'omar', 
      name: 'Omar Al-Rashid', 
      role: 'MENA Operations', 
      status: 'online', 
      avatar: 'OA',
      icon: Eye,
      color: 'bg-gray-600',
      expertise: ['Cultural Intelligence', 'Arabic NLP', 'Regional Analysis'],
      load: 72
    },
    { 
      id: 'chen', 
      name: 'Chen Wei', 
      role: 'Chinese Operations', 
      status: 'away', 
      avatar: 'CW',
      icon: Network,
      color: 'bg-gray-600',
      expertise: ['Economic Warfare', 'Supply Chain', 'Financial Analysis'],
      load: 68
    },
    { 
      id: 'hayes', 
      name: 'Commander Hayes', 
      role: 'Kinetic Operations', 
      status: 'online', 
      avatar: 'CH',
      icon: Shield,
      color: 'bg-gray-600',
      expertise: ['System Integration', 'EOD', 'Mobile Command'],
      load: 95
    },
    { 
      id: 'emily', 
      name: 'Emily Carter', 
      role: 'Infrastructure Security', 
      status: 'online', 
      avatar: 'EC',
      icon: Lock,
      color: 'bg-gray-600',
      expertise: ['Cloud Security', 'Digital Twins', 'Critical Infrastructure'],
      load: 78
    },
    { 
      id: 'james', 
      name: 'James Mitchell', 
      role: 'Financial Intelligence', 
      status: 'offline', 
      avatar: 'JM',
      icon: Cpu,
      color: 'bg-gray-600',
      expertise: ['Blockchain Analysis', 'Cross-border Tracking', 'Cryptocurrency'],
      load: 45
    }
  ];

  // System metrics
  const systemMetrics = {
    totalAssets: 47,
    activeAssets: 42,
    alerts: 8,
    threats: 12
  };

  // Chart data
  const threatActivityData = [
    { x: 'Mon', y: 12 },
    { x: 'Tue', y: 19 },
    { x: 'Wed', y: 15 },
    { x: 'Thu', y: 27 },
    { x: 'Fri', y: 23 },
    { x: 'Sat', y: 18 },
    { x: 'Sun', y: 14 }
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online': return 'bg-green-500';
      case 'away': return 'bg-yellow-500';
      case 'offline': return 'bg-gray-500';
      default: return 'bg-gray-500';
    }
  };

  const getLoadColor = (load: number) => {
    if (load >= 80) return 'text-red-600';
    if (load >= 60) return 'text-yellow-600';
    return 'text-green-600';
  };

  return (
    <div className="bg-gray-50 dark:bg-gray-900 min-h-screen">
      {/* Top Navigation */}
      <div className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-12">
            {/* Left */}
            <div className="flex items-center space-x-3">
              <div className="flex items-center space-x-2">
                <Shield className="w-6 h-6 text-gray-600" />
                <span className="text-lg font-medium text-gray-900 dark:text-white">CTAS MCP Hub</span>
              </div>
              <CTASBadge variant="success" size="sm">Live</CTASBadge>
            </div>

            {/* Center */}
            <div className="flex items-center space-x-3">
              {isInMeeting && (
                <div className="flex items-center space-x-2 bg-red-50 dark:bg-red-900/10 px-3 py-1 rounded text-xs">
                  <div className="w-1.5 h-1.5 bg-red-500 rounded-full animate-pulse"></div>
                  <span className="text-red-700 dark:text-red-300 font-medium">Active Session</span>
                </div>
              )}
              <button
                onClick={() => setIsInMeeting(!isInMeeting)}
                className={`flex items-center space-x-1 px-3 py-1 rounded text-xs font-medium ${
                  isInMeeting 
                    ? 'bg-red-600 text-white hover:bg-red-700' 
                    : 'bg-gray-600 text-white hover:bg-gray-700'
                }`}
              >
                {isInMeeting ? <LogOut className="w-3 h-3" /> : <Video className="w-3 h-3" />}
                <span>{isInMeeting ? 'End' : 'Start'}</span>
              </button>
            </div>

            {/* Right */}
            <div className="flex items-center space-x-3">
              <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                <Bell className="w-4 h-4" />
              </button>
              <button className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                <Settings className="w-4 h-4" />
              </button>
              <div className="flex items-center space-x-2">
                <div className="w-6 h-6 bg-gray-600 rounded-full flex items-center justify-center">
                  <User className="w-3 h-3 text-white" />
                </div>
                <span className="text-sm text-gray-900 dark:text-white">Operator</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto p-4">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
          {/* Left Sidebar - AI-MCP Personas (Office) */}
          <div className="lg:col-span-1 space-y-4">
            {/* Active Persona */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-3">
              <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Active MCP</h3>
              {personas.filter(p => p.id === activePersona).map((persona) => (
                <div key={persona.id} className="text-center">
                  <div className={`w-12 h-12 ${persona.color} rounded-full flex items-center justify-center mx-auto mb-2`}>
                    <persona.icon className="w-6 h-6 text-white" />
                  </div>
                  <h4 className="font-medium text-gray-900 dark:text-white text-sm">{persona.name}</h4>
                  <p className="text-xs text-gray-600 dark:text-gray-400 mb-2">{persona.role}</p>
                  <div className="space-y-1 mb-3">
                    {persona.expertise.map((skill, index) => (
                      <CTASBadge key={index} variant="info" size="sm">{skill}</CTASBadge>
                    ))}
                  </div>
                  <div>
                    <div className="flex justify-between text-xs mb-1">
                      <span>Load</span>
                      <span className={getLoadColor(persona.load)}>{persona.load}%</span>
                    </div>
                    <CTASProgress value={persona.load} variant={persona.load >= 80 ? 'danger' : persona.load >= 60 ? 'warning' : 'success'} />
                  </div>
                </div>
              ))}
            </div>

            {/* Available Personas */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-3">
              <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Available MCPs</h3>
              <div className="space-y-2">
                {personas.map((persona) => (
                  <button
                    key={persona.id}
                    onClick={() => setActivePersona(persona.id)}
                    className={`w-full flex items-center space-x-2 p-2 rounded border transition-colors ${
                      activePersona === persona.id
                        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                        : 'border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'
                    }`}
                  >
                    <div className="relative">
                      <div className={`w-8 h-8 ${persona.color} rounded-full flex items-center justify-center`}>
                        <persona.icon className="w-4 h-4 text-white" />
                      </div>
                      <div className={`absolute -bottom-0.5 -right-0.5 w-2 h-2 ${getStatusColor(persona.status)} rounded-full border border-white dark:border-gray-800`}></div>
                    </div>
                    <div className="flex-1 text-left">
                      <p className="text-xs font-medium text-gray-900 dark:text-white">{persona.name}</p>
                      <p className="text-xs text-gray-500 dark:text-gray-400">{persona.role}</p>
                    </div>
                    <div className={`text-xs font-medium ${getLoadColor(persona.load)}`}>
                      {persona.load}%
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* Main Content Area */}
          <div className="lg:col-span-3 space-y-4">
            {/* System Overview Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
              <CTASStatCard
                title="Total Assets"
                value={systemMetrics.totalAssets}
                icon={Globe}
                status="info"
              />
              <CTASStatCard
                title="Active Assets"
                value={systemMetrics.activeAssets}
                icon={Activity}
                status="success"
                trend={{ value: 5, isPositive: true }}
              />
              <CTASStatCard
                title="Active Alerts"
                value={systemMetrics.alerts}
                icon={AlertTriangle}
                status="warning"
                trend={{ value: 2, isPositive: false }}
              />
              <CTASStatCard
                title="Threats Detected"
                value={systemMetrics.threats}
                icon={Target}
                status="danger"
                trend={{ value: 8, isPositive: false }}
              />
            </div>

            {/* Charts and Communication */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
              <CTASLineChart
                title="Threat Activity (7 Days)"
                data={threatActivityData}
                color="#6B7280"
                height={200}
              />
              <CTASChat
                title="MCP Communication"
                placeholder="Interact with active MCP persona..."
                className="h-64"
              />
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

            {/* Quick Actions */}
            <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 p-4">
              <h3 className="font-medium text-gray-900 dark:text-white mb-3 text-sm">Quick Actions</h3>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                <button className="flex flex-col items-center p-3 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                  <Bot className="w-6 h-6 text-gray-600 mb-1" />
                  <span className="text-xs font-medium text-gray-900 dark:text-white">Switch MCP</span>
                </button>
                <button className="flex flex-col items-center p-3 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                  <Mail className="w-6 h-6 text-gray-600 mb-1" />
                  <span className="text-xs font-medium text-gray-900 dark:text-white">Send Task</span>
                </button>
                <button className="flex flex-col items-center p-3 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                  <BarChart3 className="w-6 h-6 text-gray-600 mb-1" />
                  <span className="text-xs font-medium text-gray-900 dark:text-white">Analytics</span>
                </button>
                <button className="flex flex-col items-center p-3 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                  <Settings className="w-6 h-6 text-gray-600 mb-1" />
                  <span className="text-xs font-medium text-gray-900 dark:text-white">Configure</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SharedComponentsDemo;
