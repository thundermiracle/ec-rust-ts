'use client';

import { useState, useMemo } from 'react';
import { ProductCard, Sidebar } from '@/components';
import { Button } from '@/components/ui/button';
import { useGetProductListQuery } from '@/store';
import { Filter, X } from 'lucide-react';

export default function Home() {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedColor, setSelectedColor] = useState<string | null>(null);
  const [selectedFeatured, setSelectedFeatured] = useState<string | null>(null);
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const { data } = useGetProductListQuery();

  const filteredProducts = useMemo(() => {
    return data?.products?.filter((product) => {
      // Category filter
      if (selectedCategory && product.category !== selectedCategory) {
        return false;
      }

      // Color filter
      if (selectedColor && !product.colors.includes(selectedColor)) {
        return false;
      }

      // Featured filter
      if (selectedFeatured) {
        switch (selectedFeatured) {
          case 'Quick Ship':
            if (!product.isQuickShip) return false;
            break;
          case 'Best Sellers':
            if (!product.isBestSeller) return false;
            break;
          case 'On Sale':
            if (!product.isOnSale) return false;
            break;
        }
      }

      return true;
    });
  }, [data?.products, selectedCategory, selectedColor, selectedFeatured]);

  const clearFilters = () => {
    setSelectedCategory(null);
    setSelectedColor(null);
    setSelectedFeatured(null);
  };

  const hasActiveFilters = !!(selectedCategory || selectedColor || selectedFeatured);

  return (
    <div className="flex min-h-screen bg-background">
      {/* Desktop Sidebar */}
      <div className="hidden lg:block">
        <Sidebar
          selectedCategory={selectedCategory}
          selectedColor={selectedColor}
          selectedFeatured={selectedFeatured}
          onCategoryChange={setSelectedCategory}
          onColorChange={setSelectedColor}
          onFeaturedChange={setSelectedFeatured}
          onClearFilters={clearFilters}
        />
      </div>

      {/* Mobile Sidebar Overlay */}
      {isSidebarOpen && (
        <div className="lg:hidden fixed inset-0 z-50 flex">
          {/* Backdrop */}
          <div 
            className="fixed inset-0 bg-black/50 backdrop-blur-sm" 
            onClick={() => setIsSidebarOpen(false)}
          />
          
          {/* Sidebar */}
          <div className="relative bg-background w-80 h-full overflow-y-auto">
            <div className="flex items-center justify-between p-4 border-b">
              <h2 className="text-lg font-semibold">Filters</h2>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setIsSidebarOpen(false)}
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
            <Sidebar
              selectedCategory={selectedCategory}
              selectedColor={selectedColor}
              selectedFeatured={selectedFeatured}
              onCategoryChange={setSelectedCategory}
              onColorChange={setSelectedColor}
              onFeaturedChange={setSelectedFeatured}
              onClearFilters={clearFilters}
              isMobile={true}
            />
          </div>
        </div>
      )}

      {/* Main Content */}
      <main className="flex-1 min-w-0">
        <div className="p-4 sm:p-6 lg:p-8">
          {/* Mobile Filter Button */}
          <div className="lg:hidden mb-6">
            <div className="flex items-center justify-between">
              <Button
                variant="outline"
                onClick={() => setIsSidebarOpen(true)}
                className="flex items-center gap-2"
              >
                <Filter className="h-4 w-4" />
                Filters
                {hasActiveFilters && (
                  <span className="ml-1 h-2 w-2 bg-primary rounded-full" />
                )}
              </Button>
              
              {hasActiveFilters && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={clearFilters}
                  className="text-muted-foreground"
                >
                  Clear all
                </Button>
              )}
            </div>
          </div>

          <div className="container mx-auto">
            {/* Header */}
            <div className="mb-6 lg:mb-8">
              <h1 className="text-2xl sm:text-3xl font-bold text-foreground mb-2 lg:mb-4">All Artifacts</h1>
              <p className="text-muted-foreground text-sm sm:text-base">Minimalist products for your home and office</p>
            </div>

            {/* Product Count */}
            <div className="mb-4 lg:mb-6">
              <p className="text-sm text-muted-foreground">
                {filteredProducts?.length} {filteredProducts?.length === 1 ? 'product' : 'products'}
              </p>
            </div>

            {/* Active Filters Display (Mobile) */}
            {hasActiveFilters && (
              <div className="lg:hidden mb-4 flex flex-wrap gap-2">
                {selectedCategory && (
                  <div className="inline-flex items-center gap-1 px-3 py-1 bg-secondary rounded-full text-sm">
                    <span>{selectedCategory}</span>
                    <button
                      onClick={() => setSelectedCategory(null)}
                      className="ml-1 text-muted-foreground hover:text-foreground"
                    >
                      <X className="h-3 w-3" />
                    </button>
                  </div>
                )}
                {selectedColor && (
                  <div className="inline-flex items-center gap-1 px-3 py-1 bg-secondary rounded-full text-sm">
                    <span>{selectedColor}</span>
                    <button
                      onClick={() => setSelectedColor(null)}
                      className="ml-1 text-muted-foreground hover:text-foreground"
                    >
                      <X className="h-3 w-3" />
                    </button>
                  </div>
                )}
                {selectedFeatured && (
                  <div className="inline-flex items-center gap-1 px-3 py-1 bg-secondary rounded-full text-sm">
                    <span>{selectedFeatured}</span>
                    <button
                      onClick={() => setSelectedFeatured(null)}
                      className="ml-1 text-muted-foreground hover:text-foreground"
                    >
                      <X className="h-3 w-3" />
                    </button>
                  </div>
                )}
              </div>
            )}

            {/* Products Grid - レスポンシブ対応 */}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-6">
              {filteredProducts?.map((product) => (
                <ProductCard key={product.id} product={product} />
              ))}
            </div>

            {/* No products message */}
            {filteredProducts?.length === 0 && (
              <div className="text-center py-16">
                <p className="text-muted-foreground text-lg mb-4">No products found matching your filters.</p>
                <Button
                  onClick={clearFilters}
                  variant="outline"
                >
                  Clear all filters
                </Button>
              </div>
            )}
          </div>
        </div>
      </main>
    </div>
  );
}
