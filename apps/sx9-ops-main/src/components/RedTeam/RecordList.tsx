import React from 'react';

interface PhaseRecord {
  id: string;
  type: string;
  description: string;
  source: string;
  relatedPhases: string[];
}

interface RecordListProps {
  records: PhaseRecord[];
}

const RecordList: React.FC<RecordListProps> = ({ records }) => {
  return (
    <div className="space-y-2">
      {records.map(record => (
        <div key={record.id} className="bg-gray-50 dark:bg-gray-700 p-2 rounded">
          <div className="flex items-center justify-between">
            <span className="text-xs font-semibold">{record.type}</span>
            <span className="text-xs bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 px-2 py-0.5 rounded">
              {record.id}
            </span>
          </div>
          <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
            {record.description}
          </p>
          <div className="flex items-center mt-1">
            <span className="text-xs bg-gray-100 dark:bg-gray-600 px-2 py-0.5 rounded">
              Source: {record.source}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
};

export default RecordList;