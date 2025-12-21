import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Target, Library, Rocket } from "lucide-react";

interface Mission {
  id: string;
  title: string;
  status: string;
  persona: string;
  harness: string;
  phase: string;
  priority: string;
  prompt_type: string;
  created_at: string;
  updated_at: string;
}

interface DashboardProps {
  onNavigate: (
    page: "dashboard" | "forge" | "missions" | "rfcs" | "settings"
  ) => void;
}

export default function Dashboard({ onNavigate }: DashboardProps) {
  const [missions, setMissions] = useState<Mission[]>([]);
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState({ active: 0, completed: 0, total: 0 });
  const [rfcCount, setRfcCount] = useState(0);

  useEffect(() => {
    loadMissions();
    loadRfcCount();
  }, []);

  const loadMissions = async () => {
    try {
      const result = await invoke<Mission[]>("list_missions");
      setMissions(result);
      setStats({
        active: result.filter((m) => m.status === "ACTIVE").length,
        completed: result.filter((m) => m.status === "COMPLETED").length,
        total: result.length,
      });
    } catch (error) {
      console.error("Failed to load missions:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadRfcCount = async () => {
    try {
      const index = await invoke<{ rfcs: any[] }>("load_rfc_index");
      setRfcCount(index.rfcs?.length || 0);
    } catch (error) {
      console.error("Failed to load RFC count:", error);
      setRfcCount(0);
    }
  };

  const recentMissions = missions.slice(0, 5);

  return (
    <div className="p-6">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-emerald-400">SX9 Dev Forge</h1>
        <p className="text-zinc-500">
          Prompt Engineering • IDE Bootstrap • Mission Control
        </p>
      </div>

      <div className="grid grid-cols-4 gap-4 mb-8">
        <div className="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
          <div className="text-3xl font-bold text-emerald-400">
            {stats.active}
          </div>
          <div className="text-xs text-zinc-500 uppercase">Active Missions</div>
        </div>
        <div className="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
          <div className="text-3xl font-bold text-cyan-400">
            {stats.completed}
          </div>
          <div className="text-xs text-zinc-500 uppercase">Completed</div>
        </div>
        <div className="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
          <div className="text-3xl font-bold text-amber-400">{stats.total}</div>
          <div className="text-xs text-zinc-500 uppercase">Total Missions</div>
        </div>
        <div className="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
          <div className="text-3xl font-bold text-purple-400">{rfcCount}</div>
          <div className="text-xs text-zinc-500 uppercase">RFCs Loaded</div>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4 mb-8">
        <button
          onClick={() => onNavigate("forge")}
          className="bg-emerald-600 hover:bg-emerald-500 rounded-lg p-6 text-left transition-all"
        >
          <div className="text-2xl mb-2">🔨</div>
          <div className="font-bold">New Prompt</div>
          <div className="text-sm text-emerald-200">Generate mission YAML</div>
        </button>
        <button
          onClick={() => onNavigate("missions")}
          className="bg-zinc-800 hover:bg-zinc-700 rounded-lg p-6 text-left border border-zinc-700 transition-all"
        >
          <Target className="w-8 h-8 mb-2" />
          <div className="font-bold">View Missions</div>
          <div className="text-sm text-zinc-400">Track active work</div>
        </button>
        <button
          onClick={() => onNavigate("rfcs")}
          className="bg-zinc-800 hover:bg-zinc-700 rounded-lg p-6 text-left border border-zinc-700 transition-all"
        >
          <Library className="w-8 h-8 mb-2" />
          <div className="font-bold">Browse RFCs</div>
          <div className="text-sm text-zinc-400">Search documentation</div>
        </button>
      </div>

      <div className="bg-zinc-800/50 rounded-lg border border-zinc-700">
        <div className="px-4 py-3 border-b border-zinc-700 flex justify-between items-center">
          <h2 className="font-medium">Recent Missions</h2>
          <button
            onClick={() => onNavigate("missions")}
            className="text-xs text-emerald-400 hover:text-emerald-300"
          >
            View All →
          </button>
        </div>
        <div className="divide-y divide-zinc-700">
          {loading ? (
            <div className="p-4 text-center text-zinc-500">Loading...</div>
          ) : recentMissions.length === 0 ? (
            <div className="p-8 text-center text-zinc-500">
              <Rocket className="w-12 h-12 mb-2 text-emerald-400" />
              <div>No missions yet</div>
              <button
                onClick={() => onNavigate("forge")}
                className="mt-4 px-4 py-2 bg-emerald-600 hover:bg-emerald-500 rounded text-sm"
              >
                Create First Mission
              </button>
            </div>
          ) : (
            recentMissions.map((mission) => (
              <div
                key={mission.id}
                className="p-4 flex items-center justify-between"
              >
                <div>
                  <div className="font-medium">
                    {mission.title || mission.id}
                  </div>
                  <div className="text-xs text-zinc-500">
                    {mission.persona} • {mission.harness} • {mission.phase}
                  </div>
                </div>
                <span
                  className={`px-2 py-1 rounded text-xs ${
                    mission.status === "ACTIVE"
                      ? "bg-emerald-900/30 text-emerald-400"
                      : mission.status === "COMPLETED"
                        ? "bg-cyan-900/30 text-cyan-400"
                        : "bg-zinc-700 text-zinc-400"
                  }`}
                >
                  {mission.status}
                </span>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
