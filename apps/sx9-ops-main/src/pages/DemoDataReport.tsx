import React from 'react';
import DemoDataReport from '@/components/DemoDataReport';

const DemoDataReportPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-6">
      <div className="max-w-7xl mx-auto">
        <DemoDataReport />
      </div>
    </div>
  );
};

export default DemoDataReportPage;
