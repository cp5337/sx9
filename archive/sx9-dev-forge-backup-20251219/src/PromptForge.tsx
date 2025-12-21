import { useState, useCallback, useEffect } from 'react';

// ═══════════════════════════════════════════════════════════════════════════════
// HARNESS CONFIGURATIONS - Pre-configured tool & mode combinations
// ═══════════════════════════════════════════════════════════════════════════════

const HARNESSES = {
  full_autonomous: {
    name: 'Full Autonomous',
    icon: '🚀',
    description: 'All tools, maximum flexibility',
    mode: 'AUTONOMOUS',
    temp: '0.3',
    timeout: '120m',
    tools: {
      web_search: true, web_fetch: true, computer_use: true, memory: true,
      linear: true, canva: true, figma: true, google_drive: true, 
      filesystem: true, vercel: false, huggingface: false
    }
  },
  research: {
    name: 'Research & Intel',
    icon: '🔬',
    description: 'Information gathering focus',
    mode: 'AUTONOMOUS',
    temp: '0.4',
    timeout: '60m',
    tools: {
      web_search: true, web_fetch: true, computer_use: true, memory: true,
      linear: false, canva: false, figma: false, google_drive: true,
      filesystem: true, vercel: false, huggingface: true
    }
  },
  build: {
    name: 'Build & Implement',
    icon: '🔧',
    description: 'Code generation focus',
    mode: 'SUPERVISED',
    temp: '0.2',
    timeout: '90m',
    tools: {
      web_search: true, web_fetch: true, computer_use: true, memory: true,
      linear: true, canva: false, figma: true, google_drive: true,
      filesystem: true, vercel: true, huggingface: false
    }
  },
  security: {
    name: 'Security Audit',
    icon: '🛡️',
    description: 'Read-only analysis focus',
    mode: 'STEP-CONFIRM',
    temp: '0.1',
    timeout: '120m',
    tools: {
      web_search: true, web_fetch: true, computer_use: true, memory: true,
      linear: true, canva: true, figma: false, google_drive: true,
      filesystem: true, vercel: false, huggingface: false
    }
  },
  planning: {
    name: 'Planning & Docs',
    icon: '📋',
    description: 'Strategy and documentation',
    mode: 'SUPERVISED',
    temp: '0.3',
    timeout: '60m',
    tools: {
      web_search: true, web_fetch: true, computer_use: true, memory: true,
      linear: true, canva: true, figma: false, google_drive: true,
      filesystem: true, vercel: false, huggingface: false
    }
  }
};

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONAS - Role definitions with preferred harnesses
// ═══════════════════════════════════════════════════════════════════════════════

const PERSONAS = {
  FORGE: {
    name: 'FORGE - Build Engineer',
    harness: 'build',
    role: 'You are FORGE, a senior build/infrastructure engineer for SX9. You create reliable, reproducible pipelines with minimal dependencies. You track all work in Linear and document decisions thoroughly.'
  },
  AXIOM: {
    name: 'AXIOM - Systems Architect',
    harness: 'build',
    role: 'You are AXIOM, a systems architect implementing clean, modular code. You use Figma for UI reference and Canva for architecture diagrams. Max 300 lines per file, always include tests.'
  },
  VECTOR: {
    name: 'VECTOR - Security Engineer',
    harness: 'security',
    role: 'You are VECTOR, a security engineer conducting thorough audits. You search for CVEs, document findings in Linear with severity levels. You NEVER modify target systems - read-only operations only.'
  },
  SENTINEL: {
    name: 'SENTINEL - Threat Analyst',
    harness: 'research',
    role: 'You are SENTINEL, a threat analyst and red team operator. You research threats, map TTPs to MITRE ATT&CK. You use Linear for tracking and Canva for kill chain diagrams.'
  },
  NEXUS: {
    name: 'NEXUS - Integration Specialist',
    harness: 'build',
    role: 'You are NEXUS, an integration specialist handling data migrations. You always have a rollback plan documented in Linear. You validate data integrity at every step.'
  },
  CIPHER: {
    name: 'CIPHER - Crypto/Privacy',
    harness: 'security',
    role: 'You are CIPHER, a cryptography and privacy engineer. You research best practices and document cryptographic decisions. You NEVER log secrets.'
  },
  SCRIBE: {
    name: 'SCRIBE - Documentation',
    harness: 'planning',
    role: 'You are SCRIBE, a technical writer creating clear documentation. You search Drive for existing docs, create visuals in Canva, track tasks in Linear.'
  }
};

// ═══════════════════════════════════════════════════════════════════════════════
// PROMPT TYPES - Task templates with defaults
// ═══════════════════════════════════════════════════════════════════════════════

const PROMPT_TYPES = {
  BUILD_PIPELINE: {
    name: 'Build Pipeline',
    icon: '🔧',
    persona: 'FORGE',
    phase: 'IMPLEMENT',
    priority: 'P1',
    hardConstraints: ['DO NOT modify source files in src/', 'DO NOT delete existing files', 'DO NOT push without approval'],
    softConstraints: ['Prefer Makefile/justfile over custom scripts', 'Document in BUILD_NOTES.md'],
    linearLabels: ['type:BUILD', 'phase:IMPLEMENT'],
    contextSources: { memory: true, linear: true, drive: true, web: true }
  },
  SECURITY_AUDIT: {
    name: 'Security Audit',
    icon: '🛡️',
    persona: 'VECTOR',
    phase: 'ANALYZE',
    priority: 'P0',
    hardConstraints: ['READ-ONLY - DO NOT modify any files', 'DO NOT exfiltrate sensitive data', 'HALT on credential exposure'],
    softConstraints: ['Flag findings with severity (CRITICAL/HIGH/MEDIUM/LOW)', 'Cross-reference CVE databases'],
    linearLabels: ['type:SECURITY', 'phase:ANALYZE'],
    contextSources: { memory: true, linear: true, drive: true, web: true }
  },
  THREAT_EMULATION: {
    name: 'Threat Emulation',
    icon: '🎯',
    persona: 'SENTINEL',
    phase: 'IMPLEMENT',
    priority: 'P1',
    hardConstraints: ['DO NOT execute against production', 'DO NOT exfiltrate real data', 'Log ALL actions', 'HALT on unintended impact'],
    softConstraints: ['Map all actions to MITRE ATT&CK', 'Document detection opportunities'],
    linearLabels: ['type:THREAT-EMULATION', 'phase:IMPLEMENT'],
    contextSources: { memory: true, linear: true, drive: false, web: true }
  },
  CODE_GENERATION: {
    name: 'Code Generation',
    icon: '💻',
    persona: 'AXIOM',
    phase: 'IMPLEMENT',
    priority: 'P2',
    hardConstraints: ['DO NOT create files > 300 lines', 'DO NOT modify outside designated module', 'All new code must include tests'],
    softConstraints: ['Follow existing code conventions', 'Include docstrings for public APIs'],
    linearLabels: ['type:CODE', 'phase:IMPLEMENT'],
    contextSources: { memory: true, linear: true, drive: true, web: true }
  },
  RFC_ALIGNMENT: {
    name: 'RFC Alignment',
    icon: '📋',
    persona: 'FORGE',
    phase: 'WALK',
    priority: 'P1',
    hardConstraints: ['DO NOT renumber existing RFCs', 'DO NOT modify RFC content except alignments', 'NO per-RFC directories'],
    softConstraints: ['Use shared LaTeX system', 'Document all alignment decisions'],
    linearLabels: ['type:RFC', 'phase:WALK'],
    contextSources: { memory: true, linear: true, drive: true, web: false }
  },
  RESEARCH: {
    name: 'Research & Analysis',
    icon: '🔬',
    persona: 'SENTINEL',
    phase: 'RESEARCH',
    priority: 'P2',
    hardConstraints: ['DO NOT modify existing documentation', 'DO NOT execute arbitrary code', 'Cite ALL sources'],
    softConstraints: ['Prefer primary sources', 'Include confidence levels on findings'],
    linearLabels: ['type:RESEARCH', 'phase:RESEARCH'],
    contextSources: { memory: true, linear: false, drive: true, web: true }
  },
  DOCUMENTATION: {
    name: 'Documentation',
    icon: '📝',
    persona: 'SCRIBE',
    phase: 'WALK',
    priority: 'P3',
    hardConstraints: ['DO NOT modify source code', 'Follow existing doc style guide'],
    softConstraints: ['Include examples for all APIs', 'Keep language clear and concise'],
    linearLabels: ['type:DOCS', 'phase:WALK'],
    contextSources: { memory: true, linear: true, drive: true, web: false }
  },
  CUSTOM: {
    name: 'Custom',
    icon: '⚙️',
    persona: '',
    phase: 'PLAN',
    priority: 'P2',
    hardConstraints: [],
    softConstraints: [],
    linearLabels: [],
    contextSources: { memory: true, linear: false, drive: false, web: false }
  }
};

// Type definitions
type HarnessKey = keyof typeof HARNESSES;
type PersonaKey = keyof typeof PERSONAS | '';
type PromptTypeKey = keyof typeof PROMPT_TYPES;

interface FormState {
  title: string;
  rfc: string;
  phase: string;
  classification: string;
  priority: string;
  mode: string;
  temp: string;
  timeout: string;
  onFail: string;
  web_search: boolean;
  web_fetch: boolean;
  computer_use: boolean;
  memory: boolean;
  linear: boolean;
  canva: boolean;
  figma: boolean;
  google_drive: boolean;
  filesystem: boolean;
  vercel: boolean;
  huggingface: boolean;
  ctx_memory: boolean;
  ctx_linear: boolean;
  ctx_drive: boolean;
  ctx_web: boolean;
  linearTeam: string;
  linearProject: string;
  createLinearIssue: boolean;
  linearLabels: string;
  workdir: string;
  forbidPaths: string;
  objective: string;
  context: string;
  hardConstraints: string;
  softConstraints: string;
  deliverables: string;
  acceptance: string;
  role: string;
  task: string;
  [key: string]: string | boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

export default function PromptForge() {
  // State
  const [promptType, setPromptType] = useState<PromptTypeKey>('CUSTOM');
  const [harness, setHarness] = useState<HarnessKey>('build');
  const [persona, setPersona] = useState<PersonaKey>('');
  
  const [form, setForm] = useState<FormState>({
    title: '',
    rfc: '',
    phase: 'PLAN',
    classification: 'INTERNAL',
    priority: 'P2',
    mode: 'SUPERVISED',
    temp: '0.2',
    timeout: '30m',
    onFail: 'HALT',
    
    // Tools
    web_search: true,
    web_fetch: true,
    computer_use: true,
    memory: true,
    linear: true,
    canva: false,
    figma: false,
    google_drive: true,
    filesystem: true,
    vercel: false,
    huggingface: false,
    
    // Context sources
    ctx_memory: true,
    ctx_linear: true,
    ctx_drive: true,
    ctx_web: true,
    
    // Linear integration
    linearTeam: 'SX9',
    linearProject: '',
    createLinearIssue: true,
    linearLabels: '',
    
    // Paths
    workdir: '',
    forbidPaths: '.env, secrets/, .git/config',
    
    // Content
    objective: '',
    context: '',
    hardConstraints: '',
    softConstraints: '',
    deliverables: '',
    acceptance: '',
    role: '',
    task: ''
  });
  
  const [output, setOutput] = useState('');
  const [copied, setCopied] = useState(false);
  const [activeTab, setActiveTab] = useState('config');

  // Apply prompt type defaults
  useEffect(() => {
    const type = PROMPT_TYPES[promptType];
    if (type) {
      const pKey = type.persona as PersonaKey;
      const p = pKey ? PERSONAS[pKey as keyof typeof PERSONAS] : null;
      const h = p ? HARNESSES[p.harness as HarnessKey] : HARNESSES.build;
      
      setPersona(pKey || '');
      if (p) setHarness(p.harness as HarnessKey);
      
      setForm(f => ({
        ...f,
        phase: type.phase,
        priority: type.priority,
        mode: h?.mode || f.mode,
        temp: h?.temp || f.temp,
        timeout: h?.timeout || f.timeout,
        ...(h?.tools || {}),
        ctx_memory: type.contextSources?.memory ?? true,
        ctx_linear: type.contextSources?.linear ?? false,
        ctx_drive: type.contextSources?.drive ?? false,
        ctx_web: type.contextSources?.web ?? false,
        hardConstraints: (type.hardConstraints || []).join('\n'),
        softConstraints: (type.softConstraints || []).join('\n'),
        linearLabels: (type.linearLabels || []).join(', '),
        role: p?.role || ''
      }));
    }
  }, [promptType]);

  // Apply harness defaults
  useEffect(() => {
    const h = HARNESSES[harness];
    if (h) {
      setForm(f => ({
        ...f,
        mode: h.mode,
        temp: h.temp,
        timeout: h.timeout,
        ...h.tools
      }));
    }
  }, [harness]);

  // Apply persona defaults
  useEffect(() => {
    if (persona && persona in PERSONAS) {
      const p = PERSONAS[persona as keyof typeof PERSONAS];
      setHarness(p.harness as HarnessKey);
      setForm(f => ({ ...f, role: p.role }));
    }
  }, [persona]);

  const set = (k: string, v: string | boolean) => setForm(f => ({ ...f, [k]: v }));
  
  const genId = () => {
    const d = new Date();
    return `PRM-${d.toISOString().slice(0,10).replace(/-/g,'')}-${d.toTimeString().slice(0,5).replace(':','')}`;
  };

  const generate = useCallback(() => {
    const id = genId();
    const date = new Date().toISOString().split('T')[0];
    
    const enabledTools = [
      form.web_search && 'web_search',
      form.web_fetch && 'web_fetch', 
      form.computer_use && 'computer_use',
      form.memory && 'memory',
      form.linear && 'linear',
      form.canva && 'canva',
      form.figma && 'figma',
      form.google_drive && 'google_drive',
      form.filesystem && 'filesystem',
      form.vercel && 'vercel',
      form.huggingface && 'huggingface'
    ].filter(Boolean);

    const contextSources = [
      form.ctx_memory && 'memory',
      form.ctx_linear && 'linear', 
      form.ctx_drive && 'google_drive',
      form.ctx_web && 'web_search'
    ].filter(Boolean);

    const yaml = `# SX9-PROMPT v2.0
# Type: ${PROMPT_TYPES[promptType]?.name || 'Custom'}
# Generated: ${date}
# ═══════════════════════════════════════════════════════════════════════════════

header:
  id: ${id}
  rfc: ${form.rfc || 'N/A'}
  title: "${form.title}"
  author: Charles E. Payne
  date: ${date}
  phase: ${form.phase}
  classification: ${form.classification}
  priority: ${form.priority}

# ═══════════════════════════════════════════════════════════════════════════════
# AGENT HARNESS
# ═══════════════════════════════════════════════════════════════════════════════

harness:
  base: ${harness}  # ${HARNESSES[harness]?.name}
  persona: ${persona || 'DEFAULT'}
  mode: ${form.mode}
  temperature: ${form.temp}
  timeout: ${form.timeout}
  on_fail: ${form.onFail}

  tools:
    enabled: [${enabledTools.join(', ')}]
    
  context_loading:
    sources: [${contextSources.join(', ')}]
    sequence:
      ${form.ctx_memory ? '- "Search memory for relevant prior work"' : ''}
      ${form.ctx_linear ? '- "Check Linear for related issues/context"' : ''}
      ${form.ctx_drive ? '- "Search Drive for internal documentation"' : ''}
      ${form.ctx_web ? '- "Web search for external references"' : ''}

  filesystem:
    workdir: "${form.workdir || './'}"
    forbid: [${form.forbidPaths}]

# ═══════════════════════════════════════════════════════════════════════════════
# LINEAR INTEGRATION
# ═══════════════════════════════════════════════════════════════════════════════

linear:
  enabled: ${form.createLinearIssue}
  team: "${form.linearTeam}"
  project: "${form.linearProject}"
  labels: [${form.linearLabels}]
  
  on_start:
    - "Create issue with mission details"
    - "Add phase/persona labels"
  on_checkpoint:
    - "Add comment with progress"
  on_complete:
    - "Update issue to Done"
    - "Link deliverable artifacts"

# ═══════════════════════════════════════════════════════════════════════════════
# MISSION
# ═══════════════════════════════════════════════════════════════════════════════

objective: |
  ${form.objective || '[DEFINE OBJECTIVE]'}

context: |
  ${form.context || '[PROVIDE CONTEXT]'}

constraints:
  hard:
${form.hardConstraints ? form.hardConstraints.split('\n').filter(Boolean).map(c => `    - "${c.trim()}"`).join('\n') : '    - "DO NOT modify files outside working directory"'}
  soft:
${form.softConstraints ? form.softConstraints.split('\n').filter(Boolean).map(c => `    - "${c.trim()}"`).join('\n') : '    - "Document all decisions"'}

deliverables:
${form.deliverables ? form.deliverables.split('\n').filter(Boolean).map((d, i) => `  - id: D${i+1}\n    name: "${d.trim()}"`).join('\n') : '  - id: D1\n    name: "[DEFINE DELIVERABLE]"'}

acceptance:
${form.acceptance ? form.acceptance.split('\n').filter(Boolean).map(a => `  - "${a.trim()}"`).join('\n') : '  - "All deliverables exist"\n  - "No constraint violations"'}

# ═══════════════════════════════════════════════════════════════════════════════
# EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

role: |
  ${form.role || 'You are a senior engineer for SX9.'}

task: |
  CONTEXT LOADING (execute first):
  ${form.ctx_memory ? '1. Search conversation memory for relevant prior work on this topic' : ''}
  ${form.ctx_linear ? '2. Check Linear for existing issues, projects, dependencies' : ''}
  ${form.ctx_drive ? '3. Search Google Drive for internal documentation' : ''}
  ${form.ctx_web ? '4. Web search for external references if needed' : ''}
  
  EXECUTION:
  ${form.task || '[DEFINE TASK STEPS]'}
  
  COMPLETION:
  ${form.createLinearIssue ? '- Update Linear issue with results' : ''}
  - Output all artifacts to /mnt/user-data/outputs
  - Summarize deliverables and any constraint notes

# ═══════════════════════════════════════════════════════════════════════════════
# TRACKING
# ═══════════════════════════════════════════════════════════════════════════════

execution:
  status: PENDING
  linear_issue: null
  started: null
  checkpoints: []
  artifacts: []
  violations: []
`;

    setOutput(yaml);
    setCopied(false);
  }, [form, promptType, harness, persona]);

  const copy = async () => {
    await navigator.clipboard.writeText(output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const download = () => {
    const blob = new Blob([output], { type: 'text/yaml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${genId()}.yaml`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const inputClass = "w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1.5 text-sm text-zinc-100 focus:border-emerald-500 focus:outline-none";
  const labelClass = "block text-xs text-zinc-500 mb-1";
  const checkClass = "flex items-center gap-2 text-xs text-zinc-300 cursor-pointer hover:text-zinc-100";

  return (
    <div className="min-h-screen bg-zinc-900 text-zinc-100 p-4">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-4 pb-3 border-b border-zinc-700">
          <div>
            <h1 className="text-xl font-bold text-emerald-400">SX9 DEV FORGE</h1>
            <p className="text-xs text-zinc-500">Prompt Engineering • Linear • IDE Bootstrap</p>
          </div>
          <div className="flex gap-2">
            <button onClick={generate} className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 rounded font-medium text-sm">
              GENERATE
            </button>
            <button onClick={download} disabled={!output} className="px-4 py-2 bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded text-sm">
              DOWNLOAD
            </button>
          </div>
        </div>

        {/* Type Selector */}
        <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-2 mb-4">
          {Object.entries(PROMPT_TYPES).map(([k, t]) => (
            <button key={k} onClick={() => setPromptType(k as PromptTypeKey)}
              className={`p-2 rounded border text-left transition-all ${
                promptType === k ? 'border-emerald-500 bg-emerald-900/20' : 'border-zinc-700 bg-zinc-800 hover:border-zinc-600'
              }`}>
              <div className="flex items-center gap-1">
                <span>{t.icon}</span>
                <span className="font-medium text-xs truncate">{t.name}</span>
              </div>
            </button>
          ))}
        </div>

        {/* Harness + Persona Row */}
        <div className="grid grid-cols-2 gap-4 mb-4">
          {/* Harness Selector */}
          <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
            <label className={labelClass}>AGENT HARNESS</label>
            <div className="grid grid-cols-5 gap-1">
              {Object.entries(HARNESSES).map(([k, h]) => (
                <button key={k} onClick={() => setHarness(k as HarnessKey)}
                  className={`p-2 rounded border text-center ${
                    harness === k ? 'border-cyan-500 bg-cyan-900/20' : 'border-zinc-700 bg-zinc-900 hover:border-zinc-600'
                  }`}>
                  <div className="text-lg">{h.icon}</div>
                  <div className="text-xs truncate">{h.name}</div>
                </button>
              ))}
            </div>
          </div>
          
          {/* Persona Selector */}
          <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
            <label className={labelClass}>PERSONA</label>
            <div className="grid grid-cols-4 gap-1">
              {Object.entries(PERSONAS).map(([k]) => (
                <button key={k} onClick={() => setPersona(k as PersonaKey)}
                  className={`p-2 rounded border text-center ${
                    persona === k ? 'border-amber-500 bg-amber-900/20' : 'border-zinc-700 bg-zinc-900 hover:border-zinc-600'
                  }`}>
                  <div className="text-xs font-medium truncate">{k}</div>
                </button>
              ))}
              <button onClick={() => setPersona('')}
                className={`p-2 rounded border text-center ${
                  !persona ? 'border-amber-500 bg-amber-900/20' : 'border-zinc-700 bg-zinc-900'
                }`}>
                <div className="text-xs">DEFAULT</div>
              </button>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-5 gap-4">
          {/* Left Column - Form */}
          <div className="lg:col-span-3 space-y-3">
            {/* Tabs */}
            <div className="flex gap-1 border-b border-zinc-700 pb-2">
              {['config', 'tools', 'linear', 'mission'].map(tab => (
                <button key={tab} onClick={() => setActiveTab(tab)}
                  className={`px-3 py-1 rounded-t text-sm ${
                    activeTab === tab ? 'bg-zinc-700 text-white' : 'text-zinc-500 hover:text-zinc-300'
                  }`}>
                  {tab.toUpperCase()}
                </button>
              ))}
            </div>

            {/* Config Tab */}
            {activeTab === 'config' && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <div className="grid grid-cols-2 gap-3">
                    <div className="col-span-2">
                      <label className={labelClass}>Title</label>
                      <input className={inputClass} value={form.title} onChange={e => set('title', e.target.value)} placeholder="Descriptive title" />
                    </div>
                    <div>
                      <label className={labelClass}>RFC Reference</label>
                      <input className={inputClass} value={form.rfc} onChange={e => set('rfc', e.target.value)} placeholder="RFC-XXXX" />
                    </div>
                    <div>
                      <label className={labelClass}>Phase</label>
                      <select className={inputClass} value={form.phase} onChange={e => set('phase', e.target.value)}>
                        {['PULL', 'ANALYZE', 'RESEARCH', 'PLAN', 'IMPLEMENT', 'WALK', 'COMMIT'].map(p => <option key={p}>{p}</option>)}
                      </select>
                    </div>
                    <div>
                      <label className={labelClass}>Priority</label>
                      <select className={inputClass} value={form.priority} onChange={e => set('priority', e.target.value)}>
                        {['P0', 'P1', 'P2', 'P3'].map(p => <option key={p}>{p}</option>)}
                      </select>
                    </div>
                    <div>
                      <label className={labelClass}>Classification</label>
                      <select className={inputClass} value={form.classification} onChange={e => set('classification', e.target.value)}>
                        {['INTERNAL', 'SENSITIVE', 'PUBLIC'].map(c => <option key={c}>{c}</option>)}
                      </select>
                    </div>
                  </div>
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <div className="grid grid-cols-4 gap-3">
                    <div>
                      <label className={labelClass}>Mode</label>
                      <select className={inputClass} value={form.mode} onChange={e => set('mode', e.target.value)}>
                        {['AUTONOMOUS', 'SUPERVISED', 'STEP-CONFIRM'].map(m => <option key={m}>{m}</option>)}
                      </select>
                    </div>
                    <div>
                      <label className={labelClass}>Temperature</label>
                      <input className={inputClass} value={form.temp} onChange={e => set('temp', e.target.value)} />
                    </div>
                    <div>
                      <label className={labelClass}>Timeout</label>
                      <input className={inputClass} value={form.timeout} onChange={e => set('timeout', e.target.value)} />
                    </div>
                    <div>
                      <label className={labelClass}>On Fail</label>
                      <select className={inputClass} value={form.onFail} onChange={e => set('onFail', e.target.value)}>
                        {['HALT', 'ROLLBACK', 'ESCALATE', 'LOG-CONTINUE'].map(f => <option key={f}>{f}</option>)}
                      </select>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Tools Tab */}
            {activeTab === 'tools' && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>ENABLED TOOLS</label>
                  <div className="grid grid-cols-4 gap-2 mt-2">
                    {[
                      ['web_search', 'Web Search', '🔍'],
                      ['web_fetch', 'Web Fetch', '📥'],
                      ['computer_use', 'Computer', '💻'],
                      ['memory', 'Memory', '🧠'],
                      ['linear', 'Linear', '📋'],
                      ['canva', 'Canva', '🎨'],
                      ['figma', 'Figma', '🖼️'],
                      ['google_drive', 'Drive', '📁'],
                      ['filesystem', 'Filesystem', '📂'],
                      ['vercel', 'Vercel', '▲'],
                      ['huggingface', 'HuggingFace', '🤗']
                    ].map(([k, l, i]) => (
                      <label key={k} className={`${checkClass} p-2 rounded border ${form[k] ? 'border-emerald-700 bg-emerald-900/20' : 'border-zinc-700'}`}>
                        <input type="checkbox" checked={form[k] as boolean} onChange={e => set(k, e.target.checked)} className="w-3 h-3" />
                        <span>{i}</span>
                        <span className="truncate">{l}</span>
                      </label>
                    ))}
                  </div>
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>CONTEXT LOADING (Pre-execution)</label>
                  <div className="grid grid-cols-4 gap-2 mt-2">
                    {[
                      ['ctx_memory', 'Search Memory', '🧠'],
                      ['ctx_linear', 'Check Linear', '📋'],
                      ['ctx_drive', 'Search Drive', '📁'],
                      ['ctx_web', 'Web Search', '🔍']
                    ].map(([k, l, i]) => (
                      <label key={k} className={`${checkClass} p-2 rounded border ${form[k] ? 'border-cyan-700 bg-cyan-900/20' : 'border-zinc-700'}`}>
                        <input type="checkbox" checked={form[k] as boolean} onChange={e => set(k, e.target.checked)} className="w-3 h-3" />
                        <span>{i}</span>
                        <span className="text-xs truncate">{l}</span>
                      </label>
                    ))}
                  </div>
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Forbidden Paths</label>
                  <input className={inputClass} value={form.forbidPaths} onChange={e => set('forbidPaths', e.target.value)} />
                </div>
              </div>
            )}

            {/* Linear Tab */}
            {activeTab === 'linear' && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={`${checkClass} mb-3`}>
                    <input type="checkbox" checked={form.createLinearIssue} onChange={e => set('createLinearIssue', e.target.checked)} className="w-4 h-4" />
                    <span className="text-sm font-medium">Create Linear Issue for this mission</span>
                  </label>
                  
                  <div className="grid grid-cols-2 gap-3">
                    <div>
                      <label className={labelClass}>Team</label>
                      <input className={inputClass} value={form.linearTeam} onChange={e => set('linearTeam', e.target.value)} placeholder="SX9" />
                    </div>
                    <div>
                      <label className={labelClass}>Project (optional)</label>
                      <input className={inputClass} value={form.linearProject} onChange={e => set('linearProject', e.target.value)} placeholder="Project name" />
                    </div>
                    <div className="col-span-2">
                      <label className={labelClass}>Labels (comma-separated)</label>
                      <input className={inputClass} value={form.linearLabels} onChange={e => set('linearLabels', e.target.value)} placeholder="type:BUILD, phase:IMPLEMENT" />
                    </div>
                  </div>
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>LINEAR WORKFLOW</label>
                  <div className="text-xs text-zinc-400 space-y-1 mt-2">
                    <p>✓ <strong>On Start:</strong> Create issue with mission details, add labels</p>
                    <p>✓ <strong>On Checkpoint:</strong> Add comment with progress update</p>
                    <p>✓ <strong>On Complete:</strong> Update to Done, link artifacts</p>
                    <p>✓ <strong>On Fail:</strong> Update status, document failure reason</p>
                  </div>
                </div>
              </div>
            )}

            {/* Mission Tab */}
            {activeTab === 'mission' && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Objective</label>
                  <textarea className={inputClass} rows={2} value={form.objective} onChange={e => set('objective', e.target.value)} placeholder="What does this accomplish?" />
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Hard Constraints (one per line)</label>
                  <textarea className={`${inputClass} font-mono`} rows={3} value={form.hardConstraints} onChange={e => set('hardConstraints', e.target.value)} placeholder="DO NOT..." />
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Soft Constraints (one per line)</label>
                  <textarea className={`${inputClass} font-mono`} rows={2} value={form.softConstraints} onChange={e => set('softConstraints', e.target.value)} placeholder="Prefer..." />
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Deliverables (one per line)</label>
                  <textarea className={`${inputClass} font-mono`} rows={2} value={form.deliverables} onChange={e => set('deliverables', e.target.value)} placeholder="file.md&#10;report.yaml" />
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Acceptance Criteria (one per line)</label>
                  <textarea className={`${inputClass} font-mono`} rows={2} value={form.acceptance} onChange={e => set('acceptance', e.target.value)} placeholder="All tests pass&#10;No errors" />
                </div>

                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Task Steps</label>
                  <textarea className={`${inputClass} font-mono`} rows={5} value={form.task} onChange={e => set('task', e.target.value)} placeholder="1. First step&#10;2. Second step" />
                </div>
              </div>
            )}
          </div>

          {/* Right Column - Output */}
          <div className="lg:col-span-2 lg:sticky lg:top-4 lg:self-start">
            <div className="bg-zinc-800 rounded border border-zinc-700 overflow-hidden">
              <div className="flex items-center justify-between px-3 py-2 bg-zinc-900 border-b border-zinc-700">
                <div className="flex items-center gap-2">
                  <span className="text-xs text-zinc-400">OUTPUT</span>
                  {persona && <span className="text-xs px-2 py-0.5 bg-amber-900/30 text-amber-400 rounded">{persona}</span>}
                  <span className="text-xs px-2 py-0.5 bg-cyan-900/30 text-cyan-400 rounded">{HARNESSES[harness]?.name}</span>
                </div>
                <button onClick={copy} disabled={!output} className="px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded">
                  {copied ? '✓ COPIED' : 'COPY'}
                </button>
              </div>
              <pre className="p-3 text-xs text-zinc-300 overflow-auto max-h-[75vh] font-mono leading-relaxed">
                {output || 'Select type → configure → GENERATE'}
              </pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
