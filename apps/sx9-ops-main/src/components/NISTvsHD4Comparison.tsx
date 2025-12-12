import React from 'react';
import { ArrowRightCircle } from 'lucide-react';

const ComparisonItem: React.FC<{ nist: string; hd4: string; description: string }> = ({ nist, hd4, description }) => (
  <div className="mb-6 p-4 bg-gray-800 rounded-lg shadow-md">
    <div className="flex justify-between mb-2 items-center">
      <div className="w-5/12">
        <h3 className="text-lg font-semibold text-blue-400">{nist}</h3>
      </div>
      <ArrowRightCircle className="text-gray-500" />
      <div className="w-5/12 text-right">
        <h3 className="text-lg font-semibold text-green-400">{hd4}</h3>
      </div>
    </div>
    <p className="text-gray-300 text-sm mt-2">{description}</p>
  </div>
);

const NISTvsHD4Comparison: React.FC = () => {
  return (
    <div className="bg-gray-900 text-white p-8 rounded-lg shadow-lg">
      <h2 className="text-3xl font-bold mb-6 text-center text-blue-400">NIST Cybersecurity Framework vs HD4 Operational Spectrum</h2>
      
      <ComparisonItem
        nist="Identify"
        hd4="Hunt"
        description="While NIST focuses on understanding assets and risks, HD4's Hunt stage actively seeks out threats and vulnerabilities in the system."
      />
      
      <ComparisonItem
        nist="Protect"
        hd4="Detect"
        description="NIST's Protect function is about safeguarding assets, whereas HD4's Detect stage emphasizes real-time threat detection and analysis."
      />
      
      <ComparisonItem
        nist="Detect"
        hd4="Disable"
        description="Both frameworks emphasize detection, but HD4 goes a step further by including a Disable stage to neutralize identified threats."
      />
      
      <ComparisonItem
        nist="Respond"
        hd4="Disrupt"
        description="NIST's Respond function is about taking action on detected events. HD4's Disrupt stage focuses on actively interfering with threat actors' operations."
      />
      
      <ComparisonItem
        nist="Recover"
        hd4="Dominate"
        description="While NIST emphasizes returning to normal operations, HD4's Dominate stage aims to establish long-term superiority over threat actors."
      />
      
      <div className="mt-8 text-center">
        <p className="text-gray-400 text-sm italic">
          Note: While both frameworks aim to improve cybersecurity, HD4 takes a more proactive and offensive approach compared to NIST's defensive stance.
        </p>
      </div>
    </div>
  );
};

export default NISTvsHD4Comparison;