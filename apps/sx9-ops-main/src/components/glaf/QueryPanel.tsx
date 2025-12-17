import { useState } from "react";
import { Play, Save, Trash2, Database, AlertCircle } from "lucide-react";

interface SavedQuery {
  id: string;
  name: string;
  query: string;
}

interface QueryPanelProps {
  onExecuteQuery: (query: string) => void;
  savedQueries: SavedQuery[];
  onSaveQuery: (name: string, query: string) => void;
  onDeleteQuery: (queryId: string) => void;
  queryResults?: any[];
  queryError?: string | null;
  isExecuting?: boolean;
}

export default function QueryPanel({
  onExecuteQuery,
  savedQueries,
  onSaveQuery,
  onDeleteQuery,
  queryResults,
  queryError,
  isExecuting,
}: QueryPanelProps) {
  const [query, setQuery] = useState("");
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [queryName, setQueryName] = useState("");

  const handleExecute = () => {
    if (query.trim()) {
      onExecuteQuery(query);
    }
  };

  const handleSave = () => {
    if (queryName.trim() && query.trim()) {
      onSaveQuery(queryName, query);
      setQueryName("");
      setShowSaveDialog(false);
    }
  };

  const loadQuery = (savedQuery: SavedQuery) => {
    setQuery(savedQuery.query);
  };

  const commonQueries = [
    { name: "All Nodes", query: "MATCH (n) RETURN n" },
    { name: "All Relationships", query: "MATCH (n)-[r]->(m) RETURN n, r, m" },
    {
      name: "Node Count by Label",
      query: "MATCH (n) RETURN n.label, COUNT(n) as count",
    },
    {
      name: "Find Paths",
      query: "MATCH p=(n)-[*1..3]->(m) WHERE n.label = 'Person' RETURN p",
    },
  ];

  return (
    <div className="h-full flex flex-col bg-dark-surface border-l border-dark-border">
      <div className="p-4 border-b border-dark-border">
        <h2 className="text-lg font-bold text-dark-text-primary flex items-center gap-2">
          <Database size={20} />
          Query Interface
        </h2>
        <p className="text-sm text-dark-text-secondary mt-1">
          Write queries to explore your graph data
        </p>
      </div>

      <div className="flex-1 flex flex-col overflow-hidden">
        <div className="p-4 flex-none flex flex-col h-1/2 border-b border-dark-border">
          <label className="block text-sm font-semibold text-dark-text-primary mb-2">
            Query Editor
          </label>
          <textarea
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="flex-1 px-3 py-2 border border-dark-border bg-dark-elevated text-dark-text-primary rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm resize-none"
            placeholder="MATCH (n) RETURN n LIMIT 25"
          />
          <div className="flex gap-2 mt-3">
            <button
              onClick={handleExecute}
              disabled={isExecuting}
              className={`flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors font-medium ${
                isExecuting ? "opacity-50 cursor-wait" : ""
              }`}
            >
              <Play size={16} />
              {isExecuting ? "Executing..." : "Execute"}
            </button>
            <button
              onClick={() => setShowSaveDialog(true)}
              className="flex items-center gap-2 px-4 py-2 bg-dark-elevated text-dark-text-primary rounded-lg hover:bg-dark-bg transition-colors font-medium"
            >
              <Save size={16} />
              Save
            </button>
          </div>
        </div>

        {/* Results Pane */}
        <div className="flex-1 flex flex-col overflow-hidden bg-gray-900">
          <div className="px-4 py-2 bg-dark-elevated border-b border-dark-border flex justify-between items-center">
            <h3 className="text-sm font-semibold text-dark-text-primary">
              Results
            </h3>
            {queryResults && (
              <span className="text-xs text-green-400">
                {queryResults.length} records
              </span>
            )}
          </div>

          <div className="flex-1 overflow-auto p-4">
            {queryError ? (
              <div className="flex items-start gap-2 text-red-400 p-4 bg-red-900/20 rounded-md border border-red-900/50">
                <AlertCircle size={16} className="mt-0.5" />
                <div className="text-sm font-mono">{queryError}</div>
              </div>
            ) : queryResults && queryResults.length > 0 ? (
              <div className="space-y-2">
                {queryResults.map((result, i) => (
                  <div
                    key={i}
                    className="p-3 bg-dark-elevated border border-dark-border rounded font-mono text-xs text-gray-300 whitespace-pre-wrap"
                  >
                    {JSON.stringify(result, null, 2)}
                  </div>
                ))}
              </div>
            ) : queryResults ? (
              <div className="text-gray-500 text-sm italic">
                No records found.
              </div>
            ) : (
              <div className="text-gray-600 text-sm">
                Execute a query to see results.
              </div>
            )}
          </div>
        </div>
      </div>

      {showSaveDialog && (
        <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-dark-surface rounded-lg shadow-xl p-6 w-96 border border-dark-border">
            <h3 className="text-lg font-bold text-dark-text-primary mb-4">
              Save Query
            </h3>
            <input
              type="text"
              value={queryName}
              onChange={(e) => setQueryName(e.target.value)}
              className="w-full px-3 py-2 border border-dark-border bg-dark-elevated text-dark-text-primary rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent mb-4"
              placeholder="Query name"
              autoFocus
            />
            <div className="flex gap-2 justify-end">
              <button
                onClick={() => setShowSaveDialog(false)}
                className="px-4 py-2 text-dark-text-secondary hover:bg-dark-elevated rounded-lg transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleSave}
                className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
