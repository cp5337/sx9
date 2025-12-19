import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Search, Folder, Tag } from "lucide-react";

interface RfcMeta {
  id: string;
  title: string;
  status: string;
  category: string;
  path: string;
  dependencies: string[];
  implementations: string[];
  tags: string[];
}

interface RfcIndex {
  rfcs: Record<string, RfcMeta>;
  categories: Record<string, string[]>;
  tags: Record<string, string[]>;
}

export default function RfcBrowser() {
  const [index, setIndex] = useState<RfcIndex | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selected, setSelected] = useState<RfcMeta | null>(null);
  const [basePath, setBasePath] = useState(
    "/Users/cp5337/Developer/sx9/01-rfc"
  );
  const [pathSet, setPathSet] = useState(false);

  const loadIndex = async () => {
    setLoading(true);
    setError(null);
    try {
      await invoke("set_rfc_base_path", { path: basePath });
      setPathSet(true);
      const result = await invoke<RfcIndex>("load_rfc_index");
      setIndex(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const searchRfcs = async () => {
    if (!search.trim()) {
      loadIndex();
      return;
    }
    setLoading(true);
    try {
      const results = await invoke<RfcMeta[]>("search_rfcs", { query: search });
      setIndex({
        rfcs: Object.fromEntries(results.map((r) => [r.id, r])),
        categories: {},
        tags: {},
      });
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const rfcList = index ? Object.values(index.rfcs) : [];
  const filteredRfcs = selectedCategory
    ? rfcList.filter((r) => r.category === selectedCategory)
    : rfcList;
  const categories = index ? Object.keys(index.categories) : [];

  const statusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case "active":
        return "bg-emerald-900/30 text-emerald-400";
      case "implemented":
        return "bg-cyan-900/30 text-cyan-400";
      case "draft":
        return "bg-amber-900/30 text-amber-400";
      case "deprecated":
        return "bg-red-900/30 text-red-400";
      default:
        return "bg-zinc-700 text-zinc-400";
    }
  };

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-emerald-400">RFC Browser</h1>
        <p className="text-zinc-500">Search and browse SX9 specifications</p>
      </div>

      {/* Path Configuration */}
      {!pathSet && (
        <div className="bg-zinc-800 rounded-lg p-4 border border-zinc-700 mb-6">
          <label className="block text-xs text-zinc-500 mb-2">
            RFC Directory Path
          </label>
          <div className="flex gap-2">
            <input
              className="flex-1 bg-zinc-900 border border-zinc-700 rounded px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none"
              value={basePath}
              onChange={(e) => setBasePath(e.target.value)}
              placeholder="/path/to/rfc/directory"
            />
            <button
              onClick={loadIndex}
              disabled={loading}
              className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 rounded text-sm"
            >
              {loading ? "Loading..." : "Load RFCs"}
            </button>
          </div>
        </div>
      )}

      {error && (
        <div className="bg-red-900/20 border border-red-700 rounded-lg p-4 mb-6 text-red-400">
          {error}
        </div>
      )}

      {pathSet && (
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Sidebar */}
          <div className="space-y-4">
            {/* Search */}
            <div className="bg-zinc-800 rounded-lg p-3 border border-zinc-700">
              <div className="flex gap-2">
                <input
                  className="flex-1 bg-zinc-900 border border-zinc-700 rounded px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && searchRfcs()}
                  placeholder="Search RFCs..."
                />
                <button
                  onClick={searchRfcs}
                  className="px-3 py-2 bg-zinc-700 hover:bg-zinc-600 rounded text-sm"
                >
                  <Search className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Categories */}
            <div className="bg-zinc-800 rounded-lg border border-zinc-700 overflow-hidden">
              <div className="px-3 py-2 bg-zinc-900 border-b border-zinc-700 text-xs text-zinc-500">
                Categories
              </div>
              <div className="p-2">
                <button
                  onClick={() => setSelectedCategory(null)}
                  className={`w-full text-left px-3 py-2 rounded text-sm ${
                    !selectedCategory
                      ? "bg-emerald-900/20 text-emerald-400"
                      : "text-zinc-400 hover:bg-zinc-700"
                  }`}
                >
                  All ({rfcList.length})
                </button>
                {categories.map((cat) => (
                  <button
                    key={cat}
                    onClick={() => setSelectedCategory(cat)}
                    className={`w-full text-left px-3 py-2 rounded text-sm ${
                      selectedCategory === cat
                        ? "bg-emerald-900/20 text-emerald-400"
                        : "text-zinc-400 hover:bg-zinc-700"
                    }`}
                  >
                    {cat} ({index?.categories[cat]?.length || 0})
                  </button>
                ))}
              </div>
            </div>

            {/* Reload */}
            <button
              onClick={loadIndex}
              className="w-full px-3 py-2 bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 rounded text-sm text-zinc-400"
            >
              ↻ Reload Index
            </button>
          </div>

          {/* RFC List */}
          <div className="lg:col-span-2 space-y-2">
            {loading ? (
              <div className="text-center text-zinc-500 py-8">
                Loading RFCs...
              </div>
            ) : filteredRfcs.length === 0 ? (
              <div className="text-center text-zinc-500 py-8">
                <div className="text-4xl mb-2">📭</div>
                <div>No RFCs found</div>
              </div>
            ) : (
              filteredRfcs.map((rfc) => (
                <div
                  key={rfc.id}
                  onClick={() => setSelected(rfc)}
                  className={`bg-zinc-800 rounded-lg p-4 border cursor-pointer transition-all ${
                    selected?.id === rfc.id
                      ? "border-emerald-500"
                      : "border-zinc-700 hover:border-zinc-600"
                  }`}
                >
                  <div className="flex justify-between items-start mb-1">
                    <div className="font-mono text-sm text-emerald-400">
                      {rfc.id}
                    </div>
                    <span
                      className={`px-2 py-0.5 rounded text-xs ${statusColor(rfc.status)}`}
                    >
                      {rfc.status}
                    </span>
                  </div>
                  <div className="font-medium mb-1">{rfc.title}</div>
                  <div className="flex gap-2 text-xs text-zinc-500">
                    <Folder className="w-3 h-3 inline" /> {rfc.category}
                      <Tag className="w-3 h-3 inline" /> {rfc.tags.slice(0, 3).join(", ")}
                  </div>
                </div>
              ))
            )}
          </div>

          {/* RFC Detail */}
          <div className="lg:sticky lg:top-4 lg:self-start">
            {selected ? (
              <div className="bg-zinc-800 rounded-lg border border-zinc-700 overflow-hidden">
                <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700">
                  <div className="font-mono text-emerald-400">
                    {selected.id}
                  </div>
                  <div className="font-medium">{selected.title}</div>
                </div>
                <div className="p-4 space-y-4">
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <div className="text-xs text-zinc-500 mb-1">Status</div>
                      <span
                        className={`px-2 py-1 rounded text-sm ${statusColor(selected.status)}`}
                      >
                        {selected.status}
                      </span>
                    </div>
                    <div>
                      <div className="text-xs text-zinc-500 mb-1">Category</div>
                      <div className="text-sm">{selected.category}</div>
                    </div>
                  </div>

                  {selected.tags.length > 0 && (
                    <div>
                      <div className="text-xs text-zinc-500 mb-2">Tags</div>
                      <div className="flex flex-wrap gap-1">
                        {selected.tags.map((tag) => (
                          <span
                            key={tag}
                            className="px-2 py-0.5 bg-zinc-700 rounded text-xs"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {selected.dependencies.length > 0 && (
                    <div>
                      <div className="text-xs text-zinc-500 mb-2">
                        Dependencies
                      </div>
                      <div className="space-y-1">
                        {selected.dependencies.map((dep) => (
                          <div key={dep} className="text-xs text-cyan-400">
                            {dep}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {selected.implementations.length > 0 && (
                    <div>
                      <div className="text-xs text-zinc-500 mb-2">
                        Implementations
                      </div>
                      <div className="space-y-1">
                        {selected.implementations.map((impl) => (
                          <div key={impl} className="text-xs text-emerald-400">
                            {impl}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Path</div>
                    <div className="text-xs text-zinc-400 font-mono break-all">
                      {selected.path}
                    </div>
                  </div>
                </div>
              </div>
            ) : (
              <div className="bg-zinc-800 rounded-lg border border-zinc-700 p-8 text-center text-zinc-500">
                <div className="text-4xl mb-2">📄</div>
                <div>Select an RFC to view details</div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
