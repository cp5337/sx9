import React, { useState, useEffect } from 'react';
import { Save, Copy, RefreshCw, Hash, Target, Eye, Zap, Ban, Crown } from 'lucide-react';
import { HashMetadata, GeneratedHashes, CounterPrimitive } from './types';
import { positionDefinitions } from './data/positionDefinitions';
import PrimitiveSelector from './components/PrimitiveSelector';
import HashDisplay from './components/HashDisplay';
import ExportPanel from './components/ExportPanel';

const HashComposer: React.FC = () => {
  const [selectedLayer, setSelectedLayer] = useState('operational');
  const [selectedType, setSelectedType] = useState('A');
  const [selectedDomain, setSelectedDomain] = useState('adversarial');
  const [hashValues, setHashValues] = useState(Array(16).fill('0'));
  const [generatedHashes, setGeneratedHashes] = useState<GeneratedHashes>({
    sch: '',
    cuid: '',
    uuid: ''
  });
  const [metadata, setMetadata] = useState<HashMetadata>({
    slug: '',
    description: '',
    tags: [],
    timestamp: new Date().toISOString(),
    ttlValue: 12.6,
    entropyValue: 0.87,
    threatLevel: 'medium'
  });

  // Counter-primitive definitions
  const counterPrimitives: Record<string, CounterPrimitive> = {
    Hunt: {
      name: 'Hunt',
      icon: <Eye className="w-4 h-4" />,
      description: 'Intelligence gathering and reconnaissance',
      color: 'bg-blue-100 text-blue-800',
      tactics: ['OSINT', 'Network scanning', 'Social engineering', 'Physical surveillance']
    },
    Detect: {
      name: 'Detect',
      icon: <Target className="w-4 h-4" />,
      description: 'Threat identification and analysis',
      color: 'bg-green-100 text-green-800',
      tactics: ['SIEM monitoring', 'Anomaly detection', 'Threat hunting', 'Vulnerability assessment']
    },
    Disrupt: {
      name: 'Disrupt',
      icon: <Zap className="w-4 h-4" />,
      description: 'Operational interference and degradation',
      color: 'bg-yellow-100 text-yellow-800',
      tactics: ['DDoS attacks', 'Supply chain disruption', 'Communication jamming', 'Resource exhaustion']
    },
    Disable: {
      name: 'Disable',
      icon: <Ban className="w-4 h-4" />,
      description: 'Capability neutralization and destruction',
      color: 'bg-red-100 text-red-800',
      tactics: ['Infrastructure destruction', 'Data deletion', 'System corruption', 'Physical damage']
    },
    Dominate: {
      name: 'Dominate',
      icon: <Crown className="w-4 h-4" />,
      description: 'Full control establishment and maintenance',
      color: 'bg-purple-100 text-purple-800',
      tactics: ['System takeover', 'Administrative control', 'Persistent access', 'Command authority']
    }
  };

  // Generate hashes based on current state
  const generateHashes = () => {
    const input = `${selectedLayer}-${selectedType}-${selectedDomain}-${metadata.description}-${hashValues.join('')}`;
    
    // Simple hash generation (replace with actual hash algorithm)
    const sch = `sch_${input.slice(0, 16)}_${Date.now().toString(16)}`;
    const cuid = `cuid_${input.slice(0, 12)}_${Math.random().toString(36).substr(2, 8)}`;
    const uuid = `uuid_${input.slice(0, 8)}_${Math.random().toString(36).substr(2, 12)}`;
    
    setGeneratedHashes({ sch, cuid, uuid });
  };

  // Copy text to clipboard
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  // Generate suggested counter-primitives
  const suggestedCounters = selectedLayer === 'operational' && metadata.description
    ? Object.keys(counterPrimitives).slice(0, 3)
    : [];

  // Update hashes when inputs change
  useEffect(() => {
    if (selectedLayer && selectedType && selectedDomain) {
      generateHashes();
    }
  }, [selectedLayer, selectedType, selectedDomain, metadata.description, hashValues]);

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Hash className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">CTAS Hash Composer</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-red-100 text-red-800 px-3 py-1 rounded text-sm font-semibold">
                Hunt→Dominate Integrated
              </span>
            </div>
          </div>
          
          <PrimitiveSelector
            selectedLayer={selectedLayer}
            selectedType={selectedType}
            selectedDomain={selectedDomain}
            metadata={metadata}
            onLayerChange={setSelectedLayer}
            onTypeChange={setSelectedType}
            onDomainChange={setSelectedDomain}
            onMetadataChange={setMetadata}
          />
        </div>

        {/* Counter-Primitive Suggestions */}
        {selectedLayer === 'operational' && metadata.description && (
          <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
            <h2 className="text-xl font-semibold text-gray-800 mb-4">Suggested Counter-Primitives</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {suggestedCounters.map((counter) => {
                const counterInfo = counterPrimitives[counter];
                if (!counterInfo) return null;
                return (
                  <div key={counter} className={`flex items-center gap-2 px-3 py-2 rounded-md ${counterInfo.color}`}>
                    {counterInfo.icon}
                    <div>
                      <div className="font-semibold">{counter}</div>
                      <div className="text-sm opacity-75">{counterInfo.description}</div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Hash Position Editor */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6">Hash Position Editor</h2>
          
          <div className="grid grid-cols-4 md:grid-cols-8 gap-4">
            {positionDefinitions.map((pos, index) => (
              <div key={pos.id} className="text-center">
                <div className="mb-2">
                  <label className="block text-xs font-medium text-gray-600 mb-1">
                    {pos.name}
                  </label>
                  <select
                    value={hashValues[index]}
                    onChange={(e) => {
                      const newValues = [...hashValues];
                      newValues[index] = e.target.value;
                      setHashValues(newValues);
                    }}
                    className="w-full p-2 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  >
                    {Object.entries(pos.values || {}).map(([key, desc]) => (
                      <option key={key} value={key}>{key} - {desc}</option>
                    ))}
                  </select>
                </div>
                
                <div className="mt-2 bg-gray-100 rounded p-2 text-center">
                  <span className="font-mono text-lg font-bold text-blue-600">
                    {hashValues[index]}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Generated Hashes Display */}
        <HashDisplay 
          generatedHashes={generatedHashes}
          copyToClipboard={copyToClipboard}
        />

        {/* Export Panel */}
        <ExportPanel
          selectedLayer={selectedLayer}
          selectedType={selectedType}
          selectedDomain={selectedDomain}
          metadata={metadata}
          generatedHashes={generatedHashes}
          hashValues={hashValues}
          suggestedCounters={suggestedCounters}
          copyToClipboard={copyToClipboard}
        />
      </div>
    </div>
  );
};

export default HashComposer;
