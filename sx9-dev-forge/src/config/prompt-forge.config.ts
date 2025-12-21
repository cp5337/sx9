import {
  Wrench,
  Shield,
  Target,
  FileCode,
  ClipboardList,
  Microscope,
  FileText,
  Settings,
  Rocket,
  Search,
  Globe,
  Monitor,
  Brain,
  Folder,
  Layout,
  Image,
} from "lucide-react";

export const PROMPT_TYPES = {
  BUILD_PIPELINE: {
    name: "Build Pipeline",
    icon: Wrench,
    persona: "FORGE",
    phase: "IMPLEMENT",
  },
  SECURITY_AUDIT: {
    name: "Security Audit",
    icon: Shield,
    persona: "VECTOR",
    phase: "ANALYZE",
  },
  THREAT_EMULATION: {
    name: "Threat Emulation",
    icon: Target,
    persona: "SENTINEL",
    phase: "IMPLEMENT",
  },
  CODE_GENERATION: {
    name: "Code Gen",
    icon: FileCode,
    persona: "AXIOM",
    phase: "IMPLEMENT",
  },
  RFC_ALIGNMENT: {
    name: "RFC Alignment",
    icon: ClipboardList,
    persona: "FORGE",
    phase: "WALK",
  },
  RESEARCH: {
    name: "Research",
    icon: Microscope,
    persona: "SENTINEL",
    phase: "RESEARCH",
  },
  DOCS: {
    name: "Documentation",
    icon: FileText,
    persona: "SCRIBE",
    phase: "WALK",
  },
  CUSTOM: { name: "Custom", icon: Settings, persona: "", phase: "PLAN" },
};

export const HARNESSES = {
  full_autonomous: {
    name: "Full Autonomous",
    desc: "0.8 • All Tools",
    Icon: Rocket,
    mode: "AUTONOMOUS",
    temp: "0.8",
    timeout: "120m",
  },
  research: {
    name: "Research & Intel",
    desc: "0.4 • Web Only",
    Icon: Microscope,
    mode: "AUTONOMOUS",
    temp: "0.4",
    timeout: "60m",
  },
  build: {
    name: "Build & Implement",
    desc: "0.1 • Filesystem",
    Icon: Wrench,
    mode: "SUPERVISED",
    temp: "0.1",
    timeout: "90m",
  },
  security: {
    name: "Security Audit",
    desc: "0.0 • Read-Only",
    Icon: Shield,
    mode: "STEP-CONFIRM",
    temp: "0.0",
    timeout: "120m",
  },
  planning: {
    name: "Planning & Docs",
    desc: "0.3 • Synthesis",
    Icon: ClipboardList,
    mode: "SUPERVISED",
    temp: "0.3",
    timeout: "60m",
  },
};

export const PERSONAS = {
  FORGE: { name: "FORGE", role: "Build Engineer", color: "#10b981" },
  AXIOM: { name: "AXIOM", role: "Systems Architect", color: "#8b5cf6" },
  VECTOR: { name: "VECTOR", role: "Security Ops", color: "#ef4444" },
  SENTINEL: { name: "SENTINEL", role: "Threat Analyst", color: "#f59e0b" },
  NEXUS: { name: "NEXUS", role: "Integration", color: "#0ea5e9" },
  SCRIBE: { name: "SCRIBE", role: "Documentation", color: "#10b981" },
};

export const TOOLS_LIST = [
  { id: "web_search", label: "Web Search", Icon: Search },
  { id: "web_fetch", label: "Web Fetch", Icon: Globe },
  { id: "computer_use", label: "Computer", Icon: Monitor },
  { id: "memory", label: "Memory", Icon: Brain },
  { id: "linear", label: "Linear", Icon: ClipboardList },
  { id: "filesystem", label: "Filesystem", Icon: Folder },
  { id: "canva", label: "Canva", Icon: Layout },
  { id: "figma", label: "Figma", Icon: Image },
];

export const CONTEXT_SOURCES = [
  { id: "ctx_memory", label: "Conversation Memory", Icon: Brain },
  { id: "ctx_linear", label: "Linear Issues", Icon: ClipboardList },
  { id: "ctx_drive", label: "Google Drive", Icon: FileText },
  { id: "ctx_web", label: "Web Search", Icon: Search },
];

export const RECENT_MISSIONS = [
  {
    id: 1,
    title: "Refactor Auth Pipeline",
    time: "2h ago",
    status: "COMPLETE",
  },
  {
    id: 2,
    title: "Audit API Endpoints",
    time: "5h ago",
    status: "IN_PROGRESS",
  },
  { id: 3, title: "Update Design Tokens", time: "1d ago", status: "FAILED" },
];

export const DEFAULT_FORM_STATE = {
  title: "Migrate Session Auth to Supabase",
  rfc: "RFC-9102",
  phase: "IMPLEMENT",
  mode: "SUPERVISED",
  priority: "P1",
  classification: "INTERNAL",
  timeout: "90m",
  objective:
    "Replace current Bolt session management with Supabase Auth RFC standard.",
  context:
    "Current auth uses deprecated Bolt patterns. We need to align with RFC-9102.",
  linearTeam: "SX9",
  linearProject: "AUTH-V2",
  linearLabels: "type:feature, priority:high",
  createLinearIssue: true,
  notifySlack: false,
  slackChannel: "#builds",
  hardConstraints: "DO NOT modify src/legacy\nDO NOT commit secrets",
  softConstraints: "Prefer functional patterns\nDocument all changes",
  deliverables: "Auth Service\nMigration Script",
  acceptance: "All tests pass\nNo regressions",
  workdir: "./",
  forbidPaths: ".env, secrets/, .git/config",
  // Tools defaults
  web_search: true,
  web_fetch: true,
  computer_use: true,
  memory: true,
  linear: true,
  filesystem: true,
  canva: false,
  figma: false,
  // Context defaults
  ctx_memory: true,
  ctx_linear: true,
  ctx_drive: true,
  ctx_web: false,
};
