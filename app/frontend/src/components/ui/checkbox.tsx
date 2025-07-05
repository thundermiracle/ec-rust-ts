import React from 'react';
import { cn } from '@/lib/utils';

interface CheckboxProps {
  id: string;
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
  description?: string;
}

export function Checkbox({
  id,
  label,
  checked,
  onChange,
  disabled = false,
  className,
  description
}: CheckboxProps) {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!disabled) {
      onChange(e.target.checked);
    }
  };

  return (
    <div className={cn("flex items-center space-x-2", className)}>
      <input
        type="checkbox"
        id={id}
        checked={checked}
        onChange={handleChange}
        disabled={disabled}
        className="h-4 w-4"
      />
      <label htmlFor={id} className="text-sm cursor-pointer">
        {label}
        {description && (
          <div className="text-xs text-muted-foreground mt-1">{description}</div>
        )}
      </label>
    </div>
  );
}

interface CheckboxGroupProps {
  options: Array<{
    id: string;
    label: string;
    description?: string;
  }>;
  values: string[];
  onChange: (values: string[]) => void;
  disabled?: boolean;
  className?: string;
}

export function CheckboxGroup({
  options,
  values,
  onChange,
  disabled = false,
  className
}: CheckboxGroupProps) {
  const handleChange = (optionId: string, checked: boolean) => {
    if (disabled) return;
    
    if (checked) {
      onChange([...values, optionId]);
    } else {
      onChange(values.filter(id => id !== optionId));
    }
  };

  return (
    <div className={cn("space-y-2", className)}>
      {options.map((option) => (
        <Checkbox
          key={option.id}
          id={option.id}
          label={option.label}
          checked={values.includes(option.id)}
          onChange={(checked) => handleChange(option.id, checked)}
          disabled={disabled}
          description={option.description}
        />
      ))}
    </div>
  );
} 