import React, { useState } from 'react';
import { Filter, X, Search, Calendar, MapPin, Layers } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Badge } from '@/components/ui/badge';

export interface FilterOption {
  id: string;
  label: string;
  value: string;
}

export interface FilterGroup {
  id: string;
  label: string;
  icon?: React.ComponentType<any>;
  options: FilterOption[];
  multiSelect?: boolean;
}

export interface FilterPanelProps {
  filters?: FilterGroup[];
  onFilterChange?: (filters: Record<string, string[]>) => void;
  onSectorChange?: (sectors: string[]) => void;
  onDateRangeChange?: (start: Date | null, end: Date | null) => void;
  onSearchChange?: (query: string) => void;
  className?: string;
  compact?: boolean;
}

const defaultFilters: FilterGroup[] = [
  {
    id: 'sectors',
    label: 'Sectors',
    icon: MapPin,
    options: [
      { id: 'energy', label: 'Energy', value: 'energy' },
      { id: 'finance', label: 'Finance', value: 'finance' },
      { id: 'healthcare', label: 'Healthcare', value: 'healthcare' },
      { id: 'transportation', label: 'Transportation', value: 'transportation' },
      { id: 'communications', label: 'Communications', value: 'communications' }
    ],
    multiSelect: true
  },
  {
    id: 'hd4Phase',
    label: 'HD4 Phase',
    options: [
      { id: 'hunt', label: 'Hunt', value: 'hunt' },
      { id: 'detect', label: 'Detect', value: 'detect' },
      { id: 'disrupt', label: 'Disrupt', value: 'disrupt' },
      { id: 'disable', label: 'Disable', value: 'disable' },
      { id: 'dominate', label: 'Dominate', value: 'dominate' }
    ],
    multiSelect: true
  },
  {
    id: 'priority',
    label: 'Priority',
    options: [
      { id: 'critical', label: 'Critical', value: 'critical' },
      { id: 'high', label: 'High', value: 'high' },
      { id: 'medium', label: 'Medium', value: 'medium' },
      { id: 'low', label: 'Low', value: 'low' }
    ],
    multiSelect: true
  }
];

export const FilterPanel: React.FC<FilterPanelProps> = ({
  filters = defaultFilters,
  onFilterChange,
  onSectorChange,
  onDateRangeChange,
  onSearchChange,
  className = '',
  compact = false
}) => {
  const [selectedFilters, setSelectedFilters] = useState<Record<string, string[]>>({});
  const [searchQuery, setSearchQuery] = useState('');
  const [isOpen, setIsOpen] = useState(false);

  const handleFilterToggle = (groupId: string, optionValue: string, multiSelect?: boolean) => {
    setSelectedFilters(prev => {
      const current = prev[groupId] || [];
      const newFilters = { ...prev };

      if (multiSelect) {
        if (current.includes(optionValue)) {
          newFilters[groupId] = current.filter(v => v !== optionValue);
        } else {
          newFilters[groupId] = [...current, optionValue];
        }
      } else {
        newFilters[groupId] = current.includes(optionValue) ? [] : [optionValue];
      }

      if (newFilters[groupId].length === 0) {
        delete newFilters[groupId];
      }

      if (onFilterChange) {
        onFilterChange(newFilters);
      }

      if (groupId === 'sectors' && onSectorChange) {
        onSectorChange(newFilters[groupId] || []);
      }

      return newFilters;
    });
  };

  const handleSearchChange = (value: string) => {
    setSearchQuery(value);
    if (onSearchChange) {
      onSearchChange(value);
    }
  };

  const clearFilters = () => {
    setSelectedFilters({});
    setSearchQuery('');
    if (onFilterChange) {
      onFilterChange({});
    }
    if (onSectorChange) {
      onSectorChange([]);
    }
    if (onSearchChange) {
      onSearchChange('');
    }
  };

  const activeFilterCount = Object.values(selectedFilters).reduce((sum, arr) => sum + arr.length, 0) + (searchQuery ? 1 : 0);

  if (compact) {
    return (
      <Popover open={isOpen} onOpenChange={setIsOpen}>
        <PopoverTrigger asChild>
          <Button variant="outline" size="sm" className={className}>
            <Filter className="h-4 w-4 mr-2" />
            Filters
            {activeFilterCount > 0 && (
              <Badge variant="default" className="ml-2 h-5 w-5 p-0 flex items-center justify-center">
                {activeFilterCount}
              </Badge>
            )}
          </Button>
        </PopoverTrigger>
        <PopoverContent align="end" className="w-80">
          <div className="space-y-4">
            {onSearchChange && (
              <div>
                <div className="flex items-center gap-2 mb-2">
                  <Search className="h-4 w-4 text-gray-400" />
                  <label className="text-sm font-medium">Search</label>
                </div>
                <Input
                  placeholder="Search..."
                  value={searchQuery}
                  onChange={(e) => handleSearchChange(e.target.value)}
                />
              </div>
            )}

            {filters.map((group) => {
              const Icon = group.icon;
              return (
                <div key={group.id}>
                  <div className="flex items-center gap-2 mb-2">
                    {Icon && <Icon className="h-4 w-4 text-gray-400" />}
                    <label className="text-sm font-medium">{group.label}</label>
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {group.options.map((option) => {
                      const isSelected = selectedFilters[group.id]?.includes(option.value) || false;
                      return (
                        <Badge
                          key={option.id}
                          variant={isSelected ? 'default' : 'outline'}
                          className="cursor-pointer"
                          onClick={() => handleFilterToggle(group.id, option.value, group.multiSelect)}
                        >
                          {option.label}
                        </Badge>
                      );
                    })}
                  </div>
                </div>
              );
            })}

            {activeFilterCount > 0 && (
              <Button variant="ghost" size="sm" onClick={clearFilters} className="w-full">
                <X className="h-4 w-4 mr-2" />
                Clear All Filters
              </Button>
            )}
          </div>
        </PopoverContent>
      </Popover>
    );
  }

  return (
    <div className={`bg-gray-800 rounded-lg border border-gray-700 p-4 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Filter className="h-5 w-5 text-gray-400" />
          <h3 className="text-sm font-semibold text-gray-300">Filters</h3>
          {activeFilterCount > 0 && (
            <Badge variant="default">{activeFilterCount} active</Badge>
          )}
        </div>
        {activeFilterCount > 0 && (
          <Button variant="ghost" size="sm" onClick={clearFilters}>
            <X className="h-4 w-4 mr-2" />
            Clear
          </Button>
        )}
      </div>

      {onSearchChange && (
        <div className="mb-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <Input
              placeholder="Search..."
              value={searchQuery}
              onChange={(e) => handleSearchChange(e.target.value)}
              className="pl-10"
            />
          </div>
        </div>
      )}

      <div className="space-y-4">
        {filters.map((group) => {
          const Icon = group.icon;
          return (
            <div key={group.id}>
              <div className="flex items-center gap-2 mb-2">
                {Icon && <Icon className="h-4 w-4 text-gray-400" />}
                <label className="text-sm font-medium text-gray-300">{group.label}</label>
              </div>
              <div className="flex flex-wrap gap-2">
                {group.options.map((option) => {
                  const isSelected = selectedFilters[group.id]?.includes(option.value) || false;
                  return (
                    <Badge
                      key={option.id}
                      variant={isSelected ? 'default' : 'outline'}
                      className="cursor-pointer hover:bg-gray-700"
                      onClick={() => handleFilterToggle(group.id, option.value, group.multiSelect)}
                    >
                      {option.label}
                    </Badge>
                  );
                })}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default FilterPanel;



