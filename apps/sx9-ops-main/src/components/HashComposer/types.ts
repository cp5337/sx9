import React from 'react';

export interface HashMetadata {
  slug: string;
  description: string;
  tags: string[];
  timestamp: string;
  ttlValue: number;
  entropyValue: number;
  threatLevel: string;
}

export interface GeneratedHashes {
  sch: string;
  cuid: string;
  uuid: string;
}

export interface CounterPrimitive {
  icon: React.ReactNode;
  name: string;
  description: string;
  color: string;
  tactics: string[];
}

export interface DomainPrimitives {
  [domain: string]: {
    Actor: string[];
    Object: string[];
    Event: string[];
    Concept: string[];
    Attribute: string[];
  };
}

export interface PositionDefinition {
  id: string;
  name: string;
  operational?: Record<string, string>;
  code?: Record<string, string>;
  counter?: Record<string, string>;
  pos?: number;
  values?: Record<string, string>;
  description?: string;
}

export interface HashComposerProps {
  // Add any props if needed
}
