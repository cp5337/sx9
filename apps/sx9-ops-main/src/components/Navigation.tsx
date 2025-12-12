import React from 'react';
import { Map, Grid, Network, BookOpen, Brain } from 'lucide-react';
import { useLocation } from 'react-router-dom';

interface NavigationProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
  setView: (view: 'map' | 'grid' | 'graph' | 'cognigraph') => void;
}

const Navigation: React.FC<NavigationProps> = ({ view, setView }) => {
  const location = useLocation();

  const getPageTitle = () => {
    const path = location.pathname.slice(1);
    return path ? path.charAt(0).toUpperCase() + path.slice(1) : '';
  };

  const isHD4Page = ['/hunt', '/detect', '/disable', '/disrupt', '/dominate', '/'].includes(location.pathname);

  const openXSD = () => {
    // Open XSD Schema interface
    window.open('/xsd', '_blank');
  };

  return (
    <nav className="bg-gray-800 text-white h-8 flex items-center px-3">
      <span className="text-xs font-semibold mr-2 pt-0.5">{getPageTitle()}</span>
      <div className="flex-grow" />
      {/* View selectors moved to right panel - space available for rich cards */}
      <button
        onClick={openXSD}
        className="p-1 rounded bg-gray-700 hover:bg-gray-600"
        title="XSD Schema"
      >
        <BookOpen size={14} />
      </button>
    </nav>
  );
};

export default Navigation;