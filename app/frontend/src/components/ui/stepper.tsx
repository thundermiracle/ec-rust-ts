import React from 'react';
import { cn } from '@/lib/utils';

interface StepperStep {
  id: string;
  label: string;
  completed?: boolean;
}

interface StepperProps {
  steps: StepperStep[];
  currentStep: string;
  className?: string;
}

export function Stepper({ steps, currentStep, className }: StepperProps) {
  const currentStepIndex = steps.findIndex(step => step.id === currentStep);
  
  return (
    <div className={cn("flex items-center space-x-4", className)}>
      {steps.map((step, index) => (
        <React.Fragment key={step.id}>
          <div className={cn(
            "flex items-center",
            index <= currentStepIndex ? "text-primary" : "text-muted-foreground"
          )}>
            <div className={cn(
              "w-8 h-8 rounded-full border-2 flex items-center justify-center text-sm font-medium",
              index < currentStepIndex ? "border-primary bg-primary text-primary-foreground" :
              index === currentStepIndex ? "border-primary bg-primary text-primary-foreground" :
              "border-muted-foreground"
            )}>
              {index + 1}
            </div>
            <span className="ml-2">{step.label}</span>
          </div>
          {index < steps.length - 1 && (
            <div className={cn(
              "flex-1 border-t",
              index < currentStepIndex ? "border-primary" : "border-muted-foreground"
            )} />
          )}
        </React.Fragment>
      ))}
    </div>
  );
} 