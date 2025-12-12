import React from 'react';

const SpreadsheetDisplay: React.FC = () => {
  return (
    <div className="bg-white dark:bg-gray-800 text-gray-800 dark:text-white p-4 rounded-lg shadow transition-colors duration-200">
      <h2 className="text-xl font-semibold mb-4">Spreadsheet Display</h2>
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead className="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Type</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Status</th>
            </tr>
          </thead>
          <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            {/* Add your table rows here */}
            <tr>
              <td className="px-6 py-4 whitespace-nowrap">Example Data</td>
              <td className="px-6 py-4 whitespace-nowrap">Example Type</td>
              <td className="px-6 py-4 whitespace-nowrap">Active</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default SpreadsheetDisplay;