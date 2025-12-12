import React from 'react';
import { Save, Copy } from 'lucide-react';
import { GeneratedHashes, HashMetadata } from '../types';

interface ExportPanelProps {
  selectedLayer: string;
  selectedType: string;
  selectedDomain: string;
  metadata: HashMetadata;
  generatedHashes: GeneratedHashes;
  hashValues: string[];
  suggestedCounters: string[];
  copyToClipboard: (text: string) => void;
}

const ExportPanel: React.FC<ExportPanelProps> = ({
  selectedLayer,
  selectedType,
  selectedDomain,
  metadata,
  generatedHashes,
  hashValues,
  suggestedCounters,
  copyToClipboard
}) => {
  // Get primitive type name for export
  const primitiveTypeName = selectedLayer === 'operational' 
    ? (selectedType === 'A' ? 'Actor' : selectedType === 'O' ? 'Object' : selectedType === 'E' ? 'Event' : selectedType === 'C' ? 'Concept' : 'Attribute')
    : selectedLayer === 'code'
    ? (selectedType === 'A' ? 'Agent' : selectedType === 'O' ? 'Object' : selectedType === 'E' ? 'Event' : selectedType === 'C' ? 'Concept' : 'Attribute')
    : (selectedType === 'H' ? 'Hunt' : selectedType === 'D' ? 'Detect' : selectedType === 'R' ? 'Disrupt' : selectedType === 'S' ? 'Disable' : 'Dominate');

  const lispCode = `(:primitive-type :${primitiveTypeName}
 :layer :${selectedLayer}
 :description "${metadata.description}"
 :domain :${selectedDomain}
 :sch-hash "${generatedHashes.sch}"
 :cuid-hash "${generatedHashes.cuid}"
 :uuid-hash "${generatedHashes.uuid}"
 :entropy ${metadata.entropyValue}
 :ttl ${metadata.ttlValue})`;

  const rdfCode = `:Node${generatedHashes.sch.slice(0,3)} rdf:type :${primitiveTypeName} ;
  ctas:hasDescription "${metadata.description}" ;
  ctas:hasSCH "${generatedHashes.sch}" ;
  ctas:hasCUID "${generatedHashes.cuid}" ;
  ctas:hasUUID "${generatedHashes.uuid}" ;
  ctas:hasEntropy "${metadata.entropyValue}" ;
  ctas:hasTTL "${metadata.ttlValue}" ;
  ctas:belongsToLayer :${selectedLayer} ;
  ctas:operatesInDomain :${selectedDomain} .`;

  const jsonData = {
    primitive_type: selectedType,
    layer: selectedLayer,
    description: metadata.description,
    domain: selectedDomain,
    hashes: generatedHashes,
    positions: hashValues,
    suggested_counters: suggestedCounters
  };

  return (
    <div className="bg-white rounded-lg shadow-xl p-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Export & Integration</h2>
      
      <div className="grid grid-cols-2 gap-6">
        <div>
          <h3 className="text-lg font-semibold mb-3">Lisp Integration</h3>
          <div className="bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm overflow-x-auto">
            <pre>{lispCode}</pre>
          </div>
        </div>
        
        <div>
          <h3 className="text-lg font-semibold mb-3">RDF Triple Export</h3>
          <div className="bg-gray-900 text-blue-400 p-4 rounded-lg font-mono text-sm overflow-x-auto">
            <pre>{rdfCode}</pre>
          </div>
        </div>
      </div>
      
      <div className="mt-6 flex gap-4">
        <button
          onClick={() => copyToClipboard(JSON.stringify(jsonData, null, 2))}
          className="flex items-center gap-2 px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          <Save className="w-4 h-4" />
          Export JSON
        </button>
        <button
          onClick={() => copyToClipboard(`${generatedHashes.sch},${generatedHashes.cuid},${generatedHashes.uuid}`)}
          className="flex items-center gap-2 px-6 py-3 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors"
        >
          <Copy className="w-4 h-4" />
          Copy Hash Triplet
        </button>
      </div>
    </div>
  );
};

export default ExportPanel;
