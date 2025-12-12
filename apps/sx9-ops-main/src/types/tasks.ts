export interface ThreatActor {
  id: string;
  name: string;
  type: string;
}

export interface CTASTask {
  id: string;
  number: string;
  title: string;
  description: string;
  status: 'Pending' | 'In Progress' | 'Completed';
  relatedActorId?: string;
  isSection: boolean;
}