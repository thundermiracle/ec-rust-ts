'use client';

import { useState, FC } from 'react';
import Image from 'next/image';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ImageIcon } from 'lucide-react';

interface ProductImageGalleryProps {
  productName: string;
  allImages: string[];
  selectedVariant: {
    isSoldOut: boolean;
    price: number;
    salePrice?: number;
  };
  isBestSeller: boolean;
  isQuickShip: boolean;
}

export const ProductImageGallery: FC<ProductImageGalleryProps> = ({
  productName,
  allImages,
  selectedVariant,
  isBestSeller,
  isQuickShip,
}) => {
  const [selectedImage, setSelectedImage] = useState(0);

  return (
    <div className="space-y-4">
      {/* Main Image */}
      <Card className="aspect-square relative overflow-hidden border-0 shadow-none bg-muted">
        {allImages[selectedImage] ? (
          <Image
            src={allImages[selectedImage]}
            alt={productName}
            fill
            className="object-cover rounded-lg"
            priority
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center">
            <ImageIcon className="w-24 h-24 text-muted-foreground/50" />
          </div>
        )}

        {/* Status Badges */}
        <div className="absolute top-4 left-4 flex flex-col gap-2">
          {selectedVariant.isSoldOut && (
            <Badge variant="destructive">Sold Out</Badge>
          )}
          {selectedVariant.salePrice && (
            <Badge className="bg-green-600 hover:bg-green-700">
              -
              {Math.round(
                ((selectedVariant.price - selectedVariant.salePrice) /
                  selectedVariant.price) *
                  100
              )}
              % OFF
            </Badge>
          )}
          {isBestSeller && (
            <Badge variant="default">Best Seller</Badge>
          )}
          {isQuickShip && (
            <Badge
              variant="secondary"
              className="bg-blue-600 text-white hover:bg-blue-700"
            >
              Quick Ship
            </Badge>
          )}
        </div>
      </Card>

      {/* Thumbnail Images */}
      {allImages.length > 1 && (
        <div className="flex gap-4">
          {allImages.map((image: string, index: number) => (
            <button
              key={index}
              onClick={() => setSelectedImage(index)}
              className={`w-20 h-20 relative bg-gray-50 rounded-lg overflow-hidden border-2 transition-colors ${
                selectedImage === index
                  ? "border-black"
                  : "border-transparent hover:border-gray-300"
              }`}
            >
              <Image
                src={image}
                alt={`${productName} ${index + 1}`}
                fill
                className="object-cover"
              />
            </button>
          ))}
        </div>
      )}
    </div>
  );
};
