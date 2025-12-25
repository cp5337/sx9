/**
 * Redux Intelligence Middleware
 *
 * NATS middleware for background intelligence queries.
 * Handles debouncing, connection management, and response subscriptions.
 *
 * TODO: Implement actual NATS connection when backend is ready.
 * For now, this is a placeholder that logs actions.
 */

import type { Middleware } from "@reduxjs/toolkit"
import { intelligenceActions } from "./actions"

// Placeholder - will be replaced with actual NATS connection
let debounceTimer: ReturnType<typeof setTimeout> | null = null

export const intelligenceMiddleware: Middleware = (store) => (next) => (action) => {
  const typedAction = action as { type: string; payload?: any }

  switch (typedAction.type) {
    case intelligenceActions.LEPTOSE_CONNECT:
      console.log("[Intelligence] Connecting to Leptose...")
      setTimeout(() => {
        store.dispatch({ type: intelligenceActions.LEPTOSE_CONNECTED })
      }, 100)
      break

    case intelligenceActions.CHROMADB_CONNECT:
      console.log("[Intelligence] Connecting to ChromaDB...")
      setTimeout(() => {
        store.dispatch({
          type: intelligenceActions.CHROMADB_CONNECTED,
          payload: { collections: ["interviews", "tools", "scenarios"] },
        })
      }, 100)
      break

    case intelligenceActions.QUERY_PATTERNS:
      if (debounceTimer) clearTimeout(debounceTimer)
      debounceTimer = setTimeout(() => {
        console.log("[Intelligence] Querying patterns:", typedAction.payload)
        store.dispatch({ type: intelligenceActions.PATTERNS_LOADING })
        setTimeout(() => {
          store.dispatch({
            type: intelligenceActions.PATTERNS_SUCCESS,
            payload: {
              query: typedAction.payload?.text,
              results: [],
              latencyMs: 50,
            },
          })
        }, 200)
      }, 500)
      break

    case intelligenceActions.QUERY_TOOLS:
      if (debounceTimer) clearTimeout(debounceTimer)
      debounceTimer = setTimeout(() => {
        console.log("[Intelligence] Querying tools:", typedAction.payload)
        store.dispatch({ type: intelligenceActions.TOOLS_LOADING })
        setTimeout(() => {
          store.dispatch({
            type: intelligenceActions.TOOLS_SUCCESS,
            payload: {
              query: typedAction.payload?.text,
              results: [],
              latencyMs: 50,
            },
          })
        }, 200)
      }, 500)
      break

    case intelligenceActions.QUERY_THREATS:
      if (debounceTimer) clearTimeout(debounceTimer)
      debounceTimer = setTimeout(() => {
        console.log("[Intelligence] Querying threats:", typedAction.payload)
        store.dispatch({ type: intelligenceActions.THREATS_LOADING })
        setTimeout(() => {
          store.dispatch({
            type: intelligenceActions.THREATS_SUCCESS,
            payload: {
              query: typedAction.payload?.text,
              results: [],
              latencyMs: 50,
            },
          })
        }, 200)
      }, 500)
      break

    case intelligenceActions.EEI_ASK:
      console.log("[Intelligence] Asking EEI:", typedAction.payload)
      store.dispatch({ type: intelligenceActions.EEI_LOADING })
      setTimeout(() => {
        store.dispatch({
          type: intelligenceActions.EEI_ANSWER,
          payload: {
            question: typedAction.payload?.question,
            answer: {
              answer: "Placeholder answer - NATS not connected",
              confidence: 0.0,
              sources: [],
              graph_path: [],
              timestamp: Date.now(),
            },
            latencyMs: 50,
          },
        })
      }, 200)
      break
  }

  return next(action)
}
