import React from 'react';
import { Code } from 'lucide-react';

interface PythonTool {
  name: string;
  description: string;
  useCase: string;
}

interface PythonIntegrationProps {
  hd4Action: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

const PythonIntegration: React.FC<PythonIntegrationProps> = ({ hd4Action }) => {
  const pythonTools: Record<string, PythonTool[]> = {
    Hunt: [
      { name: 'Spyder', description: 'Scientific Python Development Environment', useCase: 'Data analysis and visualization for threat hunting' },
      { name: 'Scrapy', description: 'Web scraping framework', useCase: 'Gathering open-source intelligence' },
      { name: 'Pandas', description: 'Data manipulation library', useCase: 'Processing and analyzing large datasets' },
    ],
    Detect: [
      { name: 'Scikit-learn', description: 'Machine learning library', useCase: 'Building anomaly detection models' },
      { name: 'PyTorch', description: 'Deep learning framework', useCase: 'Developing advanced threat detection algorithms' },
      { name: 'Matplotlib', description: 'Plotting library', useCase: 'Visualizing network traffic patterns' },
    ],
    Disable: [
      { name: 'Scapy', description: 'Packet manipulation library', useCase: 'Creating custom network packets for testing' },
      { name: 'Paramiko', description: 'SSH library', useCase: 'Automating remote system interactions' },
      { name: 'Fabric', description: 'Remote execution library', useCase: 'Coordinating actions across multiple systems' },
    ],
    Disrupt: [
      { name: 'Twisted', description: 'Event-driven networking engine', useCase: 'Building custom network protocols and services' },
      { name: 'PyQT', description: 'GUI framework', useCase: 'Creating interfaces for disruption tools' },
      { name: 'Asyncio', description: 'Asynchronous I/O library', useCase: 'Handling multiple concurrent operations' },
    ],
    Dominate: [
      { name: 'NetworkX', description: 'Network analysis library', useCase: 'Analyzing and visualizing network topologies' },
      { name: 'Flask', description: 'Web framework', useCase: 'Building lightweight web interfaces for control systems' },
      { name: 'Celery', description: 'Distributed task queue', useCase: 'Managing distributed operations across systems' },
    ],
  };

  const relevantTools = pythonTools[hd4Action] || [];

  return (
    <div className="bg-gray-800 p-4 rounded-lg mt-4">
      <h2 className="text-xl font-bold mb-4 flex items-center">
        <Code className="mr-2" size={20} />
        Python Tools for {hd4Action}
      </h2>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {relevantTools.map((tool, index) => (
          <div key={index} className="bg-gray-700 p-3 rounded-lg">
            <h3 className="font-semibold mb-2">{tool.name}</h3>
            <p className="text-sm text-gray-400 mb-2">{tool.description}</p>
            <p className="text-xs text-blue-300">Use Case: {tool.useCase}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default PythonIntegration;