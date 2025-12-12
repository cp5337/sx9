import React from 'react';
import HD4PhaseContent from '@/components/HD4PhaseContent';

interface HuntProps {
  view: 'map' | 'grid' | 'graph' | 'cognigraph';
}

const Hunt: React.FC<HuntProps> = ({ view }) => {
  return <HD4PhaseContent phase="Hunt" view={view} />;
};

export default Hunt;