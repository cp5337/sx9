import React, { useState, useEffect } from 'react';
import { Save, Copy, RefreshCw, Hash, Target, Eye, Zap, Ban, Crown } from 'lucide-react';

interface HashMetadata {
  description: string;
  ttlValue: number;
  entropyValue: number;
  threatLevel: string;
}

interface GeneratedHashes {
  sch: string;
  cuid: string;
  uuid: string;
}

interface CounterPrimitive {
  icon: React.ReactNode;
  description: string;
  color: string;
  tactics: string[];
}

interface DomainPrimitives {
  [domain: string]: {
    Actor: string[];
    Object: string[];
    Event: string[];
    Concept: string[];
    Attribute: string[];
  };
}

interface PositionDefinition {
  pos: number;
  name: string;
  operational?: Record<string, string>;
  code?: Record<string, string>;
  counter?: Record<string, string>;
  values?: Record<string, string>;
  description?: string;
}

const CTASHashComposer: React.FC = () => {
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
    description: '',
    ttlValue: 12.6,
    entropyValue: 0.87,
    threatLevel: 'medium'
  });

  // Domain-specific primitive definitions
  const domainPrimitives: DomainPrimitives = {
    adversarial: {
      Actor: [
        'Terrorist Cell Leader', 'Bomb Maker', 'Courier', 'Financier', 'Recruiter',
        'Cyber Operative', 'Sleeper Agent', 'Handler', 'Scout', 'Weapons Dealer'
      ],
      Object: [
        'IED Components', 'Vehicle', 'Communications Device', 'Weapon Cache', 'Safe House',
        'Financial Assets', 'Identity Documents', 'Surveillance Equipment', 'Encrypted Drive'
      ],
      Event: [
        'Bomb Deployment', 'Reconnaissance Mission', 'Fund Transfer', 'Cell Meeting', 'Training Session',
        'Cyber Attack', 'Dead Drop Exchange', 'Border Crossing', 'Target Surveillance'
      ],
      Concept: [
        'Attack Plan', 'Operational Security', 'Martyrdom Ideology', 'Cell Structure', 'Escape Route',
        'Target Selection', 'Timing Strategy', 'Resource Allocation', 'Risk Assessment'
      ],
      Attribute: [
        'Explosives Expertise', 'Language Skills', 'Technical Capability', 'Geographic Knowledge',
        'Security Clearance', 'Financial Resources', 'Operational Experience', 'Physical Fitness'
      ]
    },
    manufacturing: {
      Actor: [
        'Production Manager', 'Quality Inspector', 'Machine Operator', 'Maintenance Tech', 'Safety Officer',
        'Supply Chain Coordinator', 'Process Engineer', 'Automation Specialist', 'Logistics Manager'
      ],
      Object: [
        'Assembly Line', 'Raw Materials', 'Quality Sensors', 'Robotic Arm', 'Conveyor System',
        'Inventory Database', 'Production Schedule', 'Safety Equipment', 'Testing Apparatus'
      ],
      Event: [
        'Production Start', 'Quality Check', 'Material Delivery', 'Equipment Maintenance', 'Shift Change',
        'Batch Processing', 'Defect Detection', 'Order Fulfillment', 'Safety Inspection'
      ],
      Concept: [
        'Lean Manufacturing', 'Six Sigma', 'Just-in-Time', 'Quality Control', 'Process Optimization',
        'Waste Reduction', 'Efficiency Metrics', 'Safety Protocols', 'Cost Management'
      ],
      Attribute: [
        'Production Capacity', 'Error Rate', 'Throughput Speed', 'Energy Efficiency', 'Safety Rating',
        'Maintenance Schedule', 'Skill Level', 'Certification Status', 'Experience Years'
      ]
    },
    medical: {
      Actor: [
        'Attending Physician', 'Nurse Practitioner', 'Pharmacist', 'Lab Technician', 'Emergency Responder',
        'Patient', 'Radiologist', 'Surgeon', 'Medical Administrator', 'Infection Control Specialist'
      ],
      Object: [
        'Medical Device', 'Patient Record', 'Medication', 'Diagnostic Equipment', 'Treatment Protocol',
        'Laboratory Sample', 'Surgical Instrument', 'Patient Monitor', 'Emergency Kit'
      ],
      Event: [
        'Patient Admission', 'Diagnosis', 'Treatment Administration', 'Surgery', 'Discharge',
        'Lab Test', 'Medication Dosing', 'Vital Signs Check', 'Emergency Response'
      ],
      Concept: [
        'Patient Safety', 'Treatment Efficacy', 'Infection Control', 'Pain Management', 'Recovery Protocol',
        'Diagnostic Accuracy', 'Medication Adherence', 'Risk Assessment', 'Care Coordination'
      ],
      Attribute: [
        'Medical Expertise', 'Patient Condition', 'Treatment Response', 'Risk Factor', 'Compliance Level',
        'Vital Signs', 'Lab Values', 'Symptom Severity', 'Recovery Rate'
      ]
    }
  };

  // Counter-primitive definitions
  const counterPrimitives: Record<string, CounterPrimitive> = {
    Hunt: {
      icon: <Target className="w-4 h-4" />,
      description: 'Active search and discovery operations',
      color: 'bg-red-100 text-red-800',
      tactics: ['Reconnaissance', 'Intelligence Gathering', 'Pattern Analysis', 'Behavioral Tracking', 'Network Mapping']
    },
    Detect: {
      icon: <Eye className="w-4 h-4" />,
      description: 'Monitoring and anomaly recognition',
      color: 'bg-yellow-100 text-yellow-800',
      tactics: ['Sensor Monitoring', 'Signature Analysis', 'Threshold Detection', 'Behavioral Analysis', 'Alert Generation']
    },
    Disrupt: {
      icon: <Zap className="w-4 h-4" />,
      description: 'Active interference and path collapse',
      color: 'bg-orange-100 text-orange-800',
      tactics: ['Signal Jamming', 'Resource Denial', 'Pathway Blocking', 'Communication Interference', 'Process Interruption']
    },
    Disable: {
      icon: <Ban className="w-4 h-4" />,
      description: 'Neutralization and capability removal',
      color: 'bg-purple-100 text-purple-800',
      tactics: ['System Shutdown', 'Resource Exhaustion', 'Access Revocation', 'Function Termination', 'Capability Nullification']
    },
    Dominate: {
      icon: <Crown className="w-4 h-4" />,
      description: 'Control establishment and maintenance',
      color: 'bg-green-100 text-green-800',
      tactics: ['System Control', 'Authority Assertion', 'Structure Locking', 'Behavioral Override', 'Operational Superiority']
    }
  };

  // Counter-primitive mappings by threat type
  const counterMappings: Record<string, string[]> = {
    'Bomb Deployment': ['Hunt', 'Detect', 'Disable'],
    'Cyber Attack': ['Detect', 'Disrupt', 'Dominate'],
    'Reconnaissance Mission': ['Hunt', 'Detect', 'Disrupt'],
    'Cell Meeting': ['Hunt', 'Detect', 'Disrupt'],
    'Fund Transfer': ['Detect', 'Disrupt', 'Disable'],
    'Training Session': ['Hunt', 'Disrupt', 'Disable'],
    'Target Surveillance': ['Hunt', 'Detect', 'Disrupt']
  };

  // Position definitions
  const positions: PositionDefinition[] = [
    {
      pos: 1,
      name: 'Primitive Type',
      operational: {
        'A': 'Actor - Entity capable of initiating events',
        'O': 'Object - Physical/logical entity for actions', 
        'E': 'Event - Process/action with state changes',
        'C': 'Concept - Symbolic/analytical construct',
        'T': 'Attribute - Modifier or parameter'
      },
      code: {
        'F': 'Function - Discrete computational unit',
        'M': 'Module - Organizational grouping unit',
        'H': 'Header - USIM metadata and interfaces',
        'G': 'Footer - Summary and verification data',
        'N': 'Comment - Documentation primitive'
      },
      counter: {
        'U': 'Hunt - Active search and discovery',
        'D': 'Detect - Monitoring and recognition',
        'R': 'Disrupt - Interference and collapse',
        'B': 'Disable - Neutralization and removal',
        'G': 'Dominate - Control and maintenance'
      }
    },
    {
      pos: 2,
      name: 'TTL Decay Class',
      values: {
        '0': 'Immediate (seconds)',
        '1': 'Short (minutes)', 
        '2': 'Medium (hours)',
        '3': 'Long (days)',
        '4': 'Extended (weeks)',
        '5': 'Persistent (months)'
      }
    },
    {
      pos: 3,
      name: 'Entropy Class',
      values: {
        '0': 'Static (0.0-0.1)',
        '1': 'Stable (0.1-0.3)',
        '2': 'Moderate (0.3-0.5)',
        '3': 'Dynamic (0.5-0.7)',
        '4': 'Volatile (0.7-0.9)',
        '5': 'Chaotic (0.9-1.0)'
      }
    },
    {
      pos: 4,
      name: 'XOR Baseline',
      description: 'Starting XOR key for mutation detection'
    },
    {
      pos: 5,
      name: 'Operator Signature', 
      description: 'Operator assigned at commit/build'
    },
    {
      pos: 6,
      name: 'Temporal Tail Seed',
      description: 'Time-of-capture encoding'
    },
    {
      pos: 7,
      name: 'Convergence Direction',
      values: {
        'U': '↑ Ascending/Growing',
        'D': '↓ Descending/Declining', 
        'S': '↔ Stable/Lateral',
        'O': '○ Oscillating',
        'R': '↻ Rotating/Cycling'
      }
    },
    {
      pos: 8,
      name: 'Graph Role Code',
      values: {
        'S': 'Source - Origin point',
        'K': 'Sink - Terminal point',
        'R': 'Relay - Pass-through node',
        'H': 'Hub - Connection center',
        'B': 'Bridge - Cross-domain link'
      }
    },
    {
      pos: 9,
      name: 'Mutation Index',
      description: 'Update lineage counter'
    },
    {
      pos: 10,
      name: 'SHA/Checksum Hook',
      description: 'Integrity validation'
    },
    {
      pos: 11,
      name: 'Context Frame ID',
      description: 'Lisp frame reference'
    },
    {
      pos: 12,
      name: 'RDF Vector Reference',
      description: 'Triple pointer'
    },
    {
      pos: 13,
      name: 'XOR Evaluation Flag',
      values: {
        '0': 'No drift detected',
        '1': 'Minor drift (<10%)',
        '2': 'Moderate drift (10-25%)',
        '3': 'Major drift (25-50%)',
        '4': 'Severe drift (>50%)'
      }
    },
    {
      pos: 14,
      name: 'Conflict Weight Delta',
      values: {
        '0': 'No resistance',
        '1': 'Low resistance',
        '2': 'Medium resistance', 
        '3': 'High resistance',
        '4': 'Maximum resistance'
      }
    },
    {
      pos: 15,
      name: 'Storage Tier Marker',
      values: {
        'H': 'Hot - Active memory',
        'W': 'Warm - Cache storage',
        'C': 'Cold - Archive storage'
      }
    },
    {
      pos: 16,
      name: 'Terminus/Suffix Slot',  
      values: {
        'T': 'Training mode',
        'X': 'Executable mode',
        'M': 'Monitoring mode',
        'D': 'Development mode',
        'P': 'Production mode'
      }
    }
  ];

  const base64Chars = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/';

  const updateHashValue = (position: number, value: string) => {
    const newValues = [...hashValues];
    newValues[position] = value;
    setHashValues(newValues);
  };

  const generateTrivariatHashes = () => {
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

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const randomizeHash = () => {
    const newValues = Array(16).fill(0).map(() => 
      base64Chars[Math.floor(Math.random() * 64)]
    );
    newValues[0] = selectedType;
    setHashValues(newValues);
  };

  const suggestCounterPrimitives = () => {
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
    ? positions[0].operational || {}
    : selectedLayer === 'counter'
    ? positions[0].counter || {}
    : positions[0].code || {};

  const currentDomainPrimitives = domainPrimitives[selectedDomain] || {};
  const primitiveTypeName = selectedType === 'A' ? 'Actor' : 
    selectedType === 'O' ? 'Object' : 
    selectedType === 'E' ? 'Event' : 
    selectedType === 'C' ? 'Concept' : 'Attribute';
  const availablePrimitives = currentDomainPrimitives[primitiveTypeName] || [];

  const suggestedCounters = suggestCounterPrimitives();

  return (
    <div className="max-w-7xl mx-auto p-6 bg-gradient-to-br from-slate-50 to-blue-50 min-h-screen">
      <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
        <div className="flex items-center gap-3 mb-6">
          <Hash className="w-8 h-8 text-blue-600" />
          <h1 className="text-3xl font-bold text-gray-800">CTAS Enhanced Hash Composer</h1>
          <span className="bg-red-100 text-red-800 px-3 py-1 rounded text-sm font-semibold">
            Hunt→Dominate Integrated
          </span>
        </div>
        
        {/* Layer Selection */}
        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Primitive Layer
          </label>
          <div className="flex gap-4 flex-wrap">
            <label className="flex items-center">
              <input
                type="radio"
                value="operational"
                checked={selectedLayer === 'operational'}
                onChange={(e) => setSelectedLayer(e.target.value)}
                className="mr-2"
              />
              Operational Layer (197 Adversarial Primitives)
            </label>
            <label className="flex items-center">
              <input
                type="radio"
                value="counter"
                checked={selectedLayer === 'counter'}
                onChange={(e) => setSelectedLayer(e.target.value)}
                className="mr-2"
              />
              Counter-Primitive Layer (Hunt→Dominate)
            </label>
            <label className="flex items-center">
              <input
                type="radio"
                value="code"
                checked={selectedLayer === 'code'}
                onChange={(e) => setSelectedLayer(e.target.value)}
                className="mr-2"
              />
              Code Layer (Source Implementation)
            </label>
          </div>
        </div>

        {/* Domain Selection */}
        {selectedLayer === 'operational' && (
          <div className="mb-6">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Operational Domain
            </label>
            <select
              value={selectedDomain}
              onChange={(e) => setSelectedDomain(e.target.value)}
              className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
            >
              <option value="adversarial">Adversarial Intelligence</option>
              <option value="manufacturing">Manufacturing Automation</option>
              <option value="medical">Medical Systems</option>
            </select>
          </div>
        )}

        {/* Primitive Type Selection */}
        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Primitive Type (Position 1)
          </label>
          <select
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value)}
            className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
          >
            {Object.entries(currentPrimitiveTypes).map(([key, desc]) => (
              <option key={key} value={key}>{key} - {desc}</option>
            ))}
          </select>
        </div>

        {/* Domain-Specific Primitives */}
        {selectedLayer === 'operational' && availablePrimitives.length > 0 && (
          <div className="mb-6">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Domain-Specific {primitiveTypeName}s ({selectedDomain})
            </label>
            <select
              value={metadata.description}
              onChange={(e) => setMetadata({...metadata, description: e.target.value})}
              className="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
            >
              <option value="">Select a domain-specific primitive...</option>
              {availablePrimitives.map((primitive) => (
                <option key={primitive} value={primitive}>{primitive}</option>
              ))}
            </select>
          </div>
        )}

        {/* Counter-Primitive Suggestions */}
        {selectedLayer === 'operational' && metadata.description && (
          <div className="mb-6 bg-yellow-50 border border-yellow-200 rounded-lg p-4">
            <h3 className="text-sm font-semibold text-yellow-800 mb-2">
              🎯 Suggested Counter-Primitives for "{metadata.description}"
            </h3>
            <div className="flex gap-2 flex-wrap">
              {suggestedCounters.map((counter) => {
                const counterInfo = counterPrimitives[counter];
                return (
                  <div key={counter} className={`flex items-center gap-2 px-3 py-2 rounded-md ${counterInfo.color}`}>
                    {counterInfo.icon}
                    <span className="font-medium">{counter}</span>
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Counter-Primitive Details */}
        {selectedLayer === 'counter' && (
          <div className="mb-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
            <h3 className="text-lg font-semibold text-blue-800 mb-4">Counter-Primitive Framework</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {Object.entries(counterPrimitives).map(([name, info]) => (
                <div key={name} className={`border rounded-lg p-3 ${info.color} border-opacity-50`}>
                  <div className="flex items-center gap-2 mb-2">
                    {info.icon}
                    <span className="font-bold">{name}</span>
                  </div>
                  <p className="text-sm mb-2">{info.description}</p>
                  <div className="text-xs">
                    <strong>Tactics:</strong>
                    <ul className="list-disc list-inside mt-1">
                      {info.tactics.slice(0, 3).map((tactic) => (
                        <li key={tactic}>{tactic}</li>
                      ))}
                    </ul>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Hash Composition Grid */}
      <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-semibold text-gray-800">16-Position Hash Composition</h2>
          <button
            onClick={randomizeHash}
            className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors"
          >
            <RefreshCw className="w-4 h-4" />
            Randomize
          </button>
        </div>

        <div className="grid grid-cols-4 gap-4 mb-6">
          {positions.map((pos, index) => (
            <div key={index} className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <span className="bg-blue-100 text-blue-800 px-2 py-1 rounded text-sm font-bold">
                  {pos.pos}
                </span>
                <span className="font-medium text-sm">{pos.name}</span>
              </div>
              
              {pos.values ? (
                <select
                  value={hashValues[index]}
                  onChange={(e) => updateHashValue(index, e.target.value)}
                  className="w-full p-2 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                >
                  {Object.entries(pos.values).map(([key, desc]) => (
                    <option key={key} value={key}>{key} - {desc}</option>
                  ))}
                </select>
              ) : pos.pos === 1 ? (
                <select
                  value={hashValues[index]}
                  onChange={(e) => updateHashValue(index, e.target.value)}
                  className="w-full p-2 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                >
                  {Object.entries(currentPrimitiveTypes).map(([key, desc]) => (
                    <option key={key} value={key}>{key} - {desc.split(' - ')[0]}</option>
                  ))}
                </select>
              ) : (
                <div>
                  <input
                    type="text"
                    value={hashValues[index]}
                    onChange={(e) => updateHashValue(index, e.target.value.slice(0, 1))}
                    maxLength={1}
                    className="w-full p-2 border border-gray-300 rounded text-sm text-center font-mono focus:ring-2 focus:ring-blue-500"
                  />
                  {pos.description && (
                    <p className="text-xs text-gray-500 mt-1">{pos.description}</p>
                  )}
                </div>
              )}
              
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
      <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
        <h2 className="text-2xl font-semibold text-gray-800 mb-6">Generated Trivariate Hashes</h2>
        
        <div className="grid grid-cols-1 gap-4">
          {[
            { type: 'SCH', hash: generatedHashes.sch, desc: 'Synaptic Convergent Hash - Operationally Active', color: 'bg-green-100 text-green-800' },
            { type: 'CUID', hash: generatedHashes.cuid, desc: 'Cognitive Unique ID - Context-Aware Identity', color: 'bg-blue-100 text-blue-800' },
            { type: 'UUID', hash: generatedHashes.uuid, desc: 'Universal Unique ID - Persistent Identity', color: 'bg-purple-100 text-purple-800' }
          ].map((item) => (
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

      {/* Export Options */}
      <div className="bg-white rounded-lg shadow-xl p-6">
        <h2 className="text-2xl font-semibold text-gray-800 mb-6">Export & Integration</h2>
        
        <div className="grid grid-cols-2 gap-6">
          <div>
            <h3 className="text-lg font-semibold mb-3">Lisp Integration</h3>
            <div className="bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm overflow-x-auto">
              <pre>{`(:primitive-type :${primitiveTypeName}
 :layer :${selectedLayer}
 :description "${metadata.description}"
 :domain :${selectedDomain}
 :sch-hash "${generatedHashes.sch}"
 :cuid-hash "${generatedHashes.cuid}"
 :uuid-hash "${generatedHashes.uuid}"
 :entropy ${metadata.entropyValue}
 :ttl ${metadata.ttlValue})`}</pre>
            </div>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-3">RDF Triple Export</h3>
            <div className="bg-gray-900 text-blue-400 p-4 rounded-lg font-mono text-sm overflow-x-auto">
              <pre>{`:Node${generatedHashes.sch.slice(0,3)} rdf:type :${primitiveTypeName} ;
  ctas:hasDescription "${metadata.description}" ;
  ctas:hasSCH "${generatedHashes.sch}" ;
  ctas:hasCUID "${generatedHashes.cuid}" ;
  ctas:hasUUID "${generatedHashes.uuid}" ;
  ctas:hasEntropy "${metadata.entropyValue}" ;
  ctas:hasTTL "${metadata.ttlValue}" ;
  ctas:belongsToLayer :${selectedLayer} ;
  ctas:operatesInDomain :${selectedDomain} .`}</pre>
            </div>
          </div>
        </div>
        
        <div className="mt-6 flex gap-4">
          <button
            onClick={() => copyToClipboard(JSON.stringify({
              primitive_type: selectedType,
              layer: selectedLayer,
              description: metadata.description,
              domain: selectedDomain,
              hashes: generatedHashes,
              positions: hashValues,
              suggested_counters: suggestedCounters
            }, null, 2))}
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
    </div>
  );
};

export default CTASHashComposer;