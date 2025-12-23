import { create } from "zustand"

export interface HarnessState {
  leptoseStatus: string
  chromaStatus: string
  complexity: string
  promptText: string
}

export interface PersonaState {
  personaName: string
  personaRole: string
  personaTone: string
}

export interface InferenceState {
  model: string
  temperature: number
  maxTokens: number
  topP: number
}

export interface LinearState {
  issueId: string
  issueTitle: string
  issueDescription: string
  issueStatus: string
}

export interface SlackState {
  channel: string
  message: string
  notificationsEnabled: boolean
}

export interface ContextState {
  contextPath: string
  contextFiles: string[]
  activeContext: string
}

export interface UIState {
  leftRailCollapsed: boolean
  rightRailCollapsed: boolean
  leftActiveTab: string
  rightActiveTab: string
}

export interface EditorState {
  content: string
  language: string
  isDirty: boolean
  currentFile: string | null
}

interface ForgeStore {
  // State
  harness: HarnessState
  persona: PersonaState
  inference: InferenceState
  linear: LinearState
  slack: SlackState
  context: ContextState
  ui: UIState
  editor: EditorState

  // Actions
  setHarnessField: (field: keyof HarnessState, value: string) => void
  setPersonaField: (field: keyof PersonaState, value: string) => void
  setInferenceField: (field: keyof InferenceState, value: string | number) => void
  setLinearField: (field: keyof LinearState, value: string) => void
  setSlackField: (field: keyof SlackState, value: string | boolean) => void
  setContextField: (field: keyof ContextState, value: string | string[]) => void
  setUIField: (field: keyof UIState, value: string | boolean) => void
  setEditorContent: (content: string) => void
  setEditorLanguage: (language: string) => void
  setEditorFile: (filename: string | null) => void
  markEditorClean: () => void
  resetHarness: () => void
  resetAll: () => void
}

const initialHarnessState: HarnessState = {
  leptoseStatus: "",
  chromaStatus: "",
  complexity: "",
  promptText: "",
}

const initialPersonaState: PersonaState = {
  personaName: "",
  personaRole: "",
  personaTone: "",
}

const initialInferenceState: InferenceState = {
  model: "gpt-4",
  temperature: 0.7,
  maxTokens: 2048,
  topP: 1.0,
}

const initialLinearState: LinearState = {
  issueId: "",
  issueTitle: "",
  issueDescription: "",
  issueStatus: "",
}

const initialSlackState: SlackState = {
  channel: "",
  message: "",
  notificationsEnabled: false,
}

const initialContextState: ContextState = {
  contextPath: "",
  contextFiles: [],
  activeContext: "",
}

const initialUIState: UIState = {
  leftRailCollapsed: false,
  rightRailCollapsed: false,
  leftActiveTab: "harness",
  rightActiveTab: "linear",
}

const initialEditorState: EditorState = {
  content: "# Prompt Forge\n\nStart writing your prompt here...",
  language: "markdown",
  isDirty: false,
  currentFile: null,
}

export const useForgeStore = create<ForgeStore>((set) => ({
  // Initial state
  harness: initialHarnessState,
  persona: initialPersonaState,
  inference: initialInferenceState,
  linear: initialLinearState,
  slack: initialSlackState,
  context: initialContextState,
  ui: initialUIState,
  editor: initialEditorState,

  // Actions
  setHarnessField: (field, value) =>
    set((state) => ({
      harness: { ...state.harness, [field]: value },
    })),

  setPersonaField: (field, value) =>
    set((state) => ({
      persona: { ...state.persona, [field]: value },
    })),

  setInferenceField: (field, value) =>
    set((state) => ({
      inference: { ...state.inference, [field]: value },
    })),

  setLinearField: (field, value) =>
    set((state) => ({
      linear: { ...state.linear, [field]: value },
    })),

  setSlackField: (field, value) =>
    set((state) => ({
      slack: { ...state.slack, [field]: value },
    })),

  setContextField: (field, value) =>
    set((state) => ({
      context: { ...state.context, [field]: value },
    })),

  setUIField: (field, value) =>
    set((state) => ({
      ui: { ...state.ui, [field]: value },
    })),

  setEditorContent: (content) =>
    set((state) => ({
      editor: { ...state.editor, content, isDirty: true },
    })),

  setEditorLanguage: (language) =>
    set((state) => ({
      editor: { ...state.editor, language },
    })),

  setEditorFile: (currentFile) =>
    set((state) => ({
      editor: { ...state.editor, currentFile, isDirty: false },
    })),

  markEditorClean: () =>
    set((state) => ({
      editor: { ...state.editor, isDirty: false },
    })),

  resetHarness: () =>
    set({
      harness: initialHarnessState,
    }),

  resetAll: () =>
    set({
      harness: initialHarnessState,
      persona: initialPersonaState,
      inference: initialInferenceState,
      linear: initialLinearState,
      slack: initialSlackState,
      context: initialContextState,
      editor: initialEditorState,
    }),
}))
