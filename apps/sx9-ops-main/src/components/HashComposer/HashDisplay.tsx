import React from 'react';
import { Copy } from 'lucide-react';
import type { GeneratedHashes } from './types';

interface HashDisplayProps {
  generatedHashes: GeneratedHashes;
  copyToClipboard: (text: string) => void;
}

export const HashDisplay: React.FC<HashDisplayProps> = ({ generatedHashes, copyToClipboard }) => {
  const hashTypes = [
    { 
      type: 'SCH', 
      hash: generatedHashes.sch, 
      desc: 'Synaptic Convergent Hash - Operationally Active', 
      color: 'bg-green-500/20 text-green-400 border-green-500/30' 
    },
    { 
      type: 'CUID', 
      hash: generatedHashes.cuid, 
      desc: 'Cognitive Unique ID - Context-Aware Identity', 
      color: 'bg-blue-500/20 text-blue-400 border-blue-500/30' 
    },
    { 
      type: 'UUID', 
      hash: generatedHashes.uuid, 
      desc: 'Universal Unique ID - Persistent Identity', 
      color: 'bg-purple-500/20 text-purple-400 border-purple-500/30' 
    }
  ];

  return (
    <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6 mb-6">
      <h2 className="text-2xl font-semibold text-white mb-6">Generated Trivariate Hashes</h2>
      
      <div className="grid grid-cols-1 gap-4">
        {hashTypes.map((item) => (
          <div key={item.type} className="bg-slate-700/50 border border-slate-600 rounded-lg p-4">
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center gap-3">
                <span className={`px-3 py-1 rounded font-bold border ${item.color}`}>
                  {item.type}
                </span>
                <span className="text-sm text-slate-400">{item.desc}</span>
              </div>
              <button
                onClick={() => copyToClipboard(item.hash)}
                className="flex items-center gap-1 px-3 py-1 bg-slate-600/50 hover:bg-slate-600/70 rounded border border-slate-500 transition-colors text-slate-300"
              >
                <Copy className="w-4 h-4" />
                Copy
              </button>
            </div>
            <div className="bg-slate-800/50 rounded p-3 font-mono text-lg text-center tracking-wider border border-slate-600 text-green-400">
              {item.hash}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
