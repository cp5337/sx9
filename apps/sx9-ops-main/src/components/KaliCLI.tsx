import React, { useState, useRef, useEffect } from 'react';
import { Terminal, Play, Square, RotateCcw, Copy, Download } from 'lucide-react';

interface KaliCLIProps {
  sessionId?: string;
  onCommand?: (command: string) => void;
}

interface CommandHistory {
  id: string;
  command: string;
  output: string;
  timestamp: Date;
  status: 'success' | 'error' | 'running';
}

const KaliCLI: React.FC<KaliCLIProps> = ({ sessionId = 'default', onCommand }) => {
  const [commandHistory, setCommandHistory] = useState<CommandHistory[]>([]);
  const [currentCommand, setCurrentCommand] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const terminalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [commandHistory]);

  const executeCommand = async (command: string) => {
    if (!command.trim()) return;

    const commandId = Date.now().toString();
    const newCommand: CommandHistory = {
      id: commandId,
      command,
      output: '',
      timestamp: new Date(),
      status: 'running'
    };

    setCommandHistory(prev => [...prev, newCommand]);
    setIsRunning(true);
    setCurrentCommand('');

    // Simulate command execution
    setTimeout(() => {
      const output = simulateCommandOutput(command);
      setCommandHistory(prev => 
        prev.map(cmd => 
          cmd.id === commandId 
            ? { ...cmd, output, status: 'success' as const }
            : cmd
        )
      );
      setIsRunning(false);
    }, 1000 + Math.random() * 2000);

    onCommand?.(command);
  };

  const simulateCommandOutput = (command: string): string => {
    const cmd = command.toLowerCase();
    
    if (cmd.includes('nmap')) {
      return `Starting Nmap 7.94 ( https://nmap.org )
Nmap scan report for 192.168.1.1
Host is up (0.00047s latency).
Not shown: 998 closed ports
PORT    STATE SERVICE
22/tcp  open  ssh
80/tcp  open  http
443/tcp open  https

Nmap done: 1 IP address (1 host up) scanned in 2.34 seconds`;
    }
    
    if (cmd.includes('whoami')) {
      return 'root';
    }
    
    if (cmd.includes('pwd')) {
      return '/root';
    }
    
    if (cmd.includes('ls')) {
      return `Desktop  Documents  Downloads  Pictures  Videos
tools/  scripts/  reports/  logs/`;
    }
    
    if (cmd.includes('help')) {
      return `Available commands:
- nmap: Network scanning
- whoami: Current user
- pwd: Current directory
- ls: List files
- clear: Clear terminal
- help: Show this help`;
    }
    
    return `Command '${command}' not found. Type 'help' for available commands.`;
  };

  const clearTerminal = () => {
    setCommandHistory([]);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const downloadLogs = () => {
    const logContent = commandHistory
      .map(cmd => `[${cmd.timestamp.toISOString()}] $ ${cmd.command}\n${cmd.output}`)
      .join('\n\n');
    
    const blob = new Blob([logContent], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `kali-cli-${sessionId}-${Date.now()}.log`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !isRunning) {
      executeCommand(currentCommand);
    }
  };

  return (
    <div className="bg-black text-green-400 font-mono text-sm rounded-lg overflow-hidden">
      {/* Terminal Header */}
      <div className="bg-gray-800 px-4 py-2 flex items-center justify-between">
        <div className="flex items-center">
          <Terminal size={16} className="mr-2" />
          <span className="text-white">Kali CLI - Session {sessionId}</span>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={clearTerminal}
            className="p-1 hover:bg-gray-700 rounded"
            title="Clear Terminal"
          >
            <RotateCcw size={14} />
          </button>
          <button
            onClick={downloadLogs}
            className="p-1 hover:bg-gray-700 rounded"
            title="Download Logs"
          >
            <Download size={14} />
          </button>
        </div>
      </div>

      {/* Terminal Output */}
      <div 
        ref={terminalRef}
        className="p-4 h-96 overflow-y-auto"
      >
        {commandHistory.map((cmd) => (
          <div key={cmd.id} className="mb-4">
            <div className="flex items-center text-yellow-400">
              <span className="mr-2">root@kali:~$</span>
              <span>{cmd.command}</span>
              {cmd.status === 'running' && (
                <div className="ml-2 animate-pulse">...</div>
              )}
            </div>
            {cmd.output && (
              <div className="mt-2 text-green-400 whitespace-pre-wrap">
                {cmd.output}
              </div>
            )}
          </div>
        ))}
        
        {/* Current Command Line */}
        <div className="flex items-center">
          <span className="mr-2 text-yellow-400">root@kali:~$</span>
          <input
            type="text"
            value={currentCommand}
            onChange={(e) => setCurrentCommand(e.target.value)}
            onKeyPress={handleKeyPress}
            disabled={isRunning}
            className="flex-1 bg-transparent text-green-400 outline-none"
            placeholder={isRunning ? "Command running..." : "Enter command..."}
          />
          {isRunning && <div className="ml-2 animate-pulse">...</div>}
        </div>
      </div>

      {/* Quick Actions */}
      <div className="bg-gray-800 px-4 py-2 flex items-center space-x-2">
        <button
          onClick={() => executeCommand('nmap -sV 192.168.1.1')}
          disabled={isRunning}
          className="px-2 py-1 bg-blue-600 text-white text-xs rounded hover:bg-blue-700 disabled:opacity-50"
        >
          <Play size={12} className="mr-1" />
          Nmap Scan
        </button>
        <button
          onClick={() => executeCommand('whoami')}
          disabled={isRunning}
          className="px-2 py-1 bg-green-600 text-white text-xs rounded hover:bg-green-700 disabled:opacity-50"
        >
          Who Am I
        </button>
        <button
          onClick={() => executeCommand('ls -la')}
          disabled={isRunning}
          className="px-2 py-1 bg-purple-600 text-white text-xs rounded hover:bg-purple-700 disabled:opacity-50"
        >
          List Files
        </button>
      </div>
    </div>
  );
};

export default KaliCLI;
