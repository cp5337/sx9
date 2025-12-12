import React from 'react';
import HD4PhaseContent from '@/components/HD4PhaseContent';

interface DetectProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

const Detect: React.FC<DetectProps> = ({ view }) => {
  return <HD4PhaseContent phase="Detect" view={view} />;
};

export default Detect;