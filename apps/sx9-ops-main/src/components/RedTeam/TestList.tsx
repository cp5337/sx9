import React from 'react';

interface RedTeamTest {
  id: string;
  name: string;
  tactic: string;
  technique: string;
  description: string;
  phase: 'Hunt' | 'Detect' | 'Disable' | 'Disrupt' | 'Dominate';
}

interface TestListProps {
  tests: RedTeamTest[];
}

const TestList: React.FC<TestListProps> = ({ tests }) => {
  return (
    <div className="space-y-2">
      {tests.map(test => (
        <div key={test.id} className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
          <div className="flex items-center justify-between">
            <span className="text-xs font-semibold">{test.name}</span>
            <span className="text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-0.5 rounded">
              {test.id}
            </span>
          </div>
          <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
            {test.description}
          </p>
          <div className="flex items-center mt-1 space-x-2">
            <span className="text-xs bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 px-2 py-0.5 rounded">
              {test.tactic}
            </span>
            <span className="text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 px-2 py-0.5 rounded">
              {test.technique}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
};

export default TestList;