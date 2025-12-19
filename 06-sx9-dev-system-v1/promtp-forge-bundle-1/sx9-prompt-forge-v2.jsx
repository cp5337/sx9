import React, { useState, useCallback, useEffect } from 'react';
import { Folder, File, ChevronRight, ChevronDown, X, Check, RefreshCw, Search } from 'lucide-react';

// ═══════════════════════════════════════════════════════════════════════════════
// PROMPT TYPE PRESETS - These drive the entire form configuration
// ═══════════════════════════════════════════════════════════════════════════════

const PROMPT_TYPES = {
  BUILD_PIPELINE: {
    name: 'Build Pipeline',
    icon: '🔧',
    description: 'CI/CD, compilation, packaging tasks',
    defaults: {
      phase: 'IMPLEMENT',
      persona: 'FORGE - Build Engineer',
      priority: 'P1',
      mode: 'SUPERVISED',
      temp: '0.1',
      timeout: '60m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: true, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: '.env, secrets/, .git/config, credentials/',
      hardConstraints: [
        'DO NOT modify source files in src/',
        'DO NOT delete any existing files',
        'DO NOT push to remote without explicit approval'
      ],
      softConstraints: [
        'Prefer Makefile/justfile over custom scripts',
        'Document all build steps in BUILD_NOTES.md'
      ],
      role: 'You are FORGE, a senior build/infrastructure engineer for Synaptix9. You create reliable, reproducible build pipelines with minimal dependencies and clear documentation.'
    }
  },
  
  SECURITY_AUDIT: {
    name: 'Security Audit',
    icon: '🛡️',
    description: 'Vulnerability scanning, code review, threat analysis',
    defaults: {
      phase: 'ANALYZE',
      persona: 'VECTOR - Security Engineer',
      priority: 'P0',
      mode: 'STEP-CONFIRM',
      temp: '0.1',
      timeout: '120m',
      tools: {
        fsRead: true, fsWrite: false, fsDelete: false,
        shell: true, gitRead: true, gitCommit: false, gitPush: false,
        network: false, dbRead: true, dbWrite: false, webSearch: true
      },
      forbidPaths: '.env, secrets/, private/, keys/',
      hardConstraints: [
        'DO NOT modify any files - read-only audit',
        'DO NOT exfiltrate or log sensitive data',
        'DO NOT execute untrusted code',
        'HALT on any credential exposure'
      ],
      softConstraints: [
        'Flag all findings with severity levels',
        'Cross-reference CVE databases when possible'
      ],
      role: 'You are VECTOR, a security engineer conducting a thorough audit. You identify vulnerabilities, misconfigurations, and potential attack vectors with precision. You never modify files during audits.'
    }
  },
  
  RESEARCH: {
    name: 'Research & Analysis',
    icon: '🔬',
    description: 'Investigation, documentation, knowledge gathering',
    defaults: {
      phase: 'RESEARCH',
      persona: 'SENTINEL - Threat Analyst',
      priority: 'P2',
      mode: 'AUTONOMOUS',
      temp: '0.4',
      timeout: '45m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: false, gitRead: true, gitCommit: false, gitPush: false,
        network: true, dbRead: true, dbWrite: false, webSearch: true
      },
      forbidPaths: '.env, secrets/',
      hardConstraints: [
        'DO NOT modify existing documentation',
        'DO NOT execute arbitrary code',
        'Cite all sources'
      ],
      softConstraints: [
        'Prefer primary sources over aggregators',
        'Include confidence levels on findings'
      ],
      role: 'You are SENTINEL, a research analyst gathering intelligence and synthesizing findings. You prioritize accuracy, cite sources, and clearly distinguish facts from analysis.'
    }
  },
  
  CODE_GENERATION: {
    name: 'Code Generation',
    icon: '💻',
    description: 'New features, modules, implementations',
    defaults: {
      phase: 'IMPLEMENT',
      persona: 'AXIOM - Systems Architect',
      priority: 'P2',
      mode: 'SUPERVISED',
      temp: '0.2',
      timeout: '90m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: true, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: '.env, secrets/, .git/',
      hardConstraints: [
        'DO NOT create files larger than 300 lines',
        'DO NOT modify files outside designated module',
        'All new code must include tests'
      ],
      softConstraints: [
        'Follow existing code conventions',
        'Prefer composition over inheritance',
        'Include docstrings/comments for public APIs'
      ],
      role: 'You are AXIOM, a systems architect implementing clean, modular code. You write concise modules under 300 lines, include tests, and document public interfaces.'
    }
  },
  
  REFACTOR: {
    name: 'Refactoring',
    icon: '♻️',
    description: 'Code cleanup, optimization, restructuring',
    defaults: {
      phase: 'IMPLEMENT',
      persona: 'AXIOM - Systems Architect',
      priority: 'P2',
      mode: 'STEP-CONFIRM',
      temp: '0.1',
      timeout: '60m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: true,
        shell: true, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: '.env, secrets/, .git/config',
      hardConstraints: [
        'DO NOT change external behavior',
        'All tests must pass before and after',
        'Commit after each logical change'
      ],
      softConstraints: [
        'Prefer small, incremental changes',
        'Update related documentation'
      ],
      role: 'You are AXIOM performing surgical refactoring. You make incremental changes, verify tests pass after each step, and preserve external behavior.'
    }
  },
  
  MIGRATION: {
    name: 'Data Migration',
    icon: '📦',
    description: 'Schema changes, data transforms, system migrations',
    defaults: {
      phase: 'PLAN',
      persona: 'NEXUS - Integration Specialist',
      priority: 'P1',
      mode: 'STEP-CONFIRM',
      temp: '0.1',
      timeout: '120m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: true, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: true, dbWrite: true, webSearch: false
      },
      forbidPaths: '.env, secrets/, production/',
      hardConstraints: [
        'DO NOT execute on production without backup verification',
        'DO NOT proceed without rollback plan',
        'Log all data transformations'
      ],
      softConstraints: [
        'Test on staging first',
        'Include data validation checks'
      ],
      role: 'You are NEXUS, an integration specialist handling data migrations with extreme care. You always have a rollback plan, validate data integrity, and never touch production without safeguards.'
    }
  },
  
  DOCUMENTATION: {
    name: 'Documentation',
    icon: '📝',
    description: 'Technical writing, API docs, guides',
    defaults: {
      phase: 'WALK',
      persona: '',
      priority: 'P3',
      mode: 'AUTONOMOUS',
      temp: '0.3',
      timeout: '30m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: false, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: 'src/, .env, secrets/',
      hardConstraints: [
        'DO NOT modify source code',
        'Follow existing doc style guide'
      ],
      softConstraints: [
        'Include examples for all APIs',
        'Keep language clear and concise'
      ],
      role: 'You are a technical writer creating clear, accurate documentation. You follow existing style guides, include practical examples, and organize content for discoverability.'
    }
  },
  
  RFC_ALIGNMENT: {
    name: 'RFC Alignment',
    icon: '📋',
    description: 'RFC review, implementation verification, compliance',
    defaults: {
      phase: 'WALK',
      persona: 'FORGE - Build Engineer',
      priority: 'P1',
      mode: 'SUPERVISED',
      temp: '0.1',
      timeout: '60m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: true, gitRead: true, gitCommit: true, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: '.env, secrets/',
      hardConstraints: [
        'DO NOT renumber existing RFCs',
        'DO NOT modify RFC content except for alignments',
        'DO NOT create per-RFC directories'
      ],
      softConstraints: [
        'Use shared LaTeX system for all RFCs',
        'Document all alignment decisions'
      ],
      role: 'You are FORGE, a release engineer verifying RFC implementation alignment. You maintain strict RFC numbering, use centralized build systems, and document every decision.'
    }
  },
  
  THREAT_EMULATION: {
    name: 'Threat Emulation',
    icon: '🎯',
    description: 'Red team exercises, attack simulation, TTPs',
    defaults: {
      phase: 'IMPLEMENT',
      persona: 'SENTINEL - Threat Analyst',
      priority: 'P1',
      mode: 'STEP-CONFIRM',
      temp: '0.2',
      timeout: '90m',
      tools: {
        fsRead: true, fsWrite: true, fsDelete: false,
        shell: true, gitRead: true, gitCommit: false, gitPush: false,
        network: false, dbRead: true, dbWrite: false, webSearch: true
      },
      forbidPaths: '.env, secrets/, production/, live/',
      hardConstraints: [
        'DO NOT execute against production systems',
        'DO NOT exfiltrate real data',
        'All actions must be logged',
        'HALT on any unintended impact'
      ],
      softConstraints: [
        'Map actions to MITRE ATT&CK',
        'Document detection opportunities'
      ],
      role: 'You are SENTINEL conducting controlled threat emulation. You simulate adversary TTPs in isolated environments, log all actions, and identify detection opportunities.'
    }
  },
  
  CUSTOM: {
    name: 'Custom',
    icon: '⚙️',
    description: 'Start from scratch',
    defaults: {
      phase: 'PLAN',
      persona: '',
      priority: 'P2',
      mode: 'SUPERVISED',
      temp: '0.2',
      timeout: '30m',
      tools: {
        fsRead: true, fsWrite: false, fsDelete: false,
        shell: false, gitRead: true, gitCommit: false, gitPush: false,
        network: false, dbRead: false, dbWrite: false, webSearch: false
      },
      forbidPaths: '.env, secrets/, .git/',
      hardConstraints: [],
      softConstraints: [],
      role: ''
    }
  }
};

// ═══════════════════════════════════════════════════════════════════════════════
// FILE BROWSER COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

const FileBrowser = ({ isOpen, onClose, onSelect, selectionMode = 'single', fileTypes = 'all' }) => {
  const [currentPath, setCurrentPath] = useState('/');
  const [items, setItems] = useState([]);
  const [selected, setSelected] = useState([]);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [expandedDirs, setExpandedDirs] = useState(new Set());

  // Simulated file system - in production this would call an API
  const mockFileSystem = {
    '/': [
      { name: 'home', type: 'dir', path: '/home' },
      { name: 'projects', type: 'dir', path: '/projects' },
      { name: 'mnt', type: 'dir', path: '/mnt' }
    ],
    '/home': [
      { name: 'claude', type: 'dir', path: '/home/claude' }
    ],
    '/home/claude': [
      { name: 'sx9', type: 'dir', path: '/home/claude/sx9' },
      { name: 'prompts', type: 'dir', path: '/home/claude/prompts' },
      { name: '.bashrc', type: 'file', path: '/home/claude/.bashrc' }
    ],
    '/home/claude/sx9': [
      { name: '01-rfc', type: 'dir', path: '/home/claude/sx9/01-rfc' },
      { name: '02-sx9-latex', type: 'dir', path: '/home/claude/sx9/02-sx9-latex' },
      { name: '03-sx9-tools', type: 'dir', path: '/home/claude/sx9/03-sx9-tools' },
      { name: '04-sx9-research', type: 'dir', path: '/home/claude/sx9/04-sx9-research' },
      { name: '05-sx9-converge', type: 'dir', path: '/home/claude/sx9/05-sx9-converge' },
      { name: 'README.md', type: 'file', path: '/home/claude/sx9/README.md' },
      { name: 'Makefile', type: 'file', path: '/home/claude/sx9/Makefile' }
    ],
    '/home/claude/sx9/01-rfc': [
      { name: 'RFC-0001.md', type: 'file', path: '/home/claude/sx9/01-rfc/RFC-0001.md' },
      { name: 'RFC-0002.md', type: 'file', path: '/home/claude/sx9/01-rfc/RFC-0002.md' },
      { name: 'RFC-0003.md', type: 'file', path: '/home/claude/sx9/01-rfc/RFC-0003.md' },
      { name: 'RFC-REGISTRY.yaml', type: 'file', path: '/home/claude/sx9/01-rfc/RFC-REGISTRY.yaml' }
    ],
    '/projects': [
      { name: 'ctas', type: 'dir', path: '/projects/ctas' },
      { name: 'synaptix', type: 'dir', path: '/projects/synaptix' }
    ],
    '/mnt': [
      { name: 'user-data', type: 'dir', path: '/mnt/user-data' }
    ],
    '/mnt/user-data': [
      { name: 'uploads', type: 'dir', path: '/mnt/user-data/uploads' },
      { name: 'outputs', type: 'dir', path: '/mnt/user-data/outputs' }
    ]
  };

  useEffect(() => {
    if (isOpen) {
      loadDirectory(currentPath);
    }
  }, [isOpen, currentPath]);

  const loadDirectory = (path) => {
    setLoading(true);
    // Simulate async load
    setTimeout(() => {
      const contents = mockFileSystem[path] || [];
      setItems(contents);
      setLoading(false);
    }, 100);
  };

  const navigateTo = (path) => {
    setCurrentPath(path);
    setSelected([]);
  };

  const toggleSelect = (item) => {
    if (selectionMode === 'single') {
      setSelected([item]);
    } else {
      setSelected(prev => {
        const exists = prev.find(i => i.path === item.path);
        if (exists) {
          return prev.filter(i => i.path !== item.path);
        }
        return [...prev, item];
      });
    }
  };

  const confirmSelection = () => {
    onSelect(selectionMode === 'single' ? selected[0] : selected);
    onClose();
  };

  const getBreadcrumbs = () => {
    const parts = currentPath.split('/').filter(Boolean);
    const crumbs = [{ name: 'root', path: '/' }];
    let accumulated = '';
    parts.forEach(part => {
      accumulated += '/' + part;
      crumbs.push({ name: part, path: accumulated });
    });
    return crumbs;
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4">
      <div className="bg-zinc-900 border border-zinc-700 rounded-lg w-full max-w-2xl max-h-[80vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-zinc-700">
          <h3 className="font-medium text-emerald-400">Select Path</h3>
          <button onClick={onClose} className="text-zinc-400 hover:text-zinc-200">
            <X size={18} />
          </button>
        </div>

        {/* Breadcrumbs */}
        <div className="flex items-center gap-1 px-4 py-2 bg-zinc-800 text-xs overflow-x-auto">
          {getBreadcrumbs().map((crumb, i) => (
            <React.Fragment key={crumb.path}>
              {i > 0 && <ChevronRight size={12} className="text-zinc-600" />}
              <button
                onClick={() => navigateTo(crumb.path)}
                className="text-zinc-400 hover:text-emerald-400 whitespace-nowrap"
              >
                {crumb.name}
              </button>
            </React.Fragment>
          ))}
        </div>

        {/* Search */}
        <div className="px-4 py-2 border-b border-zinc-800">
          <div className="relative">
            <Search size={14} className="absolute left-2 top-1/2 -translate-y-1/2 text-zinc-500" />
            <input
              type="text"
              placeholder="Filter..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-zinc-800 border border-zinc-700 rounded pl-8 pr-3 py-1.5 text-sm text-zinc-100 focus:border-emerald-500 focus:outline-none"
            />
          </div>
        </div>

        {/* File List */}
        <div className="flex-1 overflow-auto p-2">
          {loading ? (
            <div className="flex items-center justify-center py-8 text-zinc-500">
              <RefreshCw size={16} className="animate-spin mr-2" />
              Loading...
            </div>
          ) : (
            <div className="space-y-0.5">
              {currentPath !== '/' && (
                <button
                  onClick={() => navigateTo(currentPath.split('/').slice(0, -1).join('/') || '/')}
                  className="w-full flex items-center gap-2 px-3 py-2 rounded hover:bg-zinc-800 text-left text-sm text-zinc-400"
                >
                  <Folder size={16} />
                  <span>..</span>
                </button>
              )}
              {items
                .filter(item => !searchQuery || item.name.toLowerCase().includes(searchQuery.toLowerCase()))
                .map(item => {
                  const isSelected = selected.find(s => s.path === item.path);
                  return (
                    <div
                      key={item.path}
                      className={`flex items-center gap-2 px-3 py-2 rounded cursor-pointer text-sm ${
                        isSelected ? 'bg-emerald-900/30 border border-emerald-700' : 'hover:bg-zinc-800 border border-transparent'
                      }`}
                      onClick={() => item.type === 'dir' ? navigateTo(item.path) : toggleSelect(item)}
                      onDoubleClick={() => item.type === 'dir' && navigateTo(item.path)}
                    >
                      {item.type === 'dir' ? (
                        <Folder size={16} className="text-amber-500" />
                      ) : (
                        <File size={16} className="text-zinc-500" />
                      )}
                      <span className={`flex-1 ${isSelected ? 'text-emerald-300' : 'text-zinc-200'}`}>
                        {item.name}
                      </span>
                      {item.type === 'dir' && (
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            toggleSelect(item);
                          }}
                          className={`px-2 py-0.5 text-xs rounded ${
                            isSelected ? 'bg-emerald-600 text-white' : 'bg-zinc-700 text-zinc-400 hover:bg-zinc-600'
                          }`}
                        >
                          {isSelected ? 'Selected' : 'Select'}
                        </button>
                      )}
                      {item.type === 'file' && isSelected && (
                        <Check size={14} className="text-emerald-400" />
                      )}
                    </div>
                  );
                })}
            </div>
          )}
        </div>

        {/* Selected Summary */}
        {selected.length > 0 && (
          <div className="px-4 py-2 bg-zinc-800 border-t border-zinc-700 text-xs">
            <span className="text-zinc-400">Selected: </span>
            <span className="text-emerald-400">{selected.map(s => s.path).join(', ')}</span>
          </div>
        )}

        {/* Footer */}
        <div className="flex items-center justify-between px-4 py-3 border-t border-zinc-700">
          <div className="text-xs text-zinc-500">
            {selectionMode === 'multi' ? 'Click to select multiple' : 'Click to select'}
          </div>
          <div className="flex gap-2">
            <button
              onClick={onClose}
              className="px-4 py-1.5 text-sm bg-zinc-700 hover:bg-zinc-600 rounded"
            >
              Cancel
            </button>
            <button
              onClick={confirmSelection}
              disabled={selected.length === 0}
              className="px-4 py-1.5 text-sm bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 disabled:cursor-not-allowed rounded"
            >
              Confirm
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════════════════════
// PATH INPUT COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

const PathInput = ({ label, value, onChange, selectionMode = 'single', placeholder }) => {
  const [browserOpen, setBrowserOpen] = useState(false);

  const handleSelect = (selection) => {
    if (selectionMode === 'single') {
      onChange(selection?.path || '');
    } else {
      const paths = selection.map(s => s.path).join(', ');
      onChange(paths);
    }
  };

  return (
    <div>
      <label className="block text-xs text-zinc-500 mb-1">{label}</label>
      <div className="flex gap-2">
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          className="flex-1 bg-zinc-800 border border-zinc-700 rounded px-2 py-1.5 text-sm text-zinc-100 focus:border-emerald-500 focus:outline-none font-mono"
        />
        <button
          onClick={() => setBrowserOpen(true)}
          className="px-3 py-1.5 bg-zinc-700 hover:bg-zinc-600 rounded text-sm"
        >
          Browse
        </button>
      </div>
      <FileBrowser
        isOpen={browserOpen}
        onClose={() => setBrowserOpen(false)}
        onSelect={handleSelect}
        selectionMode={selectionMode}
      />
    </div>
  );
};

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

const SX9PromptForge = () => {
  const [promptType, setPromptType] = useState('CUSTOM');
  const [formData, setFormData] = useState({
    title: '',
    rfc: '',
    phase: 'PLAN',
    classification: 'INTERNAL',
    persona: '',
    priority: 'P2',
    agent: 'CLAUDE',
    model: 'claude-sonnet-4-20250514',
    mode: 'SUPERVISED',
    temp: '0.2',
    timeout: '30m',
    onFail: 'HALT',
    fsRead: true,
    fsWrite: false,
    fsDelete: false,
    shell: false,
    gitRead: true,
    gitCommit: false,
    gitPush: false,
    network: false,
    dbRead: false,
    dbWrite: false,
    webSearch: false,
    mcp: '',
    workdir: '',
    targetFiles: '',
    allowPaths: '',
    forbidPaths: '.env, secrets/, .git/',
    gates: '',
    objective: '',
    context: '',
    hardConstraints: '',
    softConstraints: '',
    deliverables: '',
    acceptance: '',
    dependencies: '',
    rollback: '',
    role: '',
    task: '',
    outputFormat: ''
  });

  const [generated, setGenerated] = useState('');
  const [copied, setCopied] = useState(false);

  // Apply preset when prompt type changes
  useEffect(() => {
    const preset = PROMPT_TYPES[promptType];
    if (preset && preset.defaults) {
      const d = preset.defaults;
      setFormData(prev => ({
        ...prev,
        phase: d.phase || prev.phase,
        persona: d.persona || '',
        priority: d.priority || prev.priority,
        mode: d.mode || prev.mode,
        temp: d.temp || prev.temp,
        timeout: d.timeout || prev.timeout,
        ...(d.tools || {}),
        forbidPaths: d.forbidPaths || prev.forbidPaths,
        hardConstraints: (d.hardConstraints || []).join('\n'),
        softConstraints: (d.softConstraints || []).join('\n'),
        role: d.role || ''
      }));
    }
  }, [promptType]);

  const phases = ['PULL', 'ANALYZE', 'RESEARCH', 'PLAN', 'IMPLEMENT', 'WALK', 'COMMIT'];
  const classifications = ['INTERNAL', 'SENSITIVE', 'PUBLIC'];
  const priorities = ['P0-CRITICAL', 'P1-HIGH', 'P2-STANDARD', 'P3-LOW'];
  const agents = ['CLAUDE', 'CURSOR', 'CODEX', 'LOCAL-LLM', 'CUSTOM'];
  const modes = ['AUTONOMOUS', 'SUPERVISED', 'STEP-CONFIRM'];
  const failActions = ['HALT', 'ROLLBACK', 'ESCALATE', 'LOG-CONTINUE'];
  const models = ['claude-sonnet-4-20250514', 'claude-opus-4-20250514', 'gpt-4-turbo', 'gpt-4o', 'local-llama', 'custom'];

  const handleChange = (field, value) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const generatePromptId = () => {
    const now = new Date();
    return `PRM-${now.getFullYear()}${String(now.getMonth()+1).padStart(2,'0')}${String(now.getDate()).padStart(2,'0')}-${String(now.getHours()).padStart(2,'0')}${String(now.getMinutes()).padStart(2,'0')}`;
  };

  const generatePrompt = useCallback(() => {
    const id = generatePromptId();
    const date = new Date().toISOString().split('T')[0];
    
    const toolsList = [];
    if (formData.fsRead) toolsList.push('fs_read');
    if (formData.fsWrite) toolsList.push('fs_write');
    if (formData.fsDelete) toolsList.push('fs_delete');
    if (formData.shell) toolsList.push('shell');
    if (formData.gitRead) toolsList.push('git_read');
    if (formData.gitCommit) toolsList.push('git_commit');
    if (formData.gitPush) toolsList.push('git_push');
    if (formData.network) toolsList.push('network');
    if (formData.dbRead) toolsList.push('db_read');
    if (formData.dbWrite) toolsList.push('db_write');
    if (formData.webSearch) toolsList.push('web_search');

    const output = `# SX9-PROMPT v1.0
# Type: ${PROMPT_TYPES[promptType]?.name || 'Custom'}
# Generated: ${date}
# ═══════════════════════════════════════════════════════════════════════════════

header:
  id: ${id}
  rfc: ${formData.rfc || 'N/A'}
  title: "${formData.title}"
  author: Charles E. Payne
  date: ${date}
  phase: ${formData.phase}
  classification: ${formData.classification}
  persona: ${formData.persona || 'DEFAULT'}
  priority: ${formData.priority}

# ═══════════════════════════════════════════════════════════════════════════════
# AGENT HARNESS
# ═══════════════════════════════════════════════════════════════════════════════

harness:
  agent: ${formData.agent}
  model: ${formData.model}
  mode: ${formData.mode}
  temperature: ${formData.temp}
  timeout: ${formData.timeout}
  on_fail: ${formData.onFail}

  tools:
    permitted: [${toolsList.join(', ')}]
    mcp_servers: [${formData.mcp}]

  filesystem:
    workdir: "${formData.workdir || './'}"
    targets: [${formData.targetFiles}]
    allow: [${formData.allowPaths}]
    forbid: [${formData.forbidPaths}]

  gates:
${formData.gates ? formData.gates.split('\n').map(g => `    - "${g.trim()}"`).join('\n') : '    - "CHECKPOINT: Review before commit"'}

# ═══════════════════════════════════════════════════════════════════════════════
# MISSION
# ═══════════════════════════════════════════════════════════════════════════════

objective: |
  ${formData.objective || '[DEFINE OBJECTIVE]'}

context: |
  ${formData.context || '[PROVIDE CONTEXT]'}

# ═══════════════════════════════════════════════════════════════════════════════
# CONSTRAINTS
# ═══════════════════════════════════════════════════════════════════════════════

constraints:
  hard:
${formData.hardConstraints ? formData.hardConstraints.split('\n').filter(c => c.trim()).map(c => `    - ${c.trim()}`).join('\n') : '    - DO NOT modify files outside working directory'}
  soft:
${formData.softConstraints ? formData.softConstraints.split('\n').filter(c => c.trim()).map(c => `    - ${c.trim()}`).join('\n') : '    - Document all decisions'}

# ═══════════════════════════════════════════════════════════════════════════════
# DELIVERABLES
# ═══════════════════════════════════════════════════════════════════════════════

deliverables:
${formData.deliverables ? formData.deliverables.split('\n').filter(d => d.trim()).map((d, i) => `  - id: D${i+1}\n    name: "${d.trim()}"`).join('\n') : '  - id: D1\n    name: "[DEFINE DELIVERABLE]"'}

acceptance:
${formData.acceptance ? formData.acceptance.split('\n').filter(a => a.trim()).map(a => `  - ${a.trim()}`).join('\n') : '  - All deliverables exist\n  - No constraint violations'}

dependencies:
${formData.dependencies ? formData.dependencies.split('\n').filter(d => d.trim()).map(d => `  - ${d.trim()}`).join('\n') : '  - None specified'}

rollback: |
${formData.rollback || '  git checkout HEAD -- .'}

# ═══════════════════════════════════════════════════════════════════════════════
# PROMPT BODY
# ═══════════════════════════════════════════════════════════════════════════════

role: |
  ${formData.role || 'You are a senior engineer for Synaptix9 (SX9).'}

task: |
  ${formData.task || '[DEFINE TASK STEPS]'}

output_format: |
  ${formData.outputFormat || `1. Status acknowledgment
  2. Execute task steps
  3. Report at checkpoints
  4. Summarize deliverables`}

# ═══════════════════════════════════════════════════════════════════════════════
# EXECUTION LOG (Agent Completes)
# ═══════════════════════════════════════════════════════════════════════════════

execution:
  status: PENDING
  started: null
  completed: null
  log: []
  artifacts: []
  violations: []
`;

    setGenerated(output);
    setCopied(false);
  }, [formData, promptType]);

  const copyToClipboard = async () => {
    await navigator.clipboard.writeText(generated);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const downloadYaml = () => {
    const blob = new Blob([generated], { type: 'text/yaml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${generatePromptId()}.yaml`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const inputClass = "w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1.5 text-sm text-zinc-100 focus:border-emerald-500 focus:outline-none";
  const checkClass = "flex items-center gap-2 text-xs text-zinc-300 cursor-pointer hover:text-zinc-100";

  return (
    <div className="min-h-screen bg-zinc-900 text-zinc-100 p-4">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-4 pb-3 border-b border-zinc-700">
          <div>
            <h1 className="text-xl font-bold text-emerald-400">SX9 PROMPT FORGE v2</h1>
            <p className="text-xs text-zinc-500">Type-Driven • Filesystem-Aware • Deterministic</p>
          </div>
          <div className="flex gap-2">
            <button onClick={generatePrompt} className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 rounded font-medium text-sm">
              GENERATE
            </button>
            <button onClick={downloadYaml} disabled={!generated} className="px-4 py-2 bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded text-sm">
              DOWNLOAD
            </button>
          </div>
        </div>

        {/* Prompt Type Selector */}
        <div className="mb-4">
          <label className="block text-xs text-zinc-500 mb-2">PROMPT TYPE</label>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-2">
            {Object.entries(PROMPT_TYPES).map(([key, type]) => (
              <button
                key={key}
                onClick={() => setPromptType(key)}
                className={`p-3 rounded border text-left transition-all ${
                  promptType === key
                    ? 'border-emerald-500 bg-emerald-900/20'
                    : 'border-zinc-700 bg-zinc-800 hover:border-zinc-600'
                }`}
              >
                <div className="flex items-center gap-2 mb-1">
                  <span>{type.icon}</span>
                  <span className="font-medium text-sm">{type.name}</span>
                </div>
                <p className="text-xs text-zinc-500 line-clamp-1">{type.description}</p>
              </button>
            ))}
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-5 gap-4">
          {/* Left Column - Form */}
          <div className="lg:col-span-3 space-y-4">
            {/* Header Section */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Header</h3>
              <div className="grid grid-cols-2 gap-3">
                <div className="col-span-2">
                  <label className="block text-xs text-zinc-500 mb-1">Title</label>
                  <input className={inputClass} value={formData.title} onChange={(e) => handleChange('title', e.target.value)} placeholder="Descriptive title" />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">RFC Reference</label>
                  <input className={inputClass} value={formData.rfc} onChange={(e) => handleChange('rfc', e.target.value)} placeholder="RFC-XXXX" />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Phase</label>
                  <select className={inputClass} value={formData.phase} onChange={(e) => handleChange('phase', e.target.value)}>
                    {phases.map(p => <option key={p}>{p}</option>)}
                  </select>
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Classification</label>
                  <select className={inputClass} value={formData.classification} onChange={(e) => handleChange('classification', e.target.value)}>
                    {classifications.map(c => <option key={c}>{c}</option>)}
                  </select>
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Priority</label>
                  <select className={inputClass} value={formData.priority} onChange={(e) => handleChange('priority', e.target.value)}>
                    {priorities.map(p => <option key={p} value={p.split('-')[0]}>{p}</option>)}
                  </select>
                </div>
              </div>
            </div>

            {/* Agent Harness */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Agent Harness</h3>
              <div className="grid grid-cols-3 gap-3 mb-3">
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Agent</label>
                  <select className={inputClass} value={formData.agent} onChange={(e) => handleChange('agent', e.target.value)}>
                    {agents.map(a => <option key={a}>{a}</option>)}
                  </select>
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Mode</label>
                  <select className={inputClass} value={formData.mode} onChange={(e) => handleChange('mode', e.target.value)}>
                    {modes.map(m => <option key={m}>{m}</option>)}
                  </select>
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">On Fail</label>
                  <select className={inputClass} value={formData.onFail} onChange={(e) => handleChange('onFail', e.target.value)}>
                    {failActions.map(f => <option key={f}>{f}</option>)}
                  </select>
                </div>
              </div>

              {/* Tool Permissions */}
              <div className="bg-zinc-900 rounded p-3 mb-3">
                <p className="text-xs text-zinc-500 mb-2 font-medium">TOOL PERMISSIONS</p>
                <div className="grid grid-cols-4 gap-2">
                  {[
                    ['fsRead', 'FS Read'], ['fsWrite', 'FS Write'], ['fsDelete', 'FS Delete'], ['shell', 'Shell'],
                    ['gitRead', 'Git Read'], ['gitCommit', 'Git Commit'], ['gitPush', 'Git Push'], ['network', 'Network'],
                    ['dbRead', 'DB Read'], ['dbWrite', 'DB Write'], ['webSearch', 'Web Search']
                  ].map(([key, label]) => (
                    <label key={key} className={checkClass}>
                      <input
                        type="checkbox"
                        checked={formData[key]}
                        onChange={(e) => handleChange(key, e.target.checked)}
                        className="w-3 h-3 rounded"
                      />
                      {label}
                    </label>
                  ))}
                </div>
              </div>

              {/* Filesystem Paths */}
              <div className="space-y-3">
                <PathInput
                  label="Working Directory"
                  value={formData.workdir}
                  onChange={(v) => handleChange('workdir', v)}
                  placeholder="/path/to/project"
                />
                <PathInput
                  label="Target Files/Directories"
                  value={formData.targetFiles}
                  onChange={(v) => handleChange('targetFiles', v)}
                  selectionMode="multi"
                  placeholder="Files to operate on"
                />
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Forbidden Paths</label>
                  <input
                    className={inputClass}
                    value={formData.forbidPaths}
                    onChange={(e) => handleChange('forbidPaths', e.target.value)}
                    placeholder=".env, secrets/"
                  />
                </div>
              </div>
            </div>

            {/* Mission */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Mission</h3>
              <div className="space-y-3">
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Objective</label>
                  <textarea className={inputClass} rows={2} value={formData.objective} onChange={(e) => handleChange('objective', e.target.value)} placeholder="What does this accomplish?" />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Context</label>
                  <textarea className={inputClass} rows={2} value={formData.context} onChange={(e) => handleChange('context', e.target.value)} placeholder="Why now? What triggered this?" />
                </div>
              </div>
            </div>

            {/* Constraints */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Constraints</h3>
              <div className="space-y-3">
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Hard Constraints (one per line)</label>
                  <textarea className={inputClass + ' font-mono'} rows={3} value={formData.hardConstraints} onChange={(e) => handleChange('hardConstraints', e.target.value)} placeholder="DO NOT..." />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Soft Constraints (one per line)</label>
                  <textarea className={inputClass + ' font-mono'} rows={2} value={formData.softConstraints} onChange={(e) => handleChange('softConstraints', e.target.value)} placeholder="Prefer..." />
                </div>
              </div>
            </div>

            {/* Deliverables */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Deliverables</h3>
              <div className="space-y-3">
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Deliverables (one per line)</label>
                  <textarea className={inputClass + ' font-mono'} rows={3} value={formData.deliverables} onChange={(e) => handleChange('deliverables', e.target.value)} placeholder="file.md&#10;report.yaml" />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Acceptance Criteria (one per line)</label>
                  <textarea className={inputClass + ' font-mono'} rows={2} value={formData.acceptance} onChange={(e) => handleChange('acceptance', e.target.value)} placeholder="File exists&#10;Tests pass" />
                </div>
              </div>
            </div>

            {/* Task */}
            <div className="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
              <h3 className="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-3">Task Definition</h3>
              <div className="space-y-3">
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Role</label>
                  <textarea className={inputClass} rows={2} value={formData.role} onChange={(e) => handleChange('role', e.target.value)} placeholder="You are..." />
                </div>
                <div>
                  <label className="block text-xs text-zinc-500 mb-1">Task Steps</label>
                  <textarea className={inputClass + ' font-mono'} rows={5} value={formData.task} onChange={(e) => handleChange('task', e.target.value)} placeholder="1. First step&#10;2. Second step&#10;3. Third step" />
                </div>
              </div>
            </div>
          </div>

          {/* Right Column - Output */}
          <div className="lg:col-span-2 lg:sticky lg:top-4 lg:self-start">
            <div className="bg-zinc-800 rounded-lg border border-zinc-700 overflow-hidden">
              <div className="flex items-center justify-between px-4 py-2 bg-zinc-900 border-b border-zinc-700">
                <span className="text-xs font-medium text-zinc-400">GENERATED PROMPT</span>
                <button
                  onClick={copyToClipboard}
                  disabled={!generated}
                  className="px-3 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded"
                >
                  {copied ? '✓ COPIED' : 'COPY'}
                </button>
              </div>
              <pre className="p-4 text-xs text-zinc-300 overflow-auto max-h-[80vh] font-mono leading-relaxed">
                {generated || 'Select a prompt type and click GENERATE...'}
              </pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SX9PromptForge;
