import React, { useEffect, useState } from 'react';
import { getCTASTasks, createCTASTask, updateTaskStatus } from '@/utils/database';
import type { CTASTask } from '@/types/tasks';
import { AlertCircle, CheckCircle, Clock } from 'lucide-react';

const CTASTaskList: React.FC = () => {
  const [tasks, setTasks] = useState<CTASTask[]>([]);
  const [newTaskTitle, setNewTaskTitle] = useState('');
  const [newTaskDescription, setNewTaskDescription] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    try {
      setIsLoading(true);
      const result = await getCTASTasks();
      setTasks(result);
      setError(null);
    } catch (err) {
      console.error('Error fetching CTAS tasks:', err);
      setError('Failed to load CTAS tasks. Please try again later.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newTaskTitle && newTaskDescription) {
      try {
        await createCTASTask((tasks.length + 1).toString(), newTaskTitle, newTaskDescription);
        setNewTaskTitle('');
        setNewTaskDescription('');
        fetchTasks();
      } catch (err) {
        console.error('Error creating CTAS task:', err);
        setError('Failed to create CTAS task. Please try again.');
      }
    }
  };

  const handleUpdateStatus = async (taskId: string, newStatus: 'Pending' | 'In Progress' | 'Completed') => {
    try {
      await updateTaskStatus(taskId, newStatus);
      fetchTasks();
    } catch (err) {
      console.error('Error updating CTAS task status:', err);
      setError('Failed to update task status. Please try again.');
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Completed':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'In Progress':
        return <Clock className="w-5 h-5 text-yellow-500" />;
      default:
        return <AlertCircle className="w-5 h-5 text-red-500" />;
    }
  };

  if (isLoading) {
    return <div className="text-center">Loading tasks...</div>;
  }

  return (
    <div className="mt-6">
      <h2 className="text-xl font-semibold mb-4">CTAS Tasks</h2>
      {error && <div className="text-red-500 mb-4">{error}</div>}
      <form onSubmit={handleAddTask} className="mb-4">
        <input
          type="text"
          value={newTaskTitle}
          onChange={(e) => setNewTaskTitle(e.target.value)}
          placeholder="Task Title"
          className="mr-2 p-2 border rounded"
        />
        <input
          type="text"
          value={newTaskDescription}
          onChange={(e) => setNewTaskDescription(e.target.value)}
          placeholder="Task Description"
          className="mr-2 p-2 border rounded"
        />
        <button type="submit" className="p-2 bg-blue-500 text-white rounded hover:bg-blue-600">Add Task</button>
      </form>
      <ul className="space-y-4">
        {tasks.map((task) => (
          <li key={task.id} className="bg-gray-100 p-4 rounded shadow">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-semibold">{task.title}</h3>
                <p className="text-gray-600">{task.description}</p>
              </div>
              <div className="flex items-center">
                {getStatusIcon(task.status)}
                <span className="ml-2">{task.status}</span>
              </div>
            </div>
            <div className="mt-4 flex space-x-2">
              <button
                onClick={() => handleUpdateStatus(task.id, 'Pending')}
                className="p-2 bg-yellow-500 text-white rounded hover:bg-yellow-600"
              >
                Pending
              </button>
              <button
                onClick={() => handleUpdateStatus(task.id, 'In Progress')}
                className="p-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                In Progress
              </button>
              <button
                onClick={() => handleUpdateStatus(task.id, 'Completed')}
                className="p-2 bg-green-500 text-white rounded hover:bg-green-600"
              >
                Completed
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default CTASTaskList;