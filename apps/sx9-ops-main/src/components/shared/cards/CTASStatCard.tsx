import React from 'react';
import { LucideIcon } from 'lucide-react';

interface CTASStatCardProps {
  title: string;
  value: string | number;
  icon?: LucideIcon;
  trend?: {
    value: number;
    isPositive: boolean;
  };
  status?: 'success' | 'warning' | 'danger' | 'info' | 'default';
  className?: string;
}

const CTASStatCard: React.FC<CTASStatCardProps> = ({
  title,
  value,
  icon: Icon,
  trend,
  status = 'default',
  className = ''
}) => {
  const getStatusColors = () => {
    switch (status) {
      case 'success':
        return {
          bg: 'bg-green-50 dark:bg-green-900/20',
          border: 'border-green-200 dark:border-green-800',
          icon: 'text-green-600 dark:text-green-400',
          value: 'text-green-600 dark:text-green-400'
        };
      case 'warning':
        return {
          bg: 'bg-yellow-50 dark:bg-yellow-900/20',
          border: 'border-yellow-200 dark:border-yellow-800',
          icon: 'text-yellow-600 dark:text-yellow-400',
          value: 'text-yellow-600 dark:text-yellow-400'
        };
      case 'danger':
        return {
          bg: 'bg-red-50 dark:bg-red-900/20',
          border: 'border-red-200 dark:border-red-800',
          icon: 'text-red-600 dark:text-red-400',
          value: 'text-red-600 dark:text-red-400'
        };
      case 'info':
        return {
          bg: 'bg-blue-50 dark:bg-blue-900/20',
          border: 'border-blue-200 dark:border-blue-800',
          icon: 'text-blue-600 dark:text-blue-400',
          value: 'text-blue-600 dark:text-blue-400'
        };
      default:
        return {
          bg: 'bg-white dark:bg-gray-800',
          border: 'border-gray-200 dark:border-gray-700',
          icon: 'text-gray-600 dark:text-gray-400',
          value: 'text-gray-900 dark:text-white'
        };
    }
  };

  const colors = getStatusColors();

  return (
    <div className={`${colors.bg} ${colors.border} border rounded-lg shadow-sm p-4 ${className}`}>
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <p className="text-sm text-gray-600 dark:text-gray-400 font-medium">{title}</p>
          <div className="flex items-center space-x-2 mt-1">
            <p className={`text-2xl font-bold ${colors.value}`}>{value}</p>
            {trend && (
              <span className={`text-xs font-medium ${
                trend.isPositive ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
              }`}>
                {trend.isPositive ? '+' : ''}{trend.value}%
              </span>
            )}
          </div>
        </div>
        {Icon && (
          <div className={`p-2 rounded-lg bg-white dark:bg-gray-700 shadow-sm`}>
            <Icon className={`w-6 h-6 ${colors.icon}`} />
          </div>
        )}
      </div>
    </div>
  );
};

export default CTASStatCard;
