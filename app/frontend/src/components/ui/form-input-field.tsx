import React from 'react';
import { useFormContext, FieldError } from 'react-hook-form';
import { Input } from './input';
import { path } from 'ramda';
import { cn } from '@/lib/utils';

interface FormInputFieldProps extends React.InputHTMLAttributes<HTMLInputElement> {
  name: string;
  label: string;
}

export function FormInputField({ name, label, type = 'text', className, ...props }: FormInputFieldProps) {
  const { register, formState: { errors } } = useFormContext();
  const error = path<FieldError>(name.split('.'), errors);

  return (
    <div>
      <label htmlFor={name} className="block text-sm font-medium mb-2">
        {label}
      </label>
      <Input
        id={name}
        type={type}
        {...register(name)}
        className={cn(error && 'border-destructive', className)}
        {...props}
      />
      {error && <p className="text-sm text-destructive mt-1">{error.message}</p>}
    </div>
  );
} 