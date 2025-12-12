import React from 'react';
import SupabaseManagement from './SupabaseManagement';
import SurrealDBManagement from './SurrealDBManagement';
import SledManagement from './SledManagement';
import SlotGraphManagement from './SlotGraphManagement';
import BevyManagement from './BevyManagement';
import { Database } from 'lucide-react';


const DatabaseManagement: React.FC = () => {
  return (
    <div className="space-y-4">
      <h2 className="text-sm font-semibold flex items-center">
        <Database className="mr-2" size={14} />
        Database Management
      </h2>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <SupabaseManagement />
        <SurrealDBManagement />
        <SledManagement />
        <SlotGraphManagement />
        <BevyManagement />
      </div>
    </div>
  );
};

export default DatabaseManagement;