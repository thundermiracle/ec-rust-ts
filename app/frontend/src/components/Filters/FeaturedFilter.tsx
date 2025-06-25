'use client';

import { FC } from 'react';
import { Button } from '@/components/ui/button';
import {
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';

const featuredCategories = [
  'Quick Ship',
  'Best Sellers',
  'On Sale',
];

interface FeaturedFilterProps {
  selectedFeatured: string | null;
  onFeaturedChange: (featured: string | null) => void;
}

export const FeaturedFilter: FC<FeaturedFilterProps> = ({
  selectedFeatured,
  onFeaturedChange,
}) => {
  return (
    <AccordionItem value="featured" className="border-none">
      <AccordionTrigger className="text-xs font-bold text-foreground uppercase tracking-widest hover:no-underline py-3 px-0 mt-4">
        Featured
      </AccordionTrigger>
      <AccordionContent className="pb-2">
        <div className="space-y-1 pt-4">
          {featuredCategories.map((featured) => (
            <Button
              key={featured}
              variant="ghost"
              size="sm"
              onClick={() =>
                onFeaturedChange(
                  featured === selectedFeatured ? null : featured
                )
              }
              className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                selectedFeatured === featured
                  ? "text-foreground font-medium bg-muted/50"
                  : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
              }`}
            >
              {featured}
            </Button>
          ))}
        </div>
      </AccordionContent>
    </AccordionItem>
  );
} 