import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

export interface ClipboardEntry {
  id: string;
  content: string;
  source: string;
  tags: string[];
  created_at: string;
  metadata: any;
}

export interface ClipboardStats {
  total_entries: number;
  sources: Record<string, number>;
  tags: Record<string, number>;
  oldest_entry: string | null;
  newest_entry: string | null;
}

export function useAtomicClipboard() {
  const [entries, setEntries] = useState<ClipboardEntry[]>([]);
  const [loading, setLoading] = useState(false);
  const [stats, setStats] = useState<ClipboardStats | null>(null);

  const push = async (
    content: string,
    source: string,
    tags: string[] = [],
    metadata: any = {}
  ): Promise<string> => {
    return await invoke<string>("clipboard_push", {
      content,
      source,
      tags,
      metadata,
    });
  };

  const list = async (limit: number = 50) => {
    setLoading(true);
    try {
      const result = await invoke<ClipboardEntry[]>("clipboard_list", {
        limit,
      });
      setEntries(result);
      return result;
    } finally {
      setLoading(false);
    }
  };

  const get = async (id: string) => {
    return await invoke<ClipboardEntry | null>("clipboard_get", { id });
  };

  const searchByTag = async (tag: string) => {
    return await invoke<ClipboardEntry[]>("clipboard_search_by_tag", { tag });
  };

  const searchBySource = async (source: string) => {
    return await invoke<ClipboardEntry[]>("clipboard_search_by_source", {
      source,
    });
  };

  const loadStats = async () => {
    const result = await invoke<ClipboardStats>("clipboard_stats");
    setStats(result);
    return result;
  };

  const clear = async () => {
    await invoke("clipboard_clear");
    setEntries([]);
    setStats(null);
  };

  // Auto-load on mount
  useEffect(() => {
    list();
    loadStats();
  }, []);

  return {
    entries,
    loading,
    stats,
    push,
    list,
    get,
    searchByTag,
    searchBySource,
    loadStats,
    clear,
  };
}
