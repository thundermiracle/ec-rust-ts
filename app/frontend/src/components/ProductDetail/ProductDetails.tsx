'use client';

import { FC } from "react";

interface ProductDetailsProps {
  selectedVariant: {
    material?: string;
    dimensions?: string;
  };
  category: string;
}

export const ProductDetails: FC<ProductDetailsProps> = ({
  selectedVariant,
  category,
}) => {
  return (
    <div>
      <h3 className="text-lg font-medium text-foreground mb-4">
        Product Details
      </h3>
      <div className="space-y-3 text-sm text-muted-foreground">
        {selectedVariant.material && (
          <div className="flex justify-between">
            <span>Material:</span>
            <span className="text-foreground">
              {selectedVariant.material}
            </span>
          </div>
        )}
        {selectedVariant.dimensions && (
          <div className="flex justify-between">
            <span>Dimensions:</span>
            <span className="text-foreground">
              {selectedVariant.dimensions}
            </span>
          </div>
        )}
        <div className="flex justify-between">
          <span>Category:</span>
          <span className="text-foreground capitalize">
            {category.replace("-", " ")}
          </span>
        </div>
      </div>
    </div>
  );
} 