'use client';

import Link from 'next/link';
import Image from 'next/image';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ImageIcon } from 'lucide-react';

// ProductCardコンポーネントで必要なプロパティのみを定義
interface ProductCardProduct {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  image: string;
  colors: string[];
  isSoldOut?: boolean;
  isOnSale?: boolean;
  isBestSeller?: boolean;
  isQuickShip?: boolean;
}

interface ProductCardProps {
  product: ProductCardProduct;
}

const ProductCard = ({ product }: ProductCardProps) => {
  const formatPrice = (price: number) => `$${price.toLocaleString()}`;

  const discountPercentage = product.salePrice 
    ? Math.round(((product.price - product.salePrice) / product.price) * 100)
    : 0;

  return (
    <Card className="group overflow-hidden border-0 shadow-none bg-transparent">
      <Link href={`/products/${product.id}`}>
        <div className="relative overflow-hidden bg-muted rounded-lg transition-transform duration-300 group-hover:scale-[1.02]">
          {/* Product Image */}
          <div className="aspect-square relative">
            {product.image ? (
              <Image
                src={product.image}
                alt={product.name}
                fill
                className="object-cover group-hover:scale-105 transition-transform duration-300"
                sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
              />
            ) : (
              <div className="w-full h-full flex items-center justify-center bg-muted">
                <ImageIcon className="w-16 h-16 text-muted-foreground/50" />
              </div>
            )}
            
            {/* Status Badges */}
            <div className="absolute top-3 left-3 flex flex-col gap-2">
              {product.isSoldOut && (
                <Badge variant="destructive" className="text-xs">
                  Sold Out
                </Badge>
              )}
              {product.isOnSale && !product.isSoldOut && (
                <Badge className="bg-green-600 hover:bg-green-700 text-xs">
                  -{discountPercentage}%
                </Badge>
              )}
              {product.isBestSeller && (
                <Badge variant="default" className="text-xs">
                  Best Seller
                </Badge>
              )}
              {product.isQuickShip && (
                <Badge variant="secondary" className="bg-blue-600 text-white hover:bg-blue-700 text-xs">
                  Quick Ship
                </Badge>
              )}
            </div>
          </div>
        </div>

        <CardContent className="p-0 pt-4">
          {/* Product Info */}
          <div className="space-y-2">
            <h3 className="text-sm font-medium text-foreground group-hover:text-muted-foreground transition-colors line-clamp-2">
              {product.name}
            </h3>
            
            <div className="flex items-center gap-2 flex-wrap">
              {product.salePrice ? (
                <>
                  <span className="text-sm font-semibold text-foreground">
                    {formatPrice(product.salePrice)}
                  </span>
                  <span className="text-sm text-muted-foreground line-through">
                    {formatPrice(product.price)}
                  </span>
                </>
              ) : (
                <span className="text-sm font-semibold text-foreground">
                  {formatPrice(product.price)}
                </span>
              )}
            </div>

            {/* Color indicators */}
            {product.colors.length > 0 && (
              <div className="flex items-center gap-1.5 pt-1">
                {product.colors.slice(0, 4).map((color, index) => (
                  <div
                    key={index}
                    className="w-4 h-4 rounded-full border-2 border-border shadow-sm"
                    style={{
                      backgroundColor: getColorValue(color)
                    }}
                    title={color}
                  />
                ))}
                {product.colors.length > 4 && (
                  <span className="text-xs text-muted-foreground ml-1">
                    +{product.colors.length - 4}
                  </span>
                )}
              </div>
            )}
          </div>
        </CardContent>
      </Link>
    </Card>
  );
};

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
    'Beige': '#F5F5DC',
  };
  
  return colorMap[colorName] || '#CCCCCC';
};

export default ProductCard; 