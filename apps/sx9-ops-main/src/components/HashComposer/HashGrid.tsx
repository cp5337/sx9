import React from 'react';
import { positions } from './hashData';

interface HashGridProps {
  hashValues: string[];
  updateHashValue: (position: number, value: string) => void;
  selectedType: string;
  setSelectedType: (type: string) => void;
  currentPrimitiveTypes: Record<string, string>;
  selectedLayer: string;
}

export const HashGrid: React.FC<HashGridProps> = ({
  hashValues,
  updateHashValue,
  selectedType,
  setSelectedType,
  currentPrimitiveTypes,
  selectedLayer
}) => {
  return (
    <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-4 gap-4">
      {positions.map((pos, index) => (
        <div key={index} className="bg-slate-700/50 border border-slate-600 rounded-lg p-4 hover:bg-slate-700/70 transition-colors">
          <div className="flex items-center gap-2 mb-3">
            <span className="bg-blue-500/20 text-blue-400 px-2 py-1 rounded text-sm font-bold border border-blue-500/30">
              {pos.pos}
            </span>
            <span className="font-medium text-sm text-white">{pos.name}</span>
          </div>
          
          {pos.values ? (
            <select
              value={hashValues[index]}
              onChange={(e) => updateHashValue(index, e.target.value)}
              className="w-full p-2 bg-slate-600/50 border border-slate-500 rounded text-sm text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              {Object.entries(pos.values).map(([key, desc]) => (
                <option key={key} value={key} className="bg-slate-700">
                  {key} - {desc}
                </option>
              ))}
            </select>
          ) : pos.pos === 1 ? (
            <select
              value={hashValues[index]}
              onChange={(e) => {
                updateHashValue(index, e.target.value);
                setSelectedType(e.target.value);
              }}
              className="w-full p-2 bg-slate-600/50 border border-slate-500 rounded text-sm text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              {Object.entries(currentPrimitiveTypes).map(([key, desc]) => (
                <option key={key} value={key} className="bg-slate-700">
                  {key} - {desc.split(' - ')[0]}
                </option>
              ))}
            </select>
          ) : (
            <div>
              <input
                type="text"
                value={hashValues[index]}
                onChange={(e) => updateHashValue(index, e.target.value.slice(0, 1))}
                maxLength={1}
                className="w-full p-2 bg-slate-600/50 border border-slate-500 rounded text-sm text-center font-mono text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="?"
              />
              {pos.description && (
                <p className="text-xs text-slate-400 mt-1">{pos.description}</p>
              )}
            </div>
          )}
          
          <div className="mt-3 bg-slate-800/50 rounded p-2 text-center border border-slate-600">
            <span className="font-mono text-lg font-bold text-blue-400">
              {hashValues[index]}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
};
