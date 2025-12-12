import React from 'react';
import { Code, Play } from 'lucide-react';

interface ScriptScrapersProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const ScriptScrapers: React.FC<ScriptScrapersProps> = ({ hd4Action }) => {
  const scripts = [
    { id: 1, name: `${hd4Action}NetworkScan`, description: 'Scans network for vulnerabilities' },
    { id: 2, name: `${hd4Action}LogAnalyzer`, description: 'Analyzes log files for suspicious activities' },
    { id: 3, name: `${hd4Action}ThreatIntel`, description: 'Gathers threat intelligence from various sources' },
    { id: 4, name: `${hd4Action}MalwareDetector`, description: 'Detects and analyzes potential malware' },
  ];

  return (
    <div className="bg-gray-800 p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Code className="mr-2" size={20} />
        {hd4Action} Script Scrapers
      </h2>
      <div className="space-y-4">
        {scripts.map(script => (
          <div key={script.id} className="bg-gray-700 p-3 rounded-lg">
            <div className="flex justify-between items-center">
              <h3 className="font-semibold">{script.name}</h3>
              <button className="bg-blue-500 text-white px-2 py-1 rounded text-xs flex items-center">
                <Play size={12} className="mr-1" />
                Run
              </button>
            </div>
            <p className="text-sm text-gray-400 mt-1">{script.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ScriptScrapers;