import { v4 as uuidv4 } from 'uuid';

export interface SearchResult {
  id: string;
  title: string;
  description: string;
  type: 'task' | 'threat' | 'system' | 'document';
  relevance: number;
  tags: string[];
}

export const searchSimulator = {
  search: async (query: string): Promise<SearchResult[]> => {
    // Simulate search delay
    await new Promise(resolve => setTimeout(resolve, 300));
    
    const mockResults: SearchResult[] = [
      {
        id: '1',
        title: 'Network Reconnaissance',
        description: 'Conduct comprehensive network scanning and enumeration',
        type: 'task',
        relevance: 0.95,
        tags: ['hunt', 'network', 'scanning']
      },
      {
        id: '2',
        title: 'APT29 Threat Actor',
        description: 'Advanced Persistent Threat group known for sophisticated attacks',
        type: 'threat',
        relevance: 0.87,
        tags: ['apt', 'threat-actor', 'russia']
      },
      {
        id: '3',
        title: 'CTAS System Overview',
        description: 'Comprehensive Threat Analysis System documentation',
        type: 'document',
        relevance: 0.82,
        tags: ['system', 'documentation', 'overview']
      }
    ];

    // Filter results based on query
    return mockResults.filter(result => 
      result.title.toLowerCase().includes(query.toLowerCase()) ||
      result.description.toLowerCase().includes(query.toLowerCase()) ||
      result.tags.some(tag => tag.toLowerCase().includes(query.toLowerCase()))
    );
  },

  getSuggestions: async (query: string): Promise<string[]> => {
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const suggestions = [
      'network scanning',
      'threat detection',
      'vulnerability assessment',
      'APT29',
      'Lazarus Group',
      'CTAS system',
      'HD4 framework'
    ];

    return suggestions.filter(suggestion => 
      suggestion.toLowerCase().includes(query.toLowerCase())
    );
  }
};