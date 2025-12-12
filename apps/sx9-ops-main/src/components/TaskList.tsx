import React, { useState } from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';

interface Task {
  id: number;
  task_name: string;
  description: string;
}

interface Category {
  category_id: number;
  category: string;
  tasks: Task[];
}

interface TaskListProps {
  categories: Category[];
}

const TaskList: React.FC<TaskListProps> = ({ categories }) => {
  const [expandedCategories, setExpandedCategories] = useState<number[]>([]);

  const toggleCategory = (categoryId: number) => {
    setExpandedCategories(prev =>
      prev.includes(categoryId)
        ? prev.filter(id => id !== categoryId)
        : [...prev, categoryId]
    );
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4">
      <h2 className="text-lg font-semibold mb-2 text-gray-800 dark:text-white">Task List</h2>
      <div className="overflow-x-auto">
        <table className="w-full text-xs border-collapse">
          <thead>
            <tr className="bg-gray-100 dark:bg-gray-700">
              <th className="p-2 border border-gray-300 dark:border-gray-600 text-left">Category</th>
              <th className="p-2 border border-gray-300 dark:border-gray-600 text-left">Task</th>
              <th className="p-2 border border-gray-300 dark:border-gray-600 text-left">Description</th>
            </tr>
          </thead>
          <tbody>
            {categories.map(category => (
              <React.Fragment key={category.category_id}>
                <tr>
                  <td colSpan={3} className="p-2 border border-gray-300 dark:border-gray-600">
                    <button
                      onClick={() => toggleCategory(category.category_id)}
                      className="flex items-center w-full text-left font-semibold text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                    >
                      {expandedCategories.includes(category.category_id) ? (
                        <ChevronDown size={16} className="mr-1" />
                      ) : (
                        <ChevronRight size={16} className="mr-1" />
                      )}
                      <span>{category.category}</span>
                    </button>
                  </td>
                </tr>
                {expandedCategories.includes(category.category_id) && category.tasks.map(task => (
                  <tr key={task.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                    <td className="p-2 border border-gray-300 dark:border-gray-600"></td>
                    <td className="p-2 border border-gray-300 dark:border-gray-600">{task.task_name}</td>
                    <td className="p-2 border border-gray-300 dark:border-gray-600">{task.description}</td>
                  </tr>
                ))}
              </React.Fragment>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default TaskList;