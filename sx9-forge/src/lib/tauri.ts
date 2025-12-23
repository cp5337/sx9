import { invoke } from "@tauri-apps/api/core"

export const cx9API = {
  // File System Operations
  saveToDisk: async (content: string, filename: string): Promise<string> => {
    return await invoke("save_to_disk", { content, filename })
  },

  loadFromDisk: async (filename: string): Promise<string> => {
    return await invoke("load_from_disk", { filename })
  },

  listFiles: async (directory: string): Promise<string[]> => {
    return await invoke("list_files", { directory })
  },

  // System Info
  getSystemInfo: async (): Promise<SystemInfo> => {
    return await invoke("get_system_info")
  },

  // Prompt Forge Operations
  executeRustPattern: async (patternName: string, input: string): Promise<string> => {
    return await invoke("execute_rust_pattern", { patternName, input })
  },

  runQAGate: async (gateName: string, data: any): Promise<QAGateResult> => {
    return await invoke("run_qa_gate", { gateName, data })
  },

  createLinearIssue: async (apiKey: string, teamId: string, title: string, description: string): Promise<any> => {
    return await invoke("create_linear_issue", { apiKey, teamId, title, description })
  },

  sendSlackNotification: async (webhookUrl: string, channel: string, message: string): Promise<string> => {
    return await invoke("send_slack_notification", { webhookUrl, channel, message })
  },

  // Helper
  isTauri: () => {
    return typeof window !== "undefined" && "__TAURI__" in window
  },
}

export type SystemInfo = {
  platform: string
  arch: string
  version: string
}

export type QAGateResult = {
  passed: boolean
  message: string
  details?: any
}
