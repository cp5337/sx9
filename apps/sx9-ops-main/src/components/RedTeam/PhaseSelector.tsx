import React from 'react';

interface Phase {
  id: string;
  name: string;
  type: string;
}

interface PhaseSelectorProps {
  phases: Phase[];
  selectedPhase: string | null;
  onPhaseSelect: (phase: string) => void;
  getPhaseIcon: (phase: string) => React.ReactNode;
}

const PhaseSelector: React.FC<PhaseSelectorProps> = ({
  phases,
  selectedPhase,
  onPhaseSelect,
  getPhaseIcon
}) => {
  return (
    <div className="flex flex-wrap gap-2 mb-4">
      {phases.map(phase => (
        <button
          key={phase.id}
          onClick={() => onPhaseSelect(phase.name)}
          className={`flex items-center px-3 py-1 rounded-full text-xs ${
            selectedPhase === phase.name
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
          }`}
        >
          {getPhaseIcon(phase.name)}
          <span className="ml-2">{phase.name}</span>
        </button>
      ))}
    </div>
  );
};

export default PhaseSelector;