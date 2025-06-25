'use client';

import { FC } from "react";

interface ProductColorsProps {
  colors: string[];
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
    'Beige': '#F5F5DC',
  };
  
  return colorMap[colorName] || '#CCCCCC';
};

export const ProductColors: FC<ProductColorsProps> = ({ colors }) => {
  if (!colors || colors.length === 0) {
    return null;
  }

  return (
    <div>
      <h3 className="text-lg font-medium text-foreground mb-4">
        Available Colors
      </h3>
      <div className="flex gap-3">
        {colors.map((color: string, index: number) => (
          <div
            key={index}
            className="flex flex-col items-center gap-2"
          >
            <div
              className="w-10 h-10 rounded-full border-2 border-border shadow-sm"
              style={{ backgroundColor: getColorValue(color) }}
              title={color}
            />
            <span className="text-xs text-muted-foreground">
              {color}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
} 