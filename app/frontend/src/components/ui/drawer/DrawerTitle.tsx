'use client';

import React from 'react';
import { cn } from '@/lib/utils';

interface DrawerTitleProps {
  children: React.ReactNode;
  className?: string;
}

export const DrawerTitle: React.FC<DrawerTitleProps> = ({
  children,
  className,
}) => {
  return (
    <h2 className={cn("text-lg font-semibold", className)}>
      {children}
    </h2>
  );
}; 