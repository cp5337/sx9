import React from 'react';
import { Save, Copy } from 'lucide-react';
import type { GeneratedHashes, HashMetadata } from './types';

interface ExportPanelProps {
  selectedType: string;
  selectedLayer: string;
  selectedDomain: string;
  generatedHashes: GeneratedHashes;
  metadata: HashMetadata;
  suggestedCounters: string[];
  copyToClipboard: (text: string) => void;
}

export const ExportPanel: React.FC<ExportPanelProps> = ({
  selectedType,
  selectedLayer,
  selectedDomain,
  generatedHashes,
  metadata,
  suggestedCounters,
  copyToClipboard
}) => {
  const primitiveTypeName = selectedType === 'A' ? 'Actor' : 
    selectedType === 'O' ? 'Object' : 
    selectedType === 'E' ? 'Event' : 
    selectedType === 'C' ? 'Concept' : 'Attribute';

  const lispExport = `(:primitive-type :${primitiveTypeName}
 :layer :${selectedLayer}
 :description "${metadata.description}"
 :domain :${selectedDomain}
 :sch-hash "${generatedHashes.sch}"
 :cuid-hash "${generatedHashes.cuid}"
 :uuid-hash "${generatedHashes.uuid}"
 :entropy ${metadata.entropyValue}
 :ttl ${metadata.ttlValue})`;

  const rdfExport = `:Node${generatedHashes.sch.slice(0,3)} rdf:type :${primitiveTypeName} ;
  ctas:hasDescription "${metadata.description}" ;
  ctas:hasSCH "${generatedHashes.sch}" ;
  ctas:hasCUID "${generatedHashes.cuid}" ;
  ctas:hasUUID "${generatedHashes.uuid}" ;
  ctas:hasEntropy "${metadata.entropyValue}" ;
  ctas:hasTTL "${metadata.ttlValue}" ;
  ctas:belongsToLayer :${selectedLayer} ;
  ctas:operatesInDomain :${selectedDomain} .`;

  const jsonExport = JSON.stringify({
    primitive_type: selectedType,
    layer: selectedLayer,
    description: metadata.description,
    domain: selectedDomain,
    hashes: generatedHashes,
    suggested_counters: suggestedCounters
  }, null, 2);

  return (
    <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6">
      <h2 className="text-2xl font-semibold text-white mb-6">Export & Integration</h2>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div>
          <h3 className="text-lg font-semibold mb-3 text-white">Lisp Integration</h3>
          <div className="bg-slate-900 text-green-400 p-4 rounded-lg font-mono text-sm overflow-x-auto border border-slate-600">
            <pre>{lispExport}</pre>
          </div>
        </div>
        
        <div>
          <h3 className="text-lg font-semibold mb-3 text-white">RDF Triple Export</h3>
          <div className="bg-slate-900 text-blue-400 p-4 rounded-lg font-mono text-sm overflow-x-auto border border-slate-600">
            <pre>{rdfExport}</pre>
          </div>
        </div>
      </div>
      
      <div className="mt-6 flex gap-4">
        <button
          onClick={() => copyToClipboard(jsonExport)}
          className="flex items-center gap-2 px-6 py-3 bg-blue-600/20 text-blue-400 rounded-lg border border-blue-500/30 hover:bg-blue-600/30 transition-colors"
        >
          <Save className="w-4 h-4" />
          Export JSON
        </button>
        <button
          onClick={() => copyToClipboard(`${generatedHashes.sch},${generatedHashes.cuid},${generatedHashes.uuid}`)}
          className="flex items-center gap-2 px-6 py-3 bg-green-600/20 text-green-400 rounded-lg border border-green-500/30 hover:bg-green-600/30 transition-colors"
        >
          <Copy className="w-4 h-4" />
          Copy Hash Triplet
        </button>
      </div>
    </div>
  );
};
