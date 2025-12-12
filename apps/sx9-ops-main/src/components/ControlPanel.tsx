import React from 'react';
import MapView from './MapView';
import CTASGraphAnimation from './CTASGraphAnimation';

/**
 * ControlPanel.tsx
 * Main control panel component for the CTAS dashboard
 * Author: Charlie Payne
 * Date: June 15, 2023
 * 
 * This component serves as the primary control interface for the CTAS dashboard,
 * allowing users to interact with various visualizations and data views.
 * 
 * MVP:
 * - Toggle between map and graph views
 * - Basic controls for filtering and data selection
 * 
 * IOC:
 * - Integration with real-time data streams
 * - Advanced filtering and search capabilities
 * 
 * Production:
 * - Customizable control layouts
 * - Integration with AI-driven insights and recommendations
 */

interface ControlPanelProps {
  view: 'map' | 'graph';
}

const ControlPanel: React.FC<ControlPanelProps> = ({ view }) => {
  return (
    <div className="bg-gray-800 text-white rounded-lg shadow-lg h-full w-full flex flex-col">
      <div className="flex-grow w-full h-full">
        {view === 'map' ? <MapView selectedSectors={[]} /> : <CTASGraphAnimation />}
      </div>
      {/* Add additional control elements here */}
    </div>
  );
};

export default ControlPanel;