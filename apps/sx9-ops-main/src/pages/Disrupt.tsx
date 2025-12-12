import React from 'react';
import HD4PhaseContent from '@/components/HD4PhaseContent';

interface DisruptProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

const Disrupt: React.FC<DisruptProps> = ({ view }) => {
  return <HD4PhaseContent phase="Disrupt" view={view} />;
};

export default Disrupt;