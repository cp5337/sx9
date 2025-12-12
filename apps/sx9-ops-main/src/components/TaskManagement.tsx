import React, { useState, useEffect } from 'react';
import { PlusCircle, CheckCircle, AlertTriangle, Plus, Clock } from 'lucide-react';
import { getCTASTasks, createCTASTask, updateTaskStatus } from '@/utils/database';

interface Task {
  id: string;
  number: string;
  title: string;
  description: string;
  status: 'Pending' | 'In Progress' | 'Completed';
  relatedActorId?: string;
  isSection: boolean;
}

const TaskManagement: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTask, setNewTask] = useState({ title: '', description: '' });

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    const fetchedTasks = await getCTASTasks();
    setTasks(fetchedTasks);
  };

  const handleCreateTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newTask.title && newTask.description) {
      const createdTask = await createCTASTask(
        (tasks.length + 1).toString(),
        newTask.title,
        newTask.description
      );
      if (createdTask) {
        setTasks([...tasks, createdTask]);
        setNewTask({ title: '', description: '' });
      }
    }
  };

  const handleUpdateTaskStatus = async (taskId: string, newStatus: 'Pending' | 'In Progress' | 'Completed') => {
    const updatedTask = await updateTaskStatus(taskId, newStatus);
    if (updatedTask) {
      setTasks(tasks.map(task => task.id === taskId ? updatedTask : task));
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Completed':
        return <CheckCircle className="text-green-500" />;
      case 'In Progress':
        return <Clock className="text-yellow-500" />;
      default:
        return <AlertTriangle className="text-red-500" />;
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        <h2 className="text-2xl font-bold flex items-center">
          <PlusCircle className="mr-2" />
          Task Management
        </h2>
      </div>
      <div className="card-body">
        <form onSubmit={handleCreateTask} className="mb-4">
          <div className="mb-2">
            <label htmlFor="taskTitle" className="label">Task Title</label>
            <input
              id="taskTitle"
              type="text"
              value={newTask.title}
              onChange={(e) => setNewTask({ ...newTask, title: e.target.value })}
              placeholder="Enter task title"
              className="input"
            />
          </div>
          <div className="mb-2">
            <label htmlFor="taskDescription" className="label">Task Description</label>
            <textarea
              id="taskDescription"
              value={newTask.description}
              onChange={(e) => setNewTask({ ...newTask, description: e.target.value })}
              placeholder="Enter task description"
              className="input"
              rows={3}
            ></textarea>
          </div>
          <button type="submit" className="btn btn-primary w-full flex items-center justify-center">
            <PlusCircle className="mr-2" />
            Add Task
          </button>
        </form>
        <ul className="space-y-4">
          {tasks.map((task) => (
            <li key={task.id} className="bg-gray-700 p-4 rounded-md">
              <div className="flex justify-between items-center mb-2">
                <h3 className="font-semibold">{task.title}</h3>
                <div className="flex items-center">
                  {getStatusIcon(task.status)}
                  <span className="ml-2 text-sm">{task.status}</span>
                </div>
              </div>
              <p className="text-sm text-gray-300 mb-4">{task.description}</p>
              <div className="flex space-x-2">
                <button
                  onClick={() => handleUpdateTaskStatus(task.id, 'Pending')}
                  className="btn btn-secondary text-xs"
                >
                  Pending
                </button>
                <button
                  onClick={() => handleUpdateTaskStatus(task.id, 'In Progress')}
                  className="btn btn-secondary text-xs"
                >
                  In Progress
                </button>
                <button
                  onClick={() => handleUpdateTaskStatus(task.id, 'Completed')}
                  className="btn btn-secondary text-xs"
                >
                  Completed
                </button>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default TaskManagement;