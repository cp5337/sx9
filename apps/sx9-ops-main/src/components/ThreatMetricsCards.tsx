import React from 'react';
import { AlertTriangle, Shield, Target, Activity, TrendingUp, TrendingDown, MapPin } from 'lucide-react';

interface ThreatMetric {
  id: string;
  title: string;
  value: number;
  change: number; // percentage change
  severity: 'critical' | 'high' | 'medium' | 'low';
  location?: { lat: number; lng: number };
  lastUpdate: string;
}

interface ThreatMetricsCardsProps {
  onCardClick?: (metric: ThreatMetric) => void;
}

const ThreatMetricsCards: React.FC<ThreatMetricsCardsProps> = ({ onCardClick }) => {
  // Mock data - will be replaced with real data from Supabase/TAPS
  const metrics: ThreatMetric[] = [
    {
      id: 'critical-alerts',
      title: 'Critical Alerts',
      value: 3,
      change: 50,
      severity: 'critical',
      location: { lat: 40.7128, lng: -74.006 },
      lastUpdate: '2m ago'
    },
    {
      id: 'active-threats',
      title: 'Active Threats',
      value: 12,
      change: -15,
      severity: 'high',
      location: { lat: 34.0522, lng: -118.2437 },
      lastUpdate: '5m ago'
    },
    {
      id: 'deployed-assets',
      title: 'Deployed Assets',
      value: 47,
      change: 8,
      severity: 'medium',
      lastUpdate: '1m ago'
    },
    {
      id: 'intel-streams',
      title: 'Intel Streams',
      value: 234,
      change: 12,
      severity: 'low',
      lastUpdate: 'live'
    }
  ];

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'bg-red-900/50 border-red-500 hover:bg-red-900/70';
      case 'high': return 'bg-orange-900/50 border-orange-500 hover:bg-orange-900/70';
      case 'medium': return 'bg-yellow-900/50 border-yellow-500 hover:bg-yellow-900/70';
      case 'low': return 'bg-blue-900/50 border-blue-500 hover:bg-blue-900/70';
      default: return 'bg-gray-800 border-gray-600 hover:bg-gray-700';
    }
  };

  const getIcon = (id: string) => {
    switch (id) {
      case 'critical-alerts': return <AlertTriangle size={18} />;
      case 'active-threats': return <Target size={18} />;
      case 'deployed-assets': return <Shield size={18} />;
      case 'intel-streams': return <Activity size={18} />;
      default: return <Activity size={18} />;
    }
  };

  return (
    <div className="flex gap-2 px-3 py-1">
      {metrics.map((metric) => (
        <button
          key={metric.id}
          onClick={() => onCardClick?.(metric)}
          className={`flex-1 flex items-center gap-2 px-3 py-1.5 rounded border transition-all duration-200 ${getSeverityColor(metric.severity)} group cursor-pointer`}
          title={`Click to ${metric.location ? 'view on map' : 'view details'}`}
        >
          {/* Icon */}
          <div className="flex-shrink-0 text-white">
            {getIcon(metric.id)}
          </div>

          {/* Content */}
          <div className="flex-1 min-w-0">
            <div className="flex items-baseline gap-2">
              <span className="text-2xl font-bold text-white">{metric.value}</span>
              <span className="text-xs text-gray-300 truncate">{metric.title}</span>
            </div>
          </div>

          {/* Change Indicator & Location */}
          <div className="flex-shrink-0 flex flex-col items-end gap-0.5">
            {metric.change !== 0 && (
              <div className={`flex items-center gap-0.5 text-xs ${metric.change > 0 ? 'text-red-400' : 'text-green-400'}`}>
                {metric.change > 0 ? <TrendingUp size={12} /> : <TrendingDown size={12} />}
                <span>{Math.abs(metric.change)}%</span>
              </div>
            )}
            {metric.location && (
              <MapPin size={10} className="text-blue-400 opacity-70 group-hover:opacity-100 transition-opacity" />
            )}
            <span className="text-xxs text-gray-500">{metric.lastUpdate}</span>
          </div>
        </button>
      ))}
    </div>
  );
};

export default ThreatMetricsCards;

