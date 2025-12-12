import React, { useState } from 'react';
import { FolderOpen, FileText, MoveRight, Search, Tag, Calendar, User, Link } from 'lucide-react';

interface DocMetadata {
  title: string;
  description: string;
  tags: string[];
  lastUpdated: string;
  author?: string;
  relatedDocs?: string[];
  content: string;
}

interface DocCategory {
  id: string;
  name: string;
  path: string;
  description: string;
  criteria: string[];
}

const DocsScreen: React.FC = () => {
  const [selectedDoc, setSelectedDoc] = useState<DocMetadata | null>(null);
  const [selectedCategory, setSelectedCategory] = useState<DocCategory | null>(null);
  const [searchTerm, setSearchTerm] = useState('');

  const categories: DocCategory[] = [
    {
      id: 'architecture',
      name: 'Architecture',
      path: '/docs/architecture',
      description: 'System architecture and design documents',
      criteria: ['system design', 'architecture', 'infrastructure', 'scalability']
    },
    {
      id: 'components',
      name: 'Components',
      path: '/docs/components',
      description: 'Component-specific documentation',
      criteria: ['component', 'module', 'api', 'interface']
    },
    {
      id: 'guides',
      name: 'Guides',
      path: '/docs/guides',
      description: 'User and developer guides',
      criteria: ['guide', 'tutorial', 'how-to', 'manual']
    },
    {
      id: 'protocols',
      name: 'Protocols',
      path: '/docs/protocols',
      description: 'Operational procedures and protocols',
      criteria: ['protocol', 'procedure', 'sop', 'operation']
    },
    {
      id: 'analysis',
      name: 'Analysis',
      path: '/docs/analysis',
      description: 'Analysis reports and findings',
      criteria: ['analysis', 'report', 'assessment', 'study']
    },
    {
      id: 'schemas',
      name: 'Schemas',
      path: '/docs/schemas',
      description: 'Data schemas and models',
      criteria: ['schema', 'model', 'data structure', 'database']
    },
    {
      id: 'integrations',
      name: 'Integrations',
      path: '/docs/integrations',
      description: 'Integration specifications and documentation',
      criteria: ['integration', 'api', 'external', 'connector']
    }
  ];

  const handleMoveDoc = (doc: DocMetadata, category: DocCategory) => {
    // Implementation for moving doc to new category
    console.log(`Moving "${doc.title}" to ${category.path}`);
  };

  const suggestCategory = (doc: DocMetadata): DocCategory | null => {
    // Simple algorithm to suggest category based on content and metadata
    const contentLower = doc.content.toLowerCase();
    const titleLower = doc.title.toLowerCase();
    
    return categories.find(category => 
      category.criteria.some(criterion => 
        contentLower.includes(criterion) || titleLower.includes(criterion)
      )
    ) || null;
  };

  return (
    <div className="h-full bg-gray-100 dark:bg-gray-900 p-4">
      <div className="mb-4">
        <div className="flex items-center space-x-2 mb-4">
          <FolderOpen className="w-5 h-5 text-blue-500" />
          <h1 className="text-lg font-semibold">Documentation Review</h1>
        </div>
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={16} />
          <input
            type="text"
            placeholder="Search documentation..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-4 py-2 rounded-lg border bg-white dark:bg-gray-800 text-sm"
          />
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Document List */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-4">
          <h2 className="text-sm font-semibold mb-4 flex items-center">
            <FileText className="w-4 h-4 mr-2" />
            Documents
          </h2>
          <div className="space-y-2">
            {selectedDoc && (
              <div className="border dark:border-gray-700 rounded-lg p-3">
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-medium">{selectedDoc.title}</h3>
                  <div className="flex items-center space-x-2 text-xs text-gray-500">
                    <Calendar size={12} />
                    <span>{selectedDoc.lastUpdated}</span>
                  </div>
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                  {selectedDoc.description}
                </p>
                <div className="flex flex-wrap gap-2 mb-2">
                  {selectedDoc.tags.map((tag, index) => (
                    <span
                      key={index}
                      className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-xs flex items-center"
                    >
                      <Tag size={10} className="mr-1" />
                      {tag}
                    </span>
                  ))}
                </div>
                {selectedDoc.author && (
                  <div className="flex items-center text-xs text-gray-500">
                    <User size={12} className="mr-1" />
                    {selectedDoc.author}
                  </div>
                )}
                {selectedDoc.relatedDocs && selectedDoc.relatedDocs.length > 0 && (
                  <div className="mt-2">
                    <h4 className="text-xs font-medium mb-1">Related Documents:</h4>
                    <ul className="text-xs space-y-1">
                      {selectedDoc.relatedDocs.map((doc, index) => (
                        <li key={index} className="flex items-center text-blue-500">
                          <Link size={10} className="mr-1" />
                          {doc}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Category Selection */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-4">
          <h2 className="text-sm font-semibold mb-4 flex items-center">
            <FolderOpen className="w-4 h-4 mr-2" />
            Categories
          </h2>
          <div className="space-y-2">
            {categories.map(category => (
              <button
                key={category.id}
                onClick={() => setSelectedCategory(category)}
                className={`w-full text-left p-2 rounded-lg transition-colors duration-200 ${
                  selectedCategory?.id === category.id
                    ? 'bg-blue-50 dark:bg-blue-900/50 border-blue-500'
                    : 'hover:bg-gray-50 dark:hover:bg-gray-700'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-medium">{category.name}</span>
                  <MoveRight size={14} className="text-gray-400" />
                </div>
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {category.description}
                </p>
              </button>
            ))}
          </div>
        </div>

        {/* Preview and Actions */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-4">
          <h2 className="text-sm font-semibold mb-4 flex items-center">
            <FileText className="w-4 h-4 mr-2" />
            Preview
          </h2>
          {selectedDoc && selectedCategory && (
            <div>
              <div className="mb-4">
                <h3 className="font-medium mb-2">Selected Document</h3>
                <p className="text-sm">{selectedDoc.title}</p>
              </div>
              <div className="mb-4">
                <h3 className="font-medium mb-2">Target Category</h3>
                <p className="text-sm">{selectedCategory.name}</p>
                <p className="text-xs text-gray-500 mt-1">{selectedCategory.path}</p>
              </div>
              <button
                onClick={() => handleMoveDoc(selectedDoc, selectedCategory)}
                className="w-full bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 transition-colors duration-200 flex items-center justify-center"
              >
                <MoveRight size={14} className="mr-2" />
                Move Document
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default DocsScreen;