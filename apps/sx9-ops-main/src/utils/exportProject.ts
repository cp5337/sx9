import JSZip from 'jszip';
import { saveAs } from 'file-saver';

export const getProjectFiles = async (): Promise<{ name: string; content: string }[]> => {
  const files = [
    { name: 'package.json', path: '/package.json' },
    { name: 'src/App.tsx', path: '/src/App.tsx' },
    { name: 'src/main.tsx', path: '/src/main.tsx' },
    { name: 'src/index.css', path: '/src/index.css' },
    // Add more files as needed
  ];

  const fileContents = await Promise.all(
    files.map(async (file) => ({
      name: file.name,
      content: await fetch(file.path).then((res) => res.text()),
    }))
  );

  return fileContents;
};

export const exportProject = async () => {
  const zip = new JSZip();
  const files = await getProjectFiles();

  files.forEach((file) => {
    zip.file(file.name, file.content);
  });

  const content = await zip.generateAsync({ type: 'blob' });
  saveAs(content, 'ctas-project.zip');
};

export const downloadFile = (fileName: string, content: string) => {
  const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
  saveAs(blob, fileName);
};