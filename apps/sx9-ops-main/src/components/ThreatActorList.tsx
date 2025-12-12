import React, { useEffect, useState } from 'react';
import { getThreatActors, createThreatActor } from '@/utils/database';
import { UserPlus, Plus, Users } from 'lucide-react';

interface ThreatActor {
  id: string;
  name: string;
  type: string;
}

const ThreatActorList: React.FC = () => {
  const [actors, setActors] = useState<ThreatActor[]>([]);
  const [newActorName, setNewActorName] = useState('');
  const [newActorType, setNewActorType] = useState('');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchActors();
  }, []);

  const fetchActors = async () => {
    try {
      const fetchedActors = await getThreatActors();
      setActors(fetchedActors);
    } catch (err) {
      console.error('Error fetching threat actors:', err);
      setError('Failed to load threat actors. Please try again later.');
    }
  };

  const handleAddActor = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newActorName && newActorType) {
      try {
        const newActor = await createThreatActor(newActorName, newActorType);
        if (newActor) {
          setActors([...actors, newActor]);
          setNewActorName('');
          setNewActorType('');
        }
      } catch (err) {
        console.error('Error creating threat actor:', err);
        setError('Failed to create threat actor. Please try again.');
      }
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        <h2 className="text-2xl font-bold flex items-center">
          <Users className="mr-2" />
          Threat Actors
        </h2>
      </div>
      <div className="card-body">
        {error && <div className="text-red-500 mb-4">{error}</div>}
        <form onSubmit={handleAddActor} className="mb-4">
          <div className="mb-2">
            <label htmlFor="actorName" className="label">Actor Name</label>
            <input
              id="actorName"
              type="text"
              value={newActorName}
              onChange={(e) => setNewActorName(e.target.value)}
              placeholder="Enter actor name"
              className="input"
            />
          </div>
          <div className="mb-2">
            <label htmlFor="actorType" className="label">Actor Type</label>
            <input
              id="actorType"
              type="text"
              value={newActorType}
              onChange={(e) => setNewActorType(e.target.value)}
              placeholder="Enter actor type"
              className="input"
            />
          </div>
          <button type="submit" className="btn btn-primary w-full flex items-center justify-center">
            <UserPlus className="mr-2" />
            Add Actor
          </button>
        </form>
        <ul className="space-y-2">
          {actors.map((actor) => (
            <li key={actor.id} className="bg-gray-700 p-3 rounded-md flex justify-between items-center">
              <span className="font-semibold">{actor.name}</span>
              <span className="text-sm text-gray-300">{actor.type}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default ThreatActorList;