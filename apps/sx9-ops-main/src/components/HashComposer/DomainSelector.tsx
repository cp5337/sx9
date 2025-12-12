import React from 'react';

interface DomainSelectorProps {
  selectedDomain: string;
  onDomainChange: (domain: string) => void;
}

export const DomainSelector: React.FC<DomainSelectorProps> = ({ selectedDomain, onDomainChange }) => {
  const domains = [
    { id: 'adversarial', name: 'Adversarial Intelligence', description: 'Threat actors and operations', icon: '🎯' },
    { id: 'manufacturing', name: 'Manufacturing Automation', description: 'Industrial systems and processes', icon: '🏭' },
    { id: 'medical', name: 'Medical Systems', description: 'Healthcare and clinical operations', icon: '🏥' }
  ];

  return (
    <div>
      <label className="block text-sm font-medium text-slate-300 mb-3">
        Operational Domain
      </label>
      <select
        value={selectedDomain}
        onChange={(e) => onDomainChange(e.target.value)}
        className="w-full p-3 bg-slate-700/50 border border-slate-600 rounded-lg text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      >
        {domains.map((domain) => (
          <option key={domain.id} value={domain.id}>
            {domain.icon} {domain.name} - {domain.description}
          </option>
        ))}
      </select>
    </div>
  );
};
