'use client';

import React from 'react';
import { cn } from '@/lib/utils';
import { X } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface DrawerProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  children: React.ReactNode;
  side?: 'left' | 'right';
}

interface DrawerContentProps {
  children: React.ReactNode;
  className?: string;
}

interface DrawerHeaderProps {
  children: React.ReactNode;
  className?: string;
}

interface DrawerTitleProps {
  children: React.ReactNode;
  className?: string;
}

interface DrawerCloseProps {
  className?: string;
  onClick?: () => void;
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

  if (!open) return null;

  return (
    <>
      {/* Overlay */}
      <div
        className="fixed inset-0 z-50 bg-black/50"
        onClick={() => onOpenChange(false)}
      />
      
      {/* Drawer */}
      <div
        className={cn(
          "fixed z-50 bg-background shadow-lg transition-transform duration-300 ease-in-out",
          {
            // Desktop styles
            "top-0 h-full w-96": side === 'right',
            "right-0": side === 'right' && open,
            "left-0": side === 'left' && open,
            
            // Mobile styles - full screen
            "md:w-96 w-full": true,
          }
        )}
      >
        {children}
      </div>
    </>
  );
};

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

export const DrawerHeader: React.FC<DrawerHeaderProps> = ({
  children,
  className,
}) => {
  return (
    <div className={cn("flex items-center justify-between border-b p-4", className)}>
      {children}
    </div>
  );
};

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

export const DrawerClose: React.FC<DrawerCloseProps> = ({ className, onClick }) => {
  return (
    <Button variant="ghost" size="sm" className={cn("h-8 w-8 p-0", className)} onClick={onClick}>
      <X className="h-4 w-4" />
      <span className="sr-only">Close</span>
    </Button>
  );
}; 