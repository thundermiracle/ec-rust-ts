'use client';

import React from 'react';
import { cn } from '@/lib/utils';

interface DrawerProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  children: React.ReactNode;
  side?: 'left' | 'right';
}

export const Drawer: React.FC<DrawerProps> = ({
  open,
  onOpenChange,
  children,
  side = 'right',
}) => {
  React.useEffect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [open]);

  return (
    <>
      {/* Overlay */}
      <div
        className={cn(
          "fixed inset-0 z-50 bg-black/50 transition-opacity duration-300 ease-in-out",
          open ? "opacity-100" : "opacity-0 pointer-events-none"
        )}
        onClick={() => onOpenChange(false)}
      />
      
      {/* Drawer */}
      <div
        className={cn(
          "fixed z-50 bg-background shadow-lg transition-all duration-300 ease-in-out transform",
          {
            // Desktop styles
            "top-0 h-full w-96": side === 'right',
            "right-0 translate-x-0": side === 'right' && open,
            "right-0 translate-x-full": side === 'right' && !open,
            "left-0 -translate-x-full": side === 'left' && !open,
            "left-0 translate-x-0": side === 'left' && open,
            
            // Mobile styles - full screen
            "md:w-96 w-full": true,
          }
        )}
        style={{
          visibility: open ? 'visible' : 'hidden',
        }}
      >
        {children}
      </div>
    </>
  );
}; 