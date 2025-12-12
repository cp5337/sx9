import React from 'react';
import { User } from 'lucide-react';

interface Persona {
  id: string;
  name: string;
  status: 'Active' | 'Inactive';
}

const personas: Persona[] = [
  { id: '1', name: 'John Smith', status: 'Active' },
  { id: '2', name: 'Alice Johnson', status: 'Active' },
  { id: '3', name: 'Robert Lee', status: 'Inactive' },
  { id: '4', name: 'Emma Wilson', status: 'Active' },
];

const PersonasList: React.FC = () => {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4">
      <h2 className="text-lg font-semibold mb-4 text-gray-800 dark:text-white">Active Personas</h2>
      <ul className="space-y-2">
        {personas.map(persona => (
          <li key={persona.id} className="flex items-center justify-between">
            <div className="flex items-center">
              <User size={20} className="text-gray-500 dark:text-gray-400 mr-2" />
              <span className="text-gray-700 dark:text-gray-300">{persona.name}</span>
            </div>
            <span className={`px-2 py-1 text-xs font-semibold rounded-full ${
              persona.status === 'Active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
            }`}>
              {persona.status}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default PersonasList;