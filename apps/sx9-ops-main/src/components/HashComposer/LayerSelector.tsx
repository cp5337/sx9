import React from 'react';

interface LayerSelectorProps {
  selectedLayer: string;
  onLayerChange: (layer: string) => void;
}

export const LayerSelector: React.FC<LayerSelectorProps> = ({ selectedLayer, onLayerChange }) => {
  const layers = [
    { id: 'operational', name: 'Operational Layer', description: '197 Adversarial Primitives', count: 197 },
    { id: 'counter', name: 'Counter-Primitive Layer', description: 'Hunt→Dominate Framework', count: 5 },
    { id: 'code', name: 'Code Layer', description: 'Source Implementation', count: 5 }
  ];

  return (
    <div>
      <label className="block text-sm font-medium text-slate-300 mb-3">
        Primitive Layer
      </label>
      <div className="space-y-2">
        {layers.map((layer) => (
          <label key={layer.id} className="flex items-center p-3 bg-slate-700/50 rounded-lg border border-slate-600 hover:bg-slate-700/70 cursor-pointer transition-colors">
            <input
              type="radio"
              value={layer.id}
              checked={selectedLayer === layer.id}
              onChange={(e) => onLayerChange(e.target.value)}
              className="mr-3 text-blue-500 focus:ring-blue-500"
            />
            <div className="flex-1">
              <div className="flex items-center justify-between">
                <span className="font-medium text-white">{layer.name}</span>
                <span className="text-xs bg-blue-500/20 text-blue-400 px-2 py-1 rounded border border-blue-500/30">
                  {layer.count}
                </span>
              </div>
              <p className="text-sm text-slate-400">{layer.description}</p>
            </div>
          </label>
        ))}
      </div>
    </div>
  );
};
