import React, { useState, useEffect } from 'react';
import { CheckCircle, XCircle, AlertTriangle } from 'lucide-react';

interface StageProps {
  name: string;
  status: 'active' | 'inactive' | 'warning' | 'error';
  noiseLevel: number;
  onNoiseChange: (noise: number) => void;
}

const Stage: React.FC<StageProps> = ({ name, status, noiseLevel, onNoiseChange }) => {
  const getStatusIcon = () => {
    switch (status) {
      case 'active': return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'inactive': return <XCircle className="w-5 h-5 text-gray-500" />;
      case 'warning': return <AlertTriangle className="w-5 h-5 text-yellow-500" />;
      case 'error': return <XCircle className="w-5 h-5 text-red-500" />;
    }
  };

  return (
    <div className="bg-gray-700 p-4 rounded-lg shadow-md w-48">
      <div className="flex justify-between items-center mb-2">
        <h3 className="text-lg font-semibold">{name}</h3>
        {getStatusIcon()}
      </div>
      <div className="flex items-center">
        <span className="mr-2 text-sm">Noise:</span>
        <input
          type="range"
          min="0"
          max="100"
          value={noiseLevel}
          onChange={(e) => onNoiseChange(Number(e.target.value))}
          className="w-full"
        />
        <span className="ml-2 text-sm">{noiseLevel}%</span>
      </div>
    </div>
  );
};

const TargetRings: React.FC<{ stages: StageProps[] }> = ({ stages }) => {
  const size = 200;
  const center = size / 2;

  const getColor = (noiseLevel: number) => {
    const hue = 240 - (noiseLevel * 2.4); // Blue (240) to Red (0)
    return `hsl(${hue}, 100%, 50%)`;
  };

  return (
    <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="mx-auto">
      {[4, 3, 2, 1, 0].map((ring, index) => (
        <circle
          key={ring}
          cx={center}
          cy={center}
          r={((index + 1) / 5) * (size / 2)}
          fill="none"
          stroke={getColor(stages[ring]?.noiseLevel || 0)}
          strokeWidth="20"
          opacity="0.3"
        />
      ))}
      <text
        x={center}
        y={center}
        textAnchor="middle"
        dominantBaseline="middle"
        fill="#E5E7EB"
        fontSize="14"
        fontWeight="bold"
        className="text-shadow"
      >
        Target
      </text>
    </svg>
  );
};

const HD4OperationalSpectrum: React.FC = () => {
  const [stages, setStages] = useState<StageProps[]>([
    { name: 'Hunt', status: 'active', noiseLevel: 20, onNoiseChange: () => {} },
    { name: 'Detect', status: 'active', noiseLevel: 40, onNoiseChange: () => {} },
    { name: 'Disable', status: 'warning', noiseLevel: 60, onNoiseChange: () => {} },
    { name: 'Disrupt', status: 'inactive', noiseLevel: 80, onNoiseChange: () => {} },
    { name: 'Dominate', status: 'error', noiseLevel: 100, onNoiseChange: () => {} },
  ]);

  const [updateFrequency, setUpdateFrequency] = useState(5000);

  useEffect(() => {
    const interval = setInterval(() => {
      setStages(prevStages =>
        prevStages.map(stage => ({
          ...stage,
          status: ['active', 'inactive', 'warning', 'error'][Math.floor(Math.random() * 4)] as StageProps['status'],
        }))
      );
    }, updateFrequency);

    return () => clearInterval(interval);
  }, [updateFrequency]);

  const handleNoiseChange = (index: number, noise: number) => {
    setStages(prevStages =>
      prevStages.map((stage, i) =>
        i === index ? { ...stage, noiseLevel: noise } : stage
      )
    );
  };

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setUpdateFrequency(Number(event.target.value));
  };

  return (
    <div className="bg-gray-800 p-8 rounded-lg shadow-lg space-y-8">
      <h2 className="text-4xl font-bold text-center text-blue-400 mb-8">HD4 Operational Spectrum</h2>
      <div className="relative h-[600px]">
        <div className="absolute top-0 left-0">
          <Stage name={stages[0]?.name || 'Hunt'} status={stages[0]?.status || 'active'} noiseLevel={stages[0]?.noiseLevel || 0} onNoiseChange={(noise) => handleNoiseChange(0, noise)} />
        </div>
        <div className="absolute top-0 right-0">
          <Stage name={stages[1]?.name || 'Detect'} status={stages[1]?.status || 'active'} noiseLevel={stages[1]?.noiseLevel || 0} onNoiseChange={(noise) => handleNoiseChange(1, noise)} />
        </div>
        <div className="absolute bottom-0 left-0">
          <Stage name={stages[2]?.name || 'Disable'} status={stages[2]?.status || 'warning'} noiseLevel={stages[2]?.noiseLevel || 0} onNoiseChange={(noise) => handleNoiseChange(2, noise)} />
        </div>
        <div className="absolute bottom-0 right-0">
          <Stage name={stages[3]?.name || 'Disrupt'} status={stages[3]?.status || 'inactive'} noiseLevel={stages[3]?.noiseLevel || 0} onNoiseChange={(noise) => handleNoiseChange(3, noise)} />
        </div>
        <div className="absolute bottom-1/2 left-1/2 transform -translate-x-1/2 translate-y-1/2">
          <TargetRings stages={stages} />
        </div>
        <div className="absolute bottom-0 left-1/2 transform -translate-x-1/2">
          <Stage name={stages[4]?.name || 'Dominate'} status={stages[4]?.status || 'error'} noiseLevel={stages[4]?.noiseLevel || 0} onNoiseChange={(noise) => handleNoiseChange(4, noise)} />
        </div>
      </div>
      <div className="flex flex-col items-center space-y-2 mt-8">
        <label htmlFor="update-frequency" className="text-sm text-gray-300">
          Update Frequency: {updateFrequency / 1000} seconds
        </label>
        <input
          type="range"
          id="update-frequency"
          min="1000"
          max="10000"
          step="1000"
          value={updateFrequency}
          onChange={handleSliderChange}
          className="w-full max-w-xs"
        />
      </div>
      <div className="text-center mt-8">
        <p className="text-sm text-gray-300">
          Real-time status updates powered by Reservoir Computing simulations
        </p>
      </div>
    </div>
  );
};

export default HD4OperationalSpectrum;