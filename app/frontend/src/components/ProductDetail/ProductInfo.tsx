'use client';

import { Badge } from '@/components/ui/badge';
import { FC } from 'react';

interface ProductInfoProps {
  name: string;
  description: string;
  selectedVariant: {
    price: number;
    salePrice?: number;
  };
}

export const ProductInfo: FC<ProductInfoProps> = ({
  name,
  description,
  selectedVariant,
}) => {
  const formatPrice = (price: number) => `¥${price.toLocaleString()}`;

  return (
    <div>
      <h1 className="text-3xl font-bold text-foreground mb-4">
        {name}
      </h1>

      {/* Price */}
      <div className="flex items-center gap-4 mb-6">
        {selectedVariant.salePrice ? (
          <>
            <span className="text-2xl font-bold text-foreground">
              {formatPrice(selectedVariant.salePrice)}
            </span>
            <span className="text-xl text-muted-foreground line-through">
              {formatPrice(selectedVariant.price)}
            </span>
            <Badge className="bg-green-100 text-green-800 hover:bg-green-200">
              Save{" "}
              {formatPrice(
                selectedVariant.price - selectedVariant.salePrice
              )}
            </Badge>
          </>
        ) : (
          <span className="text-2xl font-bold text-foreground">
            {formatPrice(selectedVariant.price)}
          </span>
        )}
      </div>

      <p className="text-muted-foreground leading-relaxed">
        {description}
      </p>
    </div>
  );
} 