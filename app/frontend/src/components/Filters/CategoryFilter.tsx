'use client';

import { FC } from 'react';
import { Button } from '@/components/ui/button';
import {
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';
import { SidebarSkeleton } from '@/components';

interface Category {
  id: string | number;
  name: string;
}

interface CategoryFilterProps {
  categories: Category[];
  selectedCategory: string | null;
  isLoading: boolean;
  error: unknown;
  onCategoryChange: (category: string | null) => void;
}

const CategoryFilter: FC<CategoryFilterProps> = ({
  categories,
  selectedCategory,
  isLoading,
  error,
  onCategoryChange,
}) => {
  return (
    <AccordionItem value="collections" className="border-none">
      <AccordionTrigger className="text-xs font-bold text-foreground uppercase tracking-widest hover:no-underline py-3 px-0">
        Collections
      </AccordionTrigger>
      <AccordionContent className="pb-2">
        <div className="space-y-1 pt-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => onCategoryChange(null)}
            className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
              !selectedCategory
                ? "text-foreground font-medium bg-muted/50"
                : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
            }`}
          >
            All Products
          </Button>

          {isLoading && <SidebarSkeleton count={8} />}

          {!!error && (
            <div className="px-2 py-2 text-sm text-red-500">
              Failed to load categories
            </div>
          )}

          {categories.map((category) => (
            <Button
              key={category.id}
              variant="ghost"
              size="sm"
              onClick={() => onCategoryChange(category.name)}
              className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                selectedCategory === category.name
                  ? "text-foreground font-medium bg-muted/50"
                  : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
              }`}
            >
              {category.name}
            </Button>
          ))}
        </div>
      </AccordionContent>
    </AccordionItem>
  );
};

export { CategoryFilter }; 