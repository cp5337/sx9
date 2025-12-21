import React from "react";
import { Check } from "lucide-react";

interface CheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
}

export const Checkbox: React.FC<CheckboxProps> = ({
  label,
  checked,
  onChange,
  disabled = false,
}) => {
  return (
    <button
      onClick={() => !disabled && onChange(!checked)}
      disabled={disabled}
      className="flex items-center py-1 text-left w-full hover:bg-slate-800/30 rounded px-1 transition-colors"
    >
      <div
        className={`
          w-5 h-5 rounded border-2 flex items-center justify-center mr-2 flex-shrink-0
          ${checked ? "bg-emerald-600 border-emerald-600" : "border-slate-600 bg-transparent"}
          ${disabled ? "opacity-50" : ""}
        `}
      >
        {checked && <Check size={14} color="white" strokeWidth={3} />}
      </div>
      <span
        className={`text-xs ${
          disabled ? "text-slate-600" : "text-slate-300"
        } font-medium`}
      >
        {label}
      </span>
    </button>
  );
};
