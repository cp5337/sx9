import React from 'react';
import { Target, Shield, Globe, Activity } from 'lucide-react';

const OperationsOverview: React.FC = () => {
  const stats = {
    activeRegions: ['North America', 'Europe'],
    activeSectors: ['Energy', 'Defense'],
    threatActors: ['APT28', 'Lazarus Group'],
    activeTargets: ['Texas Power Grid', 'NYPD Systems']
  };

  return (
    <div className="grid grid-cols-2 gap-2">
      <div className="bg-gradient-to-br from-indigo-500 to-indigo-600 p-2 rounded-lg shadow-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Globe className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active Regions</h3>
          </div>
          <span className="text-lg font-bold text-white">{stats.activeRegions.length}</span>
        </div>
        <div className="mt-1 text-xs text-white/80">
          {stats.activeRegions.join(', ')}
        </div>
      </div>

      <div className="bg-gradient-to-br from-cyan-500 to-cyan-600 p-2 rounded-lg shadow-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Shield className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active Sectors</h3>
          </div>
          <span className="text-lg font-bold text-white">{stats.activeSectors.length}</span>
        </div>
        <div className="mt-1 text-xs text-white/80">
          {stats.activeSectors.join(', ')}
        </div>
      </div>

      <div className="bg-gradient-to-br from-rose-500 to-rose-600 p-2 rounded-lg shadow-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Target className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Threat Actors</h3>
          </div>
          <span className="text-lg font-bold text-white">{stats.threatActors.length}</span>
        </div>
        <div className="mt-1 text-xs text-white/80">
          {stats.threatActors.join(', ')}
        </div>
      </div>

      <div className="bg-gradient-to-br from-amber-500 to-amber-600 p-2 rounded-lg shadow-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Activity className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active Targets</h3>
          </div>
          <span className="text-lg font-bold text-white">{stats.activeTargets.length}</span>
        </div>
        <div className="mt-1 text-xs text-white/80">
          {stats.activeTargets.join(', ')}
        </div>
      </div>
    </div>
  );
};

export default OperationsOverview;