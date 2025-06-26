'use client';

import React from 'react';
import { cn } from '@/lib/utils';
import { X } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface DrawerCloseProps {
  className?: string;
  onClick?: () => void;
}

export const DrawerClose: React.FC<DrawerCloseProps> = ({ className, onClick }) => {
  return (
    <Button variant="ghost" size="sm" className={cn("h-8 w-8 p-0", className)} onClick={onClick}>
      <X className="h-4 w-4" />
      <span className="sr-only">Close</span>
    </Button>
  );
}; 