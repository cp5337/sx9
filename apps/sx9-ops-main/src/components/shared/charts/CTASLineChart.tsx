import React from 'react';
import { TrendingUp, TrendingDown, Minus } from 'lucide-react';

interface DataPoint {
  x: string;
  y: number;
}

interface CTASLineChartProps {
  title: string;
  data: DataPoint[];
  color?: string;
  height?: number;
  showTrend?: boolean;
  className?: string;
}

const CTASLineChart: React.FC<CTASLineChartProps> = ({
  title,
  data,
  color = '#3B82F6',
  height = 200,
  showTrend = true,
  className = ''
}) => {
  if (!data || data.length === 0) {
    return (
      <div className={`bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 ${className}`}>
        <h3 className="font-semibold text-gray-900 dark:text-white mb-2">{title}</h3>
        <div className="flex items-center justify-center h-32 text-gray-500 dark:text-gray-400">
          No data available
        </div>
      </div>
    );
  }

  // Calculate trend
  const firstValue = data[0]?.y || 0;
  const lastValue = data[data.length - 1]?.y || 0;
  const trend = lastValue - firstValue;
  const trendPercentage = firstValue !== 0 ? (trend / firstValue) * 100 : 0;

  // Calculate chart dimensions
  const maxY = Math.max(...data.map(d => d.y));
  const minY = Math.min(...data.map(d => d.y));
  const range = maxY - minY || 1;

  // Generate SVG path
  const width = 100;
  const padding = 20;
  const chartWidth = width - (padding * 2);
  const chartHeight = height - (padding * 2);

  const points = data.map((point, index) => {
    const x = padding + (index / (data.length - 1)) * chartWidth;
    const y = padding + chartHeight - ((point.y - minY) / range) * chartHeight;
    return `${x},${y}`;
  }).join(' ');

  const pathData = `M ${points}`;

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-semibold text-gray-900 dark:text-white">{title}</h3>
        {showTrend && (
          <div className="flex items-center space-x-1">
            {trend > 0 ? (
              <TrendingUp className="w-4 h-4 text-green-500" />
            ) : trend < 0 ? (
              <TrendingDown className="w-4 h-4 text-red-500" />
            ) : (
              <Minus className="w-4 h-4 text-gray-500" />
            )}
            <span className={`text-sm font-medium ${
              trend > 0 ? 'text-green-600 dark:text-green-400' :
              trend < 0 ? 'text-red-600 dark:text-red-400' :
              'text-gray-600 dark:text-gray-400'
            }`}>
              {trendPercentage > 0 ? '+' : ''}{trendPercentage.toFixed(1)}%
            </span>
          </div>
        )}
      </div>

      <div className="relative">
        <svg
          width="100%"
          height={height}
          viewBox={`0 0 ${width} ${height}`}
          className="w-full"
        >
          {/* Grid lines */}
          {[0, 25, 50, 75, 100].map((percent) => (
            <line
              key={percent}
              x1={padding}
              y1={padding + (percent / 100) * chartHeight}
              x2={width - padding}
              y2={padding + (percent / 100) * chartHeight}
              stroke="#E5E7EB"
              strokeWidth="1"
              className="dark:stroke-gray-600"
            />
          ))}

          {/* Line chart */}
          <path
            d={pathData}
            fill="none"
            stroke={color}
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />

          {/* Data points */}
          {data.map((point, index) => {
            const x = padding + (index / (data.length - 1)) * chartWidth;
            const y = padding + chartHeight - ((point.y - minY) / range) * chartHeight;
            return (
              <circle
                key={index}
                cx={x}
                cy={y}
                r="3"
                fill={color}
                className="hover:r-4 transition-all"
              />
            );
          })}

          {/* Area fill */}
          <defs>
            <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stopColor={color} stopOpacity="0.3" />
              <stop offset="100%" stopColor={color} stopOpacity="0.05" />
            </linearGradient>
          </defs>
          <path
            d={`${pathData} L ${width - padding},${height - padding} L ${padding},${height - padding} Z`}
            fill="url(#areaGradient)"
          />
        </svg>

        {/* Y-axis labels */}
        <div className="absolute left-0 top-0 h-full flex flex-col justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>{maxY.toFixed(0)}</span>
          <span>{((maxY + minY) / 2).toFixed(0)}</span>
          <span>{minY.toFixed(0)}</span>
        </div>

        {/* X-axis labels */}
        <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-2">
          {data.map((point, index) => (
            <span key={index} className="flex-1 text-center">
              {point.x}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
};

export default CTASLineChart;
