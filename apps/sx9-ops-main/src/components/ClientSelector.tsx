import React from 'react';
import { Building } from 'lucide-react';

interface Client {
  id: string;
  name: string;
  selected: boolean;
}

interface ClientSelectorProps {
  clients: Client[];
  onClientChange: (clientId: string) => void;
}

const ClientSelector: React.FC<ClientSelectorProps> = ({ clients, onClientChange }) => {
  return (
    <div className="flex items-center space-x-2 overflow-x-auto py-2">
      {clients.map(client => (
        <button
          key={client.id}
          onClick={() => onClientChange(client.id)}
          className={`flex items-center px-2 py-1 rounded-full text-xs whitespace-nowrap ${
            client.selected 
              ? 'bg-purple-500 text-white' 
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200'
          }`}
        >
          <Building className="w-3 h-3 mr-1" />
          {client.name}
        </button>
      ))}
    </div>
  );
};

export default ClientSelector;