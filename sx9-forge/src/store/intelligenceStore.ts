import { create } from "zustand"
import type { IntelligenceState } from "./intelligence/types"
import { intelligenceReducer } from "./intelligence/reducer"

/**
 * Zustand wrapper for Redux-style intelligence reducer
 * Provides the same API as Redux but with Zustand's simpler setup
 */

interface IntelligenceStore extends IntelligenceState {
  dispatch: (action: any) => void
}

export const useIntelligenceStore = create<IntelligenceStore>((set, get) => ({
  // Initial state from reducer
  ...intelligenceReducer(undefined, { type: "@@INIT" }),

  // Dispatch function that mimics Redux
  dispatch: (action: any) => {
    const currentState = {
      leptose: get().leptose,
      chromadb: get().chromadb,
      patterns: get().patterns,
      tools: get().tools,
      threats: get().threats,
      eei: get().eei,
    }

    const newState = intelligenceReducer(currentState, action)
    set(newState)
  },
}))

/**
 * Hook to use intelligence actions with automatic dispatch
 */
export function useIntelligenceActions() {
  const dispatch = useIntelligenceStore((state) => state.dispatch)
  return { dispatch }
}
