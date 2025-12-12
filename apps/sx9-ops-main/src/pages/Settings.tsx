import React from 'react';
import { Settings as SettingsIcon, Bell, Lock, Palette, HardDrive, Zap, Database } from 'lucide-react';

const Settings: React.FC = () => {
  return (
    <div className="p-4 bg-gray-100 dark:bg-gray-900 min-h-screen">
      <div className="max-w-7xl mx-auto space-y-6">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg">
          <h2 className="text-xl font-semibold mb-6 flex items-center">
            <SettingsIcon className="mr-2" />
            System Settings
          </h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <SettingsCard icon={Bell} title="Notifications" />
            <SettingsCard icon={Lock} title="Security" />
            <SettingsCard icon={Palette} title="Appearance" />
            <SettingsCard icon={Database} title="Data Management" />
            <SettingsCard icon={HardDrive} title="Storage" />
            <SettingsCard icon={Zap} title="Performance" />
          </div>
        </div>
      </div>
    </div>
  );
};

const SettingsCard: React.FC<{ icon: React.ElementType; title: string }> = ({ icon: Icon, title }) => (
  <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg shadow">
    <div className="flex items-center space-x-2">
      <Icon className="text-blue-500" size={20} />
      <h3 className="font-semibold">{title}</h3>
    </div>
  </div>
);

export default Settings;