import { readFile, writeFile } from 'fs/promises';
import { join } from 'path';
import matter from 'gray-matter';

interface DocMetadata {
  title: string;
  description: string;
  tags: string[];
  lastUpdated: string;
  author?: string;
  relatedDocs?: string[];
  content: string;
}

export const parseMarkdownFile = async (filePath: string): Promise<DocMetadata> => {
  const fileContent = await readFile(filePath, 'utf-8');
  const { data, content } = matter(fileContent);
  
  return {
    title: data.title || '',
    description: data.description || '',
    tags: data.tags || [],
    lastUpdated: data.lastUpdated || new Date().toISOString(),
    author: data.author,
    relatedDocs: data.relatedDocs,
    content: content.trim()
  };
};

export const moveDocToCategory = async (
  docPath: string,
  categoryPath: string,
  metadata: DocMetadata
): Promise<void> => {
  const fileName = docPath.split('/').pop();
  if (!fileName) throw new Error('Invalid document path');

  const newPath = join(categoryPath, fileName);
  
  // Update front matter with category information
  const frontMatter = {
    ...metadata,
    category: categoryPath.split('/').pop(),
    lastUpdated: new Date().toISOString()
  };

  const newContent = matter.stringify(metadata.content, frontMatter);
  await writeFile(newPath, newContent);
};

export const categorizeDoc = (content: string, categories: string[]): string => {
  const contentLower = content.toLowerCase();
  
  // Simple categorization based on keyword frequency
  const categoryScores = categories.map(category => ({
    category,
    score: content.split(category.toLowerCase()).length - 1
  }));

  const bestMatch = categoryScores.reduce((prev, current) => 
    current.score > prev.score ? current : prev
  );

  return bestMatch.category;
};