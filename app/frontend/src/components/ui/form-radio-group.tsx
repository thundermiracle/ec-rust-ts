import React from 'react';
import { useFormContext, Controller } from 'react-hook-form';
import { RadioGroup, type RadioOption } from './radio';

interface FormRadioGroupProps {
  name: string;
  label: string;
  options: RadioOption[];
}

export function FormRadioGroup({ name, label, options }: FormRadioGroupProps) {
  const { control } = useFormContext();

  return (
    <div>
      <label className="block text-sm font-medium mb-2">{label}</label>
      <Controller
        name={name}
        control={control}
        render={({ field, fieldState: { error } }) => (
          <>
            <RadioGroup
              {...field}
              options={options}
            />
            {error && <p className="text-sm text-destructive mt-1">{error.message}</p>}
          </>
        )}
      />
    </div>
  );
}

 