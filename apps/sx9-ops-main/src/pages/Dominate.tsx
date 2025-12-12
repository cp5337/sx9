import React from 'react';
import HD4PhaseContent from '@/components/HD4PhaseContent';

interface DominateProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

const Dominate: React.FC<DominateProps> = ({ view }) => {
  return <HD4PhaseContent phase="Dominate" view={view} />;
};

export default Dominate;