'use client';

import React from 'react';
import { cn } from '@/lib/utils';

interface DrawerContentProps {
  children: React.ReactNode;
  className?: string;
}

export const DrawerContent: React.FC<DrawerContentProps> = ({
  children,
  className,
}) => {
  return (
    <div className={cn("flex h-full flex-col", className)}>
      {children}
    </div>
  );
}; 