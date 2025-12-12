import React, { useState, useEffect } from 'react';
import { CheckCircle, AlertTriangle, Trash2, Plus, Database, RefreshCw } from 'lucide-react';

interface RedisKey {
  id: string;
  key: string;
  type: 'string' | 'hash' | 'list' | 'set' | 'zset';
  ttl: number;
  size: number;
  lastAccessed: string;
}

const RedisManager: React.FC = () => {
  const [status, setStatus] = useState<string>('disconnected');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Simulate Redis connection
    const connectToRedis = async () => {
      try {
        // Simulate connection delay
        await new Promise(resolve => setTimeout(resolve, 1000));
        setStatus('connected');
        setError(null);
      } catch (err) {
        setStatus('error');
        setError('Failed to connect to Redis');
      }
    };

    connectToRedis();
  }, []);

  const [keys, setKeys] = useState<RedisKey[]>([
    {
      id: '1',
      key: 'hunt:targets',
      type: 'hash',
      ttl: 3600,
      size: 1024,
      lastAccessed: '2023-06-15 10:30:00'
    },
    {
      id: '2',
      key: 'detect:alerts',
      type: 'list',
      ttl: 7200,
      size: 2048,
      lastAccessed: '2023-06-15 11:45:00'
    },
    {
      id: '3',
      key: 'disable:actions',
      type: 'set',
      ttl: 1800,
      size: 512,
      lastAccessed: '2023-06-15 09:15:00'
    }
  ]);

  const [newKey, setNewKey] = useState({
    key: '',
    type: 'string' as RedisKey['type'],
    ttl: 3600
  });

  const handleAddKey = () => {
    if (newKey.key) {
      setKeys([...keys, {
        id: crypto.randomUUID(),
        ...newKey,
        size: 0,
        lastAccessed: new Date().toISOString().slice(0, 19).replace('T', ' ')
      }]);
      setNewKey({ key: '', type: 'string', ttl: 3600 });
    }
  };

  const handleDeleteKey = (id: string) => {
    setKeys(keys.filter(key => key.id !== id));
  };

  const handleRefresh = () => {
    // Simulate Redis refresh
    setStatus('connected');
    setError(null);
  };

  return (
    <div className="space-y-4">
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <div className="flex justify-between items-center mb-4">
          <div className="flex items-center">
            <Database className="w-4 h-4 mr-2" />
            <h2 className="text-sm font-semibold">Redis Cache Manager</h2>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={handleRefresh}
              className="p-1 rounded bg-blue-500 text-white"
            >
              <RefreshCw size={12} />
            </button>
            {status === 'connected' ? (
              <CheckCircle className="w-4 h-4 text-green-500" />
            ) : (
              <AlertTriangle className="w-4 h-4 text-red-500" />
            )}
          </div>
        </div>

        {error && (
          <div className="bg-red-100 dark:bg-red-900/50 text-red-800 dark:text-red-200 p-2 rounded mb-4 text-xs">
            {error}
          </div>
        )}

        <div className="space-y-2">
          {keys.map(key => (
            <div
              key={key.id}
              className="flex items-center justify-between bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs"
            >
              <div>
                <div className="flex items-center">
                  <span className="font-mono">{key.key}</span>
                  <span className="ml-2 px-1.5 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded">
                    {key.type}
                  </span>
                </div>
                <div className="text-gray-500 dark:text-gray-400 mt-1">
                  <span>TTL: {key.ttl}s</span>
                  <span className="mx-2">|</span>
                  <span>Size: {key.size} bytes</span>
                  <span className="mx-2">|</span>
                  <span>Last: {key.lastAccessed}</span>
                </div>
              </div>
              <button
                onClick={() => handleDeleteKey(key.id)}
                className="p-1 rounded bg-red-500 text-white"
              >
                <Trash2 size={12} />
              </button>
            </div>
          ))}
        </div>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h3 className="text-sm font-semibold mb-2">Add New Key</h3>
        <div className="grid grid-cols-3 gap-2">
          <input
            type="text"
            placeholder="Key"
            value={newKey.key}
            onChange={(e) => setNewKey({ ...newKey, key: e.target.value })}
            className="p-1 border rounded text-xs"
          />
          <select
            value={newKey.type}
            onChange={(e) => setNewKey({ ...newKey, type: e.target.value as RedisKey['type'] })}
            className="p-1 border rounded text-xs"
          >
            <option value="string">String</option>
            <option value="hash">Hash</option>
            <option value="list">List</option>
            <option value="set">Set</option>
            <option value="zset">Sorted Set</option>
          </select>
          <input
            type="number"
            placeholder="TTL (seconds)"
            value={newKey.ttl}
            onChange={(e) => setNewKey({ ...newKey, ttl: parseInt(e.target.value) })}
            className="p-1 border rounded text-xs"
          />
        </div>
        <button
          onClick={handleAddKey}
          className="mt-2 bg-blue-500 text-white px-2 py-1 rounded text-xs flex items-center"
        >
          <Plus size={12} className="mr-1" />
          Add Key
        </button>
      </div>
    </div>
  );
};

export default RedisManager;