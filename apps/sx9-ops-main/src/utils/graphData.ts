export interface GraphNode {
  id: string;
  name: string;
  group: string;
  x?: number;
  y?: number;
}

export interface GraphLink {
  source: string;
  target: string;
  value: number;
}

export interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

export const getGraphData = (): GraphData => {
  return {
    nodes: [
      { id: '1', name: 'APT29', group: 'threat-actor' },
      { id: '2', name: 'Lazarus Group', group: 'threat-actor' },
      { id: '3', name: 'FIN7', group: 'threat-actor' },
      { id: '4', name: 'Task 1.1', group: 'task' },
      { id: '5', name: 'Task 1.2', group: 'task' },
      { id: '6', name: 'Task 2.1', group: 'task' }
    ],
    links: [
      { source: '1', target: '4', value: 1 },
      { source: '2', target: '5', value: 1 },
      { source: '3', target: '6', value: 1 },
      { source: '4', target: '5', value: 1 }
    ]
  };
};