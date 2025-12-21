import { useState } from "react";
import {
  BarChart3,
  Hammer,
  Target,
  Library,
  Settings as SettingsIcon,
  Zap,
} from "lucide-react";
import PromptForge from "./pages/PromptForge";
import Dashboard from "./pages/Dashboard";
import Missions from "./pages/Missions";
import RfcBrowser from "./pages/RfcBrowser";
import Settings from "./pages/Settings";

type Page = "dashboard" | "forge" | "missions" | "rfcs" | "settings";

function App() {
  const [currentPage, setCurrentPage] = useState<Page>("dashboard");

  const navItems: { id: Page; label: string; icon: React.ReactNode }[] = [
    {
      id: "dashboard",
      label: "Dashboard",
      icon: <BarChart3 className="w-5 h-5" />,
    },
    {
      id: "forge",
      label: "Prompt Forge",
      icon: <Hammer className="w-5 h-5" />,
    },
    { id: "missions", label: "Missions", icon: <Target className="w-5 h-5" /> },
    { id: "rfcs", label: "RFC Browser", icon: <Library className="w-5 h-5" /> },
    {
      id: "settings",
      label: "Settings",
      icon: <SettingsIcon className="w-5 h-5" />,
    },
  ];

  const renderPage = () => {
    switch (currentPage) {
      case "dashboard":
        return <Dashboard onNavigate={setCurrentPage} />;
      case "forge":
        return <PromptForge />;
      case "missions":
        return <Missions />;
      case "rfcs":
        return <RfcBrowser />;
      case "settings":
        return <Settings />;
      default:
        return <Dashboard onNavigate={setCurrentPage} />;
    }
  };

  return (
    <div className="min-h-screen bg-zinc-900 text-zinc-100 flex">
      {/* Sidebar */}
      <nav className="w-16 bg-zinc-950 border-r border-zinc-800 flex flex-col items-center py-4 gap-2">
        <div className="mb-4">
          <Zap className="w-6 h-6 text-emerald-400" />
        </div>
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => setCurrentPage(item.id)}
            className={`w-12 h-12 rounded-lg flex items-center justify-center text-xl transition-all ${
              currentPage === item.id
                ? "bg-emerald-600 text-white"
                : "text-zinc-500 hover:bg-zinc-800 hover:text-zinc-300"
            }`}
            title={item.label}
          >
            {item.icon}
          </button>
        ))}
      </nav>

      {/* Main Content */}
      <main className="flex-1 overflow-auto">{renderPage()}</main>
    </div>
  );
}

export default App;
