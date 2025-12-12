import React, { useState, useRef, useEffect } from 'react';
import { Activity } from 'lucide-react';


interface NetworkDeployment {
  id: string;
  cidr: string;
  type: 'honeytrap' | 'vraven' | 'redteam';
  status: 'active' | 'inactive';
  ttl: number;
  lastActivity: string;
  description: string;
}

interface NetworkCLIProps {
  selectedNetwork: NetworkDeployment | null;
}

const NetworkCLI: React.FC<NetworkCLIProps> = ({ selectedNetwork }) => {
  const [history, setHistory] = useState<string[]>([]);
  const [currentCommand, setCurrentCommand] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const historyRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (historyRef.current) {
      historyRef.current.scrollTop = historyRef.current.scrollHeight;
    }
  }, [history]);

  const handleCommand = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && currentCommand.trim()) {
      const command = currentCommand.trim();
      let response = '';

      if (selectedNetwork) {
        switch (command.toLowerCase()) {
          case 'help':
            response = `Available commands:
status - Show network status
ttl - Show/set TTL
scan - Scan network
deploy - Deploy new instance
terminate - Terminate network
reset - Reset network configuration
help - Show this help message`;
            break;
          case 'status':
            response = `Network: ${selectedNetwork.cidr}
Type: ${selectedNetwork.type}
Status: ${selectedNetwork.status}
TTL: ${selectedNetwork.ttl}s
Last Activity: ${new Date(selectedNetwork.lastActivity).toLocaleString()}`;
            break;
          case 'scan':
            response = `Scanning network ${selectedNetwork.cidr}...
Found 5 active hosts
2 potential threats detected
3 services identified`;
            break;
          default:
            if (command.startsWith('deploy')) {
              response = 'Deploying new instance...';
            } else if (command.startsWith('terminate')) {
              response = 'Terminating network...';
            } else {
              response = `Unknown command: ${command}`;
            }
        }
      } else {
        response = 'No network selected';
      }

      setHistory([...history, `> ${command}`, response]);
      setCurrentCommand('');
    }
  };

  return (
    <div className="bg-gray-900 text-green-400 p-2 rounded-lg font-mono text-xs">
      <div 
        ref={historyRef}
        className="h-48 overflow-y-auto mb-2 whitespace-pre-wrap"
      >
        {history.map((line, i) => (
          <div key={i} className={line.startsWith('>') ? 'text-blue-400' : ''}>
            {line}
          </div>
        ))}
      </div>
      <div className="flex items-center">
        <span className="mr-2">{'>'}</span>
        <input
          ref={inputRef}
          type="text"
          value={currentCommand}
          onChange={(e) => setCurrentCommand(e.target.value)}
          onKeyDown={handleCommand}
          className="flex-1 bg-transparent outline-none"
          placeholder={selectedNetwork ? 'Enter command (type help for commands)' : 'Select a network...'}
          disabled={!selectedNetwork}
        />
      </div>
    </div>
  );
};

export default NetworkCLI;