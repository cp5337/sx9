import React, { useEffect, useRef } from 'react';
import mermaid from 'mermaid';
import { Database } from 'lucide-react';

const SystemArchitectureDiagram: React.FC = () => {
  const mermaidRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    mermaid.initialize({ startOnLoad: true, theme: 'default' });
    mermaid.run();
  }, []);

  const diagramDefinition = `
    graph TD
    A[Frontend] --> B[API Gateway]
    B --> C[Authentication Service]
    B --> D[Threat Analysis Service]
    B --> E[Task Management Service]
    C --> F[(User Database)]
    D --> G[(Threat Database)]
    E --> H[(Task Database)]
    D --> I[External Threat Feeds]
    D --> J[ML/AI Engine]
  `;

  return (
    <div className="w-full bg-white rounded-lg shadow-lg p-4 mt-6">
      <h2 className="text-xl font-semibold mb-4">System Architecture Diagram</h2>
      <div ref={mermaidRef} className="mermaid">
        {diagramDefinition}
      </div>
    </div>
  );
};

export default SystemArchitectureDiagram;