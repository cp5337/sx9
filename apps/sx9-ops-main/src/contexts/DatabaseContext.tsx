import React, { createContext, useContext, useState } from 'react';
import { Database } from 'lucide-react';

interface DatabaseConnectionState {
  mongodb: {
    isConnected: boolean;
    url: string;
    databases: string[];
    collections: Record<string, string[]>;
    error: string | null;
  };
}

interface DatabaseContextType {
  connectionState: DatabaseConnectionState;
  setMongoDBConnection: (state: boolean) => void;
  setMongoDBDatabases: (databases: string[]) => void;
  setMongoDBCollections: (collections: Record<string, string[]>) => void;
  setMongoDBError: (error: string | null) => void;
}

const initialState: DatabaseConnectionState = {
  mongodb: {
    isConnected: false,
    url: 'mongodb://localhost:27017/',
    databases: [],
    collections: {},
    error: null
  }
};

const DatabaseContext = createContext<DatabaseContextType | undefined>(undefined);

export const DatabaseProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [connectionState, setConnectionState] = useState<DatabaseConnectionState>(initialState);

  const setMongoDBConnection = (state: boolean) => {
    setConnectionState(prev => ({
      ...prev,
      mongodb: { ...prev.mongodb, isConnected: state }
    }));
  };

  const setMongoDBDatabases = (databases: string[]) => {
    setConnectionState(prev => ({
      ...prev,
      mongodb: { ...prev.mongodb, databases }
    }));
  };

  const setMongoDBCollections = (collections: Record<string, string[]>) => {
    setConnectionState(prev => ({
      ...prev,
      mongodb: { ...prev.mongodb, collections }
    }));
  };

  const setMongoDBError = (error: string | null) => {
    setConnectionState(prev => ({
      ...prev,
      mongodb: { ...prev.mongodb, error }
    }));
  };

  return (
    <DatabaseContext.Provider value={{
      connectionState,
      setMongoDBConnection,
      setMongoDBDatabases,
      setMongoDBCollections,
      setMongoDBError
    }}>
      {children}
    </DatabaseContext.Provider>
  );
};

export const useDatabaseConnection = () => {
  const context = useContext(DatabaseContext);
  if (context === undefined) {
    throw new Error('useDatabaseConnection must be used within a DatabaseProvider');
  }
  return context;
};