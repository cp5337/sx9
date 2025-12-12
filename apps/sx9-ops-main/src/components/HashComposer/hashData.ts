import type { DomainPrimitives, CounterPrimitive, PositionDefinition } from './types';

export const domainPrimitives: DomainPrimitives = {
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

export const counterPrimitives: Record<string, CounterPrimitive> = {
  Hunt: {
    name: 'Hunt',
    icon: 'Target',
    description: 'Active search and discovery operations',
    color: 'bg-red-500/20 text-red-400 border-red-500/30',
    tactics: ['Reconnaissance', 'Intelligence Gathering', 'Pattern Analysis', 'Behavioral Tracking', 'Network Mapping']
  },
  Detect: {
    name: 'Detect',
    icon: 'Eye',
    description: 'Monitoring and anomaly recognition',
    color: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
    tactics: ['Sensor Monitoring', 'Signature Analysis', 'Threshold Detection', 'Behavioral Analysis', 'Alert Generation']
  },
  Disrupt: {
    name: 'Disrupt',
    icon: 'Zap',
    description: 'Active interference and path collapse',
    color: 'bg-orange-500/20 text-orange-400 border-orange-500/30',
    tactics: ['Signal Jamming', 'Resource Denial', 'Pathway Blocking', 'Communication Interference', 'Process Interruption']
  },
  Disable: {
    name: 'Disable',
    icon: 'Ban',
    description: 'Neutralization and capability removal',
    color: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
    tactics: ['System Shutdown', 'Resource Exhaustion', 'Access Revocation', 'Function Termination', 'Capability Nullification']
  },
  Dominate: {
    name: 'Dominate',
    icon: 'Crown',
    description: 'Control establishment and maintenance',
    color: 'bg-green-500/20 text-green-400 border-green-500/30',
    tactics: ['System Control', 'Authority Assertion', 'Structure Locking', 'Behavioral Override', 'Operational Superiority']
  }
};

export const counterMappings: Record<string, string[]> = {
  'Bomb Deployment': ['Hunt', 'Detect', 'Disable'],
  'Cyber Attack': ['Detect', 'Disrupt', 'Dominate'],
  'Reconnaissance Mission': ['Hunt', 'Detect', 'Disrupt'],
  'Cell Meeting': ['Hunt', 'Detect', 'Disrupt'],
  'Fund Transfer': ['Detect', 'Disrupt', 'Disable'],
  'Training Session': ['Hunt', 'Disrupt', 'Disable'],
  'Target Surveillance': ['Hunt', 'Detect', 'Disrupt']
};

export const positions: PositionDefinition[] = [
  {
    id: '1',
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
    id: '2',
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
    id: '3',
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
  { id: '4', name: 'XOR Baseline', description: 'Starting XOR key for mutation detection' },
  { id: '5', name: 'Operator Signature', description: 'Operator assigned at commit/build' },
  { id: '6', name: 'Temporal Tail Seed', description: 'Time-of-capture encoding' },
  {
    id: '7',
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
    id: '8',
    name: 'Graph Role Code',
    values: {
      'S': 'Source - Origin point',
      'K': 'Sink - Terminal point',
      'R': 'Relay - Pass-through node',
      'H': 'Hub - Connection center',
      'B': 'Bridge - Cross-domain link'
    }
  },
  { id: '9', name: 'Mutation Index', description: 'Update lineage counter' },
  { id: '10', name: 'SHA/Checksum Hook', description: 'Integrity validation' },
  { id: '11', name: 'Context Frame ID', description: 'Lisp frame reference' },
  { id: '12', name: 'RDF Vector Reference', description: 'Triple pointer' },
      {
      id: '13',
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
      id: '14',
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
      id: '15',
      name: 'Storage Tier Marker',
    values: {
      'H': 'Hot - Active memory',
      'W': 'Warm - Cache storage',
      'C': 'Cold - Archive storage'
    }
  },
      {
      id: '16',
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
