import React, { useState, useEffect } from 'react';
import { Database, CheckCircle2, XCircle, Loader2, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';

export interface DatabaseConnection {
  id: string;
  name: string;
  type: 'supabase' | 'surrealdb' | 'sled' | 'slotgraph';
  port?: number;
  status: 'connected' | 'disconnected' | 'connecting' | 'error';
  lastChecked?: Date;
}

export interface DatabaseConnectionPanelProps {
  databases?: DatabaseConnection[];
  onConnect?: (dbId: string) => void;
  onDisconnect?: (dbId: string) => void;
  onRefresh?: (dbId: string) => void;
  showStatus?: boolean;
  className?: string;
  compact?: boolean;
}

const defaultDatabases: DatabaseConnection[] = [
  {
    id: 'supabase',
    name: 'Supabase',
    type: 'supabase',
    port: 3000,
    status: 'disconnected'
  },
  {
    id: 'surrealdb',
    name: 'SurrealDB',
    type: 'surrealdb',
    port: 8000,
    status: 'disconnected'
  },
  {
    id: 'sled',
    name: 'Sled KVR',
    type: 'sled',
    status: 'disconnected'
  },
  {
    id: 'slotgraph',
    name: 'SlotGraph',
    type: 'slotgraph',
    status: 'disconnected'
  }
];

const statusConfig = {
  connected: {
    icon: CheckCircle2,
    color: 'text-green-500',
    bgColor: 'bg-green-500/10',
    label: 'Connected',
    badgeVariant: 'default' as const
  },
  disconnected: {
    icon: XCircle,
    color: 'text-gray-500',
    bgColor: 'bg-gray-500/10',
    label: 'Disconnected',
    badgeVariant: 'secondary' as const
  },
  connecting: {
    icon: Loader2,
    color: 'text-yellow-500',
    bgColor: 'bg-yellow-500/10',
    label: 'Connecting...',
    badgeVariant: 'secondary' as const
  },
  error: {
    icon: XCircle,
    color: 'text-red-500',
    bgColor: 'bg-red-500/10',
    label: 'Error',
    badgeVariant: 'destructive' as const
  }
};

export const DatabaseConnectionPanel: React.FC<DatabaseConnectionPanelProps> = ({
  databases = defaultDatabases,
  onConnect,
  onDisconnect,
  onRefresh,
  showStatus = true,
  className = '',
  compact = false
}) => {
  const [connectionStates, setConnectionStates] = useState<Map<string, DatabaseConnection['status']>>(
    new Map(databases.map(db => [db.id, db.status]))
  );

  const checkConnection = async (dbId: string) => {
    const db = databases.find(d => d.id === dbId);
    if (!db) return;

    setConnectionStates(prev => new Map(prev).set(dbId, 'connecting'));

    try {
      // Simulate connection check - replace with actual health check
      const response = await fetch(`http://localhost:${db.port || 8000}/health`, {
        method: 'GET',
        signal: AbortSignal.timeout(3000)
      });

      if (response.ok) {
        setConnectionStates(prev => new Map(prev).set(dbId, 'connected'));
      } else {
        setConnectionStates(prev => new Map(prev).set(dbId, 'error'));
      }
    } catch (error) {
      setConnectionStates(prev => new Map(prev).set(dbId, 'disconnected'));
    }
  };

  const handleConnect = (dbId: string) => {
    if (onConnect) {
      onConnect(dbId);
    } else {
      checkConnection(dbId);
    }
  };

  const handleDisconnect = (dbId: string) => {
    setConnectionStates(prev => new Map(prev).set(dbId, 'disconnected'));
    if (onDisconnect) {
      onDisconnect(dbId);
    }
  };

  const handleRefresh = (dbId: string) => {
    if (onRefresh) {
      onRefresh(dbId);
    } else {
      checkConnection(dbId);
    }
  };

  if (compact) {
    return (
      <div className={`flex gap-2 ${className}`}>
        {databases.map((db) => {
          const status = connectionStates.get(db.id) || db.status;
          const config = statusConfig[status];
          const Icon = config.icon;
          const isConnected = status === 'connected';

          return (
            <Button
              key={db.id}
              variant={isConnected ? 'default' : 'outline'}
              size="sm"
              onClick={() => isConnected ? handleDisconnect(db.id) : handleConnect(db.id)}
              title={`${db.name} - ${config.label}`}
              className="h-8"
            >
              <Icon className={`h-3.5 w-3.5 ${config.color}`} />
            </Button>
          );
        })}
      </div>
    );
  }

  return (
    <div className={`bg-gray-800 rounded-lg border border-gray-700 p-4 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Database className="h-5 w-5 text-gray-400" />
          <h3 className="text-sm font-semibold text-gray-300">Database Connections</h3>
        </div>
      </div>

      <div className="space-y-2">
        {databases.map((db) => {
          const status = connectionStates.get(db.id) || db.status;
          const config = statusConfig[status];
          const Icon = config.icon;
          const isConnected = status === 'connected';
          const isLoading = status === 'connecting';

          return (
            <div
              key={db.id}
              className={`flex items-center justify-between p-3 rounded-lg border ${
                isConnected ? 'border-green-500/50 bg-green-500/5' : 'border-gray-700 bg-gray-900/50'
              }`}
            >
              <div className="flex items-center gap-3 flex-1">
                <div className={`p-2 rounded ${config.bgColor}`}>
                  <Icon className={`h-4 w-4 ${config.color} ${isLoading ? 'animate-spin' : ''}`} />
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-medium text-gray-300">{db.name}</span>
                    {showStatus && (
                      <Badge variant={config.badgeVariant} className="text-xs">
                        {config.label}
                      </Badge>
                    )}
                  </div>
                  {db.port && (
                    <span className="text-xs text-gray-500">Port {db.port}</span>
                  )}
                </div>
              </div>

              <div className="flex items-center gap-2">
                {isConnected ? (
                  <>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleRefresh(db.id)}
                      title="Refresh connection"
                    >
                      <RefreshCw className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleDisconnect(db.id)}
                    >
                      Disconnect
                    </Button>
                  </>
                ) : (
                  <Button
                    variant="default"
                    size="sm"
                    onClick={() => handleConnect(db.id)}
                    disabled={isLoading}
                  >
                    {isLoading ? (
                      <>
                        <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        Connecting...
                      </>
                    ) : (
                      'Connect'
                    )}
                  </Button>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default DatabaseConnectionPanel;



