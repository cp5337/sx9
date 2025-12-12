import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { ThemeProvider } from './contexts/ThemeContext';
import { AuthProvider } from './contexts/AuthContext';
import { DatabaseProvider } from './contexts/DatabaseContext';
import { SidebarProvider } from './contexts/SidebarContext';
import { RightPanelProvider } from './contexts/RightPanelContext';
import Login from './components/Login';
import ProtectedRoute from './components/ProtectedRoute';
import Sidebar from './components/Sidebar';
import RightPanel from './components/RightPanel';
import Navigation from './components/Navigation';
import Dashboard from './pages/Dashboard';
import Hunt from './pages/Hunt';
import Detect from './pages/Detect';
import Disable from './pages/Disable';
import Disrupt from './pages/Disrupt';
import Dominate from './pages/Dominate';
import DVM from './pages/DVM';
import Tasks from './pages/Tasks';
import InfoStreams from './pages/InfoStreams';
import Containers from './pages/Containers';
import Database from './pages/Database';
import SetupScripts from './pages/SetupScripts';
import SectorAnalysis from './pages/SectorAnalysis';
import Raptor from './pages/Raptor';
import VKali from './pages/vKali';
import Settings from './pages/Settings';
import Plasma from './pages/Plasma';
import Shodan from './pages/Shodan';
import QuickScripts from './pages/QuickScripts';
import Map from './pages/Map';
import MapTest from './pages/MapTest';
import Graph from './pages/Graph';
import Documentation from './pages/Documentation';
import CLI from './pages/CLI';
import ExploitDBPage from './pages/ExploitDB';
import DemoDataReportPage from './pages/DemoDataReport';
import Gallery from './pages/Gallery';
import FireflyPage from './pages/Firefly';
import Cognigraph from './components/Cognigraph';
import SharedComponentsDemo from './components/shared/examples/SharedComponentsDemo';
import { useSidebar } from './contexts/SidebarContext';
import { useRightPanel } from './contexts/RightPanelContext';


// Import all pages
const App: React.FC = () => {
  const [view, setView] = React.useState<'map' | 'grid' | 'graph' | 'cognigraph'>('map');

  return (
    <ThemeProvider>
      <DatabaseProvider>
        <SidebarProvider>
          <RightPanelProvider>
            <Router>
              <AuthProvider>
                <Routes>
                  <Route path="/login" element={<Login />} />
                  <Route path="/*" element={
                    <ProtectedRoute>
                      <AppLayout view={view} setView={setView} />
                    </ProtectedRoute>
                  } />
                </Routes>
              </AuthProvider>
            </Router>
          </RightPanelProvider>
        </SidebarProvider>
      </DatabaseProvider>
    </ThemeProvider>
  );
};

// Separate component to use the sidebar context
const AppLayout: React.FC<{ view: 'map' | 'grid' | 'graph' | 'cognigraph'; setView: (view: 'map' | 'grid' | 'graph' | 'cognigraph') => void }> = ({ view, setView }) => {
  const { sidebarWidth } = useSidebar();
  const { panelWidth } = useRightPanel();

  return (
    <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
      <Sidebar />
      <div 
        className="flex flex-col flex-1 transition-all duration-300" 
        style={{ 
          marginLeft: `${sidebarWidth}px`,
          marginRight: `${panelWidth}px`
        }}
      >
        <Navigation view={view} setView={setView} />
        <main className="flex-1 overflow-x-hidden overflow-y-auto bg-gray-100 dark:bg-gray-900">
                        <Routes>
                          <Route path="/" element={<Dashboard view={view} />} />
                          <Route path="/hunt" element={<Hunt view={view} />} />
                          <Route path="/detect" element={<Detect view={view} />} />
                          <Route path="/disable" element={<Disable view={view} />} />
                          <Route path="/disrupt" element={<Disrupt view={view} />} />
                          <Route path="/dominate" element={<Dominate view={view} />} />
                          <Route path="/critical-infrastructure" element={<div className="p-4">Critical Infrastructure Page</div>} />
                          <Route path="/kill-chain" element={<div className="p-4">Kill Chain Page</div>} />
                          <Route path="/kill-chain/:phase" element={<div className="p-4">Kill Chain Phase</div>} />
                          <Route path="/dvm" element={<DVM />} />
                          <Route path="/tasks" element={<Tasks />} />
                          <Route path="/info-streams" element={<InfoStreams />} />
                          <Route path="/containers" element={<Containers />} />
                          <Route path="/database" element={<Database />} />
                          <Route path="/setup-scripts" element={<SetupScripts />} />
                          <Route path="/sectors" element={<SectorAnalysis />} />
                          <Route path="/raptor" element={<Raptor />} />
                          <Route path="/vkali" element={<VKali />} />
                          <Route path="/settings" element={<Settings />} />
                          <Route path="/plasma" element={<Plasma />} />
                          <Route path="/shodan" element={<Shodan />} />
                          <Route path="/exploit-db" element={<ExploitDBPage />} />
                          <Route path="/demo-report" element={<DemoDataReportPage />} />
                          <Route path="/gallery" element={<Gallery />} />
                          <Route path="/component-showcase" element={<Gallery />} />
                          <Route path="/firefly" element={<FireflyPage />} />
                          <Route path="/quick-scripts" element={<QuickScripts />} />
                          <Route path="/map" element={<Map />} />
                          <Route path="/map-test" element={<MapTest />} />
                          <Route path="/graph" element={<Graph />} />
                          <Route path="/documentation" element={<Documentation />} />
                          <Route path="/cli" element={<CLI />} />
                          <Route path="/cognigraph" element={<Cognigraph />} />
                          <Route path="/shared-components" element={<SharedComponentsDemo />} />
                        </Routes>
                      </main>
                    </div>
                    <RightPanel />
                  </div>
  );
};

export default App;