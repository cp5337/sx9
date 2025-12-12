import React from 'react';
import { Copy } from 'lucide-react';
import { GeneratedHashes } from '../types';

interface HashDisplayProps {
  generatedHashes: GeneratedHashes;
  copyToClipboard: (text: string) => void;
}

const HashDisplay: React.FC<HashDisplayProps> = ({ generatedHashes, copyToClipboard }) => {
  const hashItems = [
    { 
      type: 'SCH', 
      hash: generatedHashes.sch, 
      desc: 'Synaptic Convergent Hash - Operationally Active', 
      color: 'bg-green-100 text-green-800' 
    },
    { 
      type: 'CUID', 
      hash: generatedHashes.cuid, 
      desc: 'Cognitive Unique ID - Context-Aware Identity', 
      color: 'bg-blue-100 text-blue-800' 
    },
    { 
      type: 'UUID', 
      hash: generatedHashes.uuid, 
      desc: 'Universal Unique ID - Persistent Identity', 
      color: 'bg-purple-100 text-purple-800' 
    }
  ];

  return (
    <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Generated Trivariate Hashes</h2>
      
      <div className="grid grid-cols-1 gap-4">
        {hashItems.map((item) => (
          <div key={item.type} className="border border-gray-200 rounded-lg p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-3">
                <span className={`px-3 py-1 rounded font-bold ${item.color}`}>
                  {item.type}
                </span>
                <span className="text-sm text-gray-600">{item.desc}</span>
              </div>
              <button
                onClick={() => copyToClipboard(item.hash)}
                className="flex items-center gap-1 px-3 py-1 bg-gray-100 hover:bg-gray-200 rounded transition-colors"
              >
                <Copy className="w-4 h-4" />
                Copy
              </button>
            </div>
            <div className="bg-gray-50 rounded p-3 font-mono text-lg text-center tracking-wider">
              {item.hash}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default HashDisplay;
