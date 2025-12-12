export interface ThreatActor {
  id: string;
  name: string;
  description: string;
  created: string;
  modified: string;
  aliases: string[];
  sophistication: string;
  resource_level: string;
  primary_motivation: string;
  goals: string[];
  latitude: string;
  longitude: string;
}

export interface Indicator {
  id: string;
  name: string;
  description: string;
  pattern_type: string;
  pattern: string;
  valid_from: string;
  valid_until: string;
  x_opencti_score: number;
  created: string;
  modified: string;
}

export interface Relationship {
  id: string;
  relationship_type: string;
  description: string;
  start_time: string;
  stop_time: string;
  from: {
    id: string;
    name: string;
  };
  to: {
    id: string;
    name: string;
  };
}