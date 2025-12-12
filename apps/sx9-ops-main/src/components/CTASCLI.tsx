import React, { useState, useRef, useEffect } from 'react';
import {
  Terminal, RotateCcw, Download, Settings, Brain, Bot, Workflow, BarChart3, Layers, GitBranch, Sparkles, History, Lightbulb, Code, Cpu, Clipboard, Copy
} from 'lucide-react';
import { SYSTEM_PORTS } from '@/hooks';

interface CTASCLIProps {
  sessionId?: string;
  onCommand?: (command: string, environment: string) => void;
}

interface CommandHistory {
  id: string;
  command: string;
  output: string;
  timestamp: Date;
  status: 'success' | 'error' | 'running';
  environment: string;
  aiSuggestion?: string;
  executionTime?: number;
}

// interface AIJob {
//   id: string;
//   type: 'training' | 'inference' | 'analysis' | 'workflow';
//   status: 'queued' | 'running' | 'completed' | 'failed';
//   progress: number;
//   startTime: Date;
//   endTime?: Date;
//   model?: string;
//   dataset?: string;
//   results?: Record<string, unknown>;
// }

interface MCPServer {
  id: string;
  name: string;
  type: 'tool' | 'data-source' | 'model' | 'agent';
  status: 'connected' | 'disconnected' | 'error';
  capabilities: string[];
  lastUsed: Date;
  responseTime: number;
}

interface Persona {
  id: string;
  name: string;
  role: 'analyst' | 'operator' | 'researcher' | 'administrator' | 'ai-assistant';
  expertise: string[];
  personality: string;
  communicationStyle: string;
  context: string;
  preferences: {
    detailLevel: 'brief' | 'detailed' | 'verbose';
    technicalLevel: 'basic' | 'intermediate' | 'advanced';
    automationLevel: 'manual' | 'semi-auto' | 'full-auto';
  };
}

interface AIContext {
  currentPersona: Persona;
  activeMCPServers: MCPServer[];
  sessionContext: {
    currentTask: string;
    recentCommands: string[];
    systemState: Record<string, unknown>;
    userIntent: string;
  };
  aiCapabilities: {
    naturalLanguage: boolean;
    codeGeneration: boolean;
    analysis: boolean;
    automation: boolean;
  };
}

type Environment = 'ai-ml' | 'cognitive' | 'ooda' | 'slotgraph' | 'workflow' | 'analytics' | 'system' | 'automation';

interface EnvironmentConfig {
  name: string;
  icon: React.ReactNode;
  prompt: string;
  description: string;
  commands: string[];
  aiCapabilities: string[];
  examples: string[];
}

const CTASCLI: React.FC<CTASCLIProps> = ({ sessionId = 'default', onCommand }) => {
  const [commandHistory, setCommandHistory] = useState<CommandHistory[]>([]);
  const [currentCommand, setCurrentCommand] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const [currentEnvironment, setCurrentEnvironment] = useState<Environment>('ai-ml');
  const [showEnvironments, setShowEnvironments] = useState(false);
  const [showCommandHistory, setShowCommandHistory] = useState(false);
  const [currentPersona, setCurrentPersona] = useState<Persona | null>(null);
  const [mcpServers, setMcpServers] = useState<MCPServer[]>([]);
  const [aiContext, setAiContext] = useState<AIContext | null>(null);
  const [showPersonaSelector, setShowPersonaSelector] = useState(false);
  const [showMCPServers, setShowMCPServers] = useState(false);
  const [naturalLanguageMode, setNaturalLanguageMode] = useState(false);
  const [atomicClipboard, setAtomicClipboard] = useState<string[]>([]);
  const [clipboardConnected, setClipboardConnected] = useState(false);
  const [showAtomicClipboard, setShowAtomicClipboard] = useState(false);
  const terminalRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Persona Framework
  const personas: Persona[] = [
    {
      id: 'threat-analyst',
      name: 'Alex Threat',
      role: 'analyst',
      expertise: ['threat-intelligence', 'malware-analysis', 'incident-response'],
      personality: 'Analytical, detail-oriented, methodical',
      communicationStyle: 'Technical but clear, provides context and explanations',
      context: 'Senior threat analyst with 8+ years experience in cybersecurity',
      preferences: {
        detailLevel: 'detailed',
        technicalLevel: 'advanced',
        automationLevel: 'semi-auto'
      }
    },
    {
      id: 'ai-operator',
      name: 'Sam Operator',
      role: 'operator',
      expertise: ['system-administration', 'automation', 'monitoring'],
      personality: 'Efficient, practical, results-driven',
      communicationStyle: 'Concise, action-oriented, focuses on outcomes',
      context: 'System operator focused on maintaining optimal CTAS performance',
      preferences: {
        detailLevel: 'brief',
        technicalLevel: 'intermediate',
        automationLevel: 'full-auto'
      }
    },
    {
      id: 'researcher',
      name: 'Dr. Maya Research',
      role: 'researcher',
      expertise: ['machine-learning', 'data-science', 'research-methodology'],
      personality: 'Curious, thorough, innovative',
      communicationStyle: 'Educational, explains concepts, suggests improvements',
      context: 'AI/ML researcher developing next-generation threat detection',
      preferences: {
        detailLevel: 'verbose',
        technicalLevel: 'advanced',
        automationLevel: 'manual'
      }
    },
    {
      id: 'ai-assistant',
      name: 'CTAS AI',
      role: 'ai-assistant',
      expertise: ['natural-language-processing', 'automation', 'analysis'],
      personality: 'Helpful, intelligent, adaptive',
      communicationStyle: 'Conversational, proactive, context-aware',
      context: 'AI assistant designed to enhance operator productivity',
      preferences: {
        detailLevel: 'detailed',
        technicalLevel: 'intermediate',
        automationLevel: 'full-auto'
      }
    }
  ];

  // MCP Server Configuration
  const defaultMCPServers: MCPServer[] = [
    {
      id: 'threat-intel-server',
      name: 'Threat Intelligence MCP',
      type: 'data-source',
      status: 'connected',
      capabilities: ['threat-feeds', 'ioc-lookup', 'reputation-check'],
      lastUsed: new Date(),
      responseTime: 45
    },
    {
      id: 'ml-model-server',
      name: 'ML Model MCP',
      type: 'model',
      status: 'connected',
      capabilities: ['inference', 'training', 'optimization'],
      lastUsed: new Date(),
      responseTime: 120
    },
    {
      id: 'automation-server',
      name: 'Automation MCP',
      type: 'tool',
      status: 'connected',
      capabilities: ['workflow-execution', 'task-automation', 'orchestration'],
      lastUsed: new Date(),
      responseTime: 30
    },
    {
      id: 'cognitive-server',
      name: 'Cognitive Engine MCP',
      type: 'agent',
      status: 'connected',
      capabilities: ['reasoning', 'learning', 'adaptation'],
      lastUsed: new Date(),
      responseTime: 200
    }
  ];

  const environments: Record<Environment, EnvironmentConfig> = {
    'ai-ml': {
      name: 'AI/ML Operations',
      icon: <Brain size={14} />,
      prompt: 'ai@ctas:~$',
      description: 'AI/ML model training, inference, and analysis',
      commands: ['train', 'infer', 'analyze', 'deploy', 'monitor', 'optimize', 'help'],
      aiCapabilities: ['Model Training', 'Inference Pipeline', 'AutoML', 'Model Optimization'],
      examples: ['train --model bert --dataset threat-intel', 'infer --model trained-model --input data.json']
    },
    cognitive: {
      name: 'Cognitive Modules',
      icon: <Bot size={14} />,
      prompt: 'cogni@ctas:~$',
      description: 'Cognitive processing and reasoning',
      commands: ['reason', 'learn', 'adapt', 'query', 'context', 'help'],
      aiCapabilities: ['Cognitive Reasoning', 'Context Learning', 'Adaptive Processing', 'Knowledge Synthesis'],
      examples: ['reason --context threat-analysis --query "identify patterns"', 'learn --from incident-data --update knowledge-base']
    },
    ooda: {
      name: 'OODA Loop',
      icon: <Workflow size={14} />,
      prompt: 'ooda@ctas:~$',
      description: 'Observe, Orient, Decide, Act loop operations',
      commands: ['observe', 'orient', 'decide', 'act', 'loop', 'status', 'help'],
      aiCapabilities: ['Real-time Observation', 'Dynamic Orientation', 'AI Decision Making', 'Automated Action'],
      examples: ['observe --sources network,logs,feeds', 'decide --threat-level high --action block-ip']
    },
    slotgraph: {
      name: 'SlotGraph',
      icon: <Layers size={14} />,
      prompt: 'slot@ctas:~$',
      description: 'SlotGraph neural architecture operations',
      commands: ['create', 'train', 'query', 'visualize', 'export', 'help'],
      aiCapabilities: ['Neural Architecture', 'Graph Processing', 'Pattern Recognition', 'Knowledge Representation'],
      examples: ['create --slots 100 --connections 500', 'query --pattern "threat-indicator" --depth 3']
    },
    workflow: {
      name: 'Workflow Automation',
      icon: <GitBranch size={14} />,
      prompt: 'workflow@ctas:~$',
      description: 'AI-powered workflow orchestration',
      commands: ['create', 'run', 'monitor', 'optimize', 'schedule', 'help'],
      aiCapabilities: ['Workflow Orchestration', 'Conditional Logic', 'Error Recovery', 'Performance Optimization'],
      examples: ['create --name threat-response --steps detect,analyze,respond', 'run --workflow incident-handling --trigger alert']
    },
    analytics: {
      name: 'Analytics Engine',
      icon: <BarChart3 size={14} />,
      prompt: 'analytics@ctas:~$',
      description: 'Advanced analytics and insights',
      commands: ['analyze', 'predict', 'correlate', 'visualize', 'report', 'help'],
      aiCapabilities: ['Predictive Analytics', 'Correlation Analysis', 'Anomaly Detection', 'Insight Generation'],
      examples: ['analyze --data threat-feeds --method clustering', 'predict --model threat-evolution --horizon 7d']
    },
    system: {
      name: 'System Control',
      icon: <Cpu size={14} />,
      prompt: 'system@ctas:~$',
      description: 'System-wide AI operations and control',
      commands: ['status', 'deploy', 'monitor', 'config', 'health', 'help'],
      aiCapabilities: ['System Monitoring', 'Resource Management', 'Health Checks', 'Configuration Management'],
      examples: ['status --ai-services', 'deploy --model new-threat-detector --environment production']
    },
    automation: {
      name: 'Automation Engine',
      icon: <Sparkles size={14} />,
      prompt: 'auto@ctas:~$',
      description: 'AI-driven automation and orchestration',
      commands: ['automate', 'schedule', 'trigger', 'monitor', 'optimize', 'help'],
      aiCapabilities: ['Task Automation', 'Intelligent Scheduling', 'Event Triggering', 'Self-Optimization'],
      examples: ['automate --task threat-hunting --frequency hourly', 'trigger --event new-threat --action deploy-response']
    }
  };

  // Initialize AI Context and MCP Servers
  useEffect(() => {
    setMcpServers(defaultMCPServers);
    const defaultPersona = personas[0] || personas.find(p => p.id === 'threat-analyst');
    if (defaultPersona) {
      setCurrentPersona(defaultPersona);
      setAiContext({
        currentPersona: defaultPersona,
        activeMCPServers: defaultMCPServers,
        sessionContext: {
          currentTask: 'Initializing CTAS AI CLI',
          recentCommands: [],
          systemState: { status: 'ready' },
          userIntent: 'exploration'
        },
        aiCapabilities: {
          naturalLanguage: true,
          codeGeneration: true,
          analysis: true,
          automation: true
        }
      });
    }
  }, []);

  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [commandHistory]);

  // Initialize Atomic Clipboard connection
  useEffect(() => {
    const connectToAtomicClipboard = async () => {
      try {
        const response = await fetch(`http://localhost:${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD}/health`);
        if (response.ok) {
          setClipboardConnected(true);
          loadAtomicClipboard();
        }
      } catch (error) {
        console.log('Atomic Clipboard not available:', error);
        setClipboardConnected(false);
      }
    };

    connectToAtomicClipboard();
  }, []);

  // Atomic Clipboard Functions
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
      const response = await fetch(`http://localhost:${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD}/clipboard`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, source: 'ctas-ai-cli', persona: currentPersona?.name || 'system' })
      });
      if (response.ok) {
        loadAtomicClipboard();
      }
    } catch (error) {
      console.error('Failed to add to atomic clipboard:', error);
    }
  };

  const copyFromAtomicClipboard = async (index: number) => {
    if (atomicClipboard[index]) {
      try {
        await navigator.clipboard.writeText(atomicClipboard[index]);
        // Also set it as current command for easy execution
        setCurrentCommand(atomicClipboard[index]);
      } catch (error) {
        console.error('Failed to copy from atomic clipboard:', error);
      }
    }
  };

  const executeCommand = async (command: string) => {
    if (!command.trim()) return;

    const startTime = Date.now();
    const commandId = Date.now().toString();
    
    // Update AI context
    if (aiContext) {
      setAiContext(prev => prev ? {
        ...prev,
        sessionContext: {
          ...prev.sessionContext,
          recentCommands: [...prev.sessionContext.recentCommands.slice(-9), command],
          currentTask: command
        }
      } : null);
    }
    
    // Generate AI suggestion for the command
    const aiSuggestion = generateAISuggestion(command, currentEnvironment);
    
    const newCommand: CommandHistory = {
      id: commandId,
      command,
      output: '',
      timestamp: new Date(),
      status: 'running',
      environment: currentEnvironment,
      aiSuggestion
    };

    setCommandHistory(prev => [...prev, newCommand]);
    setIsRunning(true);
    setCurrentCommand('');

    // Execute command with AI enhancement and MCP integration
    try {
      let output = '';
      
      if (naturalLanguageMode) {
        output = await executeNaturalLanguageCommand(command);
      } else {
        output = await executeAICommand(command, currentEnvironment);
      }
      
      const executionTime = Date.now() - startTime;
      
      setCommandHistory(prev =>
        prev.map(cmd =>
          cmd.id === commandId
            ? { ...cmd, output, status: 'success' as const, executionTime }
            : cmd
        )
      );

      // Auto-save successful commands to atomic clipboard
      if (clipboardConnected && command.trim().length > 3) {
        addToAtomicClipboard(command);
      }
    } catch (error) {
      setCommandHistory(prev => 
        prev.map(cmd => 
          cmd.id === commandId 
            ? { ...cmd, output: `Error: ${error}`, status: 'error' as const, executionTime: Date.now() - startTime }
            : cmd
        )
      );
    }
    
    setIsRunning(false);
    onCommand?.(command, currentEnvironment);
  };

  const executeNaturalLanguageCommand = async (command: string): Promise<string> => {
    // Simulate natural language processing with MCP servers
    await new Promise(resolve => setTimeout(resolve, 500));
    
    const persona = currentPersona;
    const _context = aiContext?.sessionContext;
    
    if (command.toLowerCase().includes('analyze') || command.toLowerCase().includes('threat')) {
      return `🧠 ${persona?.name} analyzing threat data...
      
📊 Using MCP Servers:
- Threat Intelligence MCP: Querying threat feeds
- ML Model MCP: Running anomaly detection
- Cognitive Engine MCP: Pattern recognition

🔍 Analysis Results:
${persona?.role === 'analyst' ? 
  '🎯 Threat Level: HIGH\n📈 Confidence: 94%\n⚠️ Immediate action required' :
  persona?.role === 'researcher' ?
  '🔬 Novel patterns detected\n📊 Statistical significance: p < 0.001\n💡 Research implications identified' :
  '⚡ System optimized\n🔄 Automation triggered\n📊 Performance improved'
}

💡 ${persona?.name}'s Recommendation: ${persona?.role === 'analyst' ? 
  'Implement immediate containment measures' :
  persona?.role === 'researcher' ?
  'Further investigation of emerging patterns recommended' :
  'System response automated successfully'
}`;
    }
    
    if (command.toLowerCase().includes('help') || command.toLowerCase().includes('what can you do')) {
      return `🤖 ${persona?.name} here! I'm your ${persona?.role} assistant.

🎯 My Expertise: ${persona?.expertise.join(', ')}
💬 Communication Style: ${persona?.communicationStyle}

🚀 Available MCP Servers:
${mcpServers.map(server => `- ${server.name} (${server.status})`).join('\n')}

💡 Natural Language Commands:
- "Analyze the current threat landscape"
- "Show me system performance metrics"
- "Run a security assessment"
- "What's the latest threat intelligence?"

🔧 Technical Commands:
- Use 'help' in any environment for specific commands
- Switch environments with the environment selector
- Toggle natural language mode with the AI button

${persona?.role === 'ai-assistant' ? 
  '🤖 I can also help with:\n- Code generation\n- Workflow automation\n- Data analysis\n- System optimization' : ''
}`;
    }
    
    return `💬 ${persona?.name} processing your request...
🤖 Natural Language Processing: Active
🔗 MCP Integration: Connected

${persona?.communicationStyle === 'Technical but clear' ? 
  '📊 Technical Analysis: Command processed through AI pipeline' :
  persona?.communicationStyle === 'Concise, action-oriented' ?
  '⚡ Action Taken: Request executed efficiently' :
  '🔬 Research Mode: Analyzing request for insights'
}

💡 ${persona?.name}'s Response: ${persona?.role === 'analyst' ? 
  'Analysis complete. Review results above.' :
  persona?.role === 'operator' ?
  'Task completed successfully.' :
  persona?.role === 'researcher' ?
  'Interesting patterns detected. Further investigation recommended.' :
  'I\'ve processed your request and provided relevant information.'
}`;
  };

  const generateAISuggestion = (command: string, environment: Environment): string => {
    const suggestions: Record<Environment, string[]> = {
      'ai-ml': [
        '💡 Try adding --optimize flag for better performance',
        '🤖 Consider using --auto-tune for hyperparameter optimization',
        '📊 Add --metrics=detailed for comprehensive evaluation'
      ],
      'cognitive': [
        '🧠 Use --context=enhanced for deeper reasoning',
        '🔍 Try --pattern-recognition for better insights',
        '📚 Add --knowledge-base=latest for updated information'
      ],
      'ooda': [
        '👁️ Consider --real-time for live observation',
        '🎯 Use --threat-priority for focused decision making',
        '⚡ Add --auto-response for immediate action'
      ],
      'slotgraph': [
        '🕸️ Try --depth=5 for deeper graph traversal',
        '🔗 Use --relations=all for comprehensive analysis',
        '📈 Add --visualization=3d for better insights'
      ],
      'workflow': [
        '⚙️ Consider --parallel for concurrent execution',
        '🔄 Use --retry=auto for error recovery',
        '📋 Add --logging=detailed for better monitoring'
      ],
      'analytics': [
        '📊 Try --algorithm=advanced for better results',
        '🔮 Use --prediction=ensemble for improved accuracy',
        '📈 Add --trend-analysis for pattern recognition'
      ],
      'system': [
        '🔧 Consider --health-check for system diagnostics',
        '⚡ Use --optimize for performance tuning',
        '📊 Add --metrics=real-time for live monitoring'
      ],
      'automation': [
        '🤖 Try --ai-enhanced for intelligent automation',
        '⚡ Use --smart-trigger for context-aware execution',
        '🔄 Add --self-optimize for continuous improvement'
      ]
    };

    const envSuggestions = suggestions[environment] || [];
    return envSuggestions[Math.floor(Math.random() * envSuggestions.length)] || '💡 Try using the help command for guidance';
  };

  const executeAICommand = async (command: string, environment: Environment): Promise<string> => {
    const cmd = command.toLowerCase();
    
    // Simulate AI-enhanced command execution
    await new Promise(resolve => setTimeout(resolve, 300 + Math.random() * 700));
    
    switch (environment) {
      case 'ai-ml':
        return executeAIMLCommand(cmd);
      case 'cognitive':
        return executeCognitiveCommand(cmd);
      case 'ooda':
        return executeOODACommand(cmd);
      case 'slotgraph':
        return executeSlotGraphCommand(cmd);
      case 'workflow':
        return executeWorkflowCommand(cmd);
      case 'analytics':
        return executeAnalyticsCommand(cmd);
      case 'system':
        return executeSystemCommand(cmd);
      case 'automation':
        return executeAutomationCommand(cmd);
      default:
        return `Unknown environment: ${environment}`;
    }
  };

  // AI-Enhanced Command Execution Functions
  const executeAIMLCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🤖 AI/ML Operations - Available Commands:

📚 Training Commands:
  train --model=<model> --dataset=<dataset> [--optimize] [--auto-tune]
  train --model=transformer --dataset=threat-intel --epochs=100

🔮 Inference Commands:
  infer --model=<model> --input=<data> [--batch-size=<size>]
  infer --model=threat-detector --input=network-traffic.json

📊 Analysis Commands:
  analyze --data=<dataset> --method=<algorithm> [--metrics=detailed]
  analyze --data=malware-samples --method=clustering

🚀 Deployment Commands:
  deploy --model=<model> --environment=<env> [--scale=<instances>]
  deploy --model=anomaly-detector --environment=production

📈 Monitoring Commands:
  monitor --model=<model> [--metrics=real-time] [--alerts=on]
  monitor --model=all --metrics=performance,accuracy

⚡ Optimization Commands:
  optimize --model=<model> [--target=latency|accuracy|throughput]
  optimize --model=detector --target=latency

💡 AI Suggestions:
  - Use --auto-tune for automatic hyperparameter optimization
  - Add --metrics=detailed for comprehensive evaluation
  - Try --optimize for performance tuning`;
    }
    
    if (cmd.includes('train')) {
      return `🚀 Starting AI/ML Training...
📊 Model: ${cmd.includes('transformer') ? 'Transformer' : 'Neural Network'}
📚 Dataset: Threat Intelligence Corpus
⚡ Auto-optimization: Enabled
🔄 Epochs: 100
⏱️ Estimated time: 45 minutes

Training Progress:
Epoch 1/100 - Loss: 2.34 - Accuracy: 0.67
Epoch 2/100 - Loss: 1.89 - Accuracy: 0.72
...
Epoch 100/100 - Loss: 0.23 - Accuracy: 0.94

✅ Training completed successfully!
📈 Final metrics: Accuracy=94%, Precision=92%, Recall=89%
💾 Model saved to: /models/threat-detector-v2.1.pkl`;
    }
    
    if (cmd.includes('infer')) {
      return `🔮 Running AI Inference...
🤖 Model: Threat Detection Engine
📥 Input: Network traffic data
⚡ Processing: Real-time

Results:
🎯 Threat Detection: 3 threats identified
⚠️ High Risk: 1 (Malware signature detected)
🟡 Medium Risk: 2 (Suspicious behavior patterns)
✅ Low Risk: 0

📊 Confidence Scores:
- Threat 1: 94.2% (Malware: Emotet variant)
- Threat 2: 78.5% (Behavior: Data exfiltration attempt)
- Threat 3: 67.3% (Pattern: Reconnaissance activity)

💡 AI Recommendation: Block IP 192.168.1.45, quarantine affected systems`;
    }
    
    if (cmd.includes('analyze')) {
      return `📊 Running AI Analysis...
🧠 Algorithm: Advanced Clustering + Pattern Recognition
📈 Dataset: 10,000 threat samples
⚡ Processing: Parallel execution

Analysis Results:
🔍 Clusters Identified: 7 threat families
📊 Patterns Found: 23 behavioral signatures
🎯 Anomalies Detected: 5 novel threats

Top Threat Families:
1. APT-29 (Cozy Bear) - 34% of samples
2. Lazarus Group - 28% of samples
3. Fancy Bear - 22% of samples
4. Unknown/Novel - 16% of samples

💡 AI Insights:
- Emerging threat pattern: Supply chain attacks
- Recommended action: Update detection rules
- Risk assessment: High (novel threats detected)`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available AI/ML commands`;
  };

  const executeCognitiveCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🧠 Cognitive Engine - Available Commands:

🧠 Reasoning Commands:
  reason --context=<context> --query=<query> [--depth=<levels>]
  reason --context=threat-analysis --query="identify attack patterns"

📚 Learning Commands:
  learn --from=<data> --update=<knowledge-base> [--validate]
  learn --from=incident-reports --update=threat-knowledge

🔄 Adaptation Commands:
  adapt --scenario=<scenario> [--optimize] [--validate]
  adapt --scenario=new-threat --optimize=response

🔍 Query Commands:
  query --knowledge=<base> --question=<question> [--format=<output>]
  query --knowledge=threat-db --question="What is APT-29's TTP?"

📋 Context Commands:
  context --set=<context> [--merge] [--clear]
  context --set=network-defense --merge=threat-intel`;
    }
    
    if (cmd.includes('reason')) {
      return `🧠 Cognitive Reasoning Engine...
🎯 Context: Threat Analysis
❓ Query: Identify attack patterns
🔍 Processing: Multi-layer reasoning

Reasoning Results:
🧩 Pattern 1: Supply Chain Compromise
   - Indicators: Modified binaries, unusual network traffic
   - Confidence: 87%
   - Related: SolarWinds, Kaseya incidents

🧩 Pattern 2: Credential Harvesting
   - Indicators: Phishing emails, fake login pages
   - Confidence: 92%
   - Related: APT-29, Lazarus Group

🧩 Pattern 3: Lateral Movement
   - Indicators: SMB scanning, credential dumping
   - Confidence: 78%
   - Related: APT-28, APT-41

💡 Cognitive Insights:
- Primary threat actor: APT-29 (Cozy Bear)
- Attack vector: Supply chain compromise
- Recommended response: Isolate affected systems, update credentials`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available cognitive commands`;
  };

  const executeOODACommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🔄 OODA Loop Operations - Available Commands:

👁️ Observe Commands:
  observe --sources=<sources> [--real-time] [--filter=<criteria>]
  observe --sources=network,logs,feeds --real-time

🧭 Orient Commands:
  orient --context=<situation> [--threat-level=<level>] [--priority=<level>]
  orient --context=network-breach --threat-level=high

🎯 Decide Commands:
  decide --threat=<threat> [--options=<actions>] [--ai-enhanced]
  decide --threat=malware-outbreak --ai-enhanced

⚡ Act Commands:
  act --action=<action> [--target=<target>] [--validate]
  act --action=block-ip --target=192.168.1.45

🔄 Loop Commands:
  loop --continuous [--interval=<seconds>] [--adaptive]
  loop --continuous --interval=30`;
    }
    
    if (cmd.includes('observe')) {
      return `👁️ OODA: Observing Environment...
📡 Sources: Network sensors, log feeds, threat intel
⚡ Mode: Real-time monitoring
🔍 Filters: Active threats, suspicious activity

Observation Results:
🎯 Active Threats: 3 detected
📊 Network Anomalies: 7 identified
⚠️ Security Alerts: 12 triggered
🔄 System Status: All sensors operational

Real-time Data:
- Network traffic: 2.3 TB/hour
- Log entries: 45,000/minute
- Threat feeds: 150 active sources
- Sensor health: 98.7% uptime

💡 AI Analysis: Elevated threat level detected`;
    }
    
    if (cmd.includes('decide')) {
      return `🎯 OODA: Decision Making...
🤖 AI Enhancement: Enabled
🎯 Threat: Malware outbreak detected
⚡ Decision Engine: Multi-criteria analysis

Decision Process:
1️⃣ Threat Assessment: High risk (malware spreading)
2️⃣ Impact Analysis: Critical systems affected
3️⃣ Response Options: 5 alternatives evaluated
4️⃣ AI Recommendation: Immediate containment

🎯 Final Decision: QUARANTINE AND CONTAIN
- Action: Isolate affected network segments
- Priority: Critical (immediate execution)
- Resources: Dedicated response team
- Timeline: 15 minutes to full containment

💡 AI Confidence: 94% - Recommended action optimal`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available OODA commands`;
  };

  const executeSlotGraphCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🕸️ SlotGraph Operations - Available Commands:

🕸️ Query Commands:
  query --pattern=<pattern> [--depth=<levels>] [--relations=<type>]
  query --pattern="threat-actor" --depth=3

🔄 Traverse Commands:
  traverse --start=<node> [--relations=<type>] [--limit=<count>]
  traverse --start=ip-address --relations=all

📝 Update Commands:
  update --node=<node> --data=<data> [--merge] [--validate]
  update --node=threat-actor --data=new-ttp

🔗 Merge Commands:
  merge --source=<graph> --target=<graph> [--strategy=<method>]
  merge --source=threat-intel --target=main-graph

📊 Visualize Commands:
  visualize --type=<format> [--export=<file>] [--interactive]
  visualize --type=3d --export=threat-graph.html`;
    }
    
    if (cmd.includes('query')) {
      return `🕸️ SlotGraph Query Engine...
🔍 Pattern: Threat Actor Analysis
📊 Depth: 3 levels
🔗 Relations: All types

Query Results:
🎯 Threat Actor: APT-29 (Cozy Bear)
📊 Nodes Found: 1,247
🔗 Relationships: 3,891

Graph Analysis:
🧩 Level 1: Direct connections
   - Campaigns: 15 active
   - Tools: 23 identified
   - Targets: 47 organizations

🧩 Level 2: Secondary connections
   - Infrastructure: 89 IP addresses
   - Domains: 156 registered
   - Malware: 34 variants

🧩 Level 3: Tertiary connections
   - Supply chain: 12 vendors
   - Partners: 23 organizations
   - Affiliates: 8 groups

💡 AI Insights: Strong correlation with recent attacks`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available SlotGraph commands`;
  };

  const executeWorkflowCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `⚙️ Workflow Automation - Available Commands:

⚙️ Create Commands:
  create --name=<workflow> --steps=<steps> [--parallel] [--retry]
  create --name=threat-response --steps=detect,analyze,respond

🔄 Run Commands:
  run --workflow=<name> [--trigger=<event>] [--parameters=<params>]
  run --workflow=incident-handling --trigger=alert

📊 Monitor Commands:
  monitor --workflow=<name> [--real-time] [--metrics=<types>]
  monitor --workflow=all --real-time

⚡ Optimize Commands:
  optimize --workflow=<name> [--target=<metric>] [--ai-enhanced]
  optimize --workflow=threat-response --target=response-time

📅 Schedule Commands:
  schedule --workflow=<name> --cron=<expression> [--enabled]
  schedule --workflow=threat-hunting --cron="0 */6 * * *"`;
    }
    
    if (cmd.includes('run')) {
      return `⚙️ Executing Workflow...
🔄 Workflow: Incident Response Pipeline
⚡ Trigger: Security Alert
🤖 AI Enhancement: Enabled

Workflow Execution:
1️⃣ Detection Phase: ✅ Completed (2.3s)
   - Alert correlation: 3 related events
   - Threat classification: High risk

2️⃣ Analysis Phase: ✅ Completed (8.7s)
   - IOC extraction: 12 indicators
   - Impact assessment: Critical systems affected

3️⃣ Response Phase: 🔄 In Progress
   - Containment: Isolating affected systems
   - Eradication: Removing malware
   - Recovery: Restoring services

📊 Performance Metrics:
- Total execution time: 11.0s
- AI optimization: 23% faster than baseline
- Success rate: 100% (so far)

💡 AI Recommendation: Continue with automated response`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available workflow commands`;
  };

  const executeAnalyticsCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `📊 Analytics Engine - Available Commands:

📊 Analyze Commands:
  analyze --data=<dataset> --method=<algorithm> [--parameters=<params>]
  analyze --data=threat-feeds --method=clustering

🔮 Predict Commands:
  predict --model=<model> --input=<data> [--horizon=<time>]
  predict --model=threat-evolution --input=current-data

🔗 Correlate Commands:
  correlate --datasets=<datasets> [--method=<algorithm>] [--threshold=<value>]
  correlate --datasets=logs,feeds,alerts --method=pearson

📈 Visualize Commands:
  visualize --type=<chart> --data=<dataset> [--export=<file>]
  visualize --type=network-topology --data=threat-relationships

📋 Report Commands:
  report --type=<format> --data=<dataset> [--template=<template>]
  report --type=executive --data=threat-analysis`;
    }
    
    if (cmd.includes('analyze')) {
      return `📊 Running Advanced Analytics...
🧠 Algorithm: Multi-dimensional clustering
📈 Dataset: Threat intelligence feeds
⚡ Processing: Distributed computation

Analysis Results:
🔍 Clusters Identified: 12 threat groups
📊 Patterns Discovered: 47 behavioral signatures
🎯 Anomalies Detected: 8 novel threats

Top Insights:
1️⃣ Emerging Threat: Supply chain attacks (↑ 340%)
2️⃣ Geographic Focus: APAC region (↑ 67%)
3️⃣ Target Industry: Healthcare (↑ 89%)
4️⃣ Attack Vector: Ransomware (↑ 156%)

📈 Statistical Summary:
- Data points: 2.3M threat indicators
- Time range: Last 30 days
- Confidence: 94.7%
- Processing time: 3.2 seconds

💡 AI Recommendation: Focus on supply chain security`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available analytics commands`;
  };

  const executeSystemCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🔧 System Control - Available Commands:

📊 Status Commands:
  status --services=<services> [--health=detailed] [--metrics=real-time]
  status --services=all --health=detailed

🚀 Deploy Commands:
  deploy --service=<service> --version=<version> [--environment=<env>]
  deploy --service=ai-engine --version=2.1.0 --environment=production

📊 Monitor Commands:
  monitor --target=<target> [--metrics=<types>] [--alerts=<level>]
  monitor --target=all --metrics=cpu,memory,network

📋 Logs Commands:
  logs --service=<service> [--level=<level>] [--tail=<lines>]
  logs --service=all --level=error --tail=100

⚙️ Config Commands:
  config --show=<section> [--edit] [--validate]
  config --show=ai-services --edit

❤️ Health Commands:
  health --check=<component> [--detailed] [--fix]
  health --check=all --detailed`;
    }
    
    if (cmd.includes('status')) {
      return `🔧 System Status Report...
📊 Services: All components
❤️ Health: Detailed analysis
⚡ Metrics: Real-time monitoring

System Overview:
🟢 Frontend: Healthy (uptime: 99.9%)
🟢 Backend API: Healthy (uptime: 99.8%)
🟢 AI Engine: Healthy (uptime: 99.7%)
🟢 Database: Healthy (uptime: 99.9%)
🟢 Message Queue: Healthy (uptime: 99.6%)

Performance Metrics:
💻 CPU Usage: 45% (normal)
🧠 Memory Usage: 67% (normal)
🌐 Network I/O: 2.3 GB/s (normal)
💾 Disk Usage: 78% (normal)

AI Services Status:
🤖 Model Training: 2 jobs running
🔮 Inference Engine: 15 requests/second
🧠 Cognitive Engine: Active
🕸️ SlotGraph: Operational

💡 AI Recommendation: System performance optimal`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available system commands`;
  };

  const executeAutomationCommand = (cmd: string): string => {
    if (cmd.includes('help')) {
      return `🤖 Automation Engine - Available Commands:

🤖 Automate Commands:
  automate --task=<task> [--frequency=<interval>] [--ai-enhanced]
  automate --task=threat-hunting --frequency=hourly --ai-enhanced

📅 Schedule Commands:
  schedule --task=<task> --cron=<expression> [--enabled] [--validate]
  schedule --task=backup --cron="0 2 * * *"

⚡ Trigger Commands:
  trigger --event=<event> [--action=<action>] [--parameters=<params>]
  trigger --event=new-threat --action=deploy-response

📊 Monitor Commands:
  monitor --automation=<name> [--real-time] [--metrics=<types>]
  monitor --automation=all --real-time

⚙️ Optimize Commands:
  optimize --automation=<name> [--target=<metric>] [--ai-enhanced]
  optimize --automation=threat-response --target=efficiency`;
    }
    
    if (cmd.includes('automate')) {
      return `🤖 Setting up AI-Enhanced Automation...
🎯 Task: Threat Hunting
⏰ Frequency: Hourly execution
🤖 AI Enhancement: Enabled

Automation Configuration:
🔍 Hunting Scope: Network, endpoints, logs
🎯 Detection Methods: ML-based anomaly detection
⚡ Response Actions: Automated containment
📊 Reporting: Real-time dashboards

AI Capabilities:
🧠 Adaptive Learning: Threat pattern evolution
🎯 Predictive Analysis: Proactive threat detection
⚡ Intelligent Response: Context-aware actions
📈 Performance Optimization: Self-improving algorithms

Automation Status:
✅ Task scheduled: threat-hunting
✅ AI models loaded: 5 detection engines
✅ Response playbooks: 12 scenarios
✅ Monitoring active: Real-time alerts

💡 AI Insight: Automation will reduce response time by 67%`;
    }
    
    return `❌ Unknown command: ${cmd}
💡 Type 'help' for available automation commands`;
  };

  // Legacy command simulation functions (for backward compatibility)
  const _simulateCommandOutput = (command: string, environment: Environment): string => {
    const cmd = command.toLowerCase();
    
    switch (environment) {
      case 'ai-ml':
        return executeAIMLCommand(cmd);
      case 'cognitive':
        return executeCognitiveCommand(cmd);
      case 'ooda':
        return executeOODACommand(cmd);
      case 'slotgraph':
        return executeSlotGraphCommand(cmd);
      case 'workflow':
        return executeWorkflowCommand(cmd);
      case 'analytics':
        return executeAnalyticsCommand(cmd);
      case 'system':
        return executeSystemCommand(cmd);
      case 'automation':
        return executeAutomationCommand(cmd);
      default:
        return `Unknown environment: ${environment}`;
    }
  };

  const _simulateKaliCommand = (cmd: string): string => {
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
    
    if (cmd.includes('whoami')) return 'root';
    if (cmd.includes('pwd')) return '/root';
    if (cmd.includes('ls')) return `Desktop  Documents  Downloads  Pictures  Videos\ntools/  scripts/  reports/  logs/`;
    if (cmd.includes('help')) {
      return `Available commands:
- nmap: Network scanning
- whoami: Current user
- pwd: Current directory
- ls: List files
- clear: Clear terminal
- help: Show this help`;
    }
    
    return `Command '${cmd}' not found. Type 'help' for available commands.`;
  };

  const _simulateNetworkCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
status - Show network status
scan - Scan network
deploy - Deploy new instance
terminate - Terminate network
reset - Reset network configuration
help - Show this help message`;
    }
    if (cmd === 'status') return 'Network: 192.168.1.0/24\nStatus: Active\nHosts: 5\nThreats: 2';
    if (cmd === 'scan') return 'Scanning network...\nFound 5 active hosts\n2 potential threats detected';
    if (cmd.startsWith('deploy')) return 'Deploying new instance...\nInstance deployed successfully';
    if (cmd.startsWith('terminate')) return 'Terminating network...\nNetwork terminated';
    
    return `Unknown command: ${cmd}. Type 'help' for available commands.`;
  };

  const _simulateWSLCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
ls - List files
pwd - Current directory
whoami - Current user
ifconfig - Network interface
uname - System info
clear - Clear terminal
help - Show this help`;
    }
    if (cmd === 'ls') return 'Desktop  Documents  Downloads  Music  Pictures  Public  Templates  Videos';
    if (cmd === 'pwd') return '/home/kali';
    if (cmd === 'whoami') return 'kali';
    if (cmd === 'ifconfig') return 'eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500\n        inet 172.17.0.2  netmask 255.255.0.0  broadcast 172.17.255.255';
    if (cmd === 'uname') return 'Linux kali 5.15.90.1-microsoft-standard-WSL2 #1 SMP Fri Jan 27 02:56:13 UTC 2023 x86_64 GNU/Linux';
    
    return `Command '${cmd}' not found. Type 'help' for available commands.`;
  };

  const _simulateSystemCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
status - Show system status
deploy - Deploy services
monitor - Monitor system
logs - Show logs
config - Show configuration
switch <env> - Switch environment
clear - Clear terminal
help - Show this help`;
    }
    if (cmd === 'status') return 'CTAS System Status:\n- Frontend: Running\n- Backend: Running\n- Database: Connected\n- Services: 5/5 Active';
    if (cmd === 'deploy') return 'Deploying CTAS services...\nAll services deployed successfully';
    if (cmd === 'monitor') return 'System monitoring active\nCPU: 45%\nMemory: 67%\nNetwork: Normal';
    if (cmd === 'logs') return 'Recent logs:\n[INFO] System started\n[INFO] Database connected\n[WARN] High memory usage';
    if (cmd === 'config') return 'Configuration:\n- Demo Mode: Enabled\n- Environment: Development\n- Log Level: Info';
    if (cmd.startsWith('switch ')) {
      const env = cmd.split(' ')[1];
      if (environments[env as Environment]) {
        setCurrentEnvironment(env as Environment);
        return `Switched to ${environments[env as Environment].name} environment`;
      }
      return `Unknown environment: ${env}`;
    }
    
    return `Unknown command: ${cmd}. Type 'help' for available commands.`;
  };

  const _simulateRaptorCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
list - List RAPTOR stacks
deploy - Deploy new stack
scale - Scale stack
logs - Show stack logs
status - Show stack status
destroy - Destroy stack
help - Show this help`;
    }
    if (cmd === 'list') return 'RAPTOR Stacks:\n- stack-001: Running (3 instances)\n- stack-002: Stopped\n- stack-003: Deploying';
    if (cmd === 'status') return 'Stack Status:\n- Active: 2\n- Stopped: 1\n- Deploying: 1\n- Total: 4';
    if (cmd.startsWith('deploy')) return 'Deploying new RAPTOR stack...\nStack deployed successfully';
    if (cmd.startsWith('scale')) return 'Scaling stack...\nStack scaled to 5 instances';
    if (cmd.startsWith('destroy')) return 'Destroying stack...\nStack destroyed successfully';
    
    return `Unknown command: ${cmd}. Type 'help' for available commands.`;
  };

  const _simulateDVMCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
analyze - Analyze threat vectors
deploy - Deploy deception
monitor - Monitor deception
report - Generate report
config - Show configuration
help - Show this help`;
    }
    if (cmd === 'analyze') return 'Analyzing threat vectors...\nFound 3 potential vectors\nRisk level: Medium';
    if (cmd === 'deploy') return 'Deploying deception mechanisms...\nDeception deployed successfully';
    if (cmd === 'monitor') return 'Monitoring deception:\n- Honeypots: Active\n- Decoys: 5 deployed\n- Alerts: 2 triggered';
    if (cmd === 'report') return 'Generating DVM report...\nReport saved to /reports/dvm-report-2024-01-15.pdf';
    
    return `Unknown command: ${cmd}. Type 'help' for available commands.`;
  };

  const _simulateCognigraphCommand = (cmd: string): string => {
    if (cmd === 'help') {
      return `Available commands:
query - Query cognitive graph
analyze - Analyze relationships
visualize - Generate visualization
export - Export data
config - Show configuration
help - Show this help`;
    }
    if (cmd === 'query') return 'Querying cognitive graph...\nFound 1,247 nodes\nFound 3,891 relationships';
    if (cmd === 'analyze') return 'Analyzing relationships...\nStrongest connection: Threat-Actor -> Campaign\nWeakest connection: IP -> Domain';
    if (cmd === 'visualize') return 'Generating visualization...\nGraph exported to /exports/cognigraph-2024-01-15.png';
    if (cmd === 'export') return 'Exporting cognitive data...\nData exported to /exports/cognigraph-data.json';
    
    return `Unknown command: ${cmd}. Type 'help' for available commands.`;
  };

  const clearTerminal = () => {
    setCommandHistory([]);
  };

  const _copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const downloadLogs = () => {
    const logContent = commandHistory
      .map(cmd => `[${cmd.timestamp.toISOString()}] [${cmd.environment}] $ ${cmd.command}\n${cmd.output}`)
      .join('\n\n');
    
    const blob = new Blob([logContent], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `ctas-cli-${sessionId}-${Date.now()}.log`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !isRunning) {
      executeCommand(currentCommand);
    }
  };

  const switchEnvironment = (env: Environment) => {
    setCurrentEnvironment(env);
    setShowEnvironments(false);
    // Add environment switch message to history
    const switchMessage: CommandHistory = {
      id: Date.now().toString(),
      command: `switch ${env}`,
      output: `Switched to ${environments[env].name} environment`,
      timestamp: new Date(),
      status: 'success',
      environment: 'system'
    };
    setCommandHistory(prev => [...prev, switchMessage]);
  };

  return (
    <div className="bg-black text-green-400 font-mono text-sm rounded-lg overflow-hidden">
      {/* Terminal Header */}
      <div className="bg-gray-800 px-4 py-2 flex items-center justify-between">
        <div className="flex items-center">
          <Terminal size={16} className="mr-2" />
          <span className="text-white">CTAS AI CLI - {environments[currentEnvironment].name}</span>
          <span className="text-gray-400 ml-2">({sessionId})</span>
          <div className="ml-4 flex items-center space-x-2">
            <div className="flex items-center text-xs">
              <Brain size={12} className="mr-1 text-blue-400" />
              <span className="text-blue-400">AI Enhanced</span>
            </div>
            <div className="flex items-center text-xs">
              <Sparkles size={12} className="mr-1 text-purple-400" />
              <span className="text-purple-400">Warp-like</span>
            </div>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setShowPersonaSelector(!showPersonaSelector)}
            className="p-1 hover:bg-gray-700 rounded flex items-center"
            title="AI Persona"
          >
            <Brain size={14} className="mr-1" />
            <span className="text-xs">{currentPersona?.name || 'Persona'}</span>
          </button>
          <button
            onClick={() => setShowMCPServers(!showMCPServers)}
            className="p-1 hover:bg-gray-700 rounded flex items-center"
            title="MCP Servers"
          >
            <Code size={14} className="mr-1" />
            <span className="text-xs">MCP</span>
          </button>
          <button
            onClick={() => setShowAtomicClipboard(!showAtomicClipboard)}
            className={`p-1 rounded flex items-center ${
              clipboardConnected ? 'text-green-400 hover:bg-gray-700' : 'text-red-400 opacity-50'
            }`}
            title={`Atomic Clipboard ${clipboardConnected ? '(Connected)' : '(Disconnected)'}`}
          >
            <Clipboard size={14} className="mr-1" />
            <span className="text-xs">Clipboard</span>
          </button>
          <button
            onClick={() => setNaturalLanguageMode(!naturalLanguageMode)}
            className={`p-1 rounded flex items-center ${
              naturalLanguageMode ? 'bg-blue-600 text-white' : 'hover:bg-gray-700'
            }`}
            title="Natural Language Mode"
          >
            <Lightbulb size={14} className="mr-1" />
            <span className="text-xs">NL</span>
          </button>
          <button
            onClick={() => setShowCommandHistory(!showCommandHistory)}
            className="p-1 hover:bg-gray-700 rounded flex items-center"
            title="Command History"
          >
            <History size={14} className="mr-1" />
            <span className="text-xs">History</span>
          </button>
          <button
            onClick={() => setShowEnvironments(!showEnvironments)}
            className="p-1 hover:bg-gray-700 rounded flex items-center"
            title="Switch Environment"
          >
            <Settings size={14} className="mr-1" />
            <span className="text-xs">{environments[currentEnvironment].name}</span>
          </button>
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

      {/* Persona Selector */}
      {showPersonaSelector && (
        <div className="bg-gray-700 px-4 py-2 border-b border-gray-600">
          <div className="text-xs text-gray-300 mb-2">Select AI Persona:</div>
          <div className="grid grid-cols-2 gap-2">
            {personas.map((persona) => (
              <button
                key={persona.id}
                onClick={() => {
                  setCurrentPersona(persona);
                  setShowPersonaSelector(false);
                  if (aiContext) {
                    setAiContext(prev => prev ? {
                      ...prev,
                      currentPersona: persona
                    } : null);
                  }
                }}
                className={`p-2 rounded text-xs flex items-center ${
                  currentPersona?.id === persona.id 
                    ? 'bg-purple-600 text-white' 
                    : 'bg-gray-600 text-gray-300 hover:bg-gray-500'
                }`}
              >
                <Brain size={12} className="mr-1" />
                <div className="text-left">
                  <div className="font-semibold">{persona.name}</div>
                  <div className="text-xs opacity-75">{persona.role}</div>
                </div>
              </button>
            ))}
          </div>
        </div>
      )}

      {/* MCP Servers Panel */}
      {showMCPServers && (
        <div className="bg-gray-700 px-4 py-2 border-b border-gray-600">
          <div className="text-xs text-gray-300 mb-2">MCP Servers Status:</div>
          <div className="space-y-2">
            {mcpServers.map((server) => (
              <div key={server.id} className="flex items-center justify-between p-2 bg-gray-600 rounded">
                <div className="flex items-center">
                  <div className={`w-2 h-2 rounded-full mr-2 ${
                    server.status === 'connected' ? 'bg-green-400' :
                    server.status === 'error' ? 'bg-red-400' : 'bg-yellow-400'
                  }`} />
                  <div>
                    <div className="text-xs font-semibold">{server.name}</div>
                    <div className="text-xs opacity-75">{server.type}</div>
                  </div>
                </div>
                <div className="text-xs">
                  <div className={`${server.status === 'connected' ? 'text-green-400' : 'text-red-400'}`}>
                    {server.status}
                  </div>
                  <div className="opacity-75">{server.responseTime}ms</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Atomic Clipboard Panel */}
      {showAtomicClipboard && (
        <div className="bg-gray-700 px-4 py-2 border-b border-gray-600">
          <div className="flex items-center justify-between mb-2">
            <div className="text-xs text-gray-300">Atomic Clipboard:</div>
            <div className={`text-xs ${clipboardConnected ? 'text-green-400' : 'text-red-400'}`}>
              {clipboardConnected ? `✅ Connected (Port ${SYSTEM_PORTS.MEMORY_MESH.ATOMIC_CLIPBOARD})` : '❌ Disconnected'}
            </div>
          </div>
          {clipboardConnected ? (
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {atomicClipboard.length > 0 ? (
                atomicClipboard.map((item, index) => (
                  <div key={index} className="flex items-center justify-between p-2 bg-gray-600 rounded">
                    <div className="flex-1 truncate text-xs text-gray-300 mr-2">
                      {item.length > 60 ? `${item.substring(0, 60)}...` : item}
                    </div>
                    <div className="flex space-x-1">
                      <button
                        onClick={() => copyFromAtomicClipboard(index)}
                        className="p-1 hover:bg-gray-500 rounded"
                        title="Copy to command line"
                      >
                        <Copy size={12} />
                      </button>
                      <button
                        onClick={() => addToAtomicClipboard(item)}
                        className="p-1 hover:bg-gray-500 rounded text-blue-400"
                        title="Re-save with current persona"
                      >
                        <Brain size={12} />
                      </button>
                    </div>
                  </div>
                ))
              ) : (
                <div className="text-xs text-gray-400 text-center py-2">
                  No items in atomic clipboard
                </div>
              )}
              <div className="text-xs text-gray-400 text-center">
                💡 Commands are auto-saved to atomic clipboard
              </div>
            </div>
          ) : (
            <div className="text-xs text-gray-400 text-center py-2">
              Atomic clipboard service not available
            </div>
          )}
        </div>
      )}

      {/* Environment Selector */}
      {showEnvironments && (
        <div className="bg-gray-700 px-4 py-2 border-b border-gray-600">
          <div className="text-xs text-gray-300 mb-2">Select Environment:</div>
          <div className="grid grid-cols-2 gap-2">
            {Object.entries(environments).map(([key, env]) => (
              <button
                key={key}
                onClick={() => switchEnvironment(key as Environment)}
                className={`p-2 rounded text-xs flex items-center ${
                  currentEnvironment === key 
                    ? 'bg-blue-600 text-white' 
                    : 'bg-gray-600 text-gray-300 hover:bg-gray-500'
                }`}
              >
                {env.icon}
                <span className="ml-1">{env.name}</span>
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Terminal Output */}
      <div 
        ref={terminalRef}
        className="p-4 h-96 overflow-y-auto"
      >
        {commandHistory.map((cmd) => (
          <div key={cmd.id} className="mb-4">
            <div className="flex items-center text-yellow-400">
              <span className="mr-2">{environments[cmd.environment as Environment]?.prompt || 'ctas@system:~$'}</span>
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
            {cmd.aiSuggestion && (
              <div className="mt-1 text-blue-400 text-xs bg-blue-900/20 p-2 rounded border-l-2 border-blue-400">
                💡 AI Suggestion: {cmd.aiSuggestion}
              </div>
            )}
            {cmd.executionTime && (
              <div className="mt-1 text-gray-500 text-xs">
                ⏱️ Execution time: {cmd.executionTime}ms
                {currentPersona && (
                  <span className="ml-2">
                    🤖 {currentPersona.name} ({currentPersona.role})
                  </span>
                )}
              </div>
            )}
          </div>
        ))}
        
        {/* Welcome Message */}
        {commandHistory.length === 0 && (
          <div className="mb-4 text-blue-400">
            <div className="text-lg font-bold mb-2">🚀 Welcome to CTAS AI CLI</div>
            <div className="text-sm mb-4">
              🤖 AI-Powered Terminal with MCP Integration and Persona Framework
            </div>
            <div className="text-xs space-y-1">
              <div>🎯 Current Persona: <span className="text-yellow-400">{currentPersona?.name}</span> ({currentPersona?.role})</div>
              <div>🔗 MCP Servers: <span className="text-green-400">{mcpServers.filter(s => s.status === 'connected').length}/{mcpServers.length}</span> connected</div>
              <div>💬 Natural Language: <span className={naturalLanguageMode ? 'text-green-400' : 'text-gray-400'}>{naturalLanguageMode ? 'Enabled' : 'Disabled'}</span></div>
              <div className="mt-2">💡 Try: "help" or "What can you do?" for assistance</div>
            </div>
          </div>
        )}

        {/* Current Command Line */}
        <div className="flex items-center">
          <span className="mr-2 text-yellow-400">{environments[currentEnvironment].prompt}</span>
          <input
            ref={inputRef}
            type="text"
            value={currentCommand}
            onChange={(e) => setCurrentCommand(e.target.value)}
            onKeyPress={handleKeyPress}
            disabled={isRunning}
            className="flex-1 bg-transparent text-green-400 outline-none"
            placeholder={
              isRunning ? "Command running..." : 
              naturalLanguageMode ? "Ask me anything..." : 
              "Enter command..."
            }
          />
          {isRunning && <div className="ml-2 animate-pulse">...</div>}
          {naturalLanguageMode && (
            <div className="ml-2 text-blue-400 text-xs">
              💬 NL
            </div>
          )}
        </div>
      </div>

      {/* Quick Actions */}
      <div className="bg-gray-800 px-4 py-2 flex items-center space-x-2">
        <button
          onClick={() => executeCommand('help')}
          disabled={isRunning}
          className="px-2 py-1 bg-blue-600 text-white text-xs rounded hover:bg-blue-700 disabled:opacity-50"
        >
          Help
        </button>
        <button
          onClick={() => executeCommand('status')}
          disabled={isRunning}
          className="px-2 py-1 bg-green-600 text-white text-xs rounded hover:bg-green-700 disabled:opacity-50"
        >
          Status
        </button>
        <button
          onClick={() => executeCommand('clear')}
          disabled={isRunning}
          className="px-2 py-1 bg-purple-600 text-white text-xs rounded hover:bg-purple-700 disabled:opacity-50"
        >
          Clear
        </button>
        <div className="text-xs text-gray-400 ml-4">
          Environment: {environments[currentEnvironment].name}
        </div>
      </div>
    </div>
  );
};

export default CTASCLI;
