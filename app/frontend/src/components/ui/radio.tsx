import React from 'react';
import { cn } from '@/lib/utils';

export interface RadioOption {
  id: string;
  label: string;
  description?: string;
  value?: string | number;
}

interface RadioGroupProps {
  name: string;
  options: RadioOption[];
  value?: string;
  onChange?: (value: string) => void;
  className?: string;
  disabled?: boolean;
  layout?: 'vertical' | 'horizontal';
}

export function RadioGroup({
  name,
  options,
  value,
  onChange,
  className,
  disabled = false,
  layout = 'vertical'
}: RadioGroupProps) {
  const handleChange = (optionId: string) => {
    if (!disabled && onChange) {
      onChange(optionId);
    }
  };

  return (
    <div className={cn(
      "space-y-2",
      layout === 'horizontal' && "flex flex-wrap gap-4 space-y-0",
      className
    )}>
      {options.map((option) => (
        <div key={option.id} className="flex items-center space-x-2 border rounded p-3">
          <input
            type="radio"
            id={option.id}
            name={name}
            value={option.value || option.id}
            checked={value === option.id}
            onChange={() => handleChange(option.id)}
            disabled={disabled}
            className="h-4 w-4"
          />
          <label htmlFor={option.id} className="flex-1 cursor-pointer">
            <div className="font-medium">{option.label}</div>
            {option.description && (
              <div className="text-sm text-muted-foreground">{option.description}</div>
            )}
          </label>
        </div>
      ))}
    </div>
  );
}

interface RadioCardProps {
  option: RadioOption;
  name: string;
  value?: string;
  onChange?: (value: string) => void;
  disabled?: boolean;
  children?: React.ReactNode;
}

export function RadioCard({
  option,
  name,
  value,
  onChange,
  disabled = false,
  children
}: RadioCardProps) {
  const handleChange = () => {
    if (!disabled && onChange) {
      onChange(option.id);
    }
  };

  return (
    <div className="flex items-center space-x-2 border rounded p-3">
      <input
        type="radio"
        id={option.id}
        name={name}
        value={option.value || option.id}
        checked={value === option.id}
        onChange={handleChange}
        disabled={disabled}
        className="h-4 w-4"
      />
      <label htmlFor={option.id} className="flex-1 cursor-pointer">
        {children ? children : (
          <div>
            <div className="font-medium">{option.label}</div>
            {option.description && (
              <div className="text-sm text-muted-foreground">{option.description}</div>
            )}
          </div>
        )}
      </label>
    </div>
  );
} 