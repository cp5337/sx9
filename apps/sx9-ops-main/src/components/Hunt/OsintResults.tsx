import React from 'react';
import { Search, Globe, Users } from 'lucide-react';
import { OsintResult } from '@/components/../types/huntTypes';

interface OsintResultsProps {
  results: OsintResult;
}

const OsintResults: React.FC<OsintResultsProps> = ({ results }) => {
  return (
    <div className="space-y-4">
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-sm font-semibold flex items-center">
            <Search className="w-4 h-4 mr-2" />
            OSINT Results: {results.target}
          </h2>
          <span className="text-xs text-gray-500">
            Generated: {new Date(results.timestamp).toLocaleString()}
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <h3 className="text-xs font-semibold flex items-center mb-2">
              <Globe className="w-3 h-3 mr-1" />
              Domain Information
            </h3>
            <div className="bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs">
              <p><span className="font-semibold">Registrar:</span> {results.domainInfo.registrar}</p>
              <p><span className="font-semibold">Created:</span> {new Date(results.domainInfo.creationDate).toLocaleDateString()}</p>
              <p><span className="font-semibold">Expires:</span> {new Date(results.domainInfo.expiryDate).toLocaleDateString()}</p>
              <p><span className="font-semibold">Status:</span> {results.domainInfo.status}</p>
            </div>
          </div>

          <div>
            <h3 className="text-xs font-semibold flex items-center mb-2">
              <Users className="w-3 h-3 mr-1" />
              Social Media Presence
            </h3>
            <div className="space-y-2">
              {results.socialProfiles.map((profile, index) => (
                <div key={index} className="bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs">
                  <div className="flex justify-between items-center">
                    <span className="font-semibold">{profile.platform}</span>
                    {profile.verified && (
                      <span className="text-blue-500 text-xs">Verified</span>
                    )}
                  </div>
                  <p className="text-blue-500 hover:underline">
                    <a href={profile.url} target="_blank" rel="noopener noreferrer">
                      @{profile.username}
                    </a>
                  </p>
                  {profile.followers && (
                    <p className="text-gray-500">{profile.followers.toLocaleString()} followers</p>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="mt-4">
          <h3 className="text-xs font-semibold mb-2">Digital Footprint</h3>
          <div className="bg-gray-50 dark:bg-gray-700 p-2 rounded text-xs">
            {results.digitalFootprint.map((item: any, index) => (
              <div key={index} className="mb-2 last:mb-0">
                <p><span className="font-semibold">IP:</span> {item.ip}</p>
                <p><span className="font-semibold">Location:</span> {item.location.city}, {item.location.country}</p>
                <p><span className="font-semibold">Services:</span> {item.services.map((s: any) => `${s.service} (${s.port})`).join(', ')}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default OsintResults;