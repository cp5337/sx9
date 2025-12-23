
import { useState, useCallback } from "react"
import { api } from "../services/api"
import { cx9API } from "../lib/tauri"

export function useFileSystem() {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const savePrompt = useCallback(async (content: string, filename: string) => {
    setLoading(true)
    setError(null)
    try {
      // Try Tauri first, fallback to web API
      if (cx9API.isTauri()) {
        const path = await cx9API.saveToDisk(content, filename)
        return { success: true, data: path }
      } else {
        return await api.savePrompt(content, filename)
      }
    } catch (err) {
      const errorMsg = (err as Error).message
      setError(errorMsg)
      return { success: false, error: errorMsg }
    } finally {
      setLoading(false)
    }
  }, [])

  const loadPrompt = useCallback(async (filename: string) => {
    setLoading(true)
    setError(null)
    try {
      if (cx9API.isTauri()) {
        const content = await cx9API.loadFromDisk(filename)
        return { success: true, data: content }
      } else {
        return await api.loadPrompt(filename)
      }
    } catch (err) {
      const errorMsg = (err as Error).message
      setError(errorMsg)
      return { success: false, error: errorMsg }
    } finally {
      setLoading(false)
    }
  }, [])

  return { savePrompt, loadPrompt, loading, error }
}

export function useLinear() {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchIssue = useCallback(async (issueId: string) => {
    setLoading(true)
    setError(null)
    try {
      return await api.fetchLinearIssue(issueId)
    } catch (err) {
      const errorMsg = (err as Error).message
      setError(errorMsg)
      return { success: false, error: errorMsg }
    } finally {
      setLoading(false)
    }
  }, [])

  const createIssue = useCallback(async (title: string, description: string) => {
    setLoading(true)
    setError(null)
    try {
      return await api.createLinearIssue(title, description)
    } catch (err) {
      const errorMsg = (err as Error).message
      setError(errorMsg)
      return { success: false, error: errorMsg }
    } finally {
      setLoading(false)
    }
  }, [])

  return { fetchIssue, createIssue, loading, error }
}

export function useSlack() {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const sendNotification = useCallback(async (channel: string, message: string) => {
    setLoading(true)
    setError(null)
    try {
      return await api.sendSlackNotification(channel, message)
    } catch (err) {
      const errorMsg = (err as Error).message
      setError(errorMsg)
      return { success: false, error: errorMsg }
    } finally {
      setLoading(false)
    }
  }, [])

  return { sendNotification, loading, error }
}
