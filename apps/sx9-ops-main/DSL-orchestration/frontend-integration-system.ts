// CTAS Frontend Integration System
// Connects backend group operations to frontend component registry

import React, { useState, useEffect } from 'react';
import { CrateGroup, OperationalIntelligenceMapping, ComponentMapping } from './frontend-outputs/crate-grouping-types';

// Port Management Integration - Uses existing ctas-port-manager system
class CTASPortIntegration {
  private portManagerUrl: string;
  private listeners: Map<string, Function[]> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private isConnected = false;

  constructor(private baseUrl: string = 'http://localhost:8080') {
    this.portManagerUrl = `${baseUrl}/ctas-port-manager`;
  }

  async connect() {
    try {
      // Test connection to existing port manager
      const response = await fetch(`${this.portManagerUrl}/health`);
      if (response.ok) {
        console.log('🔗 CTAS Port Manager connected');
        this.isConnected = true;
        this.reconnectAttempts = 0;
      } else {
        throw new Error('Port manager health check failed');
      }
    } catch (error) {
      console.error('❌ Failed to connect to CTAS Port Manager:', error);
      this.attemptReconnect();
    }
  }

  private attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`🔄 Attempting to reconnect to port manager (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      setTimeout(() => this.connect(), 5000);
    }
  }

  subscribe(eventType: string, callback: Function) {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, []);
    }
    this.listeners.get(eventType)!.push(callback);
  }

  private notifyListeners(eventType: string, payload: any) {
    const callbacks = this.listeners.get(eventType);
    if (callbacks) {
      callbacks.forEach(callback => callback(payload));
    }
  }

  async send(type: string, payload: any): Promise<any> {
    if (!this.isConnected) {
      throw new Error('Not connected to port manager');
    }

    try {
      const response = await fetch(`${this.portManagerUrl}/qa5/${type}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`Port manager request failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('❌ Port manager request failed:', error);
      throw error;
    }
  }

  disconnect() {
    this.isConnected = false;
    console.log('🔌 CTAS Port Manager disconnected');
  }

  isConnectedToPortManager(): boolean {
    return this.isConnected;
  }
}

// Operational Intelligence Service - Uses existing port management
class OperationalIntelligenceService {
  private portIntegration: CTASPortIntegration;

  constructor() {
    this.portIntegration = new CTASPortIntegration();
    this.portIntegration.connect();
  }

  // Get real-time group status
  async getGroupStatus(groupId: string): Promise<any> {
    return await this.portIntegration.send('get_group_status', { groupId });
  }

  // Execute group operation
  async executeGroupOperation(groupId: string, operation: string, parallel: boolean = false): Promise<any> {
    return await this.portIntegration.send('execute_group_operation', { groupId, operation, parallel });
  }

  // Get operational intelligence mapping
  async getOperationalIntelligenceMapping(): Promise<OperationalIntelligenceMapping> {
    return await this.portIntegration.send('get_operational_intelligence_mapping', {});
  }

  // Subscribe to real-time updates
  subscribeToUpdates(callback: (update: any) => void) {
    this.portIntegration.subscribe('real_time_update', callback);
  }
}

// Group Operations Hook
export const useGroupOperations = () => {
  const [groups, setGroups] = useState<Record<string, CrateGroup>>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [operationalIntelligence, setOperationalIntelligence] = useState<OperationalIntelligenceMapping | null>(null);

  const opsService = new OperationalIntelligenceService();

  useEffect(() => {
    // Load initial data
    loadGroups();
    loadOperationalIntelligence();

    // Subscribe to real-time updates
    opsService.subscribeToUpdates((update: any) => {
      console.log('🔄 Real-time update received:', update);
      // Update groups based on the update
      if (update.type === 'group_status_update') {
        setGroups(prev => ({
          ...prev,
          [update.groupId]: update.group
        }));
      }
    });

    return () => {
      opsService.portIntegration.disconnect();
    };
  }, []);

  const loadGroups = async () => {
    try {
      setLoading(true);
      // Simulate loading groups from backend
      const mockGroups: Record<string, CrateGroup> = {
        foundation: {
          groupId: 'foundation',
          groupName: 'Foundation',
          groupType: 'core_infrastructure',
          description: 'Core infrastructure and foundation crates',
          crateCount: 12,
          crates: ['ctas-core', 'ctas-tie', 'ctas-port-manager'],
          operationalCapabilities: ['system_foundation', 'core_infrastructure']
        },
        intelligence: {
          groupId: 'intelligence',
          groupName: 'Intelligence',
          groupType: 'intelligence_processing',
          description: 'Intelligence and analysis crates',
          crateCount: 13,
          crates: ['ctas-intelligence-hub', 'ctas-gnn-standalone'],
          operationalCapabilities: ['intelligence_processing', 'threat_intelligence']
        }
      };
      setGroups(mockGroups);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load groups');
    } finally {
      setLoading(false);
    }
  };

  const loadOperationalIntelligence = async () => {
    try {
      const mapping = await opsService.getOperationalIntelligenceMapping();
      setOperationalIntelligence(mapping);
    } catch (err) {
      console.error('Failed to load operational intelligence mapping:', err);
    }
  };

  const executeOperation = async (groupId: string, operation: string, parallel: boolean = false) => {
    try {
      setLoading(true);
      const result = await opsService.executeGroupOperation(groupId, operation, parallel);
      return result;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Operation failed');
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return {
    groups,
    loading,
    error,
    operationalIntelligence,
    executeOperation
  };
};

// QA5 Master Dashboard Component
export const QA5MasterDashboard: React.FC = () => {
  const { groups, loading, error, operationalIntelligence, executeOperation } = useGroupOperations();
  const [selectedGroup, setSelectedGroup] = useState<string | null>(null);
  const [operationHistory, setOperationHistory] = useState<any[]>([]);

  const handleExecuteOperation = async (groupId: string, operation: string) => {
    try {
      const result = await executeOperation(groupId, operation, true);
      setOperationHistory(prev => [...prev, { timestamp: new Date(), groupId, operation, result }]);
    } catch (err) {
      console.error('Operation failed:', err);
    }
  };

  if (loading) {
    return <div className="qa5-dashboard loading">🔄 Loading QA5 Master Dashboard...</div>;
  }

  if (error) {
    return <div className="qa5-dashboard error">❌ Error: {error}</div>;
  }

  return (
    <div className="qa5-master-dashboard">
      <header className="dashboard-header">
        <h1>🎯 CTAS QA5 Master Dashboard</h1>
        <div className="dashboard-status">
          <span className="status-indicator online">🟢 Online</span>
          <span className="real-time-indicator">📡 Real-time</span>
        </div>
      </header>

      <div className="dashboard-content">
        <div className="groups-overview">
          <h2>📊 Groups Overview</h2>
          <div className="groups-grid">
            {Object.entries(groups).map(([groupId, group]) => (
              <div 
                key={groupId} 
                className={`group-card ${selectedGroup === groupId ? 'selected' : ''}`}
                onClick={() => setSelectedGroup(groupId)}
              >
                <h3>{group.groupName}</h3>
                <div className="group-stats">
                  <span className="crate-count">{group.crateCount} crates</span>
                  <span className="group-type">{group.groupType}</span>
                </div>
                <div className="group-capabilities">
                  {group.operationalCapabilities.slice(0, 3).map((cap, index) => (
                    <span key={index} className="capability-tag">{cap}</span>
                  ))}
                </div>
                <div className="group-actions">
                  <button onClick={(e) => { e.stopPropagation(); handleExecuteOperation(groupId, 'check'); }}>
                    🔍 Check
                  </button>
                  <button onClick={(e) => { e.stopPropagation(); handleExecuteOperation(groupId, 'build'); }}>
                    🏗️ Build
                  </button>
                  <button onClick={(e) => { e.stopPropagation(); handleExecuteOperation(groupId, 'test'); }}>
                    🧪 Test
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>

        {selectedGroup && groups[selectedGroup] && (
          <div className="group-details">
            <h2>📋 {groups[selectedGroup].groupName} Details</h2>
            <div className="group-info">
              <div className="crates-list">
                <h3>Crates:</h3>
                <ul>
                  {groups[selectedGroup].crates.map((crate, index) => (
                    <li key={index}>{crate}</li>
                  ))}
                </ul>
              </div>
              <div className="capabilities-list">
                <h3>Operational Capabilities:</h3>
                <ul>
                  {groups[selectedGroup].operationalCapabilities.map((cap, index) => (
                    <li key={index}>{cap}</li>
                  ))}
                </ul>
              </div>
            </div>
          </div>
        )}

        {operationalIntelligence && (
          <div className="operational-intelligence">
            <h2>🎯 Operational Intelligence</h2>
            <div className="intelligence-grid">
              {Object.entries(operationalIntelligence).map(([key, capability]) => (
                <div key={key} className="intelligence-card">
                  <h3>{key.replace(/([A-Z])/g, ' $1').trim()}</h3>
                  <p>{capability.description}</p>
                  <div className="capability-score">
                    Score: {capability.capabilityScore}/100
                  </div>
                  <div className="group-mapping">
                    Groups: {capability.groupMapping.join(', ')}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="operation-history">
          <h2>📜 Operation History</h2>
          <div className="history-list">
            {operationHistory.slice(-10).reverse().map((op, index) => (
              <div key={index} className="history-item">
                <span className="timestamp">{op.timestamp.toLocaleTimeString()}</span>
                <span className="operation">{op.groupId} - {op.operation}</span>
                <span className="status">{op.result?.success ? '✅' : '❌'}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

// Operational Intelligence Dashboard Component
export const OperationalIntelligenceDashboard: React.FC = () => {
  const { operationalIntelligence, loading } = useGroupOperations();

  if (loading) {
    return <div className="operational-dashboard loading">🔄 Loading Operational Intelligence...</div>;
  }

  return (
    <div className="operational-intelligence-dashboard">
      <header className="dashboard-header">
        <h1>🎯 Operational Intelligence Dashboard</h1>
        <div className="dashboard-status">
          <span className="status-indicator active">🟢 Active</span>
          <span className="threat-level">⚠️ Threat Level: Medium</span>
        </div>
      </header>

      <div className="dashboard-content">
        {operationalIntelligence && (
          <div className="intelligence-overview">
            <div className="threat-emulation">
              <h2>🎭 Threat Emulation</h2>
              <div className="capability-status">
                <span className="status">Status: Active</span>
                <span className="score">Score: {operationalIntelligence.threatEmulation.capabilityScore}/100</span>
              </div>
              <p>{operationalIntelligence.threatEmulation.description}</p>
            </div>

            <div className="intelligence-fusion">
              <h2>🧠 Intelligence Fusion</h2>
              <div className="capability-status">
                <span className="status">Status: Processing</span>
                <span className="score">Score: {operationalIntelligence.intelligenceFusion.capabilityScore}/100</span>
              </div>
              <p>{operationalIntelligence.intelligenceFusion.description}</p>
            </div>

            <div className="countermeasures">
              <h2>🛡️ Countermeasures</h2>
              <div className="capability-status">
                <span className="status">Status: Ready</span>
                <span className="score">Score: {operationalIntelligence.countermeasures.capabilityScore}/100</span>
              </div>
              <p>{operationalIntelligence.countermeasures.description}</p>
            </div>

            <div className="forensics">
              <h2>🔍 Forensics</h2>
              <div className="capability-status">
                <span className="status">Status: Available</span>
                <span className="score">Score: {operationalIntelligence.forensics.capabilityScore}/100</span>
              </div>
              <p>{operationalIntelligence.forensics.description}</p>
            </div>

            <div className="investigation">
              <h2>🕵️ Investigation</h2>
              <div className="capability-status">
                <span className="status">Status: Active</span>
                <span className="score">Score: {operationalIntelligence.investigation.capabilityScore}/100</span>
              </div>
              <p>{operationalIntelligence.investigation.description}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

// XSD Playbook Editor Component
export const XSDPlaybookEditor: React.FC = () => {
  const [playbooks, setPlaybooks] = useState<string[]>([]);
  const [selectedPlaybook, setSelectedPlaybook] = useState<string | null>(null);
  const [playbookContent, setPlaybookContent] = useState<string>('');

  useEffect(() => {
    // Load available playbooks
    setPlaybooks([
      'lisp-rdf-integration-playbook.xsd',
      'crate-interview-playbook.xsd',
      'xsd-crate-grouping-system.xsd'
    ]);
  }, []);

  const handlePlaybookSelect = (playbook: string) => {
    setSelectedPlaybook(playbook);
    // Load playbook content (simulated)
    setPlaybookContent(`<!-- ${playbook} content would be loaded here -->`);
  };

  const handleSavePlaybook = () => {
    if (selectedPlaybook) {
      console.log('💾 Saving playbook:', selectedPlaybook);
      // Save playbook logic would go here
    }
  };

  return (
    <div className="xsd-playbook-editor">
      <header className="editor-header">
        <h1>📝 XSD Playbook Editor</h1>
        <div className="editor-actions">
          <button onClick={handleSavePlaybook}>💾 Save</button>
          <button>🔄 Validate</button>
          <button>▶️ Execute</button>
        </div>
      </header>

      <div className="editor-content">
        <div className="playbook-list">
          <h2>📚 Available Playbooks</h2>
          <ul>
            {playbooks.map((playbook) => (
              <li 
                key={playbook}
                className={selectedPlaybook === playbook ? 'selected' : ''}
                onClick={() => handlePlaybookSelect(playbook)}
              >
                {playbook}
              </li>
            ))}
          </ul>
        </div>

        <div className="playbook-editor">
          <h2>✏️ Playbook Editor</h2>
          {selectedPlaybook ? (
            <div className="editor-panel">
              <div className="editor-toolbar">
                <span className="file-name">{selectedPlaybook}</span>
                <span className="file-status">🟢 Saved</span>
              </div>
              <textarea
                value={playbookContent}
                onChange={(e) => setPlaybookContent(e.target.value)}
                placeholder="XSD playbook content..."
                className="playbook-textarea"
              />
            </div>
          ) : (
            <div className="no-selection">
              <p>Select a playbook to edit</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

// Export the integration system
export const CTASFrontendIntegration = {
  QA5MasterDashboard,
  OperationalIntelligenceDashboard,
  XSDPlaybookEditor,
  useGroupOperations
};

export default CTASFrontendIntegration;
