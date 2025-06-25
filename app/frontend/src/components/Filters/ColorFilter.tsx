'use client';

import { FC } from 'react';
import { Button } from '@/components/ui/button';
import {
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';
import { SidebarSkeleton } from '@/components';

interface Color {
  id: string | number;
  name: string;
  hex: string;
}

interface ColorFilterProps {
  colors: Color[];
  selectedColor: string | null;
  isLoading: boolean;
  error: unknown;
  onColorChange: (color: string | null) => void;
}

export const ColorFilter: FC<ColorFilterProps> = ({
  colors,
  selectedColor,
  isLoading,
  error,
  onColorChange,
}) => {
  return (
    <AccordionItem value="color" className="border-none">
      <AccordionTrigger className="text-xs font-bold text-foreground uppercase tracking-widest hover:no-underline py-3 px-0 mt-4">
        Color
      </AccordionTrigger>
      <AccordionContent className="pb-2">
        <div className="space-y-1 pt-4">
          {isLoading && (
            <SidebarSkeleton count={5} showColorCircle={true} />
          )}

          {!!error && (
            <div className="px-2 py-2 text-sm text-red-500">
              Failed to load colors
            </div>
          )}

          {colors.map((color) => (
            <Button
              key={color.id}
              variant="ghost"
              size="sm"
              onClick={() =>
                onColorChange(
                  color.name === selectedColor ? null : color.name
                )
              }
              className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                selectedColor === color.name
                  ? "text-foreground font-medium bg-muted/50"
                  : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
              }`}
            >
              <div className="flex items-center gap-3">
                <div
                  className="w-3 h-3 rounded-full border border-border shadow-sm flex-shrink-0"
                  style={{
                    backgroundColor: color.hex,
                  }}
                />
                <span>{color.name}</span>
              </div>
            </Button>
          ))}
        </div>
      </AccordionContent>
    </AccordionItem>
  );
} 