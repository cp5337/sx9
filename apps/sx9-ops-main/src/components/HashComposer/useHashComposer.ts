import { useState, useEffect } from 'react';
import { domainPrimitives, counterPrimitives, counterMappings, positions } from './hashData';
import type { HashMetadata, GeneratedHashes } from './types';

export const useHashComposer = () => {
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

  const base64Chars = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/';

  const updateHashValue = (position: number, value: string): void => {
    const newValues = [...hashValues];
    newValues[position] = value;
    setHashValues(newValues);
  };

  const generateTrivariatHashes = (): void => {
    const baseHash = hashValues.join('');
    const sch = baseHash;
    const cuid = baseHash.split('').map((c, i) => 
      i % 3 === 0 ? base64Chars[(base64Chars.indexOf(c) + 1) % 64] : c
    ).join('');
    const uuid = baseHash.split('').map((c, i) => 
      i % 2 === 0 ? base64Chars[(base64Chars.indexOf(c) + 2) % 64] : c  
    ).join('');
    
    setGeneratedHashes({ sch, cuid, uuid });
  };

  const copyToClipboard = (text: string): void => {
    navigator.clipboard.writeText(text);
  };

  const randomizeHash = (): void => {
    const newValues = Array(16).fill(0).map(() => 
      base64Chars[Math.floor(Math.random() * 64)]
    );
    newValues[0] = selectedType;
    setHashValues(newValues);
  };

  const suggestCounterPrimitives = (): string[] => {
    const description = metadata.description;
    for (const [threat, counters] of Object.entries(counterMappings)) {
      if (description.toLowerCase().includes(threat.toLowerCase())) {
        return counters;
      }
    }
    return ['Hunt', 'Detect'];
  };

  useEffect(() => {
    generateTrivariatHashes();
  }, [hashValues]);

  useEffect(() => {
    updateHashValue(0, selectedType);
  }, [selectedType]);

  const currentPrimitiveTypes = selectedLayer === 'operational' 
    ? positions[0]?.operational || {}
    : selectedLayer === 'counter'
    ? positions[0]?.counter || {}
    : positions[0]?.code || {};

  const currentDomainPrimitives = domainPrimitives[selectedDomain] || {};
  const primitiveTypeName = selectedType === 'A' ? 'Actor' : 
    selectedType === 'O' ? 'Object' : 
    selectedType === 'E' ? 'Event' : 
    selectedType === 'C' ? 'Concept' : 'Attribute';
  const availablePrimitives = (currentDomainPrimitives as any)[primitiveTypeName] || [];

  const suggestedCounters = suggestCounterPrimitives();

  return {
    selectedLayer,
    setSelectedLayer,
    selectedType,
    setSelectedType,
    selectedDomain,
    setSelectedDomain,
    hashValues,
    updateHashValue,
    generatedHashes,
    metadata,
    setMetadata,
    randomizeHash,
    copyToClipboard,
    currentPrimitiveTypes,
    availablePrimitives,
    suggestedCounters,
    counterPrimitives
  };
};
