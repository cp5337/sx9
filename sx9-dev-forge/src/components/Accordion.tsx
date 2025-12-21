import React, { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";

interface AccordionProps {
  title: string;
  children: React.ReactNode;
  expanded?: boolean;
}

export const Accordion: React.FC<AccordionProps> = ({
  title,
  children,
  expanded = false,
}) => {
  const [isOpen, setIsOpen] = useState(expanded);

  return (
    <div className="mb-2 border-b border-slate-700 pb-2">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center justify-between w-full py-1 text-left"
      >
        <span className="text-[10px] font-black text-slate-500 tracking-wide uppercase">
          {title}
        </span>
        {isOpen ? (
          <ChevronDown size={14} className="text-slate-500" />
        ) : (
          <ChevronRight size={14} className="text-slate-500" />
        )}
      </button>
      {isOpen && <div className="pt-2">{children}</div>}
    </div>
  );
};
