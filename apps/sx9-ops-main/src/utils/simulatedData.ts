import { v4 as uuidv4 } from 'uuid';

export const simulatedPhases = [
  {
    id: '1',
    name: '1.0 Pre-Operational Planning',
    tasks: [
      {
        id: '1.1',
        name: 'OSINT Collection',
        status: 'In Progress',
        priority: 'High'
      },
      {
        id: '1.2',
        name: 'Network Scanning',
        status: 'Pending',
        priority: 'Medium'
      }
    ]
  },
  {
    id: '2',
    name: '2.0 Weapon and Tool Preparation',
    tasks: [
      {
        id: '2.1',
        name: 'Exploitation Frameworks',
        status: 'Completed',
        priority: 'Critical'
      }
    ]
  }
];

export const simulatedRelationships = [
  {
    id: '1',
    source: '1.1',
    target: '1.2',
    type: 'SUPPORTS',
    dependencyLevel: 'High'
  },
  {
    id: '2',
    source: '1.2',
    target: '2.1',
    type: 'LEADS_TO',
    dependencyLevel: 'Critical'
  }
];

export const simulatedMetrics = {
  totalNodes: 150,
  totalRelationships: 450,
  activePhases: 3,
  completedTasks: 25,
  inProgressTasks: 15,
  pendingTasks: 10,
  averageCompletionTime: '4.5 days',
  successRate: '85%'
};