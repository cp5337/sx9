import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Settings, Tag } from "lucide-react";

interface Mission {
  id: string;
  title: string;
  status: string;
  persona: string;
  harness: string;
  phase: string;
  priority: string;
  prompt_type: string;
  objective: string;
  linear_issue_id: string | null;
  linear_issue_url: string | null;
  created_at: string;
  updated_at: string;
  checkpoints: { id: string; message: string; timestamp: string }[];
}

export default function Missions() {
  const [missions, setMissions] = useState<Mission[]>([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<Mission | null>(null);
  const [filter, setFilter] = useState<string>("all");

  useEffect(() => {
    loadMissions();
  }, []);

  const loadMissions = async () => {
    try {
      const result = await invoke<Mission[]>("list_missions");
      setMissions(result);
    } catch (error) {
      console.error("Failed to load missions:", error);
    } finally {
      setLoading(false);
    }
  };

  const startMission = async (id: string) => {
    try {
      await invoke("start_mission", { id });
      loadMissions();
    } catch (error) {
      console.error("Failed to start mission:", error);
    }
  };

  const completeMission = async (id: string) => {
    try {
      await invoke("complete_mission", { id });
      loadMissions();
    } catch (error) {
      console.error("Failed to complete mission:", error);
    }
  };

  const bootstrapIde = async (mission: Mission) => {
    try {
      await invoke("bootstrap_ide", {
        config: {
          ide_type: "cursor",
          mission_id: mission.id,
          mission_title: mission.title,
          persona: mission.persona,
          phase: mission.phase,
          constraints: {
            hard: [],
            soft: [],
          },
        },
      });
      alert("✅ IDE bootstrapped! Check your .cursorrules file.");
    } catch (error) {
      alert(`❌ Bootstrap failed: ${error}`);
    }
  };

  const filteredMissions = missions.filter((m) => {
    if (filter === "all") return true;
    return m.status === filter.toUpperCase();
  });

  const statusColor = (status: string) => {
    switch (status) {
      case "ACTIVE":
        return "bg-emerald-900/30 text-emerald-400 border-emerald-700";
      case "COMPLETED":
        return "bg-cyan-900/30 text-cyan-400 border-cyan-700";
      case "DRAFTED":
        return "bg-zinc-700 text-zinc-300 border-zinc-600";
      case "FAILED":
        return "bg-red-900/30 text-red-400 border-red-700";
      case "BLOCKED":
        return "bg-amber-900/30 text-amber-400 border-amber-700";
      default:
        return "bg-zinc-700 text-zinc-400 border-zinc-600";
    }
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-2xl font-bold text-emerald-400">Missions</h1>
          <p className="text-zinc-500">
            Track and manage your development missions
          </p>
        </div>
        <div className="flex gap-2">
          {["all", "drafted", "active", "completed"].map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`px-3 py-1 rounded text-sm ${filter === f ? "bg-emerald-600 text-white" : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700"}`}
            >
              {f.charAt(0).toUpperCase() + f.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Mission List */}
        <div className="lg:col-span-2 space-y-3">
          {loading ? (
            <div className="text-center text-zinc-500 py-8">
              Loading missions...
            </div>
          ) : filteredMissions.length === 0 ? (
            <div className="text-center text-zinc-500 py-8">
              <div className="text-4xl mb-2">📭</div>
              <div>No missions found</div>
            </div>
          ) : (
            filteredMissions.map((mission) => (
              <div
                key={mission.id}
                onClick={() => setSelected(mission)}
                className={`bg-zinc-800 rounded-lg p-4 border cursor-pointer transition-all ${
                  selected?.id === mission.id
                    ? "border-emerald-500"
                    : "border-zinc-700 hover:border-zinc-600"
                }`}
              >
                <div className="flex justify-between items-start mb-2">
                  <div>
                    <div className="font-medium">
                      {mission.title || "Untitled Mission"}
                    </div>
                    <div className="text-xs text-zinc-500">{mission.id}</div>
                  </div>
                  <span
                    className={`px-2 py-1 rounded text-xs border ${statusColor(mission.status)}`}
                  >
                    {mission.status}
                  </span>
                </div>
                <div className="flex gap-4 text-xs text-zinc-400">
                  <span>🎭 {mission.persona || "DEFAULT"}</span>
                  <Settings className="w-3 h-3 inline" /> {mission.harness}
                  <span>📍 {mission.phase}</span>
                  <Tag className="w-3 h-3 inline" /> {mission.priority}
                </div>
                {mission.objective && (
                  <div className="mt-2 text-sm text-zinc-400 line-clamp-2">
                    {mission.objective}
                  </div>
                )}
              </div>
            ))
          )}
        </div>

        {/* Mission Detail */}
        <div className="lg:sticky lg:top-4 lg:self-start">
          {selected ? (
            <div className="bg-zinc-800 rounded-lg border border-zinc-700 overflow-hidden">
              <div className="px-4 py-3 bg-zinc-900 border-b border-zinc-700">
                <div className="font-medium">
                  {selected.title || "Untitled Mission"}
                </div>
                <div className="text-xs text-zinc-500">{selected.id}</div>
              </div>
              <div className="p-4 space-y-4">
                <div>
                  <div className="text-xs text-zinc-500 mb-1">Status</div>
                  <span
                    className={`px-2 py-1 rounded text-sm border ${statusColor(selected.status)}`}
                  >
                    {selected.status}
                  </span>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Persona</div>
                    <div className="text-sm">
                      {selected.persona || "DEFAULT"}
                    </div>
                  </div>
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Harness</div>
                    <div className="text-sm">{selected.harness}</div>
                  </div>
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Phase</div>
                    <div className="text-sm">{selected.phase}</div>
                  </div>
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Priority</div>
                    <div className="text-sm">{selected.priority}</div>
                  </div>
                </div>

                {selected.objective && (
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">Objective</div>
                    <div className="text-sm text-zinc-300">
                      {selected.objective}
                    </div>
                  </div>
                )}

                {selected.linear_issue_url && (
                  <div>
                    <div className="text-xs text-zinc-500 mb-1">
                      Linear Issue
                    </div>
                    <a
                      href={selected.linear_issue_url}
                      target="_blank"
                      rel="noopener"
                      className="text-sm text-emerald-400 hover:underline"
                    >
                      {selected.linear_issue_id} →
                    </a>
                  </div>
                )}

                {selected.checkpoints && selected.checkpoints.length > 0 && (
                  <div>
                    <div className="text-xs text-zinc-500 mb-2">
                      Checkpoints
                    </div>
                    <div className="space-y-2">
                      {selected.checkpoints.map((cp) => (
                        <div
                          key={cp.id}
                          className="text-xs bg-zinc-900 rounded p-2"
                        >
                          <div className="text-zinc-400">{cp.message}</div>
                          <div className="text-zinc-600 mt-1">
                            {new Date(cp.timestamp).toLocaleString()}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                <div className="flex gap-2 pt-2">
                  <button
                    onClick={() => bootstrapIde(selected)}
                    className="flex-1 px-3 py-2 bg-purple-600 hover:bg-purple-500 rounded text-sm"
                  >
                    🚀 Bootstrap IDE
                  </button>
                  {selected.status === "DRAFTED" && (
                    <button
                      onClick={() => startMission(selected.id)}
                      className="flex-1 px-3 py-2 bg-emerald-600 hover:bg-emerald-500 rounded text-sm"
                    >
                      Start Mission
                    </button>
                  )}
                  {selected.status === "ACTIVE" && (
                    <button
                      onClick={() => completeMission(selected.id)}
                      className="flex-1 px-3 py-2 bg-cyan-600 hover:bg-cyan-500 rounded text-sm"
                    >
                      Complete
                    </button>
                  )}
                </div>
              </div>
            </div>
          ) : (
            <div className="bg-zinc-800 rounded-lg border border-zinc-700 p-8 text-center text-zinc-500">
              <div className="text-4xl mb-2">👈</div>
              <div>Select a mission to view details</div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
