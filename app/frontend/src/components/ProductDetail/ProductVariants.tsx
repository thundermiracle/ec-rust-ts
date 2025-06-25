'use client';

import { Card, CardContent } from '@/components/ui/card';
import { FC } from 'react';

interface Variant {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  isSoldOut: boolean;
}

interface ProductVariantsProps {
  variants: Variant[];
  selectedVariant: Variant;
  onVariantChange: (index: number) => void;
}

export const ProductVariants: FC<ProductVariantsProps> = ({
  variants,
  selectedVariant,
  onVariantChange,
}) => {
  const formatPrice = (price: number) => `¥${price.toLocaleString()}`;

  if (!variants || variants.length === 0) {
    return null;
  }

  return (
    <div>
      <h3 className="text-lg font-medium text-foreground mb-4">
        Options
      </h3>
      <div className="space-y-2">
        {variants.map((variant, index: number) => (
          <Card
            key={variant.id}
            className={`cursor-pointer transition-colors ${
              selectedVariant?.id === variant.id
                ? "border-foreground bg-muted/50"
                : variant.isSoldOut
                ? "bg-muted/50 cursor-not-allowed"
                : "hover:border-muted-foreground"
            }`}
            onClick={() =>
              !variant.isSoldOut && onVariantChange(index)
            }
          >
            <CardContent className="p-4">
              <div className="flex justify-between items-center">
                <span
                  className={`font-medium ${
                    variant.isSoldOut ? "text-muted-foreground" : ""
                  }`}
                >
                  {variant.name}
                </span>
                <span
                  className={`font-bold ${
                    variant.isSoldOut ? "text-muted-foreground" : ""
                  }`}
                >
                  {variant.salePrice
                    ? formatPrice(variant.salePrice)
                    : formatPrice(variant.price)}
                </span>
              </div>
              {variant.isSoldOut && (
                <span className="text-sm text-muted-foreground">
                  Out of stock
                </span>
              )}
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
} 