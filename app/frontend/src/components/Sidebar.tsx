'use client';

import { FC } from 'react';
import { useGetCategoryListQuery, useGetColorListQuery } from '@/store';
import {
  Accordion,
} from '@/components/ui/accordion';

// 分割されたフィルターコンポーネントをインポート
import { FilterControls, FeaturedFilter, CategoryFilter, ColorFilter } from '@/components/Filters';

interface SidebarProps {
  selectedCategory: string | null;
  selectedColor: string | null;
  selectedFeatured: string | null;
  onCategoryChange: (category: string | null) => void;
  onColorChange: (color: string | null) => void;
  onFeaturedChange: (featured: string | null) => void;
  onClearFilters: () => void;
  isMobile?: boolean;
}

const Sidebar: FC<SidebarProps> = ({
  selectedCategory,
  selectedColor,
  selectedFeatured,
  onCategoryChange,
  onColorChange,
  onFeaturedChange,
  onClearFilters,
  isMobile = false,
}) => {
  const hasFilters = !!(selectedCategory || selectedColor || selectedFeatured);
  
  // APIを使用してカテゴリを取得
  const { 
    data: categoriesData, 
    isLoading: categoriesLoading, 
    error: categoriesError 
  } = useGetCategoryListQuery();
  const categories = categoriesData?.categories || [];

  const {
    data: colorsData,
    isLoading: colorsLoading,
    error: colorsError,
  } = useGetColorListQuery();
  const colors = colorsData?.colors || [];

  const containerClasses = isMobile 
    ? "bg-background" 
    : "w-72 flex-shrink-0 bg-background border-r h-screen sticky top-16 overflow-y-auto";

  const contentClasses = isMobile 
    ? "p-4 space-y-6" 
    : "p-8 space-y-8";

  return (
    <aside className={containerClasses}>
      <div className={contentClasses}>
        {/* Clear Filters */}
        <FilterControls 
          hasFilters={hasFilters} 
          onClearFilters={onClearFilters} 
        />

        <Accordion
          type="multiple"
          defaultValue={["collections", "featured", "color"]}
          className="w-full"
        >
          {/* Featured */}
          <FeaturedFilter
            selectedFeatured={selectedFeatured}
            onFeaturedChange={onFeaturedChange}
          />

          {/* Collections */}
          <CategoryFilter
            categories={categories}
            selectedCategory={selectedCategory}
            isLoading={categoriesLoading}
            error={categoriesError}
            onCategoryChange={onCategoryChange}
          />

          {/* Color */}
          <ColorFilter
            colors={colors}
            selectedColor={selectedColor}
            isLoading={colorsLoading}
            error={colorsError}
            onColorChange={onColorChange}
          />
        </Accordion>
      </div>
    </aside>
  );
};

export { Sidebar }; 