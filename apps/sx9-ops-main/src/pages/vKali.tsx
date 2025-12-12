import React, { useState, useEffect, useRef } from 'react';
import { Terminal, Shield, Zap, Target, Settings, Play, Square, AlertTriangle, CheckCircle, Clock, Database, Network, Lock, Eye, Send, Bot, Clipboard, ChevronDown, ChevronUp, Sparkles } from 'lucide-react';
import { SYSTEM_PORTS } from '../hooks';

// Kali AI Agent for natural language command interpretation
interface KaliAIResponse {
  command: string;
  explanation: string;
  tool: string;
  parameters: string[];
  confidence: number;
}

interface KaliTool {
  id: string;
  name: string;
  category: string;
  description: string;
  status: 'available' | 'running' | 'error' | 'disabled';
  icon: React.ReactNode;
  command: string;
  parameters: string[];
  lastUsed: string;
  successRate: number;
}

interface KaliSession {
  id: string;
  name: string;
  target: string;
  status: 'active' | 'completed' | 'failed' | 'paused';
  startTime: string;
  endTime?: string;
  tools: string[];
  results: any[];
}

// Terminal history entry
interface TerminalEntry {
  id: string;
  type: 'input' | 'output' | 'error' | 'ai' | 'system';
  content: string;
  timestamp: Date;
  tool?: string;
}

const VKali: React.FC = () => {
  const [tools, setTools] = useState<KaliTool[]>([]);
  const [sessions, setSessions] = useState<KaliSession[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [activeSession, setActiveSession] = useState<KaliSession | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // Terminal state
  const [terminalInput, setTerminalInput] = useState('');
  const [terminalHistory, setTerminalHistory] = useState<TerminalEntry[]>([]);
  const [showTerminal, setShowTerminal] = useState(true);
  const [aiMode, setAiMode] = useState(true); // Natural language mode
  const [atomicClipboard, setAtomicClipboard] = useState<string[]>([]);
  const [clipboardConnected, setClipboardConnected] = useState(false);
  const [showClipboard, setShowClipboard] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const terminalRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    loadKaliTools();
    loadSessions();
    connectAtomicClipboard();
    // Welcome message
    addTerminalEntry('system', '🐉 Kali Linux AI Terminal v2.0 - Type naturally or use commands');
    addTerminalEntry('system', '💡 AI Mode: ON - Ask questions like "scan the network for open ports"');
  }, []);

  // Scroll terminal to bottom
  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [terminalHistory]);

  // Atomic Clipboard connection
  const connectAtomicClipboard = async () => {
    try {
      const response = await fetch(`http://localhost:${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD}/health`);
      if (response.ok) {
        setClipboardConnected(true);
        loadAtomicClipboard();
      }
    } catch {
      setClipboardConnected(false);
    }
  };

  const loadAtomicClipboard = async () => {
    try {
      const response = await fetch(`http://localhost:${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD}/clipboard`);
      if (response.ok) {
        const data = await response.json();
        setAtomicClipboard(data.items || []);
      }
    } catch (error) {
      console.error('Failed to load atomic clipboard:', error);
    }
  };

  const addToAtomicClipboard = async (text: string) => {
    try {
      await fetch(`http://localhost:${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD}/clipboard`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: text }),
      });
      loadAtomicClipboard();
    } catch (error) {
      console.error('Failed to add to clipboard:', error);
    }
  };

  // Terminal functions
  const addTerminalEntry = (type: TerminalEntry['type'], content: string, tool?: string) => {
    setTerminalHistory(prev => [...prev, {
      id: `entry-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      type,
      content,
      timestamp: new Date(),
      tool,
    }]);
  };

  // AI interpretation of natural language to Kali commands
  const interpretNaturalLanguage = async (input: string): Promise<KaliAIResponse> => {
    // Tool mapping for natural language interpretation
    const toolMappings: Record<string, { tool: string; command: string; params: string[] }> = {
      'scan': { tool: 'nmap', command: 'nmap', params: ['-sS', '-sV'] },
      'port': { tool: 'nmap', command: 'nmap', params: ['-p-'] },
      'network': { tool: 'nmap', command: 'nmap', params: ['-sn'] },
      'exploit': { tool: 'metasploit', command: 'msfconsole', params: ['-q'] },
      'password': { tool: 'hydra', command: 'hydra', params: ['-l', 'admin'] },
      'brute': { tool: 'hydra', command: 'hydra', params: ['-t', '4'] },
      'sql': { tool: 'sqlmap', command: 'sqlmap', params: ['--batch'] },
      'injection': { tool: 'sqlmap', command: 'sqlmap', params: ['--dbs'] },
      'wireless': { tool: 'aircrack', command: 'aircrack-ng', params: [] },
      'wifi': { tool: 'aircrack', command: 'airodump-ng', params: [] },
      'sniff': { tool: 'wireshark', command: 'tshark', params: ['-i', 'eth0'] },
      'capture': { tool: 'wireshark', command: 'tcpdump', params: ['-i', 'any'] },
    };

    const lowerInput = input.toLowerCase();
    let matchedTool = 'nmap';
    let matchedCommand = 'nmap';
    let matchedParams: string[] = ['-sS'];

    // Find matching tool based on keywords
    for (const [keyword, mapping] of Object.entries(toolMappings)) {
      if (lowerInput.includes(keyword)) {
        matchedTool = mapping.tool;
        matchedCommand = mapping.command;
        matchedParams = mapping.params;
        break;
      }
    }

    // Extract potential targets (IP addresses, domains, URLs)
    const ipRegex = /\b(?:\d{1,3}\.){3}\d{1,3}(?:\/\d{1,2})?\b/g;
    const domainRegex = /\b(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,}\b/gi;
    const urlRegex = /https?:\/\/[^\s]+/gi;

    const ips = input.match(ipRegex) || [];
    const domains = input.match(domainRegex) || [];
    const urls = input.match(urlRegex) || [];

    const target = urls[0] || ips[0] || domains[0] || 'target.local';
    const fullCommand = `${matchedCommand} ${matchedParams.join(' ')} ${target}`.trim();

    return {
      command: fullCommand,
      explanation: `Using ${matchedTool} to perform the requested operation on ${target}`,
      tool: matchedTool,
      parameters: matchedParams,
      confidence: 0.85,
    };
  };

  const handleTerminalSubmit = async () => {
    if (!terminalInput.trim() || isProcessing) return;

    const input = terminalInput.trim();
    setTerminalInput('');
    setIsProcessing(true);

    // Add to atomic clipboard
    addToAtomicClipboard(input);

    // Add input to history
    addTerminalEntry('input', `${aiMode ? '🤖 ' : '$ '}${input}`);

    try {
      if (aiMode) {
        // AI interpretation mode
        const aiResponse = await interpretNaturalLanguage(input);

        addTerminalEntry('ai', `💭 Interpreting: "${input}"`);
        addTerminalEntry('ai', `🔧 Tool: ${aiResponse.tool} (${Math.round(aiResponse.confidence * 100)}% confidence)`);
        addTerminalEntry('system', `📋 Command: ${aiResponse.command}`);
        addTerminalEntry('ai', `📝 ${aiResponse.explanation}`);

        // Simulate execution
        await simulateToolExecution(aiResponse.tool, aiResponse.command);
      } else {
        // Direct command mode
        await simulateToolExecution('system', input);
      }
    } catch (error) {
      addTerminalEntry('error', `Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const simulateToolExecution = async (toolId: string, command: string) => {
    // Find the tool
    const tool = tools.find(t => t.id === toolId || t.command === command.split(' ')[0]);

    addTerminalEntry('system', `⚡ Executing: ${command}`);

    // Simulate processing delay
    await new Promise(resolve => setTimeout(resolve, 1500));

    // Simulated outputs based on tool type
    const outputs: Record<string, string[]> = {
      nmap: [
        'Starting Nmap 7.94 ( https://nmap.org )',
        'Scanning target...',
        'Discovered open port 22/tcp - SSH',
        'Discovered open port 80/tcp - HTTP',
        'Discovered open port 443/tcp - HTTPS',
        'Host is up (0.032s latency)',
        'Nmap done: 1 IP address scanned in 12.45 seconds',
      ],
      metasploit: [
        'msf6 > Starting Metasploit Framework...',
        'Loading modules...',
        '2341 exploits - 1254 auxiliary - 417 post',
        'msf6 > Ready for operation',
      ],
      hydra: [
        '[STATUS] Starting brute force attack...',
        '[22][ssh] host: target login: admin',
        '[STATUS] 1 of 1 target completed',
      ],
      sqlmap: [
        '[*] Starting SQL injection test',
        '[*] Testing connection to target URL',
        '[*] Checking for SQL injection vulnerabilities',
        '[+] Parameter vulnerable to boolean-based blind',
        '[*] Database: MySQL',
      ],
      wireshark: [
        'Capturing on interface eth0',
        'Packets captured: 127',
        'TCP: 89, UDP: 28, ICMP: 10',
      ],
      aircrack: [
        'Scanning for wireless networks...',
        'Found 3 access points',
        'BSSID: AA:BB:CC:DD:EE:FF - SSID: TestNetwork',
      ],
    };

    const toolOutputs = outputs[tool?.id || 'nmap'] || ['Command executed successfully'];

    for (const line of toolOutputs) {
      await new Promise(resolve => setTimeout(resolve, 200));
      addTerminalEntry('output', line, tool?.name);
    }

    addTerminalEntry('system', '✅ Operation complete');
  };

  const loadKaliTools = async () => {
    setIsLoading(true);
    try {
      const demoTools = getDemoKaliTools();
      setTools(demoTools);
    } catch (error) {
      console.error('Failed to load Kali tools:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const loadSessions = async () => {
    try {
      const demoSessions = getDemoSessions();
      setSessions(demoSessions);
    } catch (error) {
      console.error('Failed to load sessions:', error);
    }
  };

  const getDemoKaliTools = (): KaliTool[] => [
    {
      id: 'nmap',
      name: 'Nmap',
      category: 'Reconnaissance',
      description: 'Network discovery and security auditing',
      status: 'available',
      icon: <Network className="w-5 h-5" />,
      command: 'nmap',
      parameters: ['-sS', '-sV', '-O', '-p-'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 30).toISOString(),
      successRate: 95
    },
    {
      id: 'metasploit',
      name: 'Metasploit',
      category: 'Exploitation',
      description: 'Penetration testing framework',
      status: 'available',
      icon: <Target className="w-5 h-5" />,
      command: 'msfconsole',
      parameters: ['-q', '--no-rc'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
      successRate: 88
    },
    {
      id: 'hydra',
      name: 'Hydra',
      category: 'Password Attacks',
      description: 'Brute force password cracking',
      status: 'available',
      icon: <Lock className="w-5 h-5" />,
      command: 'hydra',
      parameters: ['-l', '-p', '-t', '4'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 120).toISOString(),
      successRate: 72
    },
    {
      id: 'sqlmap',
      name: 'SQLMap',
      category: 'Web Application',
      description: 'SQL injection and database takeover',
      status: 'available',
      icon: <Database className="w-5 h-5" />,
      command: 'sqlmap',
      parameters: ['--batch', '--random-agent'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 45).toISOString(),
      successRate: 91
    },
    {
      id: 'wireshark',
      name: 'Wireshark',
      category: 'Sniffing',
      description: 'Network protocol analyzer',
      status: 'available',
      icon: <Eye className="w-5 h-5" />,
      command: 'wireshark',
      parameters: ['-k', '-i'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 15).toISOString(),
      successRate: 98
    },
    {
      id: 'aircrack',
      name: 'Aircrack-ng',
      category: 'Wireless',
      description: 'Wireless network security assessment',
      status: 'available',
      icon: <Zap className="w-5 h-5" />,
      command: 'aircrack-ng',
      parameters: ['-w', 'wordlist.txt'],
      lastUsed: new Date(Date.now() - 1000 * 60 * 90).toISOString(),
      successRate: 85
    }
  ];

  const getDemoSessions = (): KaliSession[] => [
    {
      id: '1',
      name: 'Network Reconnaissance',
      target: '192.168.1.0/24',
      status: 'completed',
      startTime: new Date(Date.now() - 1000 * 60 * 60 * 2).toISOString(),
      endTime: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
      tools: ['nmap', 'wireshark'],
      results: [
        { tool: 'nmap', findings: '3 hosts discovered', vulnerabilities: 2 },
        { tool: 'wireshark', findings: 'Suspicious traffic detected', vulnerabilities: 1 }
      ]
    },
    {
      id: '2',
      name: 'Web Application Test',
      target: 'https://example.com',
      status: 'active',
      startTime: new Date(Date.now() - 1000 * 60 * 30).toISOString(),
      tools: ['sqlmap', 'metasploit'],
      results: [
        { tool: 'sqlmap', findings: 'SQL injection vulnerability found', vulnerabilities: 1 }
      ]
    },
    {
      id: '3',
      name: 'Password Audit',
      target: 'SSH Service',
      status: 'failed',
      startTime: new Date(Date.now() - 1000 * 60 * 120).toISOString(),
      endTime: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
      tools: ['hydra'],
      results: [
        { tool: 'hydra', findings: 'Rate limiting detected', vulnerabilities: 0 }
      ]
    }
  ];

  const executeTool = async (tool: KaliTool, target: string) => {
    try {
      // Production implementation would execute actual Kali tools
      const session: KaliSession = {
        id: `session-${Date.now()}`,
        name: `${tool.name} Scan`,
        target,
        status: 'active',
        startTime: new Date().toISOString(),
        tools: [tool.id],
        results: []
      };
      
      setSessions(prev => [session, ...prev]);
      setActiveSession(session);
      
      // Simulate tool execution
      setTimeout(() => {
        setSessions(prev => prev.map(s => 
          s.id === session.id 
            ? { ...s, status: 'completed', endTime: new Date().toISOString() }
            : s
        ));
        setActiveSession(null);
      }, 5000);
      
    } catch (error) {
      console.error('Tool execution failed:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'available':
        return 'bg-green-100 text-green-800';
      case 'running':
        return 'bg-blue-100 text-blue-800';
      case 'error':
        return 'bg-red-100 text-red-800';
      case 'disabled':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'available':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'running':
        return <Play className="w-4 h-4 text-blue-500" />;
      case 'error':
        return <AlertTriangle className="w-4 h-4 text-red-500" />;
      case 'disabled':
        return <Square className="w-4 h-4 text-gray-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-500" />;
    }
  };

  const getSessionStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-blue-100 text-blue-800';
      case 'completed':
        return 'bg-green-100 text-green-800';
      case 'failed':
        return 'bg-red-100 text-red-800';
      case 'paused':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const categories = ['all', 'Reconnaissance', 'Exploitation', 'Password Attacks', 'Web Application', 'Sniffing', 'Wireless'];
  
  const filteredTools = selectedCategory === 'all' 
    ? tools 
    : tools.filter(tool => tool.category === selectedCategory);

  return (
    <div className="h-full flex flex-col space-y-4">
      {/* Header */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <Terminal className="w-8 h-8 text-green-600" />
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Virtual Kali Linux</h1>
          </div>
          <div className="flex items-center gap-2">
            <span className="bg-green-100 text-green-800 px-3 py-1 rounded text-sm font-semibold">
              Penetration Testing
            </span>
          </div>
        </div>
        
        <p className="text-gray-600 dark:text-gray-400 mb-6">
          Virtual Kali Linux environment with integrated penetration testing tools.
        </p>

        {/* System Overview */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Shield className="w-4 h-4 text-green-500" />
              <span className="text-sm font-medium text-green-800 dark:text-green-200">Available Tools</span>
            </div>
            <span className="text-2xl font-bold text-green-900 dark:text-green-100">
              {tools.filter(t => t.status === 'available').length}
            </span>
          </div>
          <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Play className="w-4 h-4 text-blue-500" />
              <span className="text-sm font-medium text-blue-800 dark:text-blue-200">Active Sessions</span>
            </div>
            <span className="text-2xl font-bold text-blue-900 dark:text-blue-100">
              {sessions.filter(s => s.status === 'active').length}
            </span>
          </div>
          <div className="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Target className="w-4 h-4 text-purple-500" />
              <span className="text-sm font-medium text-purple-800 dark:text-purple-200">Completed Tests</span>
            </div>
            <span className="text-2xl font-bold text-purple-900 dark:text-purple-100">
              {sessions.filter(s => s.status === 'completed').length}
            </span>
          </div>
          <div className="bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <AlertTriangle className="w-4 h-4 text-orange-500" />
              <span className="text-sm font-medium text-orange-800 dark:text-orange-200">Vulnerabilities Found</span>
            </div>
            <span className="text-2xl font-bold text-orange-900 dark:text-orange-100">
              {sessions.reduce((sum, s) => 
                sum + s.results.reduce((rSum, r) => rSum + (r.vulnerabilities || 0), 0), 0
              )}
            </span>
          </div>
        </div>
      </div>

      {/* Tool Categories */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white mb-6">Kali Tools</h2>
        
        <div className="flex gap-4 mb-6 overflow-x-auto">
          {categories.map((category) => (
            <button
              key={category}
              onClick={() => setSelectedCategory(category)}
              className={`px-4 py-2 rounded-md font-medium whitespace-nowrap ${
                selectedCategory === category 
                  ? 'bg-green-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              }`}
            >
              {category} ({category === 'all' ? tools.length : tools.filter(t => t.category === category).length})
            </button>
          ))}
        </div>

        {/* Tools Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredTools.map((tool) => (
            <div key={tool.id} className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow bg-white dark:bg-gray-700">
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center gap-3">
                  <div className="p-2 bg-green-100 dark:bg-green-900/20 rounded-lg">
                    {tool.icon}
                  </div>
                  <div>
                    <h3 className="font-semibold text-gray-900 dark:text-white">{tool.name}</h3>
                    <div className="flex items-center gap-2 mt-1">
                      {getStatusIcon(tool.status)}
                      <span className={`px-2 py-1 rounded text-xs font-semibold ${getStatusColor(tool.status)}`}>
                        {tool.status}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              
              <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
                {tool.description}
              </p>
              
              <div className="space-y-2 mb-4">
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Success Rate:</span>
                  <span className="font-medium text-gray-900 dark:text-white">{tool.successRate}%</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Last Used:</span>
                  <span className="font-medium text-gray-900 dark:text-white">{new Date(tool.lastUsed).toLocaleDateString()}</span>
                </div>
              </div>
              
              <div className="flex gap-2">
                <button
                  onClick={() => executeTool(tool, 'target.example.com')}
                  disabled={tool.status !== 'available'}
                  className="flex-1 px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors text-sm"
                >
                  Execute
                </button>
                <button
                  className="px-3 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 transition-colors text-sm"
                >
                  <Settings className="w-4 h-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Active Sessions */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-800 dark:text-white mb-6">Active Sessions</h2>
        
        {sessions.length === 0 ? (
          <div className="text-center py-8">
            <Terminal className="w-12 h-12 mx-auto mb-4 text-gray-400" />
            <p className="text-gray-500 dark:text-gray-400">No active sessions</p>
          </div>
        ) : (
          <div className="space-y-4">
            {sessions.map((session) => (
              <div key={session.id} className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow bg-white dark:bg-gray-700">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <h3 className="font-semibold text-gray-900 dark:text-white">{session.name}</h3>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Target: {session.target}</p>
                  </div>
                  <span className={`px-2 py-1 rounded text-xs font-semibold ${getSessionStatusColor(session.status)}`}>
                    {session.status}
                  </span>
                </div>
                
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                  <div>
                    <span className="text-gray-500 dark:text-gray-400">Start Time:</span>
                    <p className="font-medium text-gray-900 dark:text-white">{new Date(session.startTime).toLocaleTimeString()}</p>
                  </div>
                  {session.endTime && (
                    <div>
                      <span className="text-gray-500 dark:text-gray-400">End Time:</span>
                      <p className="font-medium text-gray-900 dark:text-white">{new Date(session.endTime).toLocaleTimeString()}</p>
                    </div>
                  )}
                  <div>
                    <span className="text-gray-500 dark:text-gray-400">Tools Used:</span>
                    <p className="font-medium text-gray-900 dark:text-white">{session.tools.join(', ')}</p>
                  </div>
                  <div>
                    <span className="text-gray-500 dark:text-gray-400">Vulnerabilities:</span>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {session.results.reduce((sum, r) => sum + (r.vulnerabilities || 0), 0)}
                    </p>
                  </div>
                </div>
                
                {session.results.length > 0 && (
                  <div className="mt-4">
                    <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Results:</h4>
                    <div className="space-y-2">
                      {session.results.map((result, index) => (
                        <div key={index} className="bg-gray-50 dark:bg-gray-600 p-3 rounded">
                          <div className="flex justify-between text-sm">
                            <span className="font-medium text-gray-900 dark:text-white">{result.tool}:</span>
                            <span className="text-gray-600 dark:text-gray-400">{result.findings}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default VKali;
