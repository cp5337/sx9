import React from 'react';
import { LucideIcon } from 'lucide-react';

interface CTASMetricCardProps {
  title: string;
  value: string | number;
  subtitle?: string;
  icon?: LucideIcon;
  progress?: number; // 0-100
  status?: 'success' | 'warning' | 'danger' | 'info' | 'default';
  className?: string;
}

const CTASMetricCard: React.FC<CTASMetricCardProps> = ({
  title,
  value,
  subtitle,
  icon: Icon,
  progress,
  status = 'default',
  className = ''
}) => {
  const getStatusColors = () => {
    switch (status) {
      case 'success':
        return {
          bg: 'bg-green-50 dark:bg-green-900/20',
          border: 'border-green-200 dark:border-green-800',
          progress: 'bg-green-500',
          icon: 'text-green-600 dark:text-green-400'
        };
      case 'warning':
        return {
          bg: 'bg-yellow-50 dark:bg-yellow-900/20',
          border: 'border-yellow-200 dark:border-yellow-800',
          progress: 'bg-yellow-500',
          icon: 'text-yellow-600 dark:text-yellow-400'
        };
      case 'danger':
        return {
          bg: 'bg-red-50 dark:bg-red-900/20',
          border: 'border-red-200 dark:border-red-800',
          progress: 'bg-red-500',
          icon: 'text-red-600 dark:text-red-400'
        };
      case 'info':
        return {
          bg: 'bg-blue-50 dark:bg-blue-900/20',
          border: 'border-blue-200 dark:border-blue-800',
          progress: 'bg-blue-500',
          icon: 'text-blue-600 dark:text-blue-400'
        };
      default:
        return {
          bg: 'bg-white dark:bg-gray-800',
          border: 'border-gray-200 dark:border-gray-700',
          progress: 'bg-gray-500',
          icon: 'text-gray-600 dark:text-gray-400'
        };
    }
  };

  const colors = getStatusColors();

  return (
    <div className={`${colors.bg} ${colors.border} border rounded-lg shadow-sm p-4 ${className}`}>
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center space-x-2">
            {Icon && <Icon className={`w-4 h-4 ${colors.icon}`} />}
            <h3 className="text-sm font-medium text-gray-900 dark:text-white">{title}</h3>
          </div>
          <p className="text-2xl font-bold text-gray-900 dark:text-white mt-1">{value}</p>
          {subtitle && (
            <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">{subtitle}</p>
          )}
          {progress !== undefined && (
            <div className="mt-3">
              <div className="flex justify-between text-xs text-gray-600 dark:text-gray-400 mb-1">
                <span>Progress</span>
                <span>{progress}%</span>
              </div>
              <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div 
                  className={`${colors.progress} h-2 rounded-full transition-all duration-300`}
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default CTASMetricCard;
