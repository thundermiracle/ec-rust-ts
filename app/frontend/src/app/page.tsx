'use client';

import { useState, useMemo } from 'react';
import { products } from '../data/mockData';
import ProductCard from '../components/ProductCard';
import Sidebar from '../components/Sidebar';
import { Product } from '../types/product';
import { Button } from '@/components/ui/button';

export default function Home() {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedColor, setSelectedColor] = useState<string | null>(null);
  const [selectedFeatured, setSelectedFeatured] = useState<string | null>(null);

  const filteredProducts = useMemo(() => {
    return products.filter((product: Product) => {
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
          case 'Walnut Desk':
            if (product.category !== 'desks' || !product.colors.includes('Walnut')) return false;
            break;
          case 'Latest Artifacts':
            // For demo purposes, show products with variants
            if (!product.variants || product.variants.length === 0) return false;
            break;
        }
      }

      return true;
    });
  }, [selectedCategory, selectedColor, selectedFeatured]);

  const clearFilters = () => {
    setSelectedCategory(null);
    setSelectedColor(null);
    setSelectedFeatured(null);
  };

  return (
    <div className="flex min-h-screen bg-background">
      {/* Sidebar */}
      <Sidebar
        selectedCategory={selectedCategory}
        selectedColor={selectedColor}
        selectedFeatured={selectedFeatured}
        onCategoryChange={setSelectedCategory}
        onColorChange={setSelectedColor}
        onFeaturedChange={setSelectedFeatured}
        onClearFilters={clearFilters}
      />

      {/* Main Content */}
      <main className="flex-1 p-8">
        <div className="container mx-auto">
          {/* Header */}
          <div className="mb-8">
            <h1 className="text-3xl font-bold text-foreground mb-4">All Artifacts</h1>
            <p className="text-muted-foreground">Minimalist products for your home and office</p>
          </div>

          {/* Product Count */}
          <div className="mb-6">
            <p className="text-sm text-muted-foreground">
              {filteredProducts.length} {filteredProducts.length === 1 ? 'product' : 'products'}
            </p>
          </div>

          {/* Products Grid */}
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {filteredProducts.map((product: Product) => (
              <ProductCard key={product.id} product={product} />
            ))}
          </div>

          {/* No products message */}
          {filteredProducts.length === 0 && (
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
      </main>
    </div>
  );
}
