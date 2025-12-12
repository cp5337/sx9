import React, { useState } from 'react';
import { Brain, RefreshCw, BarChart2 } from 'lucide-react';

interface SVMStats {
  vectorDimension: number;
  totalVectors: number;
  accuracy: number;
  lastTraining: string;
}

const PineconeSVM: React.FC = () => {
  const [svmStats, setSvmStats] = useState<SVMStats>({
    vectorDimension: 1536,
    totalVectors: 1000000,
    accuracy: 0.95,
    lastTraining: '2023-06-15 08:30:00',
  });

  const handleRetrain = () => {
    // Simulating retraining process
    setSvmStats(prev => ({
      ...prev,
      accuracy: Math.min(0.99, prev.accuracy + Math.random() * 0.05),
      lastTraining: new Date().toISOString().slice(0, 19).replace('T', ' '),
    }));
  };

  return (
    <div className="bg-gray-800 text-white p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Brain className="mr-2" size={24} />
        Pinecone SVM for DVM
      </h2>
      <div className="grid grid-cols-2 gap-4">
        <div className="bg-gray-700 p-3 rounded-lg">
          <h3 className="font-semibold mb-2 flex items-center">
            <BarChart2 className="mr-2" size={16} />
            SVM Statistics
          </h3>
          <p>Vector Dimension: {svmStats.vectorDimension}</p>
          <p>Total Vectors: {svmStats.totalVectors.toLocaleString()}</p>
          <p>Accuracy: {(svmStats.accuracy * 100).toFixed(2)}%</p>
          <p>Last Training: {svmStats.lastTraining}</p>
        </div>
        <div className="bg-gray-700 p-3 rounded-lg flex flex-col justify-between">
          <h3 className="font-semibold mb-2">Actions</h3>
          <button 
            onClick={handleRetrain}
            className="bg-blue-500 text-white px-4 py-2 rounded flex items-center justify-center hover:bg-blue-600"
          >
            <RefreshCw className="mr-2" size={16} />
            Retrain SVM
          </button>
        </div>
      </div>
    </div>
  );
};

export default PineconeSVM;