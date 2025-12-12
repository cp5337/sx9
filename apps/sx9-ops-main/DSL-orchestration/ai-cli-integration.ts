// CTAS AI-CLI Integration
// Integrates AI-CLI with QA5 operational intelligence system

import React, { useState, useEffect } from 'react';
import { CTASPortIntegration } from './frontend-integration-system';

// AI-CLI Command Types
export interface AICLICommand {
  command: string;
  args: string[];
  description: string;
  category: AICLICategory;
  requiresAuth: boolean;
}

export enum AICLICategory {
  PortManagement = 'port_management',
  OperationalIntelligence = 'operational_intelligence',
  GroupOperations = 'group_operations',
  CrateInterview = 'crate_interview',
  LispRdfIntegration = 'lisp_rdf_integration',
  SystemHealth = 'system_health',
}

// AI-CLI Service Interface
export interface AICLIService {
  serviceName: string;
  port: number;
  status: 'active' | 'inactive' | 'failed';
  lastHeartbeat: Date;
  capabilities: string[];
}

// AI-CLI Integration Component
export const AICLIIntegration: React.FC = () => {
  const [portIntegration] = useState(() => new CTASPortIntegration());
  const [isConnected, setIsConnected] = useState(false);
  const [commands, setCommands] = useState<AICLICommand[]>([]);
  const [services, setServices] = useState<AICLIService[]>([]);
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [currentCommand, setCurrentCommand] = useState('');
  const [executing, setExecuting] = useState(false);
  const [lastResult, setLastResult] = useState<any>(null);

  useEffect(() => {
    initializeAICLI();
    loadCommands();
    loadServices();
  }, []);

  const initializeAICLI = async () => {
    try {
      await portIntegration.connect();
      setIsConnected(true);
      console.log('🔗 AI-CLI connected to port management system');
    } catch (error) {
      console.error('❌ Failed to connect AI-CLI:', error);
      setIsConnected(false);
    }
  };

  const loadCommands = () => {
    const aiCLICommands: AICLICommand[] = [
      // Port Management Commands
      {
        command: 'port-status',
        args: ['block_name'],
        description: 'Check status of ports in a block',
        category: AICLICategory.PortManagement,
        requiresAuth: false,
      },
      {
        command: 'port-allocate',
        args: ['block_name', 'service_name'],
        description: 'Allocate a port for a service',
        category: AICLICategory.PortManagement,
        requiresAuth: true,
      },
      {
        command: 'port-release',
        args: ['port_number'],
        description: 'Release a port',
        category: AICLICategory.PortManagement,
        requiresAuth: true,
      },

      // Operational Intelligence Commands
      {
        command: 'threat-emulation',
        args: ['scenario_name'],
        description: 'Execute threat emulation scenario',
        category: AICLICategory.OperationalIntelligence,
        requiresAuth: true,
      },
      {
        command: 'intelligence-fusion',
        args: ['source1', 'source2'],
        description: 'Fuse intelligence from multiple sources',
        category: AICLICategory.OperationalIntelligence,
        requiresAuth: true,
      },
      {
        command: 'countermeasures',
        args: ['threat_type'],
        description: 'Deploy countermeasures for threat type',
        category: AICLICategory.OperationalIntelligence,
        requiresAuth: true,
      },

      // Group Operations Commands
      {
        command: 'group-check',
        args: ['group_name'],
        description: 'Run cargo check on a group',
        category: AICLICategory.GroupOperations,
        requiresAuth: false,
      },
      {
        command: 'group-build',
        args: ['group_name'],
        description: 'Build a group of crates',
        category: AICLICategory.GroupOperations,
        requiresAuth: false,
      },
      {
        command: 'group-test',
        args: ['group_name'],
        description: 'Test a group of crates',
        category: AICLICategory.GroupOperations,
        requiresAuth: false,
      },

      // Crate Interview Commands
      {
        command: 'crate-interview',
        args: ['crate_name'],
        description: 'Interview a specific crate',
        category: AICLICategory.CrateInterview,
        requiresAuth: false,
      },
      {
        command: 'crate-analyze',
        args: ['crate_name'],
        description: 'Analyze crate capabilities',
        category: AICLICategory.CrateInterview,
        requiresAuth: false,
      },

      // LISP-RDF Integration Commands
      {
        command: 'lisp-evaluate',
        args: ['expression'],
        description: 'Evaluate LISP expression',
        category: AICLICategory.LispRdfIntegration,
        requiresAuth: false,
      },
      {
        command: 'rdf-extract',
        args: ['document_path'],
        description: 'Extract RDF triples from document',
        category: AICLICategory.LispRdfIntegration,
        requiresAuth: false,
      },

      // System Health Commands
      {
        command: 'health-check',
        args: [],
        description: 'Check system health',
        category: AICLICategory.SystemHealth,
        requiresAuth: false,
      },
      {
        command: 'status-report',
        args: [],
        description: 'Generate status report',
        category: AICLICategory.SystemHealth,
        requiresAuth: false,
      },
    ];

    setCommands(aiCLICommands);
  };

  const loadServices = () => {
    const aiCLIServices: AICLIService[] = [
      {
        serviceName: 'ai-cli-core',
        port: 17173, // BLOCK-B: AI/ML Services
        status: 'active',
        lastHeartbeat: new Date(),
        capabilities: ['command_execution', 'port_management', 'operational_intelligence'],
      },
      {
        serviceName: 'ai-cli-port-manager',
        port: 17174,
        status: 'active',
        lastHeartbeat: new Date(),
        capabilities: ['port_allocation', 'port_monitoring', 'failover_management'],
      },
      {
        serviceName: 'ai-cli-operational',
        port: 17175,
        status: 'active',
        lastHeartbeat: new Date(),
        capabilities: ['threat_emulation', 'intelligence_fusion', 'countermeasures'],
      },
    ];

    setServices(aiCLIServices);
  };

  const executeCommand = async (command: string, args: string[]) => {
    if (!isConnected) {
      setLastResult({ error: 'AI-CLI not connected to port management system' });
      return;
    }

    setExecuting(true);
    setCommandHistory(prev => [...prev, `${command} ${args.join(' ')}`]);

    try {
      let result;
      
      // Route command to appropriate service based on category
      const cmd = commands.find(c => c.command === command);
      if (!cmd) {
        throw new Error(`Unknown command: ${command}`);
      }

      switch (cmd.category) {
        case AICLICategory.PortManagement:
          result = await portIntegration.send('ai_cli_port_management', {
            command,
            args,
            block_name: args[0],
            operation: command,
          });
          break;

        case AICLICategory.OperationalIntelligence:
          result = await portIntegration.send('ai_cli_operational_command', {
            command,
            args,
            operational_command: command,
            parameters: { args },
          });
          break;

        case AICLICategory.GroupOperations:
          result = await portIntegration.send('execute_group_operation', {
            group_id: args[0],
            operation: command.replace('group-', ''),
            parallel: true,
          });
          break;

        case AICLICategory.CrateInterview:
          result = await portIntegration.send('execute_crate_interview', {
            crate_name: args[0],
          });
          break;

        case AICLICategory.LispRdfIntegration:
          result = await portIntegration.send('execute_lisp_rdf_integration', {
            command,
            args,
            expression: args[0],
          });
          break;

        case AICLICategory.SystemHealth:
          result = await portIntegration.send('health_check', {
            command,
            args,
          });
          break;

        default:
          result = await portIntegration.send('ai_cli_command', {
            command,
            args,
          });
      }

      setLastResult(result);
    } catch (error) {
      setLastResult({ error: error instanceof Error ? error.message : 'Unknown error' });
    } finally {
      setExecuting(false);
    }
  };

  const handleCommandSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentCommand.trim()) return;

    const parts = currentCommand.trim().split(' ');
    const command = parts[0];
    const args = parts.slice(1);

    executeCommand(command, args);
    setCurrentCommand('');
  };

  const getCommandsByCategory = (category: AICLICategory) => {
    return commands.filter(cmd => cmd.category === category);
  };

  return (
    <div className="ai-cli-integration">
      <header className="cli-header">
        <h1>🤖 CTAS AI-CLI Integration</h1>
        <div className="connection-status">
          <span className={`status-indicator ${isConnected ? 'connected' : 'disconnected'}`}>
            {isConnected ? '🟢 Connected' : '🔴 Disconnected'}
          </span>
        </div>
      </header>

      <div className="cli-content">
        <div className="cli-terminal">
          <div className="terminal-header">
            <h2>💻 AI-CLI Terminal</h2>
            <div className="terminal-controls">
              <button onClick={() => setCommandHistory([])}>🗑️ Clear</button>
              <button onClick={() => executeCommand('health-check', [])}>🏥 Health</button>
            </div>
          </div>

          <div className="command-history">
            {commandHistory.map((cmd, index) => (
              <div key={index} className="history-entry">
                <span className="prompt">$</span>
                <span className="command">{cmd}</span>
              </div>
            ))}
          </div>

          <form onSubmit={handleCommandSubmit} className="command-input">
            <span className="prompt">$</span>
            <input
              type="text"
              value={currentCommand}
              onChange={(e) => setCurrentCommand(e.target.value)}
              placeholder="Enter AI-CLI command..."
              disabled={executing}
              className="command-field"
            />
            <button type="submit" disabled={executing || !isConnected}>
              {executing ? '⏳' : '▶️'}
            </button>
          </form>

          {lastResult && (
            <div className="command-result">
              <h3>📋 Last Result:</h3>
              <pre>{JSON.stringify(lastResult, null, 2)}</pre>
            </div>
          )}
        </div>

        <div className="cli-sidebar">
          <div className="services-panel">
            <h3>🔧 AI-CLI Services</h3>
            {services.map((service, index) => (
              <div key={index} className={`service-item ${service.status}`}>
                <div className="service-info">
                  <span className="service-name">{service.serviceName}</span>
                  <span className="service-port">:{service.port}</span>
                </div>
                <div className="service-status">
                  <span className={`status ${service.status}`}>
                    {service.status === 'active' ? '🟢' : service.status === 'inactive' ? '🟡' : '🔴'}
                  </span>
                </div>
              </div>
            ))}
          </div>

          <div className="commands-panel">
            <h3>📚 Available Commands</h3>
            
            <div className="command-category">
              <h4>🔌 Port Management</h4>
              {getCommandsByCategory(AICLICategory.PortManagement).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>

            <div className="command-category">
              <h4>🎯 Operational Intelligence</h4>
              {getCommandsByCategory(AICLICategory.OperationalIntelligence).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>

            <div className="command-category">
              <h4>🏗️ Group Operations</h4>
              {getCommandsByCategory(AICLICategory.GroupOperations).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>

            <div className="command-category">
              <h4>🔍 Crate Interview</h4>
              {getCommandsByCategory(AICLICategory.CrateInterview).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>

            <div className="command-category">
              <h4>🧠 LISP-RDF Integration</h4>
              {getCommandsByCategory(AICLICategory.LispRdfIntegration).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>

            <div className="command-category">
              <h4>🏥 System Health</h4>
              {getCommandsByCategory(AICLICategory.SystemHealth).map((cmd, index) => (
                <div key={index} className="command-item" onClick={() => setCurrentCommand(`${cmd.command} `)}>
                  <span className="command-name">{cmd.command}</span>
                  <span className="command-desc">{cmd.description}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AICLIIntegration;
