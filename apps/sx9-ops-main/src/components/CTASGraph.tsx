import React, { useEffect, useState } from 'react';
import { getCTASTasks } from '@/utils/database';

const CTASGraph: React.FC = () => {
  const [graphInfo, setGraphInfo] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchGraphInfo();
  }, []);

  const fetchGraphInfo = async () => {
    try {
      const tasks = await getCTASTasks();
      setGraphInfo(`Nodes: ${tasks.length}, Relationships: 0`);
      setError(null);
    } catch (error: unknown) {
      console.error('Error fetching graph info:', error);
      setError('Failed to retrieve graph information. Using local data if available.');
      const localTasks = localStorage.getItem('ctasTasks');
      if (localTasks) {
        const tasks = JSON.parse(localTasks);
        setGraphInfo(`Nodes: ${tasks.length}, Relationships: 0 (local data)`);
      } else {
        setGraphInfo('No graph information available (local or remote)');
      }
    }
  };

  return (
    <div className="bg-white p-4 rounded-lg shadow">
      <h2 className="text-xl font-semibold mb-4">CTAS Relationship Graph</h2>
      {error && <div className="text-yellow-500 mb-2">{error}</div>}
      {graphInfo ? (
        <p>{graphInfo}</p>
      ) : (
        <p>Loading graph information...</p>
      )}
      <button 
        className="mt-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm"
        onClick={fetchGraphInfo}
      >
        Refresh Graph Info
      </button>
    </div>
  );
};

export default CTASGraph;