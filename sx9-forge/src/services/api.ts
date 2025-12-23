export interface APIResponse<T> {
  success: boolean
  data?: T
  error?: string
}

export interface LinearIssue {
  id: string
  title: string
  description: string
  status: string
  assignee?: string
  labels?: string[]
}

export interface SlackNotification {
  channel: string
  message: string
  timestamp?: string
}

export interface ContextFile {
  path: string
  content: string
  lastModified: string
}

import { cx9API } from "../lib/tauri"

class APIService {
  private baseURL: string

  constructor() {
    this.baseURL = import.meta.env.VITE_API_URL || "http://localhost:3000"
  }

  // File System Operations
  async savePrompt(content: string, filename: string): Promise<APIResponse<string>> {
    try {
      if (cx9API.isTauri()) {
        const path = await cx9API.saveToDisk(content, filename)
        return { success: true, data: path }
      }

      const response = await fetch(`${this.baseURL}/api/fs/save`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ content, filename }),
      })
      const data = await response.json()
      return { success: true, data: data.path }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  async loadPrompt(filename: string): Promise<APIResponse<string>> {
    try {
      if (cx9API.isTauri()) {
        const content = await cx9API.loadFromDisk(filename)
        return { success: true, data: content }
      }

      const response = await fetch(`${this.baseURL}/api/fs/load/${filename}`)
      const data = await response.json()
      return { success: true, data: data.content }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  async listPrompts(): Promise<APIResponse<string[]>> {
    try {
      if (cx9API.isTauri()) {
        const files = await cx9API.listFiles("prompts")
        return { success: true, data: files }
      }

      const response = await fetch(`${this.baseURL}/api/fs/list`)
      const data = await response.json()
      return { success: true, data: data.files }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  // Linear Integration
  async fetchLinearIssue(issueId: string): Promise<APIResponse<LinearIssue>> {
    try {
      const response = await fetch(`${this.baseURL}/api/linear/issue/${issueId}`)
      const data = await response.json()
      return { success: true, data }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  async createLinearIssue(title: string, description: string, teamId?: string): Promise<APIResponse<LinearIssue>> {
    try {
      if (cx9API.isTauri()) {
        const apiKey = import.meta.env.VITE_LINEAR_API_KEY || ""
        const team = teamId || import.meta.env.VITE_LINEAR_TEAM_ID || ""
        const result = await cx9API.createLinearIssue(apiKey, team, title, description)
        return { success: true, data: result.data.issueCreate.issue }
      }

      const response = await fetch(`${this.baseURL}/api/linear/issue`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title, description, teamId }),
      })
      const data = await response.json()
      return { success: true, data }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  async updateLinearIssue(issueId: string, updates: Partial<LinearIssue>): Promise<APIResponse<LinearIssue>> {
    try {
      const response = await fetch(`${this.baseURL}/api/linear/issue/${issueId}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(updates),
      })
      const data = await response.json()
      return { success: true, data }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  // Slack Integration
  async sendSlackNotification(channel: string, message: string): Promise<APIResponse<SlackNotification>> {
    try {
      if (cx9API.isTauri()) {
        const webhookUrl = import.meta.env.VITE_SLACK_WEBHOOK_URL || ""
        await cx9API.sendSlackNotification(webhookUrl, channel, message)
        return { success: true, data: { channel, message } }
      }

      const response = await fetch(`${this.baseURL}/api/slack/notify`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ channel, message }),
      })
      const data = await response.json()
      return { success: true, data }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  // Context Management
  async loadContext(contextPath: string): Promise<APIResponse<ContextFile[]>> {
    try {
      const response = await fetch(`${this.baseURL}/api/context/load`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ path: contextPath }),
      })
      const data = await response.json()
      return { success: true, data: data.files }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }

  // Harness Execution
  async executeHarness(config: {
    leptoseStatus: string
    chromaStatus: string
    complexity: string
    promptText: string
  }): Promise<APIResponse<{ result: string; executionTime: number }>> {
    try {
      const response = await fetch(`${this.baseURL}/api/harness/execute`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      })
      const data = await response.json()
      return { success: true, data }
    } catch (error) {
      return { success: false, error: (error as Error).message }
    }
  }
}

export const api = new APIService()
