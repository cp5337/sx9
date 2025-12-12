import React, { useState } from 'react';
import { Settings, Database, Globe, Package, Activity, Code, X, Download } from 'lucide-react';
import SystemOverview from './AdminDashboard/SystemOverview';
import ThreatFeeds from './AdminDashboard/ThreatFeeds';
import Localization from './AdminDashboard/Localization';
import APIStore from './AdminDashboard/APIStore';
import PackageStatus from './AdminDashboard/PackageStatus';
import CodeViewer from './AdminDashboard/CodeViewer';
import ProjectExport from './AdminDashboard/ProjectExport';

interface AdminDashboardProps {
  onClose: () => void;
}

const AdminDashboard: React.FC<AdminDashboardProps> = ({ onClose }) => {
  const [activeTab, setActiveTab] = useState('overview');

  const tabs = [
    { id: 'overview', label: 'System Overview', icon: Settings },
    { id: 'threatFeeds', label: 'Threat Feeds', icon: Database },
    { id: 'localization', label: 'Localization', icon: Globe },
    { id: 'apiStore', label: 'API Store', icon: Package },
    { id: 'packageStatus', label: 'Package Status', icon: Activity },
    { id: 'codeViewer', label: 'Code Viewer', icon: Code },
    { id: 'projectExport', label: 'Project Export', icon: Download },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case 'overview':
        return <SystemOverview />;
      case 'threatFeeds':
        return <ThreatFeeds />;
      case 'localization':
        return <Localization />;
      case 'apiStore':
        return <APIStore />;
      case 'packageStatus':
        return <PackageStatus />;
      case 'codeViewer':
        return <CodeViewer />;
      case 'projectExport':
        return <ProjectExport />;
      default:
        return null;
    }
  };

  return (
    <div className="flex h-screen bg-gray-100">
      <div className="w-64 bg-white shadow-md">
        <div className="p-4 flex justify-between items-center">
          <h2 className="text-xl font-semibold">Admin Dashboard</h2>
          <button onClick={onClose} className="text-gray-500 hover:text-gray-700">
            <X className="w-5 h-5" />
          </button>
        </div>
        <nav className="mt-4">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`flex items-center w-full px-4 py-2 text-left ${
                activeTab === tab.id ? 'bg-blue-500 text-white' : 'text-gray-600 hover:bg-gray-100'
              }`}
              onClick={() => setActiveTab(tab.id)}
            >
              <tab.icon className="w-5 h-5 mr-2" />
              {tab.label}
            </button>
          ))}
        </nav>
      </div>
      <div className="flex-1 p-8 overflow-auto">
        {renderTabContent()}
      </div>
    </div>
  );
};

export default AdminDashboard;