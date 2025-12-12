import React, { useState, useEffect } from 'react';
import { Code, Save, Eye } from 'lucide-react';


const CodeViewer: React.FC = () => {
  const [code, setCode] = useState('');
  const [fileName, setFileName] = useState('');

  useEffect(() => {
    // Fetch the code from the current project
    // This is a placeholder and should be replaced with actual code fetching logic
    setCode(`
ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
    `);
    setFileName('index.tsx');
  }, []);

  const handleSave = () => {
    // Implement save functionality
    console.log('Saving code...');
  };

  const handlePrint = () => {
    window.print();
  };

  const handleVSCodeConnection = () => {
    // Implement VS Code connection
    console.log('Connecting to VS Code...');
  };

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-4">Code Viewer</h2>
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-medium">{fileName}</h3>
          <div>
            <button onClick={handleSave} className="mr-2 p-2 bg-blue-500 text-white rounded hover:bg-blue-600">
              <Save className="w-5 h-5" />
            </button>
            <button onClick={handlePrint} className="mr-2 p-2 bg-green-500 text-white rounded hover:bg-green-600">
              <Eye className="w-5 h-5" />
            </button>
            <button onClick={handleVSCodeConnection} className="p-2 bg-purple-500 text-white rounded hover:bg-purple-600">
              <Code className="w-5 h-5" />
            </button>
          </div>
        </div>
        <pre className="bg-gray-100 p-4 rounded overflow-x-auto">
          <code>{code}</code>
        </pre>
      </div>
    </div>
  );
};

export default CodeViewer;