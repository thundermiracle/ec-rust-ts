'use client';

import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { FC } from 'react';

interface FilterControlsProps {
  hasFilters: boolean;
  onClearFilters: () => void;
}

export const FilterControls: FC<FilterControlsProps> = ({ hasFilters, onClearFilters }) => {
  if (!hasFilters) {
    return null;
  }

  return (
    <>
      <div className="pt-2">
        <Button
          variant="ghost"
          size="sm"
          onClick={onClearFilters}
          className="text-muted-foreground hover:text-foreground w-full justify-start px-2 py-1 h-auto font-normal underline text-xs"
        >
          Clear all filters
        </Button>
      </div>
      <Separator className="my-6" />
    </>
  );
} 