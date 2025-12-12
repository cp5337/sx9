import React, { useState, useEffect } from 'react';
import { Terminal, Square, Play, Settings, Database, Network, Shield, Zap, Globe, Activity, AlertTriangle, CheckCircle, Clock, RefreshCw } from 'lucide-react';


interface WSLDistro {
  name: string;
  status: 'Running' | 'Stopped';
  version: string;
  defaultUser: string;
  lastStarted?: string;
  tools?: string[];
}

const WSLEnvironment: React.FC = () => {
  const [distros, setDistros] = useState<WSLDistro[]>([
    {
      name: 'kali-linux',
      status: 'Running',
      version: '2023.3',
      defaultUser: 'kali',
      lastStarted: new Date().toISOString(),
      tools: ['metasploit', 'nmap', 'wireshark', 'burpsuite', 'hydra']
    }
  ]);

  const [selectedDistro, setSelectedDistro] = useState<string>('kali-linux');
  const [output, setOutput] = useState<string[]>([
    '┌──(kali㉿ctas)-[~]',
    '└─$ whoami',
    'kali',
    '┌──(kali㉿ctas)-[~]',
    '└─$ uname -a',
    'Linux kali 5.15.90.1-microsoft-standard-WSL2 #1 SMP Fri Jan 27 02:56:13 UTC 2023 x86_64 GNU/Linux'
  ]);
  const [command, setCommand] = useState('');

  const handleCommand = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && command.trim()) {
      let response = '';
      switch (command.toLowerCase()) {
        case 'ls':
          response = 'Desktop  Documents  Downloads  Music  Pictures  Public  Templates  Videos';
          break;
        case 'pwd':
          response = '/home/kali';
          break;
        case 'whoami':
          response = 'kali';
          break;
        case 'ifconfig':
          response = 'eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500\n        inet 172.17.0.2  netmask 255.255.0.0  broadcast 172.17.255.255';
          break;
        default:
          response = `Command '${command}' executed`;
      }
      setOutput(prev => [...prev, 
        `┌──(kali㉿ctas)-[~]`,
        `└─$ ${command}`,
        response
      ]);
      setCommand('');
    }
  };

  const toggleDistro = (name: string) => {
    setDistros(prev => prev.map(distro => 
      distro.name === name 
        ? { 
            ...distro, 
            status: distro.status === 'Running' ? 'Stopped' : 'Running',
            lastStarted: distro.status === 'Stopped' ? new Date().toISOString() : distro.lastStarted || undefined
          } as WSLDistro
        : distro
    ));
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <div className="space-y-4">
        <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
          <h2 className="text-sm font-semibold mb-4 flex items-center">
            <Shield size={14} className="mr-2" />
            Kali Linux WSL
          </h2>
          <div className="space-y-2">
            {distros.map(distro => (
              <div 
                key={distro.name}
                className={`p-2 rounded-lg cursor-pointer ${
                  selectedDistro === distro.name 
                    ? 'bg-blue-100 dark:bg-blue-900' 
                    : 'bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600'
                }`}
                onClick={() => setSelectedDistro(distro.name)}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    <Terminal size={12} className="mr-2" />
                    <span className="font-medium">{distro.name}</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <span className={`px-2 py-0.5 rounded-full text-xs ${
                      distro.status === 'Running' ? 'bg-green-500' : 'bg-red-500'
                    } text-white`}>
                      {distro.status}
                    </span>
                    <button 
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleDistro(distro.name);
                      }}
                      className="p-1 rounded bg-gray-200 dark:bg-gray-600 hover:bg-gray-300 dark:hover:bg-gray-500"
                    >
                      {distro.status === 'Running' ? (
                        <Square size={12} />
                      ) : (
                        <Play size={12} />
                      )}
                    </button>
                  </div>
                </div>
                <div className="mt-1 text-xs text-gray-600 dark:text-gray-400">
                  <p>Version: {distro.version}</p>
                  <p>User: {distro.defaultUser}</p>
                  {distro.lastStarted && (
                    <p>Last Started: {new Date(distro.lastStarted).toLocaleString()}</p>
                  )}
                  {distro.tools && (
                    <p>Tools: {distro.tools.join(', ')}</p>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-sm font-semibold flex items-center">
              <AlertTriangle size={14} className="mr-2" />
              WSL Status
            </h2>
            <button className="p-1 rounded bg-blue-500 hover:bg-blue-600">
              <RefreshCw size={12} className="text-white" />
            </button>
          </div>
          <div className="space-y-2 text-xs">
            <div className="flex justify-between">
              <span>WSL Version:</span>
              <span>2.0</span>
            </div>
            <div className="flex justify-between">
              <span>Kernel Version:</span>
              <span>5.15.90.1-microsoft-standard-WSL2</span>
            </div>
            <div className="flex justify-between">
              <span>Memory Usage:</span>
              <span>2.1 GB / 4.0 GB</span>
            </div>
            <div className="flex justify-between">
              <span>Active Distros:</span>
              <span>{distros.filter(d => d.status === 'Running').length} / {distros.length}</span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <h2 className="text-sm font-semibold mb-4 flex items-center">
          <Terminal size={14} className="mr-2" />
          Kali Terminal
        </h2>
        <div className="bg-gray-900 p-2 rounded-lg font-mono">
          <div className="h-48 overflow-y-auto mb-2 text-xs">
            {output.map((line, i) => (
              <div 
                key={i} 
                className={line.includes('kali㉿ctas') ? 'text-red-400' : 'text-green-400'}
              >
                {line}
              </div>
            ))}
          </div>
          <div className="flex items-center text-red-400">
            <span className="mr-2">┌──(kali㉿ctas)-[~]</span>
          </div>
          <div className="flex items-center text-red-400">
            <span className="mr-2">└─$</span>
            <input
              type="text"
              value={command}
              onChange={(e) => setCommand(e.target.value)}
              onKeyDown={handleCommand}
              placeholder="Enter command..."
              className="flex-1 bg-transparent text-green-400 outline-none text-xs"
              disabled={!selectedDistro}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default WSLEnvironment;