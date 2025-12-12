import React from 'react';
import { domainPrimitives } from '../data/domainPrimitives';
import { positionDefinitions } from '../data/positionDefinitions';
import { HashMetadata } from '../types';

interface PrimitiveSelectorProps {
  selectedLayer: string;
  selectedType: string;
  selectedDomain: string;
  metadata: HashMetadata;
  onLayerChange: (layer: string) => void;
  onTypeChange: (type: string) => void;
  onDomainChange: (domain: string) => void;
  onMetadataChange: (metadata: HashMetadata) => void;
}

const PrimitiveSelector: React.FC<PrimitiveSelectorProps> = ({
  selectedLayer,
  selectedType,
  selectedDomain,
  metadata,
  onLayerChange,
  onTypeChange,
  onDomainChange,
  onMetadataChange
}) => {
  // Get current primitive types based on selected layer
  const currentPrimitiveTypes = selectedLayer === 'operational' 
    ? positionDefinitions[0]?.operational || {}
    : selectedLayer === 'code' 
    ? positionDefinitions[0]?.code || {}
    : positionDefinitions[0]?.counter || {};

  // Get primitive type name
  const primitiveTypeName = selectedLayer === 'operational' 
    ? (selectedType === 'A' ? 'Actor' : selectedType === 'O' ? 'Object' : selectedType === 'E' ? 'Event' : selectedType === 'C' ? 'Concept' : 'Attribute')
    : selectedLayer === 'code'
    ? (selectedType === 'A' ? 'Agent' : selectedType === 'O' ? 'Object' : selectedType === 'E' ? 'Event' : selectedType === 'C' ? 'Concept' : 'Attribute')
    : (selectedType === 'H' ? 'Hunt' : selectedType === 'D' ? 'Detect' : selectedType === 'R' ? 'Disrupt' : selectedType === 'S' ? 'Disable' : 'Dominate');

  // Get available primitives for selected domain and type
  const availablePrimitives = selectedLayer === 'operational' && selectedDomain && selectedType
    ? (domainPrimitives[selectedDomain] as any)?.[primitiveTypeName] || []
    : [];

  return (
    <div className="space-y-6">
      {/* Layer Selection */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Primitive Layer
        </label>
        <div className="flex gap-4 flex-wrap">
          <label className="flex items-center">
            <input
              type="radio"
              value="operational"
              checked={selectedLayer === 'operational'}
              onChange={(e) => onLayerChange(e.target.value)}
              className="mr-2"
            />
            Operational Layer (197 Adversarial Primitives)
          </label>
          <label className="flex items-center">
            <input
              type="radio"
              value="counter"
              checked={selectedLayer === 'counter'}
              onChange={(e) => onLayerChange(e.target.value)}
              className="mr-2"
            />
            Counter-Primitive Layer (Hunt→Dominate)
          </label>
          <label className="flex items-center">
            <input
              type="radio"
              value="code"
              checked={selectedLayer === 'code'}
              onChange={(e) => onLayerChange(e.target.value)}
              className="mr-2"
            />
            Code Layer (Source Implementation)
          </label>
        </div>
      </div>

      {/* Domain Selection */}
      {selectedLayer === 'operational' && (
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Operational Domain
          </label>
          <select
            value={selectedDomain}
            onChange={(e) => onDomainChange(e.target.value)}
            className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
          >
            <option value="adversarial">Adversarial Intelligence</option>
            <option value="manufacturing">Manufacturing Automation</option>
            <option value="medical">Medical Systems</option>
          </select>
        </div>
      )}

      {/* Primitive Type Selection */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Primitive Type (Position 1)
        </label>
        <select
          value={selectedType}
          onChange={(e) => onTypeChange(e.target.value)}
          className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
        >
          {Object.entries(currentPrimitiveTypes || {}).map(([key, desc]) => (
            <option key={key} value={key}>{key} - {desc}</option>
          ))}
        </select>
      </div>

      {/* Domain-Specific Primitives */}
      {selectedLayer === 'operational' && availablePrimitives.length > 0 && (
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Domain-Specific {primitiveTypeName}s ({selectedDomain})
          </label>
          <select
            value={metadata.description}
            onChange={(e) => onMetadataChange({...metadata, description: e.target.value})}
            className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Select a domain-specific primitive...</option>
            {availablePrimitives.map((primitive: string) => (
              <option key={primitive} value={primitive}>{primitive}</option>
            ))}
          </select>
        </div>
      )}
    </div>
  );
};

export default PrimitiveSelector;
