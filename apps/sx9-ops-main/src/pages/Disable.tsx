import React from 'react';
import HD4PhaseContent from '@/components/HD4PhaseContent';

interface DisableProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

const Disable: React.FC<DisableProps> = ({ view }) => {
  return <HD4PhaseContent phase="Disable" view={view} />;
};

export default Disable;