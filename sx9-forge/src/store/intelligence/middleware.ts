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
let debounceTimer: NodeJS.Timeout | null = null

export const intelligenceMiddleware: Middleware = (store) => {
  // TODO: Initialize NATS connection
  // const natsConnection = await connect({ servers: 'nats://localhost:4222' });

  return (next) => async (action) => {
    switch (action.type) {
      case intelligenceActions.LEPTOSE_CONNECT:
        console.log("[Intelligence] Connecting to Leptose...")
        // TODO: Implement actual connection
        // For now, simulate successful connection
        setTimeout(() => {
          store.dispatch({ type: intelligenceActions.LEPTOSE_CONNECTED })
        }, 100)
        break

      case intelligenceActions.CHROMADB_CONNECT:
        console.log("[Intelligence] Connecting to ChromaDB...")
        // TODO: Implement actual connection
        setTimeout(() => {
          store.dispatch({
            type: intelligenceActions.CHROMADB_CONNECTED,
            payload: { collections: ["interviews", "tools", "scenarios"] },
          })
        }, 100)
        break

      case intelligenceActions.QUERY_PATTERNS:
        // Debounce 500ms
        if (debounceTimer) clearTimeout(debounceTimer)
        debounceTimer = setTimeout(() => {
          console.log("[Intelligence] Querying patterns:", action.payload)
          store.dispatch({ type: intelligenceActions.PATTERNS_LOADING })

          // TODO: Publish to NATS: leptose.pattern.query
          // For now, simulate response
          setTimeout(() => {
            store.dispatch({
              type: intelligenceActions.PATTERNS_SUCCESS,
              payload: {
                query: action.payload.text,
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
          console.log("[Intelligence] Querying tools:", action.payload)
          store.dispatch({ type: intelligenceActions.TOOLS_LOADING })

          // TODO: Publish to NATS: leptose.tool.query
          setTimeout(() => {
            store.dispatch({
              type: intelligenceActions.TOOLS_SUCCESS,
              payload: {
                query: action.payload.text,
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
          console.log("[Intelligence] Querying threats:", action.payload)
          store.dispatch({ type: intelligenceActions.THREATS_LOADING })

          // TODO: Publish to NATS: leptose.threat.query
          setTimeout(() => {
            store.dispatch({
              type: intelligenceActions.THREATS_SUCCESS,
              payload: {
                query: action.payload.text,
                results: [],
                latencyMs: 50,
              },
            })
          }, 200)
        }, 500)
        break

      case intelligenceActions.EEI_ASK:
        console.log("[Intelligence] Asking EEI:", action.payload)
        store.dispatch({ type: intelligenceActions.EEI_LOADING })

        // TODO: Publish to NATS: eei.query
        setTimeout(() => {
          store.dispatch({
            type: intelligenceActions.EEI_ANSWER,
            payload: {
              question: action.payload.question,
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
}
