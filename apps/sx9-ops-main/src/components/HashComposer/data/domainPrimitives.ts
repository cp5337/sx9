import { DomainPrimitives } from '../types';

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
      'Technical Skills', 'Quality Focus', 'Safety Awareness', 'Process Knowledge', 'Team Coordination',
      'Problem Solving', 'Equipment Operation', 'Regulatory Compliance', 'Continuous Improvement'
    ]
  },
  medical: {
    Actor: [
      'Surgeon', 'Nurse', 'Anesthesiologist', 'Radiologist', 'Pharmacist',
      'Medical Technician', 'Patient Care Coordinator', 'Medical Device Specialist'
    ],
    Object: [
      'Surgical Instruments', 'Medical Imaging Equipment', 'Patient Monitor', 'Medication Dispenser',
      'Electronic Health Records', 'Medical Devices', 'Laboratory Equipment', 'Emergency Equipment'
    ],
    Event: [
      'Surgery', 'Patient Assessment', 'Medication Administration', 'Diagnostic Testing',
      'Emergency Response', 'Patient Transfer', 'Medical Consultation', 'Treatment Planning'
    ],
    Concept: [
      'Patient Safety', 'Evidence-Based Medicine', 'Clinical Protocols', 'Medical Ethics',
      'Quality Assurance', 'Risk Management', 'Continuity of Care', 'Interdisciplinary Collaboration'
    ],
    Attribute: [
      'Clinical Expertise', 'Patient Communication', 'Critical Thinking', 'Attention to Detail',
      'Compassion', 'Technical Proficiency', 'Decision Making', 'Team Collaboration'
    ]
  }
};
