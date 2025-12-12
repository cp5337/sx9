import React, { useState } from 'react';
import { Radio, ChevronDown, ChevronRight, AlertTriangle, Shield, Activity } from 'lucide-react';

const OperationsSummary: React.FC = () => {
  const [expandedCards, setExpandedCards] = useState<string[]>([]);

  const operations = [
    {
      id: 'op1',
      name: 'Operation Blackout',
      target: 'Texas Power Grid',
      phase: 'Hunt',
      status: 'active',
      vRavens: 3,
      alerts: 2,
      lastActivity: '2 mins ago'
    },
    {
      id: 'op2',
      name: 'Operation Deadbolt',
      target: 'NYPD Systems',
      phase: 'Detect',
      status: 'active',
      vRavens: 2,
      alerts: 0,
      lastActivity: '5 mins ago'
    },
    {
      id: 'op3',
      name: 'Operation Firewall',
      target: 'St. Mary\'s Medical',
      phase: 'Disable',
      status: 'active',
      vRavens: 4,
      alerts: 1,
      lastActivity: '15 mins ago'
    }
  ];

  const stats = {
    activeOperations: operations.length,
    activeVRavens: operations.reduce((acc, op) => acc + op.vRavens, 0),
    totalAlerts: operations.reduce((acc, op) => acc + op.alerts, 0),
    systemsScanned: 1247
  };

  const toggleCard = (cardId: string) => {
    setExpandedCards(prev => 
      prev.includes(cardId) 
        ? prev.filter(id => id !== cardId)
        : [...prev, cardId]
    );
  };

  const isExpanded = (cardId: string) => expandedCards.includes(cardId);

  return (
    <div className="grid grid-cols-1 md:grid-cols-4 gap-2">
      {/* Operations Card */}
      <div 
        className={`bg-gradient-to-br from-blue-500 to-blue-600 rounded-lg shadow-lg cursor-pointer transition-all duration-200 ${
          isExpanded('operations') ? 'p-4' : 'p-2'
        }`}
        onClick={() => toggleCard('operations')}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Activity className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active Operations</h3>
          </div>
          <div className="flex items-center">
            <span className="text-lg font-bold text-white mr-1">{stats.activeOperations}</span>
            {isExpanded('operations') ? (
              <ChevronDown className="w-4 h-4 text-white" />
            ) : (
              <ChevronRight className="w-4 h-4 text-white" />
            )}
          </div>
        </div>
        {isExpanded('operations') && (
          <div className="mt-2">
            {operations.map(op => (
              <div key={op.id} className="text-xs text-white/80 mt-1">
                {op.name} - {op.phase}
              </div>
            ))}
          </div>
        )}
      </div>

      {/* vRavens Card */}
      <div 
        className={`bg-gradient-to-br from-green-500 to-green-600 rounded-lg shadow-lg cursor-pointer transition-all duration-200 ${
          isExpanded('vravens') ? 'p-4' : 'p-2'
        }`}
        onClick={() => toggleCard('vravens')}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Radio className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active vRavens</h3>
          </div>
          <div className="flex items-center">
            <span className="text-lg font-bold text-white mr-1">{stats.activeVRavens}</span>
            {isExpanded('vravens') ? (
              <ChevronDown className="w-4 h-4 text-white" />
            ) : (
              <ChevronRight className="w-4 h-4 text-white" />
            )}
          </div>
        </div>
        {isExpanded('vravens') && (
          <div className="mt-2 text-xs text-white/80">
            Deployed across {operations.length} operations
            {operations.map(op => (
              <div key={op.id} className="mt-1">
                {op.name}: {op.vRavens} vRavens
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Alerts Card */}
      <div 
        className={`bg-gradient-to-br from-yellow-500 to-yellow-600 rounded-lg shadow-lg cursor-pointer transition-all duration-200 ${
          isExpanded('alerts') ? 'p-4' : 'p-2'
        }`}
        onClick={() => toggleCard('alerts')}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <AlertTriangle className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Active Alerts</h3>
          </div>
          <div className="flex items-center">
            <span className="text-lg font-bold text-white mr-1">{stats.totalAlerts}</span>
            {isExpanded('alerts') ? (
              <ChevronDown className="w-4 h-4 text-white" />
            ) : (
              <ChevronRight className="w-4 h-4 text-white" />
            )}
          </div>
        </div>
        {isExpanded('alerts') && (
          <div className="mt-2 text-xs text-white/80">
            {operations.map(op => op.alerts > 0 && (
              <div key={op.id}>{op.name}: {op.alerts} alerts</div>
            ))}
          </div>
        )}
      </div>

      {/* Systems Card */}
      <div 
        className={`bg-gradient-to-br from-purple-500 to-purple-600 rounded-lg shadow-lg cursor-pointer transition-all duration-200 ${
          isExpanded('systems') ? 'p-4' : 'p-2'
        }`}
        onClick={() => toggleCard('systems')}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Shield className="w-4 h-4 mr-1 text-white" />
            <h3 className="text-white text-xs font-semibold">Systems Scanned</h3>
          </div>
          <div className="flex items-center">
            <span className="text-lg font-bold text-white mr-1">{stats.systemsScanned}</span>
            {isExpanded('systems') ? (
              <ChevronDown className="w-4 h-4 text-white" />
            ) : (
              <ChevronRight className="w-4 h-4 text-white" />
            )}
          </div>
        </div>
        {isExpanded('systems') && (
          <div className="mt-2 text-xs text-white/80">
            <div>Last scan completed 5 mins ago</div>
            <div className="mt-1">Scan coverage: 92%</div>
            <div className="mt-1">Critical systems: 156</div>
            <div className="mt-1">Vulnerabilities found: 23</div>
          </div>
        )}
      </div>
    </div>
  );
};

export default OperationsSummary;