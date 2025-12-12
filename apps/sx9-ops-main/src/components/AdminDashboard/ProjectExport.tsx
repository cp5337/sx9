import React, { useState, useEffect } from 'react';
import { Download, File } from 'lucide-react';
import { exportProject, getProjectFiles, downloadFile } from '@/components/../utils/exportProject';

interface ProjectFile {
  name: string;
  content: string;
}

const ProjectExport: React.FC = () => {
  const [files, setFiles] = useState<ProjectFile[]>([]);

  useEffect(() => {
    const fetchFiles = async () => {
      const projectFiles = await getProjectFiles();
      setFiles(projectFiles);
    };
    fetchFiles();
  }, []);

  const handleExportAll = async () => {
    try {
      await exportProject();
      console.log('Project exported successfully');
    } catch (error) {
      console.error('Error exporting project:', error);
    }
  };

  const handleDownloadFile = async (fileName: string, content: string) => {
    try {
      await downloadFile(fileName, content);
      console.log(`File ${fileName} downloaded successfully`);
    } catch (error) {
      console.error(`Error downloading file ${fileName}:`, error);
    }
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-2xl font-semibold mb-4">Project Export</h2>
      <button
        onClick={handleExportAll}
        className="mb-4 flex items-center bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
      >
        <Download className="w-5 h-5 mr-2" />
        Export All Files
      </button>
      <div className="space-y-2">
        {files.map((file, index) => (
          <div key={index} className="flex items-center justify-between bg-gray-100 p-2 rounded">
            <span className="flex items-center">
              <File className="w-5 h-5 mr-2 text-gray-500" />
              {file.name}
            </span>
            <button
              onClick={() => handleDownloadFile(file.name, file.content)}
              className="text-blue-500 hover:text-blue-700"
            >
              Download
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ProjectExport;