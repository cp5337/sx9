import React, { useState } from 'react';
import { ToggleLeft, ToggleRight } from 'lucide-react';

const ThreatFeeds: React.FC = () => {
  const [feeds, setFeeds] = useState([
    { id: 1, name: 'AlienVault OTX', enabled: true },
    { id: 2, name: 'IBM X-Force Exchange', enabled: true },
    { id: 3, name: 'Anomali LIMO', enabled: false },
    { id: 4, name: 'MISP', enabled: true },
    { id: 5, name: 'Recorded Future', enabled: false },
  ]);

  const toggleFeed = (id: number) => {
    setFeeds(feeds.map(feed => 
      feed.id === id ? { ...feed, enabled: !feed.enabled } : feed
    ));
  };

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-4">Threat Information Feeds</h2>
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Feed Name</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Action</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {feeds.map((feed) => (
              <tr key={feed.id}>
                <td className="px-6 py-4 whitespace-nowrap">{feed.name}</td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${feed.enabled ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
                    {feed.enabled ? 'Enabled' : 'Disabled'}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <button
                    onClick={() => toggleFeed(feed.id)}
                    className="text-indigo-600 hover:text-indigo-900"
                  >
                    {feed.enabled ? (
                      <ToggleRight className="w-5 h-5" />
                    ) : (
                      <ToggleLeft className="w-5 h-5" />
                    )}
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ThreatFeeds;