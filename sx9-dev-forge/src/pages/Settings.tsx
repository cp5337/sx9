import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Key, List, Library, Zap, Folder } from "lucide-react";

interface LinearTeam {
  id: string;
  name: string;
  key: string;
}

interface KeyEntry {
  name: string;
  service: string;
  active: boolean;
  created_at: string;
  last_used: string | null;
  usage_count: number;
  has_value: boolean;
}

interface VaultStats {
  total: number;
  active: number;
  inactive: number;
  with_value: number;
  vault_path: string;
}

export default function Settings() {
  // Vault state
  const [vaultStats, setVaultStats] = useState<VaultStats | null>(null);
  const [vaultEntries, setVaultEntries] = useState<KeyEntry[]>([]);
  const [standardKeys, setStandardKeys] = useState<string[]>([]);
  const [newKeyName, setNewKeyName] = useState("");
  const [newKeyValue, setNewKeyValue] = useState("");
  const [newKeyService] = useState("api");
  const [showAddKey, setShowAddKey] = useState(false);

  // Linear state
  const [linearKeySet, setLinearKeySet] = useState(false);
  const [teams, setTeams] = useState<LinearTeam[]>([]);
  const [loadingTeams, setLoadingTeams] = useState(false);

  // RFC state
  const [rfcPath, setRfcPath] = useState("/Users/cp5337/Developer/sx9/01-rfc");
  const [rfcPathSet, setRfcPathSet] = useState(false);

  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  useEffect(() => {
    loadVaultData();
    checkLinearStatus();
  }, []);

  const loadVaultData = async () => {
    try {
      const [stats, entries, keys] = await Promise.all([
        invoke<VaultStats>("vault_stats"),
        invoke<KeyEntry[]>("vault_list_entries"),
        invoke<string[]>("vault_standard_keys"),
      ]);
      setVaultStats(stats);
      setVaultEntries(entries);
      setStandardKeys(keys);
    } catch (error) {
      console.error("Failed to load vault:", error);
    }
  };

  const checkLinearStatus = async () => {
    try {
      const hasKey = await invoke<boolean>("get_linear_api_key_status");
      setLinearKeySet(hasKey);
      if (hasKey) {
        loadTeams();
      }
    } catch (error) {
      console.error("Failed to check Linear status:", error);
    }
  };

  const showMessage = (type: "success" | "error", text: string) => {
    setMessage({ type, text });
    setTimeout(() => setMessage(null), 3000);
  };

  const addKey = async () => {
    if (!newKeyName || !newKeyValue) return;
    try {
      await invoke("vault_set_key", {
        name: newKeyName,
        value: newKeyValue,
        service: newKeyService,
      });
      showMessage("success", `Added key: ${newKeyName}`);
      setNewKeyName("");
      setNewKeyValue("");
      setShowAddKey(false);
      loadVaultData();

      // If it's the Linear key, update status
      if (newKeyName === "linear") {
        await invoke("set_linear_api_key", { apiKey: newKeyValue });
        setLinearKeySet(true);
        loadTeams();
      }
    } catch (error) {
      showMessage("error", String(error));
    }
  };

  const deleteKey = async (name: string) => {
    if (!confirm(`Delete key "${name}"?`)) return;
    try {
      await invoke("vault_delete_key", { name });
      showMessage("success", `Deleted key: ${name}`);
      loadVaultData();
    } catch (error) {
      showMessage("error", String(error));
    }
  };

  const toggleKey = async (name: string, active: boolean) => {
    try {
      if (active) {
        await invoke("vault_deactivate_key", { name });
      } else {
        await invoke("vault_activate_key", { name });
      }
      loadVaultData();
    } catch (error) {
      showMessage("error", String(error));
    }
  };

  const loadTeams = async () => {
    setLoadingTeams(true);
    try {
      const result = await invoke<LinearTeam[]>("list_linear_teams");
      setTeams(result);
    } catch (error) {
      console.error("Failed to load teams:", error);
    } finally {
      setLoadingTeams(false);
    }
  };

  const saveRfcPath = async () => {
    try {
      await invoke("set_rfc_base_path", { path: rfcPath });
      setRfcPathSet(true);
      showMessage("success", "RFC path saved");
    } catch (error) {
      showMessage("error", String(error));
    }
  };

  const inputClass =
    "w-full bg-zinc-900 border border-zinc-700 rounded px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none";
  const labelClass = "block text-xs text-zinc-500 mb-2";

  // Check which standard keys are missing
  const existingKeyNames = new Set(vaultEntries.map((e) => e.name));
  const missingKeys = standardKeys.filter((k) => !existingKeyNames.has(k));

  return (
    <div className="p-6 max-w-3xl">
      <div className="mb-6 flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-emerald-400">Settings</h1>
          <p className="text-zinc-500">Configure SX9 Dev Forge integrations</p>
        </div>
        <button
          onClick={async () => {
            try {
              setMessage({ type: "success", text: "Testing Slack..." });
              const res = await invoke("send_slack", {
                message: "🦅 Verification: SX9 Frontend Bridge Operational.",
                channel: null,
              });
              console.log(res);
              setMessage({
                type: "success",
                text: "✅ Slack Verified: " + res,
              });
            } catch (e: any) {
              console.error(e);
              setMessage({
                type: "error",
                text: "❌ Slack Failed: " + e.toString(),
              });
            }
          }}
          className="px-4 py-2 bg-emerald-600/20 text-emerald-400 border border-emerald-600/30 rounded-lg hover:bg-emerald-600/30 transition-colors flex items-center gap-2"
        >
          <Zap className="w-4 h-4" />
          Test Slack
        </button>
      </div>

      {message && (
        <div
          className={`mb-6 p-4 rounded-lg border ${
            message.type === "success"
              ? "bg-emerald-900/20 border-emerald-700 text-emerald-400"
              : "bg-red-900/20 border-red-700 text-red-400"
          }`}
        >
          {message.text}
        </div>
      )}

      {/* KeyVault */}
      <div className="bg-zinc-800 rounded-lg border border-zinc-700 mb-6">
        <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Key className="w-5 h-5" />
            <span className="font-medium">KeyVault</span>
          </div>
          {vaultStats && (
            <span className="text-xs text-zinc-400">
              {vaultStats.active}/{vaultStats.total} keys active
            </span>
          )}
        </div>
        <div className="p-4 space-y-4">
          {/* Stats */}
          {vaultStats && (
            <div className="grid grid-cols-4 gap-3 text-center">
              <div className="bg-zinc-900 rounded p-2">
                <div className="text-xl font-bold text-emerald-400">
                  {vaultStats.total}
                </div>
                <div className="text-xs text-zinc-500">Total</div>
              </div>
              <div className="bg-zinc-900 rounded p-2">
                <div className="text-xl font-bold text-cyan-400">
                  {vaultStats.active}
                </div>
                <div className="text-xs text-zinc-500">Active</div>
              </div>
              <div className="bg-zinc-900 rounded p-2">
                <div className="text-xl font-bold text-amber-400">
                  {vaultStats.with_value}
                </div>
                <div className="text-xs text-zinc-500">With Value</div>
              </div>
              <div className="bg-zinc-900 rounded p-2">
                <div className="text-xl font-bold text-zinc-400">
                  {missingKeys.length}
                </div>
                <div className="text-xs text-zinc-500">Missing</div>
              </div>
            </div>
          )}

          {/* Path */}
          {vaultStats && (
            <div className="text-xs text-zinc-500 font-mono bg-zinc-900 p-2 rounded truncate">
              <Folder className="w-3 h-3 inline" /> {vaultStats.vault_path}
            </div>
          )}

          {/* Key List */}
          <div>
            <div className="flex justify-between items-center mb-2">
              <label className={labelClass}>Stored Keys</label>
              <button
                onClick={() => setShowAddKey(!showAddKey)}
                className="text-xs px-2 py-1 bg-emerald-600 hover:bg-emerald-500 rounded"
              >
                + Add Key
              </button>
            </div>

            {/* Add Key Form */}
            {showAddKey && (
              <div className="bg-zinc-900 rounded p-3 mb-3 space-y-2">
                <div className="grid grid-cols-3 gap-2">
                  <select
                    className={inputClass}
                    value={newKeyName}
                    onChange={(e) => setNewKeyName(e.target.value)}
                  >
                    <option value="">Select key...</option>
                    {missingKeys.map((k) => (
                      <option key={k} value={k}>
                        {k}
                      </option>
                    ))}
                    <option value="__custom">Custom...</option>
                  </select>
                  <input
                    type="password"
                    className={inputClass}
                    value={newKeyValue}
                    onChange={(e) => setNewKeyValue(e.target.value)}
                    placeholder="API key value"
                  />
                  <button
                    onClick={addKey}
                    disabled={!newKeyName || !newKeyValue}
                    className="px-3 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 rounded text-sm"
                  >
                    Save
                  </button>
                </div>
                {newKeyName === "__custom" && (
                  <input
                    className={inputClass}
                    value={newKeyName === "__custom" ? "" : newKeyName}
                    onChange={(e) => setNewKeyName(e.target.value)}
                    placeholder="Custom key name"
                  />
                )}
              </div>
            )}

            {/* Entries */}
            <div className="space-y-1 max-h-64 overflow-y-auto">
              {vaultEntries.length === 0 ? (
                <div className="text-sm text-zinc-500 text-center py-4">
                  No keys stored
                </div>
              ) : (
                vaultEntries.map((entry) => (
                  <div
                    key={entry.name}
                    className={`flex items-center justify-between p-2 rounded ${
                      entry.active ? "bg-zinc-900" : "bg-zinc-900/50 opacity-60"
                    }`}
                  >
                    <div className="flex items-center gap-2">
                      <span
                        className={`w-2 h-2 rounded-full ${entry.has_value ? "bg-emerald-400" : "bg-zinc-600"}`}
                      />
                      <span className="font-mono text-sm">{entry.name}</span>
                      <span className="text-xs text-zinc-500">
                        {entry.service}
                      </span>
                    </div>
                    <div className="flex items-center gap-2">
                      {entry.usage_count > 0 && (
                        <span className="text-xs text-zinc-500">
                          {entry.usage_count}×
                        </span>
                      )}
                      <button
                        onClick={() => toggleKey(entry.name, entry.active)}
                        className={`text-xs px-2 py-1 rounded ${
                          entry.active
                            ? "bg-emerald-900/30 text-emerald-400"
                            : "bg-zinc-700 text-zinc-400"
                        }`}
                      >
                        {entry.active ? "Active" : "Inactive"}
                      </button>
                      <button
                        onClick={() => deleteKey(entry.name)}
                        className="text-xs px-2 py-1 bg-red-900/30 text-red-400 rounded hover:bg-red-900/50"
                      >
                        ×
                      </button>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Missing Keys Hint */}
          {missingKeys.length > 0 && (
            <div className="text-xs text-amber-400 bg-amber-900/20 rounded p-2">
              Missing: {missingKeys.slice(0, 5).join(", ")}
              {missingKeys.length > 5 ? ` +${missingKeys.length - 5} more` : ""}
            </div>
          )}
        </div>
      </div>

      {/* Linear Integration */}
      <div className="bg-zinc-800 rounded-lg border border-zinc-700 mb-6">
        <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <List className="w-5 h-5" />
            <span className="font-medium">Linear Integration</span>
          </div>
          {linearKeySet && (
            <span className="px-2 py-1 bg-emerald-900/30 text-emerald-400 text-xs rounded">
              Connected
            </span>
          )}
        </div>
        <div className="p-4 space-y-4">
          {linearKeySet ? (
            <div>
              <label className={labelClass}>Connected Teams</label>
              {loadingTeams ? (
                <div className="text-sm text-zinc-500">Loading teams...</div>
              ) : teams.length === 0 ? (
                <div className="text-sm text-zinc-500">No teams found</div>
              ) : (
                <div className="space-y-2">
                  {teams.map((team) => (
                    <div
                      key={team.id}
                      className="flex items-center gap-2 text-sm bg-zinc-900 p-2 rounded"
                    >
                      <span className="w-8 h-8 bg-zinc-700 rounded flex items-center justify-center font-mono text-xs">
                        {team.key}
                      </span>
                      <span>{team.name}</span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          ) : (
            <div className="text-sm text-zinc-400">
              Add the <code className="bg-zinc-900 px-1 rounded">linear</code>{" "}
              key to the vault above to enable Linear integration.
            </div>
          )}
        </div>
      </div>

      {/* RFC Configuration */}
      <div className="bg-zinc-800 rounded-lg border border-zinc-700 mb-6">
        <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Library className="w-5 h-5" />
            <span className="font-medium">RFC Directory</span>
          </div>
          {rfcPathSet && (
            <span className="px-2 py-1 bg-emerald-900/30 text-emerald-400 text-xs rounded">
              Configured
            </span>
          )}
        </div>
        <div className="p-4">
          <label className={labelClass}>Base Path</label>
          <div className="flex gap-2">
            <input
              className={inputClass}
              value={rfcPath}
              onChange={(e) => setRfcPath(e.target.value)}
              placeholder="/path/to/rfc/directory"
            />
            <button
              onClick={saveRfcPath}
              disabled={!rfcPath}
              className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 rounded text-sm whitespace-nowrap"
            >
              Save
            </button>
          </div>
        </div>
      </div>

      {/* About */}
      <div className="bg-zinc-800 rounded-lg border border-zinc-700">
        <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700 flex items-center gap-2">
          <Zap className="w-5 h-5" />
          <span className="font-medium">About</span>
        </div>
        <div className="p-4 space-y-2 text-sm text-zinc-400">
          <div>
            <strong className="text-zinc-300">SX9 Dev Forge</strong>
          </div>
          <div>Version 0.1.0</div>
          <div>Prompt Engineering • IDE Bootstrap • Mission Control</div>
          <div className="pt-2 text-xs">
            Built with Tauri 2.0 + React + TypeScript
          </div>
        </div>
      </div>
    </div>
  );
}
