import React, { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Home, Search, Target, Settings, Clipboard, Radio, Code, Layers, Building,
  AlertTriangle, Shield, Zap, Globe, Cpu, Box, Database, Terminal, FileText,
  Hash, GitBranch, ToggleLeft, ToggleRight, Brain, Map, BarChart3, Eye, Palette,
  ChevronLeft, ChevronRight, ShoppingCart, Flame
} from 'lucide-react';
import { useSidebar } from '../contexts/SidebarContext';

const Sidebar: React.FC = () => {
  const location = useLocation();
  const { isCollapsed, setIsCollapsed } = useSidebar();
  const [showKillChain, setShowKillChain] = useState(false);

  // Top Section - Main Navigation
  const topSection = [
    { id: 'dashboard', name: 'Dashboard', path: '/', icon: <Home size={14} /> },
    { id: 'kill-chain', name: 'Kill Chain', path: '/kill-chain', icon: <ToggleLeft size={14} />, toggle: true },
  ];

  // HD4 Framework Section
  const hd4Section = [
    { id: 'hunt', name: 'Hunt', path: '/hunt', icon: <Search size={14} /> },
    { id: 'detect', name: 'Detect', path: '/detect', icon: <Shield size={14} /> },
    { id: 'disrupt', name: 'Disrupt', path: '/disrupt', icon: <Target size={14} /> },
    { id: 'disable', name: 'Disable', path: '/disable', icon: <Zap size={14} /> },
    { id: 'dominate', name: 'Dominate', path: '/dominate', icon: <Globe size={14} /> },
  ];

  // Kill Chain Phases (alternative to HD4)
  const killChainSection = [
    { id: 'reconnaissance', name: 'Reconnaissance', path: '/kill-chain/reconnaissance', icon: <Eye size={14} /> },
    { id: 'weaponization', name: 'Weaponization', path: '/kill-chain/weaponization', icon: <Target size={14} /> },
    { id: 'delivery', name: 'Delivery', path: '/kill-chain/delivery', icon: <Zap size={14} /> },
    { id: 'exploitation', name: 'Exploitation', path: '/kill-chain/exploitation', icon: <Shield size={14} /> },
    { id: 'installation', name: 'Installation', path: '/kill-chain/installation', icon: <Code size={14} /> },
    { id: 'command-control', name: 'Command & Control', path: '/kill-chain/command-control', icon: <Radio size={14} /> },
    { id: 'actions', name: 'Actions on Objectives', path: '/kill-chain/actions', icon: <Globe size={14} /> },
  ];

  // Operations Section
  const operationsSection = [
    { id: 'tasks', name: 'Tasks', path: '/tasks', icon: <Clipboard size={14} /> },
    { id: 'streams', name: 'Streams', path: '/info-streams', icon: <Radio size={14} /> },
    { id: 'plasma', name: 'Plasma', path: '/plasma', icon: <Shield size={14} /> },
  ];

  // Infrastructure Section
  const infrastructureSection = [
    { id: 'raptor', name: 'Raptor', path: '/raptor', icon: <Target size={14} /> },
    { id: 'vkali', name: 'vKali', path: '/vkali', icon: <Terminal size={14} /> },
    { id: 'containers', name: 'Containers', path: '/containers', icon: <Box size={14} /> },
    { id: 'exploit-db', name: 'Exploit DB', path: '/exploit-db', icon: <Database size={14} /> },
    { id: 'ai-cli', name: 'AI CLI', path: '/cli', icon: <Brain size={14} /> },
    { id: 'firefly', name: 'Firefly IAC', path: '/firefly', icon: <Flame size={14} /> },
  ];

  // System Section
  const systemSection = [
    { id: 'dvm', name: 'DVM', path: '/dvm', icon: <Cpu size={14} /> },
    { id: 'databases', name: 'Databases', path: '/database', icon: <Database size={14} /> },
    { id: 'scripts', name: 'Scripts', path: '/setup-scripts', icon: <Code size={14} /> },
    { id: 'demo-report', name: 'Demo Report', path: '/demo-report', icon: <BarChart3 size={14} /> },
    { id: 'gallery', name: 'Gallery', path: '/gallery', icon: <ShoppingCart size={14} /> },
    { id: 'shared-components', name: 'Components', path: '/shared-components', icon: <Palette size={14} /> },
    { id: 'settings', name: 'Settings', path: '/settings', icon: <Settings size={14} /> },
  ];

  const renderMenuItem = (item: any) => (
    <Link
      key={item.id}
      to={item.path}
      className={`flex items-center px-3 py-1.5 text-xs ${
        location.pathname === item.path ? 'bg-gray-700 text-white' : 'hover:bg-gray-700'
      }`}
      onClick={item.toggle ? () => setShowKillChain(!showKillChain) : undefined}
    >
      <span className="w-4">{item.icon}</span>
      {!isCollapsed && <span className="ml-2">{item.name}</span>}
    </Link>
  );

  const renderSeparator = () => (
    <div className="border-t border-gray-600 my-2 mx-3"></div>
  );

  return (
    <div className={`${isCollapsed ? 'w-12' : 'w-36'} h-full bg-gray-800 text-gray-300 fixed left-0 top-0 overflow-y-auto transition-all duration-300`}>
      {/* Collapse Toggle - Top Edge */}
      <div className="absolute top-2 left-2 z-50">
        <button
          onClick={() => setIsCollapsed(!isCollapsed)}
          className="text-gray-400 hover:text-white p-1 rounded hover:bg-gray-700 bg-gray-800 border border-gray-700"
          title={isCollapsed ? 'Expand Sidebar' : 'Collapse Sidebar'}
        >
          {isCollapsed ? <ChevronRight size={14} /> : <ChevronLeft size={14} />}
        </button>
      </div>

      <div className="p-4 pt-12">
        {!isCollapsed && (
          <div>
            <h2 className="text-base font-bold">CTAS</h2>
            <p className="text-xs text-gray-400">v7.3.1</p>
          </div>
        )}
      </div>
      
      <nav className="mt-2">
        {/* Top Section */}
        {topSection.map(renderMenuItem)}
        
        {renderSeparator()}
        
        {/* HD4 Framework or Kill Chain */}
        {showKillChain ? (
          killChainSection.map(renderMenuItem)
        ) : (
          hd4Section.map(renderMenuItem)
        )}
        
        {renderSeparator()}
        
        {/* Operations Section */}
        {operationsSection.map(renderMenuItem)}
        
        {renderSeparator()}
        
        {/* Infrastructure Section */}
        {infrastructureSection.map(renderMenuItem)}
        
        {renderSeparator()}
        
        {/* System Section */}
        {systemSection.map(renderMenuItem)}
      </nav>
    </div>
  );
};

export default Sidebar;