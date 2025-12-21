import { useState } from "react";
import {
  useAtomicClipboard,
  ClipboardEntry,
} from "../hooks/useAtomicClipboard";
import {
  Copy,
  Trash2,
  Search,
  Filter,
  Zap,
  FileText,
  Target,
  List as ListIcon,
} from "lucide-react";

export function ClipboardPanel() {
  const { entries, loading, stats, push, searchByTag, searchBySource, clear } =
    useAtomicClipboard();
  const [filter, setFilter] = useState<
    "all" | "mission" | "linear" | "voice" | "manual"
  >("all");
  const [searchTerm, setSearchTerm] = useState("");

  const filteredEntries = entries.filter((entry) => {
    const matchesFilter = filter === "all" || entry.source === filter;
    const matchesSearch =
      searchTerm === "" ||
      entry.content.toLowerCase().includes(searchTerm.toLowerCase()) ||
      entry.tags.some((tag) =>
        tag.toLowerCase().includes(searchTerm.toLowerCase())
      );
    return matchesFilter && matchesSearch;
  });

  const getSourceIcon = (source: string) => {
    switch (source) {
      case "mission":
        return <Target className="w-4 h-4" />;
      case "linear":
        return <ListIcon className="w-4 h-4" />;
      case "voice":
        return <Zap className="w-4 h-4" />;
      default:
        return <FileText className="w-4 h-4" />;
    }
  };

  const getSourceColor = (source: string) => {
    switch (source) {
      case "mission":
        return "text-emerald-400";
      case "linear":
        return "text-blue-400";
      case "voice":
        return "text-purple-400";
      default:
        return "text-gray-400";
    }
  };

  const copyToClipboard = (content: string) => {
    navigator.clipboard.writeText(content);
  };

  const sendToAI = (entry: ClipboardEntry) => {
    // TODO: Integrate with AI prompt window
    console.log("Send to AI:", entry);
    copyToClipboard(entry.content);
  };

  return (
    <div className="flex flex-col h-full bg-zinc-900 border border-zinc-700 rounded-lg">
      {/* Header */}
      <div className="p-4 border-b border-zinc-700">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold text-white flex items-center gap-2">
            <Copy className="w-5 h-5 text-emerald-400" />
            Atomic Clipboard
          </h2>
          <div className="text-sm text-gray-400">
            {stats?.total_entries || 0} entries
          </div>
        </div>

        {/* Search */}
        <div className="relative mb-3">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
          <input
            type="text"
            placeholder="Search content or tags..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-zinc-800 border border-zinc-600 rounded-lg text-white text-sm focus:outline-none focus:border-emerald-500"
          />
        </div>

        {/* Filters */}
        <div className="flex gap-2 flex-wrap">
          {["all", "mission", "linear", "voice", "manual"].map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f as any)}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition-colors ${
                filter === f
                  ? "bg-emerald-600 text-white"
                  : "bg-zinc-800 text-gray-400 hover:bg-zinc-700"
              }`}
            >
              {f.charAt(0).toUpperCase() + f.slice(1)}
              {f !== "all" && stats?.sources[f] && (
                <span className="ml-1 text-xs opacity-75">
                  ({stats.sources[f]})
                </span>
              )}
            </button>
          ))}
        </div>
      </div>

      {/* Entries List */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {loading ? (
          <div className="text-center text-gray-400 py-8">Loading...</div>
        ) : filteredEntries.length === 0 ? (
          <div className="text-center text-gray-400 py-8">
            {searchTerm || filter !== "all"
              ? "No matching entries"
              : "No clipboard entries yet"}
          </div>
        ) : (
          filteredEntries.map((entry) => (
            <div
              key={entry.id}
              className="bg-zinc-800 border border-zinc-700 rounded-lg p-3 hover:border-zinc-600 transition-colors"
            >
              {/* Entry Header */}
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center gap-2">
                  <span className={getSourceColor(entry.source)}>
                    {getSourceIcon(entry.source)}
                  </span>
                  <span className="text-xs text-gray-400">
                    {new Date(entry.created_at).toLocaleString()}
                  </span>
                </div>
                <div className="flex gap-1">
                  <button
                    onClick={() => sendToAI(entry)}
                    className="p-1 hover:bg-zinc-700 rounded transition-colors"
                    title="Send to AI"
                  >
                    <Zap className="w-4 h-4 text-purple-400" />
                  </button>
                  <button
                    onClick={() => copyToClipboard(entry.content)}
                    className="p-1 hover:bg-zinc-700 rounded transition-colors"
                    title="Copy"
                  >
                    <Copy className="w-4 h-4 text-gray-400" />
                  </button>
                </div>
              </div>

              {/* Content */}
              <div className="text-sm text-white mb-2 line-clamp-3">
                {entry.content}
              </div>

              {/* Tags */}
              {entry.tags.length > 0 && (
                <div className="flex gap-1 flex-wrap">
                  {entry.tags.map((tag, idx) => (
                    <span
                      key={idx}
                      className="px-2 py-0.5 bg-zinc-700 text-gray-300 text-xs rounded"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </div>
          ))
        )}
      </div>

      {/* Footer */}
      <div className="p-3 border-t border-zinc-700 flex justify-between items-center">
        <div className="text-xs text-gray-400">
          Showing {filteredEntries.length} of {entries.length}
        </div>
        <button
          onClick={() => clear()}
          className="flex items-center gap-1 px-3 py-1 bg-red-600/20 hover:bg-red-600/30 text-red-400 rounded-lg text-xs transition-colors"
        >
          <Trash2 className="w-3 h-3" />
          Clear All
        </button>
      </div>
    </div>
  );
}
