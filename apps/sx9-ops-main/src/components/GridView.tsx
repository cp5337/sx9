import React from 'react';

interface Task {
  task_id: number;
  task_name: string;
  description: string;
  hd4_hunt_actions: string[];
}

interface GridViewProps {
  tasks: Task[];
}

const GridView: React.FC<GridViewProps> = ({ tasks }) => {
  return (
    <div className="h-full bg-gray-800 p-2 rounded overflow-auto">
      <table className="w-full text-left text-xs">
        <thead>
          <tr className="bg-gray-700">
            <th className="p-2">ID</th>
            <th className="p-2">Task Name</th>
            <th className="p-2">Description</th>
            <th className="p-2">HD4 Actions</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.task_id} className="border-b border-gray-700">
              <td className="p-2">{task.task_id}</td>
              <td className="p-2">{task.task_name}</td>
              <td className="p-2">{task.description}</td>
              <td className="p-2">{task.hd4_hunt_actions.join(', ')}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default GridView;