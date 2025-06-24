'use client';

import { colors, featuredCategories } from '@/data/mockData';
import { useGetCategoryListQuery } from '@/store';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';

interface SidebarProps {
  selectedCategory: string | null;
  selectedColor: string | null;
  selectedFeatured: string | null;
  onCategoryChange: (category: string | null) => void;
  onColorChange: (color: string | null) => void;
  onFeaturedChange: (featured: string | null) => void;
  onClearFilters: () => void;
}

// Helper function to convert color names to CSS values
const getColorValue = (colorName: string): string => {
  const colorMap: { [key: string]: string } = {
    'Walnut': '#8B4513',
    'White Oak': '#F5F5DC',
    'Black Oak': '#2F2F2F',
    'Whitewash Oak': '#F8F8F8',
    'Black': '#000000',
    'White': '#FFFFFF',
    'Charcoal': '#36454F',
    'Mist': '#E6E6FA',
    'Smoke': '#738276',
    'Sand': '#C2B280',
    'Gray': '#808080',
    'Brass': '#B5A642',
    'Beige': '#F5F5DC',
  };
  
  return colorMap[colorName] || '#CCCCCC';
};

const Sidebar = ({
  selectedCategory,
  selectedColor,
  selectedFeatured,
  onCategoryChange,
  onColorChange,
  onFeaturedChange,
  onClearFilters,
}: SidebarProps) => {
  const hasFilters = selectedCategory || selectedColor || selectedFeatured;
  
  // APIを使用してカテゴリを取得
  const { 
    data: categoriesData, 
    isLoading: categoriesLoading, 
    error: categoriesError 
  } = useGetCategoryListQuery();
  
  const categories = categoriesData?.categories || [];

  return (
    <aside className="w-72 flex-shrink-0 bg-background border-r h-screen sticky top-16 overflow-y-auto">
      <div className="p-8 space-y-8">
        {/* Clear Filters */}
        {hasFilters && (
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
        )}

        <Accordion type="multiple" defaultValue={['collections', 'featured', 'color']} className="w-full">
          {/* Collections */}
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
                      ? 'text-foreground font-medium bg-muted/50' 
                      : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                  }`}
                >
                  All Products
                </Button>
                
                {categoriesLoading && (
                  <div className="px-2 py-2 text-sm text-muted-foreground">
                    Loading categories...
                  </div>
                )}
                
                {categoriesError && (
                  <div className="px-2 py-2 text-sm text-red-500">
                    Failed to load categories
                  </div>
                )}
                
                {categories.map((category) => (
                  <Button
                    key={category.id}
                    variant="ghost"
                    size="sm"
                    onClick={() => onCategoryChange(category.id)}
                    className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                      selectedCategory === category.id
                        ? 'text-foreground font-medium bg-muted/50'
                        : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                    }`}
                  >
                    {category.name}
                  </Button>
                ))}
              </div>
            </AccordionContent>
          </AccordionItem>

          {/* Featured */}
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
                    onClick={() => onFeaturedChange(featured === selectedFeatured ? null : featured)}
                    className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                      selectedFeatured === featured
                        ? 'text-foreground font-medium bg-muted/50'
                        : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                    }`}
                  >
                    {featured}
                  </Button>
                ))}
              </div>
            </AccordionContent>
          </AccordionItem>

          {/* Color */}
          <AccordionItem value="color" className="border-none">
            <AccordionTrigger className="text-xs font-bold text-foreground uppercase tracking-widest hover:no-underline py-3 px-0 mt-4">
              Color
            </AccordionTrigger>
            <AccordionContent className="pb-2">
              <div className="space-y-1 pt-4">
                {colors.map((color) => (
                  <Button
                    key={color}
                    variant="ghost"
                    size="sm"
                    onClick={() => onColorChange(color === selectedColor ? null : color)}
                    className={`w-full justify-start px-2 py-2 h-auto font-normal text-sm transition-colors ${
                      selectedColor === color
                        ? 'text-foreground font-medium bg-muted/50'
                        : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                    }`}
                  >
                    <div className="flex items-center gap-3">
                      <div
                        className="w-3 h-3 rounded-full border border-border shadow-sm flex-shrink-0"
                        style={{
                          backgroundColor: getColorValue(color)
                        }}
                      />
                      <span>{color}</span>
                    </div>
                  </Button>
                ))}
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </aside>
  );
};

export default Sidebar; 