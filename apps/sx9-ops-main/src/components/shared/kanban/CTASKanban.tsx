import React, { useState } from 'react';
import { Plus, MoreVertical, Clock, User, AlertTriangle, CheckCircle } from 'lucide-react';

interface Task {
  id: string;
  title: string;
  description: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
  assignee: string;
  dueDate: string;
  status: 'todo' | 'in-progress' | 'review' | 'done';
  tags: string[];
}

interface CTASKanbanProps {
  title?: string;
  tasks?: Task[];
  onTaskMove?: (taskId: string, newStatus: string) => void;
  onTaskAdd?: (task: Omit<Task, 'id'>) => void;
  className?: string;
}

const CTASKanban: React.FC<CTASKanbanProps> = ({
  title = 'CTAS Task Board',
  tasks: initialTasks = [],
  onTaskMove,
  onTaskAdd,
  className = ''
}) => {
  const [tasks, setTasks] = useState<Task[]>(initialTasks.length > 0 ? initialTasks : [
    {
      id: '1',
      title: 'Investigate APT29 Activity',
      description: 'Analyze recent APT29 indicators and update threat intelligence',
      priority: 'critical',
      assignee: 'Threat Team',
      dueDate: '2024-01-15',
      status: 'in-progress',
      tags: ['threat-hunting', 'apt29']
    },
    {
      id: '2',
      title: 'Update Firewall Rules',
      description: 'Implement new firewall rules based on threat intelligence',
      priority: 'high',
      assignee: 'Network Team',
      dueDate: '2024-01-12',
      status: 'todo',
      tags: ['network', 'firewall']
    },
    {
      id: '3',
      title: 'Conduct Phishing Simulation',
      description: 'Run quarterly phishing simulation for all employees',
      priority: 'medium',
      assignee: 'Security Team',
      dueDate: '2024-01-20',
      status: 'review',
      tags: ['phishing', 'training']
    },
    {
      id: '4',
      title: 'Backup System Audit',
      description: 'Verify all backup systems are functioning correctly',
      priority: 'high',
      assignee: 'IT Team',
      dueDate: '2024-01-10',
      status: 'done',
      tags: ['backup', 'audit']
    },
    {
      id: '5',
      title: 'Vulnerability Assessment',
      description: 'Complete monthly vulnerability scan of critical systems',
      priority: 'medium',
      assignee: 'Security Team',
      dueDate: '2024-01-18',
      status: 'todo',
      tags: ['vulnerability', 'scanning']
    },
    {
      id: '6',
      title: 'Incident Response Training',
      description: 'Conduct incident response training for SOC team',
      priority: 'high',
      assignee: 'Training Team',
      dueDate: '2024-01-25',
      status: 'in-progress',
      tags: ['training', 'incident-response']
    }
  ]);

  const columns = [
    { id: 'todo', title: 'To Do', color: 'bg-gray-100 dark:bg-gray-700' },
    { id: 'in-progress', title: 'In Progress', color: 'bg-blue-100 dark:bg-blue-900' },
    { id: 'review', title: 'Review', color: 'bg-yellow-100 dark:bg-yellow-900' },
    { id: 'done', title: 'Done', color: 'bg-green-100 dark:bg-green-900' }
  ];

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'critical': return 'bg-red-500';
      case 'high': return 'bg-orange-500';
      case 'medium': return 'bg-yellow-500';
      case 'low': return 'bg-green-500';
      default: return 'bg-gray-500';
    }
  };

  const getPriorityIcon = (priority: string) => {
    switch (priority) {
      case 'critical': return <AlertTriangle className="w-3 h-3" />;
      case 'high': return <AlertTriangle className="w-3 h-3" />;
      case 'medium': return <Clock className="w-3 h-3" />;
      case 'low': return <CheckCircle className="w-3 h-3" />;
      default: return <Clock className="w-3 h-3" />;
    }
  };

  const handleDragStart = (e: React.DragEvent, taskId: string) => {
    e.dataTransfer.setData('taskId', taskId);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const handleDrop = (e: React.DragEvent, status: string) => {
    e.preventDefault();
    const taskId = e.dataTransfer.getData('taskId');
    const updatedTasks = tasks.map(task =>
      task.id === taskId ? { ...task, status: status as any } : task
    );
    setTasks(updatedTasks);
    if (onTaskMove) {
      onTaskMove(taskId, status);
    }
  };

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 className="font-semibold text-gray-900 dark:text-white">{title}</h3>
        <button className="flex items-center space-x-2 px-3 py-1 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm">
          <Plus className="w-4 h-4" />
          <span>Add Task</span>
        </button>
      </div>

      {/* Kanban Board */}
      <div className="p-4">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {columns.map((column) => (
            <div
              key={column.id}
              className={`${column.color} rounded-lg p-3 min-h-96`}
              onDragOver={handleDragOver}
              onDrop={(e) => handleDrop(e, column.id)}
            >
              <div className="flex items-center justify-between mb-3">
                <h4 className="font-medium text-gray-900 dark:text-white">{column.title}</h4>
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  {tasks.filter(task => task.status === column.id).length}
                </span>
              </div>
              
              <div className="space-y-3">
                {tasks
                  .filter(task => task.status === column.id)
                  .map((task) => (
                    <div
                      key={task.id}
                      draggable
                      onDragStart={(e) => handleDragStart(e, task.id)}
                      className="bg-white dark:bg-gray-800 rounded-lg p-3 border border-gray-200 dark:border-gray-600 shadow-sm cursor-move hover:shadow-md transition-shadow"
                    >
                      <div className="flex items-start justify-between mb-2">
                        <h5 className="font-medium text-gray-900 dark:text-white text-sm">
                          {task.title}
                        </h5>
                        <button className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                          <MoreVertical className="w-4 h-4" />
                        </button>
                      </div>
                      
                      <p className="text-xs text-gray-600 dark:text-gray-400 mb-3 line-clamp-2">
                        {task.description}
                      </p>
                      
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center space-x-1">
                          <div className={`w-2 h-2 rounded-full ${getPriorityColor(task.priority)}`}></div>
                          <span className="text-xs text-gray-600 dark:text-gray-400 capitalize">
                            {task.priority}
                          </span>
                        </div>
                        <span className="text-xs text-gray-500 dark:text-gray-400">
                          {task.dueDate}
                        </span>
                      </div>
                      
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-1">
                          <User className="w-3 h-3 text-gray-400" />
                          <span className="text-xs text-gray-600 dark:text-gray-400">
                            {task.assignee}
                          </span>
                        </div>
                      </div>
                      
                      <div className="flex flex-wrap gap-1 mt-2">
                        {task.tags.map((tag, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full text-xs"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default CTASKanban;
