import React, { createContext, useContext, useState, ReactNode } from 'react';

interface RightPanelContextType {
  isOpen: boolean;
  setIsOpen: (open: boolean) => void;
  isCollapsed: boolean;
  setIsCollapsed: (collapsed: boolean) => void;
  panelWidth: number;
}

const RightPanelContext = createContext<RightPanelContextType | undefined>(undefined);

export const RightPanelProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [isOpen, setIsOpen] = useState(true);
  const [isCollapsed, setIsCollapsed] = useState(false);
  const panelWidth = isCollapsed ? 48 : 144; // w-12 = 48px, w-36 = 144px (matches left sidebar)

  return (
    <RightPanelContext.Provider value={{ isOpen, setIsOpen, isCollapsed, setIsCollapsed, panelWidth }}>
      {children}
    </RightPanelContext.Provider>
  );
};

export const useRightPanel = () => {
  const context = useContext(RightPanelContext);
  if (context === undefined) {
    throw new Error('useRightPanel must be used within a RightPanelProvider');
  }
  return context;
};

