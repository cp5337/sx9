import { useState, useEffect } from 'react';
import {
  Cloud,
  Settings,
  Play,
  Square,
  RotateCcw,
  DollarSign,
  Activity,
  Server,
  Database,
  Globe,
  Shield,
  Zap,
  AlertTriangle,
  CheckCircle,
  Clock,
  TrendingUp,
  FileText,
  Layers,
  GitBranch,
  Terminal,
  Power
} from 'lucide-react';

interface TerraformState {
  status: 'applying' | 'destroying' | 'planning' | 'idle' | 'error';
  lastApply?: Date;
  resourceCount?: number;
  currentAction?: string;
  // RFC 3339 compliance for timestamps
  rfc3339_timestamp?: string;
  // QA5 DoD EA compliance metadata
  qa5_assessment?: {
    source_reliability: 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
    information_credibility: '1' | '2' | '3' | '4' | '5' | '6';
    overall_score: string;
    assessment_method: 'AUTOMATED' | 'MANUAL' | 'HYBRID';
  };
}

interface CloudRunService {
  name: string;
  status: 'SERVING' | 'STOPPED' | 'DEPLOYING' | 'ERROR';
  url: string;
  cpu: string;
  memory: string;
  instances: number;
  lastDeploy?: Date;
  // RFC 3339 compliant timestamp
  rfc3339_last_deploy?: string;
  // DoD EA compliance metadata
  ea_metadata?: {
    view_type: 'SV-1' | 'SV-2' | 'TV-1' | 'OV-1';
    classification: 'UNCLASSIFIED' | 'CONFIDENTIAL' | 'SECRET';
    security_controls: string[];
  };
}

// RFC-compliant API response structure
interface RFC_APIResponse<T = any> {
  status: 'success' | 'error' | 'pending';
  timestamp: string; // RFC 3339 format
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: any;
  };
  qa5_metadata?: {
    source_reliability: 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
    information_credibility: '1' | '2' | '3' | '4' | '5' | '6';
  };
}

interface CostMetrics {
  currentMonth: number;
  budgetLimit: number;
  projectedMonth: number;
  dailyAverage: number;
  services: Array<{
    name: string;
    cost: number;
    percentage: number;
  }>;
}

export default function ABE_IAC_ControlSurface() {
  const [terraformState, setTerraformState] = useState<TerraformState>({
    status: 'idle',
    resourceCount: 0
  });

  const [cloudRunServices, setCloudRunServices] = useState<CloudRunService[]>([]);
  const [costMetrics, setCostMetrics] = useState<CostMetrics>({
    currentMonth: 0,
    budgetLimit: 50,
    projectedMonth: 0,
    dailyAverage: 0,
    services: []
  });

  const [selectedOperation, setSelectedOperation] = useState<string | null>(null);
  const [showLogs, setShowLogs] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);

  // Initialize ABE infrastructure data
  useEffect(() => {
    const loadABEInfrastructure = async () => {
      try {
        // Simulated ABE infrastructure state based on actual Terraform configuration
        setCloudRunServices([
          {
            name: 'cognetix-abe-external-api-service',
            status: 'SERVING',
            url: 'https://cognetix-abe-external-api-service-gen-lang-client-0290627006.us-central1.run.app',
            cpu: '1000m',
            memory: '2Gi',
            instances: 2,
            lastDeploy: new Date('2024-11-22T12:45:00Z')
          },
          {
            name: 'cognetix-abe-ingestion-service',
            status: 'SERVING',
            url: 'https://cognetix-abe-ingestion-service-gen-lang-client-0290627006.us-central1.run.app',
            cpu: '2000m',
            memory: '4Gi',
            instances: 1,
            lastDeploy: new Date('2024-11-22T12:44:00Z')
          },
          {
            name: 'cognetix-abe-summarization-service',
            status: 'SERVING',
            url: 'https://cognetix-abe-summarization-service-gen-lang-client-0290627006.us-central1.run.app',
            cpu: '2000m',
            memory: '8Gi',
            instances: 3,
            lastDeploy: new Date('2024-11-22T12:46:00Z')
          }
        ]);

        setCostMetrics({
          currentMonth: 23.47,
          budgetLimit: 50,
          projectedMonth: 31.20,
          dailyAverage: 1.02,
          services: [
            { name: 'Cloud Run Services', cost: 15.30, percentage: 65.2 },
            { name: 'Cloud Storage', cost: 4.20, percentage: 17.9 },
            { name: 'Pub/Sub', cost: 2.10, percentage: 8.9 },
            { name: 'Load Balancer', cost: 1.87, percentage: 8.0 }
          ]
        });

        setTerraformState({
          status: 'idle',
          resourceCount: 23,
          lastApply: new Date('2024-11-22T12:46:15Z')
        });

      } catch (error) {
        console.error('Failed to load ABE infrastructure:', error);
      }
    };

    loadABEInfrastructure();

    // Refresh every 30 seconds
    const interval = setInterval(loadABEInfrastructure, 30000);
    return () => clearInterval(interval);
  }, []);

  const handleTerraformOperation = async (operation: 'plan' | 'apply' | 'destroy') => {
    setSelectedOperation(operation);
    setTerraformState(prev => ({
      ...prev,
      status: operation === 'plan' ? 'planning' : operation === 'apply' ? 'applying' : 'destroying',
      currentAction: `terraform ${operation}`
    }));

    const operationLogs = [
      `🚀 Starting terraform ${operation} for ABE infrastructure...`,
      `📍 Project: gen-lang-client-0290627006`,
      `📍 Location: us-central1`,
      `🔧 Terraform working directory: /cognetix-abe/`,
    ];

    if (operation === 'plan') {
      operationLogs.push(
        `📋 Planning changes to ABE infrastructure...`,
        `🔍 Checking current state against desired state...`,
        `✅ Plan completed: ${Math.floor(Math.random() * 5)} to add, 0 to change, 0 to destroy`
      );
    } else if (operation === 'apply') {
      operationLogs.push(
        `🔧 Applying Terraform configuration...`,
        `☁️ Updating Cloud Run services...`,
        `🗄️ Configuring Cloud Storage buckets...`,
        `📡 Setting up Pub/Sub topics...`,
        `🌐 Configuring Global Load Balancer...`,
        `✅ Apply completed successfully!`,
        `📊 Resources: ${terraformState.resourceCount} managed`
      );
    } else if (operation === 'destroy') {
      operationLogs.push(
        `⚠️ WARNING: This will destroy ABE infrastructure!`,
        `🗑️ Destroying Cloud Run services...`,
        `🗑️ Removing Cloud Storage buckets...`,
        `🗑️ Deleting Pub/Sub topics...`,
        `✅ Destroy completed`
      );
    }

    // Simulate operation progress
    for (let i = 0; i < operationLogs.length; i++) {
      setTimeout(() => {
        setLogs(prev => [...prev, operationLogs[i]]);
      }, i * 1000);
    }

    setTimeout(() => {
      setTerraformState(prev => ({
        ...prev,
        status: 'idle',
        currentAction: undefined,
        lastApply: operation === 'apply' ? new Date() : prev.lastApply
      }));
      setSelectedOperation(null);
    }, operationLogs.length * 1000);
  };

  const handleServiceControl = async (serviceName: string, action: 'start' | 'stop' | 'restart') => {
    console.log(`🎯 ${action.toUpperCase()} action for Cloud Run service: ${serviceName}`);

    setCloudRunServices(prev => prev.map(service =>
      service.name === serviceName
        ? {
            ...service,
            status: action === 'stop' ? 'STOPPED' : 'DEPLOYING',
            lastDeploy: new Date()
          }
        : service
    ));

    // Simulate Cloud Run operation
    setTimeout(() => {
      setCloudRunServices(prev => prev.map(service =>
        service.name === serviceName
          ? { ...service, status: action === 'stop' ? 'STOPPED' : 'SERVING' }
          : service
      ));
    }, 3000);
  };

  const getServiceStatusIcon = (status: string) => {
    switch (status) {
      case 'SERVING': return <CheckCircle className="w-4 h-4 text-green-400" />;
      case 'DEPLOYING': return <Clock className="w-4 h-4 text-yellow-400 animate-spin" />;
      case 'STOPPED': return <Square className="w-4 h-4 text-gray-400" />;
      case 'ERROR': return <AlertTriangle className="w-4 h-4 text-red-400" />;
      default: return <AlertTriangle className="w-4 h-4 text-orange-400" />;
    }
  };

  const getTerraformStatusColor = (status: string) => {
    switch (status) {
      case 'idle': return 'text-green-400';
      case 'planning': return 'text-blue-400';
      case 'applying': return 'text-yellow-400';
      case 'destroying': return 'text-red-400';
      case 'error': return 'text-red-400';
      default: return 'text-gray-400';
    }
  };

  return (
    <div className="bg-card border border-border rounded-lg p-6 shadow-lg">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 rounded-lg bg-gradient-to-r from-blue-500/20 to-cyan-500/20 border border-blue-500/30">
          <Cloud className="text-blue-400 w-6 h-6" />
        </div>
        <div className="flex-1">
          <h2 className="text-xl font-semibold text-card-foreground">ABE Infrastructure Control Surface</h2>
          <p className="text-sm text-muted-foreground">Infrastructure as Code management for ABE (Automated Business Environment)</p>
          <div className="flex items-center gap-4 mt-1 text-xs">
            <span className="flex items-center gap-1 text-blue-400">
              <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse"></div>
              Project: gen-lang-client-0290627006
            </span>
            <span className="flex items-center gap-1 text-cyan-400">
              <div className="w-2 h-2 bg-cyan-400 rounded-full animate-pulse"></div>
              Region: us-central1
            </span>
          </div>
        </div>
      </div>

      {/* Infrastructure Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div className="bg-background border border-border rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <Layers className="w-4 h-4 text-blue-400" />
            <span className="text-sm font-medium">Resources</span>
          </div>
          <div className="text-2xl font-bold text-blue-400">{terraformState.resourceCount}</div>
          <div className="text-xs text-muted-foreground">managed by Terraform</div>
        </div>

        <div className="bg-background border border-border rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <Server className="w-4 h-4 text-green-400" />
            <span className="text-sm font-medium">Services</span>
          </div>
          <div className="text-2xl font-bold text-green-400">
            {cloudRunServices.filter(s => s.status === 'SERVING').length}
          </div>
          <div className="text-xs text-muted-foreground">/ {cloudRunServices.length} Cloud Run</div>
        </div>

        <div className="bg-background border border-border rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <DollarSign className="w-4 h-4 text-yellow-400" />
            <span className="text-sm font-medium">Cost</span>
          </div>
          <div className="text-2xl font-bold text-yellow-400">${costMetrics.currentMonth.toFixed(2)}</div>
          <div className="text-xs text-muted-foreground">/ ${costMetrics.budgetLimit} budget</div>
        </div>

        <div className="bg-background border border-border rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <GitBranch className={`w-4 h-4 ${getTerraformStatusColor(terraformState.status)}`} />
            <span className="text-sm font-medium">Terraform</span>
          </div>
          <div className={`text-2xl font-bold ${getTerraformStatusColor(terraformState.status)}`}>
            {terraformState.status.toUpperCase()}
          </div>
          <div className="text-xs text-muted-foreground">
            {terraformState.lastApply ? `Last: ${terraformState.lastApply.toLocaleTimeString()}` : 'Never applied'}
          </div>
        </div>
      </div>

      {/* Terraform Operations */}
      <div className="bg-background border border-border rounded-lg p-4 mb-6">
        <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
          <div>
            <h3 className="font-semibold text-sm mb-1 flex items-center gap-2">
              <Terminal className="w-4 h-4" />
              Terraform Operations
            </h3>
            <p className="text-xs text-muted-foreground">
              {terraformState.status !== 'idle'
                ? `${terraformState.currentAction} in progress...`
                : 'Infrastructure as Code management for ABE platform'
              }
            </p>
          </div>

          <div className="flex gap-2">
            <button
              onClick={() => handleTerraformOperation('plan')}
              disabled={terraformState.status !== 'idle'}
              className="flex items-center gap-2 px-3 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-900 disabled:opacity-50 text-white text-sm rounded-lg transition-colors"
            >
              <FileText className="w-4 h-4" />
              Plan
            </button>

            <button
              onClick={() => handleTerraformOperation('apply')}
              disabled={terraformState.status !== 'idle'}
              className="flex items-center gap-2 px-3 py-2 bg-green-600 hover:bg-green-700 disabled:bg-green-900 disabled:opacity-50 text-white text-sm rounded-lg transition-colors"
            >
              <Play className="w-4 h-4" />
              Apply
            </button>

            <button
              onClick={() => handleTerraformOperation('destroy')}
              disabled={terraformState.status !== 'idle'}
              className="flex items-center gap-2 px-3 py-2 bg-red-600 hover:bg-red-700 disabled:bg-red-900 disabled:opacity-50 text-white text-sm rounded-lg transition-colors"
            >
              <Square className="w-4 h-4" />
              Destroy
            </button>

            <button
              onClick={() => setShowLogs(!showLogs)}
              className="flex items-center gap-2 px-3 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded-lg transition-colors"
            >
              <Terminal className="w-4 h-4" />
              Logs
            </button>
          </div>
        </div>
      </div>

      {/* Terraform Logs */}
      {showLogs && (
        <div className="bg-black border border-border rounded-lg p-4 mb-6 font-mono text-sm">
          <div className="flex items-center gap-2 mb-2 text-green-400">
            <Terminal className="w-4 h-4" />
            <span>Terraform Operations Log</span>
          </div>
          <div className="max-h-40 overflow-y-auto space-y-1">
            {logs.length === 0 ? (
              <div className="text-muted-foreground">No recent operations...</div>
            ) : (
              logs.map((log, index) => (
                <div key={index} className="text-green-300 text-xs">{log}</div>
              ))
            )}
          </div>
          <button
            onClick={() => setLogs([])}
            className="mt-2 text-xs text-muted-foreground hover:text-foreground"
          >
            Clear logs
          </button>
        </div>
      )}

      {/* Cloud Run Services */}
      <div className="bg-background border border-border rounded-lg p-4 mb-6">
        <h3 className="font-semibold text-sm mb-3 flex items-center gap-2">
          <Server className="w-4 h-4" />
          Cloud Run Services
        </h3>

        <div className="space-y-3">
          {cloudRunServices.map((service) => (
            <div key={service.name} className="bg-card border border-border rounded-lg p-3">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    {getServiceStatusIcon(service.status)}
                    <span className="font-medium text-sm">{service.name}</span>
                    <span className="text-xs bg-blue-500/20 text-blue-400 px-2 py-1 rounded">
                      {service.status}
                    </span>
                  </div>
                  <div className="flex items-center gap-4 text-xs text-muted-foreground">
                    <span>CPU: {service.cpu}</span>
                    <span>Memory: {service.memory}</span>
                    <span>Instances: {service.instances}</span>
                    {service.lastDeploy && (
                      <span>Last Deploy: {service.lastDeploy.toLocaleTimeString()}</span>
                    )}
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => handleServiceControl(service.name, 'start')}
                    disabled={service.status === 'SERVING' || service.status === 'DEPLOYING'}
                    className="p-2 hover:bg-green-500/20 text-green-400 rounded-lg transition-colors disabled:opacity-50"
                    title="Start Service"
                  >
                    <Play className="w-4 h-4" />
                  </button>

                  <button
                    onClick={() => handleServiceControl(service.name, 'restart')}
                    disabled={service.status === 'DEPLOYING'}
                    className="p-2 hover:bg-yellow-500/20 text-yellow-400 rounded-lg transition-colors disabled:opacity-50"
                    title="Restart Service"
                  >
                    <RotateCcw className="w-4 h-4" />
                  </button>

                  <button
                    onClick={() => handleServiceControl(service.name, 'stop')}
                    disabled={service.status === 'STOPPED' || service.status === 'DEPLOYING'}
                    className="p-2 hover:bg-red-500/20 text-red-400 rounded-lg transition-colors disabled:opacity-50"
                    title="Stop Service"
                  >
                    <Square className="w-4 h-4" />
                  </button>

                  <a
                    href={service.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="p-2 hover:bg-blue-500/20 text-blue-400 rounded-lg transition-colors"
                    title="Open Service URL"
                  >
                    <Globe className="w-4 h-4" />
                  </a>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Cost Monitoring */}
      <div className="bg-background border border-border rounded-lg p-4">
        <h3 className="font-semibold text-sm mb-3 flex items-center gap-2">
          <DollarSign className="w-4 h-4" />
          Cost Monitoring & Budgets
        </h3>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm">Monthly Usage</span>
              <span className="text-sm font-medium">${costMetrics.currentMonth.toFixed(2)} / ${costMetrics.budgetLimit}</span>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2 mb-3">
              <div
                className="bg-gradient-to-r from-green-400 to-yellow-400 h-2 rounded-full transition-all duration-300"
                style={{ width: `${Math.min((costMetrics.currentMonth / costMetrics.budgetLimit) * 100, 100)}%` }}
              ></div>
            </div>

            <div className="space-y-2 text-xs">
              <div className="flex justify-between">
                <span>Daily Average:</span>
                <span>${costMetrics.dailyAverage.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span>Projected Month:</span>
                <span className={costMetrics.projectedMonth > costMetrics.budgetLimit ? 'text-red-400' : 'text-green-400'}>
                  ${costMetrics.projectedMonth.toFixed(2)}
                </span>
              </div>
            </div>
          </div>

          <div>
            <div className="text-sm mb-2">Cost Breakdown</div>
            <div className="space-y-2">
              {costMetrics.services.map((service, index) => (
                <div key={index} className="flex items-center justify-between text-xs">
                  <span>{service.name}</span>
                  <div className="flex items-center gap-2">
                    <span>${service.cost.toFixed(2)}</span>
                    <span className="text-muted-foreground">({service.percentage.toFixed(1)}%)</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}